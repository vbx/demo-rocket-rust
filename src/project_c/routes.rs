use rocket::get;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Projet C!"
}

#[get("/about")]
fn about() -> &'static str {
    "About Projet C"
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, about]
}
