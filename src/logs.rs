use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn log_init() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // this creates a new event, outside of any spans.
}

#[cfg(test)]
mod test_crossbeam_channel {
    use std::time::Duration;

    use super::log_init;
    use crossbeam_channel::unbounded;
    use tracing::info;

    ///测试函数，测试crossbeam，loop并发
    /// 复制sender或者revicer仅仅是复制了句柄，不会创建新的通道
    ///单生产者多消费者模式，异步并发
    async fn multy_channel_use() {
        log_init();
        let (sender, recver) = unbounded();
        for i in 1..100 {
            sender.send(i).unwrap();
        }
        //多消费者
        loop {
            if recver.is_empty() {
                break;
            }
            // info!("开始了一个新的loop");
            let t1 = recver.clone();
            tokio::spawn(async move {
                let pp = t1.recv().unwrap();
                println!("{}", pp);
            })
            .await
            .unwrap();
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    ///控制并发度
    async fn multy_channel_use_control() {
        log_init();
        let (sender, recver) = unbounded();
        for i in 1..10 {
            sender.send(i).unwrap();
        }
        std::thread::spawn(move || {
            let sed = sender.clone();
            for pp in 20..30 {
                sed.send(pp);
            }
        });
        //多消费者
        //获取一个通道接收者句柄，向里面传输数据
        for i in 0..1000 {
            if recver.is_empty() {
                break;
            }
            // info!("开始了第{}新的loop", i);
            let t1 = recver.clone();
            tokio::spawn(async move {
                let pp = t1.recv().unwrap();
                println!("{}", pp);
            })
            .await
            .unwrap();
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    #[test]
    fn test_multy_channel_use() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(multy_channel_use_control());
    }
}

#[cfg(test)]
mod test_os_path {
    use std::{path::PathBuf, str::FromStr};

    ///测试path
    fn path_useful() {
        let slistr = r#"/home/admin"#;
        let ddd = PathBuf::from_str(slistr).unwrap().join("asdadsad");
        // / assert_/eq!(Path::new("/etc").join("passwd"), PathBuf::from("/etc/passwd"));
        println!("{:?}", ddd);
    }

    #[test]
    fn test_path_useful() {
        path_useful();
    }
}

///无法进行异步迭代的函数，废弃
// async fn req_dy(max_cursor_num: &str, client: Client, headers: HeaderMap, sender: Sender<String>) {
//     //内循环迭代cusor
//     let sec_uid = get_sec_id("https://www.douyin.com/user/MS4wLjABAAAAA_GQLUmv6kfmHaqOwyg3znCzA6eGO1fMsdTLaWp_PMXiNCo3tje6XmG9fmbstwFZ");
//     let t1 = "https://www.iesdouyin.com/web/api/v2/aweme/post/".to_string()
//         + "?"
//         + "sec_uid="
//         + sec_uid
//         + "&count=21"
//         + "&max_cursor="
//         + max_cursor_num
//         + "&aid=1128"
//         + "&_signature=2Vx9mxAZh0o-K4Wdv7NFKNlcfY";

//     let response = client
//         .get(t1)
//         .headers(headers.clone())
//         .send()
//         .await
//         .unwrap();

//     let response_text = response.text().await.unwrap();
//     // // println!("{:?}",response_text);
//     let posts: Value = serde_json::from_str(&response_text).unwrap();

//     // user_name = js['aweme_list'][0]['author']['nickname']
//     let user_name = posts["aweme_list"][0]["author"]["nickname"]
//         .as_str()
//         .unwrap()
//         .clone();
//     for uou in 0..20 {
//         let url_sig = posts["aweme_list"][uou]["video"]["play_addr"]["url_list"][0].clone();
//         println!("{:?}", url_sig);
//     }
//     let max_cursor_rt = posts["max_cursor"].as_u64();
//     println!("{:?}", max_cursor_rt);
// }

///递归验证
#[cfg(test)]
mod test_digui {
    use futures::{future::BoxFuture, FutureExt};

    #[allow(dead_code)]
    fn recursive<'a>(parent_name: &'a str, mut times: i32) -> BoxFuture<'a, i32> {
        async move {
            // 在屏幕上输出名字和次数
            println!("parent name : {}{}", parent_name, times);
            times += 1;
            // let new_name = format!("{}{}", parent_name, times);
            // 执行100次后
            if times <= 10 {
                // 对字符串和次数进行处理
                // recursive(new_name, times).await;
                // 对返回值进行处理
                // x = recursive(new_name, times).await;
                recursive(parent_name, times).await;
            }
            // 返回值
            times
        }
        .boxed()
    }
}

// #[cfg(test)]
// ///linux中文件名最长为255字符,windows中无文件名长度限制
// mod test_video_name_split {
//     #[test]
//     fn test_sp() {
//         let name = "从没经历过这么寒冷的天气，也从没见过这么美的蓝。当我抬起头，流星恰好划过小镇的木屋，世界仿佛在这一刻静止。我去过国内几乎所有省份，但从未见过国内其他地方有如此纯净治愈的星空。如果有机会，希望能带你再去一次。#最美星空 #治愈系风景 #星空 #抖音图文来了,星空 #治愈系风景 #星空 #抖音图文来了,星空 #治愈系风景 #星空 #抖音图文来了,星空 #治愈系风景 #星空 #抖音图文来了,星空 #治愈系风景 #星空 #抖音图文来了";
//         let mut ppd = String::new().as_str();

//         if name.len() > 255 {
//             let nnn1: Vec<_> = name.split("，").collect();
//             ppd = nnn1[0];
//             println!("{}", ppd);
//         };
//         println!("{:?}", &ppd);
//     }
// }
