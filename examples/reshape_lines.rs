use ar_reshaper::ArabicReshaper;

fn main() {
    let reshaper = ArabicReshaper::default();

    let lines = ["به نام خدا", "همه چی درست میشه!"];

    for line in reshaper.reshape_lines(lines) {
        println!("{line}");
    }
}
