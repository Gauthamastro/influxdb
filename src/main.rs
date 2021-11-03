use influxdb2::{Client};

use chrono::{DateTime, Utc};
use influxdb2::models::{DataPoint, LanguageRequest, Query};
use futures::prelude::*;

const BUCKET: &str = "engineering's Bucket";
const HOST: &str = "https://eu-central-1-1.aws.cloud2.influxdata.com";
const ORG: &str = "polkadex";
const TOKEN: &str = "";

#[tokio::main]
async fn main() {

    let client = Client::new(HOST, ORG, TOKEN);

    let points = vec![
        DataPoint::builder("cpu")
            .tag("host", "server01")
            .field("usage", 0.5)
            .build().unwrap(),
        DataPoint::builder("cpu")
            .tag("host", "server01")
            .tag("region", "us-west")
            .field("usage", 0.87)
            .build().unwrap(),
    ];

    client.write(BUCKET, stream::iter(points)).await.unwrap();


}