---
id: validate-tier3-curved-boundary-containment
kind: issue
title: validate tier 3 — face-boundary containment on curved surfaces (the last unmarked deferral in the not-yet-checked list)
status: dispatched
opened: 2026-08-19
refs: [226, 635, cone-holes-outside-their-face-are-subtracted-silently]
priority: P1
cost: H
branch: restcurve/region-arm
---

## From GitHub issue 638

Opened 2026-08-19; 0 comments.

Opened by the **S39/H2 stale-claims lane** (#635) so a genuinely unimplemented item keeps a findable marker. Three prose sites carried "M3, with pcurves" as this item's schedule; M3 shipped long ago, so those labels were stale claims and had to go — but deleting them without replacement would have left a real gap with no marker at all, which is the failure mode S39 exists to hunt. This issue is the replacement marker.

## What is not checked

`topo::validate`'s tier-3 header, under **"What tier 3 does NOT yet check (deferred, named)"**:

> **Face-boundary containment on curved surfaces** — a face's loops actually bounding a region of its surface.

What IS covered today, so the gap is stated precisely:

- **Planar, between vertices** — check 5 (sample containment against adjacent planar faces).
- **Planar, orientation half** — check 6 (loop-role winding against the outward normal, line-bounded loops).
- **Curved analytic, orientation half** — check 6's curved arm (M6-6: boundary material side vs the sense bit).
- **Planar, the loop-NESTING half** — check 9's nesting arm
  (`ValidationError::RingOutsideOuter`): a face's ring lies strictly inside that
  face's own outer loop, decided by the crate's trilean containment walk over the
  ring's vertices. Added by `tier3-accepts-a-ring-outside-its-outer-loop`. It
  reaches a face on a `Plane` whose outer loop is in `boolean::contain`'s
  `Polygon` class (no arc anywhere, so the walked polygon IS the loop's
  region) or its `Disc` class (one circle, decided by `disc_side`), and is
  silent on the other arc-bearing classes and on any non-planar face
  (`work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk.md`).

What remains uncovered:

- containment against **curved** surfaces;
- the **region-bounding** statement for curved faces, and for the planar loop
  classes check 9's nesting arm is silent on (the disc class and the no-walk
  class);
- the curved arm's documented residuals — the rimless sphere band; NURBS faces; the quadrature-owned conic-trimmed walls (whose boundary parse refuses typed and is therefore exempt: such a body's flips, single-face and whole-body, certify green today, executed on the tilted-section cylinder and pinned as residual).

## Why it is a marker and not a plan

No design work is proposed here and none is implied. The item is real, it is unscheduled, and the point of the issue is that it stays findable from the three code sites that cite it rather than dissolving into an undated "deferred".

## Cited from

- `crates/topo/src/validate.rs` — `validate_geometric`'s not-yet-checked list (the
  bullet above, which now records check 9's nesting arm as covering the planar
  nesting half and names the arm's own residue as sitting inside this deferral)
- `crates/topo/src/validate.rs` — `ValidationError::PlanarBoundaryResidual`'s doc
- `crates/topo/src/validate.rs` — the tier-3 check-5 rustdoc and its in-body check-list comment

Related but distinct: the **material wedge side** (lamina/zero-volume detection) is a separate bullet in the same list with its own state — since M5 S10 the per-face half is no longer missing; what is absent is the edge-local pairing. Not covered by this issue.

## Home

`work/issues/`: `crates/topo/src/validate.rs` is in no open program's `paths` — S-BOOL's territory is `topo/src/boolean` and `topo/src/splitting`, and S-CERT's stops at `geom-brep/src/props`.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/topo/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Evidence (TOPO, PR 3598, 2026-10-01)

`Body::set_face_surfaces_describing` (`crates/topo/src/attach.rs`)
checks a moved face's boundary residuals on a plane only. On a curved
chart it takes the swap unchecked, as tier 3 does at rest.

The keys-only `set_face_surface` refuses such a swap
(`RechartUnvouched`). Its text says the describing door does not check
a curved chart, and cites this row.

Seven of `boolean::boxes`' arc-sector relabels (sphere ×2, the bulged
patch, the kinds list, torus ×2 and the two-lane row) go through the
describing door only because of this gap. When curved containment is
checked there, they move back to `set_face_surface_stranding_for_tests`,
whose rule is a `// Lifts …` reason at each call site.

`attach::tests::a_curved_swap_is_refused_keys_only_and_taken_unchecked_by_the_describing_door`
pins today's `Ok` and names this row in its message.

## Evidence (REACH, plane × cone split fix pass, 2026-10-01)

A wrong body passed every tier. Before the fix to the split's arc-side
window across a cone apex, the upright cone (`revolve` of the triangle
`(0,0), (1,0), (0,1)` about `y`) cut through `(0, 0.4, 0)` with normal
`(sin 0.5, cos 0.5, 0)` produced a lower half of volume 1.0815, more
than the whole cone (`π/3`). It passed tiers 1, 2 and 3. One of its two
cone faces took the section ellipse the long way round, through the
part of the ellipse below the base plane `y = 0`; the arc is on the cone
and on the cutting plane, so every residual is zero.

