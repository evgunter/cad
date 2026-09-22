---
id: viewer-substituted-value-class-is-crate-wide
kind: issue
title: The substituted-value class spans crates/viewer; one file has been swept
status: open
opened: 2026-09-15
priority: P1
cost: D
---


## The class

*A value the function did not compute, returned in the shape of one it
did* — a NaN or a failed conversion floored, clamped or substituted
into a plausible-looking number, so the caller cannot tell a real
answer from a stand-in. The project is fail-loud (`CLAUDE.md`,
`docs/DESIGN.md`).

**This row exists because one file has been swept and the class is
wider than one file.** `crates/viewer/src/datums.rs` is closed — five
members fixed under `inclusive-rule-range-draws-a-line-on-a-nan-count`
and `metres-per-pixel-swallows-a-nan-depth` — and that is an
INSTANCE, not a population. The next lane should start from the list
below rather than from a single site.

**The standard is not "never substitute."** `triangle_normal` and
`BoundsProbe::new` both substitute and are accepted, because each
names its consumer and weighs the two failures at the site;
`datums.rs`'s `grid_pitch` rustdoc is the model of doing that well. A
site may close with an argued citation.

## Known members, all verified by reading the site

**Already on a slate:**

- `work/chrome/gpu-index-counts-substitute-u32-max` — `gpu.rs`'s two
  `u32::try_from(…).unwrap_or(u32::MAX)` (`:562`, `:960`). Open.
- `work/chrome/degenerate-triangle-normal-is-substituted` — the
  accepted-substitution end of the same class. Open.
- `work/chrome/max-grid-lines-truncates-a-ruling-and-calls-it-one` —
  the one `datums.rs` member the sweep left standing. **Closed** by PR
  3018; what it cost, and the lesson that is not about grids, is the
  section below.

**Not on any slate, found by the greps below:**

| site | shape |
| --- | --- |
| `sketch.rs`, arc point count | `((theta.abs() / step).ceil() as usize).clamp(1, MAX_ARC_POINTS)` — mechanically `datums.rs`'s `rule_patch` cast with a different floor: a NaN casts to `0` and the clamp lifts it to ONE arc point |
| `sketch.rs`, the bounds accumulator | `lo[axis] = lo[axis].min(point[axis])` / `hi[axis] = hi[axis].max(point[axis])` — `f64::min`/`max` answer with the other operand against a NaN, so a point that is not a point is silently absent from the bounds it should poison |
| `scene.rs`, the diagonal | `if diagonal.is_finite() { diagonal } else { 0.0 }` — a bounding-box diagonal that overflowed is returned as a scene of no size, which is a number this function did not compute |
| `scene.rs`, the delta solve | `(constant / requested.get()) as usize`, under an explicit `#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]` — the allow says the truncation was seen; it does not say a NaN reads as zero triangles |
| `bounds.rs`, the integral seed | `seed: if integral { seed.max(1.0) } else { seed }` |
| `camera.rs`, the dolly floor | `(self.distance - self.scene_radius).max(floor)` |
| `app.rs`, the features share | `((wanted + FEATURES_SLACK) / stack).clamp(0.0, FEATURES_SHARE_CAP)` |

Dispositions are UNDECIDED for every row of the second table: each
needs the site read and the two failures weighed, and some will close
by argument. Two `clamp(0.0, 1.0)` sites in `pickindex.rs` and one in
`theme.rs` are parametric or channel clamps where the bound is the
definition, and are listed here only so a later sweep does not
re-derive them as candidates.

## The greps, and what they cannot match

Run over `crates/viewer/src`:

1. `\.max\(|\.min\(|\.clamp\(` filtered off integer contexts;
2. float→int casts — `as usize|u32|u64|i64` preceded by
   `ceil|floor|round|abs|sqrt|powi|trunc` or by a division or product;
3. `saturating_|checked_|unwrap_or\(`;
4. `is_finite|is_nan|is_infinite|NAN|INFINITY|MIN_POSITIVE`;
5. `} else {` with context, for a literal in the else arm.

**Blind spots, which are the reason this is a starting population and
not a census:**

- **`f64::max`/`min`'s NaN-preference is invisible in the source
  text.** Grep 1 finds the call; nothing says which operand wins. Two
  of the six rows above are that and nothing else.
- **A substitution spelled as plain arithmetic** — a division that
  yields `inf`, a norm that overflows — has no idiom to match.
- **Substituted STATE rather than values**: a mark silently not
  appended, a row silently not badged.
- **A substitution spelled as a type default** (`unwrap_or_default`,
  `Default::default()`, `#[derive(Default)]`).
- **Reachability.** No grep says whether a caller already guards. In
  `datums.rs` exactly that decided two of the five dispositions, and
  it cost a source read per site.
- **Cross-crate arrivals**: a public struct's field substituted
  before it reaches `viewer` at all.
- **Test fixtures read as production.** `scene.rs:1397`'s
  `(constant / delta) as usize` is inside `#[cfg(test)] mod tests` and
  is NOT a member; it is named here so the next sweep does not file
  it.

## Fence

`crates/viewer/src/**` — CHROME's and VIEW's by the territories
table. `sketch.rs`, `scene.rs`, `camera.rs`, `app.rs` and `bounds.rs`
are VIEW's this week, so whoever takes this row draws the fence with
VIEW first; the `datums.rs` and `gpu.rs` ground is CHROME's.

## What the `MAX_GRID_LINES` member cost, and who else is carrying it

`max-grid-lines-truncates-a-ruling-and-calls-it-one` closed by
shrinking the patch to what the cap rules rather than truncating the
line list. **The lesson is not about grids.** The reason the previous
sweep read that site as a backstop and left it standing was one
sentence of its own rustdoc — *"Not a budget the design expects to
spend"* — followed by an arithmetic estimate nobody had measured.
Measured, the cap is crossed from an ordinary orbit seat on an 8K
window, and the estimate in the doc was taken at a different
constant than the one in the code.

**`sketch.rs`'s arc point count, in the table above, carries the
identical argument**: *"A cap, not a budget: this only stops a radius
large enough to…"*. That sentence is unmeasured in exactly the same
way, and it is what will make a reader skip the site again. A lane
taking that row should drive the cap before it accepts the
reachability claim, whatever it then decides about the substitution.

This is §5's *"an invariant discovered by a bugfix otherwise protects
only the code that already knew"*, written down where the next reader
of the class will see it.
