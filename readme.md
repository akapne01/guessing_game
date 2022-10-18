# Purpose
To familiarize myself with Rust syntax by making a program. 

# Reference: 
https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

# About Rust
- Prelude is a list of things that Rust automatically imports into every Rust program. You can explore what is included in the prelude here: https://doc.rust-lang.org/std/prelude/index.html.
- Prelude can be seen as a pattern to make using multiple types more convinient. You will find other preludes in the standard library. Like: `std::io::prelude`.
- Various libraries in Rust ecosystem may also define their own preludes, but they need to be imported in the code. 
- Variables are immutable by default. 
# Reading user input in Rust:
- To ne able to capture user input need to import crate: `use std::io;`.
- It can be used without import as follows directly in the code where `Stdin` is used: `std::io::Stdin`. 
- Stdin is a type that represents a handle to the standart input for your terminal.
- `read_line(&mut guess)` method calls `read_line` method on the standard input handle to get input from user. 
- We are passing `&mut guess` as arguement to `read_line` to tell it in what string to store the input in.
- This String needs to be mutable, because we are initalising the String to an empty string because at this point we do not know yet what the guess will be.
- When user enters guess, we read and validate it to make sure that it is number that we have received. If user eneters anything else the Rust will panic at runtime and that is not what we want. In this case, we would want to display a relevant error message, and ask for a user input until we can validate it, because the number is enetered, so our program will be able to compare if the guess was made.
- read_line takes takes what usesr types as a standard input and appends that into a string, so we pass string as an argument.
- The `&` indicates that this argument is a reference. This gives you a way to let multiple parts of your program access one piece of data without needing to copy that data into memory multiple times. 
- It is safe and easy to use references. They are immutable by default. Hence you need to write `&mut guess` rather than `&guess` to make it mutable.
- `read_line` assigns user input to the String, but it also returns a `Result` value. Result is enum. It can be one of multiple states. We call each possibe state a  variant.
- Purpose of Result enum is to encode error-handling information. 
- `Results` varriants are: `Ok` and `Err`. Err means that operation failed, and Err contains information about how or why the operation failed. Result enum has methods defined on them. An instance of a `Result` has an `expect` method that you can call. If this instance of `Result` is an `Err` value, `expect` will cause the program to crash and display the message that you passed as an argument to `expect`. If an error is returned, then it is likely that it is comming from the underlying oeprating system. If instance of `Result` is an `Ok` value, `expect` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user's input.
- If the method is not called anywhere, the program still compiles, but you will get a warning: warning: unused `Result` that must be used. Rust warns that you haven’t used the `Result` value returned from `read_line`, indicating that the program hasn’t handled a possible error. The right way to suppress this warning is to actually write error handling, but in our case we jsut want to crash this program when a problem occurs, so we can use `expect`. 
# Random Number Generator
- It is not included in the standard prelude, but is available through rand crate. Add to Cargo.toml file rand = "0.8.3" as dependency.
following import: `use rand::Rng;`.
# Adding a crate as a dependency
- `rand` is a crate. 
- Crate is a collection of Rust source code files.
- This project produces a binary crate, which is an executable. The `rand` crate is a library crate, which contains code intendend to be used in other programs and can't be executed on its own.
- Cargo understands Semantic Versioning (SemVer), which is a standard for writing version numbers. The number `0.8.3` is actually shothand for `^0.8.3`, which means any version that is at least `0.8.3` but below `0.9.0`. Cargo considers these versions to have public APIs still compatible with the version `0.8.3`, and this specification ensures you will get the latest patch release that will still compile with the code in this chaper. Any version `0.9.0` or greater is not guaranteed to have the same APIs as what the following examples use.
- When we include external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from https://Crates.io. It is where people in the Rust ecosystem post thero open source Rust projects for others to use. 
- After downloading the creates, Rust compiles them and then compiles the project with the dependnecies available.
- Next time you run Cargo knows that it has already downloaded and compiled the dependencies, and you haven't chnaged anything about them in Cargo.toml file. Cargo also knows that you haven't change anything about them in Cargo.toml file. 
- Cargo.lock file ensures reproducible builds. It's often checked into source control (git) with the rest of the code of your project. 

# Updating Creates
- `cargo update` will ignore Cargo.lock file and figure out the latest versions that fit your specification in Cargo.toml.
- Cargo will then write those versions to Cargo.lock file. 