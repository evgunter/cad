The arc-held ladder of `oneround/conic-pair-readers` (PR 7543).

- `ladder.rs`: the harness, run as an example of `geom-brep`
  (`--features test-support`, `CONIC_LADDER_OUT=<dir>`). Each pose records
  whether both spans lie in the session box (the harness's predicate, not the kernel's cover-based one: no
  coordinate past `SESSION_BOX_EXTENT`, on the spans' ends and 4095
  parameters between) and the stored phase, from which the oracle reads
  whether the seam is at the anchor. Each pose has a twin `20ε` apart
  (clear of the escalation at `10ε`) and one `1000ε` apart.
- `oracle.py`: the 60-digit judge (against the curve the stored numbers
  state); it names each refusal's row.
- `join.py base-dir head-dir`: base and head joined pose by pose. Its
  first table, the poses in the box with the seam at the anchor, is the
  unit's acceptance (0 wrong, 0 right-to-wrong); the past-box poses
  beside it carry no claim.
- `ladder/`: the tables at 1e-9 and 1e-12 on the base (PR 7510's head
  `0645d2b68` merged with the head's main) and on the final head, and
  `join-base-head.txt`.
- `suspects/`: the suspects lane's harness and its tables.
- `seamed/`: the reviewer's ladder with every section seamed a quarter turn
  off its anchor, and its tables, as the review ran them (PR 7543's first
  head and its base). No claim of this unit covers a seam off the anchor.
