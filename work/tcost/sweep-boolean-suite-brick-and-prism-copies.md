---
id: sweep-boolean-suite-brick-and-prism-copies
kind: issue
title: sweep tests: brick/prism copy classes outside the blend family
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
