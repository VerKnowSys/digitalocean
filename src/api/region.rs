use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use crate::method::List;
use crate::request::RegionRequest;
use crate::request::Request;
use crate::{ROOT_URL, STATIC_URL_ERROR};
use getset::{Getters, Setters};
use serde::Deserialize;
use serde::Serialize;
use url::Url;

const REGIONS_SEGMENT: &str = "regions";

/// A region in DigitalOcean represents a datacenter where Droplets can be
/// deployed and images can be transferred.
///
/// Each region represents a specific datacenter in a geographic location. Some
/// geographical locations may have multiple "regions" available. This means
/// that there are multiple datacenters available within that area.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#regions)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
#[get = "pub"]
pub struct Region {
    /// A human-readable string that is used as a unique identifier for each
    /// region.
    name: String,

    /// The display name of the region. This will be a full name that is used
    /// in the control panel and other interfaces.
    slug: String,

    /// This attribute is set to an array which contains the identifying slugs
    ///  for the sizes available in this region.
    sizes: Vec<String>,

    /// This is a boolean value that represents whether new Droplets can be
    /// created in this region.
    available: bool,

    /// This attribute is set to an array which contains features available in
    /// this region
    features: Vec<String>,
}

impl Region {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-regions)
    pub fn list() -> RegionRequest<List, Vec<Region>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(REGIONS_SEGMENT);

        Request::new(url)
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RegionListResponse {
    regions: Vec<Region>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Region> {
    type Response = RegionListResponse;
}

impl HasPagination for RegionListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for RegionListResponse {
    type Value = Vec<Region>;

    fn value(self) -> Vec<Region> {
        self.regions
    }
}
