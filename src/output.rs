use crate::input::RefLine;
use dxf::{
    entities::{Circle, DimensionBase, Entity, EntityType, Line, OrdinateDimension, Text},
    enums::{HorizontalTextJustification, VerticalTextJustification},
    tables::{DimStyle, Layer},
    Color, Drawing, Point,
};
use std::error::Error;

fn set_layer(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let ref_line_layer = Layer {
        name: input.layer_name().ref_line(),
        line_type_name: "CENTER".to_string(),
        color: Color::from_index(3),
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
    is_vertical: bool,
) -> Result<(), Box<dyn Error>> {
    let dim_style = DimStyle {
        name: "mydim".to_string(),
        dimensioning_text_height: 1000.0,
        dimensioning_arrow_size: 500.0,
        dimension_extension_line_offset: 2000.0,
        ..Default::default()
    };

    drawing.add_dim_style(dim_style);

    let gap = 5000.0;

    let dimension_base = if is_vertical {
        DimensionBase {
            definition_point_1: Point {
                x: (x1 + x2) / 2.0 - gap,
                y: (y1 + y2) / 2.0,
                z: 0.0,
            },
            text_mid_point: Point {
                x: (x1 + x2) / 2.0 - gap,
                y: (y1 + y2) / 2.0,
                z: 0.0,
            },
            dimension_style_name: "mydim".to_string(),
            text_rotation_angle: 270.0,
            ..Default::default()
        }
    } else {
        DimensionBase {
            definition_point_1: Point {
                x: (x1 + x2) / 2.0,
                y: (y1 + y2) / 2.0 - gap,
                z: 0.0,
            },
            text_mid_point: Point {
                x: (x1 + x2) / 2.0,
                y: (y1 + y2) / 2.0 - gap,
                z: 0.0,
            },
            dimension_style_name: "mydim".to_string(),
            ..Default::default()
        }
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
        write_dimension(
            drawing,
            x1,
            0.0,
            x2,
            0.0,
            input.layer_name().dimension(),
            false,
        )?;
    }

    Ok(())
}

fn write_y_dimensions(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    for i in 1..input.num_y_axis() {
        let coords = get_y_coords(input)?;
        let y1 = coords[(i - 1) as usize];
        let y2 = coords[i as usize];
        write_dimension(
            drawing,
            0.0,
            y1,
            0.0,
            y2,
            input.layer_name().dimension(),
            true,
        )?;
    }

    Ok(())
}

fn write_x_axes(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let coords = get_x_coords(input)?;

    for i in 0..input.num_x_axis() {
        let x = coords[i as usize];
        let y = -7000.0;

        let text = Text {
            value: input.x_axes()[i as usize].clone(),
            location: Point::new(x, y, 0.0),
            text_height: 1000.0,
            relative_x_scale_factor: 0.85,
            horizontal_text_justification: HorizontalTextJustification::Center,
            second_alignment_point: Point::new(x, y, 0.0),
            vertical_text_justification: VerticalTextJustification::Middle,
            ..Default::default()
        };

        let mut text = Entity::new(EntityType::Text(text));
        text.common.layer = input.layer_name().dimension();
        drawing.add_entity(text);

        let circle = Circle {
            radius: 1000.0,
            center: Point::new(x, y, 0.0),
            ..Default::default()
        };

        let mut circle = Entity::new(EntityType::Circle(circle));
        circle.common.layer = input.layer_name().dimension();
        drawing.add_entity(circle);
    }
    Ok(())
}

fn write_y_axes(drawing: &mut Drawing, input: &RefLine) -> Result<(), Box<dyn Error>> {
    let coords = get_y_coords(input)?;

    for i in 0..input.num_y_axis() {
        let x = -7000.0;
        let y = coords[i as usize];

        let text = Text {
            value: input.y_axes()[i as usize].clone(),
            location: Point::new(x, y, 0.0),
            text_height: 1000.0,
            relative_x_scale_factor: 0.85,
            horizontal_text_justification: HorizontalTextJustification::Center,
            second_alignment_point: Point::new(x, y, 0.0),
            vertical_text_justification: VerticalTextJustification::Middle,
            rotation: 270.0,
            ..Default::default()
        };

        let mut text = Entity::new(EntityType::Text(text));
        text.common.layer = input.layer_name().dimension();
        drawing.add_entity(text);

        let circle = Circle {
            radius: 1000.0,
            center: Point::new(x, y, 0.0),
            ..Default::default()
        };

        let mut circle = Entity::new(EntityType::Circle(circle));
        circle.common.layer = input.layer_name().dimension();
        drawing.add_entity(circle);
    }
    Ok(())
}

pub fn write(input: RefLine, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut drawing = Drawing::new();

    set_layer(&mut drawing, &input)?;

    write_x_lines(&mut drawing, &input)?;

    write_y_lines(&mut drawing, &input)?;

    write_x_dimensions(&mut drawing, &input)?;

    write_y_dimensions(&mut drawing, &input)?;

    write_x_axes(&mut drawing, &input)?;

    write_y_axes(&mut drawing, &input)?;

    drawing.save_file(output_file)?;

    Ok(())
}
