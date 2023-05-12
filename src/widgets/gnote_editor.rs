use gtk::{
    glib::{self, clone, Object, ParamFlags, ParamSpec, ParamSpecString, Value},
    prelude::*,
    subclass::prelude::*,
    template_callbacks,
};
use std::{
    cell::{Cell, RefCell},
    sync::Once,
};

// Remove the following line
// use adw::prelude::*;
mod imp {
    use super::*;
    use crate::widgets::gnote_text_buffer::GnoteTextBuffer;
    use gtk::glib::subclass::Signal;
    use gtk::template_callbacks;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bil4x4/gnote/editor")]
    pub struct GnoteEditor {
        #[template_child]
        pub title: TemplateChild<gtk::Entry>,
        #[template_child]
        pub note: TemplateChild<gtk::TextView>,
        #[template_child]
        pub note_buffer: TemplateChild<GnoteTextBuffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GnoteEditor {
        const NAME: &'static str = "GnoteEditor";
        type Type = super::GnoteEditor;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GnoteEditor {
        fn properties() -> &'static [ParamSpec] {
            use once_cell::sync::Lazy;
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("title").build(),
                    ParamSpecString::builder("note").build(),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("title-changed")
                        .param_types([<String>::static_type()])
                        .build(),
                    Signal::builder("note-changed")
                        .param_types([<String>::static_type()])
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "title" => {
                    let title = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.title.set_text(title)
                }
                "add-folder-visible" => {
                    let note = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.note_buffer.set_text(note)
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "title" => self.title.text().as_str().to_value(),
                "note" => {
                    let start = self.note_buffer.start_iter();
                    let end = self.note_buffer.end_iter();
                    self.note_buffer
                        .text(&start, &end, true)
                        .as_str()
                        .to_value()
                }
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();

            let note_css_provider = gtk::CssProvider::new();
            note_css_provider.load_from_data(
                "\
                textview {\
                font-family: sans;\
                font-size: 24px;\
                padding: 10px;\
            }"
                .as_bytes(),
            );
            gtk::StyleContext::add_provider(
                &self.note.style_context(),
                &note_css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            let title_css_provider = gtk::CssProvider::new();
            title_css_provider.load_from_data(
                "\
                entry {\
                font-family: sans;\
                font-size: 32px;\
                padding-left: 10px;\
                padding-right: 10px;\
                padding-top: 20px;\
                padding-bottom: 10px;\
            }"
                .as_bytes(),
            );
            gtk::StyleContext::add_provider(
                &self.title.style_context(),
                &title_css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            self.note_buffer.init(&self.note);
        }
    }
    impl WidgetImpl for GnoteEditor {}
    impl BoxImpl for GnoteEditor {}
}

glib::wrapper! {
    pub struct GnoteEditor(ObjectSubclass<imp::GnoteEditor>)
        @extends gtk::Widget, gtk::Box;
}

#[template_callbacks]
impl GnoteEditor {
    pub fn new() -> Self {
        Object::new::<Self>(&[])
    }

    #[template_callback]
    fn handle_add_note_clicked(&self, button: &gtk::Button) {
        self.emit_by_name::<()>("add-note", &[]);
    }

    #[template_callback]
    fn handle_title_changed(&self, title: &gtk::Entry) {
        println!("Title Changed");
    }

    #[template_callback]
    fn handle_note_buffer_changed(&self, note_buffer: &gtk::TextBuffer) {
        println!("Note Changed");
    }

    #[template_callback]
    fn handle_insert_image_clicked(&self, button: &gtk::Button) {
        self.imp().note_buffer.insert_image(&self.imp().note);
        self.imp().note.grab_focus();
    }

    #[template_callback]
    fn handle_insert_check_box_clicked(&self, button: &gtk::Button) {
        self.imp().note_buffer.insert_check_box(&self.imp().note);
        self.imp().note.grab_focus();
    }

    #[template_callback]
    fn handle_bullet_point_clicked(&self, button: &gtk::Button) {
        self.imp().note_buffer.insert_bullet();
        self.imp().note.grab_focus();
    }

    #[template_callback]
    fn handle_indent_less_clicked(&self, button: &gtk::Button) {
        self.imp().note_buffer.indent_less();
        self.imp().note.grab_focus();
    }

    #[template_callback]
    fn handle_indent_more_clicked(&self, button: &gtk::Button) {
        self.imp().note_buffer.indent_more();
        self.imp().note.grab_focus();
    }
}
