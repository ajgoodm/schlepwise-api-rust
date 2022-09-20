use rocket;

use crate::api::handlers;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/households",
            routes![
                handlers::household::all_households,
                handlers::household::create_household,
                handlers::household::get_household,
                handlers::household::update_household,
                handlers::household::delete_household,
                handlers::family_member::all_family_members,
                handlers::family_member::create_family_member,
                handlers::family_member::get_family_member,
                handlers::family_member::delete_family_member,
            ],
        )
        .launch();
}