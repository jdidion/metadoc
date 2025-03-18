# Generate documentation from arbitrary structs

## tl;dr

The [`metadoc-example`](metadoc-example) crate shows a usage example. The key points are:

* Depends on
  * `metadoc`: Enables deriving a `metadata` function for a struct, which catures the struct name, documentation, and field metadata
  * `minijinja` and `minijinja-embed`: Template library
  * `serde`: Enables deriving serializers and deserializers for metadata structs - this is required to be able to use them in the templates
  * `clap`: Enables deriving a CLI parser

The first three dependencies are optional and only included when the `metric-docs` feature is enabled.

The CLI has a (hidden) `--metric-docs` option (which is also conditional on the `metric-docs` feature being enabled) that causes the metric documentation to be generated. The intention is that the `metric-docs` feature and `--metric-docs` option would only be used at build time to generate the documentation, using the following command:

```
cargo run -p metadoc-example -F metric-docs -- --metric-docs
```

## A bit more detail

This example shows how you would define custom metrics and generate documentation for them, similar to what is provided for [`picard`](https://broadinstitute.github.io/picard/picard-metric-definitions.html).

Each metric conditionally derives `metadoc::Described`.

There is a [`doc`](src/metrics/doc.rs) module that is also conditional on the `metric-docs` feature that provides an adapter between the `metadoc::Descriptor` and a much simpler data structure designed to be used in the [template](src/templates/metric.html). See the [minijinja](https://docs.rs/minijinja/latest/minijinja) documentation for details on the template syntax.
