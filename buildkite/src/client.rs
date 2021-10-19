use crate::agent::AgentService;
use crate::build::{BuildService, BuildServiceIterator};
use crate::http::HttpClient;
use crate::organization::OrganizationService;
use crate::pipeline::PipelineService;

/// Client is the Buildkite API client
pub struct Client {
    /// The buildkite client
    client: HttpClient,
}

impl Client {
    /// new returns a new buildkite client
    pub fn new(token: &str) -> Self {
        Client {
            client: HttpClient::new(token.to_string()),
        }
    }

    pub fn organization<'a>(&'a self) -> OrganizationService {
        OrganizationService::new(&self.client)
    }

    pub fn agent<'a>(&'a self) -> AgentService {
        AgentService::new(&self.client)
    }

    pub fn build<'a>(&'a self) -> BuildService {
        BuildService::new(&self.client)
    }

    pub fn build_iterator<'a>(&'a self, org: &str, pipeline: &str) -> BuildServiceIterator {
        BuildServiceIterator::new(&self.client, org, pipeline)
    }

    pub fn pipeline<'a>(&'a self) -> PipelineService {
        PipelineService::new(&self.client)
    }
}
