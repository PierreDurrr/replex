use crate::models::*;
use crate::utils::*;
use anyhow::Result;
use axum::http::{uri::Uri, Request};
// use crate::axum::http::{uri::Uri, Request, Response};
use cached::proc_macro::cached;
use http::HeaderValue;

use http::Response;
use hyper::client::HttpConnector;
use hyper::Body;

use std::convert::TryFrom;

type HttpClient = hyper::client::Client<HttpConnector, Body>;
#[derive(Debug, Clone)]
pub struct PlexClient {
    pub http_client: HttpClient,
    pub host: String,

    pub accept: String,

    // /// `X-Plex-Provides` header value. Comma-separated list.
    // ///
    // /// Should be one or more of `controller`, `server`, `sync-target`, `player`.
    // pub x_plex_provides: String,

    // /// `X-Plex-Platform` header value.
    // ///
    // /// Platform name, e.g. iOS, macOS, etc.
    // pub x_plex_platform: String,

    // /// `X-Plex-Platform-Version` header value.
    // ///
    // /// OS version, e.g. 4.3.1
    // pub x_plex_platform_version: String,

    // /// `X-Plex-Product` header value.
    // ///
    // /// Application name, e.g. Laika, Plex Media Server, Media Link.
    // pub x_plex_product: String,

    // /// `X-Plex-Version` header value.
    // ///
    // /// Application version, e.g. 10.6.7.
    // pub x_plex_version: String,

    // /// `X-Plex-Device` header value.
    // ///
    // /// Device name and model number, e.g. iPhone3,2, Motorola XOOM™, LG5200TV.
    // pub x_plex_device: String,

    // /// `X-Plex-Device-Name` header value.
    // ///
    // /// Primary name for the device, e.g. "Plex Web (Chrome)".
    // pub x_plex_device_name: String,
    /// `X-Plex-Client-Identifier` header value.
    ///
    /// UUID, serial number, or other number unique per device.
    ///
    /// **N.B.** Should be unique for each of your devices.
    pub x_plex_client_identifier: String,

    /// `X-Plex-Token` header value.
    ///
    /// Auth token for Plex.
    pub x_plex_token: String,

    /// `X-Plex-Sync-Version` header value.
    ///
    /// Not sure what are the valid values, but at the time of writing Plex Web sends `2` here.
    pub x_plex_sync_version: String,
}

impl PlexClient {
    pub fn get(&self, path: String) -> hyper::client::ResponseFuture {
        // let path = req.uri().path();
        // let path_query = req
        //     .uri()
        //     .path_and_query()
        //     .map(|v| v.as_str())
        //     .unwrap_or(path);
        // let uri = format!("{}{}", self.host, path_query);

        // Default is gzip. Dont want that
        // req.headers_mut()
        //     .insert("Accept-Encoding", HeaderValue::from_static("identity"));

        let uri = format!("{}{}", self.host, path);
        // dbg!(&uri);
        let mut request = Request::builder()
            .uri(uri)
            .header("X-Plex-Client-Identifier", &self.x_plex_client_identifier)
            .header("X-Plex-Token", &self.x_plex_token)
            .header("Accept", &self.accept)
            .body(Body::empty())
            .unwrap();
        self.http_client.request(request)
    }

    pub async fn get_section_collections(&self, id: u32) -> Result<Vec<MetaData>> {
        let mut resp = self
            .get(format!("/library/sections/{}/collections", id))
            .await
            .unwrap();
        // dbg!(&resp);
        let mut container: MediaContainerWrapper<MediaContainer> =
            from_response(resp).await.unwrap();
        // dbg!(&container.media_container.children());
        // let plex_client = create_client_from_request(&req).unwrap();
        // let plex_api = plex_api::Server::new("http://100.91.35.113:32400", plex_client).await.unwrap();
        // let mut collections = vec![];
        // let api = self.plex_api.clone().unwrap();
        // for library in api.libraries() {
        //     // library.media

        //     let mut resp: MediaContainerWrapper<MediaContainer> = api
        //         .client()
        //         .get(format!("/library/sections/{}/collections", library.id()))
        //         .json()
        //         .await?;
        //     collections.append(&mut resp.media_container.metadata);
        // }
        // println!("no cache");
        // Ok(MediaContainerWrapper::default().media_container.metadata)
        Ok(container.media_container.children())
    }

    pub async fn get_item_by_key(
        self,
        key: String,
    ) -> Result<MediaContainerWrapper<MediaContainer>> {
        let mut resp = self.get(key).await.unwrap();
        let mut container: MediaContainerWrapper<MediaContainer> =
            from_response(resp).await.unwrap();
        Ok(container)
    }
}

impl From<&Request<Body>> for PlexClient {
    fn from(req: &Request<Body>) -> Self {
        Self {
            http_client: HttpClient::new(),
            host: "http://100.91.35.113:32400".to_string(),
            x_plex_token: get_header_or_param("x-plex-token".to_string(), &req).unwrap(),
            x_plex_client_identifier: get_header_or_param(
                "x-plex-client-identifier".to_string(),
                &req,
            )
            .unwrap(),
            x_plex_sync_version: "2".to_owned(),
            accept: get_header_or_param("Accept".to_string(), &req).unwrap()
        }
    }
}
