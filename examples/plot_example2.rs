use plotters::prelude::*;
use poly;

fn fac(n : i32) -> i32{
    let mut result = 1;
    for i in 1..=n{
        result = result * i
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    //start of the plot
    let root = BitMapBackend::new("post002_plot1.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    //the polynomial is formatted in the caption
    let mut chart = ChartBuilder::on(&root)
        .caption("Approximations of sin", ("sans-serif", 25).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-7f32..7f32, -1.2f32..1.2f32)?;

    chart.configure_mesh().draw()?;

    //here the polynomial is evaluated
    //the polynomial is formatted in the plot label
    for (deg,color) in [(1,RED),(3,BLUE),(5,MAGENTA),(7,GREEN),(9,CYAN)].iter(){
        let coeff : Vec<f32> = (0..=*deg)
            .map(|n| if n % 2 == 0 { 0.0 }
                 else {
                     let nf = fac(n) as f32;
                     let sign = if ((n - 1) / 2) % 2 == 0 { 1.0 } else { -1.0 } ;
                     sign/nf
                 })
            .collect();
        let p0 = poly::poly1::Poly::new(coeff);
        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 * 7.0 / 50.0).map(|x| (x, p0.eval(x,0.0))),
                &color,
            ))?
            .label(format!("sin{}",deg))
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color.clone()));
    }
    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 * 7.0 / 50.0).map(|x| (x, x.sin())),
            &BLACK,
        ))?
        .label("sin")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
