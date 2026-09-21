---
id: shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim
kind: issue
title: shell10_r2_probes restates offset_together's scope-walk fixtures verbatim, by its own module doc
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`two_boxes`,
  `moves_of`, `break_a_loop`, `SQUARE`) against
  `crates/topo/src/offset_together.rs`'s `mod scope_walks` (the same
  four, ~:960–1010). A fifth, `faces_of`, left the family on
  2026-09-20 — see below.
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

`faces_of` **has left this family**: the `solid_of_face` fold first
rewrote the walk inside both copies, and the `faces_of` unit
(`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`,
2026-09-20) then deleted both helpers outright onto
`Body::faces_of_solid`. What remains here is `two_boxes`, `moves_of`,
`break_a_loop` and `SQUARE`.

That fold left one thing for this row to collect. `Body::faces_of_solid`
refuses (`Option`), so every former `faces_of(body, solid)` call site
in these two modules now carries an `.expect`, and they collapse to one
the day this family gets a single home with a local adapter over the
door — this row's decision. **The count, the measurement date and the
argument about whether it is a duplication at all live in one place**,
`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`'s
X4 section; restating them here is the accumulation this program has a
row about (`orient-module-prose-accumulation`).

The prose disclosure is the finding's own instrument: the reviewer
brief's Q1 sweep (`rg -n 'verbatim|re-derived|ported from|mirror of'`)
would have found it at any point since the module landed, and nothing
in CI, review or the logs reads that prose.

## Why this is filed on dup and not on shell

`scripts/work.py territory` puts `crates/topo/src/offset_together.rs`
on **shell**'s ground; `crates/topo/src/shell10_r2_probes.rs` is
unclaimed. The row is filed on dup because its subject is the
duplication — one fixture family under two homes — and the decision it
asks for is *where the shared copy lives*, which is a test-support
layout question rather than an offset-together one. **A shell lane
opening `offset_together.rs` will not see this row**, so it is
cross-referenced from
`work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`;
a shell lane that would rather own it should move the file, per
`work/README.md`'s one-file-one-item rule.
