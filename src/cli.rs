use std::collections::HashMap;

use clap::Parser;

use crate::download;
use anyhow::Error;
use crossbeam_channel::unbounded;
use reqwest::Client;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// 抖音视频主页url
    #[clap(short, long)]
    pub url: String,

    /// 下载地址(/home)
    #[clap(short, long)]
    pub path: String,

    ///单线程(single)vs多线程(multi)
    #[clap(short, long)]
    pub threading: Option<String>,

    ///txt,csv file
    #[clap(long)]
    pub file: Option<String>,
}

pub async fn cli_run() -> Result<(), Error> {
    let args = Args::parse();
    let all_path = args.path;
    let user_url = args.url;
    let client = Client::new();
    let headers = download::gene_headers().await;
    let (sender, recver) = unbounded();
    //获取下载链接
    let pt1 = tokio::spawn(download::req_dy_digui(
        "".to_string(),
        client,
        headers,
        sender,
        user_url,
    ))
    .await;
    match pt1 {
        Ok(_) => println!("1"),
        Err(_) => println!("2"),
    }

    match args.threading {
        Some(value) => {
            if value.as_str() == "single" {
                tokio::spawn(download::use_recv2download_dy_video_with_lines(
                    recver.clone(),
                    all_path.clone(),
                ))
                .await?
                .unwrap();
            }
            if value.as_str() == "multi" {
                loop {
                    if recver.is_empty() {
                        break;
                    } else {
                        tokio::spawn(download::use_recv2download_dy_video(
                            recver.clone(),
                            all_path.clone(),
                        ))
                        .await?
                        .unwrap();
                    }
                }
            }
        }
        _ => {
            tokio::spawn(download::use_recv2download_dy_video_with_lines(
                recver,
                all_path.clone(),
            ))
            .await?
            .unwrap();
        }
    };
    Ok(())
}
