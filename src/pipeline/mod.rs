use log;
use std::fs;
use std::io::Result;
use std::path::Path;
use yaml_rust2::Yaml;
use yaml_rust2::YamlLoader;

mod pipeline_config; 
pub use pipeline_config::PipelineConfig; // Re-export PipelineConfig 


pub fn load_file(file: &str) -> Result<Yaml> {
    let path = Path::new(file);
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];

    Ok(doc.clone()) // Return a copy of the doc
}

pub fn copy_files(source_dir: &str, destination_dir: &str) -> Result<()> {
    log::debug!("pipeline::copy_files {} to {}", source_dir, destination_dir);
    // Create the output directory if it doesn't exist
    fs::create_dir_all(destination_dir)?;

    // Iterate over files in the source directory
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let destination_path = format!("{}/{}", destination_dir, filename);

            // Copy the file
            fs::copy(path, destination_path)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn list_files_recursively(path: &str) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                log::debug!("{}", path.display());

                if path.is_dir() {
                    list_files_recursively(&path.to_string_lossy()); // Recursion!
                }
            }
        }
    } else {
        log::warn!("Error reading directory: {}", path);
    }
}

#[allow(dead_code)]
pub fn status() {
    if let Err(err) = fs::copy(
        "/tmp/transfer/resources/status.yaml",
        "/kratix/metadata/status.yaml",
    ) {
        log::warn!("Error during file copy: {}", err);
    }
}


