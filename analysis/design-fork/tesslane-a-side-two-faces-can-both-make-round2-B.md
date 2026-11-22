## For Ev

**Where I land now (likely): A's place, C's cure.** The side is divided once, at the join, in every patch that uses it as an interior side except one. The cut point is the 3-D midpoint of the side. No certificate is needed and no lane is re-entered. Two separate fixes go with it: the chord-pass floor from C, and a fold cure.

### What moved me
- **C's measurement shows my first claim was wrong.** I claimed the seven unsplit bored-ball cells would mesh under my answer. They would not:
  - The bore's straight seam and the outer face's arc seam join the same two vertices, and each is one chord, so both are the segment (0, 1).
  - The census allows two uses per chord, so four uses pass it, and `check_mesh` refuses.
  - C's probe output shows it: with the side cure alone, all 7 cells return `Ok` and fail `check_mesh`. With the chord floor added (an arc that shares its two end vertices with another edge takes at least two chords), none fail.
  - That is a chord-pass defect, separate from the side question, and the fix belongs in the chord pass. (sure)
- **The walk's reach test makes the mesh depend on ε.** I missed this and confirmed it in the tree. The pinned threshold in `split_rulings::a_side_whose_reach_the_band_cannot_decide_is_cut` is a different δ for each ε row. That goes against your "No ε is read" ruling. It is one more reason to drop the test. (sure)
- **Splitting at the chord midpoint needs no certificate.** The four new triangles are pieces of two triangles that already passed, lying in the same planes. So the δ promise holds without re-running anything. My answer re-entered each lane with an avoid list and ran in rounds. That needed:
  - a re-entry point on every lane;
  - the avoid list folded into the memo key;
  - a loop that can repeat, because a rebuild can flip other sides.
  The join-side split needs none of those. I drop my rounds. (likely)
- **C's counts make the over-cut concrete.** Today the walk makes 1,211 cuts where 406 are needed. The triangulating arms make 103 rebuilds inserting 997 points, where about 502 shared sides exist. And no lane rule is needed to mesh the default suite. (sure, as C's numbers)

### What did not move me
- That no lane reads its neighbours, and that the fact is decided at the join: all three of us agree, and I still hold it. (sure)
- The memo-key premise correction: all three found it (the identity is already folded), and that the walk reads the neighbour's surface, which the key does not fold. (sure)

### Is my first argument answered?
- *No lane predicts; decide at the join:* shared by all three, so it stands.
- *The rectangle lane can rebuild:* now moot, since no lane rebuilds.
- *Re-run with an avoid list:* answered and dropped (above). A's single pass without rebuild, and C's split with no lane involved, are both simpler.
- *Both faces give way:* answered (below). I was unsure of it, and I change it.
- *The seven cells mesh:* wrong (above).

### The three differences, as I now recommend
1. **The cure: a split at the chord midpoint, at the join (likely).**
   - The alternative is A's split at a surface point, certified. That parts the two sheets on curved sides. Nothing promises a mesh free of self-contact, though, and the sheets already share the segment before any split.
   - A surface point can fold or fail its certificate. C's needle cells already refuse `FoldedTriangle` without the lane rule, though with the shared-side insert switched off generally, not specifically with a surface-point split.
   - The chord midpoint keeps δ by construction, and on planes and rulings it is the same point anyway.
   - Choose A's version only if you want the sheets parted.
2. **Who gives way: all but one patch (likely).**
   - With midpoint splits, "every interior user" would put two private vertices at the identical position, one in each patch. That is pointless, and it costs two extra triangles per conflict.
   - The keeper: a patch whose own boundary carries the side, where one does. Otherwise the first in arena order. That tie-break means nothing beyond determinism (D9), but it is harmless: the split happens after the memo, so no key reads it. I am unsure whether you would rather pay the two triangles than have a meaningless rule.
3. **What goes beside it: both of C's companions (likely).**
   - The chord floor. Without it, the seven cells move from a typed refusal to `Ok` with a four-use edge that only `check_mesh` sees. That is worse.
   - A fold cure. On the needle cone, the shared-side insert was hiding folds.

### Text
- `lib.rs`'s purity sentence and `memo.rs`'s per-lane list lose the face across each edge. That clause was written by an agent (commits `4e30668774` and `6113bc3d1b`), not in your words.
- No README design clause changes.

## For the orchestrator

- **Errors I see in report A:**
  - It says its answer meshes the 7 cells. It does not without C's chord floor; C measured `check_mesh` failing on (0, 1).
  - It says a split certificate "is expected to pass". That is reasoned, not run. C's needle cells are the nearest evidence, and they fold.
  - Its rule "every patch with the side inside splits" leaves two coincident private vertices if it is ever paired with a midpoint split.
- **Errors I see in report C:**
  - Its answer E says a re-meshed face "becomes a function of its neighbours' patches, so the memo cannot key it alone". That is not so: the avoid list is a declared input the key can fold, as my first report did. The real costs of E are re-entering every lane and the rounds, which is reason enough.
  - It says a face the join cut "reports no stored-patch id". Keeping the split as join data applied after placement would let the pick index key the stored patch plus a per-face split list. That is worth a line in the build unit.
- **Unbuilt and unrun, flagged by C:**
  - the branch for an overused side that a chord carries;
  - the keeper rule (C's probe kept the first in arena order);
  - the ε 10⁻⁶ and 10⁻¹² rows;
  - the slow suite and other crates.
  A build unit should run these.
- **Who files what:**
  - The chord-floor case belongs beside `two-coincident-edges-between-one-vertex-pair-mesh-non-manifold`, as its own row: the body is not pinched, only its coarse chords are.
  - The fold cure goes with `a-needle-funnel-cut-at-its-apex-refuses-folded`.
- **Still standing from my first report:** the stale comment above `joined()` saying no lane-made body reaches the census.
- **This round:** I ran nothing. I read C's `probe.diff`, `cure.txt` (7 `check_mesh` errors) and `cure_floor.txt` (0), and the ε-row thresholds in `split_rulings.rs`.
