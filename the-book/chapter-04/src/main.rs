// ------ 4.1: What is Ownership? ------

fn main() {
    let x = true;
    read(x);
}

fn read(y: bool) {
    if y {
        println!("y is true!"); // or is it?
    }
}

fn _fake_main() {
    let n = 5; // L1 stack (n = 5)
    let y = _plus_one(n); // L3 stack (n = 5 & y = 6)
    println!("The value of y is: {y}");
}

fn _plus_one(x: i32) -> i32 {
    return x + 1; // L2 Stack (n = 5 in main, x = 5 here)
}

fn _moving_pointers() {
    let a = Box::new([0; 1_000_000]); // 1 million element array lol!
    let b = a; // a is dead -- rip :( (this is so we don't free twice because undefined behavior is "bad" lol)
}

fn _fake_main_two() {
    let a_num = 4;
    make_and_drop();
    // no more a_box memory stored on the heap.
}

fn make_and_drop() {
    let a_box = Box::new(5);
} // a_box is dead there for the memroy is free weeee~!

fn _string_madness() {
    let first = String::from("Ferris"); // first -> Box["Ferris"]
    let full = _add_sufffix(first); // full -> Box["Ferris Jr."] & name is now dead x|
    println!("{full}");

    // Important note first, name, and full all point to the "same" area in memory not different boxes
}

fn _add_sufffix(mut name: String) -> String { 
    name.push_str(" Jr."); // name -> Box["Ferris Jr."] & first is now dead x|
    return name;
}

// ------ 4.2: & References & Borrowing & ------

// Here we re-build the reference type to see why its useful (verbose code is meh sometimes (although safe!))
fn _rebuilding_references() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = _use_and_return(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}

fn _use_and_return(g1: String, g2 String) -> (String, String) {
    println!("{} {}!", g1, g2);
    return (g1, g2);
}

// Instead same thing using refs
fn _just_use_ref() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2);
}

// m1 & m2 are still the owners | g1 & g2 just reference/borrow the value
fn _using_refs(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

// Here we just show how to dereferene, although rust does this implicitly in a lot of areas for us
fn _derefrence() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;        // *x reads the heap value, so a = 1
    *x += 1;                // *x on the left-side modifies the heap value, so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let b: i32 = **r1;      // two dereferences get us to the heap value

    let r2: &i32 = &*x;     // r2 points to the heap value directly
    let c: i32 = *r2;       // so only one dereference is needed to read it
}