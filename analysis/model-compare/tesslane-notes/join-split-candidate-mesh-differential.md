(join-split implementer) (2), the `mesh` half. Whole suite at the default ε, seed 1, per-call digests. `sweep` is running: head, foot, then foot + pair, about 40 minutes each.

**Foot alone, against the head:**
- 91 calls move points; triangle counts and outcomes are unchanged. Totals: 46,188,750 triangles on the head, 46,188,750 with the foot.
- The calls are in the rows that exercise the split: memo split-patch 2, trimmed split-with-no-retry 2, ball under its pole 1, cone face beside its plane 24, ringless cone near apex 2, rod sliver 7, crossing rods 8, shell thinner than δ 14, walked face beside both rims 28, join same at every ε 3.
- Reds: 1 pin, plus the instrument's ε inventory.
  - `the_join_splits_the_same_at_every_eps`: its pinned mesh bytes move with the point (that move is real).
  - `the_eps_inventory_is_pinned`: tessellate.rs 3 → 4, my probe plumbing; it goes away when the probe does.

**Foot + pair, against foot alone: +5,394 triangles of 46,184,718 (+0.012%).** 82 calls move, no outcome flips, no refusals. It fires only on these rows:

| row | calls | triangles |
|---|---|---|
| rod sliver keeps no side its plane neighbour can make | 7 | +3,086 |
| trimmed: a side two faces make is split with no retry (the rod sliver) | 2 | +828 |
| memo: a patch split where the patches join names no entry (the rod sliver) | 2 | +524 |
| a cone face keeps no side its plane neighbour can make | 24 | +376 |
| the bored ball probe cells (local) | 27 | +300 |
| a shell thinner than δ meshes, split or unsplit | 14 | +168 |
| crossing rods | 1 | +36 |
| ringless cone face near its apex | 2 | +32 |
| join splits the same at every ε | 1 | +20 |
| ball less a half-space under its pole | 1 | +16 |
| walked face beside both rims | 1 | +8 |

- Per call these are large: +44% to +100%. Rod sliver δ 1e-3 1518 → 2648; ball under its pole 16 → 32.
- Every one is a fixture built to have two faces closer than δ: a sliver, a shell, a cone or rod grazing its plane, a cap under a pole.
- **No body outside that class moved in `mesh`.** Whether (c)'s 1% is read per call or over the suite, the trigger adds nothing on the other bodies.
- New reds under foot + pair, both pin moves by design:
  - `a_walked_face_whose_neighbour_holds_both_rims_shares_no_edge`: its cut counts rise, the extra splits.
  - The bytes pin, as with foot alone.

**Foot + pair + centroid:** identical to foot + pair on all 13,998 calls. The centroid point never fires in `mesh` either; nor in the class table or on the bored ball.

`sweep`'s half follows, with the per-body list.
