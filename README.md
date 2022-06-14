# 介绍
douyin爬虫，提供web用户页面的url，由此url抓s取用户视频； 

```bash
dy 0.1.0
lito0210@outlook.com
dy spider

USAGE:
    dy [OPTIONS] --url <URL> --path <PATH>

OPTIONS:
        --file <FILE>              txt,csv file(未实现)
    -h, --help                     Print help information
    -p, --path <PATH>              下载地址(/home)
    -t, --threading <THREADING>    单线程(single)vs多线程(multi)
    -u, --url <URL>                抖音视频主页url
    -V, --version                  Print version information
```

# 使用命令
cargo run -- --url https://test.test --path /home/


该仓库仅供个人学习使用,请勿用于任何商业用途,造成的后果与作者无关；
