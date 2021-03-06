#[macro_use] extern crate log;
#[macro_use] extern crate clap;
extern crate rdkafka;

use clap::{App, Arg};

use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::config::ClientConfig;

mod example_utils;
use example_utils::setup_logger;

fn print_metadata(brokers: &str, topic: Option<&str>, timeout_ms: i32, fetch_offsets: bool) {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create::<BaseConsumer<_>>()
        .expect("Consumer creation failed");

    trace!("Consumer created");

    let metadata = consumer.fetch_metadata(topic, timeout_ms)
        .expect("Failed to fetch metadata");

    println!("Cluster information:");
    println!("  Broker count: {}", metadata.brokers().len());
    println!("  Topics count: {}", metadata.topics().len());
    println!("  Metadata broker name: {}", metadata.orig_broker_name());
    println!("  Metadata broker id: {}\n", metadata.orig_broker_id());

    println!("Brokers:");
    for broker in metadata.brokers() {
        println!("  Id: {}  Host: {}:{}  ", broker.id(), broker.host(), broker.port());
    }

    println!("\nTopics:");
    for topic in metadata.topics() {
        println!("  Topic: {}  Err: {:?}", topic.name(), topic.error());
        for partition in topic.partitions() {
            println!("     Partition: {}  Leader: {}  Replicas: {:?}  ISR: {:?}  Err: {:?}",
                     partition.id(),
                     partition.leader(),
                     partition.replicas(),
                     partition.isr(),
                     partition.error());
            if fetch_offsets {
                let (low, high) = consumer.fetch_watermarks(topic.name(), partition.id(), 1000)
                    .unwrap_or((-1, -1));
                println!("       Low watermark: {}  High watermark: {}", low, high);
            }
        }
    }
}

fn main() {
    let matches = App::new("metadata fetch example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Fetch and print the cluster metadata")
        .arg(Arg::with_name("brokers")
             .short("b")
             .long("brokers")
             .help("Broker list in kafka format")
             .takes_value(true)
             .default_value("localhost:9092"))
        .arg(Arg::with_name("offsets")
             .long("offsets")
             .help("Enables offset fetching"))
        .arg(Arg::with_name("topic")
            .long("topic")
            .help("Only fetch the metadata of the specified topic")
            .takes_value(true))
        .arg(Arg::with_name("log-conf")
             .long("log-conf")
             .help("Configure the logging format (example: 'rdkafka=trace')")
             .takes_value(true))
        .arg(Arg::with_name("timeout")
             .long("timeout")
             .help("Metadata fetch timeout in seconds")
             .takes_value(true)
             .default_value("60.0"))
        .get_matches();

    setup_logger(true, matches.value_of("log-conf"));

    let brokers = matches.value_of("brokers").unwrap();
    let timeout = value_t!(matches, "timeout", f32).unwrap();
    let topic = matches.value_of("topic");
    let fetch_offsets = matches.is_present("offsets");

    print_metadata(brokers, topic, (timeout * 1000f32) as i32, fetch_offsets);
}
