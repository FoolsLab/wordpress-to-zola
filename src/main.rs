use std::{env, fs::File, io::{Read, Write}};

use chrono::{NaiveDate, format::format, DateTime};
use serde::Deserialize;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Deserialize)]
struct Channel {
    // title: String,
    #[serde(rename = "item")]
    items: Vec<Article>,
}

// #[derive(Deserialize)]
// #[serde(rename_all = "kebab-case")]
// enum ChannelItem {
//     Item(Article),
//     Title(String),
//     Link(String),
//     Description(String),
//     #[serde(rename = "pubDate")]
//     PubDate(String),
//     Language(String),
//     #[serde(rename = "wp__wxr_version")]
//     Version(String),
//     #[serde(rename = "wp__base_site_url")]
//     SiteUrl(String),
//     #[serde(rename = "wp__base_blog_url")]
//     BlogUrl(String),
//     #[serde(rename = "wp__author")]
//     Author(String),
//     Generator(String),
// }

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    link: String,

    #[serde(rename = "pubDate")]
    pub_date: String,

    #[serde(rename = "content__encoded")]
    content: String,

    #[serde(rename = "wp__post_date")]
    post_date: String,

    #[serde(rename = "wp__status")]
    status: String,
    
    #[serde(rename = "wp__post_name")]
    post_name: String,

    
    #[serde(rename = "wp__post_id")]
    post_id: i32,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_name = &args[1];

    println!("{}", file_name);

    let mut file = File::open(file_name)?;
    let mut xml = String::new();
    file.read_to_string(&mut xml)?;

    let data :Channel = serde_xml_rs::from_str(xml.as_str())?;

    for item in data.items {
        let post_date = NaiveDate::parse_from_str(item.post_date.as_str(), "%Y-%m-%d %H:%M:%S")?;

        let content = item.content.as_str();

        match item.status.as_str() {
            "draft" | "private" => {
                let meta = format!(
"+++
title = \"{}\"
date = {}
draft = true
+++
", item.title, post_date.format("%Y-%m-%d"));
                println!("draft: {}", item.title);
                let mut out_file = File::create(format!("data/drafts/{}.md", item.post_id))?;
                
                out_file.write_all(meta.as_bytes())?;
                out_file.write_all(content.as_bytes())?;
            },
            "publish" => {
                let meta = format!(
"+++
title = \"{}\"
date = {}
+++
", item.title, post_date.format("%Y-%m-%d"));
                println!("published: {}", item.title);
                let mut out_file = File::create(format!("data/{}.md", item.post_name))?;

                out_file.write_all(meta.as_bytes())?;
                out_file.write_all(content.as_bytes())?;
            },
            ty => {
                println!("unknown: {}, {}", item.title, ty);
            }
        }
    }

    Ok(())
}
