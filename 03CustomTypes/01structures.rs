#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32
}


#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left: tl, bottom_right: br} = rect;
    let Point { x: x1, y: y1 } = tl;
    let Point { x: x2, y: y2 } = br;
    return (x2-x1) * (y2-y1)
}

fn square(point: Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: point.x,
            y: point.y
        },
        bottom_right: Point {
            x: point.x + width,
            y: point.y + width
        }
    }
}


fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    println!("Point coordinates: ({}, {})", point.x, point.y);
    
    let bottom_right = Point { x: 5.2, ..point };

    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 1.0},
        bottom_right: Point { x: 2.0, y: 4.0}
    };

    println!("Rectangle area: {}", rect_area(rect));

    println!("Square: {}", square(Point {x: 0.0, y: 2.3 }, 5.0).top_left.x)


}