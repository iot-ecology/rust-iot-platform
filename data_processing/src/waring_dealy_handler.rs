use common_lib::models::DataRowList;

pub async fn handler_waring_once(
    dt: DataRowList,
    waring_collection: String,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

use serde::{Deserialize, Serialize};
