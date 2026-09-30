---
id: chords-spells-the-half-edge-to-face-walk-twice
kind: issue
title: mesh/src/chords spells the half-edge to face walk twice
status: open
opened: 2026-09-19
priority: P1
cost: E
---


## Finding

- **Where**: `crates/mesh/src/chords.rs` (~:518 and ~:661).
- **Importance**: low
- **Confidence**: sure — both sites read.
- **Raised by**: PR #2857's fix pass, 2026-09-19, announced by seam from
  `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.

Both sites walk half-edge → `parent_loop` → `.face` by hand.
`pncad::topo::Body::face_of_half_edge` (PR #2857) is `pub`, total and
`Option`-returning, and `mesh` already depends on `topo`, so both fold
with no refusal lost. Two sites, so this is small; it is filed rather
than mentioned because a residue disclosed only in prose dies with the
directory it was disclosed in.

`crates/mesh/tests` holds five more and is
`work/tint/the-half-edge-to-face-walk-is-spelled-per-test-file.md`.

## What the instrument could not see

`git grep parent_loop` over `crates/mesh/`, read site by site. It cannot
see a walk split across a helper boundary.
