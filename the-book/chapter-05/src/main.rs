fn main() {
    // example1();
    // example2();
    example3();
}

fn print_area(area: u32) {
    println!(
        "\n\tThe area of the rectangle is {} square pixels :)\n",
        area
    ); 
}

// --------- No Structure --------- //
fn example1() {
    let width = 30;
    let height = 50;

    let area = area1(width, height);
    print_area(area);
}

fn area1(width: u32, height: u32) -> u32 {
    return width * height;
}

// ------ A little Structure ------ //
fn example2() {
    let rect = (30, 50);
    let area = area2(rect);
    print_area(area);
}

fn area2(rect: (u32, u32)) -> u32 {
    return rect.0 * rect.1;
}


// ------ A lottle Structure ------ //
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn example3() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let square = Rectangle::square(12);

    println!("\n\tArea of rect: {} \n\tArea of square: {}", rect.area(), square.area());
}

