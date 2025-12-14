# The Journal

## 12/12/25

### The Prelude

By default rust only brings a small set of items from the standard library into scope. See more [here](https://doc.rust-lang.org/std/prelude/index.html)!

## 12/14/25

### Constant Evaluation

Rust can do a set amount of compile time operations for constant values. See more [here](https://doc.rust-lang.org/reference/const_eval.html);

### Shadowing Madness

You can shadow variables, change them in an inner scope and leave the outer scoped variable unaffected.
 
```Rust
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // x = 12
    }

    println!("The value of x is: {x}"); // x = 6
} 
```