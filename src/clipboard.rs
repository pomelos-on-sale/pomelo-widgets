//! System-wide unified clipboard for Pomelo OS.
//!
//! Provides thread-safe, cross-application text read and write operations that
//! automatically adapt to the environment:
//! - On Desktop/Simulator (Linux, macOS, Windows): reads from and writes to the
//!   host operating system's native clipboard, while keeping in-memory state in sync.
//! - On Embedded (ESP32-S3): operates on Pomelo OS's in-memory clipboard.

use std::sync::RwLock;

static CLIPBOARD: RwLock<String> = RwLock::new(String::new());

#[cfg(not(target_os = "espidf"))]
mod platform {
    pub fn host_read() -> Option<String> {
        let mut cb = arboard::Clipboard::new().ok()?;
        cb.get_text().ok()
    }

    pub fn host_write(content: &str) {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(content);
        }
    }

    pub fn host_clear() {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text("");
            let _ = cb.clear();
        }
    }
}

/// Read the current contents of the system clipboard.
pub fn read() -> String {
    #[cfg(not(target_os = "espidf"))]
    {
        if let Some(text) = platform::host_read() {
            if !text.is_empty() {
                return text;
            }
        }
    }

    CLIPBOARD
        .read()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

/// Write text into the system clipboard.
pub fn write(content: impl AsRef<str>) {
    let s = content.as_ref();
    if let Ok(mut guard) = CLIPBOARD.write() {
        guard.clear();
        guard.push_str(s);
    }

    #[cfg(not(target_os = "espidf"))]
    {
        platform::host_write(s);
    }
}

/// Clear the system clipboard.
pub fn clear() {
    if let Ok(mut guard) = CLIPBOARD.write() {
        guard.clear();
    }

    #[cfg(not(target_os = "espidf"))]
    {
        platform::host_clear();
    }
}

/// Returns true if the system clipboard is empty.
pub fn is_empty() -> bool {
    read().is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clipboard_read_write_lifecycle() {
        clear();
        assert!(is_empty());
        assert_eq!(read(), "");

        write("https://example.com/calendar.ics");
        assert!(!is_empty());
        assert_eq!(read(), "https://example.com/calendar.ics");

        clear();
        assert!(is_empty());
        assert_eq!(read(), "");
    }
}
