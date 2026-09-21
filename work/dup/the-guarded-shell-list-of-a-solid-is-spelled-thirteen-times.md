---
id: the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times
kind: issue
title: Body::shells_of_solid is the missing door: the guarded shell list of a solid is spelled thirteen times in topo/src
status: open
opened: 2026-09-20
priority: P1
cost: D
---



## Finding

- **Where**: thirteen sites in `crates/topo/src`, listed below.
- **Importance**: medium
- **Confidence**: sure. All thirteen were read.
- **Raised by**: the style review of PR #2898 (the `faces_of` fold),
  2026-09-20.

`Body::solid_of_face` exists; `Body::faces_of_solid` was minted by PR
#2898. **The shell-side door between them does not exist**, and its
shape is written out thirteen times: resolve a solid, refuse if it does
not, take its shell list.

### (a) The guarded shell LIST — 8 sites

`get_solid(solid).ok_or(<error>)?.shells.clone()` (or `.iter()`), each
with its own refusal:

| site | refusal |
| --- | --- |
| `boolean/combine.rs` (~:475) | `ok_or_else(corrupt)` |
| `boolean/finish.rs` (~:225) | `JoinDesync { what: "operand solid no longer resolves" }` |
| `boolean/ops.rs` (~:2062) | `corrupt("re-cut solid lost")` |
| `movefac.rs` (~:259) | `EulerOpError::StaleKey { EntityId::Solid }` |
| `shell.rs` (~:1014) | `ShellError::Corrupt { EntityId::Solid }` |
| `splitting/finish.rs` (~:338) | `SplitFinishError::Corrupt` |
| `splitting/finish.rs` (~:726) | `ok_or_else(corrupt)` |
| `offset_together.rs` (~:824) | `?` on `Option` |

**Two of them are the same four-line sequence modulo the error
constructor and the accumulator's name** — `splitting/finish.rs`
(~:338–346) and `boolean/finish.rs` (~:225–233): take the shell list,
then `for shell in shells { all.extend(body.movefac(shell)?) }`.

### (b) The solid's ONE shell — 5 sites

`let [shell] = solid_data.shells[..] else { … }`, `.shells[0]`,
`.shells.last()`:

| site | on failure |
| --- | --- |
| `euler_kill.rs` (~:422) | `EulerOpError::SolidNotSingleShell` |
| `seqgen.rs` (~:1043) | `false` |
| `seqgen.rs` (~:1223) | a `.then_some` guard on `shells.last()` |
| `seqgen.rs` (~:1361) | `[0]`, unguarded |
| `review_m1_pr4.rs` (~:1611) | `false` |

The split matters for the fix: (a) is one door with a posture
question — the eight refusals are eight different types, which is the
same argument that kept `offset_together::scope_of_moves` out of the
`solid_of_face` fold — and (b) may be a second door
(`Body::the_only_shell_of`) or may be (a) plus a caller-side match.

## How it was missed, which is the part worth keeping

PR #2898's third instrument was **every `Solid::shells`-shaped field
read** (139 at `b29fe8bd1`), classified by whether a `.faces` read
followed within 8 lines (41). Against these thirteen sites, run at the
same base:

- **9 are in the 98 the `.faces` filter discarded** — they take the
  shell list and stop, so the filter that defined "list a solid's
  faces" removed them by construction.
- **4 were inside the 41** — `offset_together.rs` (~:824),
  `seqgen.rs` (~:1043, ~:1361), `review_m1_pr4.rs` (~:1611) — and were
  dispositioned only against THAT row's class ("does this materialise
  the solid's face list? no"). Correct for that row, and a bucket drop
  for this one.

**This row published 8/5 first, and it was wrong twice over.**
`euler_kill.rs` reads `solid_data.shells[..]` at ~:425 and
`shell_data.faces[..]` at ~:434 — **nine** lines, outside the 8-line
window — so it is in the 98, not the 41. And the list of five given
for "inside the 41" was **not that partition at all**: it was group
(b) above with one member swapped. Two different splits of thirteen
were presented as one, and **the coincidence that both are 8/5 is
exactly what made it read as verified**. The groups are (a) 8 / (b) 5
by SHAPE; the instrument partition is 9 out / 4 in. They are not the
same cut and neither implies the other.

**Two bucket drops, 139 → 41 → 12 named.** Method item 9 says a bucket
disposition is where a census loses things; here it lost a whole
sibling class rather than a member. The instrument was not blind — the
question it was asked was narrower than what it had in hand, and that
conclusion holds under either partition.

And `Body::faces_of_solid`'s own rustdoc now points a caller straight
at the undoored shape — *"a caller that needs the shells kept apart
walks them"* — without saying that walking them is written thirteen
ways.

## Why this is filed on dup, and off its territory owners' slates

The thirteen sites sit on **boolean/curved** (`combine.rs`,
`finish.rs`, `ops.rs`), **shell** (`shell.rs`, `offset_together.rs`),
**splitting** and unclaimed ground (`movefac.rs`, `euler_kill.rs`,
`seqgen.rs`, `review_m1_pr4.rs`). No one owner holds a majority, the
subject is the duplication class rather than any one file's behaviour,
and the decision it asks for — mint `Body::shells_of_solid`, and
whether (b) is a second door — is a `topo::Body` door question. A lane
opening any one of those files will not see this row; it is
cross-referenced from
`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`.
An owner that would rather hold it should move the file, per
`work/README.md`'s one-file-one-item rule.

## Not measured here

No census was taken outside `crates/topo/src`. The same shape in
`crates/sweep/src`, in `tests/` trees or in the cargo roots outside
`--workspace` is unmeasured, and thirteen is therefore a floor for the
crate and says nothing about the tree.
