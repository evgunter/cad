---
id: the-guarded-shell-list-of-a-solid-is-spelled-eleven-times-outside-topo-src
kind: issue
title: The guarded shell list of a solid is spelled eleven more times outside crates/topo/src, all test-side
status: open
opened: 2026-09-20
refs: [the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times]
---

## Finding

- **Where**: eleven sites in `crates/sweep/tests` (7),
  `crates/topo/tests` (2) and `crates/editor-core/tests` (1), listed
  below.
- **Importance**: low
- **Confidence**: sure. The census is an enumeration over a required
  atom, not a search over a shape.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20.

`Body::shells_of_solid` now exists. These eleven sites still resolve a
solid by key and reach into [`Solid::shells`] themselves:

| site | spelling |
| --- | --- |
| `sweep/tests/shell5_r1_probes.rs` (~:539) | `.unwrap().shells.len()` |
| `sweep/tests/shell5_r2_probes.rs` (~:250) | `.expect("the thin solid").shells[0]` |
| `sweep/tests/shell5_r2_probes.rs` (~:337) | `&.expect("a thin solid").shells` |
| `sweep/tests/shell5_r2_probes.rs` (~:386) | `.expect("the minted solid").shells` |
| `sweep/tests/shell8_common.rs` (~:225) | `.unwrap().shells.clone()` |
| `sweep/tests/shell9_r1_probes.rs` (~:214, ~:387) | `.unwrap().shells.len() == 2`, inside a `find` |
| `sweep/tests/shell10_r2_probes.rs` (~:142) | `.unwrap().shells[0]` |
| `topo/tests/graft_disjoint.rs` (~:237) | a `get_solid(k)` whose window reaches `.shells` |
| `topo/tests/void_door.rs` (~:308) | `.unwrap().shells.len()` |
| `editor-core/tests/asm_roots.rs` (~:518) | `if let Some(s) = body.get_solid(solid)` then `s.shells` |

## The census that produced it, and its denominator

Every `get_solid(` occurrence in every tracked file, no path argument —
**41** at `cd9fdfd6b` — classified by whether `.shells` appears within
eight lines: **34 do**. Of those 34, one is `Body::get_solid`'s own
definition (a window over-fire onto the arena read two lines below),
one is a row of this program's own prose, **21 are in
`crates/topo/src`** and are the fold's subject, and these **11** are
the remainder.

The atom is closed, not asserted: `Solid` has exactly **one** field
(`entity.rs`, `pub shells: Vec<ShellKey>`), so a caller that resolves a
solid and reads anything reads this; no `Solid::shells()` accessor
exists (`git grep 'fn shells'` finds `Body::shells`, the arena
iterator, and one `sweep/tests` helper); no site in the tree
destructures `Solid { shells, .. }` as a pattern; and **no site builds
one solid's shell list by scanning the shell arena on the `Shell::solid`
back-pointer** — 344 `.shells()` calls, 44 with a solid token within
four lines, every one of them an arena COUNT rather than a per-solid
selection. What the enumeration cannot see is a member assembled by a
macro, which is unfalsifiable by text.

## Why this is filed on dup, and why it was not folded

`crates/sweep/tests` and `crates/topo/tests` are S-TCOST's and
S-TINT's ground in every crate; `crates/editor-core/tests` likewise.
The `faces_of_solid` fold that preceded this one left its three
`sweep/tests` members to `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md`
for exactly this reason, and a sibling lane was live in
`crates/viewer/tests` while this one ran.

It is filed here rather than split across three tint rows because the
subject is one duplication class with one home, and that home is a
`topo::Body` door. A tint or tcost lane that would rather own it
should move the file, per `work/README.md`'s one-file-one-item rule.

**None of the eleven is exempt.** The reference-walk class that keeps a
member hand-written — a row that is a door's own reference, or that
asserts an ownership list and its back-pointer against each other —
has **six sites in four rows**, every one of them in
`crates/topo/src` (`body.rs` ~:1411, ~:1488 and ~:1494; `euler.rs`
~:2825; `movefac.rs` ~:490 and ~:491 — see the parent row).
These eleven are plain callers.
