use crate::log_test;
use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use std::{fmt, fs, fs::File as StdFile, io::prelude::*, path::PathBuf};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NoteFileItem {
    pub(crate) title: String,
    #[serde(deserialize_with = "from_base64", serialize_with = "to_base64")]
    pub body: Option<String>,
    pub children: Option<Vec<NoteFileItem>>,
    pub is_folder: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NoteFile {
    pub children: Option<Vec<NoteFileItem>>,
}

fn from_base64<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(base64_str) => {
            if base64_str.is_empty() {
                Ok(None)
            } else {
                general_purpose::STANDARD_NO_PAD
                    .decode(&base64_str)
                    .map_err(serde::de::Error::custom)
                    .and_then(|decoded| {
                        String::from_utf8(decoded).map_err(serde::de::Error::custom)
                    })
                    .map(Some)
            }
        }
        None => Ok(None),
    }
}

fn to_base64<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(data) => {
            let base64_data = general_purpose::STANDARD_NO_PAD.encode(data);
            serializer.serialize_some(&base64_data)
        }
        None => serializer.serialize_none(),
    }
}

impl NoteFileItem {
    fn fmt_recursive(&self, f: &mut fmt::Formatter<'_>, depth: usize) -> fmt::Result {
        let indent = "  ".repeat(depth);
        let is_folder_str = if self.is_folder { " (folder)" } else { "" };
        write!(f, "{}- title: {}{}\n", indent, self.title, is_folder_str)?;
        if let Some(body) = &self.body {
            write!(f, "{}  body: {}\n", indent, body)?;
        }
        write!(f, "{}  is_folder: {}\n", indent, self.is_folder)?;
        if let Some(children) = &self.children {
            write!(f, "{}  children:\n", indent)?;
            for child in children {
                child.fmt_recursive(f, depth + 1)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for NoteFileItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, 0)
    }
}

impl fmt::Display for NoteFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(children) = &self.children {
            for child in children {
                write!(f, "{}", child)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for NoteFile {
    fn eq(&self, other: &Self) -> bool {
        match (&self.children, &other.children) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (i, child_a) in a.iter().enumerate() {
                    if child_a.title != b[i].title {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl NoteFile {
    pub fn load(path: &str) -> Result<NoteFile, Box<dyn std::error::Error>> {
        // Read JSON file
        let mut file =
            StdFile::open(path).map_err(|e| format!("Failed to open file {}: {}", path, e))?;
        let mut deserialized_data = String::new();
        file.read_to_string(&mut deserialized_data)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;
        log_test!(
            "=====================================\nLoading File contents {}:\n{}",
            path,
            deserialized_data
        );

        // Deserialize JSON data
        let file_data: NoteFile = serde_json::from_str(&deserialized_data)
            .map_err(|e| format!("Failed to deserialize file {}: {}", path, e))?;

        Ok(file_data)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize NoteFile to JSON
        let serialized_data = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize NoteFile: {}", e))?;
        log_test!(
            "=====================================\nSaving File contents {}:\n{}",
            path,
            serialized_data
        );

        // Write JSON data to file
        let mut file =
            StdFile::create(path).map_err(|e| format!("Failed to create file {}: {}", path, e))?;
        file.write_all(serialized_data.as_bytes())
            .map_err(|e| format!("Failed to write data to file {}: {}", path, e).into())
    }
}

fn ensure_gnote_directory() -> PathBuf {
    let mut gnote_path = dirs::home_dir().expect("Couldn't get user's home directory");
    gnote_path.push(".gnote");

    if !gnote_path.exists() {
        fs::create_dir(&gnote_path).expect("Couldn't create .gnote directory");
    }

    gnote_path
}

pub fn get_settings_path() -> String {
    let mut settings_path = ensure_gnote_directory();

    #[cfg(test)]
    {
        settings_path.push("settings_test.json");
    }
    #[cfg(not(test))]
    {
        settings_path.push("settings.json");
    }

    settings_path.to_str().unwrap().to_owned()
}

pub fn get_notes_path() -> String {
    let mut notes_path = ensure_gnote_directory();

    #[cfg(test)]
    {
        notes_path.push("notes_test.json");
    }
    #[cfg(not(test))]
    {
        notes_path.push("notes.json");
    }

    notes_path.to_str().unwrap().to_owned()
}

// ############################ UNIT TESTS ############################

/*#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::test_data;

    fn assert_note_file_equal(n1: &NoteFile, n2: &NoteFile) {
        assert_eq!(n1.children.is_some(), n2.children.is_some());
        if let (Some(n1_children), Some(n2_children)) = (&n1.children, &n2.children) {
            assert_eq!(n1_children.len(), n2_children.len());
            for (n1_child, n2_child) in n1_children.iter().zip(n2_children.iter()) {
                assert_note_file_item_equal(n1_child, n2_child);
            }
        }
    }

    fn assert_note_file_item_equal(n1: &NoteFileItem, n2: &NoteFileItem) {
        assert_eq!(n1.title, n2.title);
        assert_eq!(n1.body, n2.body);
        assert_eq!(n1.is_folder, n2.is_folder);
        assert_eq!(n1.children.is_some(), n2.children.is_some());
        if let (Some(n1_children), Some(n2_children)) = (&n1.children, &n2.children) {
            assert_eq!(n1_children.len(), n2_children.len());
            for (n1_child, n2_child) in n1_children.iter().zip(n2_children.iter()) {
                assert_note_file_item_equal(n1_child, n2_child);
            }
        }
    }

    #[test]
    fn test_save_and_load_io() {
        let note_file_save = test_data::get();
        log_test!(
            "=====================================\nSave Test Data:\n{}",
            note_file_save
        );

        let mut temp_file =
            tempfile::NamedTempFile::new().expect("Failed to create temporary file");
        note_file_save
            .save(temp_file.path().to_str().unwrap())
            .unwrap_or_else(|e| {
                log_test!("Error: {}", e);
                panic!("Failed to save JSON data: {}", e)
            });

        let note_file_load = match NoteFile::load(temp_file.path().to_str().unwrap()) {
            Ok(note_file_load) => note_file_load,
            Err(e) => {
                log_test!("Error: {}", e);
                return;
            }
        };
        log_test!(
            "=====================================\nLoad Test Data:\n{}",
            note_file_load
        );

        assert_note_file_equal(&note_file_save, &note_file_load);
    }
}
*/
