use std::{fs::File, io::BufWriter};
use std::io::BufReader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use struct_iterable::Iterable;

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct PassInfo {
    pub username: String,
    pub password: String,
    pub url: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct Pass {
    #[serde(flatten)]
    pub data: HashMap<String, PassInfo>
}

pub fn read_pass_file(client_id: &String) -> Result<Pass, Box<dyn std::error::Error>> {
    let mut abs_root = std::env::current_exe()?;
    abs_root.pop();
    abs_root.push(client_id.clone() + ".json");

    // Open the file
    let file = File::open(abs_root)?;

    // Create a buffered reader for efficient reading
    let reader = BufReader::new(file);

    // Deserialize the JSON directly from the reader into your struct
    let passfile: Pass = serde_json::from_reader(reader)?;

    return Ok(passfile);
}

pub fn write_pass_file(client_id: &String, data: &HashMap<String, PassInfo>) -> Result<bool, std::io::Error> {
    let mut abs_root = std::env::current_exe()?;
    abs_root.pop();
    abs_root.push(client_id.clone() + ".json");

    let file = File::create(abs_root)?;
    let writer = BufWriter::new(file);

    return match serde_json::to_writer(writer, &data) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false)
    };
}