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
    dna: [u8; 16],
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

impl User {
    fn get_name(&mut self) -> &String {
        self.age = 100;
        &self.name
    }

    fn build() -> Self {
        User {
            name: String::from(""),
            age: 0,
            active: false,
            name_part: String::from(""),
            dna: [0; 16],
        }
    }
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


fn main() {

    let list_of_statuses: Vec<Status> =
    (0u32..20)
    .map(Status::Value)
    .collect();

    println!("{:?}",list_of_statuses);
    let a : String = "abcdefg".to_string();
    let b : &str = &a[1..2];
    let cdddd = String::from("sff  fff ggg");
    let sfsfs:Vec<&str> = cdddd.split(' ').collect();
    println!("{}",b);

    for i in 1..5 {
        println!("### {}", i);
    }

    let mut a = vec![1,2,3,4];
    for b in a.iter_mut() {
        *b = *b + 2;
    }
    println!("{:?}", a);

    // let sum: Vec<_> = Counter::new().zip(Counter::new().skip(1))
    //                              .map(|(a, b)| a * b)
    //                              .filter(|x| x % 3 == 0)
    //                              .collect();
    let sum: Vec<_> = Counter::new().zip(Counter::new().skip(2))
                                 .map(|(a, b)| (a ,b))
                                //  .filter(|x| x % 3 == 0)
                                 .collect();
    // assert_eq!(18, sum);
    println!("vec is {:?}", sum);

    let test_closure = |num| num+2;
    let num = 40;
    let test = match num {
        1 => {
                println!("1dd");2
        },
        2 => {println!("2dd");3},
        3 => {println!("3dd");4},
        4 => {println!("4dd");5},
        _ => {println!("other");6},
        // _ => panic!("dfs"),
    };

    // let mut sd = "dsf";
    // sd.push_str("_dssfsdf");
    // let mut i = &sd[0];
    // *i = 'f';

    // println!("#####  {}",sd);

    println!("closure: {}", test_closure(3));
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
        dna:[0;16]
    };

    user1.age = 81;
    user1.dna[3] = 3;
    println!("## name is {}", user1.get_name());

    println!("{:#?}", user1);
    println!("{:?}", user1);
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
