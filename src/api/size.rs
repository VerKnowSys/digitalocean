use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use crate::method::List;
use crate::request::Request;
use crate::request::SizeRequest;
use crate::{ROOT_URL, STATIC_URL_ERROR};
use getset::{Getters, Setters};
use serde::Deserialize;
use serde::Serialize;
use url::Url;

const SIZES_SEGMENT: &str = "sizes";

/// The sizes objects represent different packages of hardware resources that
/// can be used for Droplets. When a Droplet is created, a size must be
/// selected so that the correct resources can be allocated.
///
/// Each size represents a plan that bundles together specific sets of
/// resources. This includes the amount of RAM, the number of virtual CPUs,
/// disk space, and transfer. The size object also includes the pricing
/// details and the regions that the size is available in.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#sizes)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
#[get = "pub"]
pub struct Size {
    /// A human-readable string that is used to uniquely identify each size.
    slug: String,

    /// This is a boolean value that represents whether new Droplets can be
    /// created with this size.
    available: bool,

    /// The amount of transfer bandwidth that is available for Droplets created
    /// in this size. This only counts traffic on the public interface. The
    /// value is given in terabytes.
    transfer: f64,

    /// This attribute describes the monthly cost of this Droplet size if the
    /// Droplet is kept for an entire month. The value is measured in US
    /// dollars.
    price_monthly: f64,

    /// This describes the price of the Droplet size as measured hourly. The
    /// value is measured in US dollars.
    price_hourly: f64,

    /// The amount of RAM allocated to Droplets created of this size. The value
    /// is represented in megabytes.
    memory: usize,

    /// The number of virtual CPUs allocated to Droplets of this size.
    vcpus: usize,

    /// The amount of disk space set aside for Droplets of this size. The value
    /// is represented in gigabytes.
    disk: usize,

    /// An array containing the region slugs where this size is available for
    /// Droplet creates.
    regions: Vec<String>,
}

impl Size {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-images)
    pub fn list() -> SizeRequest<List, Vec<Size>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SIZES_SEGMENT);

        Request::new(url)
    }
}

// There is no signular size return.

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SizeListResponse {
    sizes: Vec<Size>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Size> {
    type Response = SizeListResponse;
}

impl HasPagination for SizeListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for SizeListResponse {
    type Value = Vec<Size>;

    fn value(self) -> Vec<Size> {
        self.sizes
    }
}
