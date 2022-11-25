pub mod cli;
pub mod download;
pub mod jwt_auth;
pub mod log_init;
pub mod logs;
pub mod req_api {
    use axum::{extract::Query, Json};
    use crossbeam_channel::unbounded;
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    use crate::{download, log_init::log_writer_init};

    pub async fn list_elems(url: &str) -> Vec<String> {
        // let all_path = "/vdb/dy/del";
        let user_url = url;
        let client = Client::new();
        let headers = download::gene_headers().await;
        let (sender, recver) = unbounded();
        tokio::time::timeout(
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
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Elp {
        #[serde(rename = "douyin_text")]
        pub name: String,
        #[serde(rename = "video")]
        pub url: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Dylist {
        pub elem: Vec<Elp>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Dyurl {
        pub url: String,
        // passwd: String,
    }
    pub async fn list_dy(qu: Query<Dyurl>) -> Json<Dylist> {
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
        let ppd = Dylist { elem: newv };
        Json(ppd)
    }
}
