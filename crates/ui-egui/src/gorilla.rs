//! GorillaPhoto customisation layer.
//!
//! GorillaPhoto is a fork of PhotoCraft (storytold/photocraft). To keep upstream updates easy to
//! merge, everything GorillaPhoto changes lives in this module (and `assets/gorilla/`); upstream
//! files only call into it at a few small hook points, listed in `GORILLA.md` at the repo root.
//! Internal machine names (crates, ids, the `.pcraft` format, settings folders) are left alone so
//! upstream code and existing files keep working.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock, PoisonError};

/// The product name shown to the user.
pub const APP_NAME: &str = "GorillaPhoto";

/// The upstream product name, replaced in every user-facing string.
const UPSTREAM_NAME: &str = "PhotoCraft";

/// Project links (Help menu, About, start screen).
pub const GITHUB: &str = "https://github.com/Gollira0401/GolliraPhoto";
pub const ISSUES: &str = "https://github.com/Gollira0401/GolliraPhoto/issues";
/// The upstream project GorillaPhoto is built on (credited in About).
pub const UPSTREAM_GITHUB: &str = "https://github.com/storytold/photocraft";
/// Link label for the upstream project. Deliberately not passed through `tl!`, which would rebrand it.
pub const UPSTREAM_LABEL: &str = "Based on PhotoCraft";

/// Show the upstream ArtCraft community links (Discord button, ArtCraft website). Off: the ArtCraft
/// brand licence (`docs/brand/LICENSE-brand.txt`) doesn't let forks suggest ArtCraft endorses them.
pub const SHOW_ARTCRAFT_COMMUNITY: bool = false;

/// The title bar mark, 128 px (where Photoshop shows its "Ps" tile).
pub const ICON_PNG_128: &[u8] = include_bytes!("../../../assets/gorilla/hicolor/128x128/gorillaphoto.png");
/// The window and taskbar icon, 256 px.
pub const ICON_PNG_256: &[u8] = include_bytes!("../../../assets/gorilla/hicolor/256x256/gorillaphoto.png");
/// The padded 1024 px render (macOS Dock).
pub const ICON_PNG_1024: &[u8] = include_bytes!("../../../assets/gorilla/gorillaphoto-1024.png");

/// Replace the upstream product name in a UI string.
///
/// Strings without it come back unchanged and allocation-free. Rebranded strings are interned:
/// UI strings come from a small fixed set (catalog entries and code literals), so the cache stays
/// small and the returned borrow is `'static`.
pub fn rebrand(s: &str) -> &str {
    if !s.contains(UPSTREAM_NAME) {
        return s;
    }
    static CACHE: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();
    let mut map = CACHE.get_or_init(Default::default).lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(&v) = map.get(s) {
        return v;
    }
    let v: &'static str = Box::leak(s.replace(UPSTREAM_NAME, APP_NAME).into_boxed_str());
    map.insert(s.to_owned(), v);
    v
}

/// [`rebrand`] for an owned string.
pub fn rebrand_owned(s: String) -> String {
    if s.contains(UPSTREAM_NAME) { s.replace(UPSTREAM_NAME, APP_NAME) } else { s }
}

/// GorillaPhoto's adjustments to a theme's tokens. Applied on top of every upstream theme in
/// `theme::Tokens::for_kind`, so colour changes for GorillaPhoto live here, not in `theme.rs`.
///
/// The default look is the upstream Pro theme (Photoshop's dark gray), kept as is; change
/// tokens here to give GorillaPhoto its own colours.
pub fn tune_tokens(t: crate::theme::Tokens) -> crate::theme::Tokens {
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebrand_replaces_the_product_name() {
        assert_eq!(rebrand("About PhotoCraft"), "About GorillaPhoto");
        assert_eq!(rebrand("PhotoCraftについて"), "GorillaPhotoについて");
        assert_eq!(rebrand("Layers"), "Layers");
        // Interned: the same input returns the same allocation.
        assert!(std::ptr::eq(rebrand("Quit PhotoCraft"), rebrand("Quit PhotoCraft")));
        assert_eq!(rebrand_owned("PhotoCraft 1.0".into()), "GorillaPhoto 1.0");
    }

    #[test]
    fn icons_decode() {
        assert!(eframe_free_png_check(ICON_PNG_128));
        assert!(eframe_free_png_check(ICON_PNG_256));
        assert!(eframe_free_png_check(ICON_PNG_1024));
    }

    fn eframe_free_png_check(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x89, b'P', b'N', b'G'])
    }
}
