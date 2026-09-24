---
id: the-same-solid-two-shell-body-is-hand-built-three-times
kind: issue
title: The same-solid multi-shell body is hand-built four times in topo/src, and the newest copy is a candidate home
status: closed
opened: 2026-09-20
priority: P4
cost: E
closed: 2026-09-24
branch: dup/topo-fixture-batch
pr: 3152
---



## Finding

- **Where**: `crates/topo/src/euler_ring.rs`'s `fused_two_shell_body`
  (~:2752–2763), `crates/topo/src/body.rs`'s
  `faces_of_solid_answers_arena_order_where_the_shell_walk_would_not`
  (~:1453–1470) and `crates/topo/src/validate.rs` (~:6912–6921).
- **Importance**: medium
- **Confidence**: sure. All three were read against each other.
- **Raised by**: the style review of PR #2898's fix pass, 2026-09-20.

**A solid with two shells is not constructible through the public
operators** — `mvfs` mints one solid per shell — so every row that
needs one writes the arena by hand. The first two do it with the same
three statements, in the same order, differing only in local names:

```
get_shell_mut(<second>.shell).unwrap().solid = <first solid>;
get_solid_mut(<first solid>).unwrap().shells.push(<second>.shell);
get_solid_mut(<second>.solid).unwrap().shells.clear();   // euler_ring
body.solids.remove(..); body.solid_provenance.remove(..) // body.rs
```

The last line is where they diverge, and **both spellings have a
cost**: `euler_ring`'s leaves a shell-less solid behind
(`ValidationError::SolidWithoutShells`, pass 9), and `body.rs`'s must
pair the arena removal with its provenance removal the way `kvfs`
does, or it leaks a `LeakedProvenance` entry instead. A shared fixture
is where that pairing gets written once.

The third (`validate.rs`) takes the other route — `add_shell` onto an
existing solid, then move a face's back-pointer — and is deliberately
arity-broken, so it is a sibling rather than a copy.

**The duplication is self-declared at both copy sites.**
`euler_ring`'s carries *"not constructible through the public
operators (`mvfs` mints one solid per shell), so the second shell is
re-homed by raw in-crate write"*; `body.rs`'s says the same thing in
its own words. That is the prose census instrument this program
already owns (method item 10) finding a copy no structural regex
reached, and it found it **in a fix pass whose subject was
duplication, in the file whose duplication row that pass closed**
(`listing-a-solids-faces-is-spelled-four-times-in-topo-src`). The
lane's own X4 self-report covered the walk under test and not the
fixture around it.

## What the fix has to decide, which is why it is not that unit's

A shared `fn same_solid_two_shell_body()` would have to live somewhere
all three modules reach — `fixtures.rs` is the obvious candidate and
already carries `ngon_pillow`, `pillow`, `raw_prism`, `ops_*`. But the
three rows want different bodies:

| site | base body | what it needs afterwards |
| --- | --- | --- |
| `euler_ring` | `ops_pillow()` | two `MvfsCreated` handles, for `kfmrh`'s two face arguments |
| `body.rs` | `pillow()` | a face MOVED between the two shells, so the shells interleave with the face arena |
| `validate.rs` | `pillow()` | the arity floor still broken, so pass 9 fires |

So it is a parameterised fixture or it is nothing, and deciding its
parameters is a `fixtures.rs` layout question rather than a fix at any
one call site. `euler_ring.rs` and `validate.rs` are also not the
`faces_of_solid` unit's ground.

## Why this is filed on dup

`scripts/work.py territory` leaves `crates/topo/src/body.rs`,
`euler_ring.rs` and `validate.rs` unclaimed. The subject is the
duplication class and the decision is a shared-fixture home, which is
this program's charter. Cross-referenced from
`work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`.

## 2026-09-20: a fourth copy was minted and folded back, same file

`Body::shells_of_solid`'s reference row (`body.rs`) needed the same
body and hand-built it again — the four raw writes, **the
`solids.remove` / `solid_provenance.remove` pair included, and the
comment explaining the pairing dropped**. Forty lines below the copy
this row already names, in a unit about one thing spelled *n* times,
found by the reader rather than the lane. It is the copy this row's
last paragraph predicts: prose at the copy site is what found the
first three, and a copy that says nothing about itself is invisible to
that instrument.

