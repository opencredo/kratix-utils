use serde_yaml::{Value, to_string};
use std::fs::{write}; // Import 'write' for generic write operations, File for opening 
use std::io::{Error}; // Import ErrorKind
use std::fs::read_to_string;
use toml::Value as TomlValue;


/// Handle Pipeline transformation
///
/// Need to use properties.toml or pass a functton or interface
pub fn transform(_res_dir: &str, _res_path: &str, _kout_dir: &str, _kin_dir: &str) {
    
   log::debug!("resource  {}", _res_path);

   let mut base_instance: Value = serde_yaml::from_str(load_file(_res_path).unwrap().as_str())
    .expect("Error deserializing YAML file");

     /* 1. GET Kratix Input Object i.e Promise Req */
    let new_kin_path = format!("{}/object.yaml", _kin_dir);
    log::debug!("kratix input {}", new_kin_path);
    let kin_doc: Value = serde_yaml::from_str(load_file(&new_kin_path).unwrap().as_str())
        .expect("Error deserializing YAML file"); 


    let new_prop_path = format!("{}/properties.toml", _res_dir);
    let properties_toml = match load_file(&new_prop_path) {
        Ok(file_contents) => file_contents,
        Err(err) => {
            log::error!("Error loading properties.toml: {}", err);  // Log the error
            return;   // Or handle the error in some other way
        }
    };
    let mappings: TomlValue = toml::from_str(&properties_toml).unwrap();
    log::debug!("mappings.as_table() {:?}",mappings.as_table());

    for (toml_key, toml_value) in mappings.as_table().unwrap() {
        let source_path = toml_value.as_str().unwrap(); // Get path to copy from kin_doc
        let target_path = toml_key; // Get path to assign to in base_instance

        // Dynamically fetch the value from kin_doc & perform assignment
        let value_to_assign = get_nested_value(&kin_doc, source_path.split('.').collect());
        set_nested_value(&mut base_instance, target_path.split('.').collect(), value_to_assign);
    }

    // let new_name = kin_doc["spec"]["name"].as_str().unwrap().to_string();

    // log::debug!("Template Instance Name {}",base_instance["metadata"]["name"].as_str().unwrap().to_string());
    

    // base_instance["metadata"]["name"] = serde_yaml::Value::String(new_name.clone());

    // log::debug!("Promise Request Name {}",base_instance["metadata"]["name"].as_str().unwrap().to_string());


    let new_kout_path = format!("{}/flink-instance.yaml", _kout_dir);
    log::debug!("kratix output {}", new_kout_path);

    // // Serialize the modified YAML
    let yaml_str = to_string(&base_instance).unwrap();

    write(new_kout_path.clone(), yaml_str).unwrap();


}


fn load_file(path: &str) -> Result<String, Error> {
    read_to_string(path)
}

fn get_nested_value(value: &Value, path: Vec<&str>) -> Value {
    let mut current = value;
    for key in path {
        current = &current[key];
    }
    current.clone() 
}

fn set_nested_value(value: &mut Value, path: Vec<&str>, new_value: Value) {
    let mut current = value;
    for key in path.iter().take(path.len() - 1) { // Navigate to the parent
        current = &mut current[key]; 
    }
    let last_key = path.last().unwrap();
    current[last_key] = new_value;
}

