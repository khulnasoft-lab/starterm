//! Implements a file system watcher using the `notify` crate.

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// An enum representing the events we care about from the watcher.
#[derive(Debug)]
pub enum FsEvent {
    FileChanged(PathBuf),
    FileRemoved(PathBuf),
    // Add more specific events as needed
}

/// A wrapper around `notify::RecommendedWatcher` to provide a simpler API.
pub struct FsWatcher {
    /// Receives processed file system events.
    pub rx: Receiver<FsEvent>,
    // The watcher needs to be kept in scope to receive events.
    _watcher: RecommendedWatcher,
}

impl FsWatcher {
    /// Creates a new watcher that monitors the given paths.
    ///
    /// Events are sent over the returned `Receiver`.
    pub fn new(paths: Vec<PathBuf>) -> Result<Self, notify::Error> {
        let (tx, rx) = channel();
        let (notify_tx, notify_rx) = channel();

        let mut watcher = RecommendedWatcher::new(notify_tx, Config::default())?;

        for path in paths {
            watcher.watch(&path, RecursiveMode::NonRecursive)?;
        }

        // Spawn a thread to process raw notify events and forward them.
        // This decouples the watcher from our main event loop and allows for debouncing.
        thread::spawn(move || {
            Self::event_processor(notify_rx, tx);
        });

        Ok(Self {
            rx,
            _watcher: watcher,
        })
    }

    /// The internal event processing loop.
    fn event_processor(rx: Receiver<Result<Event, notify::Error>>, tx: Sender<FsEvent>) {
        // TODO: Implement debouncing logic to handle rapid-fire events,
        // especially on platforms like macOS.
        for res in rx {
            if let Ok(event) = res {
                match event.kind {
                    notify::EventKind::Modify(_) => {
                        if let Some(path) = event.paths.first() {
                            tx.send(FsEvent::FileChanged(path.clone())).ok();
                        }
                    }
                    notify::EventKind::Remove(_) => {
                        if let Some(path) = event.paths.first() {
                            tx.send(FsEvent::FileRemoved(path.clone())).ok();
                        }
                    }
                    _ => { /* Ignore other events for now */ }
                }
            }
        }
    }
}