use rocket::get;
use crate::lib::common; // Import de la fonction `common`

#[get("/")]
fn index() -> String {
    let random_number = common::generate_random_number(100);
    format!("Hello Projet B! Random number: {}", random_number)
}

#[get("/about")]
fn about() -> &'static str {
    "About Projet B"
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, about]
}
