use anyhow::Error;
use crossbeam_channel::{Receiver, Sender};
use futures::future::BoxFuture;
use futures::FutureExt;
use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::{self, Client};
use serde_json;
use serde_json::Value;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use tokio;
use tracing::info;

///递归-异步-特殊的包装函数
pub fn req_dy_digui(
    max_cursor_num: String,
    client: Client,
    headers: HeaderMap,
    sender: Sender<String>,
    user_url: String,
) -> BoxFuture<'static, String> {
    async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        // info!("递归抓取抖音视频");
        let sec_uid = get_sec_id(user_url.as_str());
        let t1 = "https://www.iesdouyin.com/web/api/v2/aweme/post/".to_string()
            + "?"
            + "sec_uid="
            + sec_uid
            + "&count=21"
            + "&max_cursor="
            + max_cursor_num.as_str()
            + "&aid=1128"
            + "&_signature=2Vx9mxAZh0o-K4Wdv7NFKNlcfY";

        let response = client
            .get(t1)
            .headers(headers.clone())
            .send()
            .await
            .unwrap();

        let response_text = response.text().await.unwrap();
        let posts: Value = serde_json::from_str(&response_text).unwrap();

        //获取下载url和名字，并通过channel进行推送;

        for uou in 0..25 {
            tokio::time::sleep(Duration::from_millis(20)).await;
            // let url_sig = posts["aweme_list"][uou]["video"]["play_addr"]["url_list"][0].clone();
            // let arc_posts = Arc::clone(&posts);
            let dy_video = posts["aweme_list"][uou]["desc"].clone();
            let url_sig = posts["aweme_list"][uou]["video"]["play_addr"]["url_list"][0].clone();
            // let dy_video = posts["desc"][uou].clone();
            // println!("{:?}", dy_video);
            // let video_name = posts.clone();
            if url_sig.as_str() == None {
                break;
            } else {
                let dy_url = url_sig.as_str().unwrap().to_string();

                let dy_name = dy_video.as_str().unwrap();

                //send_format
                if dy_name.len() > 240 {
                    let d_name_vec: Vec<_> = dy_name.split("，").collect();
                    let info_msg = d_name_vec[0];
                    info!(info_msg);
                    let sending_msg = dy_url + "@@" + d_name_vec[0];
                    sender.send(sending_msg).unwrap();
                } else {
                    info!(dy_name);
                    let sending_msg = dy_url + "@@" + dy_name;
                    sender.send(sending_msg).unwrap();
                }
            }
        }
        let max_cursor_rt = posts["max_cursor"].as_u64();
        if max_cursor_rt == Some(0u64) {
            return "0".to_string();
        }
        req_dy_digui(
            max_cursor_rt.unwrap().to_string(),
            client,
            headers,
            sender,
            user_url,
        )
        .await
    }
    .boxed()
}

///异步下载器，提供url和reqwest client进行下载
#[cfg(target_os = "linux")]
async fn download_dy_video(
    d_url: String,
    d_name: String,
    d_path: String,
    client: &Client,
    headers: &HeaderMap,
) -> Result<(), Error> {
    let response = client.get(d_url).headers(headers.clone()).send().await?;
    //会出现部分超长名字，需要以一定规则进行切割 linux最长支持255
    let d_name_splited = d_name.to_string() + ".mp4";
    let download_path = PathBuf::from_str(&d_path)?;
    let contents = response.bytes().await?;
    let _files = tokio::fs::write(download_path.join(d_name_splited), contents).await?;
    Ok(())
}

