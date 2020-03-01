// Copyright 2019 Walmart Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate std;

extern crate clap;
#[macro_use]
extern crate log;
extern crate sawtooth_sdk;

use crate::produce_consume::handler::ProduceConsumeHandler;
use clap::App;
use clap::Arg;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use sawtooth_sdk::processor::TransactionProcessor;
use std::process;

pub mod produce_consume;
pub mod proto;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let matches = App::new("produce-consume")
        .author("Walmart Inc.")
        .version("1.0")
        .about("Sample sawtooth-sabre smart contract")
        .arg(
            Arg::with_name("connect")
                .short("C")
                .long("connect")
                .help("Connect endpoint for the Validator")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .multiple(true),
        )
        .get_matches();

    let endpoint = matches
        .value_of("connect")
        .unwrap_or("tcp://localhost:4004");

    let console_log_level;
    match matches.occurrences_of("verbose") {
        0 => console_log_level = LogLevelFilter::Warn,
        1 => console_log_level = LogLevelFilter::Info,
        2 => console_log_level = LogLevelFilter::Debug,
        3 | _ => console_log_level = LogLevelFilter::Trace,
    }

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l:5.5})} | {({M}:{L}):20.20} | {m}{n}",
        )))
        .build();

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(console_log_level))
    {
        Ok(x) => x,
        Err(_) => process::exit(1),
    };

    match log4rs::init_config(config) {
        Ok(_) => (),
        Err(_) => process::exit(1),
    }

    let handler = ProduceConsumeHandler::new();
    let mut processor = TransactionProcessor::new(endpoint);

    info!("Console logging level: {}", console_log_level);

    processor.add_handler(&handler);
    processor.start();
}
