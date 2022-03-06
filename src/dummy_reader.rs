use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Dummy{
    pub name : String,
    pub fields : Vec<String>,
    pub values : Vec<Value>,
    pub cmds : Vec<String>
}

pub fn load_dummy(filepath: String) -> Dummy
{
    println!("Filepath: {}", filepath);
    //read json file into a string
    let raw_json =  match fs::read_to_string(filepath)
    {
        Ok(data) => data,
        Err(e) => panic!("Failed to read file: {}", e)
    };
    
    //attempt to convert json to DummyList
    let dummy  = serde_json::from_str(&raw_json);
    return match dummy
    {
        Ok(list) => list,
        Err(error) => panic!("Failed to parse dummies.json, error: {:?}", error),
    };
}