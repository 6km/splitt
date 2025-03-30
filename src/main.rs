use clap::{Parser, command};
use std::io::{self, Read};

#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None)]
struct Cli {
    /// The text to split. If not provided, the tool reads from standard input (stdin).
    text: Option<String>,

    /// The separator used to split the text.
    #[arg(long, short = 's')]
    separator: String,

    /// Keep empty strings.
    #[arg(long = "keep-empty")]
    keep_empty: bool,

    /// Output the results as a raw array of strings.
    #[arg(long)]
    raw: bool,

    /// The output is printed in chunks; which means each part is printed separately.
    #[arg(long, short = 'c')]
    chunked: bool,
}

fn main() {
    // Parse arguments and options with 'clap'.
    let cli = Cli::parse();

    // The text to split.
    let text = match cli.text {
        Some(t) => t,
        None => {
            let mut stdin_buffer = String::new();
            let mut stdin = io::stdin();
            let _ = stdin.read_to_string(&mut stdin_buffer);

            stdin_buffer
        }
    };

    // The separator/delimiter.
    let separator = cli.separator;

    // Splitting the text into an array.
    let array: Vec<&str> = text
        .split(&separator)
        .filter(|str| {
            // remove empty strings if --keep-empty is not specified
            if cli.keep_empty {
                true
            } else {
                !str.is_empty()
            }
        })
        .collect();

    // If the user wants a raw array,
    if cli.raw {
        // print the raw array.
        // ex: ["part1", "part2", ...]
        println!("{:?}", array);

        // Stop the program.
        return;
    }

    // If the user wants chunked print messages,
    if cli.chunked {
        // print each part separately.
        // ex:
        // - println!(part1);
        // - println!(part2);
        // - println!(...);
        for chunk in array {
            println!("{}", chunk);
        }

        // Stop the program.
        return;
    }

    // Print the result in one message. Not in chunks.
    // ex:
    // - println!(all parts at once in a single message);
    let result = array.join("\n");
    println!("{}", result)
}
