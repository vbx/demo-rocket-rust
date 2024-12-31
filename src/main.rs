#[macro_use] extern crate rocket;

#[cfg(feature = "project_a")]
mod project_a;

#[cfg(feature = "project_b")]
mod project_b;

#[cfg(feature = "project_c")]
mod project_c;

mod lib;

#[launch]
fn rocket() -> _ {
    let mut rocket = rocket::build();

    #[cfg(feature = "project_a")]
    {
        rocket = rocket.mount("/project_a", project_a::routes());
    }

    #[cfg(feature = "project_b")]
    {
        rocket = rocket.mount("/project_b", project_b::routes());
    }

    #[cfg(feature = "project_c")]
    {
        rocket = rocket.mount("/project_c", project_c::routes());
    }

    rocket
}
