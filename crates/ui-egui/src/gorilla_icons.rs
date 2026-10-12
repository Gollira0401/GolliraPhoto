//! GorillaPhoto's original Photoshop-style tool icons (generated from `assets/gorilla/tools/`).
//!
//! Plugged into `icons.rs` at two hook points (see `GORILLA.md`): `ICONS` joins the embedded icon
//! list and `tool_icon` picks the icon for each tool before the upstream mapping.

use crate::state::Tool;

/// `(name, svg)` pairs, names prefixed `gp-`.
pub const ICONS: &[(&str, &[u8])] = &[
    ("gp-background-eraser", include_bytes!("../../../assets/gorilla/tools/gp-background-eraser.svg")),
    ("gp-blur", include_bytes!("../../../assets/gorilla/tools/gp-blur.svg")),
    ("gp-brush", include_bytes!("../../../assets/gorilla/tools/gp-brush.svg")),
    ("gp-burn", include_bytes!("../../../assets/gorilla/tools/gp-burn.svg")),
    ("gp-clone-stamp", include_bytes!("../../../assets/gorilla/tools/gp-clone-stamp.svg")),
    ("gp-content-aware-move", include_bytes!("../../../assets/gorilla/tools/gp-content-aware-move.svg")),
    ("gp-count", include_bytes!("../../../assets/gorilla/tools/gp-count.svg")),
    ("gp-crop", include_bytes!("../../../assets/gorilla/tools/gp-crop.svg")),
    ("gp-custom-shape", include_bytes!("../../../assets/gorilla/tools/gp-custom-shape.svg")),
    ("gp-direct-selection", include_bytes!("../../../assets/gorilla/tools/gp-direct-selection.svg")),
    ("gp-dodge", include_bytes!("../../../assets/gorilla/tools/gp-dodge.svg")),
    ("gp-ellipse-marquee", include_bytes!("../../../assets/gorilla/tools/gp-ellipse-marquee.svg")),
    ("gp-ellipse-shape", include_bytes!("../../../assets/gorilla/tools/gp-ellipse-shape.svg")),
    ("gp-eraser", include_bytes!("../../../assets/gorilla/tools/gp-eraser.svg")),
    ("gp-eyedropper", include_bytes!("../../../assets/gorilla/tools/gp-eyedropper.svg")),
    ("gp-gradient", include_bytes!("../../../assets/gorilla/tools/gp-gradient.svg")),
    ("gp-hand", include_bytes!("../../../assets/gorilla/tools/gp-hand.svg")),
    ("gp-healing", include_bytes!("../../../assets/gorilla/tools/gp-healing.svg")),
    ("gp-history-brush", include_bytes!("../../../assets/gorilla/tools/gp-history-brush.svg")),
    ("gp-lasso", include_bytes!("../../../assets/gorilla/tools/gp-lasso.svg")),
    ("gp-line", include_bytes!("../../../assets/gorilla/tools/gp-line.svg")),
    ("gp-magic-eraser", include_bytes!("../../../assets/gorilla/tools/gp-magic-eraser.svg")),
    ("gp-magic-wand", include_bytes!("../../../assets/gorilla/tools/gp-magic-wand.svg")),
    ("gp-magnetic-lasso", include_bytes!("../../../assets/gorilla/tools/gp-magnetic-lasso.svg")),
    ("gp-mixer-brush", include_bytes!("../../../assets/gorilla/tools/gp-mixer-brush.svg")),
    ("gp-move", include_bytes!("../../../assets/gorilla/tools/gp-move.svg")),
    ("gp-note", include_bytes!("../../../assets/gorilla/tools/gp-note.svg")),
    ("gp-object-selection", include_bytes!("../../../assets/gorilla/tools/gp-object-selection.svg")),
    ("gp-paint-bucket", include_bytes!("../../../assets/gorilla/tools/gp-paint-bucket.svg")),
    ("gp-patch", include_bytes!("../../../assets/gorilla/tools/gp-patch.svg")),
    ("gp-path-selection", include_bytes!("../../../assets/gorilla/tools/gp-path-selection.svg")),
    ("gp-pattern-stamp", include_bytes!("../../../assets/gorilla/tools/gp-pattern-stamp.svg")),
    ("gp-pen", include_bytes!("../../../assets/gorilla/tools/gp-pen.svg")),
    ("gp-pencil", include_bytes!("../../../assets/gorilla/tools/gp-pencil.svg")),
    ("gp-polygon", include_bytes!("../../../assets/gorilla/tools/gp-polygon.svg")),
    ("gp-polygon-lasso", include_bytes!("../../../assets/gorilla/tools/gp-polygon-lasso.svg")),
    ("gp-quick-selection", include_bytes!("../../../assets/gorilla/tools/gp-quick-selection.svg")),
    ("gp-rect-marquee", include_bytes!("../../../assets/gorilla/tools/gp-rect-marquee.svg")),
    ("gp-rectangle", include_bytes!("../../../assets/gorilla/tools/gp-rectangle.svg")),
    ("gp-red-eye", include_bytes!("../../../assets/gorilla/tools/gp-red-eye.svg")),
    ("gp-ruler", include_bytes!("../../../assets/gorilla/tools/gp-ruler.svg")),
    ("gp-sharpen", include_bytes!("../../../assets/gorilla/tools/gp-sharpen.svg")),
    ("gp-slice", include_bytes!("../../../assets/gorilla/tools/gp-slice.svg")),
    ("gp-slice-select", include_bytes!("../../../assets/gorilla/tools/gp-slice-select.svg")),
    ("gp-smudge", include_bytes!("../../../assets/gorilla/tools/gp-smudge.svg")),
    ("gp-sponge", include_bytes!("../../../assets/gorilla/tools/gp-sponge.svg")),
    ("gp-spot-healing", include_bytes!("../../../assets/gorilla/tools/gp-spot-healing.svg")),
    ("gp-triangle", include_bytes!("../../../assets/gorilla/tools/gp-triangle.svg")),
    ("gp-type", include_bytes!("../../../assets/gorilla/tools/gp-type.svg")),
    ("gp-vertical-type", include_bytes!("../../../assets/gorilla/tools/gp-vertical-type.svg")),
    ("gp-zoom", include_bytes!("../../../assets/gorilla/tools/gp-zoom.svg")),
];

