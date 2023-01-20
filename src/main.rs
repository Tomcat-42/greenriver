use byte_unit::Byte;
use colored::Colorize;
use math;
use std::{error::Error, path::PathBuf};
use tabled::{
    builder::Builder,
    format::Format,
    object::{Columns, Rows},
    Modify, Style,
};

use clap::{Parser, Subcommand};
use greenfield::{io, quantization};

#[derive(Parser, Debug)]
#[command(author = "Pablo Alessandro Santos Hugen", version = "0.1.0", about = "A command line tool to work with greenfield images", long_about = None)]
struct Cli {
    /// Actions to perform
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[clap(arg_required_else_help = true)]
enum Commands {
    /// Quantizes an image and saves it to a file
    Quantize {
        /// Number of bits of the r channel
        #[arg(default_value = "8")]
        bits_r: u8,
        /// Number of bits of the g channel
        #[arg(default_value = "8")]
        bits_g: u8,
        /// Number of bits of the b channel
        #[arg(default_value = "8")]
        bits_b: u8,

        /// The path to the input
        #[arg(default_value = "/dev/stdin")]
        input: PathBuf,
        /// The path to the output image
        #[arg(default_value = "/dev/stdout")]
        output: PathBuf,
    },

    /// Inspet an image
    Inspect {
        /// The path to the input image
        #[arg(default_value = "/dev/stdin")]
        input: PathBuf,
    },

    /// Converts an image
    Convert {
        /// The path to the input image
        #[arg(default_value = "/dev/stdin")]
        input: PathBuf,
        /// The path to the output image
        #[arg(default_value = "/dev/stdout")]
        output: PathBuf,
    },
}

impl Commands {
    pub fn inspect(input: PathBuf) -> Result<(), Box<dyn Error>> {
        let image = io::load_image(
            &input,
            quantization::UniformQuantization::new(8, 8, 8).unwrap(),
        )?;
        let (width, height) = image.dimensions();
        let quantization = image.quantization();
        let quantization::UniformQuantization {
            bits_r,
            bits_g,
            bits_b,
        } = quantization;
        let num_pixels = image.colors().count();
        let pixel_size = (bits_r + bits_g + bits_b) as usize;
        let data_size = num_pixels * pixel_size;

        let mut builder = Builder::default();
        builder
            .set_columns([
                "".to_string(),
                "Magic".to_string(),
                "Width".to_string(),
                "Height".to_string(),
                "Quantization".to_string(),
                "Data".to_string(),
            ])
            .add_record([
                "Value".to_string(),
                "b'grnfld42'".to_string(),
                width.to_string(),
                height.to_string(),
                quantization.to_string(),
                format!("{}x{} [RGB]", width, height),
            ])
            .add_record([
                "Size".to_string(),
                format!("{} b ({})", 64, "8 B"),
                format!("{} b ({})", 32, "4 B"),
                format!("{} b ({})", 32, "4 B"),
                format!("{} b ({})", 9, "~1 B"),
                format!(
                    "{} b ({})",
                    data_size,
                    Byte::from_bytes(math::round::ceil(data_size as f64 / 8.0_f64, 0) as u128)
                        .get_appropriate_unit(true)
                ),
            ]);

        let mut table = builder.build();
        table
            .with(Style::modern())
            .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.blue().to_string())))
            .with(Modify::new(Rows::single(0)).with(Format::new(|s| s.green().to_string())));

        println!("{}", table);

        Ok(())
    }

    pub fn quantize(
        uniform_quantization: quantization::UniformQuantization,
        input: PathBuf,
        output: PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let image = io::load_image(&input, uniform_quantization)?;
        io::save_image(&image, &output)?;
        Ok(())
    }
    pub fn convert(input: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
        let image = io::load_image(&input, quantization::UniformQuantization::new(8, 8, 8)?)?;
        io::save_image(&image, &output)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let Cli { command } = cli;

    match command {
        Commands::Inspect { input } => Commands::inspect(input),
        Commands::Quantize {
            bits_r,
            bits_g,
            bits_b,
            input,
            output,
        } => Commands::quantize(
            quantization::UniformQuantization::new(bits_r, bits_g, bits_b)?,
            input,
            output,
        ),
        Commands::Convert { input, output } => Commands::convert(input, output),
    }
}
