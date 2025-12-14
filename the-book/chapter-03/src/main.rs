const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // mutability();
    // shadowing();
    // test_subtraction();
    // named_tuple();
    // another_function(5);
    // print_labeled_measurement(5, 'h');
    // for_sum();
    while_sum();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn test_subtraction() {
    //let x: u8 = 5;
    //let y: u8 = x - 6;

    //println!("{y}");
}

fn named_tuple() {
    let tup = (500, 'c', 1.3);
    let (x, c, y) = tup;

    println!("{}, {}, {}", x, c, y);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn while_sum() {
    const count: usize = 10;
    let a = [5; count];

    let mut sum = 0;

    let mut index = 0;
    while index < count {
        sum += a[index];
        index += 1;
    }

    println!("{sum}");
}

fn for_sum() {
    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }

    println!("{sum}");
}