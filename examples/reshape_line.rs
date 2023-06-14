use ar_reshaper::ArabicReshaper;

fn main() {
    let mut reshaper = ArabicReshaper::default();

    // we can modify config after creating `ArabicReshaper`
    reshaper.modify_config(|c| {
        c.delete_harakat = true;
    });

    println!("{}", reshaper.reshape("ههه"));
}
