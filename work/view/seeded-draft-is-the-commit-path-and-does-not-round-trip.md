---
id: seeded-draft-is-the-commit-path-and-does-not-round-trip
kind: issue
title: the delta field seeds its draft with {:.3}, and that draft IS the commit path: focus-and-leave quantises delta to the nearest micrometre, or refuses a value the user never typed
status: open
opened: 2026-09-11
refs: [2366]
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
