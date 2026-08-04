use leptos::prelude::*;

pub const ZONE_DROPDOWN: i32 = 30;
pub const ZONE_BACKDROP: i32 = 40;
pub const ZONE_MODAL: i32 = 50;
pub const ZONE_NESTED_FLOATING: i32 = 60;
pub const ZONE_TOAST: i32 = 70;
pub const ZONE_TOOLTIP: i32 = 80;
pub const ZONE_CRITICAL: i32 = 90;

#[derive(Clone, Copy)]
pub struct ZStack {
    top: RwSignal<i32>,
    lock_count: RwSignal<u32>,
}

impl ZStack {
    pub fn new() -> Self {
        Self {
            top: RwSignal::new(0),
            lock_count: RwSignal::new(0),
        }
    }

    pub fn acquire_pair(&self, zone_base: i32) -> (i32, i32) {
        let current = self.top.get_untracked();
        let backdrop = current.max(zone_base) + 1;
        let content = backdrop + 1;
        self.top.set(content);
        (backdrop, content)
    }

    /// Call when an overlay opens that should block background scroll
    /// (modals, calendar/tooltip/popover panels — anything floating).
    /// Returns a guard; drop it (or call `.release()`) when the overlay closes.
    pub fn lock_scroll(&self) {
        let was_zero = self.lock_count.get_untracked() == 0;
        self.lock_count.update(|n| *n += 1);
        if was_zero {
            apply_scroll_lock(true);
        }
    }

    pub fn unlock_scroll(&self) {
        let mut now_zero = false;
        self.lock_count.update(|n| {
            if *n > 0 {
                *n -= 1;
            }
            now_zero = *n == 0;
        });
        if now_zero {
            apply_scroll_lock(false);
        }
    }
}

fn apply_scroll_lock(locked: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(doc) = leptos::prelude::document().body() {
            let _ = doc
                .class_list()
                .toggle_with_force("overflow-hidden", locked);
        }
    }
}

pub fn provide_z_stack() {
    provide_context(ZStack::new());
}

pub fn expect_z_stack() -> ZStack {
    expect_context::<ZStack>()
}
