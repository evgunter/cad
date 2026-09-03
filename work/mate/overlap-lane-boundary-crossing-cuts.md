---
id: overlap-lane-boundary-crossing-cuts
kind: issue
title: The overlap lane cuts only at coincident boundary vertices: boundary-crossing cuts (the D3 reach gap blocking the ef_bound_backed migration)
status: open
opened: 2026-09-01
github: 1500
refs: [1496, 969, 1063]
---

## From GitHub issue 1500

Opened 2026-09-01; 0 comments.

Filed at MATE-9's adjudication (PR #1496) as the scheduled home for the migration blocker both review arms confirmed disclosed-but-unscheduled.

**The gap.** MATE-9 implemented the region-confined `ef_bound_backed` variant the ratified unified strength calls for (`docs/MATE-4B-CROSSING-DESIGN.md`) and MEASURED it not clean: every #969/#1063/MATE-4a/MATE-5/MATE-8 certifying seat held except the straddle seat itself, which traded its crossings for a new hard `EdgeFaceOverlap` — because the overlap lane cuts an edge's dive cells only at COINCIDENT boundary vertices, so the dive cell's bounds are the edge's own endpoints (measured witness (0.45, 0.3, 0.5) = the midpoint of a cell bounded at 0/0.9), outside the verified interface. Both review arms reproduced the measurement exactly (two red rows at the measured commit, no others) and confirmed the diagnosis and the grandfather fallback as the honest, ruling-permitted call.

**The unit this issue funds**: the overlap lane learns boundary-crossing cuts — cells split where an edge crosses the counterpart face's boundary, not only at coincident vertices. That is a D3 cut-schedule redesign (doctrine-adjacent; in census.rs but not in MATE-9's scope). With it, the `ef_bound_backed` migration re-attempts under the ruling's measured-migration protocol: the anomaly pin `r2_an_unrelated_declared_pair_backs_the_ef_bound` is the mechanical guard (it reds under any correct re-attempt, exactly as it did under the measured one), and the C3/C4 grandfather note shrinks by one name.

Referenced from: the C3/C4 grandfather note (`docs/CONTACT-DESIGN.md`), `census.rs`'s module docs and the rung's site, and PR #1496's measurement record. The wider grandfather roster (vv/vf/ve sweeps, ee_bound's face-pair arms) stays unmeasured until each migration's own attempt; this issue is the first migration's blocker only.

## Home

`work/mate/` — the ground is `crates/topo/src/census.rs`, a S-MATE territory glob, and the issue is MATE-9's own adjudication residue on the declared-contact strengths S-MATE charters.
