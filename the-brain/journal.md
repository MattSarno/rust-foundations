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

## 12/18/25

### Undefined Behavior

"Safe" code is the absence of undefined behavior. Chapter 4 is mainly focused on operations on memory but [here](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) is a list of the rest of them.

## 12/19/25

### Slice Fun

Slices are essentially "fat" pointers to an area in memory. They have a pointer to the block of memory and a size to show where they stop. These are really useful for when you need to return or reference a portion of the memory but need to make sure it doesn't go out of scope when you finally use the reference.

```Rust
fn main() {
    let s = String::from("Hello, World!");
    let hello: &str = &s[0..5];
    let world: &str = &s[7..11];

    println!("{} {}", hello, world);
}
```