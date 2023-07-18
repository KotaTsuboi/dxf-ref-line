use crate::input::RefLine;
use dxf::{
    entities::{Circle, Entity, Line, Polyline},
    tables::Layer,
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let concrete_layer = Layer {
        name: input.layer_name().ref_line(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(concrete_layer);

    let rebar_layer = Layer {
        name: input.layer_name().dimension(),
        color: Color::from_index(3),
        ..Default::default()
    };

    drawing.add_layer(rebar_layer);

    Ok(())
}

fn write_line(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: &str,
) -> Result<(), Box<dyn Error>> {
    let line = Line {
        p1: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        p2: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
        ..Default::default()
    };
    let mut line = Entity::new(dxf::entities::EntityType::Line(line));
    line.common.layer = layer.to_string();
    drawing.add_entity(line);
    Ok(())
}

fn write_x_lines(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let mut x = 0.0;

    let length: f64 = input.y_spans().iter().sum();

    for i in 0..input.num_x_axis() {
        write_line(drawing, x, 0.0, x, length, &input.layer_name().ref_line())?;
        if i < input.num_x_axis() - 1 {
            x += input.x_spans()[i as usize];
        }
    }

    Ok(())
}

fn write_y_lines(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let mut y = 0.0;

    let length: f64 = input.x_spans().iter().sum();

    for i in 0..input.num_y_axis() {
        write_line(drawing, 0.0, y, length, y, &input.layer_name().ref_line())?;
        if i < input.num_y_axis() - 1 {
            y += input.y_spans()[i as usize];
        }
    }

    Ok(())
}

pub fn write(input: RefLine, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    write_x_lines(&mut drawing, &input)?;

    write_y_lines(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
