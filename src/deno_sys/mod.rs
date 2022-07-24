/// The global `Deno` object
///
/// [Deno documentation](https://doc.deno.land/deno/stable/~/Deno)
#[allow(non_snake_case)]
pub mod Deno {
    use js_sys::Object;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        /// The `env` variable on the global `Deno` object.
        /// Allows to get, set and delete environment variables
        ///
        /// [Deno documentation](https://doc.deno.land/deno/stable/~/Deno.env)
        #[allow(non_camel_case_types)]
        #[wasm_bindgen(js_namespace = Deno, extends = Object, js_name="env")]
        #[derive(Clone, Debug)]
        pub type env;

        #[wasm_bindgen(js_namespace = Deno, static_method_of = env)]
        pub fn get(key: String) -> Option<String>;

        #[wasm_bindgen(js_namespace = Deno, static_method_of = env)]
        pub fn set(key: String, value: String);

        #[wasm_bindgen(js_namespace = Deno, static_method_of = env)]
        pub fn delete(delete: String);

        #[wasm_bindgen(js_namespace = Deno, static_method_of = env, js_name=toObject)]
        pub fn to_object() -> Object;
    }
}
