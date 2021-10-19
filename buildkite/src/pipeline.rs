use crate::http;
use crate::http::HttpClient;
use crate::types::{Pipeline, Result};

pub struct PipelineService<'a> {
    /// The buildkite client
    pub client: &'a HttpClient,
}

impl<'a> PipelineService<'a> {
    pub fn new(client: &'a HttpClient) -> PipelineService {
        PipelineService { client }
    }

    /// List pipelines returns the pipeline list
    pub fn list_pipelines(&self, organization: &str) -> Result<Vec<Pipeline>> {
        let base_url = http::org_url(organization);
        let url = format!("{}/pipelines", base_url);
        self.client.get_response(url.as_str())
    }

    /// Get pipeline returns the pipeline for the given slug
    pub fn get_pipeline(&self, organization: &str, slug: &str) -> Result<Pipeline> {
        let base_url = http::org_url(organization);
        let url = format!("{}/pipelines/{}", base_url, slug);
        self.client.get_response(url.as_str())
    }
}
