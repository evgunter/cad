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

Every value-shaped member over `crates/viewer/src` is fixed, closed by
argument at its site, or on a slate. The per-hit list, with the greps
and their dispositions, is in the `vgeom/substitution-sweep` PR body.
The greps:

1. `\.max\(|\.min\(|\.clamp\(`, read for NaN preference and for which
   operand wins;
2. float→int casts (`as usize|u32|u64|i64|…`);
3. `saturating_|checked_|unwrap_or\(`;
4. `is_finite|is_nan|is_infinite|NAN|INFINITY|MIN_POSITIVE`, and
   `} else {` with a literal arm;
5. `fold`/`reduce` over `f64::max`/`f64::min`;
6. `unwrap_or_default|Default::default()`;
7. a division by a measured length, norm, count or extent (the
   plain-arithmetic `inf` shape).

## What remains

Two of the class's blind spots have no idiom to grep, and nothing
above searched them:

- **Substituted STATE rather than values**: a mark silently not
  appended, a row silently not badged, a collection defaulted to empty
  where the read refused. Pass 6's hits were read at the line and are
  collection defaults for an absent optional; whether each absence is
  legitimate turns on its caller, which that pass did not follow.
- **Cross-crate arrivals**: a public struct's field substituted before
  it reaches `viewer` at all (an `editor-core` or kernel door answering
  a stand-in the viewer then draws faithfully). A sweep of this needs
  the viewer's inbound doors listed first, and it lands on other
  programs' ground.

Test fixtures read as production are not members: `#[cfg(test)]`
modules were excluded from every pass.
