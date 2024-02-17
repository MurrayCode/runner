use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct RunnerArgs {
    #[clap(subcommand)]
    entity: EntityType,
}

#[derive(Subcommand, Debug)]
enum EntityType {
    /// Manages different types of runs
    Run(RunCommand),
    /// Manages users
    User(UserCommand),
    /// Manages recorded runs
    Record(RecordCommand),
}

#[derive(Debug, Args)]
struct RunCommand {
    #[clap(subcommand)]
    subcmd: RunSubCommand,
}

#[derive(Debug, Subcommand)]
enum RunSubCommand {
    /// Creates a new run
    Create(CreateRun),
    /// Lists all runs
    List(ListRun),
    /// Shows details of a run
    Show(ShowRun),
    /// Deletes a run
    Delete(DeleteRun),
}

#[derive(Debug, Args)]
struct CreateRun {
    /// The distance in meters
    distance: f64,
    /// The duration in seconds
    duration: i32,
}

#[derive(Debug, Args)]
struct ListRun {
    /// The user id
    user_id: Option<i32>,
}

#[derive(Debug, Args)]
struct ShowRun {
    /// The run id
    id: i32,
}

#[derive(Debug, Args)]
struct DeleteRun {
    /// The run id
    id: i32,
}

#[derive(Debug, Args)]
struct UserCommand {
    #[clap(subcommand)]
    subcmd: UserSubCommand,
}

#[derive(Debug, Subcommand)]
enum UserSubCommand {
    /// Creates a new user
    Create(CreateUser),
    /// Lists all users
    List(ListUsers),
    /// Shows details of a user
    Show(ShowUser),
    /// Deletes a user
    Delete(DeleteUser),
}

#[derive(Debug, Args)]
struct CreateUser {
    /// The user name
    name: String,
}

#[derive(Debug, Args)]
struct ListUsers {}

#[derive(Debug, Args)]
struct ShowUser {
    /// The user id
    id: i32,
}

#[derive(Debug, Args)]
struct DeleteUser {
    /// The user id
    id: i32,
}

#[derive(Debug, Args)]
struct RecordCommand {
    #[clap(subcommand)]
    subcmd: RecordSubCommand,
}
#[derive(Debug, Subcommand)]
enum RecordSubCommand {
    /// Records a new run
    Create(CreateRecordedRun),
    /// Lists all recorded runs
    List(ListRecordedRuns),
    /// Shows details of a recorded run
    Show(ShowRecordedRun),
    /// Deletes a recorded run
    Delete(DeleteRecordedRun),
}

#[derive(Debug, Args)]
struct CreateRecordedRun {
    /// The user id
    user_id: i32,
    /// The distance in meters
    distance: f64,
    /// The duration in seconds
    duration: i32,
}

#[derive(Debug, Args)]
struct ListRecordedRuns {
    /// The user id of the runs you want to list
    user_id: i32,
}

#[derive(Debug, Args)]
struct ShowRecordedRun {
    /// The recorded run id
    id: i32,
}

#[derive(Debug, Args)]
struct DeleteRecordedRun {
    /// The recorded run id
    id: i32,
}
