// use kratix_utils::pipeline::PipelineConfig;
// use kratix_utils::promise;
// use std::env;

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_request() -> Result<(), String> {
        let x = 4.0;

        // Extract validated environment variables
        // let workflow_type = env::var("KRATIX_WORKFLOW_TYPE").unwrap();
        // let base_instance = env::var("BASE_INSTANCE").unwrap();
        // let dep_dir = env::var("DEPENDENCIES_DIR").unwrap();
        // let res_dir = env::var("RESOURCES_DIR").unwrap();
        // let kratix_input_dir = env::var("KRATIX_INPUT").unwrap();
        // let kratix_output_dir = env::var("KRATIX_OUTPUT").unwrap();

        // let config = PipelineConfig::new(
        //     &base_instance,
        //     &res_dir,
        //     &dep_dir,
        //     &kratix_output_dir,
        //     &kratix_input_dir,
        //     &workflow_type,
        // );

        // promise::transform(
        //     config.res_dir(),
        //     config.base_instance(),
        //     config.kratix_output_dir(),
        //     config.kratix_input_dir(),
        // );

        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}
