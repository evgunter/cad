---
id: germ
kind: program
title: GERM — the curved lanes' section and root arithmetic: the cone and torus operand lanes, the charges and margins their roots and sections read, the arms short of a lane, and the refusals wider than they need to be
status: ready
opened: 2026-09-20
group: kernel/boolean/curved
prefix: germ/
tag: (GERM orchestrator)
ab_band: 7000-7099
paths: [crates/topo/src/boolean/sectors.rs, crates/geom-brep/src/intersect.rs, crates/geom-brep/src/implicit.rs, crates/geom-brep/src/curve_face/circle_roots.rs, crates/geom-brep/src/curve_face/circle_torus.rs, crates/topo/src/boolean/reduce.rs, crates/geom/src/curves.rs, crates/topo/src/boolean/conic_quadric/*, crates/topo/src/boolean/carrier_touch.rs, crates/topo/src/ring_path.rs, crates/topo/src/boolean/section_cert/*, crates/topo/src/rim_wedge.rs]
keep_out: [opened by REACH's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - REACH measured 151 budget points, about five sittings, and was cut into six tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on at once), the six are REACH GERM CONTACT ZIP BOXES PIN and they SHARE crates/topo/src/boolean/* by design - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, TANG shares chord_join.rs and boolean/rest.rs for the declared-tangency and germ/pierce lanes, CHART owns the pcurve and SSI side, GERMCERT (GERM's 2026-10-20 rounding and certificate tail), ROOTS (REACH's 2026-10-08 root and section arithmetic successor) and APEX (REACH's 2026-10-08 numeric-frontier successor) merged in 2026-11-02 when the budget came to count only P0 P1 and unbanded rows (docs/doc-ledger/germcert-roots-and-apex-merge-into-germ.md) and GERMTAIL (GERM's refusal and structure tail) merged back in on 2026-11-20, crates/geom-brep/src/intersect.rs crates/topo/src/ring_path.rs and crates/topo/src/boolean/* are shared with GERMTAIL ORBIT TANG CLEAVE and the REACH successors, GERMTAIL merged in on 2026-11-20 (docs/doc-ledger/germtail-and-honeroot-merge-into-germ.md) - it held no P0 or P1 row, from GERMTAIL: opened 2026-10-20 by GERM's closing cut (work/README.md Track size: GERM measured about 48 budget points against 30 and splits; Ev in chat 2026-10-19: finish what is important and leave GERM or its successor ready) - below GERM's P1 and P2 rows the slate is nearly all P3, from GERMTAIL: so the cut is by finding class into GERMCERT (rounding and certificate margins) and GERMTAIL (refusals and structure), from GERMTAIL: the rows moved by git mv with ids and bodies unchanged, from GERMTAIL: crates/topo/src/boolean/* crates/topo/src/ring_path.rs and crates/geom-brep/src/intersect.rs are shared ground with GERM and its sibling successor and with CLEAVE CHART TANG and the REACH successors - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, HONEROOT merged in on 2026-11-20 (docs/doc-ledger/germtail-and-honeroot-merge-into-germ.md) - it held no P0 or P1 row, from HONEROOT: opened 2026-10-26 by HONE's priority-seam split per work/README.md Track size - HONE carried 118.5 budget points against 30 and was cut into HONE (its P2 rows) HONEROOT HONEJOIN HONEONE HONEWORD and HONESPLIT - every row moved by git mv with its id - all six share crates/topo/src/splitting/* and crates/topo/src/boolean/* with each other and with CLEAVE by design - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live]
---

**The curved lanes' section and root arithmetic.** The cone and
torus are the two operand kinds the boolean lane reaches last. GERM's
P0 rows are closed: the ray-torus root search's wrong answer at
ε = 1e-12, the cone and torus arms of `curved_face_containment`, and
the declared cover for a coincident torus pair. The circle × torus walk
reads its Taylor terms at the conic point (PR 5394), and a near-square
plane × cylinder/cone section is the exact Ellipse (Ev, PR 5156).
`c5-plane-torus-cone-cylinder-arms` moved to SECT on 2026-10-25.

What is left is Interval and error propagation through the root and
section lanes, in four strands that merged into one track on
2026-11-02, when the budget came to count only the rows a sitting
dispatches:

- **Band reach on the conic sections** (GERM's own P1 and P2 rows): the
  meeting-axes pose's endpoint at 1e-12; an oblique rod clear of a cone
  refuses the extent scan at 1e-12; a conic section just past its band
  fails certification downstream; a sliver crossing's long bisector
  ellipse is served past what its evaluation can represent; coaxial
  unions with a tangent cone.
- **The root lanes' charges and the arms one step short of a lane**
  (from ROOTS, REACH's successor of 2026-10-08): the circle × cylinder
  square arm charges its root slack with the whole term bound; the
  conic × quadric first-harmonic arm charges the dropped second
  harmonic to crossings the ladder places; the circle × plane first
  harmonic's one door carries an unaudited noise charge. A line
  parallel to a cone's generator has no lane; a section circle inside a
  plane face's hole refuses at the edge boxes; a tilted section through
  a chart pole is not split there; a revolved face's narrow-phase
  anchor sits on its seam. Two rows owe coverage (the ellipse × torus
  door at ε 1e-12; the operand gate's curved arms through a finished
  fixture).
- **Margins that read the wrong quantity** (GERMCERT, GERM's rounding
  and certificate tail of 2026-10-20): the section arms decide a tube,
  radius or aperture on the band where tier 3 reads its lower bound;
  carriers read off `acos`/`atan2` near a degenerate pose carry rounding
  no margin reads; the conic doors walk their Taylor terms through the
  harmonics' noise; the root-slack meter reads an Interval root at its
  whole width; plane × cone levers its trileans at two reaches;
  `in_sector` escalates before a definite reading arrives. None is a
  wrong answer on record: each is a refusal or a margin that a lane
  must measure on current main first.
- **Sound refusals that sit wider than they need to** (from APEX,
  REACH's numeric-frontier successor of 2026-10-08): the cone root lanes
  (PR 4135) refuse a wide zone about the apex; their depth rung is
  levered by the carrier's reach and their lead rung by the segment's
  length; the far-nappe tell-off has no row where it decides. At
  ε 1e-12 an f64 first-order bound cannot place a shallow circle ×
  sphere crossing. The off-face touch lanes (PRs 4128 and 4159) keep the
  pierce door for a straddling span whose roots do not settle and for a
  conic touching a torus off its face, and their decisions have
  uncalibrated margins; `face_extent` misses a curved face's interior
  far point; a near-right-angle cylinder pair stops at a first-order
  sector side.

GERMTAIL, GERM's refusal and structure tail (arms that give up where
the answer is regular, readings decided outside their home), and
HONEROOT, the root doors' and sector readers' P3 certification gaps,
merged back in on 2026-11-20: neither held a P0 or P1 row. Their
charters are the `## From` sections below.

Charter and order: `work/germ/plan.md`; narrative in `work/germ/log.md`.

## From GERMTAIL (merged 2026-11-20)

GERMTAIL — GERM's refusal and structure tail: arms that give up where the answer is regular, and readings decided outside their home

**Where GERM's lanes still refuse a regular answer, or decide it in the
wrong place.** A certified interior loop is refused rather than cut
into both faces; a NURBS face against a plane has no component arm;
the square-wall arm refuses R-order at a tube's top; a full revolve's
return-half rim ends two rounded half-turns short past r ≈ 4100 at
1e-12; ring paths along a ruling or the crest read nothing, and
`ring_path` decides curve meetings outside `geom_brep::curve_pair`.
Two P4 rows are wording and a mislabelled backstop.

Slate, order and posture: `work/germ/plan.md`; narrative in
`docs/doc-ledger/germtail-and-honeroot-merge-into-germ.md`.

## From HONEROOT (merged 2026-11-20)

HONEROOT — the boolean's root doors and sector readers: P3 certification gaps in their margins

**The root doors and the sector readers decide on charged margins,
in the right dimension.** The circle-root subdivision decides a
piece's side on the bare residual and meters roots the arc never
reaches; line roots carry no root-slack meter at all; the ruling's
root order compares raw parameters across its seam; a Villarceau
circle on a torus is still `Uncertain`; parameter spans cross to
metres through `levered`; `rim_wedge` and `in_sector` decide bare
cosines and sines; the subdivision's width row has no row that goes
red without it; and an arc tangent to a face at its end is split where
it leaves the zero band.

Charter, lanes and slate: `work/germ/plan.md`; narrative in
`docs/doc-ledger/germtail-and-honeroot-merge-into-germ.md`.
