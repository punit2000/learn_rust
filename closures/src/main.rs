use std::{thread, time::Duration};

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]

enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl  Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red =0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1
            }
        }
        if num_red > num_blue{
            ShirtColor::Red
        } else {
            ShirtColor:: Blue
        }
    }

}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} get {:?}",
        user_pref1, giveaway1 
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} get {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    expensive_closure(500);

    let example_closure = |x| x;

    //let s = example_closure(String::from("hello"));
    let n = example_closure(5); 


    let mut list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    //let only_borrows = || println!("From closure: {list:?}");
    // let mut borrows_mutably = || list.push(7);
    //println!("Before calling closure : {list:?}");
    //only_borrows();
    // borrows_mutably();
    // println!("After calling closure: {list:?}");

    thread::spawn(move || println!("Fron thread: {list:?}"))
        .join()
        .unwrap();

    // println!("After calling closure: {list:?}");

    let mut list = [
        Rectangle { width:10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // list.sort_by_key(|r| r.width );
    // println!("{list:#?}");

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    let mut num_sort_operations = 0;

    list.sort_by_key(|r|{
        num_sort_operations += 1;
        r.width
    });
    println!("{list:?}, sorted in {num_sort_operations} operations");
}


