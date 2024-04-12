#[allow(unused_imports)]
use yaml_rust2::{Yaml,YamlEmitter};
use std::fs::write;
use crate::kratix::pipeline::load_file;
use log;

pub fn transform(_res_path: &str, _kout_dir: &str,_kin_dir: &str) {
    //get the resource base instance yaml
    let _doc = load_file(_res_path).expect("Error loading YAML file"); //STEP 2.
 
    log::debug!("resource  {}", _res_path);


    let new_kin_path = format!("{}/object.yaml", _kin_dir);
    log::debug!("kratix input {}", new_kin_path);
    let _kin_doc = load_file(&new_kin_path).expect("Error loading YAML file"); //STEP 2.
    
    let name = _kin_doc["spec"]["name"].as_str().unwrap().to_string();

    log::debug!("{:?}", name);
    
     /* WRITE NEW FILE WITH REQ CHANGES */
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(&_doc).unwrap();

    let new_kout_path = format!("{}/flink-instance.yaml", _kout_dir);
    log::debug!("kratix output {}", new_kout_path);
    write(new_kout_path, out_str).unwrap();
}
