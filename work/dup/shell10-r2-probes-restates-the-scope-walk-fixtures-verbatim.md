---
id: shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim
kind: issue
title: shell10_r2_probes restates offset_together's scope-walk fixtures verbatim, by its own module doc
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`two_boxes`,
  `faces_of`, `moves_of`, `break_a_loop`, `SQUARE`) against
  `crates/topo/src/offset_together.rs`'s `mod scope_walks` (the same
  five, ~:960–1010).
- **Importance**: medium
- **Confidence**: sure — the duplication is **self-declared in prose at
  the copy site**. `shell10_r2_probes.rs`'s module doc says: *"The
  helpers restate `offset_together::scope_walks`'s (private to that
  module), verbatim."*
- **Raised by**: the `solid_of_face` fold, 2026-09-19.

Two in-`src` test modules of one crate carry one fixture family. The
copy site names the original, names that it is verbatim, and names WHY
— the original is private to its module — so the fix is a home
question, not a discovery question: either `scope_walks`'s helpers move
somewhere both modules reach (`test_support_fixtures` is the obvious
candidate; `quad_prism` and `graft_disjoint`, which both copies build
on, are already crate-visible), or the probe module imports them.

`faces_of` is the member of this family the `solid_of_face` fold
touched — it folded the walk INSIDE both copies onto the door and left
both copies standing, because the home decision is this row's and not
that unit's.

The prose disclosure is the finding's own instrument: the reviewer
brief's Q1 sweep (`rg -n 'verbatim|re-derived|ported from|mirror of'`)
would have found it at any point since the module landed, and nothing
in CI, review or the logs reads that prose.
