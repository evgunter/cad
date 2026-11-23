---
id: placeholder-question-has-two-spellings-across-topo
kind: issue
title: The placeholder question is asked through two doors across topo: NetState::Placeholder and is_placeholder()
status: review
opened: 2026-09-14
priority: P1
cost: M
pr: 7495
branch: walks/surface-slot
---


## What

The placeholder question — *is this `Nurbs` payload the `mvfs` seed's
"no description yet" net?* — is asked through two `geom` doors, and a
reader cannot tell from a call site's spelling whether the poisoned
state was considered there.

- `NurbsSurface::net_state() == NetState::Placeholder`, matched
  exhaustively over the three states: `crates/topo/src/merge_faces.rs`
  (`MergeKind::of`), `crates/topo/src/validate.rs` (tier-3 check 1, the
  arm S330 landed), `crates/topo/src/tier3_tests.rs`.
- `NurbsSurface::is_placeholder()`, a two-way answer: in `topo`,
  `census.rs` (the control-net box arm, the source walk's placeholder
  filter, a fixture count), `pcurves.rs` (`Surface::Nurbs(payload) =>
  !payload.is_placeholder()`), `props.rs`, `transform.rs` (two sites),
  `replace_face.rs`; outside it, `crates/geom-brep/src/certify.rs` (two
  sites, one of them a `described` closure spelled as the negation),
  `crates/geom-brep/src/edge_nurbs.rs` (three `wall.is_placeholder()`
  gates), `crates/geom-brep/src/pcurve_cache.rs`, `crates/mesh/src/
  {chords.rs,trimmed.rs}`, `crates/step-import/src/{entities.rs,adopt.rs}`.

Line numbers are omitted on purpose; the symbol names are the citation.

**S330 did not retire the older door, and said so.** `NetState`'s docs
(`crates/geom/src/surfaces/nurbs.rs`, at `NurbsSurface::net_state`):
*"`NurbsSurface::is_placeholder` remains for the callers that only ask
the placeholder question, and agrees with this by construction — both
read `net`'s one implementation."* `work/topo/S330.md`'s `## Closed`
names the exhaustive `net_state` match as check 1's mechanism and lists
the sibling two-state reads as a residue filed as a class. That class is
`work/topo/described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`
(thirteen sites; PIPE's), and its question is which ARM a poisoned net
reaches at each consumer. This row is the narrower spelling question:
a site that genuinely asks only "placeholder or not" — a filter, a
fixture count — is honest under `is_placeholder()`; a site whose other
arm is *described* (`!payload.is_placeholder()` read as "real geometry")
is the pipe row's business and reads `net_state()` when that row's lane
reaches it. What is asked here: decide, once, whether the two-way door
stays for the sites that only ask its question — then each such site
says so where it stands, or the door's docs do — or whether every
consumer reads `net_state()` so the exhaustive match is the one shape a
reader meets and `is_placeholder()` retires from `NurbsSurface` (its
curve twin with it).

## Home

Filed on TOPO's slate because TOPO's review found it (R2 on `D263`,
PR 2548: the merge door spells the question one way and its
neighbours the other). The door itself, `crates/geom/src/surfaces/
nurbs.rs`, is PROPS' (`work.py territory`), and the `topo` sites that
spell it the old way are CURVED's, TRIM's, S-CERT's and SHELL's ground
per `work/topo/program.md`'s `keep_out` — so the decision is PROPS' to
take and this row moves there by `git mv` the day PROPS claims it.
Sweep: `is_placeholder()` and `NetState::Placeholder` over `crates/`,
which cannot match a site composing `net::is_placeholder` or
`net::any_poison` directly (`geom`'s own `net.rs`, the implementation,
is excluded).

## Staleness audit (2026-11-22, WALKS, origin/main 7709b0ea)

Still live; about 27 production `is_placeholder()` sites now against the row's ~20 (new: clearance, props/quad_lane, ring_path spline_wall and sector_face, geom-brep dihedral_side, mesh chart_table, step-export writer, step-import build; gone from pcurves, props top level, pcurve_cache, mesh trimmed, step-import adopt). The described-net row the `!is_placeholder()`-as-described sites deferred to is closed, so those sites (geom-brep certify.rs, step-import build.rs, geom's `matches!` guards) have no other owner. Re-priced M; weighed as fork F2.
## The question to Ev (2026-11-22, fork F2)

Two designers weighed it independently and converged: the two (in fact four) doors exist only because "this face has no surface yet" is stored as an in-band all-NaN NURBS net. Their recommendation: tag the body's surface SLOT in `topo` (undescribed | described `Surface`), retire `NetState`, `net_state`, both `is_placeholder`s, `Surface::is_placeholder_chart` and the placeholder constructors, and leave one finiteness predicate (NaN and ±∞) as the only question asked of a net. That reverses geom's crate-doc "three states" doctrine and possibly an earlier retirement of a unit variant, and is moderately hard to reverse, so it goes to Ev. The curve placeholder (no producer outside tests) is deleted regardless, by `walks/curve-placeholder`.

## Decided (Ev, 2026-11-22, PR 7423)

"cool. yes it seems good to have some kind of 'in progress surface' enum that can be a real surface or a placeholder." The enum is the body's surface SLOT in `topo` (described `Surface` | pending), not a variant of geom's `Surface`: one typed refusal where a face's surface is handed to geometry; `NetState`, `net_state`, both `is_placeholder`s, `is_placeholder_chart` and the placeholder constructors retire; one finiteness predicate (NaN and ±∞) is the only question asked of a net.

- 2026-11-23 — `walks/curve-placeholder` (PR 7433) takes the curve half: `NurbsCurve3::placeholder`, `NurbsCurve3::is_placeholder` and `Curve3::nurbs_placeholder` are deleted with every reader arm (`topo::transform`'s `map_carrier`, `step-export`'s carrier gate). The surface slot Ev decided above is still to build, so the row stays open for it.
