---
id: fixed-precision-length-renders-can-read-as-a-value-they-cannot-be
kind: issue
title: a length the chrome renders at fixed precision can read as a value that length cannot be, outside the δ field
status: closed
opened: 2026-09-12
closed: 2026-09-12
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

## Closed

All four members fixed, through one door rather than four patches, and
the population above held on re-derivation against the merge base
(`03e5154`): the same four sites, at the same lines, each verified by
reading the line rather than by matching the grep again.

**`## The members`' line numbers are as at `03e5154` and are left
there**, because they name a tree in which the defect exists and this
diff is what ends it — a repoint would make them name lines that no
longer hold what the row describes. Where each subject lives now:
`delta_badge` at `crates/viewer/src/frame.rs:1385`,
`FittedDelta::wording` at `crates/viewer/src/scene.rs:918`,
`Bounds::wording`'s `show` at `crates/viewer/src/bounds.rs:235-241`,
and the camera readout at `crates/viewer/src/pane/view.rs:55`, with the
angle beside it at `:49`.

`crates/viewer/src/readout.rs` is the door — `number(value)`, the
shortest decimal spelling that reads back as the value within the
render's own accuracy, and a scientific one when no decimal spelling
does. It is a module rather than a generalisation of `render_mm`
because two of the four sites hold no `DisplayTolerance`, and a rule on
that type would have been written twice: once as the method and once by
hand wherever the type is absent.

- `frame.rs`'s `delta_badge` and `scene.rs`'s `FittedDelta::wording`
  call `DisplayTolerance::render_mm`, which is now the δ-facing door
  onto `readout::number` and carries the millimetre conversion and
  nothing else.
- `bounds.rs`'s `Bounds::wording` calls `readout::number` directly, and
  the module's care about not overclaiming is what decides it: `{:.4}`
  overclaimed at BOTH ends, reading `valid from 0.0000 mm` for a floor
  the search found above zero and `1024.0000` for a reach a doubling
  established to one figure.
- `pane/view.rs`'s camera readout takes the one decision the row asked
  for — WHICH of its numbers are lengths. The three distances go through
  the door; the two angles keep `{:.1}°`, because a yaw of zero is a yaw
  a camera has and a distance of zero is not a distance the camera can
  be at.

**The δ door's second predicate is gone, and that is the argument
rather than a tidy.** `render_mm` used to ask both that a spelling read
back within tolerance AND that it read back as a δ `DisplayTolerance::
new` accepts. For a strictly positive δ the first implies the second, so
the second could not fail — a predicate no input falsifies is
documentation. `no_delta_renders_as_a_number_a_delta_cannot_be` measures
the implication over the whole type, which is where a claim like that
belongs.

**Two residues, both filed rather than disclosed:**

- `the-scientific-arm-rounds-out-of-the-type` — the fallback `{:.3e}`
  rounds past `f64::MAX`, so the module's own rule has one exception.
  Pre-existing in `render_mm`, pinned by a row, and the trade (width
  against truth) is written down where it can be re-taken.
- `a-drag-field-renders-a-length-at-a-precision-its-drag-speed-sets` —
  **the member this row's own sweep could not match.** A precision spec
  in a format string is not the only way to write a fixed precision: an
  `egui::DragValue` derives one from its DRAG SPEED, and a length field
  at `FIELD_DRAG_SPEED` falls back to `{:.3}` over millimetres, which is
  the same spec this family was filed against. The property is *a length
  rendered at a precision fixed independently of the length*; the
  pattern was *a format string that names one*.
