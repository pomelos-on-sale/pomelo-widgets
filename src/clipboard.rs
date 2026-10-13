//! System-wide in-memory clipboard for Pomelo OS.
//!
//! Provides thread-safe, cross-application text read and write operations that
//! work identically on both the desktop simulator and ESP32-S3 hardware.

use std::sync::RwLock;

static CLIPBOARD: RwLock<String> = RwLock::new(String::new());

/// Read the current contents of the system clipboard.
pub fn read() -> String {
    CLIPBOARD
        .read()
        .map(|guard| guard.clone())
        .unwrap_or_default()
}

/// Write text into the system clipboard.
pub fn write(content: impl AsRef<str>) {
    if let Ok(mut guard) = CLIPBOARD.write() {
        guard.clear();
        guard.push_str(content.as_ref());
    }
}

/// Clear the system clipboard.
pub fn clear() {
    if let Ok(mut guard) = CLIPBOARD.write() {
        guard.clear();
    }
}

/// Returns true if the system clipboard is empty.
pub fn is_empty() -> bool {
    CLIPBOARD
        .read()
        .map(|guard| guard.is_empty())
        .unwrap_or(true)
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
