use super::domain::Domain;
use super::{ApiLinks, ApiMeta};
use super::{HasPagination, HasResponse, HasValue};
use crate::method::{Create, Delete, Get, List, Update};
use crate::request::{DomainRecordRequest, DomainRequest};
use crate::STATIC_URL_ERROR;
use getset::{Getters, Setters};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use url::Url;

const DOMAIN_RECORDS_SEGMENT: &str = "records";

/// Domain record resources are used to set or retrieve information about the
/// individual DNS records configured for a domain. This allows you to build
/// and manage DNS zone files by adding and modifying individual records for a
/// domain.
///
/// Requests with this output this type are accessed via [`Domain::get(..).records()`](../request/type.DomainRequest.html#method.records).
/// Make sure to check the functions in [`DomainRecordRequest`](../request/type.DomainRecordRequest.html)
///
/// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
#[derive(Deserialize, Serialize, Debug, Clone, Getters, Setters)]
#[get = "pub"]
pub struct DomainRecord {
    /// A unique identifier for each domain record.
    id: usize,

    /// The type of the DNS record (ex: A, CNAME, TXT, ...).
    ///
    /// *Note:* Since `type` is a keyword in Rust `kind` is used instead.
    #[serde(rename = "type")]
    kind: String,
    // 'type' is reserved in Rust.
    /// The name to use for the DNS record.
    name: String,

    /// The value to use for the DNS record.
    data: String,

    /// The priority for SRV and MX records.
    priority: Option<usize>,

    /// The port for SRV records.
    port: Option<usize>,

    /// This value is the time to live for the record, in seconds. This defines
    /// the time frame that clients can cache queried information before a refresh
    /// should be requested.
    ttl: usize,

    /// The weight for SRV records.
    weight: Option<usize>,
}

impl DomainRequest<Get, Domain> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#list-all-domain-records)
    pub fn records(mut self) -> DomainRecordRequest<List, Vec<DomainRecord>> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(DOMAIN_RECORDS_SEGMENT);

        self.transmute()
    }
}

impl DomainRecordRequest<List, Vec<DomainRecord>> {
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#create-a-new-domain-record)
    pub fn create<S: AsRef<str> + Display + Serialize>(
        mut self,
        kind: S,
        name: S,
        data: S,
    ) -> DomainRecordRequest<Create, DomainRecord> {
        self.url_mut().path_segments_mut().expect(STATIC_URL_ERROR);

        self.set_body(json!({
            "type": kind,
            "name": name,
            "data": data
        }));

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#retrieve-an-existing-domain-record)
    pub fn get(mut self, id: usize) -> DomainRecordRequest<Get, DomainRecord> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#update-a-domain-record)
    pub fn update(mut self, id: usize) -> DomainRecordRequest<Update, DomainRecord> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.transmute()
    }

    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#delete-a-domain-record)
    pub fn delete(mut self, id: usize) -> DomainRecordRequest<Delete, ()> {
        self.url_mut()
            .path_segments_mut()
            .expect(STATIC_URL_ERROR)
            .push(&id.to_string());

        self.transmute()
    }
}

impl DomainRecordRequest<Create, DomainRecord> {
    /// The priority for SRV and MX records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn priority(mut self, val: Option<usize>) -> Self {
        self.body_mut()["priority"] = json!(val);
        self
    }

    /// The port for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn port(mut self, val: Option<usize>) -> Self {
        self.body_mut()["port"] = json!(val);
        self
    }

    /// This value is the time to live for the record, in seconds. This defines
    /// the time frame that clients can cache queried information before a
    /// refresh should be requested.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn ttl(mut self, val: usize) -> Self {
        self.body_mut()["ttl"] = json!(val);
        self
    }

    /// The weight for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn weight(mut self, val: Option<usize>) -> Self {
        self.body_mut()["weight"] = json!(val);
        self
    }
}

impl DomainRecordRequest<Update, DomainRecord> {
    /// The record type (A, MX, CNAME, etc).
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn kind<S: AsRef<str> + Display + Serialize>(mut self, val: S) -> Self {
        self.body_mut()["type"] = json!(val);
        self
    }

    /// The host name, alias, or service being defined by the record.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn name<S: AsRef<str> + Display + Serialize>(mut self, val: S) -> Self {
        self.body_mut()["name"] = json!(val);
        self
    }

    /// Variable data depending on record type. See the Domain Records section
    /// for more detail on each record type.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn data<S: AsRef<str> + Display + Serialize>(mut self, val: S) -> Self {
        self.body_mut()["data"] = json!(val);
        self
    }

    /// The priority for SRV and MX records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn priority(mut self, val: Option<usize>) -> Self {
        self.body_mut()["priority"] = json!(val);
        self
    }

    /// The port for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn port(mut self, val: Option<usize>) -> Self {
        self.body_mut()["port"] = json!(val);
        self
    }

    /// This value is the time to live for the record, in seconds. This defines
    /// the time frame that clients can cache queried information before a
    /// refresh should be requested.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn ttl(mut self, val: usize) -> Self {
        self.body_mut()["ttl"] = json!(val);
        self
    }

    /// The weight for SRV records.
    ///
    /// [Digital Ocean Documentation.](https://developers.digitalocean.com/documentation/v2/#domain-records)
    pub fn weight(mut self, val: Option<usize>) -> Self {
        self.body_mut()["weight"] = json!(val);
        self
    }
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DomainRecordResponse {
    domain_record: DomainRecord,
}

impl HasValue for DomainRecordResponse {
    type Value = DomainRecord;

    fn value(self) -> DomainRecord {
        self.domain_record
    }
}

impl HasResponse for DomainRecord {
    type Response = DomainRecordResponse;
}

/// Response type returned from Digital Ocean.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DomainRecordListResponse {
    domain_records: Vec<DomainRecord>,
    links: ApiLinks,
    meta: ApiMeta,
}

impl HasResponse for Vec<DomainRecord> {
    type Response = DomainRecordListResponse;
}

impl HasPagination for DomainRecordListResponse {
    fn next_page(&self) -> Option<Url> {
        self.links.next()
    }
}

impl HasValue for DomainRecordListResponse {
    type Value = Vec<DomainRecord>;

    fn value(self) -> Vec<DomainRecord> {
        self.domain_records
    }
}
