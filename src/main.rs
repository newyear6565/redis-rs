// extern crate redis;
mod common;
mod ui;
use redis::Commands;
use crate::common::service::say_hello;

enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Start(bool),
    Continue(String),
    Num(i32),
    Float(f32),
}

impl Message {
    fn do_sth(&self) {
        match self {
            Message::Quit => println!("our app quit"),
            Message::Move{x, y} => println!("move to {}, {}", x, y),
            Message::Start(v) => {
                if *v {
                    println!("Total start");
                } else {
                    println!("start without init");
                }
            },
            Message::Continue(v) => println!("message {}", v),
            Message::Num(v) => {
                if *v == 100 {
                    println!("perfect!!");
                } else {
                    println!("sth not good");
                }
            },
            _ => (),
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    active: bool,
    name_part: String,
}

impl User {
    fn get_name(&mut self) -> &String {
        self.age = 100;
        &self.name
    }
}

fn main() {
    let num = 40;
    let test = match num {
        1 => {
                println!("1dd");2
        },
        2 => {println!("2dd");3},
        3 => {println!("3dd");4},
        4 => {println!("4dd");5},
        _ => {println!("other");6},
    };

    for num in (1..4).rev() {
        println!("{}", num);
    }
    let name = String::from("test dd");
    let name_part = String::from("part name");
    let age:u16 = 20;

    let mut user1 = User {
        name,
        age,
        active: true,
        name_part,
    };

    user1.age = 81;
    println!("## name is {}", user1.get_name());

    println!("{:#?}", user1);
    println!("test is {}", test);
    println!("String is {}", &get_string());

    let msg:Message = Message::Continue(String::from("do what?"));
    msg.do_sth();
    let msg:Message = Message::Move{x:1, y:2};
    msg.do_sth();
    let msg:Message = Message::Start(true);
    msg.do_sth();
    let msg:Message = Message::Start(false);
    msg.do_sth();
    let msg:Message = Message::Quit;
    msg.do_sth();
    let msg:Message = Message::Num(100i32);
    msg.do_sth();
    let msg:Message = Message::Num(99);
    msg.do_sth();
    let msg:Message = Message::Float(3.33);
    if let Message::Float(t) = msg {
        println!("{} the same !!!", t);
    }

    say_hello::say();
    ui::get_ui_id();
    // let _tmp = match fetch_an_integer() {
    //     Ok(value) => value,
    //     Err(err) => {
    //         println!("{}", err.to_string());
    //         -1.0
    //     },
    // };

    // println!("get value:{}", _tmp);
}

fn get_string() -> String {
    String::from("dsfs")
}

fn fetch_an_integer() -> redis::RedisResult<f64> {
    // connect to redis
    let client = redis::Client::open("redis://:foobared@127.0.0.1:9999/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set_ex("my_key", 42.9, 10)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}
