use leptos::prelude::*;

/// SSR-safe lookup of the shared overlay mount point. Returns `None` on
/// any non-wasm32 target (server render) or if the element is missing
/// client-side. Every portal-based component should route through this
/// instead of re-deriving the cfg guard.
pub fn overlay_root() -> Option<web_sys::Element> {
    #[cfg(target_arch = "wasm32")]
    {
        document().get_element_by_id("modal-root")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
