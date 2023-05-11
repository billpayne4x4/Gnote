use crate::{
    log_error,
    tools::io::{self, NoteFile, NoteFileItem},
};
use gtk::{
    glib::{self, ParamSpec, ParamSpecBoolean, Value},
    prelude::*,
    subclass::prelude::*,
    TreeIter, TreePath, TreeStore, TreeViewColumn,
};
use once_cell::sync::Lazy;
use std::cell::Cell;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bil4x4/gnote/tree_view")]
    pub struct GnoteTreeView {
        #[template_child]
        pub tree_store: TemplateChild<gtk::TreeStore>,
        #[template_child]
        pub tree_selection: TemplateChild<gtk::TreeSelection>,

        pub add_note_visible: Cell<bool>,
        pub add_folder_visible: Cell<bool>,
        pub remove_item_visible: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GnoteTreeView {
        const NAME: &'static str = "GnoteTreeView";
        type Type = super::GnoteTreeView;
        type ParentType = gtk::TreeView;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GnoteTreeView {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecBoolean::builder("add-note-visible")
                        .default_value(false)
                        .build(),
                    ParamSpecBoolean::builder("add-folder-visible")
                        .default_value(false)
                        .build(),
                    ParamSpecBoolean::builder("remove-item-visible")
                        .default_value(false)
                        .build(),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "add-note-visible" => {
                    let add_note_visible = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.add_note_visible.replace(add_note_visible);
                }
                "add-folder-visible" => {
                    let add_folder_visible = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.add_folder_visible.replace(add_folder_visible);
                }
                "remove-item-visible" => {
                    let remove_item_visible = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.remove_item_visible.replace(remove_item_visible);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "add-note-visible" => self.add_note_visible.get().to_value(),
                "add-folder-visible" => self.add_folder_visible.get().to_value(),
                "remove-item-visible" => self.remove_item_visible.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for GnoteTreeView {}
    impl TreeViewImpl for GnoteTreeView {}
}

glib::wrapper! {
    pub struct GnoteTreeView(ObjectSubclass<imp::GnoteTreeView>)
        @extends gtk::Widget, gtk::TreeView;
}

#[gtk::template_callbacks]
impl GnoteTreeView {
    pub fn new() -> Self {
        let gnote_tree_view = glib::Object::new::<Self>(&[]);

        gnote_tree_view
    }

    #[template_callback]
    pub fn handle_row_activated(
        &self,
        _tree_path: &TreePath,
        _tree_view_column: Option<&TreeViewColumn>,
    ) {
        println!("Selection changed");

        if let Some((_, iter)) = self.imp().tree_selection.selected() {
            //let name = self.imp().tree_store.get_value(&iter, 0).get::<String>().unwrap();
            let is_folder = self
                .imp()
                .tree_store
                .get_value(&iter, 2)
                .get::<bool>()
                .unwrap();
            let has_parent = self.imp().tree_store.iter_parent(&iter).is_some();

            if !has_parent {
                self.imp().remove_item_visible.set(false);
            } else {
                self.imp().remove_item_visible.set(true);
            }
            self.notify("remove-item-visible");

            if is_folder {
                self.imp().add_folder_visible.set(true);
                self.imp().add_note_visible.set(true);
            } else {
                self.imp().add_folder_visible.set(false);
                self.imp().add_note_visible.set(false);
            }
            self.notify("add-folder-visible");
            self.notify("add-note-visible");
        }
    }

    pub fn add_folder(&self, name: &str) {
        if let Some((_, selected_iter)) = self.imp().tree_selection.selected() {
            let child_count = self.imp().tree_store.iter_n_children(Some(&selected_iter));

            if child_count == 1 {
                let child_iter = self
                    .imp()
                    .tree_store
                    .iter_nth_child(Some(&selected_iter), 0)
                    .unwrap();
                if self
                    .imp()
                    .tree_store
                    .get_value(&child_iter, 0)
                    .get::<String>()
                    .unwrap()
                    .as_str()
                    == ""
                {
                    self.imp().tree_store.remove(&child_iter);
                }
            }
        }

        let selected = self.imp().tree_selection.selected();
        let iter = self.imp().tree_store.insert_with_values(
            selected.as_ref().map(|(_, iter)| iter),
            None,
            &[(0, &name), (1, &""), (2, &true)],
        );

        self.imp().tree_store.insert_with_values(
            Some(&iter),
            None,
            &[(0, &""), (1, &""), (2, &true)],
        );
    }

    pub fn add_note(&self, name: &str) {
        if let Some((_, selected_iter)) = self.imp().tree_selection.selected() {
            let child_count = self.imp().tree_store.iter_n_children(Some(&selected_iter));

            if child_count == 1 {
                let child_iter = self
                    .imp()
                    .tree_store
                    .iter_nth_child(Some(&selected_iter), 0)
                    .unwrap();
                if self
                    .imp()
                    .tree_store
                    .get_value(&child_iter, 0)
                    .get::<String>()
                    .unwrap()
                    .as_str()
                    == ""
                {
                    self.imp().tree_store.remove(&child_iter);
                }
            }
        }

        let selected = self.imp().tree_selection.selected();
        self.imp().tree_store.insert_with_values(
            selected.as_ref().map(|(_, iter)| iter),
            None,
            &[(0, &name), (1, &""), (2, &false)],
        );
    }

    pub fn remove_item(&self) {
        let selected = self.imp().tree_selection.selected();
        let (_, selected_iter) = selected.unwrap();

        if self
            .imp()
            .tree_store
            .get_value(&selected_iter, 2)
            .get::<bool>()
            .unwrap()
        {
            self.remove_folder();
        } else {
            self.remove_note();
        }
    }

    fn remove_folder(&self) {
        let selected = self.imp().tree_selection.selected();
        let (_, selected_iter) = selected.unwrap();

        let parent_iter = self.imp().tree_store.iter_parent(&selected_iter);

        // Remove the folder
        self.imp().tree_store.remove(&selected_iter);

        if let Some(parent) = parent_iter {
            let child_count = self.imp().tree_store.iter_n_children(Some(&parent));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                self.imp().tree_store.insert_with_values(
                    Some(&parent),
                    None,
                    &[(0, &""), (1, &""), (2, &true)],
                );
            }
        }
    }

    fn remove_note(&self) {
        let selected = self.imp().tree_selection.selected();
        let (_, selected_iter) = selected.unwrap();

        let parent_iter = self.imp().tree_store.iter_parent(&selected_iter);

        // Remove the note
        self.imp().tree_store.remove(&selected_iter);

        if let Some(parent) = parent_iter {
            let child_count = self.imp().tree_store.iter_n_children(Some(&parent));

            // If the parent has no children left, add a dummy item
            if child_count == 0 {
                self.imp().tree_store.insert_with_values(
                    Some(&parent),
                    None,
                    &[(0, &""), (1, &""), (2, &true)],
                );
            }
        }
    }

    pub fn save(&self) {
        println!("Saving...");
        fn build_note_file_item(tree_store: &TreeStore, iter: &TreeIter) -> NoteFileItem {
            let title = tree_store
                .get_value(iter, 0)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());
            let body = tree_store
                .get_value(iter, 1)
                .get::<String>()
                .unwrap_or_else(|_| "".to_string());
            let is_folder = tree_store.get_value(iter, 2).get::<bool>().unwrap_or(false);

            let mut children = None;
            if is_folder {
                if let Some(mut child_iter) = tree_store.iter_children(Some(iter)) {
                    let mut child_items = Vec::new();

                    loop {
                        child_items.push(build_note_file_item(&tree_store, &child_iter));
                        if !tree_store.iter_next(&mut child_iter) {
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

        let mut root_iter = self.imp().tree_store.iter_nth_child(None, 0).unwrap();
        let mut root_items = Vec::new();

        loop {
            root_items.push(build_note_file_item(&self.imp().tree_store, &root_iter));
            if !self.imp().tree_store.iter_next(&mut root_iter) {
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

    pub fn load(&self) {
        self.imp().tree_store.clear();

        let note_file = match NoteFile::load(&io::get_notes_path()) {
            Ok(note_file_load) => note_file_load,
            Err(e) => {
                log_error!("Failed to load file - {}", e);
                return;
            }
        };

        fn insert_note_file_item(
            tree_store: &TreeStore,
            item: &NoteFileItem,
            parent: Option<&TreeIter>,
        ) {
            let iter = tree_store.insert_with_values(
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
                    insert_note_file_item(tree_store, child, Some(&iter));
                }
            }
        }

        if let Some(root_items) = &note_file.children {
            for item in root_items {
                insert_note_file_item(&self.imp().tree_store, item, None);
            }
        }
    }
}
