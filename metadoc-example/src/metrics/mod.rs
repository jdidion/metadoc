#[cfg(feature = "metric-docs")]
mod doc;
mod example;

pub use example::ExampleMetric;

#[cfg(feature = "metric-docs")]
pub use doc::{METRIC_META, Metadata};
