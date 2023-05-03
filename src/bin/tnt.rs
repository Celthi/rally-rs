use std::thread;
use tnt::config_env;
#[cfg(not(target_os = "windows"))]
use tnt::msg::consumer;
use tnt::web;
use tracing::level_filters;
#[cfg(not(target_os = "windows"))]
fn consume() {
    consumer::event_loop().expect("consumer event loop failing."); //
}
#[cfg(target_os = "windows")]
fn consume() {}
fn main() {
    config_env::ensure_config();
    let filter = level_filters::LevelFilter::INFO;

    tracing_subscriber::fmt().with_max_level(filter).init();

    let mut v = vec![];
    let j = thread::spawn(|| {
        web::event_loop().expect("web event loop failing."); // fine to crash as cannot start the web server
    });
    v.push(j);
    let j = thread::spawn(|| {
        consume();
    });
    v.push(j);
    for t in v {
        t.join().expect("cannot join the thread.");
    }
}
