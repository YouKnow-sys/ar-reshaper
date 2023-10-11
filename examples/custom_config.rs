use ar_reshaper::prelude::*;

fn main() {
    let reshaper = ReshaperConfig::new(Language::ArabicV2, LigaturesFlags::none()).to_reshaper();

    println!("{}", reshaper.reshape("سهل"));
}
