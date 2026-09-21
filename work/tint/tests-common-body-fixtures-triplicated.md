---
id: tests-common-body-fixtures-triplicated
kind: issue
title: Three crates' tests/common each carry the same six body fixtures; three pairs are byte-identical and three have drifted
status: open
opened: 2026-09-15
priority: P3
cost: E
---

## Finding

- **Where**: `crates/mesh/tests/common/mod.rs`, `crates/stl/tests/common/mod.rs`,
  `crates/step-export/tests/common/mod.rs`
- **Importance**: medium
- **Confidence**: sure — the identical/drifted split below was measured, not grepped
- **Raised by**: the `S52` lane (SUITE), 2026-09-15, on a census of the
  `tests/common/` trees supplied by a sibling review lane

Six body fixtures are declared in more than one of the three trees. **The
split matters more than the count**, because it is the drift the duplication
class predicts, caught midway:

| fixture | mesh | stl | step-export | |
| --- | --- | --- | --- | --- |
| `ball` | ✓ | ✓ | ✓ | mesh and stl byte-identical; step-export differs |
| `donut` | ✓ | ✓ | ✓ | mesh and stl byte-identical; step-export differs |
| `l_prism` | ✓ | ✓ | — | byte-identical |
| `cone` | ✓ | ✓ | ✓ | **all three differ** |
| `washer` | ✓ | ✓ | ✓ | **all three differ** |
| `holed_prism` | ✓ | ✓ | — | **differ** |

So three of the six have already drifted, and nobody chose the drift: each
tree's copy was edited for its own suite. Whoever takes this reconciles rather
than merges — for each of `cone`, `washer` and `holed_prism`, decide which
body is right and say what moved, because at least two suites are metering a
shape they did not mean to be metering.

**The home exists and is reachable from all three crates.**
`crates/sweep/src/test_support.rs` is generic over the scalar and already
carries `cube`, `brick`, `prism`, `prism_at`, `prism_on`, `extruded`, `dome`,
`waisted`, `ball_poled_z`, `lantern`, `spool` and `sphere_zone`; `mesh`,
`step-export` and `stl` each already carry
`sweep = { path = "../sweep", features = ["test-support"] }` in
`[dev-dependencies]`, so joining costs a `use` and no manifest change. `ball`,
`donut`, `cone` and `washer` are revolves and sit beside `dome`; `l_prism`
and `holed_prism` are one and two loops through `extruded`.

The same census names `axis_y` ×2, `validated` ×2 and `p2` ×3 across the same
trees. Those are three-line vocabulary rather than fixtures and the routing
rule (`crates/sweep/src/test_support.rs`'s header) argues they stay local —
worth deciding explicitly rather than sweeping in.

Related, on this slate: `work/tint/sweep-boolean-suite-brick-and-prism-copies.md`,
whose `brick` half `S52` closed and whose `prism(pts, h)` half is still
open against the same home.

## The fixture is three trees; its PROFILE is thirty sites (2026-09-18, `dup/one-prism-builder`)

`l_prism`'s six corners — `(0,0) (2,0) (2,1) (1,1) (1,2) (0,2)`, the
L-shaped hexagon of area 3 — are written out **30 times across 28
files**, measured at `7730008d`:

```
crates/editor-core/tests/  m10_5_r1_probes_interval.rs  m10_6_r1_probes_interval.rs
                           perf12_census_bvh_diff.rs
crates/mesh/tests/         common/mod.rs (the l_prism fixture)  r1_probes_issue303.rs
crates/profile/tests/      common/mod.rs
crates/stl/tests/          common/mod.rs (the l_prism fixture)
crates/sweep/tests/        bitdump.rs  blend3_r2_probes.rs  blend4_concave_fillet.rs
                           bool1_fix_pass.rs  extrude_acceptance.rs  extrude_interval.rs
                           fillet_h6_cap_rim.rs  k_report.rs  m5_pr12_battery.rs
                           mass_props.rs  mass_props_interval.rs  p1b_r1_probes.rs
                           review_blend4_r2_probes.rs  review_blend_e2_r1_probes.rs
                           review_chamfer_r1_probes.rs  review_m2_pr4.rs
                           review_m2_pr4_interval.rs  review_m3_pr1_sweep.rs
                           verbs_chamfer.rs
crates/topo/tests/         cube_doors_agree.rs  review_m3_pr4.rs (x3)
```

**This row's `l_prism` is two of those thirty.** Reconciling the three
`tests/common` fixtures leaves the other twenty-eight standing, so the
class this row names is a strict subset of the class its own fixture
belongs to. That is worth knowing before the reconciliation is scoped:
deciding "which `l_prism` body is right" does not decide which of thirty
sites should be naming a shared profile.

**The X4 instance for the `dup/one-prism-builder` unit, and it is
stated as one.** That unit closed a builder duplication and, in the
guard it wrote to prove the closure, added the thirtieth copy of this
profile literal (`cube_doors_agree.rs`'s `REFLEX_L`). It is a
legitimate copy on its own terms — that file restates its inputs
deliberately, so that a guard is never comparing a builder against its
own expression of the same thing — but it is a copy, and the unit did
not see it, because:

**No instrument this program has ever run could see a duplicated
profile literal.** Every sweep so far keys on a *builder*: a name
(`prism_z`, `brick`, `cube_ops`), a construction (`mvfs(`,
`MevSite::Fan`, `find_half_edge(seed`), or an operator census over `fn`
bodies. A profile is **data passed to** a builder, so it is invisible to
all of them — and a literal spelled across six source lines is
invisible to `git grep` as well, which is why the count above needed a
different instrument.

**The instrument the class needs**, stated so the next census does not
have to invent it: flatten each `.rs` file's whitespace, extract every
`(<float>, <float>)` pair in order, and slide a window over the pair
sequence looking for a known corner list. It sees through the wrapper
spellings that defeat a text grep — `p2(0.0, 0.0)` and
`(0.0, 0.0)` both reduce to the same pair — and it generalises to any
fixture profile, not just this one. It cost about thirty lines.

Two caveats on the remedy, neither of which this row has to settle
today: a shared profile constant has the **same cross-crate reach
problem as the bodies** (`topo`, `sweep`, `mesh`, `stl`, `profile` and
`editor-core` cannot all name one home before link 3 of
`work/dup/brick-has-two-constructions-and-two-homes.md` lands), and at
least one site should **keep its literal on purpose** — a guard that
restates its own inputs is not duplicating, it is refusing to compare a
thing against itself.
