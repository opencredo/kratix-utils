# Kratix-Utils

Kratix-Utils is a ready-to-run collection of functions that help to fullfil a promise.

The goal of kratix_utils is to help you establish best practices and consistent formatting across multiple promises, and ensure developers are adhering to those conventions.

### How to use kratix_utils
```bash
cd internal/configure-pipeline
cargo new sample
```
### In the main.rs include a call to run_pipeline
```bash
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use std::{env, process};

fn main() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    let args: Vec<String> = env::args().collect();

    // TODO: pass function to do IOC transform
    match args[1].as_str() {
        "pipeline" => kratix_utils::run_pipeline(args),
        _ => {
            log::warn!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
```


Kratix Utils is licensed under a MIT License.
