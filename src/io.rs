use serde::Deserialize;
use std::fs::File as StdFile;
use std::fmt;
use std::io::prelude::*;
use base64::{Engine, engine::general_purpose};
use super::log_test;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct NoteFileItem {
    title: String,
    #[serde(deserialize_with = "from_base64", serialize_with = "to_base64")]
    body: Option<String>,
    children: Option<Vec<NoteFileItem>>,
    is_folder: bool,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct NoteFile {
    children: Option<Vec<NoteFileItem>>,
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
                general_purpose::STANDARD_NO_PAD.decode(&base64_str)
                    .map_err(serde::de::Error::custom)
                    .and_then(|decoded| String::from_utf8(decoded).map_err(serde::de::Error::custom))
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
        let mut file = StdFile::open(path).map_err(|e| format!("Failed to open file {}: {}", path, e))?;
        let mut deserialized_data = String::new();
        file.read_to_string(&mut deserialized_data).map_err(|e| format!("Failed to read file {}: {}", path, e))?;
        log_test!("=====================================\nLoading File contents {}:\n{}",path, deserialized_data);

        // Deserialize JSON data
        let file_data: NoteFile = serde_json::from_str(&deserialized_data)
            .map_err(|e| format!("Failed to deserialize file {}: {}", path, e))?;

        Ok(file_data)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Serialize NoteFile to JSON
        let serialized_data = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize NoteFile: {}", e))?;
        log_test!("=====================================\nSaving File contents {}:\n{}",path, serialized_data);

        // Write JSON data to file
        let mut file = StdFile::create(path)
            .map_err(|e| format!("Failed to create file {}: {}", path, e))?;
        file.write_all(serialized_data.as_bytes())
            .map_err(|e| format!("Failed to write data to file {}: {}", path, e).into())
    }
}

// ############################ UNIT TESTS ############################

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> NoteFile {
        let note_file = NoteFile {
            children: Some(vec![NoteFileItem {
                title: String::from("TITLE: Folder 1 (Root)"),
                body: Some(String::from("BODY: notes 0.1 (Root)")),
                children: Some(vec![
                    NoteFileItem {
                        title: String::from("TITLE: Note 1"),
                        body: Some(String::from("BODY: note 1.1")),
                        children: None,
                        is_folder: false,
                    },
                    NoteFileItem {
                        title: String::from("TITLE: Folder 2"),
                        body: None,
                        children: Some(vec![
                            NoteFileItem {
                                title: String::from("TITLE: Note 2"),
                                body: Some(String::from("BODY: note 2.2")),
                                children: None,
                                is_folder: false,
                            },
                            NoteFileItem {
                                title: String::from("TITLE: Folder 3"),
                                body: None,
                                children: Some(vec![
                                    NoteFileItem {
                                        title: String::from("TITLE: Note 3"),
                                        body: Some(String::from("BODY: note 3.3")),
                                        children: None,
                                        is_folder: false,
                                    },
                                    NoteFileItem {
                                        title: String::from("TITLE: Note 4"),
                                        body: Some(String::from("BODY: note 3.4")),
                                        children: None,
                                        is_folder: false,
                                    },
                                ]),
                                is_folder: true,
                            },
                        ]),
                        is_folder: true,
                    },
                ]),
                is_folder: true,
            }]),
        };

        note_file
    }

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
    fn test_io() {
        let note_file_save = test_data();
        log_test!("=====================================\nSave Test Data:\n{}", note_file_save);

        let mut temp_file = tempfile::NamedTempFile::new().expect("Failed to create temporary file");
        note_file_save.save(temp_file.path().to_str().unwrap()).unwrap_or_else(|e| {
            log_test!("Error: {}", e);
            panic!("Failed to save JSON data: {}", e)
        });

        let note_file_load = NoteFile::load(temp_file.path().to_str().unwrap()).unwrap_or_else(|e| {
            log_test!("Error: {}", e);
            panic!("Failed to load JSON data: {}", e)
        });
        log_test!("=====================================\nLoad Test Data:\n{}", note_file_load);

        assert_note_file_equal(&note_file_save, &note_file_load);
    }
}

