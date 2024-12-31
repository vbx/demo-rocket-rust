mod routes;

pub fn routes() -> Vec<rocket::Route> {
    routes::get_routes()
}
