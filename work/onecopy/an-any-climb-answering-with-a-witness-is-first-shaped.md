---
id: an-any-climb-answering-with-a-witness-is-first-shaped
kind: issue
title: A climb under Answering::Any whose answer names a witness (which gap, which vertex, which limb) reports whichever rung answered first: First-shaped under Any
status: open
opened: 2026-11-09
priority: P2
cost: M
---


Found by PR 6637's FULL review (S9).

Under `Answering::Any`, a kept rung ahead of the answer is spliced superseded: its reading would have given the same answer. That holds where the answer is a value every answering rung gives alike (a side, a bool, unit). It does not hold where the answer names the rung itself: which gap, which vertex, which limb. A rung kept ahead may truly have been the witness, so the answer depends on rung order, and the climb is `First`-shaped under `Any` (`Answering::First`'s doc, the WHICH rule). Each such site needs one of three things:
- answer a witness-free value under `Any`, carrying the witness as a diagnostic only;
- climb under `First`;
- state why which witness is reported does not matter to its consumer.

**Every `climb(Answering::Any, …)` whose answer is neither unit nor `bool`** (`onecopy/refusal-standing` at PR 6637's head; tests excluded):

| site | answer | witness? |
|---|---|---|
| `topo::replace_face::gaps_apart` | `(index, gap)`; `read_ends` reads the vertex off the index into `VertexDisagreement { vertex, gap }`, `corner_apart` the gap | **yes** |
| `topo::census::wedges_overlap` (≈2558) | `Ok(i)`, the overlapping pair's index | **yes** |
| `topo::census` vertex probe (≈9046) | `Ok(v)`, the vertex found inside | **yes** |
| `geom_brep::edge_nurbs` analytic rung 3 (≈2137) | `AnalyticRung3Refusal::Limb { operand, margin }` | **yes**: names the end decided off and its margin |
| `topo::shell::walls_cross` (QML T2) | the crossing piece's overlap, named in the refusal | **yes**: the overlap names which piece crossed |
| `topo::boolean::rim_wedge::rims` (≈165, with `Any` from `shares_rim`) | `Option<(Rim, MarginDiag)>`, read as `is_some()` at that caller | the rim is a witness, discarded by that caller; check the other callers |
| `topo::offset_nappe::face_nappe_reading` | `Nappe` | no: the two answering rungs are exclusive (`station_min ≤ station_max`) |
| `topo::splitting::finish` (≈2102), `topo::ray_walk` (≈175), `topo::stands` (≈223), `topo::side_reader` (≈656), `topo::one_region` (≈251) | a side or verdict | no, where the premise that every rung reads one fact holds (`Alike` once [ev] 6592 lands) |
| `topo::chord_join` (≈3988, ≈4064, ≈4108, ≈4849, ≈5482) | a sign, a parity, a `RingSide`, or the caller's `R` | no for the sign and side readings; ≈3988's generic `R` to check |
| `topo::link::Sector::place` | `Place::Inside` | no |
| `topo::splitting::pieced_arc::contact` (≈118) | `ArcHit::End` | no (one value) |
| `topo::census` meeting probes (≈11266, ≈11378, ≈11497) | `bool` through `meeting_found` | no |
| `topo::census` (≈11663, `climb_evidence`) | to classify | — |

The list was taken by grepping `Answering::Any` and reading each site's answer; a site that spells the `Answering` through a variable is not on it (`rim_wedge::rims` was found only through its caller).
