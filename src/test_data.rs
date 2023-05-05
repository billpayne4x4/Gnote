use crate::io::{NoteFile, NoteFileItem};

pub fn get() -> NoteFile {
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
