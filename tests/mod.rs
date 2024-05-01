use kratix_utils::{pipeline::PipelineConfig, run_custom_pipeline, ResourceRequest};
use std::{env, path::Path};

#[cfg(test)]
mod tests {
    use super::*;

    const WORKSPACE: &str = "WORKSPACE";
    const KRATIX_WORKFLOW_TYPE: &str = "KRATIX_WORKFLOW_TYPE";

    #[derive(Clone)]
    pub struct MyPromise {
        pub params: String,
    }

    impl ResourceRequest for MyPromise {
        fn transform(&self, _conf: &PipelineConfig) -> String {
            let new_kin_path = format!("{}/object.yaml", _conf.kratix_input_dir());

            format!("{} modify {:?}", self.params, new_kin_path)
        }
    }

    #[test]
    fn test_list_files_recursively() -> Result<(), String> {
        let current_dir = env::current_dir().unwrap();
        env::set_var(WORKSPACE, current_dir);
        let x = "tests/test-input";
        kratix_utils::pipeline::list_files_recursively(x);

        Ok(())
    }

    #[test]
    fn test_promise_request() -> Result<(), String> {
        init_logger();

        let current_dir = env::current_dir().unwrap();
        env::set_var(WORKSPACE, current_dir);
        env::set_var(KRATIX_WORKFLOW_TYPE, "promise");

        let request = MyPromise {
            params: String::from("(custom)"),
        };

        let _result = run_custom_pipeline(Some(request));

        let new_kout_path = format!("{}/.gitkeep", _result.kratix_output_dir());
        assert_eq!(Path::new(&new_kout_path).exists(), true);

        Ok(())
    }

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }
}
