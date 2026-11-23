# REPEAT — a profile repeats part of itself (plan)

The build spec is each unit's own body; the ruling is the parent row's
(`a-loop-cannot-repeat-inside-a-profile`) and the two clauses it cites.

## The slate, in dependency order

1. **`profile-readers-read-kernel-steps-through-the-written-address-map`**
   (H, open): one map from kernel (loop, step) to written (loop, step)
   plus member, identity today. Settles the riskiest unknown: whether
   every flat reader can be routed through one map.
2. **`a-profile-piece-name-carries-its-member`** (M, open): the piece's
   name gains a member key, empty everywhere. Runs beside 1, coupled at
   `ProfilePieces::publish`.
3. **`a-profiles-loop-list-repeats-a-loop-with-each`** (H, parked on 1
   and 2): the first block, with the reach rule, the stray-read refusal
   and the lowering.
4. **`a-chain-repeats-a-run-of-its-steps-with-each`** (H, parked on 3):
   the run, and the door's lattice-state check with the count unknown.
5. **`a-ring-closes-each-k-from-its-start-point`** (M, parked on 4):
   `each k from P[k] { …, to Next }`.

Off this slate, parked on 3 or 4: the viewer half on VSEAM, the
Python half on PYDOOR.

## Exit shape

Every unit merged and the two consumer halves closed; the parent row
closes with the last. No exit criteria are set, so no walk is owed.
