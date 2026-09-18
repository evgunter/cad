---
id: seeded-draft-is-the-commit-path-and-does-not-round-trip
kind: issue
title: the delta field seeds its draft with {:.3}, and that draft IS the commit path: focus-and-leave quantises delta to the nearest micrometre, or refuses a value the user never typed
status: closed
opened: 2026-09-11
refs: [2366]
closed: 2026-09-11
---



(FIX orchestrator) Reported by the `path-error-numbers-below-1e-9-render-as-zero`
lane (PR 2366) from its sweep for fixed-precision renderers, outside
its fence, per `docs/prompts/implementer-discipline.md` §6. Verified
and re-framed here before filing — the lane's reading was right about
the site and understated the class.

## The defect

`crates/viewer/src/pane/view.rs:74-100`, `delta_ui`.

```rust
let in_force = self.delta.get() * 1.0e3;                       // mm
let text = self.drafts.delta_mm.get_or_insert_with(|| format!("{in_force:.3}"));
// …
if field.lost_focus() && let Some(typed) = self.drafts.delta_mm.take() {
    match typed.trim().parse::<f64>() {
        Ok(mm) => *self.delta_request = Some(mm * 1.0e-3),
        …
```

The string the field is **seeded** with is the same string that is
**parsed back and committed** on `lost_focus`. So the seed has to
round-trip, and `{:.3}` over millimetres does not. The function's own
header is careful that there is *"one commit path, not two"* and that
a half-typed draft never reaches the tessellator — both true, and
neither is this. The hazard is not the typed value. It is the value
**nobody typed**.

## Two failures, and the second is the one the report missed

**1. Below 500 nm — a refusal for a value the user never entered.**
δ = 0.0004 mm seeds `"0.000"`. Focus the field, click away without
typing: `0.0` is committed, and `DisplayTolerance::new`
(`crates/viewer/src/scene.rs:75`, `delta.is_finite() && delta > 0.0`)
refuses it as `SceneError::InvalidDisplayTolerance`. The user is told
their δ is invalid, about a number the UI put in the box.

**2. At every other magnitude — silent quantisation.** δ = 0.0016 mm
seeds `"0.002"`. Focus, click away: δ becomes 2 µm. **No refusal, no
notice, nothing red** — the value simply changes by up to 500 nm
because a field was focused and left. This is the worse half: failure
1 is loud and failure 2 is silent, and failure 2 does not depend on
the sub-micrometre band at all.

## The class, stated so the fix is not a decimal count

**A field whose seeded display text is also its commit path must seed
a spelling that round-trips exactly.** Adding decimals to `{:.3}`
narrows the band and closes nothing — any fixed precision quantises
something. The shapes that actually close it: seed the exact spelling
and shorten only for display, or keep the draft `None` until the user
types (so `lost_focus` on an untouched field commits nothing), or
commit only on a real edit rather than on any focus loss.

**Prefer the door that makes "untouched" unrepresentable** over one
that makes the seed more precise. That is the lane's own reading and I
endorse it: this is a round-trip defect, not a formatting one.

## Siblings — same shape, prose only, lower stakes

Reported by the same lane, not verified here beyond the citation:

- `crates/viewer/src/bounds.rs:216-222` — `Bounds::wording`'s
  `{written:.4}`; a bound under 0.1 µm written in mm reads
  `0.0000 mm`, in the one place whose own doc says *"the probe's
  limits become a sentence a user reads"*.
- `crates/viewer/src/frame.rs:1376` and `crates/viewer/src/scene.rs:878`
  — δ as `{:.3}` mm in prose.

These three render only; none is a commit path, which is exactly what
separates them from the row above.

## Home

`crates/viewer/src/pane/view.rs` is VIEW's. Filed straight onto this
slate rather than `work/issues/` per `work/README.md` (*"When the
owning program is clear, file the item straight onto that program's
slate"*). Checked for a duplicate before filing: no open VIEW row
mentions `delta_ui` or `delta_mm`, and the four rows citing
`DisplayTolerance` are about where the TYPE lives
(`index-seam-vocabulary-sits-in-the-wrong-module`,
`the-picture-key-never-became-a-type`, `pick-index-built-on-ui-thread`)
or about a δ **already** refused by it
(`status-line-writers-bypass-the-ranking:50`) — none is this.

FIX does not claim this row; re-home or re-cut it freely.

## Closed

Fixed by making the draft mean what
`crates/viewer/src/drafts.rs:33-39` already says it means — *the View
pane's δ field, in millimetres AS TYPED*. A keystroke is now the only
thing that makes `delta_mm` `Some`; a field that was focused and left
holds no draft, so it commits nothing and refuses nothing. The two
failures above are held by rows in `crates/viewer/src/pane/view.rs`'s
own test module, which drives the field's focus lifecycle through a
headless `egui::Context`: seeding the draft unconditionally again makes
them fail with `Some(0.0)` and `Some(2e-6)`, the two numbers this item
names.

The `{:.3}` render stays, and is now only a render. What it still
costs — a δ below 500 nm reading `0.000` in the field, and an
edit-then-undo committing that reading — is filed as
`delta-field-renders-a-sub-micrometre-delta-as-zero`, on this slate,
with the arithmetic for why a more precise seed is not the answer.

The three siblings were checked rather than taken on the citation:
`Bounds::wording` reaches `pane/properties.rs:702` and `:756` through
`ui.weak`, `frame::delta_badge` builds a `Badge`, and
`FittedDelta::wording` is that badge's detail. Nothing parses any of
them back. Render-only, as this item says.
