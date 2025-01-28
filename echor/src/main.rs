use clap::{Arg, Command};
fn main() {
    let _basic_info = Command::new("echor")
        .version("0.1.0")
        .author("Ziyang XU, ziyang.xu@telecom-paris.fr")
        .about("Echo with return code")
        .arg(
            Arg::new("Test")
                .short('t')
                .long("test")
                .help("Test mode help message"),
        )
        .get_matches();
}
