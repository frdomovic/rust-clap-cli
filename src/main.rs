use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
    .subcommand(Command::new("register-person")
        .arg(
            Arg::new("firstname")
                .short('f')
                .long("first")
                .aliases(["fname"])
                .required(true)
                .help("The person's first name")
                .conflicts_with("lastname")
        )
        .arg(
            Arg::new("lastname")
                .short('l')
                .long("last")
                .aliases(["lname", "last-name"])
                .required(true)
                .help("This argument takes person's last name")
        )
    )
    .subcommand(Command::new("register-pet")
        .arg(
            Arg::new("petname")
                .long("pet-name")
                .short('n')
                .required(true)
        )
    )
    .about("This application registers people with their doctor's office")
    // .group(
    //     ArgGroup::new("person-register")
    //         .arg("firstname")
    //         .arg("lastname")
    // )
    // .group(
    //     ArgGroup::new("dog-register")
    //     .arg("petname")
// )
    .arg(
        Arg::new("fluf")
            .long("fluf")
            .help("random argument")
    )
    .get_matches();

    //println!("fluffy: {}", match_result.get_one::<String>("petname").unwrap_or(&"NO PET NAME".to_string()))
    let pet_args = match_result.subcommand_matches("register-pet");
    println!("does petname exists {}", pet_args.unwrap().get_one::<String>("petname").unwrap());
}
