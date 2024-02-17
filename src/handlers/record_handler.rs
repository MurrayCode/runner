use crate::args::{
    CreateRecordedRun, DeleteEntity, ListRecordedRuns, RecordCommand, RecordSubCommand, ShowEntity,
    UpdateRecordedRun,
};

pub fn handle_record_command(record: RecordCommand) {
    let command = record.subcmd;
    match command {
        RecordSubCommand::Create(create) => {
            handle_create_record(create);
        }
        RecordSubCommand::Update(update) => {
            handle_update_record(update);
        }
        RecordSubCommand::List(list) => {
            handle_list_record(list);
        }
        RecordSubCommand::Show(show) => {
            handle_show_record(show);
        }
        RecordSubCommand::Delete(delete) => {
            handle_delete_record(delete);
        }
    }
}

pub fn handle_create_record(record: CreateRecordedRun) {
    println!("Creating record: {:?}", record)
}

pub fn handle_update_record(record: UpdateRecordedRun) {
    println!("Updating record: {:?}", record.id)
}

pub fn handle_list_record(record: ListRecordedRuns) {
    println!("Listing records")
}

pub fn handle_show_record(record: ShowEntity) {
    println!("Showing record: {:?}", record.id)
}

pub fn handle_delete_record(record: DeleteEntity) {
    println!("Deleting record: {:?}", record.id)
}
