// struct User {
//     active: bool,
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32, i32, i32);

// struct UnitStruct;

// struct Square {
//     width: u32,
//     height: u32,
// }

// impl Square {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn whats_my_width(&self) -> u32 {
//         self.width
//     }

//     fn change_width(&mut self, new_width: u32) {
//         self.width = new_width
//     } 
// }

// struct MyString<'a> {
//     text: &'a str,
// }

#[derive(Debug)]
struct Car {
    mpg: i32,
    color: String,
    top_speed: f32
}

impl Car {
    fn set_mpg(&mut self, new_mpg: i32) {
        self.mpg = new_mpg;
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }

    fn set_top_speed(&mut self, new_ts: f32) {
        self.top_speed = new_ts;
    }
}


fn main() {
    let mut bmw = Car{mpg: 2, color:String::from("red"), top_speed: 102.3};
    bmw.set_mpg(10);
    bmw.set_color(String::from("black"));
    bmw.set_top_speed(250.2);
    println!("{:?}", bmw);

    // let str1 = String::from("This is my string");
    // let x = MyString{text: str1.as_str()};
    // let s: &'static str = "I have a static lifetime";

    // let user1 = User{active: true, username: String::from("Tyler"), sign_in_count: 0};
    // println!("{}", user1.username);

    // let user2 = build_user(String::from("Tyler2"));
    // println!("{}", user2.username);

    // let cords = Coordinates(1,2,3);

    // let mut sq = Square{width: 5, height: 5};
    // println!("{}", sq.area());
    // println!("{}", sq.whats_my_width());
    // sq.change_width(10);
    // println!("{}", sq.whats_my_width());

    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // } //x is dropped

    // println!("{}", r); // 'a

    // &i32
    // &'a i32
    // &'a mut i32
}

// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
//     y
// } //'a for one parameter, 'b for second parameter