use rocket::get;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Projet A!"
}

#[get("/about")]
fn about() -> &'static str {
    "About Projet A"
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, about]
}
