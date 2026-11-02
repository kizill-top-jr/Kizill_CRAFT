use kizill_net::{GrabOrder, Grabber, Pantry};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    let pantry = Pantry::open("./data/pantry");
    let grabber = Grabber::new(pantry, 4);

    let order = GrabOrder {
        url: "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json".into(),
        sha1: "96a5005655dfb44d613f391535434b1f7778fb3d".into(),
        dest: PathBuf::from("./data/test/manifest.json"),
    };

    let result = grabber.grab_one(order).await;
    println!("result: {:?}", result);
    Ok(())
}
