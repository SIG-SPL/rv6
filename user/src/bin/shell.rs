#![no_std]
#![no_main]

#[macro_use]
extern crate ulib;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum ShellCommand {
//     /// list files
//     Ls(Path),
//     /// change directory
//     Cd(Path),
//     /// make directory
//     Mkdir(Path),
//     /// remove directory
//     Rmdir(Path),
//     /// remove file
//     Rm(Path),
//     /// create file
//     Touch(Path),
//     /// empty command
//     Empty,
//     /// print current directory
//     Pwd,
//     /// exit
//     Exit,
//     /// help
//     Help,
// }

// impl ShellCommand {
//     pub fn parse(input: &str) -> Result<Self, Error> {
//         let mut iter = input.split_whitespace();
//         let cmd = iter.next();
//         match cmd {
//             Some(cmd) => match cmd {
//                 "ls" => {
//                     let path = iter.next().unwrap_or(".");
//                     Ok(Self::Ls(Path::new(path)))
//                 }
//                 "cd" => {
//                     let path = iter.next().unwrap_or("/");
//                     Ok(Self::Cd(Path::new(path)))
//                 }
//                 "mkdir" => {
//                     let path = match iter.next() {
//                         Some(path) => path,
//                         None => return Err(Error::Message("mkdir need a path".to_string())),
//                     };
//                     Ok(Self::Mkdir(Path::new(path)))
//                 }
//                 "rmdir" => {
//                     let path = match iter.next() {
//                         Some(path) => path,
//                         None => return Err(Error::Message("rmdir need a path".to_string())),
//                     };
//                     Ok(Self::Rmdir(Path::new(path)))
//                 }
//                 "rm" => {
//                     let path = match iter.next() {
//                         Some(path) => path,
//                         None => return Err(Error::Message("rm need a path".to_string())),
//                     };
//                     Ok(Self::Rm(Path::new(path)))
//                 }
//                 "touch" => {
//                     let path = match iter.next() {
//                         Some(path) => path,
//                         None => return Err(Error::Message("touch need a path".to_string())),
//                     };
//                     Ok(Self::Touch(Path::new(path)))
//                 }
//                 "pwd" => Ok(Self::Pwd),
//                 "exit" => Ok(Self::Exit),
//                 "help" => Ok(Self::Help),
//                 _ => Err(Error::UnknownCommand(cmd.to_string())),
//             },
//             None => Ok(Self::Empty),
//         }
//     }
// }

#[no_mangle]
pub extern "C" fn main() -> i32 {
    println!("Hello, RV6!");
    0
}
