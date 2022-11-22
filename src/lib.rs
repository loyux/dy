pub mod cli;
pub mod download;
mod log_init;
pub mod logs;
mod req_api {

    //提供一个端口，通过url能够获取该url的所有发的视频的标题和链接，在选择进行下载，或者全部下载

    use std::{collections::HashMap, fmt::Pointer, net::SocketAddr, time::Duration};

    use axum::{
        extract::Query,
        http::uri::Authority,
        response::{IntoResponse, Response},
        routing::{get, post},
        Json, Router,
    };
    use crossbeam_channel::unbounded;
    use reqwest::Client;
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_json::{json, Value};

    use crate::{
        download::{self, download_dy_video},
        log_init::log_writer_init,
    };

    async fn list_elems(url: &str) -> Vec<String> {
        let all_path = "/vdb/dy/del";
        let user_url = url;
        let client = Client::new();
        let headers = download::gene_headers().await;
        let (sender, recver) = unbounded();
        let pt1 = tokio::time::timeout(
            Duration::from_secs(60),
            download::req_dy_digui(
                "".to_string(),
                client,
                headers,
                sender,
                user_url.to_string(),
            ),
        )
        .await
        .unwrap();

        // let pt1 = tokio::spawn(download::req_dy_digui(
        //     "".to_string(),
        //     client,
        //     headers,
        //     sender,
        //     user_url.to_string(),
        // ))
        // .await;
        let ve: Vec<_> = recver.try_iter().collect();
        // dbg!(ve);
        ve
    }
    #[tokio::test]
    async fn wo() {
        log_writer_init().unwrap();
        list_elems("https://www.douyin.com/user/MS4wLjABAAAAkpFlVUVUjmDjzqrX-sZJTjkkB8chzoIcKljL2qYGSvlJtC7JoeLJH3-MaU53UVp9").await;
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Plw {
        title: String,
        url: String,
    }
    #[derive(Serialize, Deserialize)]
    struct Elp {
        #[serde(rename = "kaka")]
        name: String,
        #[serde(rename = "sadas")]
        url: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Kaka {
        elem: Vec<Elp>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Dyurl {
        url: String,
        // passwd: String,
    }
    pub async fn list_dy(qu: Query<Dyurl>) -> Json<Kaka> {
        // dbg!(qu);
        // let d = ;
        let pd = list_elems(&qu.url).await;
        // println!("{:?}", pd);
        let mut newv = Vec::new();
        // let mut ppp = HashMap::new();
        for i in pd {
            let dc: Vec<String> = i.split("@@").map(|x| x.to_string()).collect();
            // let d1 = format!("name:{}, url:{}", dc[0], dc[1]);
            let elp = Elp {
                name: dc[1].clone(),
                url: dc[0].clone(),
            };
            newv.push(elp);
        }
        let ppd = Kaka { elem: newv };
        Json(ppd)
    }

    // #[tokio::test]
    // async fn fmls() {
    //     list_dy().await;
    // }

    pub async fn handle() -> Json<Value> {
        println!("aaaaaaaaaaaa");
        Json(json!({"a":"sad","b":"asd"}))
    }
    #[tokio::test]
    async fn wuo() {
        log_writer_init().unwrap();
        let app = Router::new().route("/api", get(list_dy));

        let addr = SocketAddr::from(([0, 0, 0, 0], 80));
        println!("listening on {addr}");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
