---
id: boolean-in-band-arms-read-ahead-of-the-declaration
kind: issue
title: topo: in-band arms read ahead of the declaration — the Boolean's sweep and sector primitives escalate before any face-pair declaration is read, so a declared pair refuses exactly as an undeclared one
status: open
opened: 2026-09-30
---


(TOPO, the fix pass of PR 3513: the review's M-1, the class row.)

## What

D4 ¶1 (i) offers "declare" only at a door whose declaration would
change the verdict. PR 3513's fix pass walked every coincidence site its
per-site table marked "yes" and found that most escalate before any
face-pair declaration is read, so a declaration changes nothing there.
Those sites escalate as a coincidence no declaration read ahead of it
settles (`BooleanDecision::Coincidence(_, DeclarationRead::Moot | Spent)`,
`crates/topo/src/boolean/refusal_routes.rs`), which names the
question's own lever and, where it passes on a nonzero sign, the
tolerance, and no declaration. That is honest about today's order; this
row is the order itself. Where a verified declaration names
the pair, reading it first would let it settle the in-band arm, as the
plane ladder's declared rung already does for parallelism
(`plane_eq::declared_rung`: "In-band offset/parallel margins are
accepted").

Executed evidence (the fix pass's probe): two bricks, the upper one's
bottom face `mid = (zero + escalate)/2` above the lower one's top,
unioned undeclared and with the pair declared `Rest`. Both refuse
identically, as the vertex-on-face coincidence on
`bool_vertex_face_side` from the planar sweep; at a zero-band gap the
declared union succeeds and the undeclared one is
`UndeclaredCoincidence`. So the declaration settles the pair at the
plane ladder, and the in-band sweep never gets there.

The sites, each asked ahead of any declaration read:

- `boolean::reduce::sweep_direction` (`reduce.rs`): the conic × plane
  offset (`bool_conic_face_plane_offset`), the conic root lane's
  coincidence rungs (`BooleanDecision::of_conic_root`), and the vertex
  side against a plane (`bool_vertex_face_side`, both arms). The sweep
  takes `declared` but reads it only on curved faces.
- `boolean::reduce::curved_face_arm`: the endpoint sides
  (`bool_vertex_face_side`, before the declared-cover rungs) and the
  `(Positive, Positive)` line clearance
  (`bool_line_cylinder_clearance`), which reads `covered` nowhere. (The
  covered circle's clearance, listed here first, is asked after
  `covered` is read: `covered` is the declaration read, and that
  refusal is a declared door's, `DeclarationRead::Spent`.) For the last, the roots decide what the clearance bound
  cannot, as they do on its `Zero | Negative` arm and on the torus
  circle rung, so an in-band bound could fall through to
  `wall_crossing` rather than refuse; that is a verdict change, left
  to this row.
- `boolean::sectors` (`sectors.rs`): `side_code`'s chord and
  `enters_material` readings, `within`, `parallel_same` and
  `pair_search` (which takes no declarations). `vtxfac::classify_vertex_on_face` reads each
  bound's side before its coplanar lump reads the pair's class, and
  `mod`'s vertex–vertex pass runs `pair_search` before `recl` reads any
  declaration.
- `boolean::recl::parallel_same_dir` (`recl.rs`, `bool_ee_collinear`,
  `bool_dir_same`): the mention grouping, before `resolve_edge_edge`.
- `boolean::join::frame_refusal` (`join.rs`): the section pose, which
  reads parameter-source evidence and no face-pair declaration.

Fixed in the same pass by reading the declaration first:
`vtxfac`'s coplanar sector (`bool_sector_coplanar`) now reads
`declared.class_of` before an in-band parallelism refuses, so a declared
pair's lump takes the residue and only an undeclared pair escalates, as
a `Coincidence`.

## Repair shape

Per site, read the pair's declaration before the in-band arm refuses,
and let a verified declaration take the in-band residue the way the
declared plane rung does (intent plus non-contradiction), returning the
site to `BooleanDecision::Coincidence`, whose refusal then offers the
declaration truthfully. The sweep would need the vertex's (or edge's)
incident faces' declared partners against the face it meets; the sector
primitives would need the pair's class threaded to `side_code`,
`within` and `pair_search`. Each site moves on its own, and a row per
site pins the declared posture's verdict beside the undeclared one's
refusal (`reduce::declaration_order_rows` is the shape).

## Since (PR 3513's second fix pass)

`BooleanDecision::Proximity` is gone: every coincidence escalation is
`BooleanDecision::Coincidence(Coincide, DeclarationRead)`, and the
sites above state `DeclarationRead::Moot`, which ends in the
coincidence's lever and the tolerance and offers no declaration. The
repair shape is unchanged: a site that comes to read the pair's
declaration first states `DeclarationRead::of(class)` there, and its
refusal then offers the declaration where the pair is undeclared.

## Since (PR 3513's third fix pass)

A door's read is minted by the lookup, not stated:
`DeclaredPairs::read(pairs, question, admitted)` spends a declared
pair's class, and settles an undeclared one only for a class the door
admits there that the question states it is settled by
(`Coincide::settled_by`); `DeclarationRead::Settles` carries a
`Settling` no other module can build. The sweep's sites above now read
the edge's parent faces against the face (`reduce::edge_face_read`), so
a declared pair reads `Spent` there, and each question's ending comes
from its own pass set. `sectors`' poisoned norm and the germ line are
the kernel's own checks (`BooleanDecision::SelfCheck`), the bisector is
its own decision, and the sphere scan's questions are
`BooleanDecision::Sphere`, which take no declarations and carry no
read. The repair shape is unchanged: a site that comes to read the
pair's declaration first, and to let it settle the question, adds the
question to `Coincide::settled_by` and passes the classes its door
admits.
