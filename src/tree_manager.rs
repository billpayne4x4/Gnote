use adw::gdk::BUTTON_PRIMARY;
use super::{
    io,
    io::{NoteFile, NoteFileItem},
};
use crate::log_error;
use gtk::{self, gdk::{self, BUTTON_SECONDARY, EventType::ButtonPress}, gio::{Menu, MenuItem, SimpleAction}, Orientation, prelude::*, PositionType, TreeIter, TreeStore, TreeView, EventControllerLegacy, Button, Inhibit, PopoverMenu, TreeModelFilter, GestureClick, Application};
use gtk::gdk::Rectangle;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct TreeManager {
    tree_view: Option<TreeView>,
    tree_store: Option<TreeStore>,
    add_note: Option<Button>,
    add_folder: Option<Button>,
    remove_item: Option<Button>,
    selected_iter: Option<TreeIter>,
}

impl Default for TreeManager {
    fn default() -> Self {
        TreeManager {
            tree_view: None,
            tree_store: None,
            add_note: None,
            add_folder: None,
            remove_item: None,
            selected_iter: None,
        }
    }
}

impl Default for &TreeManager {
    fn default() -> Self {
        &TreeManager {
            tree_view: None,
            tree_store: None,
            add_note: None,
            add_folder: None,
            remove_item: None,
            selected_iter: None,
        }
    }
}

impl Clone for TreeManager {
    fn clone(&self) -> Self {
        Self {
            tree_view: self.tree_view.clone(),
            tree_store: self.tree_store.clone(),
            add_note: self.add_note.clone(),
            add_folder: self.add_folder.clone(),
            remove_item: self.remove_item.clone(),
            selected_iter: self.selected_iter.clone(),
        }
    }
}


impl TreeManager {
    pub fn new(tree_view: &TreeView, tree_store: &TreeStore, add_note: &Button, add_folder: &Button, remove_item: &Button) -> Self {
        Self {
            tree_view: Some(tree_view.clone()),
            tree_store: Some(tree_store.clone()),
            add_note: Some(add_note.clone()),
            add_folder: Some(add_folder.clone()),
            remove_item: Some(remove_item.clone()),
            selected_iter: None,
        }
    }

