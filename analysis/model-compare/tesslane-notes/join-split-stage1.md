join-split STAGE-1 REPORT (head a9e9185ae1, work in progress: instrument + tables in work/tesslane/join-split-stage1/, to be removed in stage 2)

VERDICT: no half refuses on any body (zero, every ε, mesh + sweep). But ONE BODY THAT MESHES TODAY DOES NOT under the stage-2 plan, through the fold cure, not the split. I am STOPPING for your answer (details §4).

1. INSTRUMENT. Each lane (rectangle, trimmed CDT arms incl. spline, walk, planar) offers, for every triangle side between two shared ids, the split at its chart midpoint (an apex end takes the other end's u), lifted by the lane's own surface eval, with both halves of each triangle on it read on THAT LANE's certificate (row.cert → cert::verdict), the winding read (validate::resolved_normal) and the lane's fold read. The join's census finds overused sides; every patch holding one inside (two of its triangles, not its own boundary chord) applies its offer; a failing half refuses the face typed. Switches: polygon shared-side test answers none (kills the CDT arms' insert AND the walk's cut), chord floor (two edges on one vertex pair, both one chord → curved ones take 2), fold cure (a boundary fold with no grid corner inserts its inside point), OverusedSide replaced by the split. Every tessellate call logs (test, body fingerprint, δ, outcome); baseline runs diffed against probe runs.

2. TABLE — splits applied, mesh suite (all 546 incl. slow; pins of today's cut counts/bytes softened so every body is reached):
pair (inside patches) | splits 1e-9 / 1e-6 / 1e-12 | refused cert | fold | winding | worst half/δ
plane + trim cyl | 802/802/802 | 0 | 0 | 0 | 0.780
rect sphere + walk cyl | 224/224/224 | 0 | 0 | 0 | 0.905
rect cyl + rect sphere | 184/184/184 | 0 | 0 | 0 | 0.905
rect torus + walk cyl | 184/184/184 | 0 | 0 | 0 | 0.345
rect cyl + rect torus | 180/180/180 | 0 | 0 | 0 | 0.345
plane + trim cone | 138/52/138 | 0 | 0 | 0 | 0.404
trim cyl + trim cyl | 128/136/132 | 0 | 0 | 0 | 0.471
spline + spline (the lens) | 4/4/4 | 0 | 0 | 0 | 0.000
plane + trim sphere | 2/2/2 | 0 | 0 | 0 | 0.128
Total 1,846 / 1,768 / 1,850 splits over 59 / 57 / 73 meshes. No two triangles computed different points for one side; no side left overused; 2 new points at one position (the lens, below). Bodies lost to the split: 0 at every ε. Gained: the unsplit bored ball (OverusedSide today) meshes at every δ; the rod sliver (UnclearedSharedSide today) meshes.
sweep's 131 meshing modules: default ε all (slow incl.): 276 splits (trim cyl+trim cyl 240, worst 0.497; plane+trim sphere 36, worst 0.207), 0 refused. 1e-6 and 1e-12 fast set only (the slow set is 33 min a run): 691/691 both ways at 1e-6, 0 splits; 1e-12 one red in BOTH baseline and probe (a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta — not mine, not on your list).

3. APEX, DEVELOPMENT, LENS, PICK MEMO.
- Apex: no applied split had an apex end anywhere in the suite. Over every OFFERED inside candidate (1.6 M plane, ~1.2 M curved, 13,697 meshes): apex candidates rect cone 25,061, walk cone 3,399, trim sphere 1,511, rect sphere 254, rect spindle 24, trim cone 27 — none fails its certificate (worst 0.995 δ). In the development the apex is the origin, so the developed midpoint of apex–P is the chart rule exactly.
- Development: the development's midpoint and the chart midpoint differ by up to 5.5e3 δ on wide rectangle-cone sides (0.235 δ on trimmed cone); halves certify at either. The cone certificate reads chart corners only, so both are "certified"; the chart midpoint is a choice, not a forced one.
- Lens (faces_that_bound_no_region::a_lens_of_two_short_arcs...): both spline faces split one side, each at its own chart midpoint, and the two new points are BITWISE ONE POSITION, sag 0, cert 0: the two faces are one sheet, so "a point of its own surface" cannot separate them — the four-triangle edge returns for any reader joining by position. The test passes (it checks ids). A zero-volume body; it meshes today (the insert's points also lie in that one plane).
- Pick memo (editor_core resolve/pick.rs, PickMemo::table): read, not run. It serves a per-patch table under the patch memo's StoredPatchId, asserting bit-identical placed corners, and `unreachable!`s on a triangle-count mismatch. A memo-hit patch split in one picture and not the next (a neighbour changed) reports the same id: the editor PANICS; two different split sets of equal count would serve a stale table silently. Stage 2 must report no stored id for a patch the join split (or an id naming its splits), and the memo must store a patch's offered splits so a hit can be split.

4. THE LOST BODY (why I stop). sweep tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta at δ 4e-2 meshes today (24,572 tris) and refuses FoldedTriangle with the rules off + fold cure. No split is involved. Per attempt on the four trimmed sphere faces: today attempt 0 has 134 shared-side triangles AND a boundary fold, one rebuild with 134 inserts clears both. Rules off: 133 of those 134 are boundary folds (fold cure inserts them), attempt 1 has 13 folded GRID triangles, the grid-fold cure drops corners (which can be the inserted points), 42→44 folds, budget exhausted. So on this face today's "shared-side" insert is a fold cure in disguise, and the fold cure as specced (insert for a fold with no grid corner) does not reproduce it. Same mechanism renames two refusing-today cells (mesh reach_probes near-pole tube; sweep spheres_crossing...beside_its_pole; needle funnel): FoldedTriangle → UnclearedSharedSide (the probe's cure borrows that budget; a shipped one would refuse FoldedTriangle).
Options I see: (a) the fold cure inserts for EVERY folded triangle (grid ones too) instead of dropping grid corners, on its own budget — changes today's grid-fold cure for all bodies, needs a run; (b) keep the drop but never drop an inserted point; (c) accept the loss as a refusal and file it. I can measure (a) and (b) in about 40 min if you want that before deciding.

5. WHAT THE SPEC HAD WRONG / NEW.
- Over offered-but-never-needed candidates the split is NOT safe by construction: a half would fold against its face on walk cone (1,675 sides), rect cone (57), trim sphere (297), rect torus (32), trim cone 6, rect cyl 6 (mostly flipped against the parent too: the chart midpoint sits far off a wide straight side), and 9,937 plane sides give a half below winding resolution (half of a thin triangle). None of these sides is overused anywhere measured; if one ever is, the face refuses typed. Certificate: no offered half exceeds or is unresolved.
- "1,416 splits over 59 meshes" (spec) vs mine 1,846 / 59 at default: the difference is the pins the probes did not soften.
- The memo key's face-across entry also stands in for the walk's may_make, which reads the NEIGHBOUR'S SURFACE, not just its identity — a key gap today, moot when it retires.

6. COST TO SHIP (stage 2 shape): lanes compute the offer eagerly (a patch stays a function of face and δ; the memo stores offers with the patch) — 2.8 M offered inside sides over the suite's 45 M triangles; cost to be timed. The census runs on shared ids before placement, so split points land after their own patch's interior. Then chord floor beside chords::short_loop_floors, the fold cure, the retirements.

Stopping. My turn ends now.
