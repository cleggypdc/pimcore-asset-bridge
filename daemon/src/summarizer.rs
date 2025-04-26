use crate::event::{FileEvent, FileEventType};
use chrono::Utc;
use notify::{Event, EventKind};
use std::path::PathBuf;

pub fn summarize_event(event: Event, watched_root: &PathBuf) -> Option<FileEvent> {
    match event.kind {
        EventKind::Create(_) => {
            event.paths.get(0).map(|path| FileEvent {
                event: FileEventType::Created,
                path: path.to_string_lossy().to_string(),
                from: None,
                to: None,
                timestamp: Utc::now(),
            })
        }
        EventKind::Modify(modify_kind) => {
            match modify_kind {
                notify::event::ModifyKind::Name(_) => {
                    if event.paths.len() == 2 {
                        let from = event.paths.get(0)?;
                        let to = event.paths.get(1)?;
                        let from_inside = from.starts_with(watched_root);
                        let to_inside = to.starts_with(watched_root);

                        if from_inside && to_inside {
                            Some(FileEvent {
                                event: FileEventType::Renamed,
                                path: to.to_string_lossy().to_string(),
                                from: Some(from.to_string_lossy().to_string()),
                                to: Some(to.to_string_lossy().to_string()),
                                timestamp: Utc::now(),
                            })
                        } else if from_inside && !to_inside {
                            Some(FileEvent {
                                event: FileEventType::MovedOut,
                                path: from.to_string_lossy().to_string(),
                                from: Some(from.to_string_lossy().to_string()),
                                to: Some(to.to_string_lossy().to_string()),
                                timestamp: Utc::now(),
                            })
                        } else if !from_inside && to_inside {
                            Some(FileEvent {
                                event: FileEventType::MovedIn,
                                path: to.to_string_lossy().to_string(),
                                from: Some(from.to_string_lossy().to_string()),
                                to: Some(to.to_string_lossy().to_string()),
                                timestamp: Utc::now(),
                            })
                        } else {
                            None
                        }
                    } else {
                        event.paths.get(0).map(|path| FileEvent {
                            event: FileEventType::Renamed,
                            path: path.to_string_lossy().to_string(),
                            from: None,
                            to: None,
                            timestamp: Utc::now(),
                        })
                    }
                }
                _ => {
                    event.paths.get(0).map(|path| FileEvent {
                        event: FileEventType::Modified,
                        path: path.to_string_lossy().to_string(),
                        from: None,
                        to: None,
                        timestamp: Utc::now(),
                    })
                }
            }
        }
        EventKind::Remove(_) => {
            event.paths.get(0).map(|path| FileEvent {
                event: FileEventType::Deleted,
                path: path.to_string_lossy().to_string(),
                from: None,
                to: None,
                timestamp: Utc::now(),
            })
        }
        _ => None,
    }
}
