// first version 
//fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// tuple version
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct version
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main() {
    //let scale = 2;

    // let rect1 = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    //dbg!(&rect1);

    let sq = Rectangle::square(3);
    println!("Square: {}", sq.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    //println!("rect1 is {:#?}", rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


    // Method calls are syntactic sugar for function calls
    let mut r = Rectangle { 
        width: 1,
        height: 2
    };

    // equivalent calls
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    // equivalent calls
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    
    let rect = Rectangle {
        width: 0,
        height: 0
    };
    println!("{}", rect.area());
    
    let other_rect = Rectangle { width: 1, height: 1 };
    let max_rect = rect.max(other_rect);

}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }