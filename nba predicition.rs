extern crate csv;
extern crate nalgebra as na;
extern crate plotters;
extern crate statrs;
extern crate itertools;

use std::error::Error;
use std::fs::File;
use std::string::String;
use csv::Reader;
use itertools::sorted;
use plotters::prelude::*;
use statrs::statistics::Variance;
use statrs::distribution::{Normal, Univariate};
use statrs::statistics::Mean;
use statrs::function::pdf::Pdf;
use statrs::function::lnpdf::LnPdf;
use statrs::statistics::Min;
use statrs::statistics::Max;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let mut rdr = Reader::from_path("C:\\path\\to\\your\\file.csv")?;

    // Define your data structures here
    // For example, Player, PTS, TRB, AST, etc.

    let data: Vec<PlayerData> = rdr.deserialize()
        .map(|result| {
            let record: PlayerData = result.unwrap();
            record
        })
        .collect();

    // Data processing and analysis here

    // Example of sorting by score in descending order
    let sorted_data = sorted(data.iter(), |a, b| b.score.cmp(&a.score))
        .collect::<Vec<&PlayerData>>();

    // Example of creating a bar chart (requires the plotters crate)
    let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let scores: Vec<f64> = sorted_data.iter().take(5).map(|player| player.score).collect();
    let players: Vec<String> = sorted_data.iter().take(5).map(|player| player.name.clone()).collect();

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("Top 5 Players Offensively", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_ranged(0..5, 0.0..scores.iter().cloned().fold(0.0/0.0, f64::max))?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        scores.iter().enumerate().map(|(i, score)| {
            let mut score_label = score.to_string();
            score_label.push_str(" PTS");
            let pos = (i as i32, *score);
            return Rectangle::new([(i as f64 - 0.4, 0.0), (i as f64 + 0.4, *score)], BLUE.filled())
                .label(score_label)
                .legend(|(x, y)| Rectangle::new([(x, y - 50), (x + 50, y)], BLUE.filled()));
        }),
    )?;
    
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw()?;
    
    Ok(())
}
