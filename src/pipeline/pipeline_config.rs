pub struct PipelineConfig {
    base_instance: String,
    res_dir: String,
    dep_dir: String,
    kratix_output_dir: String,
    kratix_input_dir: String,
    workflow_type: String,
}

impl PipelineConfig {
    pub fn new(
        base_instance: &str,
        res_dir: &str,
        dep_dir: &str,
        kratix_output_dir: &str,
        kratix_input_dir: &str,
        workflow_type: &str,
    ) -> PipelineConfig {
        PipelineConfig {
            base_instance: base_instance.to_string(),
            res_dir: res_dir.to_string(),
            dep_dir: dep_dir.to_string(),
            kratix_output_dir: kratix_output_dir.to_string(),
            kratix_input_dir: kratix_input_dir.to_string(),
            workflow_type: workflow_type.to_string(),
        }
    }

    pub fn base_instance(&self) -> &str {
        &self.base_instance
    }

    pub fn res_dir(&self) -> &str {
        &self.res_dir
    }

    pub fn dep_dir(&self) -> &str {
        &self.dep_dir
    }

    pub fn kratix_output_dir(&self) -> &str {
        &self.kratix_output_dir
    }

    pub fn kratix_input_dir(&self) -> &str {
        &self.kratix_input_dir
    }

    pub fn workflow_type(&self) -> &str {
        &self.workflow_type
    }
}
