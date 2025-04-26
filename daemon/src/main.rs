mod event;
mod summarizer;
mod signal;

use event::FileEvent;
use summarizer::summarize_event;
use signal::shutdown_signal;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event};
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use std::env;
use std::process;

#[tokio::main]
async fn main() -> notify::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <directory-to-watch>", args[0]);
        process::exit(1);
    }

    let watch_path = &args[1];
    let watch_pathbuf = PathBuf::from(watch_path);

    let (tx, mut rx) = mpsc::channel::<Event>(100);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }, 
        Config::default()
    )?;

    watcher.watch(&watch_pathbuf, RecursiveMode::Recursive)?;

    println!("Watching directory: {}", watch_path);

    let mut event_batch: Vec<FileEvent> = Vec::new();

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                if let Some(file_event) = summarize_event(event, &watch_pathbuf) {
                    event_batch.push(file_event);
                }
            }
            _ = sleep(Duration::from_secs(2)) => {
                if !event_batch.is_empty() {
                    let serialized = serde_json::to_string(&event_batch).unwrap();
                    println!("{}", serialized);
                    event_batch.clear();
                }
            }
            _ = shutdown_signal() => {
                if !event_batch.is_empty() {
                    println!("Flushing final batch before shutdown...");
                    let serialized = serde_json::to_string(&event_batch).unwrap();
                    println!("{}", serialized);
                }
                break;
            }
        }
    }

    Ok(())
}
