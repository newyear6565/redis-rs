extern crate redis;
use redis::Commands;

fn main() {
    let _tmp = match fetch_an_integer() {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err.to_string());
            -1.0
        },
    };

    // if _tmp == -1.0 {
    //     println!("err:{}", _tmp);
    // }
    println!("get value:{}", _tmp);
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
