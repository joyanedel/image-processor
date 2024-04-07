use std::{env, str::FromStr};

use argparse_rs::{ArgParser, ArgType};

fn main() {
    let mut parser = ArgParser::new("image-processor".into());
    configure_parser(&mut parser);

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_arguments(&args, &mut parser);
    println!("{:?}", parsed_arguments)
}

fn configure_parser(parser: &mut ArgParser) {
    parser.add_opt(
        "filter",
        Some("grayscale"),
        'f',
        true,
        "Filter to apply to the image",
        ArgType::Option,
    );

    parser.add_opt("file", None, 'i', true, "Input source", ArgType::Option);
}

fn parse_arguments(args: &Vec<String>, parser: &mut ArgParser) -> Args {
    let parsed = parser.parse(args.iter(), false).unwrap();

    let filter = parsed.get("filter").expect("Bad filter specified");

    let filepath = parsed.get("file").unwrap();

    Args {
        filter: filter,
        filepath: filepath,
    }
}

#[derive(Debug)]
struct Args {
    filter: ImageFilter,
    filepath: String,
}

#[derive(Debug)]
enum ImageFilter {
    GrayScale,
}

impl FromStr for ImageFilter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "grayscale" => Ok(ImageFilter::GrayScale),
            _ => Err(()),
        };
    }
}
