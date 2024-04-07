use image::{io::Reader as ImageReader, DynamicImage};
use std::{env, fmt::Display, str::FromStr};

use argparse_rs::{ArgParser, ArgType};

fn main() {
    let mut parser = ArgParser::new("image-processor".into());
    configure_parser(&mut parser);

    let args: Vec<String> = env::args().collect();

    let parsed_arguments = parse_arguments(&args, &mut parser);
    let absolute_filepath = get_absolute_filepath(&parsed_arguments.filepath);

    verify_filepath(&absolute_filepath);

    // let file_content = open_file(&absolute_filepath);

    // for line in file_content {
    //     println!("+ {}", line);
    // }

    let img = open_image(&absolute_filepath);
    let img_with_applied_filter = apply_filter(&img, &parsed_arguments.filter);
    save_img(&img_with_applied_filter, "output.png");
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

fn open_image(filepath: &str) -> DynamicImage {
    ImageReader::open(filepath).unwrap().decode().unwrap()
}

fn apply_filter(img: &DynamicImage, filter: &ImageFilter) -> DynamicImage {
    return match filter {
        ImageFilter::GrayScale => img.grayscale(),
        ImageFilter::Blur => img.blur(10.0),
    };
}

fn save_img(img: &DynamicImage, output_path: &str) {
    match img.save(output_path) {
        Ok(_) => println!("File saved"),
        Err(_) => println!("Couldn't save file"),
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
    Blur,
}

impl FromStr for ImageFilter {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "grayscale" => Ok(Self::GrayScale),
            "blur" => Ok(Self::Blur),
            _ => Err(()),
        };
    }
}

impl Display for ImageFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            ImageFilter::GrayScale => write!(f, "grayscale"),
            ImageFilter::Blur => write!(f, "blur"),
        };
    }
}
