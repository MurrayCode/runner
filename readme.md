# Runner CLI App

Weekend (17/18th Feb 2024) upskilling project to look into writing CLI's in Rust + handling SQL with Diesel. 

![goals](images/goals.png)

## Usage 

```cargo  build && cargo install --path .```

## Base Commands

Usage: runner 'command'

Commands:
  * run   -  Manages different types of runs
  * user  -  Manages users
  * record - Manages recorded runs
  * help  -  Print this message or the help of the given subcommand(s)

Options:
*  -h, --help     Print help
* -V, --version  Print version

## Sub Commands

### Run 
---
Manages different types of runs

Usage: runner run 'COMMAND'

Commands:
  * create  -   Creates a new run
  * update  -   Updates a run
  * list    -   Lists all runs
  * show    -   Shows details of a run
  * delete  -   Deletes a run
  * randomise - Randomises the run
  * help   -    Print this message or the help of the given  subcommand(s)

Options:
 * -h, --help  Print help

### User
---
Manages users

Usage: runner user 'COMMAND'

Commands:
  * create -  Creates a new user
  * update -  Updates a user
  * list   -  Lists all users
  * show   -  Shows details of a user
  * delete -  Deletes a user
  * help   -  Print this message or the help of the given subcommand(s)

Options:
 * -h, --help  Print help

### Record
--- 

Manages recorded runs

Usage: runner record 'COMMAND'

Commands:
  * create - Records a new run
  * update - Updates a recorded run
  * list   - Lists all recorded runs
  * show   - Shows details of a recorded run
  * delete - Deletes a recorded run
  * help   - Print this message or the help of the given subcommand(s)

Options:
*  -h, --help  Print help
