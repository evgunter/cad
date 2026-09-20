---
id: the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times
kind: issue
title: Body::shells_of_solid is the missing door: the guarded shell list of a solid is spelled thirteen times in topo/src
status: open
opened: 2026-09-20
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
followed within 8 lines (41). Of these thirteen sites:

- **8 are in the 98 the `.faces` filter discarded** — they take the
  shell list and stop, so the filter that defined "list a solid's
  faces" removed them by construction.
- **5 were inside the 41** — `euler_kill.rs` (~:422), `seqgen.rs`
  (~:1043, ~:1361), `review_m1_pr4.rs` (~:1611),
  `offset_together.rs` (~:824) — and were dispositioned only against
  THAT row's class ("does this materialise the solid's face list?
  no"). Correct for that row, and a bucket drop for this one.

**Two bucket drops, 139 → 41 → 12 named.** Method item 9 says a bucket
disposition is where a census loses things; here it lost a whole
sibling class rather than a member. The instrument was not blind — the
question it was asked was narrower than what it had in hand.

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


## One hand-written walk that must stay hand-written

`crates/topo/src/body.rs`'s
`faces_of_solid_answers_arena_order_where_the_shell_walk_would_not`
spells the `solid.shells -> shell.faces` walk out by hand. It is the
REFERENCE the door is compared against — the row asserts the two agree
as a set and disagree as a sequence — so folding it onto any door
would make the test compare a thing with itself. A lane sweeping this
class should skip it, and say so rather than leaving it undispositioned.
