use metadoc::Described;

/// An example metric.
/// This metric counts the number of things.
#[derive(Default)]
#[cfg_attr(feature = "metric-docs", derive(Described))]
pub struct ExampleMetric {
    /// The number of things.
    count: usize,
}

impl ExampleMetric {
    pub fn inc(&mut self) {
        self.count += 1;
    }
}
