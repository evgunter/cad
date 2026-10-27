---
id: a-kernel-defect-is-a-panic-or-a-typed-refusal-and-nothing-says-which
kind: issue
title: D9 says a kernel bug panics, while KERNEL_DEFECT_ENDING and eval's typed defect arms refuse one typed, and no ratified text says which a site gets
status: closed
opened: 2026-10-19
priority: P2
cost: M
design: true
pr: 5411
branch: wire/ev-kernel-defect-rule
closed: 2026-10-20
---


Returned as a question by `wire-breaks` (PR 5406), Part 1 of
`eval-handles-internal-breaks-three-ways-and-run-op-carries-node-facts-as-loose-options`.

## The two texts

- **D9 `[failure]`** (`docs/DESIGN.md`, the state-class table) says a
  state that only a kernel bug reaches MUST panic (row 4,
  `unreachable!`), and that downgrading one to a typed error launders a
  bug. Ev engaged with this text in the PR 4006 ruling (0a6a27d97d,
  fork-log row 64): the torn-body class panics, and argument misses
  stay typed.
- **`geom_core::KERNEL_DEFECT_ENDING`** (`crates/geom-core/src/predicate.rs`,
  "the one ending of a refusal that only a kernel defect reaches") and
  the eval variant docs ("surfaced typed rather than panicking") build
  the opposite: a typed refusal for a kernel defect. They were
  agent-written (ab7c781ba1, ENCL), and no Ev ratification was found.
  About 44 source files use the ending.

## Why it matters

- In `crates/editor-core/src/eval/`:
  - `NodeErrorKind::MissingSlot` (about 10 raises in `wire.rs`),
    `VerbArity`, `ProfileAnchor`, `ProfilePieces` and
    `ProfileRecordMissing`;
  - about 20 `NamingError::Emission` raises;
  - `anchor.rs:298` and the defect arms of `parts.rs`.
  
  Each is a "kernel defect" refused typed.
- Real input reaches at least one of them:
  - a union fold row naming a pierce vertex no operand table names
    reached `Emission { what: ROW_CELL_NOT_IN_TABLE }`
    (`work/cleave/a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold.md`,
    fixed in PR 5753);
  - so for a defect like that, D9's "never panics on any input" and
    "a kernel bug must panic" disagree.
- Applied as written, D9 turns all these sites into `unreachable!` and
  leaves five public arms dead. pncad-py's tags and the class census
  name all five.

## What needs deciding

Which internal breaks are a panic and which are a typed kernel-defect
refusal, stated as a rule a lane can apply site by site, and what D9
would say. That includes:
- whether a defect that real input is known to reach is row 4 at all;
- what the pncad-py boundary rule implies.
