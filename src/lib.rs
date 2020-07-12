extern crate cfg_if;
extern crate wasm_bindgen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        use wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    if #[cfg(feature = "simple_replace")] {
        #[wasm_bindgen]
        pub fn render_template(template: String, name: String) -> String {
            (template + "\n\n(rust, simple replace)").replace("{{ name }}", &name)
        }
    }
}


cfg_if! {
    if #[cfg(feature = "regex")] {
        use regex::Regex;
        #[wasm_bindgen]
        pub fn render_template(template: String, name: String) -> String {
            let name_re = Regex::new(r"\{\{ ?name ?\}\}").unwrap();
            return name_re.replace_all(&(template + "\n\n(rust, regex)"), name.as_str()).to_string();
        }
    }
}
