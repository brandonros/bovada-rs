mod bovada;
mod event_type;
mod structs;
mod utilities;
mod ws_error;

use std::error::Error;
use std::sync::Arc;

use crate::bovada::Bovada;
use crate::ws_error::WsError;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // get slug from arguments
    let args = std::env::args().collect::<Vec<String>>();
    let slug = args.get(1).ok_or(WsError::InvalidArgumentsError)?;
    // create api instance
    let api = Arc::new(Bovada::new(slug.clone()));
    // build async runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;
    // spawn async into runtime and wait
    runtime.block_on(async {
        // get event ids
        let event_ids = api.get_event_ids().await?;
        // spawn 1 subscriber future per event ID
        let task_handles = event_ids
            .into_iter()
            .map(|event_id| {
                let api_clone = api.clone();
                tokio::task::spawn(async move {
                    api_clone.create_event_subscription(event_id.clone()).await
                })
            })
            .collect::<Vec<_>>();
        // await all futures in parallel
        let results = futures::future::join_all(task_handles).await;
        // error check all results
        for result in results {
            let join_result = result?;
            let _task_result = join_result?;
        }
        // return
        Ok(())
    })
}
