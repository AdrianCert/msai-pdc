use super::models::Mechanism;
use super::models::Protocol;

use clap::builder::TypedValueParser as _;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(term_width = 0)]
pub struct CliArguments {
    #[arg(
        long,
        default_value_t = Protocol::TCP,
        short = 'p',
        value_parser = clap::builder::PossibleValuesParser::new(["udp", "tcp"])
        .map(|s| s.parse::<Protocol>().unwrap()),
    )]
    pub protocol: Protocol,

    #[arg(
        long,
        default_value_t = Mechanism::Stream,
        short = 'm',
        value_parser = clap::builder::PossibleValuesParser::new(["stream", "sync"])
        .map(|s| s.parse::<Mechanism>().unwrap()),
    )]
    pub mechanism: Mechanism,

    #[arg(
        long,
        short = 's',
        value_parser =  clap::builder::RangedU64ValueParser::<u16>::new().range(1..65535)
    )]
    pub size: u16,
    // as we do 2 binary we don't need this
    // #[arg(value_parser = clap::builder::PossibleValuesParser::new(["client", "server"])
    // .map(|s| s.parse::<values::Role>().unwrap()))]
    // pub role: values::Role,
}
