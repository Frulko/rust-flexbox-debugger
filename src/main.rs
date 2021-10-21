use stretch::geometry::Size;
use stretch::style::*;
use stretch::Error;

fn main() -> Result<(), Error> {
    let mut stretch = stretch::node::Stretch::new();
    let child = stretch.new_node(
        Style { size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto }, ..Default::default() },
        (&[]).to_vec(),
    )?;
    let node = stretch.new_node(
        Style {
            size: Size { width: Dimension::Points(200.0), height: Dimension::Points(100.0) },
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        (&[child]).to_vec(),
    )?;

    stretch.compute_layout(node, Size::undefined())?;
    println!("node: {:#?}", stretch.layout(node)?);
    println!("child: {:#?}", stretch.layout(child)?);

    Ok(())
}