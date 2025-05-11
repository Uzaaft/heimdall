//! Watch a single file for changes
//!
//! This module provides functionality to watch a file for modifications
//! and receive notifications when changes occur.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, SystemTime};

use crate::error::AppResult;

/// Event types that can be detected by the file watcher
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchEvent {
    /// File was created
    Created(PathBuf),
    /// File was modified
    Modified(PathBuf),
    /// File was deleted
    Deleted(PathBuf),
    /// Error occurred while watching
    Error(String),
}

/// Simplified metadata for file watching
#[derive(Debug, Clone)]
struct FileMetadata {
    modified: SystemTime,
    size: u64,
}

/// File watcher that monitors a single file for changes
pub struct FileWatcher {
    /// Path being watched
    path: PathBuf,
    /// Thread handle for the watcher
    watcher_thread: Option<thread::JoinHandle<()>>,
    /// Flag to control the watcher thread
    running: bool,
}

impl FileWatcher {
    /// Create a new file watcher and start watching the specified file
    ///
    /// Returns a tuple containing the watcher and a receiver for file change events
    pub fn new<P: AsRef<Path>>(
        path: P,
        poll_interval: Duration,
    ) -> AppResult<(Self, Receiver<WatchEvent>)> {
        let path = path.as_ref().to_path_buf();
        let (sender, receiver) = mpsc::channel();

        // Get initial metadata
        let mut last_metadata = Self::get_file_metadata(&path);

        // Create a flag for controlling the thread
        let running = true;

        // Clone path for the thread
        let thread_path = path.clone();

        // Spawn a thread to watch for changes
        let watcher_thread = thread::spawn(move || {
            let mut is_running = running;

            while is_running {
                let current_metadata = Self::get_file_metadata(&thread_path);

                match (&current_metadata, &last_metadata) {
                    // File exists now but didn't before
                    (Some(_), None) => {
                        if sender
                            .send(WatchEvent::Created(thread_path.clone()))
                            .is_err()
                        {
                            // Receiver was dropped, stop the thread
                            is_running = false;
                            break;
                        }
                    }
                    // File existed before but doesn't now
                    (None, Some(_)) => {
                        if sender
                            .send(WatchEvent::Deleted(thread_path.clone()))
                            .is_err()
                        {
                            is_running = false;
                            break;
                        }
                    }
                    // Both exist, check for modifications
                    (Some(current), Some(previous)) => {
                        if (current.modified != previous.modified || current.size != previous.size)
                            && sender
                                .send(WatchEvent::Modified(thread_path.clone()))
                                .is_err()
                        {
                            is_running = false;
                            break;
                        }
                    }
                    // Both don't exist, do nothing
                    (None, None) => {}
                }

                // Update metadata
                last_metadata = current_metadata;

                // Sleep for the specified interval
                thread::sleep(poll_interval);
            }
        });

        let watcher = FileWatcher {
            path,
            watcher_thread: Some(watcher_thread),
            running,
        };

        Ok((watcher, receiver))
    }

    /// Stop watching the file
    pub fn stop(&mut self) {
        self.running = false;
        if let Some(handle) = self.watcher_thread.take() {
            let _ = handle.join();
        }
    }

    /// Get a reference to the receiver
    /// Get the path being watched
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Helper function to get file metadata
    fn get_file_metadata(path: &Path) -> Option<FileMetadata> {
        match fs::metadata(path) {
            Ok(metadata) => {
                let modified = metadata.modified().ok()?;
                let size = metadata.len();
                Some(FileMetadata { modified, size })
            }
            Err(_) => None,
        }
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Watch a single file for changes
///
/// This is a convenience function that creates a FileWatcher and returns only the receiver.
/// Note that the FileWatcher will be dropped when this function returns, which means the
/// watcher thread will be stopped and no events will be received.
///
/// For long-term watching, use FileWatcher::new() instead.
pub fn watch_file<P: AsRef<Path>>(
    path: P,
    poll_interval: Duration,
) -> AppResult<Receiver<WatchEvent>> {
    let (_, receiver) = FileWatcher::new(path, poll_interval)?;
    Ok(receiver)
}
