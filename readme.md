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
