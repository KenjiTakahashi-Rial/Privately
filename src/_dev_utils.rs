mod dev_db;

use tokio::sync::OnceCell;
use tracing::info;

pub async fn init() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        info!("{:<12} - dev_db::init", "DEV");
        dev_db::init().await.unwrap();
    })
    .await;
}
