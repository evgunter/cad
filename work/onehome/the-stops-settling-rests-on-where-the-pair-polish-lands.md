---
id: the-stops-settling-rests-on-where-the-pair-polish-lands
kind: issue
title: The curve-pair walk's stop settles or refuses by where its Gauss–Newton pair polish lands, a heuristic stationary point
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Filed by ONEHOME (2026-11-21), closing
`work/onehome/the-curve-pair-walks-stop-verdict-rests-on-which-heuristic-foot-it-polishes.md`.
This is a question of refusal, not of soundness.

**What the stop reads.** The certified walk's stop
(`crates/geom-brep/src/curve_pair/walk.rs`, `meet` and `read_polished`)
polishes the pair from its two pieces' middles by Gauss–Newton held to
the spans (`polish::pair_held`), and reads that pair under the window
its pieces make. The window's bound is sound at any pair the polish
returns (`pose`'s module docs, "The gap's lower bound"):
- it expands about whatever `(p, q)` the polish gave, and the part of
  the gap a move could still take up is the slack `σ_w`;
- the walk's span is each piece's run from the polished parameter;
- a gap in the band at the pair is a meeting "with no premise on the
  polish".

**What rests on the polish.** Whether the window settles does. A pair
the polish leaves off the pieces' least distance carries a larger slack
and a longer run, so the basin (`curve_pair_walk_basin`), the move or
the floor can fail where they would hold at the least pair. The pair
then escalates or refuses where a better landing would decide. Gauss–Newton
from the middles is a heuristic stationary point, so the same pieces
can settle or refuse by its arithmetic.

**Measured.** On the three sweep rows the closed item names, all 743
stops polish to within `4.2·10⁻¹⁴` m of their pieces' least distance (a
`201 × 201` grid refined by Gauss–Newton held to the pieces). Holding
the feet to the pieces, or taking the whole carrier's, changes no
outcome. The probe is on the unmerged branch `onehome/stop-foot-probe`.
No counter-draw is known: one would be a pair whose polish from the
middles lands on a non-minimal stationary pair, a concave chord against
a circle the obvious shape to try.

**The guard.** `walk::tests::every_stop_polishes_to_its_pieces_least_distance`
draws the stops' three carrier pairs in one plane about a unit circle
and checks every stop against a dense oracle. It is red where the stop
reads its pieces' middles unpolished.

**Open.** Whether the stop should read a pair the window's premise
certifies, the pieces' least, or settle from any stationary pair, is
not decided. It is wanted only if a counter-draw turns up.

