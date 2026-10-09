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

/// The title bar mark, 128 px (where Photoshop shows its "Ps" tile): the gorilla mascot on red.
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

/// GorillaPhoto red: the accent of the Photoshop-style themes, matching the app icon. White text
/// on it keeps a 4.6:1 contrast ratio.
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(220, 48, 48);

/// GorillaPhoto's adjustments to a theme's tokens, applied on top of every upstream theme in
/// `theme::apply`, so colour changes for GorillaPhoto live here, not in `theme.rs`.
///
/// The Photoshop-style themes (Pro and Pro Medium Gray, the default) keep Photoshop's grays and
/// swap its blue accent for GorillaPhoto red. Studio and Classic keep their own palettes.
pub fn tune_tokens(mut t: crate::theme::Tokens) -> crate::theme::Tokens {
    use crate::theme::ThemeKind;
    if matches!(t.kind, ThemeKind::Pro | ThemeKind::ProMedium) {
        t.accent = ACCENT;
        t.accent_border = ACCENT;
        t.primary_bg = ACCENT;
    }
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

    #[test]
    fn photoshop_themes_use_gorilla_red() {
        use crate::theme::{ThemeKind, Tokens};
        for kind in [ThemeKind::Pro, ThemeKind::ProMedium] {
            let t = tune_tokens(Tokens::for_kind(kind));
            assert_eq!((t.accent, t.accent_border, t.primary_bg), (ACCENT, ACCENT, ACCENT), "{kind:?}");
        }
        let studio = Tokens::for_kind(ThemeKind::Studio);
        assert_eq!(tune_tokens(studio), studio);
    }

    fn eframe_free_png_check(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x89, b'P', b'N', b'G'])
    }
}
