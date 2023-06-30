use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub const LOCKED: bool = true;
pub const UNLOCKED: bool = false;

#[repr(C)]
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
    name: &'static str, // debug purpose
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(data: T, _name: &'static str) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            value: UnsafeCell::new(data),
            name: _name,
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // spin
            core::hint::spin_loop();
        }
        // debug!("{} acquired", self.name);
        Guard { lock: self }
    }
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        // Safety: We have the lock, so no one else can access the data
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: We have the lock, so no one else can access the data
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        // debug!("{} released", self.lock.name);
        self.lock.locked.store(UNLOCKED, Ordering::Release);
    }
}

/// TODO: use our own os primitive to implement this
#[repr(C)]
pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked, no waiters
    /// 2: locked, one or more waiters
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        if self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.state.swap(2, Ordering::Acquire) != 0 {
                // wait(&self.state, 2)
            }
        }
        MutexGuard { mutex: self }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if self.mutex.state.swap(0, Ordering::Release) == 2 {
            // wake_one(&self.mutex.state);
        }
    }
}
