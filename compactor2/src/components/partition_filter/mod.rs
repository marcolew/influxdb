use std::fmt::{Debug, Display};

use async_trait::async_trait;
use data_types::{ParquetFile, PartitionId};

pub mod and;
pub mod has_files;
pub mod logging;
pub mod metrics;
pub mod never_skipped;

#[async_trait]
pub trait PartitionFilter: Debug + Display + Send + Sync {
    async fn apply(&self, partition_id: PartitionId, files: &[ParquetFile]) -> bool;
}
