use clap::{Arg, Command, builder::PossibleValuesParser};
pub struct Args {
    pub sort_type: String,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("search-and-sort")
            .arg(
                Arg::new("sort_type")
                     .short('t')
                     .long("sort_type")
                     .help("Select the type of sort you would like to use ex) bubble-sort")
                     .action(clap::ArgAction::Set)
                     .value_parser(PossibleValuesParser::new(["bubble-sort"])) // Allowed values

            )
            .get_matches();

        let sort_type = matches
            .get_one::<String>("sort_type")
            .map(|s| s.as_str())
            .unwrap_or_default()
            .to_string();

        Self {
            sort_type
        }
    }
}