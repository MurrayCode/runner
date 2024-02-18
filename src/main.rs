#[macro_use]
extern crate diesel;
extern crate dotenv;

mod args;
mod db;
mod handlers;
mod models;
mod schema;

use args::{EntityType, RunnerArgs};
use clap::Parser;

fn main() {
    let args = RunnerArgs::parse();
    match args.entity {
        EntityType::Run(run) => handlers::run_handler::handle_run_command(run),
        EntityType::User(user) => handlers::user_handler::handle_user_command(user),
        EntityType::Record(record) => handlers::record_handler::handle_record_command(record),
    }
}
