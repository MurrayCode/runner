use crate::args::{CreateUser, DeleteEntity, ShowEntity, UpdateUser, UserCommand, UserSubCommand};
use crate::db::establish_connection;
use crate::models::{NewUser, User as DBUser};
use diesel::prelude::*;

pub fn handle_user_command(user: UserCommand) {
    let command = user.subcmd;
    match command {
        UserSubCommand::Create(create) => {
            handle_create_user(create);
        }
        UserSubCommand::Update(update) => {
            handle_update_user(update);
        }
        UserSubCommand::List => {
            handle_list_users();
        }
        UserSubCommand::Show(show) => {
            handle_show_user(show);
        }
        UserSubCommand::Delete(delete) => {
            handle_delete_user(delete);
        }
    }
}

pub fn handle_create_user(user: CreateUser) {
    println!("Creating user: {:?}", user);
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let new_user = NewUser {
        username: &user.name,
        email: &user.email,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");
}

pub fn handle_update_user(user: UpdateUser) {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    println!("Updating user: {:?}", user.id);
    diesel::update(users.find(user.id))
        .set((username.eq(&user.name), email.eq(&user.email)))
        .execute(&mut connection)
        .expect(&format!("Unable to find user {}", user.id));
}

pub fn handle_list_users() {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let results = users.load::<DBUser>(&mut connection).unwrap();

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user);
    }
}

pub fn handle_show_user(user: ShowEntity) {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = users.find(user.id).first::<DBUser>(&mut connection);

    match result {
        Ok(user) => println!("{:?}", user),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn handle_delete_user(user: DeleteEntity) {
    use crate::schema::users::dsl::*;

    let mut connection = establish_connection();
    let result = diesel::delete(users.find(user.id)).execute(&mut connection);
    match result {
        Ok(_) => println!("Deleted user {}", user.id),
        Err(e) => println!("Error: {}", e),
    }
}
