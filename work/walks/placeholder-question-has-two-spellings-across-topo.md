---
id: placeholder-question-has-two-spellings-across-topo
kind: issue
title: The placeholder question is asked through two doors across topo: NetState::Placeholder and is_placeholder()
status: open
opened: 2026-09-14
priority: P1
cost: E
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
`work/pipe/described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`
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
