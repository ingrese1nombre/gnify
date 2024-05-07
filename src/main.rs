use std::sync::Arc;

use gnify_app::api;

smol_macros::main! {
    async fn main(ex: &Arc<smol_macros::Executor<'_>>) -> Result<(), Box<dyn std::error::Error>>  {
        api::run(ex.clone()).await?;
        Ok(())
    }
}
