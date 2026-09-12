---
id: fixed-precision-length-renders-can-read-as-a-value-they-cannot-be
kind: issue
title: a length the chrome renders at fixed precision can read as a value that length cannot be, outside the δ field
status: open
opened: 2026-09-12
---


(VIEW) The render-only siblings of
`delta-field-renders-a-sub-micrometre-delta-as-zero`, given a file of
their own now that the fix for that row exists and they do not use it.
Reported as siblings by PR 2366's lane, verified at the close of
`seeded-draft-is-the-commit-path-and-does-not-round-trip`, and
re-verified here against this tree — where the sweep also found one the
parent row did not name. The list below is the population of record; no
count of it is written anywhere in this file.

## The members

Found by sweeping `crates/viewer/src` for a precision spec in a format
string (`{…:.N}`), which is how all but one of these were written. What
that pattern cannot match: a render with no precision spec at all
(`{}`, `to_string`, `{:e}`), a precision passed as a variable, and a
rounding done by hand before the format — checked, and the only
variable precision in the crate is the new render's own search, the
only `round()` is `theme.rs:540` on a colour byte, and there is no
`{:e}` outside the new render. It also cannot see renders outside this
crate, which are other programs' ground.

- `crates/viewer/src/frame.rs:1376` — `delta_badge`,
  `format!("δ {:.3} mm chosen", fitted.delta.get() * 1.0e3)`. This one
  is the sharpest of them: the δ it announces is the budget's own
  choice, `constant / TRIANGLE_BUDGET`, so a body whose cost constant
  is under a triangle·millimetre gets a badge reading *"δ 0.000 mm
  chosen"* — a number `DisplayTolerance::new` refuses, in the sentence
  whose whole job is to say which δ the picture is at.
- `crates/viewer/src/scene.rs:946` — `FittedDelta::wording`, the same
  δ and the requested one at `{:.3}` in the badge's detail. (Cited as
  `scene.rs:878` when the parent row filed it; the fix for the field
  moved it.)
- `crates/viewer/src/bounds.rs:216-221` — `Bounds::wording`'s
  `{written:.4}`, a probed bound divided by its display unit's factor.
  A bound under 0.1 µm written in mm reads `0.0000 mm`.
- `crates/viewer/src/pane/view.rs:43` — the camera readout,
  `"distance {:.1} mm (band {:.1}–{:.1})"`. The one the parent row did
  not name, and the one this sweep added: the same shape over a
  different length. `Camera::min_distance` is `scene_radius * 0.05`
  (`crates/viewer/src/camera.rs:531-533`), so every part under about a
  millimetre reads `band 0.0–…` — a distance the camera refuses, since
  a camera at zero is inside the model. It sits in the same function as
  the δ field; what keeps it out of that fix is that the line above it
  renders an ANGLE at the same precision (`{:.1}°`, `view.rs:38`) and
  `0.0°` is a yaw a camera really has, so the readout wants one decision
  about which of its numbers are lengths rather than a per-number patch.

## What the field's fix does and does not give them

`DisplayTolerance::render_mm` (`crates/viewer/src/scene.rs`) is the δ
render that cannot read as a number δ cannot be: the shortest decimal
spelling that fits ten characters and reads back, through the
millimetre conversion, as a δ the door accepts within four significant
figures, and a scientific spelling when no decimal one does.

- The first two sites hold a `DisplayTolerance` and can call it
  directly. What stops that being this unit's work is not the call: it
  is that both sites render δ inside a sentence, and a sentence
  carrying `4.000e-4` where it used to carry `0.000` is a wording
  decision about a badge and a status line, not a field's width.
  `crates/viewer/tests/display_budget.rs`'s
  `a_coarsened_picture_says_so_in_both_numbers` builds its needles with
  `format!("{:.3}", …)` and would want re-pointing at the same time.
- `Bounds::wording` cannot use it at all: its subject is a probed
  length in the user's own display unit, not a δ in millimetres, so it
  needs the same RULE — prefer a spelling that reads back as the value
  it renders — at its own precision and unit. A shared helper over
  `(value, precision)` is the shape if they land together; whether
  the probe bracket wants one at all is its module's call, since its
  own doc is careful that a bracket reads as *the furthest value found
  valid* rather than as an exact number.

Not swept with the field because the field was the only member an edit
could reach, and because a wording change to a badge, a status line and
a camera readout is a different review from a control's arithmetic.
