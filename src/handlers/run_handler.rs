use crate::args::{CreateRun, DeleteEntity, RunCommand, RunSubCommand, ShowEntity, UpdateRun};
use crate::db::establish_connection;
use crate::models::{NewRun, Run as DBRun};
use diesel::prelude::*;

pub fn handle_run_command(run: RunCommand) {
    let command = run.subcmd;
    match command {
        RunSubCommand::Create(create) => {
            handle_create_run(create);
        }
        RunSubCommand::Update(update) => {
            handle_update_run(update);
        }
        RunSubCommand::List => {
            handle_list_runs();
        }
        RunSubCommand::Show(show) => {
            handle_show_run(show);
        }
        RunSubCommand::Delete(delete) => {
            handle_delete_run(delete);
        }
    }
}

pub fn handle_create_run(run: CreateRun) {
    println!("Creating run: {:?}", run);
    use crate::schema::runs::dsl::*;

    let mut connection = establish_connection();
    let new_run = NewRun {
        distance: &run.distance,
        duration: &run.duration,
    };

    diesel::insert_into(runs)
        .values(&new_run)
        .execute(&mut connection)
        .expect("Error saving new run");
}

pub fn handle_update_run(run: UpdateRun) {
    use crate::schema::runs::dsl::*;
    let mut connection = establish_connection();

    println!("Updating run: {:?}", run.id);

    diesel::update(runs.find(run.id))
        .set((distance.eq(&run.distance), duration.eq(&run.duration)))
        .execute(&mut connection)
        .expect(&format!("Unable to find run {}", run.id));
}

pub fn handle_list_runs() {
    use crate::schema::runs::dsl::*;
    let mut connection = establish_connection();
    println!("Listing runs");
    let results = runs.load::<DBRun>(&mut connection).unwrap();
    for run in results {
        println!("{:?}", run);
    }
}

pub fn handle_show_run(run: ShowEntity) {
    use crate::schema::runs::dsl::*;
    let mut connection = establish_connection();
    println!("Showing run: {:?}", run.id);
    let result = runs.find(run.id).first::<DBRun>(&mut connection);
    match result {
        Ok(run) => println!("{:?}", run),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn handle_delete_run(run: DeleteEntity) {
    use crate::schema::runs::dsl::*;
    let mut connection = establish_connection();
    println!("Deleting run: {:?}", run.id);
    diesel::delete(runs.find(run.id))
        .execute(&mut connection)
        .expect(&format!("Unable to find run {}", run.id));
}
