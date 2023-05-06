use super::{
    io,
    io::{NoteFile, NoteFileItem},
};
use crate::log_error;
use gtk::{
    self,
    gdk::{self, BUTTON_SECONDARY, EventType::ButtonPress},
    gio::{Menu, MenuItem, SimpleAction},
    Orientation,
    prelude::*,
    PositionType,
    TreeIter, TreeStore, TreeView, EventControllerLegacy,
    Button, Inhibit, PopoverMenu, TreeModelFilter,
    GestureClick,
};
use gtk::gdk::Rectangle;

pub struct TreeManager;

impl TreeManager {
    pub fn init(tree: &TreeView, store: &TreeStore) {
        let gesture = GestureClick::new();
        gesture.set_button(BUTTON_SECONDARY);
        let tree_clone = tree.clone();

        tree_clone.parent();

        //let root = tree_clone. .get_root().unwrap().downcast::<ApplicationWindow>().unwrap();
        SimpleAction::new("tree.add_note", None)
            .connect_activate(move |_, _| {
                println!("Add Note");
            });

        gesture.connect_released(move |gesture, n_press, x, y| {
            if let Some((model, iter)) = tree_clone.selection().selected() {
                let store = match model.downcast::<TreeStore>() {
                    Ok(store) => store,
                    Err(_) => {
                        log_error!("Failed to downcast TreeModel to TreeStore");
                        return;
                    },
                };

                let is_title_empty: bool = store.get_value(&iter, 0).get::<String>().unwrap().is_empty();
                let is_folder: bool = store.get_value(&iter, 2).get().unwrap();
                let parent: Option<TreeIter> = store.iter_parent(&iter);

                if parent.is_some() {
                    let menu = Menu::new();
                    let delete_section = Menu::new();
                    let popup_menu = PopoverMenu::from_model(Some(&menu));
                    if is_folder || is_title_empty {
                        let add_section = Menu::new();
                        add_section.append(Some("Add Note"), Some("tree.add_note"));
                        add_section.append(Some("Add Folder"), Some("tree.add_folder"));
                        menu.append_section(None, &add_section);
                        delete_section.append(Some("Delete Folder"), Some("tree.delete_folder"));
                        popup_menu.set_size_request(0, 125);
                    } else {
                        delete_section.append(Some("Delete Note"), Some("tree.delete_note"));
                    }
                    menu.append_section(None, &delete_section);

                    popup_menu.set_parent(&tree_clone);
                    popup_menu.set_has_arrow(false);
                    popup_menu.set_position(PositionType::Right);
                    popup_menu.set_pointing_to(Some(&Rectangle::new(x as i32, y as i32, 0, 0)));
                    popup_menu.popup();
                }

                println!("title: {:?}", store.get_value(&iter, 0).get::<String>().unwrap());
            }

            println!("gesture: {:?}", gesture);
            println!("n_press: {:?}", n_press);
            println!("x: {:?}", x);
            println!("y: {:?}", y);
        });
        tree.add_controller(&gesture);

        let filter_model = TreeModelFilter::new(store, None);
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
        store: &TreeStore,
        name: &str,
        parent: Option<&TreeIter>,
    ) -> TreeIter {
        if let Some(p) = parent {
            let child_count = store.iter_n_children(Some(p));

            if child_count == 1 {
                let child_iter = store.iter_nth_child(Some(p), 0).unwrap();
                if store.get_value(&child_iter, 0).get::<String>().unwrap().as_str() == "" {
                    store.remove(&child_iter);
                }
            }
        }

        let iter = store.insert_with_values(parent, None, &[(0, &name), (1, &""), (2, &true)]);
        store.insert_with_values(Some(&iter), None, &[(0, &""), (1, &""), (2, &true)]);

        iter
    }

    pub fn remove_folder(store: &TreeStore, iter: &TreeIter) {
        let parent_iter = store.iter_parent(iter);

        // Remove the folder
        store.remove(iter);

        if let Some(parent) = parent_iter {
            let child_count = store.iter_n_children(Some(&parent));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                store.insert_with_values(Some(&parent), None, &[(0, &""), (1, &""), (2, &true)]);
            }
        }
    }

    pub fn add_note(
        store: &TreeStore,
        name: &str,
        parent: Option<&TreeIter>,
    ) -> TreeIter {
        if let Some(p) = parent {
            let child_count = store.iter_n_children(Some(p));

            if child_count == 1 {
                let child_iter = store.iter_nth_child(Some(p), 0).unwrap();
                if store.get_value(&child_iter, 0).get::<String>().unwrap().as_str() == "" {
                    store.remove(&child_iter);
                }
            }
        }

        let iter = store.insert_with_values(parent, None, &[(0, &name), (1, &""), (2, &false)]);

        iter
    }

    pub fn remove_note(store: &TreeStore, iter: &TreeIter) {
        let parent_iter = store.iter_parent(iter);

        // Remove the note
        store.remove(iter);

        if let Some(parent) = parent_iter {
            let child_count = store.iter_n_children(Some(&parent));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                store.insert_with_values(Some(&parent), None, &[(0, &""), (1, &""), (2, &true)]);
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
