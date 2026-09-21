---
id: sweep-boolean-suite-brick-and-prism-copies
kind: issue
title: sweep tests: the prism(pts, h) copy class outside the blend family (the brick half closed by S52)
status: open
opened: 2026-09-03
priority: P4
cost: E
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

## Re-derived (2026-09-15, lane D)

**VERDICT: PARTIAL** — the `brick` class reproduces member for member;
the `prism` class has moved substantially and the row's two
byte-identical pairs are both wrong today.

**Commands.** `grep -rn 'fn brick' crates/sweep/tests/`,
`grep -rn 'fn prism' crates/sweep/tests/`, then each body extracted with
`awk '/fn <sig>/,/^}/'` and compared by `md5sum` / `diff`.

### `brick(x, y, z)` — REPRODUCES, 6 members, unchanged

Byte-identical (one md5 across all three):
`bool2_cone_doors.rs:106`, `bool3_torus_doors.rs:178`,
`verbs_gate_r1_probes.rs:46`.

Diverged by spelling only, exactly as the row says:
`bool2_r2_probes.rs:61` (the same body with the `Profile::new(…)`
inlined into the `extrude` call) and `r1_probes_m9_3.rs:362` (identical
but for `geom_core::Affine3::` vs `Affine3::`).
`s49_census_jurisdiction.rs:71` is still the `brick(z0, h)` half-width
variant.

**One member the row did not name**: `verbs_shell.rs:48`,
`pub(crate) fn brick(x0, x1, y0, y1, z0, z1)` — the same job over six
scalars instead of three tuples, doc'd *"R2's `brick`; shared with the
review rows"*. It is already a shared fixture for its own family, which
makes it the obvious candidate home for the six.

### `prism` — MOVED; the row's pair claims are both stale

The row's byte-identical pairs today:

- `sf2a_r2_probes.rs` = `verbs_shell.rs` — **no longer identical**:
  `diff` is one line, `verbs_shell.rs:127` is now `pub(crate) fn prism`.
- `m8_4_intersection_iso.rs` = `r1_p2_probes.rs` — **still a
  byte-identical pair, but of a different fixture**. Both are now
  `fn prism(scale: f64)` (`:47` and `:53`), a three-section lofted
  offset square prism through `sweep::loft_body`, not "a polygon on
  `SketchPlane::xy()`, extruded by `h`". The duplication survives; the
  row's description of it does not.

`prism(pts: &[(f64, f64)], h: f64)` members today — **9**, and the
byte-identical pair is now a NEW one:

- `sf2a_r2_probes.rs:38` = `shell5_r1_dump.rs:21` (same md5) —
  `shell5_r1_dump.rs` is **not on the row's list**;
- `verbs_shell.rs:127` (identical but for `pub(crate)`);
- singles: `m5_pr12_fix_pass.rs:21`, `review_pr12_probes.rs:24`,
  `offd2_r1_probes.rs:34`, `review_chamfer_r1_probes.rs:52`,
  `sf2a_r1.rs:37` (delegates to a local `try_polygon`, so a diverged
  body), `sf2a_r2_interval_probe.rs:21` (over `Interval`).

**Left the class**: `tcost_k3_certificate.rs:125` is now `fn prism()`
with no parameters, returning `arc_prism(1.0e5 * Tol::witness().get().eps)`
— a different fixture with its own derivation in its doc.

### A stale pointer this row causes

`crates/sweep/tests/common/cavity.rs`'s module doc restates this
census in-tree — *"`sf2a_r2_probes.rs` = `verbs_shell.rs`,
`m8_4_intersection_iso.rs` = `r1_p2_probes.rs`, plus six singletons"* —
and both equalities are now wrong. It also tracks the row at
`work/tcost/sweep-boolean-suite-brick-and-prism-copies.md`, a path that
has not existed since the 2026-09-11 move to `work/tint/`.

**Blind spot.** Re-derived on the row's own `fn brick` / `fn prism`
names; a builder doing the same job under a third name is still
unmatched, which is the blind spot the row itself declares. Not
re-argued on cost — the row says it is not a cost finding and nothing
here touches that.

**Recommend: keep open, rewrite the prism half and add
`verbs_shell.rs`'s `brick`.**

## Re-scoped by `S52` (2026-09-15, PR #2639), on top of lane D's re-derivation

Lane D's census above was taken against `main` before #2639 landed and
is the census of record for the `prism` half. The `brick` half is now
closed, so only D's `prism` findings are live.

**The `brick` half is closed.** All six sites this row names build
through `sweep::test_support::brick`, and so does the seventh D added —
`verbs_shell.rs`'s six-scalar `pub(crate) fn brick`, which is **gone**
rather than promoted. D nominated it as "the obvious candidate home for
the six"; that turned out to be the wrong home, because the same box is
spelled in six other crates and a `sweep/tests/` item cannot serve them.

**Eight more members D's grep could not see.** `boxy` / `boxy_at` is the
same box under another name in `verbs_shell.rs`, `offd2_r1_probes.rs`,
`shellfix1_bitdump.rs`, `m5_pr12_battery.rs`, `m5_s13_review_probes.rs`,
`review_s12_adv.rs`, `m5_pr12_refusals.rs` and `shell5_r1_probes.rs`.
That is exactly the blind spot D declares — *"a builder doing the same
job under a third name is still unmatched"* — firing as written, twice
over, since #2639's own first pass missed it too. All eight are
converted.

**The home is not `cavity.rs`.** This row's closing paragraph nominates
it and says a cross-crate home "is LIB-U6's territory, which this tree's
routing rule says is deliberately not built here". #2639 built the
cross-crate home: `crates/sweep/src/test_support.rs`, reached from
another crate's suites through an off-by-default feature on a
dev-dependency edge. `cavity.rs`'s own builders now delegate to it.
Anything taking the remainder extends **that** module.

**D's stale-pointer finding is fixed**: `cavity.rs`'s module doc no
longer restates this census (the two equalities D showed were wrong are
gone with it) and now points at `work/tint/`, not `work/tcost/`.

**Still open: the `prism` half, as D re-derived it** — nine members of
`prism(pts, h)` plus the separate lofted-`prism(scale)` pair in
`m8_4_intersection_iso.rs` / `r1_p2_probes.rs`, which is a different
fixture and arguably a different row. The home now has the two doors the
tuple-written ones collapse onto: `sweep::test_support::prism(verts, h,
tol)` and `sweep::test_support::corners(&[(f64, f64)])`; `crates/mesh/`
did exactly that conversion in #2639. Re-run D's commands before
converting, and grep the construction (`Extrusion::Distance`) rather
than the name, which is the lesson both censuses paid for.

Siblings filed by the same lane: `work/tint/topo-tests-brick-copies.md`,
`work/tint/tests-common-body-fixtures-triplicated.md`,
`work/tint/sweep-per-step-differencing-helper-unhomed.md`,
`work/tint/sweep-test-support-two-wrapper-conventions.md`.
