#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;


        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }
    }
}

use std::net::TcpListener;


fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    let mut only_borrows = || println!("From closure {:?}", list.push(8));

    //println!("Before calling closure {:?}", list);
    only_borrows();
    println!("After calling closure {:?}", list);


        //let listener = TcpListener::bind("127.0.0.1:4001").unwrap();
        //for stream in listener.incoming() {
        //    println!("hallo {:?}", stream);
        //}
        //This listens properly, but it blocks

        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
        };

        let user_pref1 = Some(ShirtColor::Blue);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with pref1 {:?} gets {:?}", user_pref1, giveaway1
            );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with pref2 {:?} gets {:?}", user_pref2, giveaway2
        );

}
