use crate::http;
use crate::http::HttpClient;
use crate::iterator::{BuildkiteIterator, Paginator};
use crate::types::{Build, Log, Result};

use serde::{de::DeserializeOwned, Deserialize};
use std::rc::Rc;

pub struct BuildService {
    pub client: Rc<HttpClient>,
}

impl BuildService {
    pub fn new(client: Rc<HttpClient>) -> BuildService {
        BuildService { client }
    }

    pub fn iter_for<T: for<'de> Deserialize<'de>>(
        &self,
        org: &str,
        pipeline: &str,
    ) -> BuildkiteIterator<T> {
        BuildkiteIterator::new(Box::new(BuildServiceIterator::new(
            Rc::clone(&self.client),
            org,
            pipeline,
        )))
    }

    pub fn list(&mut self, org: &str, pipeline: &str) -> Result<Vec<Build>> {
        let base_url = http::org_url(org);
        let url = format!("{}/pipelines/{}/builds", base_url, pipeline);
        self.client.get_response(url.as_str())
    }

    pub fn logs(&self, org: &str, pipeline: &str, build: &str, job: &str) -> Result<Log> {
        let base_url = http::org_url(org);
        let url = format!(
            "{}/pipelines/{}/builds/{}/jobs/{}/log",
            base_url, pipeline, build, job
        );
        self.client.get_response(url.as_str())
    }
}

pub struct BuildServiceIterator {
    pub client: Rc<HttpClient>,
    org: String,
    pipeline: String,
}

impl<T> Paginator<T> for BuildServiceIterator
where
    T: DeserializeOwned,
{
    fn get_page(&self, page: u32) -> Result<Vec<T>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/pipelines/{}/builds", base_url, &self.pipeline);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &page.to_string())]);

        result
    }
}

impl BuildServiceIterator {
    pub fn new(client: Rc<HttpClient>, org: &str, pipeline: &str) -> BuildServiceIterator {
        BuildServiceIterator {
            client,
            org: org.to_string(),
            pipeline: pipeline.to_string(),
        }
    }
}
