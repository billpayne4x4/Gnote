fn main() {
    glib_build_tools::compile_resources(
        &["src/ui/"],
        "src/ui/gnote.gresource.xml",
        "gnote.gresource",
    );
}