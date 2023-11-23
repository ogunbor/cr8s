use clap::{Command, Arg};

extern crate cr8s;

fn main () {
    let matches = Command::new("Cr8s")
        .about("Cr8s commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("User management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a user with multiple roles attached")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                )
                .subcommand(
                    Command::new("list")
                        .about("List all available users")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID")
                        .arg(Arg::new("id").required(true))
                )
        ).get_matches();
    
    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => create_user(),
            Some(("list", sub_matches)) => list_users(),
            Some(("delete", sub_matches)) => delete_user(),
        },
        _ => {},
    }
}
