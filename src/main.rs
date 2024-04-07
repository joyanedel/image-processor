use std::{env, fmt::Display, str::FromStr};

use argparse_rs::{ArgParser, ArgType};

fn main() {
    let mut parser = ArgParser::new("image-processor".into());
    configure_parser(&mut parser);

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_arguments(&args, &mut parser);
    let absolute_filepath = get_absolute_filepath(&parsed_arguments.filepath);

    verify_filepath(&absolute_filepath);

    let file_content = open_file(&absolute_filepath);

    for line in file_content {
        println!("+ {}", line);
    }
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

fn get_absolute_filepath(filename: &str) -> String {
    format!(
        "{}/{}",
        env::current_dir().unwrap().as_path().to_str().unwrap(),
        filename
    )
}

fn verify_filepath(filepath: &str) {
    let metadata = std::fs::metadata(filepath);

    if metadata.is_err() {
        panic!("Cannot access file {}", filepath);
    }

    if !metadata.unwrap().is_file() {
        panic!("Must specify a valid file");
    };
}

fn open_file(filepath: &str) -> Vec<String> {
    std::fs::read_to_string(&filepath)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
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

impl Display for ImageFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            ImageFilter::GrayScale => write!(f, "grayscale"),
        };
    }
}
