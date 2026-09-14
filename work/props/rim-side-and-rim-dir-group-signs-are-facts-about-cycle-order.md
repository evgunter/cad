---
id: rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order
kind: issue
title: props_rim_side and props_rim_dir_group read whichever rim the loop walk from Cycle::first meets first, so their recorded signs are facts about cycle order rather than about the face
status: open
opened: 2026-09-14
---

Filed by the TOPO revert-wrap fix pass (PR 2573, 2026-09-14), on this
slate because `crates/geom-brep/src/props/*` is PROPS's glob.

## Finding

Two of the closed-form props predicates are relative to whichever rim
the caller's loop walk hands over FIRST, so their recorded signs are
facts about cycle order rather than about the face:

- `props_rim_side` — `linear_rim_side`'s inner `side`
  (`crates/geom-brep/src/props/curved.rs`) reads
  `b.rims.first()` and classifies `lo + hi − 2·level` on THAT rim;
- `props_rim_dir_group` — `du_of_rims` seeds its groups from the first
  rim and classifies every later rim's `d_u − g.1` against it.

Both are compensated downstream (`Positive ⇒ rim.d_u_sign`,
`Negative ⇒ .flip()`), so the flux and the readings do not depend on
which rim comes first. The recorded verdicts do: measured on
`voided_rod` (`crates/sweep/tests/shell_census_is_thread_count_invariant.rs`),
both read `Negative ×2` when the reverted rod's wall loops are anchored
where the forward walk left them and `Positive ×2` once
`Body::revert` moves every loop's anchor to its source predecessor
(PR 2573) — same face, same rims, opposite recorded sign. The rim the
walk meets first is the anchor's, and the anchor is a topological
convention (`topo`'s `LoopBoundary::Cycle`), not a property of the
face.

Why it matters here: the k_stats verdict channel is this program's
(`crates/geom-core/src/k_stats.rs`), and k-lint's population and every
golden over recorded verdicts move under a pure re-anchoring of a loop
because of these two predicates alone — the other eleven the census
records on that body are per-rim, per-meridian or per-face facts and
count the same whichever rim comes first
(`voided_rods_verdicts_as_a_sorted_multiset`, the row PR 2573 added,
pins the counts).

Closing shape, undecided: pick the reference rim by a property of the
face rather than by cycle order (the rim at the LOWER level, say, so
`props_rim_side`'s margin is always `hi − lo`-signed and the recorded
sign is the face's), or record these two under a name that says they
are relative reads. The related open row on the same function is
`rim-stores-its-traversal-direction-twice` (`du_of_rims`' direction
compare through the funnel).
