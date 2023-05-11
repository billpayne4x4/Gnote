use crate::widgets::gnote_tree_view::GnoteTreeView;
use adw::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bil4x4/gnote/window")]
    pub struct GnoteWindow {
        // Template widgets
        #[template_child]
        pub gnote_tree_view: TemplateChild<GnoteTreeView>,
        #[template_child]
        pub title: TemplateChild<gtk::Entry>,
        #[template_child]
        pub note: TemplateChild<gtk::TextView>,
        #[template_child]
        pub note_buffer: TemplateChild<gtk::TextBuffer>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GnoteWindow {
        const NAME: &'static str = "GnoteWindow";
        type Type = super::GnoteWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GnoteWindow {}
    impl WidgetImpl for GnoteWindow {}
    impl WindowImpl for GnoteWindow {}
    impl ApplicationWindowImpl for GnoteWindow {}
    impl AdwApplicationWindowImpl for GnoteWindow {}
}

glib::wrapper! {
    pub struct GnoteWindow(ObjectSubclass<imp::GnoteWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl GnoteWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: GnoteWindow = glib::Object::new(&[("application", application)]);
        window.imp().gnote_tree_view.add_folder("My Notes");
        window
    }

    #[template_callback]
    fn handle_add_folder_clicked(&self) {
        println!("Add folder clicked");
        self.imp().gnote_tree_view.add_folder("New Folder");
    }

    #[template_callback]
    fn handle_add_note_clicked(&self) {
        println!("Add note clicked");
        self.imp().gnote_tree_view.add_note("New Note");
    }

    #[template_callback]
    fn handle_remove_item_clicked(&self) {
        println!("Remove item clicked");
        self.imp().gnote_tree_view.remove_item()
    }
}
