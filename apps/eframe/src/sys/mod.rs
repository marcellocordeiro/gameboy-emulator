cfg_select! {
    target_arch = "wasm32" => {
        mod wasm;
        pub use wasm::*;
    }

    _ => {
        mod native;
        pub use native::*;
    }
}
