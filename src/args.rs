use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct RunnerArgs {
    #[clap(subcommand)]
    pub entity: EntityType,
}

#[derive(Subcommand, Debug)]
pub enum EntityType {
    /// Manages different types of runs
    Run(RunCommand),
    /// Manages users
    User(UserCommand),
    /// Manages recorded runs
    Record(RecordCommand),
}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[clap(subcommand)]
    pub subcmd: RunSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum RunSubCommand {
    /// Creates a new run
    Create(CreateRun),
    /// Updates a run
    Update(UpdateRun),
    /// Lists all runs
    List,
    /// Shows details of a run
    Show(ShowEntity),
    /// Deletes a run
    Delete(DeleteEntity),
    /// Randomises the run
    Randomise,
}

#[derive(Debug, Args)]
pub struct CreateRun {
    /// The distance in meters
    pub distance: i32,
    /// The duration in seconds
    pub duration: i32,
}

#[derive(Debug, Args)]
pub struct UpdateRun {
    /// The id of the run to update
    pub id: i32,
    /// The new distance in meters
    pub distance: i32,
    /// The new duration in seconds
    pub duration: i32,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcmd: UserSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand {
    /// Creates a new user
    Create(CreateUser),
    /// Updates a user
    Update(UpdateUser),
    /// Lists all users
    List,
    /// Shows details of a user
    Show(ShowEntity),
    /// Deletes a user
    Delete(DeleteEntity),
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The user name
    pub name: String,
    /// The users email
    pub email: String,
}
#[derive(Debug, Args)]
pub struct UpdateUser {
    /// The id of the user to update
    pub id: i32,
    /// The new user name
    pub name: String,
    /// The new email
    pub email: String,
}
#[derive(Debug, Args)]
pub struct ShowEntity {
    /// The user id
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct DeleteEntity {
    /// The user id
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct RecordCommand {
    #[clap(subcommand)]
    pub subcmd: RecordSubCommand,
}
#[derive(Debug, Subcommand)]
pub enum RecordSubCommand {
    /// Records a new run
    Create(CreateRecordedRun),
    /// Updates a recorded run
    Update(UpdateRecordedRun),
    /// Lists all recorded runs
    List(ListRecordedRuns),
    /// Shows details of a recorded run
    Show(ShowEntity),
    /// Deletes a recorded run
    Delete(DeleteEntity),
}

#[derive(Debug, Args)]
pub struct CreateRecordedRun {
    /// The user id
    pub user_id: i32,
    /// The run id
    pub run_id: i32,
}

#[derive(Debug, Args)]
pub struct ListRecordedRuns {
    /// The user id of the runs you want to list
    pub user_id: i32,
}

#[derive(Debug, Args)]
pub struct UpdateRecordedRun {
    /// The id of the recorded run to update
    pub id: i32,
    /// The new run id
    pub run_id: i32,
}
