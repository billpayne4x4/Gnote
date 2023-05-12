use adw::gdk::Display;
use adw::gio::UnixSocketAddressType::Path;
use gtk::gdk::{ContentFormats, Paintable, Texture};
use gtk::gio::Cancellable;
use gtk::{
    builders::FileChooserDialogBuilder,
    gdk::{Key, ModifierType},
    gdk_pixbuf,
    glib::{self, clone, Object, ParamFlags, ParamSpec, ParamSpecString, Value},
    interface_age, pango,
    prelude::*,
    subclass::prelude::*,
    FileChooserAction, GestureClick, Image, Inhibit, ResponseType, TextBuffer, TextView,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
use std::{
    cell::{Cell, RefCell},
    sync::Once,
};

#[derive(Serialize, Deserialize)]
struct Element {
    r#type: String,
    data: String,
    start_iter: u32,
}

#[derive(Serialize, Deserialize)]
struct TextBufferContent {
    text: String,
    elements: Vec<Element>,
}

const INDENT: &'static str = "  ";
const BULLET: char = '•';
const CHECK_BOX_UNCHECKED: char = '☐';
const CHECK_BOX_CHECKED: char = '☑';
const SPECIAL_CHAR_PADDING: &'static str = " "; // After the bullet and check box characters

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct GnoteTextBuffer;

    #[glib::object_subclass]
    impl ObjectSubclass for GnoteTextBuffer {
        const NAME: &'static str = "GnoteTextBuffer";
        type Type = super::GnoteTextBuffer;
        type ParentType = TextBuffer;
    }

    impl ObjectImpl for GnoteTextBuffer {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for GnoteTextBuffer {}
    impl TextBufferImpl for GnoteTextBuffer {}
}

glib::wrapper! {
    pub struct GnoteTextBuffer(ObjectSubclass<imp::GnoteTextBuffer>)
        @extends TextBuffer;
}

impl GnoteTextBuffer {
    pub fn new() -> Self {
        Object::new::<Self>(&[])
    }

    pub fn init(&self, text_view: &gtk::TextView) {
        let key_controller = gtk::EventControllerKey::new();
        text_view.add_controller(&key_controller);

        let self_clone = self.clone();
        key_controller.connect_key_pressed(move |_controller, key, _keycode, state| {
            let is_enter_or_return = key == Key::Return || key == Key::KP_Enter;

            if is_enter_or_return && !state.contains(ModifierType::SHIFT_MASK) {
                let buffer = self_clone.imp().instance();

                let mut current_iter = buffer.iter_at_mark(&buffer.get_insert());
                let mut start = current_iter.clone();
                let mut end = start.clone();
                end.forward_to_line_end();
                start.set_line_offset(0);

                let line_text = buffer.text(&start, &end, false);

                let bullet_pattern = format!(
                    r"^((?:{})*)(?:({}|{}|{}))",
                    regex::escape(INDENT),
                    regex::escape(BULLET.to_string().as_str()),
                    regex::escape(
                        (CHECK_BOX_CHECKED.to_string() + SPECIAL_CHAR_PADDING.to_string().as_str())
                            .as_str()
                    ),
                    regex::escape(
                        (CHECK_BOX_UNCHECKED.to_string()
                            + SPECIAL_CHAR_PADDING.to_string().as_str())
                        .as_str()
                    ),
                );
                let bullet_re = Regex::new(&bullet_pattern).unwrap();
                if let Some(captures) = bullet_re.captures(&line_text) {
                    if let Some(indent_capture) = captures.get(1) {
                        if let Some(char_capture) = captures.get(2) {
                            if let Some(mut first_char) =
                                String::from(char_capture.as_str()).chars().nth(0)
                            {
                                let indent_count = indent_capture.as_str().len() / INDENT.len();

                                if first_char == CHECK_BOX_CHECKED {
                                    first_char = CHECK_BOX_UNCHECKED;
                                }

                                buffer.insert(
                                    &mut current_iter,
                                    format!(
                                        "\n{}{}{}",
                                        INDENT.repeat(indent_count),
                                        first_char,
                                        SPECIAL_CHAR_PADDING
                                    )
                                    .as_str(),
                                );
                                return Inhibit(true);
                            }
                        }
                    }
                }
            }
            Inhibit(false)
        });

        let gesture_click = GestureClick::new();
        gesture_click.connect_pressed(clone!(@weak self as self_clone, @weak text_view => move |_gesture, n_press, x, y| {
            if n_press == 1 { // Single click
                let buffer = self_clone.imp().instance();
                if let Some(mut iter) = text_view.iter_at_location(x as i32, y as i32) {
                    iter.backward_char();

                    if iter.char() == CHECK_BOX_CHECKED || iter.char() == CHECK_BOX_UNCHECKED {
                        let new_check_box_char = if iter.char() == CHECK_BOX_CHECKED { CHECK_BOX_UNCHECKED } else { CHECK_BOX_CHECKED };
                        let mut end_iter = iter.clone();
                        end_iter.forward_char();
                        buffer.delete(&mut iter, &mut end_iter);
                        buffer.insert(&mut iter, &new_check_box_char.to_string());
                    }
                }
            }
        }));
        text_view.add_controller(&gesture_click);

        text_view.connect_paste_clipboard(|text_view| {
            if let Some(display) = Display::default() {
                let clipboard = display.clipboard();
                clipboard.read_texture_async(
                    Cancellable::NONE,
                    clone!(@weak text_view => move |res| {
                        let buffer = text_view.buffer();
                        if let Ok(Some(texture)) = res {
                            let mut start = buffer.iter_at_mark(&buffer.get_insert());
                            buffer.insert_paintable(&mut start, &texture);
                        }
                    }),
                );
            }
        });
    }

    pub fn insert_image(&self, text_view: &gtk::TextView) {
        let buffer = self.imp().instance();

        // Create a new FileChooserDialog
        let file_chooser = FileChooserDialogBuilder::new()
            .title("Choose an image file")
            .action(FileChooserAction::Open)
            .build();

        file_chooser.add_buttons(&[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);

        file_chooser.connect_response(clone!(@weak buffer => move |dialog, response| {
            if response == ResponseType::Ok {
                if let Some(file) = dialog.file() {
                    if let Some(file_path) = file.path() {
                        // Load the image into a Pixbuf
                        let pixbuf = gdk_pixbuf::Pixbuf::from_file(&file_path).expect("Could not load image");
                        let texture = Texture::for_pixbuf(&pixbuf);
                        let mut start = buffer.iter_at_mark(&buffer.get_insert());
                        buffer.insert_paintable(&mut start, &texture);
                    }
                }
            }
            dialog.destroy();
        }));

        file_chooser.present();
    }

    pub fn insert_check_box(&self, text_view: &gtk::TextView) {
        let buffer = self.imp().instance();
        let mut start = buffer.iter_at_mark(&buffer.get_insert());
        start.set_line_offset(0);

        let mut end = start.clone();
        end.forward_to_line_end();

        let line_text = CHECK_BOX_UNCHECKED.to_string()
            + SPECIAL_CHAR_PADDING
            + buffer.text(&start, &end, false).trim_start();
        buffer.delete(&mut start, &mut end);
        buffer.insert(&mut start, &line_text);
    }

    pub fn insert_bullet(&self) {
        let buffer = self.imp().instance();
        let mut start = buffer.iter_at_mark(&buffer.get_insert());
        start.set_line_offset(0);

        let mut end = start.clone();
        end.forward_to_line_end();

        let line_text = BULLET.to_string()
            + SPECIAL_CHAR_PADDING
            + buffer.text(&start, &end, false).trim_start();
        buffer.delete(&mut start, &mut end);
        buffer.insert(&mut start, &line_text);
    }

    pub fn indent_more(&self) {
        let buffer = self.imp().instance();
        let mut current_iter = buffer.iter_at_mark(&buffer.get_insert());
        let mut start_of_line = buffer.iter_at_line(current_iter.line()).unwrap();

        buffer.insert(&mut start_of_line, INDENT);
    }

    pub fn indent_less(&self) {
        let buffer = self.imp().instance();
        let mut current_iter = buffer.iter_at_mark(&buffer.get_insert());
        let mut start_of_line = buffer.iter_at_line(current_iter.line()).unwrap();
        let mut end_of_indent = start_of_line.clone();

        end_of_indent.forward_chars(INDENT.len() as i32);

        let text_at_start = buffer.text(&start_of_line, &end_of_indent, false);

        if text_at_start.as_str() == INDENT {
            buffer.delete(&mut start_of_line, &mut end_of_indent);
        }
    }
}
