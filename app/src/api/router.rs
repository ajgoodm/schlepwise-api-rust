use rocket;

use crate::connection;
use crate::api::handlers;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/households",
            routes![
                handlers::household::all_households,
                handlers::household::create_household,
                handlers::household::get_household,
                handlers::household::update_household,
                handlers::household::delete_household,
            ],
        ).launch();
}
