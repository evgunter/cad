---
id: a-doors-rustdoc-carries-an-unguarded-census-sentence
kind: issue
title: Two doors in a row carry an unguarded census sentence in their rustdoc; the tree has the machine for a guard
status: open
opened: 2026-09-19
priority: P4
cost: E
---



## Finding

- **Where**: `crates/topo/src/body.rs` — `Body::face_of_half_edge`
  (~:906) and, until this row's PR, `Body::solid_of_face` (~:876).
  The machine that is not used: `crates/topo/src/source_walk.rs`.
- **Importance**: medium
- **Confidence**: sure. Both sentences were read; `source_walk.rs`'s
  existing population gate (`DOORS_MEASURED`) was read.
- **Raised by**: the `solid_of_face` fold's fix pass, 2026-09-19.

Two `Body` doors in successive PRs shipped a **count of the tree** in
their rustdoc:

| door | sentence | what held it true |
| --- | --- | --- |
| `face_of_half_edge` | *"That is a population, not an exception. It is sixteen sites in this crate alone"* | nothing |
| `solid_of_face` | *"Every spelling in THIS CRATE that refuses uniformly across the two hops reads through here"* + an enumeration of four consumers | nothing |

Both were false or stale within one PR of being written. The second was
false at the moment it was committed (`seqgen::fusion_remake_shell`,
in the same file as two sites that PR folded, and
`euler_ring::fused_two_shell_body`, one of the eight it folded, was in
neither the claim's enumeration nor its scope). That is the class:
**a doc sentence that measures the tree and is checked by nobody.**

`solid_of_face`'s sentence has since been rewritten to claim a
*population* — the three shapes of caller that cannot read through the
door — with the measurement moved to
`work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`
and the absence of a guard stated at the claim site.
`face_of_half_edge`'s *"sixteen"* is untouched and unguarded.

## The tree already has the machine

`crates/topo/src/source_walk.rs` is an in-tree, comment-and-string
blanked, **code-only** scanner over `topo/src`, reading through
`test_utils::source::code_only`. It already hosts exactly this shape of
gate: `DOORS_MEASURED = 52`, asserted against what the walk finds, with
a message telling the next lane to move the constant. None of the five
instruments the `solid_of_face` census ran used it; every one ran over
raw text, which is why prose such as `validate.rs:100`'s
`//! shell.solid` reads as code to a grep and why the censuses each had
to disclose a line-noise blind spot that `code_only` does not have.

## What is owed

A doc measurement owes one of three things **at the claim site**:

1. a mechanical guard — a `source_walk.rs` population count over
   `code_only` text, in the shape `DOORS_MEASURED` already has;
2. a scheduled re-measure that names when and by whom; or
3. a written *"unguardable, and here is why"*.

`solid_of_face` now has (3) plus a dated row. The unit this row asks
for is (1), for both doors and for the next one: a `source_walk.rs`
walk that finds the two-hop back-pointer compositions in `topo/src` and
reds when the count moves. Its honest limits are the ones
`source_walk.rs`'s header already discloses — delegation needs a call
graph, `cfg` needs evaluation, and *"`topo/src` only"* is not the
class's full scope, since five members sit in `topo/tests` and
`sweep/tests`.

**Both of those doors' claims are about `topo/src` alone**, which is
what makes a `source_walk.rs` guard the right size for them: the guard
would cover exactly the scope the sentences claim, and the out-of-scope
members would stay where they already are, on
`work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md`.

## A third site, 2026-09-19: `sweep::test_support::block`

`crates/sweep/src/test_support.rs` (`block`'s rustdoc, ~:156):

> *"The alternative was **seven** private copies of the construction
> under one more name, which is what this replaced."*

Same shape as the two above: a number of the tree, in a door's rustdoc,
with nothing holding it. It is worse-behaved than either, because it
reads as history (*"was … which is what this replaced"*) while sitting
where a reader takes it for the door's population — and the population
is not seven.

**Measured 2026-09-19** (every tracked file under `crates/`, `demos/`
and `benches/`; files that name the door, `test_support::block(` plus
the bare call where it is imported): **106 call sites in 22 files**.
`verbs_shell.rs` alone has 16 and `shell8_r2_probes.rs` 21. Six of the
106 were added by `private-extruded-box-builders-outside-the-brick-door`'s
PR, which is what surfaced the sentence.

The remedy this row already argues for applies unchanged: make no
census claim at the door. `block`'s useful content is the sentence
above it — that it is a second VIEW of `brick` and not a second body —
and that is an invariant, which is what a door's rustdoc is for.

