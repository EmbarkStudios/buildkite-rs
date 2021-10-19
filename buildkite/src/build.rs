use crate::http;
use crate::http::HttpClient;
use crate::types::{Build, Log, Result};

const BUILDS_PER_PAGE: u32 = 25;

pub struct BuildService<'a> {
    pub client: &'a HttpClient,
}

impl<'a> BuildService<'a> {
    pub fn new(client: &'a HttpClient) -> BuildService {
        BuildService { client }
    }

    pub fn iter_for(&self, org: &str, pipeline: &str) -> BuildServiceIterator<'a> {
        BuildServiceIterator::new(self.client, org, pipeline)
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

pub struct BuildServiceIterator<'a> {
    pub client: &'a HttpClient,
    org: String,
    pipeline: String,
    builds: Vec<Build>,
    current_page: u32,
    current_index: u32,
}

impl<'a> BuildServiceIterator<'a> {
    pub fn new(client: &'a HttpClient, org: &str, pipeline: &str) -> BuildServiceIterator<'a> {
        BuildServiceIterator {
            client,
            org: org.to_string(),
            pipeline: pipeline.to_string(),
            builds: vec![],
            current_page: 1,
            current_index: 0u32,
        }
    }

    fn next_page(&mut self) -> Result<Vec<Build>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/pipelines/{}/builds", base_url, &self.pipeline);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &self.current_page.to_string())]);

        self.current_page += 1;

        result
    }
}

impl<'a> Iterator for BuildServiceIterator<'a> {
    type Item = Build;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.current_index % BUILDS_PER_PAGE) as usize;
        self.current_index += 1;

        if idx == 0 {
            match self.next_page() {
                Ok(l) => self.builds = l,
                Err(e) => {
                    println!("Err: {:?}", e);
                    return None;
                }
            }
        }

        if self.builds.len() > idx {
            Some(self.builds[idx].clone())
        } else {
            None
        }
    }
}
