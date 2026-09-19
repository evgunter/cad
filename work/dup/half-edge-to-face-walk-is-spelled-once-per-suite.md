---
id: half-edge-to-face-walk-is-spelled-once-per-suite
kind: issue
title: The half-edge → loop → face walk is spelled once per file across topo, sweep and their suites
status: open
opened: 2026-09-19
---


## Finding

- **Where**: `crates/topo/src/shell.rs` (`offending_face`'s `face_of_he`
  closure) and `crates/topo/src/replace_face.rs` (`edge_faces`'s
  `face_of` closure) — **byte-identical closure bodies under two
  names, both in `topo/src`**; `crates/topo/src/query.rs`
  (`face_kind_across`), `crates/topo/src/splitting/join.rs`
  (`he_face`), `crates/sweep/src/blend/surgery.rs` (`face_of_half`),
  `crates/sweep/src/blend/battery.rs` (`face_of`),
  `crates/topo/src/test_support_fixtures.rs` (`face_surface_of_he`,
  the one PR 2842 folded four spellings onto), and ~49 more files.
- **Importance**: medium
- **Confidence**: sure. The two `topo/src` closures were read against
  each other; `query.rs`, `join.rs`, `blend/surgery.rs` and
  `blend/battery.rs` were read individually. The tail was measured, not
  read.
- **Raised by**: the PR 2842 review, recorded by that PR's fix pass,
  2026-09-19.

One walk: half-edge → its `parent_loop` → that loop's `.face`, and in
about half the spellings one more hop to `.surface`. It is three lines
and it is written out per file.

## The measurement (2026-09-19, at PR 2842's head)

A regex for the SHAPE — `get_loop(… parent_loop …) … .face`, across
line breaks, over every tracked `.rs` file — matches **56 files**:

| bucket | files |
| --- | --- |
| `topo/src` | 15 |
| `topo/tests` | 2 |
| `sweep/src` | 4 |
| `sweep/tests` | 31 |
| `editor-core/src` | 2 |
| `mesh/tests` | 1 |
| `step-export/tests` | 1 |

`parent_loop` appears on 335 tracked lines in all, so the walk is the
bulk of every read of that field.

**The class is not uniform, and the fold is not one function.** Three
error postures are in use and they are not interchangeable:

- `Option`-returning (`shell.rs`, `replace_face.rs`,
  `blend/surgery.rs`, `blend/battery.rs`, `query.rs`) — a stale key is
  an honest `None`;
- typed-`Result`-returning (`splitting/join.rs`'s `he_face` →
  `SplitJoinError`, `sweep/src/swept.rs`'s `face_surface_key` →
  `EulerOpError::StaleKey`) — a stale key is a refusal;
- `unwrap`/`expect` (most of `sweep/tests`) — a stale key is a test
  failure.

So the unit is one `Option`-returning door on `Body` with the
`Result` spellings as thin wrappers over it, not a single signature
imposed on 56 call sites.

## Why this is filed and not closed

PR 2842 folded **four** spellings of `face_surface_of_he` onto one
exported function and its body called that *"what the row asked for"*.
The row it cites is a **name** census, and a name census closes name
collisions. This class is the opposite shape — **one thing under many
names** — so four folded spellings is a half-fix of it, and PR 2842's
body is corrected to say so.

## Cheapest next pair

`crates/topo/src/shell.rs`'s `face_of_he` and
`crates/topo/src/replace_face.rs`'s `face_of`. Both are in `topo/src`,
both are `Option`-returning, both are local `let`-bound closures over
`body`, and their bodies are **byte-identical**:

```rust
|he| -> Option<FaceKey> { Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face) }
```

Neither was disclosed by any census in S-DUP's links 1–3. Folding them
onto one `pub(crate) fn` in `topo` costs two call sites and settles the
signature question for the rest of the class.

## What the instruments could not see

- The regex reads text, so a walk split across a helper boundary (`let
  l = …parent_loop; … get_loop(l)?.face` more than a few lines apart)
  falls outside its window; the 56 is a floor.
- It cannot separate "this file spells the walk" from "this file spells
  it four times", so the file count is not a call-site count.
- It says nothing about the `.surface` hop, which about half the
  spellings carry and which decides whether one door serves both.
