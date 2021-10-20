use crate::http;
use crate::http::HttpClient;
use crate::iterator::{BuildkiteIterator, Paginator};
use crate::types::{Pipeline, Result};

use serde::{de::DeserializeOwned, Deserialize};
use std::rc::Rc;

pub struct PipelineService {
    /// The buildkite client
    pub client: Rc<HttpClient>,
}

impl PipelineService {
    pub fn new(client: Rc<HttpClient>) -> PipelineService {
        PipelineService { client }
    }

    pub fn iter_for<T: for<'de> Deserialize<'de>>(&self, org: &str) -> BuildkiteIterator<T> {
        BuildkiteIterator::new(Box::new(PipelineServiceIterator::new(
            Rc::clone(&self.client),
            org,
        )))
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

pub struct PipelineServiceIterator {
    pub client: Rc<HttpClient>,
    org: String,
}

impl<T> Paginator<T> for PipelineServiceIterator
where
    T: DeserializeOwned,
{
    fn get_page(&self, page: u32) -> Result<Vec<T>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/pipelines", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &page.to_string())]);

        result
    }
}

impl PipelineServiceIterator {
    pub fn new(client: Rc<HttpClient>, org: &str) -> PipelineServiceIterator {
        PipelineServiceIterator {
            client,
            org: org.to_string(),
        }
    }
}
