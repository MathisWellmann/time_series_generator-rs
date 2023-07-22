use plotters::prelude::*;

use super::Series3D;

const MARGIN: u32 = 10;
const CIRCLE_RADIUS: u32 = 2;

/// Plot in 3 dimensions.
pub(crate) fn plot_3d(
    points: &Series3D,
    filename: &str,
    dims: (u32, u32),
) -> Result<(), Box<dyn std::error::Error>> {
    let area = BitMapBackend::new(&filename, dims).into_drawing_area();

    area.fill(&WHITE).unwrap();
    let root_area = area
        .titled(filename, ("sans-serif", 20).into_font())
        .unwrap();
    let mut cc0 = ChartBuilder::on(&root_area)
        .margin(MARGIN)
        .set_all_label_area_size(20)
        .build_cartesian_3d(points.x_range(), points.y_range(), points.z_range())?;

    cc0.with_projection(|mut pb| {
        pb.yaw = 0.2;
        pb.scale = 0.9;
        pb.into_matrix()
    });
    cc0.configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;
    cc0.draw_series(
        points
            .iter()
            .map(|(x, y, z)| Circle::new((*x, *y, *z), CIRCLE_RADIUS, BLACK.filled())),
    )?
    .label("points")
    .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    cc0.draw_series(LineSeries::new(points.clone().inner(), BLACK))
        .unwrap()
        .label("line through points")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    cc0.configure_series_labels().draw()?;

    Ok(())
}
