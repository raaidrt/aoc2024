mod day;
mod graph;
mod stage;

use clap::Parser;
use fraction::ToPrimitive;

use std::{error::Error, fmt, fs, io::Read, path};

#[derive(Debug)]
struct DayError(u8);

impl fmt::Display for DayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid day: {}", self.0)
    }
}

impl Error for DayError {}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Args {
    #[arg(long)]
    day: u8,
    #[arg(long)]
    file_name: String,
    #[arg(long, default_value_t = false)]
    plot: bool,
    #[arg(long)]
    stage: stage::Stage,
}

use paste::paste;
use seq_macro::seq;
use stage::Stage;

seq!(N in 1..=25 {
    fn run(day: u8, s: &str, stage: Stage) -> Result<String, Box<dyn Error>> {
        match day {
            #(
                N => paste!(day::[<day N>]::run(s, stage)),
            )*
            _ => Err(Box::new(DayError(day.into()))),
        }
    }
});

const NUM_THREADS_MIN: u16 = 1;
const NUM_THREADS_MAX: u16 = 10;

use std::time::Instant;
use textplots::{AxisBuilder, Chart, ColorPlot, LabelBuilder, LineStyle, Shape};
use tqdm::tqdm;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut file = fs::File::open(path::Path::new(&args.file_name))?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    let result = result;

    let mut final_result = String::new();
    let num_threads_range: Vec<u16> = (NUM_THREADS_MIN..=NUM_THREADS_MAX).collect();
    let mut times = vec![f32::from(0.0); num_threads_range.len()];
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    if args.plot {
        for (i, num_threads) in tqdm(num_threads_range.iter().enumerate())
            .desc(format!("Running Day {} with different RAYON_NUM_THREADS", args.day).into())
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(usize::from(*num_threads))
                .build()
                .unwrap();
            let before = Instant::now();
            final_result = pool.install(|| run(args.day, &result, args.stage.clone()).unwrap());
            times[i] = before.elapsed().as_secs_f32();
        }
        println!("\nRuntime (s) against Number of Threads Available");

        // Create pairs of points from x and y values
        let points: Vec<(f32, f32)> = num_threads_range
            .iter()
            .map(|x| f32::from(*x))
            .zip(times)
            .map(|(x, y)| (x, y))
            .collect();

        let lines = Shape::Lines(&points);
        let mut chart = Chart::new(
            180,
            60,
            f32::from(NUM_THREADS_MIN),
            f32::from(NUM_THREADS_MAX),
        );

        let chart = chart
            .linecolorplot(&lines, rgb::Rgb { r: 0, g: 255, b: 0 })
            .x_label_format(textplots::LabelFormat::Custom(Box::new(move |x| {
                x.trunc().to_usize().unwrap().to_string()
            })));
        let chart = chart.x_axis_style(LineStyle::Dashed);
        let chart = chart.y_axis_style(LineStyle::Dashed);
        chart.nice();
    } else {
        final_result = run(args.day, &result, args.stage)?;
    }

    println!(
        "Running Day {} w/ Input File: {}\nResult:\n-----------------------\n{}",
        args.day, args.file_name, final_result
    );
    Ok(())
}
