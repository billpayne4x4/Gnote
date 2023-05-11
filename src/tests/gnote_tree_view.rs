/*use gtk::{
    glib::StaticType,
    TreeIter, TreeStore,
    prelude::{TreeModelExt, TreeModelExtManual},
};
use crate::io::{NoteFile, NoteFileItem};
use crate::tests::test_data;

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
    let test_data_tree_store = create_tree_store(&test_data);
    let loaded_data_tree_store = create_tree_store(&NoteFile { children: None });

    TreeManager::save(&test_data_tree_store);
    TreeManager::load(&loaded_data_tree_store);

    assert!(tree_store_equal(
        &test_data_tree_store,
        &loaded_data_tree_store
    ));
}*/
