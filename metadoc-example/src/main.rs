use clap::Parser;
#[cfg(feature = "metric-docs")]
use metadoc_example::metrics::METRIC_META;
use minijinja::{Environment, context};

#[derive(Parser)]
struct Cli {
    #[cfg(feature = "metric-docs")]
    #[arg(long, hide = true)]
    metric_docs: bool,
}

fn main() {
    let args = Cli::parse();

    #[cfg(feature = "metric-docs")]
    if args.metric_docs {
        let mut env = Environment::new();
        minijinja_embed::load_templates!(&mut env);
        let tmpl = env.get_template("metric.html").unwrap();
        for metric_meta in METRIC_META.iter() {
            println!("{:?}", metric_meta);
            println!("{}", tmpl.render(context!(meta => metric_meta)).unwrap());
        }
    }
}
