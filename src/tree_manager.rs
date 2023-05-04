use gtk::prelude::{TreeModelExt, TreeModelExtManual, TreeModelFilterExt};

pub struct TreeManager;

impl TreeManager {
    pub fn init(store: &gtk::TreeStore) {
        let filter_model = gtk::TreeModelFilter::new(store, None);
        filter_model.set_visible_func(move |model, iter| {
            let title = model
                .get_value(iter, 0)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());

            let is_folder = model
                .get_value(iter, 2)
                .get::<bool>()
                .unwrap_or(false);

            title != "" && is_folder
        });
    }

    pub fn add_folder(store: &gtk::TreeStore, name: &str, parent: Option<&gtk::TreeIter>) -> gtk::TreeIter {
        if let Some(p) = parent {
            let child_count = store.iter_n_children(Some(p));

            if child_count == 1 {
                let child_iter = store.iter_nth_child(Some(p), 0).unwrap();
                store.remove(&child_iter);
            }
        }

        let iter = store.insert_with_values(parent, None, &[(0, &name), (1, &""), (2, &true)]);
        store.insert_with_values(Some(&iter), None, &[(0, &""), (1, &""), (2, &true)]);

        iter
    }

    pub fn remove_folder(store: &gtk::TreeStore, iter: &gtk::TreeIter) {
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

    pub fn add_note(store: &gtk::TreeStore, name: &str, parent: Option<&gtk::TreeIter>) -> gtk::TreeIter {
        if let Some(p) = parent {
            let child_count = store.iter_n_children(Some(p));

            if child_count == 1 {
                let child_iter = store.iter_nth_child(Some(p), 0).unwrap();
                store.remove(&child_iter);
            }
        }

        let iter = store.insert_with_values(parent, None, &[(0, &name), (1, &""), (2, &false)]);

        iter
    }

    pub fn remove_note(store: &gtk::TreeStore, iter: &gtk::TreeIter) {
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
}
