use super::region::Region;
use super::snapshot::Snapshot;
use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use crate::method::{Create, Delete, Get, List};
use crate::request::Request;
use crate::request::{SnapshotRequest, VolumeRequest};
use crate::{ROOT_URL, STATIC_URL_ERROR};
use chrono::{DateTime, Utc};
use getset::{Getters, Setters};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use url::Url;

const VOLUME_SEGMENT: &str = "volumes";
const SNAPSHOTS_SEGMENT: &str = "snapshots";

/// Block Storage volumes provide expanded storage capacity for your Droplets
/// and can be moved between Droplets within a specific region. Volumes
/// function as raw block devices, meaning they appear to the operating system
/// as locally attached storage which can be formatted using any file system
/// supported by the OS. They may be created in sizes from 1GiB to 16TiB.
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#block-storage)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
#[get = "pub"]
pub struct Volume {
    /// The unique identifier for the Block Storage volume.
    id: String,

    /// The region that the Block Storage volume is located in. When setting a
    /// region, the value should be the slug identifier for the region. When
    /// you query a Block Storage volume, the entire region object will be
    /// returned.
    region: Region,

    /// An array containing the IDs of the Droplets the volume is attached to.
    /// Note that at this time, a volume can only be attached to a single
    /// Droplet.
    droplet_ids: Vec<usize>,

    /// A human-readable name for the Block Storage volume. Must be lowercase
    /// and be composed only of numbers, letters and "-", up to a limit of 64
    /// characters.
    name: String,

    /// An optional free-form text field to describe a Block Storage volume.
    description: String,

    /// The size of the Block Storage volume in GiB (1024^3).
    size_gigabytes: f32,

    /// A time value given in ISO8601 combined date and time format that
    /// represents when the Block Storage volume was created.
    created_at: DateTime<Utc>,
}

impl Volume {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-block-storage-volumes)
    pub fn list() -> VolumeRequest<List, Vec<Volume>> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT);

        Request::new(url)
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-block-storage-volume)
    pub fn create<S>(name: S, size_gigabytes: usize) -> VolumeRequest<Create, Volume>
    where
        S: AsRef<str> + Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT);

        let mut req = Request::new(url);
        req.set_body(json!({
            "name": name,
            "size_gigabytes": size_gigabytes,
        }));
        req
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-block-storage-volume)
    pub fn get<S: AsRef<str> + Serialize + Display>(id: S) -> VolumeRequest<Get, Volume> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-block-storage-volume-by-name)
    pub fn get_by_name<S>(name: S, region: S) -> VolumeRequest<Get, Volume>
    where
        S: AsRef<str> + Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT);

        url.query_pairs_mut()
            .append_pair("name", name.as_ref())
            .append_pair("region", region.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-block-storage-volume)
    pub fn delete<S: AsRef<str> + Serialize + Display>(id: S) -> VolumeRequest<Delete, ()> {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT)
            .push(id.as_ref());

        Request::new(url)
    }
    /// [Digital Ocean Documentation.](hhttps://developers.digitalocean.com/documentation/v2/#delete-a-block-storage-volume-by-name)
    pub fn delete_by_name<S>(name: S, region: S) -> VolumeRequest<Delete, ()>
    where
        S: AsRef<str> + Serialize + Display,
    {
        let mut url = ROOT_URL.clone();
        url.path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(VOLUME_SEGMENT);

        url.query_pairs_mut()
            .append_pair("name", name.as_ref())
            .append_pair("region", region.as_ref());

        Request::new(url)
    }
}

impl VolumeRequest<List, Vec<Volume>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-block-storage-volumes)
    pub fn region<S: AsRef<str> + Serialize + Display>(mut self, region: S) -> Self {
        self.url_mut()
            .query_pairs_mut()
            .append_pair("region", region.as_ref());

        self
    }
}

impl VolumeRequest<Get, Volume> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-snapshots-for-a-volume)
    pub fn snapshots(mut self) -> SnapshotRequest<List, Vec<Snapshot>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SNAPSHOTS_SEGMENT);

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-snapshot-from-a-volume)
    pub fn snapshot<S>(mut self, name: S) -> SnapshotRequest<Create, Snapshot>
    where
        S: AsRef<str> + Serialize + Display,
    {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(SNAPSHOTS_SEGMENT);

        self.set_body(json!({ "name": name }));

        self.transmute()
    }
}

impl VolumeRequest<Create, Volume> {
    /// An optional free-form text field to describe a Block Storage volume.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#block-storage)
    pub fn description<S: AsRef<str> + Serialize + Display>(mut self, val: S) -> Self {
        self.body_mut()["description"] = json!(val);
        self
    }

    /// The region where the Block Storage volume will be created. When setting
    /// a region, the value should be the slug identifier for the region. When
    /// you query a Block Storage volume, the entire region object will be
    /// returned.
    ///
    /// **Note:** Should not be specified with a `snapshot_id`.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#block-storage)
    pub fn region<S: AsRef<str> + Serialize + Display>(mut self, val: S) -> Self {
        self.body_mut()["region"] = json!(val);
        self
    }

    /// The unique identifier for the volume snapshot from which to create the
    /// volume.
    ///
    /// **Note:** Should not be specified with a `region_id`.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#block-storage)
    pub fn snapshot_id<S: AsRef<str> + Serialize + Display>(mut self, val: S) -> Self {
        self.body_mut()["snapshot_id"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VolumeListResponse {
    volumes: Vec<Volume>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<Volume> {
    type Response = VolumeListResponse;
}

impl HasPagination for VolumeListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for VolumeListResponse {
    type Value = Vec<Volume>;

    fn value(self) -> Vec<Volume> {
        self.volumes
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VolumeResponse {
    volume: Volume,
}

impl HasResponse for Volume {
    type Response = VolumeResponse;
}

impl HasValue for VolumeResponse {
    type Value = Volume;

    fn value(self) -> Volume {
        self.volume
    }
}