    pub fn init(&self, application: &Application) {
         let self_clone = self.clone();

        self.tree_view.as_ref().unwrap().connect_row_activated(move |tree_view, path, column| {
            if let Some((model, iter)) = tree_view.selection().selected() {
                let store = match model.downcast::<TreeStore>() {
                    Ok(store) => store,
                    Err(_) => {
                        log_error!("Failed to downcast TreeModel to TreeStore");
                        return;
                    },
                };

                let is_title_empty: bool = self_clone.tree_store.as_ref().unwrap().get_value(&iter, 0).get::<String>().unwrap().is_empty();
                let is_folder: bool = self_clone.tree_store.as_ref().unwrap().get_value(&iter, 2).get().unwrap();
                let parent: Option<TreeIter> = self_clone.tree_store.as_ref().unwrap().iter_parent(&iter);

                if parent.is_some() {
                    self_clone.remove_item.as_ref().unwrap().set_visible(true);
                } else {
                    self_clone.remove_item.as_ref().unwrap().set_visible(false);
                }

                if is_folder {
                    self_clone.add_note.as_ref().unwrap().set_visible(true);
                    self_clone.add_folder.as_ref().unwrap().set_visible(true);
                } else {
                    self_clone.add_note.as_ref().unwrap().set_visible(false);
                    self_clone.add_folder.as_ref().unwrap().set_visible(false);
                }
            }
        });

        let filter_model = TreeModelFilter::new(
            <gtk::TreeStore as AsRef<gtk::TreeModel>>::as_ref(&self.tree_store.as_ref().unwrap()),
            None,
        );
        filter_model.set_visible_func(move |model, iter| {
            let title = model
                .get_value(iter, 0)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());

            let is_folder = model.get_value(iter, 2).get::<bool>().unwrap_or(false);

            title != "" && is_folder
        });
    }

    pub fn add_folder(
        &self,
        name: &str,
    ) -> TreeIter {
        if let Some(p) = self.selected_iter {
            let child_count = self.tree_store.as_ref().unwrap().iter_n_children(Some(&p));

            if child_count == 1 {
                let child_iter = self.tree_store.as_ref().unwrap().iter_nth_child(Some(&p), 0).unwrap();
                if self.tree_store.as_ref().unwrap().get_value(&child_iter, 0).get::<String>().unwrap().as_str() == "" {
                    self.tree_store.as_ref().unwrap().remove(&child_iter);
                }
            }
        }

        let iter = self.tree_store.as_ref().unwrap().insert_with_values(
            match self.selected_iter {
                Some(ref p) => Some(&p),
                None => None,
            }, None, &[(0, &name), (1, &""), (2, &true)]);
        self.tree_store.as_ref().unwrap().insert_with_values(Some(&iter), None, &[(0, &""), (1, &""), (2, &true)]);

        iter
    }

    pub fn remove_folder(self) {
        let parent_iter = self.tree_store.as_ref().unwrap().iter_parent(&self.selected_iter.as_ref().unwrap());

        // Remove the folder
        self.tree_store.as_ref().unwrap().remove(&self.selected_iter.as_ref().unwrap());

        if let Some(parent) = parent_iter {
            let child_count = self.tree_store.as_ref().unwrap().iter_n_children(Some(&parent));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                self.tree_store.as_ref().unwrap().insert_with_values(Some(&parent), None, &[(0, &""), (1, &""), (2, &true)]);
            }
        }
    }

    pub fn add_note(
       &self,
        name: &str
    ) -> TreeIter {
        if let Some(p) = self.selected_iter.as_ref() {
            let child_count = self.tree_store.as_ref().unwrap().iter_n_children(Some(&p));

            if child_count == 1 {
                let child_iter = self.tree_store.as_ref().unwrap().iter_nth_child(Some(&p), 0).unwrap();
                if self.tree_store.as_ref().unwrap().get_value(&child_iter, 0).get::<String>().unwrap().as_str() == "" {
                    self.tree_store.as_ref().unwrap().remove(&child_iter);
                }
            }
        }

        let iter = self.tree_store.as_ref().unwrap().insert_with_values(Some(&self.selected_iter.as_ref().unwrap()), None, &[(0, &name), (1, &""), (2, &false)]);

        iter
    }

    pub fn remove_note(&self) {
        let parent_iter = self.tree_store.as_ref().unwrap().iter_parent(&self.selected_iter.as_ref().unwrap());

        // Remove the note
        self.tree_store.as_ref().unwrap().remove(&self.selected_iter.as_ref().unwrap());

        if let Some(p) = parent_iter {
            let child_count = self.tree_store.as_ref().unwrap().iter_n_children(Some(&p));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                self.tree_store.as_ref().unwrap().insert_with_values(Some(&p), None, &[(0, &""), (1, &""), (2, &true)]);
            }
        }
    }

    pub fn save(store: &TreeStore) {
        println!("Saving...");
        fn build_note_file_item(store: &TreeStore, iter: &TreeIter) -> NoteFileItem {
            let title = store
                .get_value(iter, 0)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());
            let body = store
                .get_value(iter, 1)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());
            let is_folder = store.get_value(iter, 2).get::<bool>().unwrap_or(false);

            let mut children = None;
            if is_folder {
                if let Some(mut child_iter) = store.iter_children(Some(iter)) {
                    let mut child_items = Vec::new();

                    loop {
                        child_items.push(build_note_file_item(store, &child_iter));
                        if !store.iter_next(&mut child_iter) {
                            break;
                        }
                    }

                    children = Some(child_items);
                }
            }

            NoteFileItem {
                title,
                body: Some(body),
                children,
                is_folder,
            }
        }

        let mut root_iter = store.iter_nth_child(None, 0).unwrap();
        let mut root_items = Vec::new();

        loop {
            root_items.push(build_note_file_item(store, &root_iter));
            if !store.iter_next(&mut root_iter) {
                break;
            }
        }

        let note_file = NoteFile {
            children: Some(root_items),
        };

        note_file.save(&io::get_notes_path()).unwrap_or_else(|e| {
            log_error!("Error: {}", e);
        });
    }

    pub fn load(store: &TreeStore) {
        store.clear();

        let note_file = match NoteFile::load(&io::get_notes_path()) {
            Ok(note_file_load) => note_file_load,
            Err(e) => {
                log_error!("Failed to load file - {}", e);
                return;
            }
        };

        fn insert_note_file_item(
            store: &TreeStore,
            item: &NoteFileItem,
            parent: Option<&TreeIter>,
        ) {
            let iter = store.insert_with_values(
                parent,
                None,
                &[
                    (0, &item.title),
                    (1, &item.body.as_ref().unwrap_or(&"".to_string())),
                    (2, &item.is_folder),
                ],
            );

            if let Some(children) = &item.children {
                for child in children {
                    insert_note_file_item(store, child, Some(&iter));
                }
            }
        }

        if let Some(root_items) = &note_file.children {
            for item in root_items {
                insert_note_file_item(&store, item, None);
            }
        }
    }

    pub fn set_selected_iter(&mut self, iter: Option<TreeIter>) {
        match iter {
            Some(i) => {
                self.selected_iter = Some(i.clone());
            }
            None => {}
        }
    }
}

