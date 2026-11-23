join-split FOLD-CURE TABLE (stopping for the ruling). (a) = a point inside every folded triangle, refusing FoldedTriangle when the rounds run out; (b) = lattice-corner drop kept, an inserted point never dropped, a point for a fold with nothing droppable. Rules off, chord floor and split on; each against main at the SAME target, every call diffed.

TARGET = HEADROOM 0.5 (today's)
| run | lost | gained (refuse on main, mesh now) | renamed | tris | rounds used (faces per round index) |
| mesh 1e-9 (a) | 0 | 4: needle funnel 1, near-pole tube 3 | needle 2e-4 Uncleared→Folded | −0.001% | 0:19 1:15 2:11 3:4 4:4 5:4 6:4 |
| mesh 1e-9 (b) | 0 | 3: tube 3 | same | −0.001% | 0:17 1:7 2:7 3:4 4:2 |
| mesh 1e-6 (a)=(b) | 0 | 3: tube | — | −0.001% | 0:3 1:3 2:3 |
| mesh 1e-12 (a) | 0 | 4 | same as 1e-9 | −0.001% | as 1e-9 |
| mesh 1e-12 (b) | 0 | 3 | same | −0.001% | — |
| sweep default (a) | 0 | 0 | 0 | −0.014% | 0:10 1:10 2–6:6 |
| sweep default (b) | 0 | 0 | 0 | −0.012% | 0:10 1:10 2:6 3:10 4:10 5:6 6:6 |
(sweep: the tilted pair meshes under both, as on main at 0.5; the 6 faces reaching round 6 refuse FoldedTriangle under both, as on main: spheres_crossing beside its pole.)

TARGET = δ (UNPROVED_HEADROOM set to 1.0; your five cells + both suites' fold cells)
| run | lost | gained | tris | rounds |
| sweep tilted_sphere_pair + spheres_crossing (a) | 0 | 1: tilted pair 4e-2 (all 4 sphere faces, ONE round, 188 points each) | — | identical to (b) here: none of those folds has a lattice corner |
| same (b) | 0 | 1: tilted pair 4e-2 | — | same |
| mesh 1e-9 (a) | 1* | 4: near-pole tube 2 (incl. 5e-3), needle funnel 1 (+ split_rulings/rod cells) | −0.002% | 0:24 1:20 2:13 3:13 4:9 5:2 6:2 |
| mesh 1e-9 (b) | 1* | 5: tube 2, needle funnel 2 | −0.002% | 0:16 1:12 2:7 3:7 4:7 |
*the same one under both, and not a fold: reach_probes::crossing_rods_keep_no_side_a_neighbour_can_make refuses CertificateExceeded at δ target with the shared-side insert off (main at 1.0 meshes it because the insert happens to shorten a triangle). A certificate refusal at 1.0: per your note the sizing unit's, but it appears because the insert retires, so you may want it filed against this unit too.

Reading: at 0.5 (a) gains one cell (b) does not (needle funnel at 1e-9/1e-12); at δ (b) gains one needle-funnel cell (a) does not, and (a) spends more rounds there (2 faces reach round 6 and refuse FoldedTriangle; (b) stops by round 4). Neither loses a body to a fold at either target; both clear all five of your witness cells. Neither is clearing the tilted pair by the target's luck: at δ both take it in one round.

The head: (a) is built and committed (4922edde4a), before retirement 1. Main merged (bb90c60104 + later); the merge moved main's new spline-wall digests under retirement 1 only (spline faces lose the arm's needless insert, no split, check_mesh and volume unchanged), re-pinned in 0a0c273e2e with a base run. Retirement 2 is coded and held uncommitted. Which cure?
