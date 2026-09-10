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
