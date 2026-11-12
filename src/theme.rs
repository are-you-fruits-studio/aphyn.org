//! Light and dark themes. `index.html` picks the theme before the first paint
//! (saved choice, else the system setting, else dark); this module reads that
//! choice and switches it.

const STORAGE_KEY: &str = "aphyn-theme";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn current() -> Theme {
        let light = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
            .and_then(|e| e.get_attribute("data-theme"))
            .is_some_and(|t| t == "light");
        if light {
            Theme::Light
        } else {
            Theme::Dark
        }
    }

    pub fn toggled(self) -> Theme {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    /// Applies the theme to the page and remembers it for the next visit.
    pub fn apply(self) {
        let Some(window) = web_sys::window() else {
            return;
        };
        if let Some(root) = window.document().and_then(|d| d.document_element()) {
            let _ = root.set_attribute("data-theme", self.name());
        }
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(STORAGE_KEY, self.name());
        }
    }
}
