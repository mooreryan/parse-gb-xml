use parse_gb_xml::Config;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();

    parse_gb_xml::run(config);
}