///windows下路径，写入文件名问号格式不统一导致报错，需要处理这些特殊符号
#[cfg(target_os = "windows")]
async fn download_dy_video(
    d_url: String,
    d_name: String,
    d_path: String,
    client: &Client,
    headers: &HeaderMap,
) {
    let response = client
        .get(d_url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    //测试发现，问号格式不统一导致
    //code: 123, kind: InvalidFilename, message: "文件名、目录名或卷标语法不正确
    let ppp = d_name.replace("\n", "").replace("?", "？");
    if d_path.ends_with("\\") {
        let download_path = d_path + &ppp.trim() + ".mp4";

        let contents = response.bytes().await.unwrap();
        let _files = tokio::fs::write(download_path, contents).await.unwrap();
    } else {
        let d_path = d_path + "\\";
        let download_path = d_path + &ppp.trim() + ".mp4";

        let contents = response.bytes().await.unwrap();
        let _files = tokio::fs::write(download_path, contents).await.unwrap();
    }
}

//测试下载功能;
#[test]
fn test_download_dy() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("accept-encoding", "deflate".parse().unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("zh-CN,zh;q=0.9").unwrap(),
    );
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent","Mozilla/5.0 (iPhone; CPU iPhone OS 13_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148".parse().unwrap());
    let d_url = "https://v11.douyinvod.com/09280a4c512252b31b7beed45d0f4da5/627d101c/video/tos/cn/tos-cn-ve-15c001-alinc2/d9b0b61237664874a64a2f9824bef795/?a=1128&ch=96&cr=0&dr=0&lr=all&cd=0%7C0%7C0%7C0&cv=1&br=1706&bt=1706&cs=0&ds=3&ft=blh3-IQQqUuxfdoZPo0OW_EklpPixBGhZSX39eFmO-AqV12&mime_type=video_mp4&qs=0&rc=NjZlOWk1PDk1Ojs1M2hlaEBpMzhwbGk6Zjh2PDMzNGkzM0A1Xy4tYTFjXzMxLzYtYWIxYSNic2pxcjQwMHFgLS1kLS9zcw%3D%3D&l=2022051220480501021010504145061AA7".to_string();
    let pdpdpd = download_dy_video(
        d_url,
        "hello".to_string(),
        "".to_string(),
        &client,
        &headers,
    );
    rt.block_on(pdpdpd);
}

///构造获取user_id
pub fn get_sec_id(users: &str) -> &str {
    let sec_uid: Vec<&str> = users.split("/").collect();
    let sec_uid_step: Vec<&str> = sec_uid[4].split("?").collect();
    sec_uid_step[0]
}

#[test]
fn test_get_sec_id() {
    let sec_id = get_sec_id("https://www.douyin.com/user/MS4wLjABAAAAA_GQLUmv6kfmHaqOwyg3znCzA6eGO1fMsdTLaWp_PMXiNCo3tje6XmG9fmbstwFZ");
    println!("{:#?}", sec_id);
}

///生成请求头
pub async fn gene_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept-encoding", "deflate".parse().unwrap());
    headers.insert(
        "accept-language",
        HeaderValue::from_str("zh-CN,zh;q=0.9").unwrap(),
    );
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent","Mozilla/5.0 (iPhone; CPU iPhone OS 13_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148".parse().unwrap());
    headers
}

///直接下载
pub async fn use_recv2download_dy_video(
    recver: Receiver<String>,
    user_path: String,
) -> Result<(), Error> {
    let recv: Vec<_> = recver.try_iter().collect();
    let client = Client::new();
    let headers = gene_headers().await;
    for single_msg in recv {
        let msg_v: Vec<_> = single_msg.split("@@").collect();
        //开始下载
        // info!("开始执行下载视频");
        // let tt1 = msg_v[0].to_string();
        let tt2 = msg_v[1].to_string();
        info!("下载视频名字:{:?}", tt2);
        // println!("{}", &msg_v[0]);
        // println!("{}", &msg_v[1]);
        download_dy_video(
            msg_v[0].to_string(),
            msg_v[1].to_string(),
            user_path.to_string(),
            &client,
            &headers,
        )
        .await
        .expect("download video error");
    }
    Ok(())
}

///带进度条下载
pub async fn use_recv2download_dy_video_with_lines(
    recver: Receiver<String>,
    user_path: String,
) -> Result<(), Error> {
    let length = recver.len();
    println!("{}", length);
    let recv: Vec<_> = recver.try_iter().collect();
    let client = Client::new();
    let headers = gene_headers().await;
    //进度条初始化
    let m = MultiProgress::new();
    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("#->");
    let pb = m.add(ProgressBar::new(length as u64));
    pb.set_style(sty.clone());
    for i in 0..length {
        pb.set_message(format!("item#{}", i + 1));
        pb.inc(1);
        let msg_v: Vec<_> = recv[i].split("@@").collect();
        download_dy_video(
            msg_v[0].to_string(),
            msg_v[1].to_string(),
            user_path.to_string(),
            &client,
            &headers,
        )
        .await
        .expect("download video error");
    }
    pb.finish();
    Ok(())
}

#[cfg(test)]
mod arc_test {
    use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
    use std::{sync::Arc, time::Duration};
    ///arc 异步测试
    async fn arc_used() {
        for _ in 0..10 {
            tokio::spawn(async {
                let m = MultiProgress::new();

                let sty = ProgressStyle::with_template(
                    "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                )
                .unwrap()
                .progress_chars("##->");

                let pb = m.add(ProgressBar::new(300));
                pb.set_style(sty.clone());

                let posts = Arc::new(vec![1, 2, 3, 4, 5]);
                for i in 0..300 {
                    pb.set_message(format!("item #{}", i + 1));
                    pb.inc(1);
                    let mul_posts = Arc::clone(&posts);
                    let _pdd = mul_posts[0];
                    tokio::time::sleep(Duration::from_millis(3)).await;
                }
                pb.finish();
            })
            .await
            .unwrap();
        }
    }
    #[test]
    fn test_arc_used() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(arc_used());
    }
}

#[cfg(test)]
mod test_eline {
    use std::time::Duration;

    use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

    async fn goeline() {
        let m = MultiProgress::new();
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##->");
        let length = 100;
        let pb = m.add(ProgressBar::new(length as u64));
        pb.set_style(sty.clone());
        pb.set_style(sty.clone());
        //并发下载，进度条会卡住，造成一下子下载完成的样子，单线程异步下载
        for i in 0..length as u64 {
            pb.set_message(format!("#{}", i + 1));
            pb.inc(1);
            tokio::spawn(async {
                let _pp = 123;
            });
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        pb.finish();
    }
    #[test]
    fn test_goeline() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(goeline());
    }
}