What a check could have seen, from the edge dump:

- **The planar section face's loop is not simple.** Its two ellipse
  edges carry parameters `(π, 7.539)` and `(1.256, π)`: together the
  whole ellipse, so the loop runs through its own vertex
  `(0.732, 0, 0.681)` in the middle of an edge. Check 6 reads the
  loop's winding sign and check 9 its rings, and neither asks whether
  a loop's edges meet only at its vertices.
- **The cone face crosses the base disc's plane away from any edge.**
  The long arc dips to `y < 0`, so the cone face passes through the
  plane of the base face inside that face. Global self-intersection is
  the deferral the validate header names; check 5 tests an edge only
  against its ADJACENT planar faces, and the base is not adjacent to
  the arc.

The first is the cheaper statement, and it has its own item:
`tier3-never-checks-a-planar-loop-is-simple`. The split's arc-side rule now takes the window from the apex-
closed lift (`chord_join::run_azimuth_window`), and the 144-pose sweep
of that cone is a row
(`sweep/tests/reach_cone_split.rs`
`an_upright_cone_splits_at_every_pose_through_its_apex_faces`).

## A case the arm's rows should cover (FLUX, 2026-10-19)

`work/conearm/cone-holes-outside-their-face-are-subtracted-silently`: a
cone face's correctly wound hole lying outside the loops that bound it
is subtracted from the face's area with no refusal, on a band (P09: band
`[1, 2]` less a window `[3, 3.3] × [1, 2]` measures `4.0654`), a cap
(P47) and a window (P49). The probes and their values are in that row;
by Ev's PR 4822 decision the check is this arm's, not the lane's.


## Spec, unit 2: the region arm (RESTCURVE orchestrator, 2026-10-31)

The design is ratified: Ev, `[ev]` PR 6085, decisions 1 and 2; fork-log row 221. DESIGN `[topology.tiers]` tier 3 now carries "one region per face". This unit builds that clause's tier-3 check on every surface kind. Unit branch: `restcurve/region-arm`. Review tier: **dual, concurrent pair**. The unit adds a tier-3 refusal on every curved kind, and its instrument is a reader the boolean shares (difficulty H).

### What to build
1. **(b) Each surface cycle lies on the face's side of every other.**
   - Take the face's surface boundary (`topo::surface_boundary::surface_cycles`, PR 6088). For each cycle, take one decided point on it and ask the point-in-face doors (`boolean::{ruled,sphere,torus,spline}_region`, behind `curved_face_containment`) restricted to ONE other cycle, whether the point lies on the face's side.
   - Refuse a cycle that lies outside another. This covers a ring outside its face and a ring inside another ring, on every kind. On a plane, it must agree with check 9's nesting arm, or replace it; say which, and keep one reader.
   - A path no decision clears is set aside. A cycle that no path reads is a typed exemption, named in validate's not-yet-checked list. Its residue is exactly the doors' per-kind carrier inventories (`region::boundary`'s `admits`); list them.
   - Claim to check first: whether the doors accept a single cycle without re-reading the face's other loops. Today they read every loop at once. If restricting them needs a new entry, give it one entry, not a second walker.
2. **(c) An open surface's ends lie off the face.** Check 6's planar and cylinder/cone arms already decide this. Make them this clause's instance: cite them and name them, with no second reading.
3. **(d) Torus.** Essential cycles form a separating pair, with their windings read as the lift's integers (`loop_windings` / joint elements), not re-derived.
4. **(e) The chart-singularity inside test** (`geom_brep::chart_singularity::locate`'s interior/outside placement) becomes (b)'s side reading of the singular point.
   - `walk_loop` and its certified area-error bound retire.
   - `a-loop-the-chart-singularity-walk-cannot-certify-is-exempt` closes by retirement.
   - The at-a-vertex and inside-an-edge placements stay; they are the one incidence reading.
5. **`region::boundary` reads through `surface_cycles`.** The filter and the walk become one, which is F6 from PR 6088's review.

### Rows
- **The cone holes outside their face:** CONEARM's `cone-holes-outside-their-face-are-subtracted-silently` (P09 band less a window, P47 cap, P49 window). Each refuses at tier 3. Take the values from that row.
- **Curved faces:** a ring inside another ring on a cylinder and on a sphere; a tilted cap holding a pole.
- **The torus:** an essential pair, accepted; a mis-paired pair, refused.
- **No false refusals:** PR 6054's 10 µm sliver 1 cm from a pole passes. Every valid body in the corpus and the step-import fixtures still passes; run both at ε 1e-6, the default and 1e-12.
- **Undecided readings:** a cycle read in band escalates typed, with its margin (D4 ¶1).

### Not in this unit
- **(a) the meeting arms off the plane:** `check-9-meeting-arms-silent-off-a-plane-and-on-ellipse-spiric-nurbs-edges`, P3, its own row.
- **Retiring props' region guards** (PR 4822's list: the cone's `HolesExceedFace`, contractibility, vertex-distinct, once-round). These follow once this arm covers them; file the follow-up on CONEARM's slate.
- **The mesher.**
