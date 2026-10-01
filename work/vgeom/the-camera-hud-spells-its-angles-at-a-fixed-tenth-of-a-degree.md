---
id: the-camera-hud-spells-its-angles-at-a-fixed-tenth-of-a-degree
kind: issue
title: the camera readout spells yaw and pitch at a fixed tenth of a degree, beside a distance rendered on the crate's grid
status: open
opened: 2026-09-22
priority: P3
cost: E
---


(VGEOM) Found by the sweep behind `vgeom/render-grid`, whose class is
**a display grid derived from a presentation constraint rather than
from what the model can distinguish**. It is the one production hit
outside the module that unit repaired.

## The finding

`crate::pane::view`'s `view_pane` writes

```rust
ui.label(format!(
    "camera yaw {:.1}°, pitch {:.1}°",
    self.camera.yaw().to_degrees(),
    self.camera.pitch().to_degrees()
));
```

(`crates/viewer/src/pane/view.rs`, the `camera yaw` label) and the
very next label renders the distance band through `camera_mm` →
`crate::props::written_text` → `crate::readout::number`. **One label,
two number policies**: the distance is on the crate's grid and the two
angles are on a tenth of a degree, which is 1.7·10⁻³ rad — a precision
nothing derived. Two orientations a user can reach with the mouse and
tell apart in the picture read as the same two numbers.

## Why the render-grid unit did not take it

Two reasons, and the second is the one that matters.

- `crates/viewer/src/pane/view.rs` was fenced to `FIELD_WIDTH` for
  that lane, and `crates/viewer/src/camera.rs` — where `yaw` and
  `pitch` come from — is `vgeom/camera-band`'s.
- **The fix is not `{:.1}` → `readout::number`.** It is
  `props::written_text(radians, DEG.def())`, which is a notation
  change, and the grid it would land on is the question
  `the-render-grids-cap-is-a-length-and-angles-go-through-it` asks:
  `readout`'s absolute cap is one decade below ε, ε is a LENGTH, and
  nothing says what an angle render owes. Rendering yaw through that
  cap would spell ten decimals of a degree, which is not obviously
  better than one.

So the two rows are one decision and a one-line edit, in that order.

## What a fix has to answer

Whether the camera readout is a QUANTITY the chrome writes in a
notation — in which case it takes `written_text` and the `deg` row of
the closed unit table, and inherits whatever grid the sibling row
settles — or a HUD legend about view state no document holds, in which
case a fixed precision is a legitimate choice and the label should say
so where it makes it. The present code says neither.
