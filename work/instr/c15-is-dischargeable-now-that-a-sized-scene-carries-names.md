---
id: c15-is-dischargeable-now-that-a-sized-scene-carries-names
kind: issue
title: C15 is dischargeable: the canal is the first sized scene whose names separate its pairs
status: open
opened: 2026-09-10
---


Two of `tools/tess-lint/tests/baseline_census.rs`'s guards fired on the
same re-cut, and both of them fired because they were built to. The
teapot's spout became a LOFTED canal (`demos/tour/src/teapot.rs`), and
with it the corpus gained its first scene that carries a Hessian-SIZED
row AND a durable per-face NAME at the same time.

Until this re-cut the two sets were disjoint, and that disjointness was
the whole reason the join keys on the ORDINAL: only a body that arrived
from an evaluated document can hand `tess_meter::face_rows` a name
table, and no sized scene was one. The canal is: it is a `Node::Loft`
in the teapot's own recipe document, and its eight lateral walls are
`nurbs` charts, which is what makes them sized.

**`no_scene_carrying_a_sized_row_carries_a_name`** — the weaker
trigger, and it names the moment: `["teapot/teapotspout"]`.

**`no_indistinguishable_pair_is_separated_by_the_name_column`** — the
one that makes `C15` DISCHARGEABLE rather than merely reachable. Four
pairs of the canal's laterals are indistinguishable by every
`IDENTITY_COLUMNS` entry — same `nurbs` chart, same trim box, same
divisions, because they are congruent quarter-arc walls — and their
names tell all four pairs apart:

```
teapot/teapotspout faces 2/3: Lateral{loop 0, segment 2} vs {loop 0, segment 3}
teapot/teapotspout faces 4/5: Lateral{loop 0, segment 0} vs {loop 0, segment 1}
teapot/teapotspout faces 6/7: Lateral{loop 1, segment 1} vs {loop 1, segment 0}
teapot/teapotspout faces 8/9: Lateral{loop 1, segment 3} vs {loop 1, segment 2}
```

That is exactly the guard's own condition: *"`name` now tells these
pairs apart, and no `IDENTITY_COLUMNS` entry does — which is what makes
this the C15 case and not a re-key. The sweep hands the gate a durable
per-face identity here, so the join has something to key on besides the
ordinal."*

**What this issue is NOT.** It is not the work. Re-keying rule 4 over
the rows that carry a name is an INSTR unit with its own care —
the join has to stay ordinal for every row without a name, the two
regimes have to agree on a scene that has both, and the re-key
detector's own message has to say which regime it read a row under.
The demo PR that surfaced this re-cut both census assertions from a
DISJOINTNESS to a positive pin naming this scene and this issue, so the
alarm is recorded rather than silenced: the guards now say "exactly
this scene, and here is who owns it", and they red again the day a
SECOND scene joins — which is the next thing that should make someone
look.

**The scene is not the reason this sits here.** `work/instr`'s charter
says so already: *"C15 sits on this slate because the census that
detects it is here rather than because the scene is; no lane here edits
a scene."* The canal is only the first body in the corpus that made the
two columns overlap.


**Update: the same scene, re-authored, made the case sharper and
bigger.** The canal's sections became regular OCTAGONS (a circle is
rational, and at ε = 1e-12 the rational quadrature lane refused the
body's mass properties outright), and the separated-pair census went
from four pairs to twelve:

```
teapot/teapotspout faces 2/5, 3/4, 7/8, 11/12, 14/17, 15/16
teapot/teapotspout faces 6/9, 10/13
teapot/teapotspout faces 6/10, 6/13, 9/10, 9/13     <- NEW IN KIND
```

The first two lines are more of what was already here: congruent walls
within one loop. **The third line is a different claim.** Faces 6 and 9
are OUTER walls; faces 10 and 13 are BORE walls. Every identity column
agrees across them — `nurbs`, the unit trim box, 2 × 201 divisions —
while the rows carry 733 triangles against 658 and 322 grid cells
against 250. So the CSV can no longer say which LOOP a wall came from,
and the two loops are not the same amount of work.

Why the re-authoring did that, which is the part worth keeping: a
curved wall and a curved bore have different CURVATURE, so the sizing
lane gave them different divisions and `nu`/`nv` separated them. Flat
walls scale without changing shape — the bore is three quarters of the
wall and just as straight across — so both read `nu = 2` and the only
column left that separates them is `name`.

**And this is where the SECOND census assertion fired.** Those four
cross-loop pairs are exactly the ones whose two recoverable slacks
stopped agreeing (322/317 against 250/238), which is the sub-tolerance
early warning `an_undetected_swap_costs_the_gate_nothing_on_the_committed_baseline`
exists to give. The swap is still INVISIBLE to the gate — that
assertion still passes, so rule 2 is unaffected — but the margin that
made it free has gone on these four. Both pins are positive now and
name these lists, so each keeps saying something.

For the unit itself this changes no requirement and adds one test case
worth having: a scene where the named rows the re-key would cover
include pairs whose UNDERLYING work differs, not merely pairs that are
congruent. A re-key that keys those four apart changes what the gate
compares, where keying congruent walls apart does not.
