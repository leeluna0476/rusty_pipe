#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let area1 = area_two_var(width1, height1);
    println!("{area1}");

    let area2 = area_tuple( &(40, 20) );
    println!("{area2}");

    let rectangle = Rectangle {
        width: 10,
        height: 20
    };

    let area3 = area_struct(&rectangle);
    println!("{area3}");
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);

    let area4 = Rectangle {
        width: dbg!(30 * width1),
        height: 40
    };

    dbg!(area4);
}

fn area_two_var(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: &(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
