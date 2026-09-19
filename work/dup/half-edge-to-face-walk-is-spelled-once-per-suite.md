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

## Re-measured 2026-09-19, and `topo/src` closed (PR: this branch)

`Body::face_of_half_edge` is the door: `&self`, `Option<FaceKey>`,
beside `Body::solid_of_face`. `topo/src` is folded onto it — **20 call
sites in 13 files** — and the 3-hop walk is now spelled **once** in
`topo/src`, in the door.

**The census was low, and by about half.** A type-directed probe
(`#[deprecated]` on `HalfEdge::parent_loop` and `Loop::face`, then
pairing the warning spans within ±3 lines over
`cargo check --workspace --all-targets`) finds **103 files / 145
walk-shaped sites** where the regex found 56 / 73. It adds two buckets
the table above has no row for — `step-import` (src and tests) and the
`sweep`/`step-export` **examples** — and it is immune to line breaks,
local renames and formatting. It over-counts where an unrelated `.face`
read sits within three lines of a `parent_loop` read, so 56/73 is the
floor and 103/145 the ceiling. A prose census (the walk described in
words, over every tracked file) adds what neither reaches: a **fourth
error posture** (`editor-core/src/names/emit.rs`'s `ok_or_else(bug)`),
a **third byte-identical twin** of the "cheapest pair"
(`sweep/src/blend/build.rs`, with a fourth in
`sweep/src/test_support.rs`), two **arena-direct** spellings
(`body.loops.get(…)`, which no `get_loop`-anchored regex can see), and
the walk written out as **prose** in `merge_faces.rs`'s doc comment.

**Two corrections to the table above.**
`sweep/src/swept.rs`'s `face_surface_key` is **not a member**: it walks
face → surface and never touches `parent_loop`. And the `.surface` hop
is not "about half" uniformly — it is 10 of 73 sites in the same
statement, 31 of 73 within 400 characters, and the concentration is
`sweep/tests` (24 of 41); in `topo/src` it was 1 of 17. One door
serving both was therefore refused: the surface hop is `get_face(f)?`,
already its own door, and the site that carries it needs the `Face`,
not the surface key.

**The plan's "`Result` wrappers over the `Option` door" is right only
where the variant is entity-agnostic.** Where a refusal names *which*
key went stale, the door cannot express it — folding
`splitting/join.rs`'s `he_face` onto it turns `corrupt_loop` into
`corrupt_he`, and the whole 727-test `topo` lib suite stays green. The
same shape is at `editor-core/src/names/emit.rs` (`DANGLING_MATE` vs
`DANGLING_LOOP`). Those sites keep their own walk; a guard now reds on
the fold.

## What remains

| bucket | 3-hop sites |
| --- | --- |
| `sweep/tests` | 25 |
| `sweep/src` | 2 (`blend/build.rs`, `test_support.rs` — byte-identical twins of the folded pair) |
| `sweep/examples` | 1 |
| `topo/tests` | 1 |

plus the 2-hop loop → face spellings (the half-edge data already in
hand) at `topo/src`'s `chord_join.rs`, `seqgen.rs` and `shell.rs`, and
every `editor-core`, `mesh`, `step-export` and `step-import` site.
`sweep` is BLEND's and `sweep/tests` is S-TCOST's and S-TINT's, so the
bulk of this row is announced by seam, not owned here.

## What the instruments could not see

- The regex reads text, so a walk split across a helper boundary (`let
  l = …parent_loop; … get_loop(l)?.face` more than a few lines apart)
  falls outside its window; the 56 is a floor.
- It cannot separate "this file spells the walk" from "this file spells
  it four times", so the file count is not a call-site count.
- It says nothing about the `.surface` hop, which about half the
  spellings carry and which decides whether one door serves both.
