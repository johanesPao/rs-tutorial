// use std::fmt::Debug;

// fn simple_print<T: Debug>(value: T) {
//     println!("{:?}", value);
// }

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

trait Kendaraan {
    fn set_mil_per_galon(&mut self, mil_per_galon: i32) -> ();
    fn set_warna(&mut self, warna: String) -> ();
    fn set_kecepatan_tertinggi(&mut self, kecepatan_tertinggi: i32) -> ();
}

#[derive(Debug)]
struct Motor {
    mil_per_galon: i32,
    warna: String,
    kecepatan_tertinggi: i32
}

#[derive(Debug)]
struct Mobil {
    mil_per_galon: i32,
    warna: String,
    kecepatan_tertinggi: i32
}

impl Kendaraan for Motor {
    fn set_mil_per_galon(&mut self, mil_per_galon: i32) -> () {
        self.mil_per_galon = mil_per_galon;
    }
    fn set_warna(&mut self, warna: String) -> () {
        self.warna = warna;
    }
    fn set_kecepatan_tertinggi(&mut self, kecepatan_tertinggi: i32) -> () {
        self.kecepatan_tertinggi = kecepatan_tertinggi;
    }
}

impl Kendaraan for Mobil {
    fn set_mil_per_galon(&mut self, mil_per_galon: i32) -> () {
        self.mil_per_galon = mil_per_galon;
    }
    fn set_warna(&mut self, warna: String) -> () {
        self.warna = warna;
    }
    fn set_kecepatan_tertinggi(&mut self, kecepatan_tertinggi: i32) -> () {
        self.kecepatan_tertinggi = kecepatan_tertinggi;
    }
}

fn set_mil_per_galon(kendaraan: &mut impl Kendaraan, mil_per_galon: i32) {
    kendaraan.set_mil_per_galon(mil_per_galon)
}
fn set_warna(kendaraan: &mut impl Kendaraan, warna: String) {
    kendaraan.set_warna(warna)
}
fn set_kecepatan_tertinggi(kendaraan: &mut impl Kendaraan, kecepatan_tertinggi: i32) {
    kendaraan.set_kecepatan_tertinggi(kecepatan_tertinggi)
}

fn main() {

    let mut kawasaki = Motor {mil_per_galon: 20, warna: String::from("Black"), kecepatan_tertinggi: 250};
    let mut bmw = Mobil {mil_per_galon: 34, warna: String::from("Red"), kecepatan_tertinggi: 320};

    println!("Before Modification:");
    println!("{:?}", kawasaki);
    println!("{:?}", bmw);

    set_mil_per_galon(&mut kawasaki, 50);
    set_warna(&mut kawasaki, String::from("White"));
    set_kecepatan_tertinggi(&mut kawasaki, 150);
    set_mil_per_galon(&mut bmw, 80);
    set_warna(&mut bmw, String::from("Black"));
    set_kecepatan_tertinggi(&mut bmw, 370);

    println!("After Modification:");
    println!("{:?}", kawasaki);
    println!("{:?}", bmw);

    // let a_string = String::from("jPao");
    // simple_print(a_string);

    // let coord = Point {x: 5.0, y: 5.0};
    // let coord2 = Point {x: 1.0, y: 2.0};

    // let sum = coord + coord2;
    // println!("{:?}", sum);

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