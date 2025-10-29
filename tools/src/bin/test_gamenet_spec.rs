use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

fn process(path: &Path) -> Result<(), Box<dyn Error>> {
    let _spec: libtw2_gamenet_spec::Spec = serde_json::from_slice(&fs::read(path)?)?;
    Ok(())
}

fn main() {
    use clap::Arg;
    use clap::Command;

    libtw2_logger::init();

    let matches = Command::new("Gamenet spec reader")
        .about("Reads a gamenet spec file and does nothing with it.")
        .arg(
            Arg::new("SPEC")
                .help("Sets the gamenet spec file to read")
                .required(true),
        )
        .get_matches();

    let path = Path::new(matches.get_one::<std::ffi::OsString>("SPEC").unwrap());

    match process(path) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}: {:?}", path.display(), err);
            process::exit(1);
        }
    }
}
