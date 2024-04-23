pub mod pipeline;
pub mod promise;
use crate::pipeline::PipelineConfig;
use dotenv::dotenv;
use log;
use std::{env, process};

// Structure to hold potential errors
#[derive(Debug)]
struct EnvVarError {
    var_name: String,
}

pub fn run_pipeline(args: Vec<String>) {
    dotenv().ok();

    // Validate environment variables up front
    match validate_env_vars() {
        Ok(()) => (), // Everything is good, proceed
        Err(errors) => {
            eprintln!("Error: Missing environment variables:");
            for error in errors {
                log::warn!(" - {}", error.var_name);
            }
            process::exit(1);
        }
    }

    if args.len() < 2 {
        log::warn!("Usage: <command> [build, pipeline, load, push, rmi, pull]");
        process::exit(1);
    }

    // Extract validated environment variables
    let workflow_type = env::var("KRATIX_WORKFLOW_TYPE").unwrap();
    let base_instance = env::var("BASE_INSTANCE").unwrap();
    let dep_dir = env::var("DEPENDENCIES_DIR").unwrap();
    let res_dir = env::var("RESOURCES_DIR").unwrap();
    let kratix_input_dir = env::var("KRATIX_INPUT").unwrap();
    let kratix_output_dir = env::var("KRATIX_OUTPUT").unwrap();

    let config = PipelineConfig::new(
        &base_instance,
        &res_dir,
        &dep_dir,
        &kratix_output_dir,
        &kratix_input_dir,
        &workflow_type,
    );

    log::debug!("<- Start Pipeline ({}) ->", config.workflow_type());

    match config.workflow_type() {
        "promise" => {
            // Fullful promise.yaml
            if let Err(err) =
                // tmp/transfer/dependecies -> /kratix/output
                pipeline::copy_files(config.dep_dir(), config.kratix_output_dir())
            {
                log::warn!("Error during file copy: {}", err);
            }
        }
        "resource" => {
            log::debug!("  1. transform resource");
            // Fullfil resource_request.yaml
            promise::transform(
                config.res_dir(),
                config.base_instance(),
                config.kratix_output_dir(),
                config.kratix_input_dir(),
            );
        }
        "request" => {
            log::debug!("  1. transform request");
            // Fullfil resource_request.yaml
            promise::transform(
                config.res_dir(),
                config.base_instance(),
                config.kratix_output_dir(),
                config.kratix_input_dir(),
            );
        }
        _ => {
            log::error!("No workflow_type");
        }
    }

    //pipeline::status();

    //pipeline::list_files_recursively(_kratix_output_dir);

    log::debug!("<- End Pipeline ->");
}

// validation function
fn validate_env_vars() -> Result<(), Vec<EnvVarError>> {
    let required_vars = vec![
        "KRATIX_WORKFLOW_TYPE",
        "BASE_INSTANCE",
        "DEPENDENCIES_DIR",
        "RESOURCES_DIR",
        "KRATIX_INPUT",
        "KRATIX_OUTPUT",
    ];

    let mut errors = Vec::new();

    for var_name in required_vars {
        if env::var(var_name).is_err() {
            errors.push(EnvVarError {
                var_name: var_name.to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
