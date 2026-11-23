---
id: planar
kind: program
title: PLANAR — two curves in one plane read once, through C5's plane instance, by profile validation, check 9, the census and both ray walks
status: ready
opened: 2026-11-22
group: kernel/geom/curves
prefix: planar/
tag: (PLANAR orchestrator)
paths: [crates/geom-brep/src/curve_pair.rs, crates/geom-brep/src/curve_pair/*, crates/topo/src/ray_walk.rs, crates/profile/src/seg.rs, crates/profile/src/seg_reach_fuzz.rs]
keep_out: [opened by PROFILE's cut of Ev's plane ruling (PR 7290 - 2026-11-21) - the build crosses geom-brep and topo and profile and its first units are not profile ground, the curve_pair files are CPAIR's and ONECOPY's too - run scripts/work.py territory on your branch and announce on both logs where a unit changes what meet answers, check 9 (crates/topo/src/validate.rs) is RESTFRONT's and BOXFINISH's and MEASUREDOOR's and ONEDOOR's ground and the census (crates/topo/src/census.rs) is ATREST's and CONTACTHOLD's and HELDRECORDS's and ONEDOOR's - the units that move their readers announce there before landing, crates/profile is PROFILE's - the pair pass and the ray walk move onto the reader here and PROFILE's own rows on that pass (the shared-vertex rows and the ovoid's ellipse segment) wait on this program's units, the touch-kind row is ONECOPY's (a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point) and two units here wait on it, C5 in crates/geom-brep/README.md is RATIFIED text - a change to what it decides is an [ev] PR and a re-word forced by an approved change lands with it]
---

Opened on 2026-11-22 by PROFILE's cut of the plane ruling (PR 7290,
Ev 2026-11-21: "sounds good!"). The ruled text is C5's co-surface
paragraph in `crates/geom-brep/README.md` and the `profile` row of
`docs/DESIGN.md`'s Layering table.

## Charter

Two curves in one plane (two edges of one plane record, or two
segments of one sketch plane) meet through one reading: C5's plane
instance in `geom-brep`, entered through the curve-pair table's own
in-plane arms with coplanarity given by a type and never read, each
candidate settled by its place on each span, its spans keeping their
ends as their holders have them. Its readers are profile validation's
pair pass, check 9's ring-against-outer contact reading, the census's
pairs on one plane record and both planar ray walks. The private
readers they carry today (`profile::seg`'s pair and ray readers,
check 9's arms 4 and 5) are deleted, `seg`'s only after a differential
against the instance agrees.

Not this program's: profile's joint pass; the two fillet
constructions' placement (LB4 and their own rows); the touch kind on
`curve_pair::Meeting`, which is ONECOPY's P1 row and which two units
here wait on.

## Why a program of its own

PROFILE was over its 30-point budget before this build, and the build
is seven hard units, 56 points of P1 work. Its seam is a mechanism, not a crate:
one reader for the planar curve-pair question, built up in
`geom-brep` and then read by three crates. The first two units are
CPAIR's ground (the pose and the span), and CPAIR's charter is the
reader's residue rather than its new instance. So the build is its own
track, beside CPAIR in `kernel/geom/curves`.

## Ratified ground (cited, not re-litigated)

- C5 (`crates/geom-brep/README.md`), its co-surface paragraph as
  PR 7290 wrote it, and PR 6567 (two curves on one surface record are
  not a pair for `meet`).
- `[one-way.readings]` (`docs/DESIGN.md`): carrier-pair readings live
  in `geom-brep`; a bound on where a span can lie is a method on its
  cover.
- The Layering table's `profile` row (PR 7290): profile reads how two
  of its segments meet, and how a ray crosses one, through
  `geom-brep`'s curve-pair reading.
- PROFILE's list (`work/profile/program.md`) for everything on the
  profile side.
