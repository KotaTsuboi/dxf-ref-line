use getset::{CopyGetters, Getters};
use serde_derive::Deserialize;
use std::error::Error;
use std::io::Read;
use std::{fs, io::BufReader};

#[derive(Deserialize, Getters, CopyGetters, Debug)]
pub struct RefLine {
    #[getset(get_copy = "pub")]
    num_x_axis: u32,
    #[getset(get_copy = "pub")]
    num_y_axis: u32,
    #[getset(get_copy = "pub")]
    num_floor: u32,
    #[getset(get = "pub")]
    x_spans: Vec<f64>,
    #[getset(get = "pub")]
    y_spans: Vec<f64>,
    #[getset(get = "pub")]
    floor_heights: Vec<f64>,
    layer_name: Option<LayerName>,
}

impl RefLine {
    pub fn layer_name(&self) -> LayerName {
        self.layer_name.clone().unwrap_or(LayerName {
            ref_line: Some("通り芯".to_string()),
            dimension: Some("寸法".to_string()),
        })
    }
}

#[derive(Deserialize, Clone, Getters, CopyGetters, Debug)]
pub struct LayerName {
    ref_line: Option<String>,
    dimension: Option<String>,
}

impl LayerName {
    pub fn ref_line(&self) -> String {
        self.ref_line
            .clone()
            .unwrap_or_else(|| "通り芯".to_string())
    }

    pub fn dimension(&self) -> String {
        self.dimension
            .clone()
            .unwrap_or_else(|| "通り芯".to_string())
    }
}

fn read_file(path: &str) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_input(file_path: &str) -> Result<RefLine, Box<dyn Error>> {
    let s = read_file(file_path).expect("failed to read file");

    let toml: Result<RefLine, toml::de::Error> = toml::from_str(&s);

    let toml = toml.expect("failed to parse toml");

    Ok(toml)
}
