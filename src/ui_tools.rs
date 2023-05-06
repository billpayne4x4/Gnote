use gtk::{prelude::WidgetExt, Widget};

pub fn get_root_widget<T: WidgetExt>(widget: &T) -> Option<Widget> {
    let mut parent = widget.parent();
    while let Some(p) = parent {
        parent = p.parent();
    }
    parent
}
