use crate::args::{CreateUser, DeleteEntity, ShowEntity, UpdateUser, UserCommand, UserSubCommand};

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
    println!("Creating user: {:?}", user)
}

pub fn handle_update_user(user: UpdateUser) {
    println!("Updating user: {:?}", user.id)
}

pub fn handle_list_users() {
    println!("Listing users")
}

pub fn handle_show_user(user: ShowEntity) {
    println!("Showing user: {:?}", user.id)
}

pub fn handle_delete_user(user: DeleteEntity) {
    println!("Deleting user: {:?}", user.id)
}
