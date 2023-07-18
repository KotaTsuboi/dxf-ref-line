use crate::input::RefLine;
use dxf::{
    entities::{Circle, DimensionBase, Entity, Line, OrdinateDimension, Polyline},
    enums::DimensionType,
    tables::{DimStyle, Layer},
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let ref_line_layer = Layer {
        name: input.layer_name().ref_line(),
        color: Color::from_index(2),
        ..Default::default()
    };

    drawing.add_layer(ref_line_layer);

    let dimension_layer = Layer {
        name: input.layer_name().dimension(),
        color: Color::from_index(3),
        ..Default::default()
    };

    drawing.add_layer(dimension_layer);

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

fn get_x_coords(input: &RefLine) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut coords = Vec::new();

    let mut x = 0.0;

    for i in 0..input.num_x_axis() {
        coords.push(x);
        if i < input.num_x_axis() - 1 {
            x += input.x_spans()[i as usize];
        }
    }

    Ok(coords)
}

fn get_y_coords(input: &RefLine) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut coords = Vec::new();

    let mut y = 0.0;

    for i in 0..input.num_y_axis() {
        coords.push(y);
        if i < input.num_y_axis() - 1 {
            y += input.y_spans()[i as usize];
        }
    }

    Ok(coords)
}

fn write_x_lines(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let length: f64 = input.y_spans().iter().sum();

    for x in get_x_coords(input)? {
        write_line(drawing, x, 0.0, x, length, &input.layer_name().ref_line())?;
    }

    Ok(())
}

fn write_y_lines(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let length: f64 = input.x_spans().iter().sum();

    for y in get_y_coords(input)? {
        write_line(drawing, 0.0, y, length, y, &input.layer_name().ref_line())?;
    }

    Ok(())
}

fn write_dimension(
    drawing: &mut Drawing,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    layer: String,
) -> Result<(), Box<dyn Error>> {
    let dim_style = DimStyle {
        name: "mydim".to_string(),
        dimensioning_text_height: 1000.0,
        ..Default::default()
    };

    drawing.add_dim_style(dim_style);

    let dimension_base = DimensionBase {
        definition_point_1: Point {
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0 - 1000.0,
            z: 0.0,
        },
        text_mid_point: Point {
            x: (x1 + x2) / 2.0,
            y: (y1 + y2) / 2.0 - 1000.0,
            z: 0.0,
        },
        dimension_style_name: "mydim".to_string(),
        ..Default::default()
    };

    let dimension = OrdinateDimension {
        dimension_base,
        definition_point_2: Point {
            x: x1,
            y: y1,
            z: 0.0,
        },
        definition_point_3: Point {
            x: x2,
            y: y2,
            z: 0.0,
        },
    };

    let mut dimension = Entity::new(dxf::entities::EntityType::OrdinateDimension(dimension));

    dimension.common.layer = layer;

    drawing.add_entity(dimension);

    Ok(())
}

fn write_x_dimensions(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    for i in 1..input.num_x_axis() {
        let coords = get_x_coords(input)?;
        let x1 = coords[(i - 1) as usize];
        let x2 = coords[i as usize];
        write_dimension(drawing, x1, 0.0, x2, 0.0, input.layer_name().dimension())?;
    }

    Ok(())
}

pub fn write(input: RefLine, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_x_lines(&mut drawing, &input)?;

    write_y_lines(&mut drawing, &input)?;

    write_x_dimensions(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
