mod structs;
use std::{path::PathBuf, fmt::Write};
use csv::{ReaderBuilder, Writer, WriterBuilder};

use pubchem::Compound;
use structs::{FileContent, CompoundData, CleanedData};

pub fn read_csv(path: &str) ->Vec<FileContent> {
    let path = PathBuf::from(path);
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(path).unwrap();
    let mut data: Vec<FileContent> = Vec::new();
    for item in reader.deserialize(){
        match item {
            Ok(d) => data.push(d),
            Err(_x) => println!("{_x}"),
        };
    }
    data
}

pub fn get_compound_data(inchi: &str) -> CompoundData {
    let comp = Compound::with_inchikey(inchi);
    let smiles = match comp.canonical_smiles(){
        Ok(s) => s,
        Err(_x) => "".to_string(),
    };
    let synonyms: Vec<String> = match comp.synonyms() {
        Ok(x) => x,
        Err(_x) => Vec::new(),
    };
    let name = match comp.title(){
        Ok(x) => x.to_string(),
        Err(_x) => "Does not exist".to_string(),
    };
    CompoundData::new(name, smiles, synonyms)
}


pub fn write_csv(path: &str, file: &str, data: &str) {
    let path = PathBuf::from(path);
    let path = path.join(file);
    let cleaned_data: Vec<CleanedData> = serde_json::from_str(data).unwrap();
    let mut writer = WriterBuilder::new()
        .delimiter(b';')
        .from_path(path)
        .unwrap();
    for item in cleaned_data{
        writer.serialize(item).unwrap();
    }
    let _ = writer.flush();
}
