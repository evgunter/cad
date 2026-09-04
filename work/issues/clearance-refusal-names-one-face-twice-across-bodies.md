---
id: clearance-refusal-names-one-face-twice-across-bodies
kind: issue
title: ClearanceRefusal prints one FaceKey twice for a cross-body pair, at two renderings over three raise sites, and has no Display at all
status: open
opened: 2026-09-04
---


Found by FIX's `mate-contradiction-names-one-mate-twice` lane while
sweeping the shape it was fixing one file over — *a pair that can
collapse to one subject* — and widened by that PR's style review.
Reported rather than filed by either, per
`docs/prompts/implementer-discipline.md` §6; placed here by the FIX
orchestrator. SHELL is the natural claimant
(`crates/editor-core/src/clearance.rs` is in its territory glob).

## The defect

`crates/editor-core/src/clearance.rs` guards against naming one face
twice with `x.face != y.face` — but that guard sits under a
`same_body` branch (`:1630`).

`FaceKey` is documented as a key into **a body's** face arena
(`crates/topo/src/entity.rs:155`), so two different bodies routinely
carry the same key value. For a **cross-body** pair, the guard does not
apply and the refusal prints *"between faces F and F"* about two
genuinely different faces.

That is the same class the FIX unit closed at
`MateFault::Contradictory` ("mates 6 and 6 cannot both hold"): a
diagnostic naming one subject twice, which reads to a user as an
indexing bug and hides the shape the code already knows.

## Wider than first reported

The lane reported one rendering. The review found:

- **Three `PoisonEnclosure` raise sites** — `:1646`, `:1730`, `:2567`.
- **Two renderings, not one.** The site the lane cited (`:967`) is
  correct, and `detail()` at `:629` prints `{a:?}/{b:?}`, so the
  cross-body case prints one key twice there too.
- **`ClearanceRefusal` has no `Display` at all**, which is the
  standing-convention half: the layer that raised a failure names it,
  and a consumer's only honest option without a `Display` is a debug
  rendering.

So this is a class of ≥2 renderings over 3 sites, not an instance.

## What the fix probably is, not decided here

Two halves, and they are separable:

1. **The guard**: `x.face != y.face` is the wrong question for a
   cross-body pair. Either the comparison carries the body, or the
   rendering names which body each face belongs to — the second is
   likely better, since a reader given two bare arena keys cannot tell
   them apart even when they differ.
2. **The `Display`**: `ClearanceRefusal` owes one under the same rule
   that produced `work/fix/error-types-with-no-display-class` — closed
   2026-09-04, whose durable finding was that the census which produced
   its list was keyed on the bare type name and missed types spelled
   `impl core::fmt::Display for`. Worth re-checking this one by reading
   rather than by grep, for that reason.

## Home

`work/issues/` — `crates/editor-core/src/clearance.rs` is SHELL's
territory glob. Re-home by header edit.