Folded at once, and **not** by anticipating this row's decision: both
`body.rs` copies now call a LOCAL `adopt_shell_into(body, solid,
minted, at)` in that file's test module, which carries the pairing
comment and a pointer here. The class is still **three sites** —
`euler_ring.rs`, `body.rs`, `validate.rs` — and `body.rs`'s is now one
named block instead of two open-coded ones. Where the shared fixture
lives, and with what parameters, is still the table above.

## 2026-09-24: a FOURTH site landed on main — the class is four, not three

`crates/topo/src/tier3_tests.rs`'s `refile_shells(body, donor, keeper)`
arrived after this row was written and is the same construction
generalised: every shell of `donor` re-homed under `keeper` by raw
write (back-pointers, then `keeper`'s list), `donor`'s list cleared,
and the arena removal PAIRED with its provenance removal. It is the
closest thing in the tree to the shared fixture this row asks for — an
all-shells `adopt_shell_into` — which makes it a candidate home, not
only a fourth copy. Found by the `Body::shells_of_solid` unit's re-sweep
at its merge with main; recorded, not folded, because the home is this
row's decision.

## Not measured

Only `crates/topo/src` was read. Whether `crates/*/tests` or the cargo
roots outside `--workspace` hand-build the same body is unmeasured;
three is a floor for one crate. The instrument that found it was prose
at the copy site, not a structural match, so a fourth copy that says
nothing about itself would not have been found this way either.

## Re-census at the merge base (2026-09-24, `dup/topo-fixture-batch`, `6db5b87f2`)

**Instrument 1, the required atom**: every write of a shell's
back-pointer, `git grep '\.solid = '` over every tracked file, no path
argument. Eight hits: `body.rs`'s `adopt_shell_into`,
`euler_ring.rs`'s `fused_two_shell_body` (and the two corruption
writes in `kfmrh_refuses_a_dead_shared_solid`, which point both shells
at a dead key), `tier3_tests.rs`'s `refile_shells`, `validate.rs`'s
`face_and_shell_back_pointer_mismatches_are_reported`, and production
`boolean/combine.rs` and `movefac.rs`. **Blind spot**: a shell born
already pointing at the keeper (`add_shell(Shell { solid: .. })`),
which writes no `.solid =`.

**Instrument 2, at that gap**: every `shells.push(`/`extend(`/
`insert(` over every tracked file. Outside `topo/src` it returns
nothing; inside, every hit is production, a raw-family fixture
building its one shell, or a corruption row that `add_shell`s a NEW
shell — empty, or holding faces moved out of the body's own — to
provoke the error it names: `validate.rs`'s tier-1/tier-2 rows,
`review_m1_pr5_internal.rs`'s round-robin torn body,
`review_m0_pr7.rs`'s and `review_m1_pr1.rs`'s doubly-owned faces, and
`euler_kill.rs`'s `kvfs_rejects_extra_shells_and_rings`. None re-homes
an existing shell, which is what the three members do.

**The class is three definitions over six uses**: `adopt_shell_into`
(two rows), `fused_two_shell_body` (two rows), `refile_shells` (three
rows). The row's `validate.rs` site is the corruption family above: a
sibling, as the row said, and not folded — each of those rows builds
exactly the broken state its assertion names.

**The three differ, and the difference was measured, not assumed.**
`adopt_shell_into` inserts at a chosen position and removes the donor
with its provenance; `refile_shells` appends and does the same;
`fused_two_shell_body` appends and LEAVES the emptied donor standing.
Folded onto `refile_shells` as the row proposed: position is decided
by which solid keeps (the `shells_of_solid` row now refiles the pillow's
shell under the minted solid, so the minted solid's own shell is first
— the same list-against-arena order it asserts). The PR's plant V2
restores the leave-it-standing spelling inside the shared body and
reads which rows see it.

## Closed (2026-09-24, PR #3152)

`fixtures::refile_shells(body, donor, keeper)` is the one spelling,
with the pairing comment; `body.rs`, `euler_ring.rs` and
`tier3_tests.rs` call it. The PR body carries the plant table.
