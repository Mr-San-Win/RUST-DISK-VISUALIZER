use trash;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{info, error};

pub fn move_to_trash(paths: &[PathBuf]) -> Result<()> {
    for p in paths {
        match trash::delete(p) {
            Ok(_) => {
                info!("Moved to trash: {:?}", p);
            }
            Err(e) => {
                error!("Failed to move to trash: {:?}, error: {}", p, e);
                return Err(anyhow::anyhow!("Failed to delete {:?}: {}", p, e));
            }
        }
    }
    Ok(())
}
