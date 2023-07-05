use ndarray::Array;
use plotters::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frequency = 2.0;  // Frequency of the sine wave
    let samples = 1000;  // Number of samples
    let sample_interval = 0.01;  // Time interval between samples

    // Generate the time values for each sample
    let t = Array::linspace(0., sample_interval * samples as f64, samples);

    // Generate the sine wave
    let y = &t * frequency * 2.0 * PI; 
    let y = y.mapv(f64::sin);

    // Prepare the plot
    let root = BitMapBackend::new("sine_wave.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sine Wave", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..100f64, -2f64..2f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        t.iter().zip(y.iter()).map(|(x, y)| (*x, *y)),
        &RED,
    ))?.label("sine wave")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
