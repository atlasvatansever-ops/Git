 use std::collections::HashMap;
use std::env;
use std::process;
use std::str::FromStr;
use std::string::String;
use std::doSomething;
fn main() {  
   
   fn doSomething(x: String) {
    println!("doSomething: {}", x);
   }

 // we move the value inside this function.
 // now doSomething is the owner of x.
 // Rust will free the memory associated with x 
 // as soon as it goes out of "doSomething" scope.
    doSomething(x);

 // The compiler will throw an error since we tried to use the value x
 // but since we moved it inside "doSomething"
 // we cannot use it as we don't have ownership
 // and the value may have been dropped.
    println!("{}", x);
}