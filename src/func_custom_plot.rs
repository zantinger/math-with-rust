use crate::DrawResult;
use plotters::{element::CoordMapper, prelude::*, style::RGBAColor};
use plotters_canvas::CanvasBackend;

/// Draw power function f(x) = x^power.
pub fn draw(canvas_id: &str, power: i32) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    let font: FontDesc = ("sans-serif", 20.0).into();
    let mut data1: [(f32, f32); 3] = [
        (-5.0, -5.0),
        (0.0, 5.0),
        (5.0, -5.0)
    ];

    fn add(v1: (f32, f32), v2: (f32, f32)) -> (f32, f32) {
        (v1.0 + v2.0, v1.1 + v2.1)
    }

    fn add_vec(slice: &mut[(f32, f32)]) {
        for c in slice {
            // c.0 = c.0 + 0.5;
            *c = add(*c, (-1.5, -2.0));
        }

    }

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption(format!("some caption {}", power), font)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-5f32..5f32, -5f32..5f32)?;

    chart.configure_mesh().x_labels(3).y_labels(3).draw()?;

    // chart.draw_series(LineSeries::new(
    //     (-1..=1)
    //         .map(|x| x as f32)
    //         // .map(|x| (x, x.powf(power as f32))),
    //         .map(|x| (x, 1.0 * x)),
    //     &RED,
    // ))?;

    chart.draw_series(data1.iter().map(|point| Circle::new(*point, 5, &BLUE)));
    // add_vec(&mut data1);
    chart.draw_series(std::iter::once(Polygon::new(data1, &RED.mix(0.2))))?;

    root.present()?;
    return Ok(chart.into_coord_trans());

}
