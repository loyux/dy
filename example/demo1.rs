use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// 抖音视频主页url
    #[clap(short, long)]
    pub url: Option<String>,

    /// 下载文件的路径(/home)
    #[clap(short, long)]
    pub path: String,

    ///单线程(single)vs多线程(multi)
    #[clap(short, long)]
    pub threading: Option<String>,

    ///txt,csv file
    #[clap(long)]
    pub file: Option<String>,
}

fn main() {
    println!("hello");
    let args = Args::parse();
    
}


//https://www.douyin.com/user/MS4wLjABAAAArIEQhRh680GFCYcJVAku9nv94mBrFwQcthmPQ5Gy0bCYF84FBY_e2ROLP8yqw8Cw