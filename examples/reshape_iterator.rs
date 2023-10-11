use ar_reshaper::prelude::*;

fn main() {
    let lines = [
        "به نام خدا",
        "همه چی درست میشه!",
        "راست خیلی قشنگه ها",
        "ازش خوشم میاد",
    ];

    for line in lines.iter().reshape_default() {
        println!("{line}");
    }
}
