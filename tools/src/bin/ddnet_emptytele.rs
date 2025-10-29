use libtw2_map::Error;
use std::path::Path;
use std::process;

fn tele_tile_name(index: u8) -> Option<&'static str> {
    Some(match index {
        10 => "TELEINEVIL",
        14 => "TELEINWEAPON",
        15 => "TELEINHOOK",
        26 => "TELEIN",
        27 => "TELEOUT",
        29 => "TELECHECK",
        30 => "TELECHECKOUT",
        31 => "TELECHECKIN",
        63 => "TELECHECKINEVIL",
        _ => return None,
    })
}

fn process(path: &Path) -> Result<(), Error> {
    let mut map = libtw2_map::Reader::open(path)?;
    let game_layers = map.game_layers()?;

    let tele = if let Some(t) = game_layers.teleport() {
        t
    } else {
        return Ok(());
    };
    let tele_tiles = map.tele_layer_tiles(tele)?;
    for ((y, x), &t) in tele_tiles.indexed_iter() {
        if t.index != 0 && t.number == 0 {
            if let Some(name) = tele_tile_name(t.index) {
                println!("{}: {}: ({}, {})", path.display(), name, x, y);
            } else {
                println!("{}: unknown ({}): ({}, {})", path.display(), t.index, x, y);
            }
        }
    }
    Ok(())
}

fn main() {
    use clap::Arg;
    use clap::Command;
    use std::ffi::OsString;

    libtw2_logger::init();

    let matches = Command::new("DDNet teleporter scanner")
        .about("Scans map files for weird teleporters.")
        .arg(
            Arg::new("MAP")
                .help("Sets the map file to analyse")
                .num_args(1..)
                .required(true),
        )
        .get_matches();

    let maps = matches.get_many::<OsString>("MAP").unwrap();

    let mut error = false;
    for map in maps {
        let map = Path::new(map);
        match process(map) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{}: {:?}", map.display(), err);
                error = true;
            }
        }
    }
    if error {
        process::exit(1);
    }
}
