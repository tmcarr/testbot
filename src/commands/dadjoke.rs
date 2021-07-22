use serde::{Serialize, Deserialize};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub jsonapi: Jsonapi,
    pub data: Vec<Daum>,
    pub meta: Meta2,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jsonapi {
    pub version: String,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: SelfField,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelfField {
    #[serde(rename = "Global Quote")]
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub links: Links,
    pub attributes: Attributes,
    pub relationships: Relationships,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(rename = "drupal_internal__nid")]
    pub drupal_internal_nid: i64,
    #[serde(rename = "drupal_internal__vid")]
    pub drupal_internal_vid: i64,
    pub langcode: String,
    #[serde(rename = "revision_timestamp")]
    pub revision_timestamp: String,
    #[serde(rename = "revision_log")]
    pub revision_log: ::serde_json::Value,
    pub status: bool,
    pub title: String,
    pub created: String,
    pub changed: String,
    pub promote: bool,
    pub sticky: bool,
    #[serde(rename = "default_langcode")]
    pub default_langcode: bool,
    #[serde(rename = "revision_translation_affected")]
    pub revision_translation_affected: bool,
    pub metatag: ::serde_json::Value,
    pub path: Path,
    #[serde(rename = "field_joke_opener")]
    pub field_joke_opener: String,
    #[serde(rename = "field_joke_response")]
    pub field_joke_response: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub alias: String,
    pub pid: i64,
    pub langcode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    #[serde(rename = "node_type")]
    pub node_type: NodeType,
    #[serde(rename = "revision_uid")]
    pub revision_uid: RevisionUid,
    pub uid: Uid,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeType {
    pub data: Data,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Related {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionUid {
    pub data: Data,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uid {
    pub data: Data,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Omitted {
    pub detail: String,
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Help {
    pub href: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item9QK6Jlx {
    pub href: String,
    pub meta: Meta3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub links: Links,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta2 {
    pub omitted: Omitted,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta3 {
    pub rel: String,
    pub detail: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next {
    pub href: String,
}


// Results limited with something like this: https://www.fatherhood.gov/jsonapi/node/dad_jokes?page[limit]=1&page[offset]=51

#[command]
#[aliases("joke", "dj")]
#[description = "Presents a random dad joke"]
#[usage = ""]
async fn dadjoke(ctx: &Context, msg: &Message) -> CommandResult {
    const ENDPOINT: &str = "https://fatherhood.gov/jsonapi/node/dad_jokes";
    let jokes = reqwest::get(ENDPOINT).await?.json::<Root>().await?;

    let results = format!("{}? {}", jokes.data.Daum.0.field_joke_opener, jokes.data.Daum.0.field_joke_response);

    let _ = msg.channel_id.say(&ctx.http, results).await;
    Ok(())
}
