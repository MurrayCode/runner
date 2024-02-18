use crate::args::{
    CreateRecordedRun, DeleteEntity, ListRecordedRuns, RecordCommand, RecordSubCommand, ShowEntity,
    UpdateRecordedRun,
};
use crate::db::establish_connection;
use crate::models::{NewRecord, Record as DBRun};
use diesel::prelude::*;

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
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();

    println!("Creating record: {:?}", record);
    let new_record = NewRecord {
        user_id: &record.user_id,
        run_id: &record.run_id,
        distance: &record.distance,
        duration: &record.duration,
    };

    diesel::insert_into(records)
        .values(&new_record)
        .execute(&mut connection)
        .expect("Error saving new record");
}

pub fn handle_update_record(record: UpdateRecordedRun) {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();

    println!("Updating record: {:?}", record.id);
    diesel::update(records.find(record.id))
        .set((distance.eq(&record.distance), duration.eq(&record.duration)))
        .execute(&mut connection)
        .expect(&format!("Unable to find record {}", record.id));
}

pub fn handle_list_record(record: ListRecordedRuns) {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();
    println!("Listing records for user: {:?}", record.user_id);
    let results = records
        .filter(user_id.eq(record.user_id))
        .load::<DBRun>(&mut connection)
        .unwrap();
    for record in results {
        println!("{:?}", record);
    }
}

pub fn handle_show_record(record: ShowEntity) {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();
    println!("Showing record: {:?}", record.id);

    let result = records.find(record.id).first::<DBRun>(&mut connection);
    match result {
        Ok(record) => println!("{:?}", record),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn handle_delete_record(record: DeleteEntity) {
    use crate::schema::records::dsl::*;
    let mut connection = establish_connection();
    println!("Deleting record: {:?}", record.id);
    let result = diesel::delete(records.find(record.id)).execute(&mut connection);
    match result {
        Ok(_) => println!("Deleted record {}", record.id),
        Err(e) => println!("Error: {}", e),
    }
}
