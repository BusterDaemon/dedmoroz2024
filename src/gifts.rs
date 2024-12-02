use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Gift {
    id: u64,
    latitude: f64,
    longitude: f64,
    weight: f64
}

pub fn read_gifts(path: &str) -> Result<Vec<Gift>, csv::Error>{
    let rdr = csv::Reader::from_path(path);
    let mut gifts: Vec<Gift> = vec![];
    match rdr {
        Ok(mut r) => {
            for result in r.deserialize() {
                let rec = result?;
                gifts.push(rec);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(gifts)
}