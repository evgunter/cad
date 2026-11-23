---
id: conic
kind: program
title: CONIC — a conic's position is read from the point it stores: one polynomial along a conic, the centre a datum, a conic from a conic a map
status: ready
opened: 2026-11-23
group: kernel/geom/curves
prefix: conic/
tag: (CONIC orchestrator)
paths: [crates/geom/src/curves/conic.rs, crates/geom/src/curves/on_carrier.rs, crates/geom/src/curves/on_carrier/*, crates/geom-brep/src/curve_face/conic_oracle.rs, crates/geom-brep/src/curve_face/conic_oracle/*]
keep_out: [opened 2026-11-23 by splitting ONEROUND per work/README.md Track size after Ev's answer on PR 7523 fixed the charter - ONEROUND measured 47 budget points against 30 - every row moved by git mv with its id kept and a dated note on the row - the ratified contract is the conic clauses of crates/geom-core/README.md Float rounding (Ev on PRs 7258 and 7523) and the three designers' final texts in PR 7523's body - ONEROUND keeps the two conic units in flight on 2026-11-23 (oneround/conic-readers PR 7510 and oneround/conic-pair-readers) and this track's rows build on them - the readers live in geom-brep topo sweep step-import and mesh so territory is shared with CPAIR SECTCONE SECTTORUS PCERT and the boolean tracks by design - run scripts/work.py territory on your branch and announce the seam in the PR where a change is coupled]
---

**Every reading of where a conic is, is read from the point the conic
stores.** PR 7258 (Ev, 2026-11-22) let a construction hold a conic about
a point of its arc, its centre then derived; PR 7523 (Ev, 2026-11-23)
says what a reader may ask such a conic. This track builds that answer:

- one opaque **position** on `Conic`, and one **polynomial along a
  conic** in `geom` built from it, which every reader of degree one to
  four poses its question in;
- the **centre as a datum**, a wrapped value for reporters and for
  mints of a new entity's datum, which no decision reads;
- a **conic made from a conic as a map** that keeps its placement;
- the constructions whose centre runs off holding a point of their arc
  (four mints), and an anchor at a general point of the arc;
- and then `Conic::frame`, `along`, `semi_axis_vectors`,
  `unit_in_plane`, `u_ref` and `v_ref` leave, and the gate that pins
  their readers retires.

Slate and order: `work/conic/plan.md`; narrative in `work/conic/log.md`.
