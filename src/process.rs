use anyhow::Result;
use csv::Reader;
use std::fs;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_name(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    // let records = reader.deserialize().map(|record|record.unwrap()).collect::<Vec<Record>>();
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        //println!("{:?}", record);
        ret.push(record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
