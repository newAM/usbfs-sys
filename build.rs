fn main() {
    #[cfg(feature = "bindgen")]
    {
        use bindgen::callbacks::ParseCallbacks;
        use std::env;
        use std::path::PathBuf;

        #[derive(Debug)]
        struct Parse;
        impl ParseCallbacks for Parse {
            fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
                if name.starts_with("USBDEVFS_URB_TYPE_") {
                    Some(bindgen::callbacks::IntKind::U8)
                } else {
                    None
                }
            }
            fn item_name(&self, original_item_name: &str) -> Option<String> {
                // TODO: maybe leave names unchanged to help online searches for these terms
                for prefix in &["USBDEVFS_", "usbdevfs_"] {
                    if original_item_name.starts_with(prefix) {
                        return Some(original_item_name[prefix.len()..].to_string());
                    }
                }
                None
            }
        }

        // The bindgen::Builder is the main entry point
        // to bindgen, and lets you build up options for
        // the resulting bindings.
        let bindings = bindgen::Builder::default()
            // TODO: actually might be simpler to copy the headers into this repo,
            .header("/usr/include/linux/usbdevice_fs.h")
            .generate_comments(true)
            .whitelist_type("^usbdevfs.*")
            .whitelist_function("^usbdevfs.*")
            .whitelist_var("^USBDEVFS.*")
            .parse_callbacks(Box::new(Parse))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
