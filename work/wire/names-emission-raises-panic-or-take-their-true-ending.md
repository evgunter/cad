---
id: names-emission-raises-panic-or-take-their-true-ending
kind: issue
title: The ~135 non-test NamingError::Emission raises in names/ and eval/wire.rs each panic (row 0/4) or take their true row's ending, per D9 as rewritten on PR 5411
status: closed
opened: 2026-10-20
priority: P2
cost: H
refs: [a-kernel-defect-is-a-panic-or-a-typed-refusal-and-nothing-says-which]
closed: 2026-10-20
---


## Closed (2026-10-20)

Built in three lanes, split by file. Each PR had a FULL review and was merged:
- **PR 5503** (`emit_topo.rs`): 76 raises before.
  - 8 went to row 0, 45 to row 4 and 3 to row 2.
  - 7 stay typed because legal input reaches them, 12 because a correct kernel may reach them and no variant takes them, and 1 is a defect.
- **PR 5501** (`emit_union`, `borders`, `join_names`, `canonical`, `defer`): 2 went to row 0 and about 59 to row 4.
  - 6 stay typed because legal input reaches them, and 12 because a correct kernel may reach them.
- **PR 5502** (the rest of editor-core): 57 went to row 4.
  - 2 stay typed because they are reached, and 2 more were filed.

**Where the survivors live:**
- `emit-topo-raises-a-correct-kernel-may-reach-have-no-variant` (24 raises that need a row-1 or row-2 variant);
- `emit-rs-shared-helpers-raise-emission-for-torn-reads` (12 shared-helper raises, plus 4 on the D10 hold);
- `wirerest/a-verb-door-hands-back-any-familys-record-…` (3 raises);
- `held-out-goes-once-the-ring-tie-row-is-fixed` (parked);
- the reach rows each PR cites.

**Class rule from the reviews:** a row-4 argument holds for every K > 1 and every ε that `Tolerance` accepts, and does not rest on two independent tolerance decisions agreeing. PR 5503's first head panicked under `CAD_AMBIGUITY_K=1.5`.

Signed (WIRE orchestrator).
