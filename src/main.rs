#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
use rocket::http::RawStr;
use rocket::*;


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    tags: Vec<String>,
}


#[get("/")]
fn hello() -> rocket::response::content::Html<String> {
    let person1 = Person{
        name: "Taimoor Khan".to_owned(),
        age: 21,
        tags:   vec!["coding".to_string(), "watching movies".to_string()],
    };
    let response = format!("
    <h1 style='color:blue'>Welcom to Rust Add-it By Taimoor khan</h1></br>
    <table style='border: 1px solid #000; width: 50%'>
      <tr style='border: 1px solid #000'>
        <th style='border: 1px solid #000'>Name</th>
        <th style='border: 1px solid #000'>Age</th>
        <th style='border: 1px solid #000'>Hobies</th>
      </tr>
      <tr>
        <td style='border: 1px solid #000'>{:?}</td>
        <td style='border: 1px solid #000'>{:?}</td>
        <td style='border: 1px solid #000'>{:?}</td>
      </tr>
    </table>
    ", person1.name, person1.age, person1.tags);
    // return content::Json("{'name' : 'Hello world'}")
    return rocket::response::content::Html(response)
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
fn index(num: Result<i16 , &RawStr>) -> response::content::Json<String> { 
    match num {
        Ok(mut int_num) => {
            let num = int_num;
            int_num += 10;
            let result = format!("Your Value {} : Response {}", num, int_num);
            response::content::Json(result)
        },
        Err(err_str) => {
            let mut slplitter: Vec<&str> = err_str.split("%20",).collect();
            let mut msg = String::from("");
            println!("{}", err_str);
            
            for words in slplitter {
                msg.push_str(words);
                msg.push(' ');    
            }
            let err_msg = format!("Not a Number: {}", msg);
            response::content::Json(err_msg)
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello]).launch();
}


