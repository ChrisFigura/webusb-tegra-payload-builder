use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;

/// The struct for managing the arguments passed to the program.
#[derive(StructOpt)]
#[structopt(name = "WebUSB Tegra Payload Builder", about = "Builds payloads for use in WebUSB from binary files")]
struct Args {
    #[structopt(parse(from_os_str), help = "The path to the input file")]
    path: PathBuf,

    #[structopt(short = "n", long = "name", help = "The name of the const in the generated js file")]
    name: Option<String>,

    #[structopt(short = "o", long = "output", help = "The path to the output file, stdout if not specified")]
    output: Option<PathBuf>
}

fn main() {
    // Get the arguments passed in from the command line.
    let argv = Args::from_args();

    // Try to read the contents of the input file.
    let content = match fs::read_to_string(&argv.path) {
        // If it was successful, read out the contents.
        Ok(content) => content,

        // If there was an error, exit with a nonzero status code.
        Err(error) => {
            eprintln!("\x1b[1;31merror: \x1b[0;m{}", error); 
            std::process::exit(1);
        }
    };

    // Get the name for the js variable.
    let name = match argv.name {
        None => "payload",
        Some(ref name) => name
    };

    // Create a string for storing the bytes.
    let mut bytes = String::new();

    // For each byte in the file, append it to the bytes string.
    for byte in content.as_bytes() {
        bytes.push_str(format!("0x{:02x?}, ", byte).as_str());
    }

    // If the file was empty, exit with a nonzero status code.
    if bytes.len() == 0 {
        eprintln!("\x1b[1;31merror: \x1b[0;mInput file was empty");
        std::process::exit(1);
    }

    // Format the bytes into the js const.
    let result = format!("const {} = new Uint8Array([{}]);\n", &name, &bytes[..bytes.len() - 2]);

    // Print or output the resulting string.
    match argv.output {
        // If no output was specified, print it to stdout.
        None => print!("{}", &result),
        // If an output was specified, try to write it to the file.
        Some(ref output) => match fs::write(output, &result) {
            // If it was successful, do nothing.
            Ok(_) => (),

            // If there was an error writing to the file, exit with a nonzero status code.
            Err(error) => {
                eprintln!("\x1b[1;31merror: \x1b[0;m{}", error); 
                std::process::exit(1);
            }
        }
    };
}
