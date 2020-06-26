#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/add-it/<num>/<name>")]
fn index(num: u64 , name: String) -> String {
    let add = num + 5;
    return format!("you entere {} and I add-it {}", num , add)

// implementing Even or Odd feature
    // if num%2 == 0 {
    //     return format!("The number {} is Even", num)
    // } else {
    //     return format!("The number {} is Odd", num)
    // }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}