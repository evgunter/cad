---
id: VERBS-1031B
kind: unit
title: the full-valence coplanar pair — issue 1031's remaining half
status: dispatched
opened: 2026-09-03
github: 1031
refs: [VERBS-C5ARMS]
---

Issue #1031's half B: the full-valence coplanar sector pair the lily's
wall 7 measured, left open when the pole half landed with #1131. The
standing plan (VERBS log, 2026-08-30): the unit's OPENING MEASUREMENT
decides fork-or-dissolve — either the repaired-lantern re-measurement
shows the pair now expressible through the existing sector machinery
(dissolve, close #1031 with the measurement) or it names a genuine
design fork that goes to Ev as a measured dump, never an (a)/(b) menu
built on guesses. No spec until that measurement is taken; the
KERNEL-VERBS register's deferral note (the LILYWELD ruling lineage)
points at this item. Queued after RIMCAP.

**Opening measurement taken (2026-09-03, at main 56fe8099d) — split verdict:**

- **(a) DISSOLVED for the lily wall-7/wall-2 class** the deferral was
  about: on the repaired lantern with the operand gate scratch-widened,
  the sequence runs gate → F7 passes → `CurvedPierceUnsupported`
  (reduce.rs:1099, the shared curved-pierce substrate the banked germ
  lanes already own). The gate-admission deferral closes on this
  measurement.
- **(b) STANDS for the full-valence coplanar pair itself; the live
  consumer is the TEAPOT CUP**, not the lily (the register's meridian
  gloss was itself wrong — the pair is a latitude annulus, two disjoint
  collinear Line segments, all four endpoints valence 4, no shared
  vertex). Measured door: `MergedFaceRoleAmbiguous` at
  merge_faces.rs:1556 — and the instrumented mechanism is one level
  deeper than the issue records: the kef/kemr surgery COMPLETES; the
  role pass fails because `loop_winding` (merge_faces.rs:1457) is
  Line-bounded-cycles only and the merged annulus's outline and ring
  are both circles. The whole gap is one arc-bounded winding arm.

Fork for Ev (options measured, no recommendation): (1) repair op —
an arc-bounded winding arm in `loop_winding` (closed-form signed area
for Circle carriers; kef/kemr already complete); (2) producer — full
`revolve` mints off-axis planar walls as one face (touches the
two-π-band convention and every half-wall-counting fixture, and
would STILL need the winding arm when re-charting merged walls —
the measured twist that makes this the "do (1) first" branch of the
issue's own recorded decision rule).
**Ruling (Ev, in-chat 2026-09-03):** "my former decision rule still
makes sense and you should do (1)" — the repair op. Option 2 (the
revolve producer change) rejected. Spec ratified as
`docs/VERBS-1031B-SPEC.md` on `mngr/kernel-verbs`; the ratification's
own code read found the arc machinery ALREADY EXISTS in
`boolean::join::ring_run_ccw` (the bulge term, the same
`bool_ring_run_winding` predicate) — the unit is a port into
`loop_winding`, making merge_faces the predicate's fourth
identically-stated site.

**Dispatched (VERBS-1031B lane, branch `verbs/1031b-winding`).** The
opening measurement re-taken at the lane's head reproduces the spec's
door verbatim — `MergedFaceRoleAmbiguous { face: FaceKey(4v1) }`, and
the instrumented mechanism confirms the premise one level deeper: the
merged annulus's outline and ring are each a TWO-half-edge cycle on
Circle carriers, so `loop_winding` answers `Ok(None)` twice and the
role pass sees `positives: []`.
