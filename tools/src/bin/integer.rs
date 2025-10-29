use arrayvec::ArrayVec;
use clap::{Arg, ArgAction, Command};
use libtw2_packer::with_packer;
use libtw2_packer::Unpacker;
use libtw2_tools::unhexdump::Unhexdump;
use libtw2_tools::warn_stdout::Stdout;

fn main() {
    let matches = Command::new("Teeworlds variable-length integer encoding")
        .about(
            "Takes an integer and writes out its Teeworlds variable-length\
                or vice versa",
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .action(ArgAction::SetTrue)
                .help(
                    "Decode a hexdump instead, hexdump must be enclosed in pipes,\
                   sorry :(",
                ), // TODO
        )
        .arg(
            Arg::new("INTEGER")
                .help("The integer to de-/encode")
                .required(true),
        )
        .get_matches();
    let integer = matches.get_one::<String>("INTEGER").unwrap();
    if !matches.get_flag("decode") {
        let integer: i32 = integer.parse().expect("invalid input");
        let mut out: ArrayVec<u8, 32> = ArrayVec::new();
        out.extend(std::iter::repeat(0u8).take(out.capacity()));
        let len = {
            with_packer(out.as_mut_slice(), |mut p| {
                p.write_int(integer).unwrap();
                p.written().len()
            })
        };
        out.truncate(len);
        hexdump::hexdump(&out);
    } else {
        let mut un = Unhexdump::new();
        un.feed(integer.as_bytes()).unwrap();
        let encoded = un.into_inner().unwrap();
        let mut up = Unpacker::new(&encoded);
        println!("{}", up.read_int(&mut Stdout).unwrap());
        up.finish(&mut Stdout);
    }
}