/// The GorillaPhoto icon for a tool.
pub fn tool_icon(t: Tool) -> Option<&'static str> {
    Some(match t {
        Tool::Move => "gp-move",
        Tool::RectMarquee => "gp-rect-marquee",
        Tool::EllipseMarquee => "gp-ellipse-marquee",
        Tool::Lasso => "gp-lasso",
        Tool::PolygonLasso => "gp-polygon-lasso",
        Tool::MagneticLasso => "gp-magnetic-lasso",
        Tool::MagicWand => "gp-magic-wand",
        Tool::Crop => "gp-crop",
        Tool::Eyedropper => "gp-eyedropper",
        Tool::Ruler => "gp-ruler",
        Tool::Note => "gp-note",
        Tool::Count => "gp-count",
        Tool::Brush => "gp-brush",
        Tool::Pencil => "gp-pencil",
        Tool::MixerBrush => "gp-mixer-brush",
        Tool::Eraser => "gp-eraser",
        Tool::BackgroundEraser => "gp-background-eraser",
        Tool::MagicEraser => "gp-magic-eraser",
        Tool::Gradient => "gp-gradient",
        Tool::PaintBucket => "gp-paint-bucket",
        Tool::Type => "gp-type",
        Tool::VerticalType => "gp-vertical-type",
        Tool::Hand => "gp-hand",
        Tool::Zoom => "gp-zoom",
        Tool::SpotHealing => "gp-spot-healing",
        Tool::Healing => "gp-healing",
        Tool::Patch => "gp-patch",
        Tool::ContentAwareMove => "gp-content-aware-move",
        Tool::RedEye => "gp-red-eye",
        Tool::CloneStamp => "gp-clone-stamp",
        Tool::PatternStamp => "gp-pattern-stamp",
        Tool::HistoryBrush => "gp-history-brush",
        Tool::Blur => "gp-blur",
        Tool::Sharpen => "gp-sharpen",
        Tool::Smudge => "gp-smudge",
        Tool::Dodge => "gp-dodge",
        Tool::Burn => "gp-burn",
        Tool::Sponge => "gp-sponge",
        Tool::QuickSelection => "gp-quick-selection",
        Tool::ObjectSelection => "gp-object-selection",
        Tool::Pen => "gp-pen",
        Tool::PathSelection => "gp-path-selection",
        Tool::DirectSelection => "gp-direct-selection",
        Tool::Rectangle => "gp-rectangle",
        Tool::EllipseShape => "gp-ellipse-shape",
        Tool::Triangle => "gp-triangle",
        Tool::Polygon => "gp-polygon",
        Tool::Line => "gp-line",
        Tool::CustomShape => "gp-custom-shape",
        Tool::Slice => "gp-slice",
        Tool::SliceSelect => "gp-slice-select",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svgs_are_recolourable() {
        assert_eq!(ICONS.len(), 51);
        for (name, bytes) in ICONS {
            let s = std::str::from_utf8(bytes).unwrap_or_default();
            assert!(s.contains("currentColor"), "{name}");
            assert!(s.contains("</svg>"), "{name}");
        }
    }

    #[test]
    fn every_icon_used_by_a_tool_exists() {
        for t in Tool::ALL {
            if let Some(n) = tool_icon(t) {
                assert!(ICONS.iter().any(|(k, _)| *k == n), "{n}");
            }
        }
    }
}
