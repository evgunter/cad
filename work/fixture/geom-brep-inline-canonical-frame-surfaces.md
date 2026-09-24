---
id: geom-brep-inline-canonical-frame-surfaces
kind: issue
title: geom-brep tests spell the canonical-frame elementary surfaces inline at 34 use sites
status: open
opened: 2026-09-03
priority: P4
cost: E
---


TCOST-8 gave `crates/geom-brep/tests/shared/surf.rs` one home for the
elementary surfaces built in the canonical frame — centred at the
origin, about `+z`, `u_ref = +x` — and routed the fifteen NAMED
helpers that built them there (`sphere`/`cylinder`/`cyl`/`zcyl`/
`unit_cylinder`/`torus`).

**Its census pattern could only see a named `fn`.** Sweeping instead
for the SHAPE — `Surface::{Sphere,Cylinder,Torus} {` with an origin
centre, a `+z` axis and a `+x` `u_ref`, in any of the spellings the
tree uses (`p3(0,0,0)`, `Point3::new(0,0,0)`, `Point3::origin()`;
`v3(0,0,1)`, `Vec3::new(0,0,1)`, `Vec3::unit_z()`; and the same for
`u_ref`) — finds **34 more, spelled inline at their use sites**. Every
`file:line` below is a **merge-base (`8433129ac`) line number**, taken
before TCOST-8's diff, so they can be resolved against `git show
8433129ac:<path>`:

- `Sphere` x 11: `intersect_table.rs:302`, `m5_pr7_ssi.rs:2073`,
  `m5_pr9_tangent.rs:357`, `mesh11r2_base_probes.rs:30`,
  `offa_r1_probes.rs:118`, `offa_r1_probes.rs:245`,
  `r2_probes.rs:132`, `review_m5_pr7_enclosure.rs:105`,
  `s58_iso_rectangle.rs:236`, `s58_iso_rectangle.rs:596`,
  `s58_iso_rectangle.rs:635`
- `Cylinder` x 14: `intersect_table.rs:814`, `m5_pr7_ssi.rs:851`,
  `m5_pr7_ssi.rs:882`, `mesh11_arc_branch.rs:336`,
  `offa_r1_probes.rs:48`, `pcurve_p1a_meter.rs:203`,
  `pcurve_parameter_finding.rs:47`, `r2_mesh7_door_probes.rs:126`,
  `r2_probes.rs:215`, `review_m2_pr3_certify.rs:225`,
  `review_m2_pr3_certify.rs:275`, `review_m2_pr3_certify.rs:356`,
  `review_m5_pr7_adversarial.rs:47`, `s58_iso_rectangle.rs:51`
- `Torus` x 9: `m5_pr9_tangent.rs:308`, `mesh11_arc_branch.rs:386`,
  `offa_r1_probes.rs:128`, `offa_r1_probes.rs:156`,
  `r2_mesh7_door_probes.rs:50`, `review_pr12_meridian_probe.rs:22`,
  `s58_iso_rectangle.rs:283`, `s58_iso_rectangle.rs:692`,
  `s81_one_rim_level_rule.rs:50`

Two of them are no longer inline: `offa_r1_probes.rs:48` and
`pcurve_parameter_finding.rs:47` were the bodies of the `zcyl` and
`cylinder` helpers TCOST-8 took into `shared::surf`. **The residue is
therefore 32**, and it is 32 use sites, not 32 helpers.

**Why TCOST-8 did not take them, and what a taker owes.** These are
not helpers; they are fixtures written at the row, and the frame is
sometimes the row's subject rather than incidental to it (a sphere at
the origin about `+z` is what makes a polar-degeneracy row a polar
row). Routing them through `shared::surf` is therefore a judgement per
site, not a mechanical rewrite, and each one that stays owes a reason
AT the site under `shared/mod.rs`'s second rule. That is a unit's worth
of reading; it is not a defect in the tree today, only an unpaid
consolidation.

No behaviour is at stake: every site listed builds a value bit-identical
to `shared::surf::{sphere,cylinder,torus}` at its radii.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

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

**VERDICT: REPRODUCES**, with the floor RAISED: the residue is **40**
inline use sites, not 32.

**Command.** A parser rather than a grep, because the fields wrap across
lines: for every `Surface::{Sphere,Cylinder,Torus} {` in
`crates/geom-brep/tests/**`, brace-match the literal, then require the
`center`/`origin` to match `p3(0,0,0)` / `p(0,0,0)` /
`Point3::new(0,0,0)` / `Point3::origin()`, the `axis` to match
`v3(0,0,1)` / `v(0,0,1)` / `Vec3::new(0,0,1)` / `Vec3::unit_z()`, and
`u_ref` the `+x` equivalents (a wrapper such as `ip(…)`/`iv(…)` around
any of them still matches). Script kept at
`/tmp/.../scratchpad/census.py`; it is ten lines of `re` over
`os.walk`.

**Result: 43 canonical-frame constructions, 3 of which ARE
`shared/surf.rs`'s own `sphere`/`cylinder`/`torus`. Residue = 40.**

Per file: `intersect_table.rs` x8 (`307`, `652`, `1180`, `1278`,
`1284`, `1436`, `1442`, `1465`), `s58_iso_rectangle.rs` x6 (`51`, `236`,
`283`, `596`, `635`, `692`), `offa_r1_probes.rs` x4 (`112`, `122`,
`150`, `239`), `review_m2_pr3_certify.rs` x3 (`225`, `275`, `356`),
`m5_pr7_ssi.rs` x3 (`851`, `882`, `2078`), `m5_pr9_tangent.rs` x2
(`293`, `342`), `mesh11_arc_branch.rs` x2 (`336`, `386`),
`r2_mesh7_door_probes.rs` x2 (`50`, `126`), `r2_probes.rs` x2 (`132`,
`215`), and one each in `m5_pr12_circle_certificate.rs:66`,
`mesh11r2_base_probes.rs:32`, `pcurve_p1a_meter.rs:215`,
`review_m5_pr7_enclosure.rs:105`, `review_pr12_meridian_probe.rs:22`,
`rim_dim_review_probes.rs:43`, `rim_dim_scale_twins.rs:262`,
`s81_one_rim_level_rule.rs:49`.

**All 9 Torus and all 11 Sphere sites the row lists survive** (line
drift only). Of the 14 Cylinder sites:

- `offa_r1_probes.rs`'s and `pcurve_parameter_finding.rs`'s were
  absorbed, as the row already recorded — confirmed in the tree:
  `use crate::shared::surf::cylinder as zcyl;` and
  `fn cylinder() -> Surface<f64> { surf::cylinder(R) }`;
- `review_m5_pr7_adversarial.rs:47` **left the class by changing frame,
  not by absorption**: it is now
  `Surface::Cylinder { origin: Point3::new(0.01, 0.0, 0.0), … }`, an
  off-axis fixture. (That file does use `surf::sphere(1.0)` at `:33`.)

**What grew.** `intersect_table.rs` alone now carries 8 canonical-frame
literals where the row named 2, and three files not on the list at all
carry one each (`m5_pr12_circle_certificate.rs`,
`rim_dim_review_probes.rs`, `rim_dim_scale_twins.rs`) — the last two in
the `p(…)`/`v(…)` spelling the row's enumeration did not list.

**Blind spots.** (a) Field-shorthand constructions
(`Surface::Cylinder { origin, axis, radius, u_ref }` over bound
variables) are not matched — 46 `Surface::*{` sites in this corpus have
no `field:` form at all, most being `let … else` destructures, but a
shorthand construction of a canonical-frame value would be missed.
(b) A frame built from named constants rather than literals is missed.
(c) `shared/surf.rs`'s own "deliberately not absorbed" list
(`review_m6_3_chart_probes.rs`, `pcurve_conic.rs`, `offset_mint.rs`) was
confirmed non-canonical by the same parser, so none of those is in the
40.

**Recommend: keep open, correct the count to 40**, and note that the
per-site judgement the row asks for now has a worked precedent on both
sides (absorbed in `offa_r1_probes`/`pcurve_parameter_finding`, kept-and-
re-framed in `review_m5_pr7_adversarial`).
