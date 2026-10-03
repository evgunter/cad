---
id: edge-chord-len-defaults-to-one-metre
kind: issue
title: edge_chord_len's None defaults to a 1 m arm at two plane-identity sites
status: open
opened: 2026-09-01
github: 1529
refs: [1398, 501]
priority: P3
cost: E
---

## From GitHub issue 1529

Opened 2026-09-01; 0 comments.

(S-CERT orchestrator) Filed from CERT-8's sweep (PR 1398), where it was disclosed but unscheduled.

`crates/topo/src/boolean/reduce.rs:594` and `crates/topo/src/merge_faces.rs:1049` both spell `edge_chord_len(..).unwrap_or_else(T::one)`. The arm levers a plane-pair normal comparison to metres, and where the edge has no chord the meter silently becomes **1 m** — a value with no relation to the geometry, on a pair whose real features may be microns or kilometres. It is a sibling of issue 501's defect class and was deliberately left by CERT-8: the shape is different (a *length* arm on `Surface::Plane`-gated pairs — both sites are behind a plane destructuring — so no chart stretch is involved and no exported stretch bound would help).

The question the retirement must answer is whether a missing chord should default at all. Three candidates: refuse typed (the edge has no metric to compare against); use the faces' own extent as the arm; or prove `None` unreachable at both sites and take D2 row 0 or row 4. Both sites are `Margin::levered` already, so the change is local once the answer is chosen. It should ship with a scale twin — the same declared pair at 1e-3 and 1e3 — since a fixed 1 m default is exactly what a scale twin exposes. D2-addendum classification owed by the taking unit. S-CERT-adjacent ground (`topo` booleans / `merge_faces`); no live claimant.

## Home

`work/cert/` — filed by the S-CERT orchestrator out of CERT-8's sweep, and it is a scale-honesty defect on a `Margin::levered` arm owing a D2-addendum row, S-CERT's charter rather than S-BOOL's operand gates.

## Re-homed (2026-09-06)

Moved from `work/cert/` to `work/bool/` on S-CERT's exit walk PR
(#1924, its handoffs ledger; merged by Ev 2026-09-06 = ratified), before
`work/cert/` was deleted at sweep 7 of `docs/DOC-LEDGER.md`. Id, body
and header are unchanged; the directory is the claim (`work/README.md`).
The `## Home` section above naming `work/cert/` is superseded by this
line and is kept as the record of why the file was filed there.

## Re-homed at S-BOOL's exit (2026-09-16)

Moved from `work/bool/` to CURVED (its charter names S-BOOL's ceded ground and inherits at S-BOOL's exit) when S-BOOL closed (`docs/S-BOOL-EXIT-WALK.md`); the item's content, id and history are unchanged.

## One site fixed, and the two copies have diverged (PR 3532, 2026-09-30)

The `merge_faces.rs` site is fixed by PR 3532: `Body::edge_chord_len`
there now takes the two start vertices the adjacency scan already
resolved and returns `Result<T, DanglingRef>`, so a vertex or point
that does not resolve refuses naming it and no `1` stands in for the
arm. The two functions are no longer copies: `boolean/reduce.rs`'s
free `edge_chord_len(body, edge) -> Option<T>` still walks
edge → half-edges → vertices → points and folds every failure into
`None`. What remains is that file's two sites: the default at
`reduce.rs`'s declared-pair arm (`edge_chord_len(body, edge_key).unwrap_or_else(T::one)`,
~`:606`) and the `Option`-returning helper itself (~`:653`). The fix
there can follow the `merge_faces` shape (pass the resolved ends in, or
return the key that failed) and still owes its scale twin.

## The reduce.rs sites are gone (PR 3992, 2026-10-03)

`boolean/reduce.rs`'s `edge_chord_len` and its `unwrap_or_else(T::one)`
default are deleted. The maximal-faces gate now levers at the shared
edge's extent (`readback::edge_extent`, `geom_brep::edge_extent` over
the edge's certified carrier), and a lookup that fails on the way
refuses `BooleanError::CorruptOperand` with `Corruption::Edge { edge,
absence }` naming it; no length stands in. Rows:
`boolean::reduce::neighbour_extent_rows::a_dangling_curve_refuses_as_a_corrupt_operand`
(a mutant restoring a `1` default reds it), and the scale twin
`neighbours_across_a_closed_edge::a_disc_on_its_hosts_plane_refuses_as_coplanar_neighbours`
(the same pair at 1e-3, 1 and 1e3). The merge's site no longer reads an
edge length at all: it levers at the pair's reach
(`boolean::rest::pair_extent`). Nothing of this row remains open that
this branch knows of; its owner closes it.
