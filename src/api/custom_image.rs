use super::{HasResponse, HasValue};
use crate::method::Create;
use crate::request::CustomImageRequest;
use crate::request::Request;
use crate::{ROOT_URL, STATIC_URL_ERROR};
use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

const IMAGES_SEGMENT: &str = "images";

/// A custom image is an image with an user-supplied raw image.
/// The body must contain a url attribute pointing to a Linux virtual machine image to be imported into DigitalOcean.
/// The image must be in the raw, qcow2, vhdx, vdi, or vmdk format.
/// It may be compressed using gzip or bzip2 and must be smaller than 100 GB after being decompressed.
///
/// [Digital Ocean Documentation.](https://www.digitalocean.com/docs/images/custom-images/)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
pub struct CustomImage {
    /// A unique number that can be used to identify and reference a specific
    /// image.
    id: usize,

    /// The display name that has been given to an image. This is what is shown
    /// in the control panel and is generally a descriptive title for the image
    /// in question.
    name: String,

    /// The kind of image, describing the duration of how long the image is
    /// stored. This is either "snapshot" or "backup".
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    kind: String,
    // 'type' is reserved in Rust.
    /// This attribute describes the base distribution used for this image.
    distribution: String,

    /// This attribute is an array of the regions that the image is available
    /// in. The regions are represented by their identifying slug values.
    regions: Vec<String>,

    /// Tags to quickly find an image or to group multiple images
    /// under a common name
    tags: Vec<String>,

    /// A time value given in ISO8601 combined date and time format that
    /// represents when the Image was created.
    created_at: DateTime<Utc>,

    /// A brief description about the image
    description: String,

    /// The status of the image
    status: String,
}

impl CustomImage {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-custom-image)
    pub fn create<S: AsRef<str> + Display + Serialize>(
        name: S,
        image_url: S,
        region: S,
        distribution: S,
        desc: S,
        tags: Vec<S>,
    ) -> CustomImageRequest<Create, CustomImage> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(IMAGES_SEGMENT);

        let mut req = Request::new(url);
        req.set_body(json!({
            "name": name,
            "url": image_url,
            "region": region,
            "distribution": distribution,
            "description": desc,
            "tags": tags
        }));
        req
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomImageResponse {
    image: CustomImage,
}

impl HasResponse for CustomImage {
    type Response = CustomImageResponse;
}

impl HasValue for CustomImageResponse {
    type Value = CustomImage;

    fn value(self) -> CustomImage {
        self.image
    }
}
