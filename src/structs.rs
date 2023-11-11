use std::string;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileContent{
    id: String,
    compound_name: String,
    inchi_key: String,
}


#[derive(Debug, Default, Serialize)]
pub struct CompoundData{
    name: String,
    smiles: String,
    synonyms: Vec<String>,
}

impl CompoundData {
    pub fn new(name: String, smiles: String, synonyms: Vec<String>)  -> Self{
        CompoundData { name: name, smiles: smiles, synonyms: synonyms }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanedData{
    id: String,
    compound_name: String,
    inchi_key: String,
    synonyms: String,
    previous_name: String,
}
