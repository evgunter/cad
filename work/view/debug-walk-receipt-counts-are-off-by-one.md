---
id: debug-walk-receipt-counts-are-off-by-one
kind: issue
title: the out-of-fence hit list says seven and lists eight, and the citation receipt says 16 with 15 resolving where 16 occurrences carry 14 resolutions
status: open
opened: 2026-09-06
refs: [2093]
---



Found by the style review of #2093. Two of that PR's own receipts
disagree with the lists printed beside them.

## The out-of-fence hit list says seven and has eight rows

The PR body: *"**Seven outside it**, every one the same shape"* — and
the table under that sentence has **eight** rows. The item file
(`## Sweep, and what the pattern could not match`) and
`work/view/log.md`'s entry both repeat "seven" over the same eight.

`rg -nE 'impl[^=]*\bDebug\b for' crates/ | rg -v '^crates/viewer/'`
gives eight hits at this branch's head:

    crates/topo/src/param_source.rs:84
    crates/geom-core/src/spline/hull.rs:262
    crates/geom-core/src/spline/hull.rs:331
    crates/geom-core/src/spline/hull.rs:367
    crates/geom-core/src/spline/hull.rs:396
    crates/geom-core/src/spline/knots.rs:340
    crates/geom/src/curves/nurbs.rs:221
    crates/geom/src/surfaces/nurbs.rs:164

The concrete count is **nine**: `crates/geom/src/curves/nurbs.rs:221` is inside
`nurbs_curve!`, invoked twice (`crates/geom/src/curves/nurbs.rs:1492`
and `:1493`) for `CurveWindow2` and `CurveWindow3`. The PR names that
row "the macro-generated NURBS curve windows", plural, so the fact is
known and the arithmetic still lands on seven.

**And one of the eight is not the shape the sentence claims.**
`crates/topo/src/param_source.rs:82-87` is
`pub struct ParamSource(Arc<[u8]>)` and its `Debug` is
`write!(f, "ParamSource(<{} bytes>)", self.0.len())` — no
`debug_struct`, no `finish()`, and no field census to fall behind. The
PR hedges it in a parenthesis ("a newtype — the weakest of the set")
while the sentence above it says "every one ... ends in `finish()`".

## The citation receipt says 16 and 15 and cannot mean both

The stated rule: *"every coordinate of the form `path:N`, `path:N-M`,
or a bare `:N` / `:N-M` inheriting the path named immediately before
it, appearing in the item file or in the lines this branch adds to
`work/view/log.md` and `crates/viewer/README.md`."* Run over those
three inputs it produces **16 occurrences** of **15 distinct
coordinates** — the failing one,
`crates/viewer/src/session.rs:1663-1674`, appears twice: once in the
item's `## The duplication` and again in `## Closed`, where the PR
quotes it in order to say it does not resolve.

So: 16 occurrences, **14** resolve. Or 15 distinct, **14** resolve.
"16; 15 resolve" is neither. (The added lines of `log.md` and
`README.md` contribute nothing — every coordinate is in the item.)

Each of the 15 was read with `sed -n Np` at head; the merge-base
coordinate `session.rs:1879-1891` was read against `237570479`. Only
`session.rs:1663-1674` fails, and the decision to leave it as filed is
right — it is a sentence about a defect that no longer exists.

## The rule's own blind spot is not stated

`work/view/log.md:1833` is the coordinate the PR's sibling-check
paragraph turns on, and it reaches the log as prose — *"The entry at
line 1833 of this log"* — so the stated rule does not match it and it
is outside the 16. A receipt that names its enumeration rule but not
what the rule cannot see is the same unverified-negative shape as a
sweep with an unstated blind spot.
