// TODO: IDK what these macros do tbh
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket_contrib::{serve::StaticFiles, templates::Template};

mod utils;


//This attribute generates a route using the following function
// This is later called when applicable by the application
#[get("/")]
fn index() -> Template {
    Template::render("home", ())
}

// same as before but accepts an optional paremeter
#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in: {}", cmd);

    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "yt" => utils::youtube::construct_youtube_url(&cmd),
        "r" => utils::reddit::construct_reddit_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "am" => utils::amazon::construct_amazon_url(&cmd),
        "pi" => utils::pinterest::construct_pinterest_url(&cmd),
        "pr" => utils::protonmail::construct_proton_url(&cmd),
        "sof" => utils::stackoverflow::construct_stackoverflow_url(&cmd),
        "mf" => utils::mobafire::construct_mobafire_url(&cmd),
        "poro" => utils::porofessor::construct_porofessor_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };
    Redirect::to(redirect_url)
}

fn main() {
    // Creates an instance of the rocket webserver
    // Prepares the predefined routes with a base url of "/"
    // And starts the web application
    rocket::ignite()
        .mount("/resources", StaticFiles::from("resources"))
        .mount("/", routes![index, search])
        .attach(Template::fairing())
        .launch();
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_search_query() {
        let query = "test & best".to_string();

        search(query);
    }
}
