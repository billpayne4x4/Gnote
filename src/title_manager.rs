use gtk::{self, Entry, prelude::*, TreeStore};
use gtk::glib::clone;
use gtk::prelude::EntryExt;
use crate::log_error;

#[derive(Debug, Default, Clone)]
pub struct TitleManager {
    title: Option<Entry>,
}

impl TitleManager {
    pub fn new(title: &Entry) -> Self {
        Self { title: Some(title.clone()) }
    }

    pub fn init(&self, tree_view: &gtk::TreeView) {
        if let Some(title) = self.title.as_ref() {
            let tree_view_clone = tree_view.clone();
            title.connect_changed(clone!(@strong title, @strong tree_view_clone => move |_| {
                if let Some((model, iter)) = tree_view_clone.selection().selected() {
                    let store = match model.downcast::<TreeStore>() {
                        Ok(store) => store,
                        Err(_) => {
                            log_error!("Failed to downcast TreeModel to TreeStore");
                            return;
                        },
                    };
                    store.set_value(&iter, 0, &title.text().to_value());
                }
            }));
        }
   }
}