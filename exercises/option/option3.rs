// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), //"match" own the p by default, add ref keyword to borrow
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
