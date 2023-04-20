use std::fmt::Debug;

fn simple_print<T: Debug>(value: T) {
    println!("{:?}", value);
}

// struct Point<T, U> {
//     x: T, //i32
//     y: U  //i32
// }

// trait Overview {
//     fn overview(&self) -> String {
//         String::from("This is a Rust course!")
//     }
// }

// struct Course {
//     headline: String,
//     author: String,
// }

// impl Drop for Course {
//     fn drop(&mut self) {
//         println!("Dropping: {}", self.author)
//     }
// }

// struct AnotherCourse {
//     headline: String,
//     author: String,
// }

// impl Overview for Course {
    
// }

// impl Overview for AnotherCourse {
//     fn overview(&self) -> String {
//         format!("{}, {}", self.author, self.headline)
//     }
// }

// use std::ops::Add;

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T
// }

// impl<T> Add for Point<T>
//     where
//     T: Add<Output = T> {
//         type Output = Self;
//         fn add(self, rhs: Self) -> Self {
//             Point {
//                 x: self.x + rhs.x,
//                 y: self.y + rhs.y
//             }
//         }
//     }

fn main() {

    let a_string = String::from("jPao");
    simple_print(a_string);

    // let coord = Point {x: 5.0, y: 5.0};
    // let coord2 = Point {x: 1.0, y: 2.0};

    // let sum = coord + coord2;
    // println!("{:?}", sum);

    // let mut kawasaki = Motorcycle {mpg: 20, color: String::from("Black"), top_speed: 250};
    // let mut bmw = Car {mpg: 34, color: String::from("Red"), top_speed: 320};

    // println!("Before Modification:");
    // println!("{:?}", kawasaki);
    // println!("{:?}", bmw);

    // kawasaki.set_mpg(50);
    // kawasaki.set_color(String::from("White"));
    // kawasaki.set_top_speed(150);
    // bmw.set_mpg(80);
    // bmw.set_color(String::from("Black"));
    // bmw.set_top_speed(370);

    // println!("After Modification:");
    // println!("{:?}", kawasaki);
    // println!("{:?}", bmw);

    // let coord = Point {x: 5.0, y: 5.0};
    // let coord2 = Point {x: 'x', y: 5.0};

    // let course1 = Course{headline: String::from("Headline!"), author: String::from("Tyler")};
    // let course2 = AnotherCourse{headline: String::from("Another Headline!"), author: String::from("Another Tyler")};

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // drop(course1);
}

// fn call_overview(item: &impl Overview) {
//     println!("Overview: {}", item.overview());
// }

// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)

// trait Clone: Sized {
//     fn clone(&self) -> self;
//     fn clone_from(&mut self, source: &self) {
//         *self = source.clone()
//     }
// }


// trait Set {
//     fn set_mpg(&mut self, new_mpg: i32) -> ();
//     fn set_color(&mut self, new_color: String) -> ();
//     fn set_top_speed(&mut self, new_top_speed: i32) -> ();
// }

// #[derive(Debug)]
// struct Motorcycle {
//     mpg: i32,
//     color: String,
//     top_speed: i32
// }

// #[derive(Debug)]
// struct Car {
//     mpg: i32,
//     color: String,
//     top_speed: i32
// }

// impl Set for Motorcycle {
//     fn set_mpg(&mut self, new_mpg: i32) -> () {
//         self.mpg = new_mpg;
//     }
//     fn set_color(&mut self, new_color: String) -> () {
//         self.color = new_color;
//     }
//     fn set_top_speed(&mut self, new_top_speed: i32) -> () {
//         self.top_speed = new_top_speed;
//     }
// }

// impl Set for Car {
//     fn set_mpg(&mut self, new_mpg: i32) -> () {
//         self.mpg = new_mpg;
//     }
//     fn set_color(&mut self, new_color: String) -> () {
//         self.color = new_color;
//     }
//     fn set_top_speed(&mut self, new_top_speed: i32) -> () {
//         self.top_speed = new_top_speed;
//     }
// }