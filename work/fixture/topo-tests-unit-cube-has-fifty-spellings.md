---
id: topo-tests-unit-cube-has-fifty-spellings
kind: issue
title: The unit cube is spelled 49 times across 19 suites, and three conventions for naming a nullary brick wrapper now coexist
status: open
opened: 2026-09-16
priority: P4
cost: E
---

## Finding

- **Where**: `crates/topo/tests/` — 19 suites, led by `review_m3_pr4.rs`
  (6), `m3_pr5_boolean_ops.rs` (5), `m5_pr8_bvh_diff.rs` (5),
  `m3_pr4_boolean.rs` (4), `m3_pr6_tier3prime.rs` (4).
- **Importance**: low-medium
- **Confidence**: sure about the count at the citation below; the
  remedy is a taste decision
- **Raised by**: the style review of the `dup-brick` lane's PR (S-DUP),
  2026-09-16

After that PR, `brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))` — the unit
cube, the single most common fixture in the crate — appears **49 times
across 19 files**:

```
grep -rohE 'brick(::<[A-Za-z0-9_]+>)?\(\(0\.0, 1\.0\), \(0\.0, 1\.0\), \(0\.0, 1\.0\)\)' \
  crates/topo/tests/*.rs | wc -l
```

The `dup-brick` PR argued, for `geom_origin_rows.rs`, that inlining the
call at thirteen sites in one file would mint a thirteen-fold spelling
of the unit cube and that a nullary fixture was therefore right. The
argument is correct and it is **already true sixteen times over
elsewhere** — the lane recognised the class, fixed the instance it was
standing on, and filed nothing. This is that filing.

**The decision the row owes**: whether `common` should carry a nullary
`unit_cube()` (or whatever it is called) over `brick`, and if so what
happens to `geometric_cube`, which is the OTHER unit cube in the same
file and is a different body (it stops before
`describe_as_intersections`). Naming is the whole difficulty: a
`unit_cube` next to a `geometric_cube` that is not it would be worse
than 49 spellings.

## The same question in smaller form: three wrapper conventions

That PR also left three different conventions for "a named nullary or
near-nullary wrapper over `brick`", each argued separately in its body
and individually defensible:

- `geom_origin_rows.rs::unit_brick` — **renamed** so it cannot be
  mistaken for the shared door;
- `review_f7_pole_r1_probes.rs::distant_brick` — **kept its name**
  because its doc explains why the body is far away and the name earns
  the wrapper;
- `corner_table.rs::top` / `::leg` — **domain names**, where the wrapper
  is about the fixture's role and not about the box.

Collectively that is three rules. Whichever way the nullary-unit-cube
question goes should settle this too.
