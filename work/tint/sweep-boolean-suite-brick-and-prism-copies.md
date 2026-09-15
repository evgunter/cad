---
id: sweep-boolean-suite-brick-and-prism-copies
kind: issue
title: sweep tests: the prism(pts, h) copy class outside the blend family (the brick half closed by S52)
status: open
opened: 2026-09-03
---


Found by TCOST-10's census while homing the blend tree's fixture
builders in `crates/sweep/tests/common/cavity.rs`. Two copy classes of
the same shape survive elsewhere in the same crate's corpus; they are a
different suite family, so TCOST-10 named them at the home rather than
absorbing them.

**`brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64>`** —
a rectangle on a translated XY sketch plane, extruded, `unwrap`ping
both doors. Byte-identical in three places:

- `crates/sweep/tests/bool2_cone_doors.rs:105`
- `crates/sweep/tests/bool3_torus_doors.rs:177`
- `crates/sweep/tests/verbs_gate_r1_probes.rs:46`

and diverged (same job, different spelling) in three more:

- `crates/sweep/tests/bool2_r2_probes.rs:64`
- `crates/sweep/tests/r1_probes_m9_3.rs:356`
- `crates/sweep/tests/s49_census_jurisdiction.rs:71` (`brick(z0, h)`,
  half-width about the axis)

**`prism(pts: &[(f64, f64)], h: f64) -> Body<f64>`** — a polygon on
`SketchPlane::xy()`, extruded by `h`. Byte-identical in two pairs:

- `crates/sweep/tests/sf2a_r2_probes.rs:42` = `crates/sweep/tests/verbs_shell.rs:102`
- `crates/sweep/tests/m8_4_intersection_iso.rs:46` = `crates/sweep/tests/r1_p2_probes.rs:47`

and singly in `m5_pr12_fix_pass.rs:21`, `offd2_r1_probes.rs:38`,
`review_chamfer_r1_probes.rs:47`, `review_pr12_probes.rs:26`,
`sf2a_r1.rs:42`, `sf2a_r2_interval_probe.rs:21` (over `Interval`) and
`tcost_k3_certificate.rs:126`.

Not a cost finding: these are constructors, and TCOST-10 measured no
execution change from homing the blend tree's. The value is the same —
one place for a fixture to be, and the independence claims made
checkable at the copies that stay. `crates/sweep/tests/common/cavity.rs`
is the home to extend; its module doc carries the rule.

Deliberately out of scope for a sweep of this class: `topo`, `mesh`,
`stl`, `step-export` and `editor-core` hold their own `brick`/`prism`,
and a cross-crate home is LIB-U6's territory per `common/mod.rs`'s
routing rule. What that census could not match: a builder that is the
same job under another name, since it grepped `fn brick`/`fn rod`/
`fn prism`/`fn vented`/`fn cavity` and then compared comment-stripped
bodies.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.


## Re-scoped by `S52` (2026-09-15, PR #2639)

**The `brick` half of this row is closed.** All six sites it names —
`bool2_cone_doors.rs`, `bool3_torus_doors.rs`, `verbs_gate_r1_probes.rs`,
`bool2_r2_probes.rs`, `r1_probes_m9_3.rs`, `s49_census_jurisdiction.rs` —
now build through `sweep::test_support::brick`, and so do the eight
`boxy`/`boxy_at` copies of the same box under another name, which this
row's census could not see because it grepped by name (its own blind
spot, stated in the body above, firing exactly as written).

**The home moved, so this row's last paragraph is now wrong.** It
nominates `crates/sweep/tests/common/cavity.rs` and says a cross-crate
home "is LIB-U6's territory, which this tree's routing rule says is
deliberately not built here". `S52` built the cross-crate home:
`crates/sweep/src/test_support.rs`, reached from another crate's suites
through an off-by-default feature on a dev-dependency edge, and
`cavity.rs`'s own builders now delegate to it. Anything taking the
remainder below extends **that** module, not `cavity.rs`.

**Still open here: the `prism(pts, h)` class** — a polygon on
`SketchPlane::xy()` extruded by `h`, over `(f64, f64)` pairs. The sites
this row lists (`sf2a_r2_probes.rs` = `verbs_shell.rs`,
`m8_4_intersection_iso.rs` = `r1_p2_probes.rs`, plus `m5_pr12_fix_pass.rs`,
`offd2_r1_probes.rs`, `review_chamfer_r1_probes.rs`, `review_pr12_probes.rs`,
`sf2a_r1.rs`, `sf2a_r2_interval_probe.rs`, `tcost_k3_certificate.rs`) are
untouched, and the home now has the two doors they need:
`sweep::test_support::prism(verts, h, tol)` and
`sweep::test_support::corners(&[(f64, f64)])`, which is the pair
tuple-written suites collapse onto (`crates/mesh/tests/` did exactly
that in #2639). Re-count before converting: this row's census is from
2026-09-03.

Siblings filed by the same lane: `work/tint/topo-tests-brick-copies.md`,
`work/tint/tests-common-body-fixtures-triplicated.md`,
`work/tint/sweep-per-step-differencing-helper-unhomed.md`.
