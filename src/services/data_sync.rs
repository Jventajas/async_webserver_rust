use tokio::time;
use tracing::info;

pub struct DataSyncService {

}

impl DataSyncService {

    pub fn new() -> Self {
        Self {

        }
    }


    pub async fn sync_data(&self, interval_seconds: u64) {

        tokio::spawn(async move {
           let mut interval = time::interval(time::Duration::from_secs(interval_seconds));

            loop {
                interval.tick().await;
                info!("Starting data sync");
            }

        });
    }

}