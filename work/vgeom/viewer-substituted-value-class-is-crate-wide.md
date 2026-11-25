---
id: viewer-substituted-value-class-is-crate-wide
kind: issue
title: The substituted-value class in crates/viewer is swept for values; substituted state and cross-crate arrivals are not
status: open
opened: 2026-09-15
priority: P3
cost: M
---


## The class

*A value the function did not compute, returned in the shape of one it
did* — a NaN or a failed conversion floored, clamped or substituted
into a plausible-looking number, so the caller cannot tell a real
answer from a stand-in. The project is fail-loud (`CLAUDE.md`,
`docs/DESIGN.md`).

**The standard is not "never substitute."** A site may close with an
argued citation that names its consumer and weighs the two failures:
`sketch.rs`'s `MAX_ARC_POINTS` and `scene.rs`'s `triangle_normal` are
the models, and the door-answers-Option/Result shape of
`vgeom/refusal-floor` (#3000) is the fix shape.

## What has been swept

Each value-shaped hit over `crates/viewer/src` is fixed, closed by an
argument at its site, filed on a slate, or listed under *What remains*
below. The per-hit list is in the `vgeom/substitution-sweep` PR body
(PR 7764). The greps:

1. `\.max\(|\.min\(|\.clamp\(`, read for NaN preference and for which
   operand wins;
2. float→int casts (`as usize|u32|u64|i64|…`);
3. `saturating_|checked_|unwrap_or\(`;
4. `is_finite|is_nan|is_infinite|NAN|INFINITY|MIN_POSITIVE`, and
   `} else {` with a literal arm;
5. `fold`/`reduce` over `f64::max`/`f64::min`;
6. `unwrap_or_default()`;
7. a division by a measured length, norm, count or extent (the
   plain-arithmetic `inf` shape), and `recip()`/`1.0 /`.

## What remains

**Accepted by reachability only, with no argument at the site.** Each
was read and judged unreachable or harmless, but the judgement lives in
a PR body, which is not where the next reader will look. Each needs its
argument at the site, or a fix:

- `combine.rs` `width_along`'s NaN-dropping fold. Not proved: mesh
  positions carry no finiteness guarantee. The alignment with
  `scene::extent` is `work/chrome/a-measures-scale-probe-drops-a-nan-point-the-fits-extent-refuses.md`.
- `pane/create.rs` `profile_x_extent`'s reduce, over `validated.loops()`,
  on the ground that validation cannot certify a non-finite vertex.
- `props.rs` `Members::span`'s fold, over `Ok` member values, on the
  ground that the document's literal door refuses a non-finite one.
- `sketch.rs` `trace_preview`'s `fold(0.0, f64::max)`, on the ground
  that `editor_core::trace::trace_marks` filters a NaN crossing by
  `along <= span`.
- `readout.rs` `widest_render`'s fold over glyph advances, on the
  ground that font metrics are finite.

**Blind spots no grep above reaches:**

- **Reachability.** No grep says whether a caller already guards. The
  list above is exactly that kind of disposition, and in the earlier
  `datums.rs` sweep it decided two of five sites. It costs a source
  read per site.
- **Type defaults spelled `Default::default()` or
  `#[derive(Default)]`.** Pass 6 read only `unwrap_or_default()`. A
  `Default` field on a numeric struct substitutes a zero silently, and
  those hits were not read.
- **Substituted STATE rather than values**: a mark silently not
  appended, a row silently not badged, a collection defaulted to empty
  where the read refused. Pass 6's hits were read at the line and are
  collection defaults for an absent optional. Whether each absence is
  legitimate turns on its caller, which that pass did not follow.
- **Cross-crate arrivals**: a public struct's field substituted before
  it reaches `viewer` at all (an `editor-core` or kernel door answering
  a stand-in the viewer then draws faithfully). A sweep of this needs
  the viewer's inbound doors listed first, and it lands on other
  programs' ground.

Test fixtures read as production are not members: `#[cfg(test)]`
modules were excluded from every pass.

## An unmeasured cap rationale is where the next instance hides

`datums.rs`'s `MAX_GRID_LINES` was left standing by an earlier sweep
on one sentence of its own rustdoc, *"Not a budget the design expects
to spend"*, followed by an estimate nobody had measured. Measured, an
ordinary orbit seat on an 8K window crossed it. `sketch.rs`'s
`MAX_ARC_POINTS` carried the same kind of sentence, and when driven it
was crossed by any circle of radius past 1.33 m at the opening δ. A cap
whose rustdoc says it is never reached is a claim to measure before
accepting, whatever the site is.
