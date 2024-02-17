use crate::args::{
    CreateRun, DeleteEntity, ListRuns, RunCommand, RunSubCommand, ShowEntity, UpdateRun,
};

pub fn handle_run_command(run: RunCommand) {
    let command = run.subcmd;
    match command {
        RunSubCommand::Create(create) => {
            handle_create_run(create);
        }
        RunSubCommand::Update(update) => {
            handle_update_run(update);
        }
        RunSubCommand::List(list) => {
            handle_list_runs(list);
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
    println!("Creating run: {:?}", run)
}

pub fn handle_update_run(run: UpdateRun) {
    println!("Updating run: {:?}", run.id)
}

pub fn handle_list_runs(run: ListRuns) {
    println!("Listing runs")
}

pub fn handle_show_run(run: ShowEntity) {
    println!("Showing run: {:?}", run.id)
}

pub fn handle_delete_run(run: DeleteEntity) {
    println!("Deleting run: {:?}", run.id)
}
