/* window.rs
 *
 * Copyright 2023 Unknown
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
//use super::tree_store::TreeStoreManager;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/bil4x4/gnote/window")]
    pub struct GnoteWindow {
        // Template widgets
        #[template_child]
        pub tree_view: TemplateChild<gtk::TreeView>,
        #[template_child]
        pub tree_store: TemplateChild<gtk::TreeStore>,
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

impl GnoteWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: GnoteWindow = glib::Object::new(&[("application", application)]);

        window.note_buffer().set_text("Hello World");
        window.tree_store().insert_with_values(None, None, &[(0, &&"name1"), (1, &&"folder1"), (2, &&"Data1")]);
        window.tree_store().insert_with_values(None, None, &[(0, &&"name2"), (1, &&"folder2"), (2, &&"Data2")]);
        window.tree_store().insert_with_values(None, None, &[(0, &&"name3"), (1, &&"folder3"), (2, &&"Data3")]);

        //TreeStoreManager::add_folder(&window.tree_store(), "test1");
        //window.tree_view().queue_draw();
        //window.tree_view().expand_all();
        window
    }

    pub fn tree_store(&self) -> gtk::TreeStore {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.tree_store.clone()
    }

    pub fn tree_view(&self) -> gtk::TreeView {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.tree_view.clone()
    }

    pub fn note_buffer(&self) -> gtk::TextBuffer {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.note_buffer.clone()
    }

    pub fn title(&self) -> gtk::Entry {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.title.clone()
    }
}