// ############################ UNIT TESTS ############################

mod tests {
    use super::*;
    use crate::test_data;
    use gtk::glib::StaticType;

    fn create_tree_store(note_file: &NoteFile) -> TreeStore {
        let store = TreeStore::new(&[
            String::static_type(),
            String::static_type(),
            bool::static_type(),
        ]);

        fn insert_note_file_item(
            store: &TreeStore,
            item: &NoteFileItem,
            parent: Option<&TreeIter>,
        ) {
            let iter = store.insert_with_values(
                parent,
                None,
                &[
                    (0, &item.title),
                    (1, &item.body.as_ref().unwrap_or(&"".to_string())),
                    (2, &item.is_folder),
                ],
            );

            if let Some(children) = &item.children {
                for child in children {
                    insert_note_file_item(store, child, Some(&iter));
                }
            }
        }

        if let Some(root_items) = &note_file.children {
            for item in root_items {
                insert_note_file_item(&store, item, None);
            }
        }

        store
    }

    fn iter_equal(
        a: &TreeStore,
        a_iter: &TreeIter,
        b: &TreeStore,
        b_iter: &TreeIter,
    ) -> bool {
        let a_title: String = a.get_value(&a_iter, 0).get().unwrap();
        let a_body: String = a.get_value(&a_iter, 1).get().unwrap();
        let a_is_folder: bool = a.get_value(&a_iter, 2).get().unwrap();

        let b_title: String = b.get_value(&b_iter, 0).get().unwrap();
        let b_body: String = b.get_value(&b_iter, 1).get().unwrap();
        let b_is_folder: bool = b.get_value(&b_iter, 2).get().unwrap();

        if a_title != b_title || a_body != b_body || a_is_folder != b_is_folder {
            return false;
        }

        let mut a_child_iter = a.iter_nth_child(Some(&a_iter), 0);
        let mut b_child_iter = b.iter_nth_child(Some(&b_iter), 0);

        loop {
            match (a_child_iter.as_ref(), b_child_iter.as_ref()) {
                (Some(a_child), Some(b_child)) => {
                    if !iter_equal(a, a_child, b, b_child) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => return false,
                (None, None) => break,
            }

            if !a.iter_next(a_child_iter.as_mut().unwrap()) {
                a_child_iter = None;
            }
            if !b.iter_next(b_child_iter.as_mut().unwrap()) {
                b_child_iter = None;
            }
        }

        true
    }

    fn tree_store_equal(a: &TreeStore, b: &TreeStore) -> bool {
        let mut a_iter = a.iter_first().unwrap();
        let mut b_iter = b.iter_first().unwrap();

        loop {
            if !iter_equal(a, &a_iter, b, &b_iter) {
                return false;
            }
            if !a.iter_next(&mut a_iter) || !b.iter_next(&mut b_iter) {
                break;
            }
        }

        true
    }

    #[test]
    fn test_save_and_load_tree_store() {
        // Initialize GTK
        gtk::init().unwrap();

        let test_data = test_data::get();
        let test_data_tree_store = tests::create_tree_store(&test_data);
        let loaded_data_tree_store = tests::create_tree_store(&NoteFile { children: None });

        TreeManager::save(&test_data_tree_store);
        TreeManager::load(&loaded_data_tree_store);

        assert!(tree_store_equal(
            &test_data_tree_store,
            &loaded_data_tree_store
        ));
    }
}
