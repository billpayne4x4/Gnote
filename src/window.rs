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

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::{Ref, RefCell, RefMut};
use crate::tree_manager::TreeManager;
use crate::note_manager::NoteManager;
use crate::title_manager::TitleManager;

mod imp {
    use crate::note_manager::NoteManager;
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
        #[template_child]
        pub add_note: TemplateChild<gtk::Button>,
        #[template_child]
        pub add_folder: TemplateChild<gtk::Button>,
        #[template_child]
        pub remove_item: TemplateChild<gtk::Button>,

        pub tree_manager: RefCell<TreeManager>,
        pub note_manager: RefCell<NoteManager>,
        pub title_manager: RefCell<TitleManager>,
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

    impl ObjectImpl for GnoteWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let tree_manager = TreeManager::new(&self.tree_view, &self.tree_store, &self.title, &self.note_buffer, &self.add_note, &self.add_folder, &self.remove_item);
            self.tree_manager.replace(tree_manager);

            let note_manager = NoteManager::new(&self.note, &self.note_buffer);
            self.note_manager.replace(note_manager);

            let title_manager = TitleManager::new(&self.title);
            self.title_manager.replace(title_manager);
        }
    }
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
        let mut window: GnoteWindow = glib::Object::new(&[("application", application)]);


        {
            let tree = window.tree_view();
            let store = window.tree_store();

            window.title_manager().init(&tree);
            window.note_manager().init(&tree);

            let mut iter1: gtk::TreeIter;
            let mut iter2: gtk::TreeIter;
            {
                window.tree_manager().init(application.as_ref());
            }
            {
                iter1 = window.tree_manager().add_folder("My Notes");
            }
            {
                window.tree_manager_mut().set_selected_iter(Some(iter1));
            }
            {
                iter2 = window.tree_manager().add_folder("My Folder 1");
            }
            {
                window.tree_manager_mut().set_selected_iter(Some(iter2));
            }
            {
                let tree_manager = window.tree_manager();
                tree_manager.add_folder("My Folder 2");
                tree_manager.add_note("My Note");
            }
            // Test data
            /*tree_manager.set_selected_iter(Some(tree_manager.add_folder("My Notes")));
            tree_manager.add_folder("My Folder 11");
            tree_manager.set_selected_iter(Some(tree_manager.add_folder("My Folder 22")));
            tree_manager.add_note("My Note");*/
            //TreeManager::remove_folder(&store, &iter2);
            //TreeManager::remove_note(&store, &iter4);
        }
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

    pub fn add_note(&self) -> gtk::Button {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.add_note.clone()
    }

    pub fn add_folder(&self) -> gtk::Button {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.add_folder.clone()
    }

    pub fn remove_item(&self) -> gtk::Button {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.remove_item.clone()
    }

    pub fn tree_manager(&self) -> Ref<TreeManager> {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.tree_manager.borrow()
    }

    pub fn tree_manager_mut(&mut self) -> RefMut<TreeManager> {
        let mut self_ = imp::GnoteWindow::from_instance(self);
        self_.tree_manager.borrow_mut()
    }

    pub fn note_manager(&self) -> Ref<NoteManager> {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.note_manager.borrow()
    }

    pub fn note_manager_mut(&mut self) -> RefMut<NoteManager> {
        let mut self_ = imp::GnoteWindow::from_instance(self);
        self_.note_manager.borrow_mut()
    }

    pub fn title_manager(&self) -> Ref<TitleManager> {
        let self_ = imp::GnoteWindow::from_instance(self);
        self_.title_manager.borrow()
    }

    pub fn title_manager_mut(&mut self) -> RefMut<TitleManager> {
        let mut self_ = imp::GnoteWindow::from_instance(self);
        self_.title_manager.borrow_mut()
    }
}