use gtk::{self, glib::{Cast, clone, ToValue}, traits::TreeViewExt, TextBuffer, TextView, TreeView, traits::TextBufferExt};
use crate::log_error;

#[derive(Debug, Default, Clone)]
pub struct NoteManager {
    note: Option<TextView>,
    note_buffer: Option<TextBuffer>,
}

impl NoteManager {
    pub fn new(note: &TextView, note_buffer: &TextBuffer) -> Self {
        Self { note: Some(note.clone()), note_buffer: Some(note_buffer.clone()) }
    }

    pub fn init(&self, tree_view: &TreeView) {
        if let Some(note_buffer) = self.note_buffer.as_ref() {
            let tree_view_clone = tree_view.clone();
            note_buffer.connect_changed(clone!(@strong note_buffer => move |_| {
                if let Some((model, iter)) = tree_view_clone.selection().selected() {
                    let store = match model.downcast::<gtk::TreeStore>() {
                        Ok(store) => store,
                        Err(_) => {
                            log_error!("Failed to downcast TreeModel to TreeStore");
                            return;
                        },
                    };
                    let note_text = note_buffer.text(&note_buffer.start_iter(), &note_buffer.end_iter(), false);
                    let note = note_text.as_str();
                    store.set_value(&iter, 1, &note.to_value());
                }
            }));
        }
    }
}