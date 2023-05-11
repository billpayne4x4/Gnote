use gtk::{
    glib::{self, clone, ParamFlags, ParamSpec, Value, Object, ParamSpecBoolean},
    prelude::*, subclass::prelude::*, template_callbacks
};
use std::{
    sync::Once,
    cell::{Cell, RefCell}
};

// Remove the following line
// use adw::prelude::*;
mod imp {
    use gtk::glib::subclass::Signal;
    use gtk::template_callbacks;
    use once_cell::sync::Lazy;
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bil4x4/gnote/action_buttons")]
    pub struct GnoteActionButtons {
        #[template_child]
        pub add_note: TemplateChild<gtk::Button>,
        #[template_child]
        pub add_folder: TemplateChild<gtk::Button>,
        #[template_child]
        pub remove_item: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GnoteActionButtons {
        const NAME: &'static str = "GnoteActionButtons";
        type Type = super::GnoteActionButtons;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            klass.
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GnoteActionButtons {
        fn properties() -> &'static [ParamSpec] {
            use once_cell::sync::Lazy;
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
                    self.add_note.set_visible(add_note_visible);
                },
                "add-folder-visible" => {
                    let add_folder_visible = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.add_folder.set_visible(add_folder_visible);
                },
                "remove-item-visible" => {
                    let remove_item_visible = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.remove_item.set_visible(remove_item_visible);
                },
                _ => unimplemented!(),
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("add-folder").build(),
                    Signal::builder("add-note").build(),
                    Signal::builder("remove-item").build(),
                ]
            });
            SIGNALS.as_ref()
        }


        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "add-note-visible" => self.add_note.get_visible().to_value(),
                "add-folder-visible" => self.add_folder.get_visible().to_value(),
                "remove-item-visible" => self.remove_item.get_visible().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for GnoteActionButtons {}
    impl BoxImpl for GnoteActionButtons {}
}

glib::wrapper! {
    pub struct GnoteActionButtons(ObjectSubclass<imp::GnoteActionButtons>)
        @extends gtk::Widget, gtk::Box;
}

#[template_callbacks]
impl GnoteActionButtons {
    pub fn new() -> Self {
        Object::new::<Self>(&[])
    }

    #[template_callback]
    fn handle_add_note_clicked(&self, button: &gtk::Button) {
        println!("Add note");
        self.emit_by_name::<()>("add-note", &[]);
    }

    #[template_callback]
    fn handle_add_folder_clicked(&self, button: &gtk::Button) {
        println!("Add folder");
    }

    #[template_callback]
    fn handle_remove_item_clicked(&self, button: &gtk::Button) {
        println!("Remove item");
    }
}
