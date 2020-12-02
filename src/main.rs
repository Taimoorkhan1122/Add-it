#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::http::RawStr;

#[get("/")]
fn hello() -> &'static str {
    return "Hello world from rust server"
}

#[get("/add-it/<num>")] //getting dynamic parameters

// =======Basic implementation==========

// fn index(num: i16) -> String {
//     let add = num + 5;
//     return format!("you entere {} and I add-it {}", num , add)
// }

//     =======Implementation with Error handling==========

/* 
To convert dynamic paths segments into desired type Rocket implents a trait
From_param when when a path contains a dynamic segment <param> where param 
has some type T that implements FromParam, T::from_param will be called.

We can use the returned values from Result<T, T::Error> to catch the erro
if for some reason From_param fails to convert the type.
*/
fn index(num: Result<i16 , &RawStr>) -> String { 
    match num {
        Ok(mut int_num) => {
            let num = int_num;
            int_num += 10;
            format!("you entered {:?} and I add-it {}", num, int_num)
        },
        Err(err_str) => {
            let slplitter: Vec<&str> = err_str.split("%20",).collect();
            let mut msg = String::from("");
            
            for words in slplitter {
                msg.push_str(words);
                msg.push(' ');    
            }
            format!("Not a Number: {}", msg)
        }
    }
    

}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}


