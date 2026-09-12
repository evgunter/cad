---
id: delta-field-renders-a-sub-micrometre-delta-as-zero
kind: issue
title: the δ field renders a δ below 500 nm as 0.000 mm, and an edit-then-undo commits that render
status: open
opened: 2026-09-11
---


(VIEW) Residue of `seeded-draft-is-the-commit-path-and-does-not-round-trip`,
disclosed and filed in the PR that closed it.

## What is left

`crates/viewer/src/pane/view.rs`'s `delta_field` no longer seeds its
draft with the rendered δ, so a field nobody typed into commits
nothing. The render itself is unchanged and is still `{:.3}` over
millimetres, so:

- **It is wrong on screen below 500 nm.** δ = 0.4 µm reads `0.000` in
  the View pane's field, which is a number no δ may be
  (`crates/viewer/src/scene.rs:75`, `DisplayTolerance::new`, refuses
  `0.0`). The field is the one place a user reads the δ in force as a
  number they can act on.
- **And one path still commits it.** The render is the text an edit
  starts from, so a user who types a character into the field and
  deletes it again leaves the render in the box as their own draft,
  and that draft commits: δ = 1.6 µm becomes 2 µm. Narrower than the
  focus-and-leave the parent closed — it takes two keystrokes that
  cancel out — but it is the same quantisation, by the same render.

## Why it was not fixed with the parent

The parent's defect is that the SEED was the commit path; this is that
the RENDER is lossy, which is the family the parent files under
"siblings — same shape, prose only, lower stakes"
(`crates/viewer/src/bounds.rs:216-222`, `crates/viewer/src/frame.rs:1376`,
`crates/viewer/src/scene.rs:878`). Verified while closing the parent:
all three of those sink into `ui.weak` or a `Badge` and nothing parses
them back, so they are render-only as the parent says. This row is the
fourth member and the only one an edit can reach.

## What a fix cannot be

Not more decimals, and not the exact spelling either. A round-trip
seed does not close it: seeding the shortest round-tripping spelling of
`δ · 1e3` and parsing it back through `· 1e-3` returns a different
`f64` for **4,155 of 28,600** sampled δ in [1e-12, 1e-1] m (14.5%),
every one of them by exactly 1 ULP, because the lossy step is the unit
conversion and not the format. A preimage spelling exists for 97.7% of
those (within 1 ULP of the naive product) and for **669 of 28,600**
(2.3%) there is no `f64` millimetre value at all whose product with
`1e-3` is the δ in force. So the property to aim at is a render that
never reads as a number δ cannot be — not a render that round-trips.
