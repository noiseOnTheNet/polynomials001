use plotters::prelude::*;
use poly::poly2::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    //polynomial definition
    let p0 = Poly::new(vec![0.0, 2.0, 0.0, -3.0]);

    //start of the plot
    let root = BitMapBackend::new("post002_plot0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    //the polynomial is formatted in the caption
    let mut chart = ChartBuilder::on(&root)
        .caption(format!("y={}",p0), ("sans-serif", 25).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;

    chart.configure_mesh().draw()?;

    //here the polynomial is evaluated
    //the polynomial is formatted in the plot label
    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, p0(x))),
            &RED,
        ))?
        .label(format!("y = {}",p0))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
