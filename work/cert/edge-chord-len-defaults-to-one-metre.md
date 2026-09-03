---
id: edge-chord-len-defaults-to-one-metre
kind: issue
title: "edge_chord_len's None defaults to a 1 m arm at two plane-identity sites"
status: open
opened: 2026-09-01
github: 1529
refs: [1398, 501]
---

## From GitHub issue 1529

Opened 2026-09-01; 0 comments.

(S-CERT orchestrator) Filed from CERT-8's sweep (PR 1398), where it was disclosed but unscheduled.

`crates/topo/src/boolean/reduce.rs:594` and `crates/topo/src/merge_faces.rs:1049` both spell `edge_chord_len(..).unwrap_or_else(T::one)`. The arm levers a plane-pair normal comparison to metres, and where the edge has no chord the meter silently becomes **1 m** — a value with no relation to the geometry, on a pair whose real features may be microns or kilometres. It is a sibling of issue 501's defect class and was deliberately left by CERT-8: the shape is different (a *length* arm on `Surface::Plane`-gated pairs — both sites are behind a plane destructuring — so no chart stretch is involved and no exported stretch bound would help).

The question the retirement must answer is whether a missing chord should default at all. Three candidates: refuse typed (the edge has no metric to compare against); use the faces' own extent as the arm; or prove `None` unreachable at both sites and take D2 row 0 or row 4. Both sites are `Margin::levered` already, so the change is local once the answer is chosen. It should ship with a scale twin — the same declared pair at 1e-3 and 1e3 — since a fixed 1 m default is exactly what a scale twin exposes. D2-addendum classification owed by the taking unit. S-CERT-adjacent ground (`topo` booleans / `merge_faces`); no live claimant.

## Home

`work/cert/` — filed by the S-CERT orchestrator out of CERT-8's sweep, and it is a scale-honesty defect on a `Margin::levered` arm owing a D2-addendum row, S-CERT's charter rather than S-BOOL's operand gates.
