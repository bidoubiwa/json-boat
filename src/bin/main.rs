use json_boat::json_combine;
use std::env::args;
use std::io;

fn main() -> io::Result<()> {
    let arguments: Vec<_> = args().skip(1).collect();

    if arguments.iter().any(|arg| arg == "--help") || arguments.is_empty() {
        println!(
            "json-boat: 0.1.0
    USAGE:
      json-boat [files]
    "
        );
        return Ok(());
    }

    let stdout = io::stdout();
    let stdout = stdout.lock();

    json_combine(arguments, stdout);
    // dbg!(arguments);
    Ok(())
}
