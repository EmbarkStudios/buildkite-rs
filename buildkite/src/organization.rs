use crate::http;
use crate::http::HttpClient;
use crate::iterator::{BuildkiteIterator, Paginator};
use crate::types::{Organization, Result};

use serde::{de::DeserializeOwned, Deserialize};
use std::rc::Rc;

pub struct OrganizationService {
    /// The buildkite client
    pub client: Rc<HttpClient>,
}

impl OrganizationService {
    pub fn new(client: Rc<HttpClient>) -> OrganizationService {
        OrganizationService { client }
    }

    pub fn iter<T: for<'de> Deserialize<'de>>(&self) -> BuildkiteIterator<T> {
        BuildkiteIterator::new(Box::new(OrganizationServiceIterator::new(Rc::clone(
            &self.client,
        ))))
    }

    pub fn list(&self) -> Result<Vec<Organization>> {
        let base_url = http::base_url();
        let url = format!("{}/organizations", base_url);
        self.client.get_response(url.as_str())
    }

    pub fn get(&self, org: &str) -> Result<Organization> {
        let url = http::org_url(org);
        self.client.get_response(url.as_str())
    }
}

pub struct OrganizationServiceIterator {
    pub client: Rc<HttpClient>,
}

impl<T> Paginator<T> for OrganizationServiceIterator
where
    T: DeserializeOwned,
{
    fn get_page(&self, page: u32) -> Result<Vec<T>> {
        let base_url = http::base_url();
        let url = format!("{}/organizations", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &page.to_string())]);

        result
    }
}

impl OrganizationServiceIterator {
    pub fn new(client: Rc<HttpClient>) -> OrganizationServiceIterator {
        OrganizationServiceIterator { client }
    }
}
