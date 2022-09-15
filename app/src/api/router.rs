use rocket;

use crate::connection;
use crate::api;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/households",
            routes![
                api::handler::all_households,
                api::handler::create_household,
                api::handler::get_household,
                api::handler::update_household,
                api::handler::delete_household,
            ],
        ).launch();
}
