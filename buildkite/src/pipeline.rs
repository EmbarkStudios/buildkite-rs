use crate::http;
use crate::http::HttpClient;
use crate::types::{Pipeline, Result};

const PIPELINES_PER_PAGE: u32 = 25;

pub struct PipelineService<'a> {
    /// The buildkite client
    pub client: &'a HttpClient,
}

impl<'a> PipelineService<'a> {
    pub fn new(client: &'a HttpClient) -> PipelineService {
        PipelineService { client }
    }

    pub fn iter_for(&self, org: &str) -> PipelineServiceIterator<'a> {
        PipelineServiceIterator::new(self.client, org)
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

pub struct PipelineServiceIterator<'a> {
    pub client: &'a HttpClient,
    org: String,
    pipelines: Vec<Pipeline>,
    current_page: u32,
    current_index: u32,
}

impl<'a> PipelineServiceIterator<'a> {
    pub fn new(client: &'a HttpClient, org: &str) -> PipelineServiceIterator<'a> {
        PipelineServiceIterator {
            client,
            org: org.to_string(),
            pipelines: vec![],
            current_page: 1,
            current_index: 0u32,
        }
    }

    fn next_page(&mut self) -> Result<Vec<Pipeline>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/pipelines", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &self.current_page.to_string())]);

        self.current_page += 1;

        result
    }
}

impl<'a> Iterator for PipelineServiceIterator<'a> {
    type Item = Pipeline;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.current_index % PIPELINES_PER_PAGE) as usize;
        self.current_index += 1;

        if idx == 0 {
            match self.next_page() {
                Ok(l) => self.pipelines = l,
                Err(e) => {
                    println!("Err: {:?}", e);
                    return None;
                }
            }
        }

        if self.pipelines.len() > idx {
            Some(self.pipelines[idx].clone())
        } else {
            None
        }
    }
}
