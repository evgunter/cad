---
id: a-doors-rustdoc-carries-an-unguarded-census-sentence
kind: issue
title: Two doors in a row carry an unguarded census sentence in their rustdoc; the tree has the machine for a guard
status: closed
opened: 2026-09-19
priority: P4
cost: E
closed: 2026-09-24
branch: dup/topo-fixture-batch
pr: 3152
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


## Which remedy the tree prefers (2026-09-24, `dup/topo-fixture-batch`)

This row asked for a `source_walk.rs` guard. The program's method has
since ruled the other way — `work/dup/plan.md` item 13: *"The fix is
not to guard the sentence; it is to make no census claim at the door
and let the row hold the measurement, dated, with its instruments
named."* `solid_of_face` already took that shape. So both remaining
carriers here were rewritten, not guarded:

- `Body::face_of_half_edge`: the *"Every spelling in this crate … reads
  through here"* universal, the *"Six more"* and the *"sixteen sites"*
  are gone. What stays is the invariant (a caller whose refusal names
  the stale key keeps its own walk, and that is a population) and the
  same *"No census is claimed here"* pointer `solid_of_face` carries,
  to `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`.
- `sweep::test_support::block`: the *"seven private copies"* history
  is gone; the sentence says what the door is (a second view of
  `brick`) and claims nothing about its callers.

## The class, census at `6db5b87f2`

The counts in this section were taken at `6db5b87f2` and are not
re-taken here; the filed row below re-took passes 2 and 3 at
`1d5922f1b` (102 and 42).

**Pass 1**, line-shaped: a `///` line holding a count of three or more
(word or digit) followed within two words by *sites, copies,
spellings, callers, call sites, suites, consumers, places, files*,
over every tracked `crates/*/src` file. **55 hits.** Blind spot: a
count and its noun on two lines.

**Pass 2**, at that gap: each file's consecutive `///`/`//!` lines
joined into one string, same pattern. **107 hits, 50 that pass 1
missed** — among them **this row's own subject**, `body.rs`'s
*"sixteen / sites"*, and `sweep::test_support`'s module-header
*"six integration suites"*. A line is not this class's unit
(`the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files`
found the same). Blind spot: an ORDINAL count.

**Pass 3**, at that gap: ordinals (*third … twentieth*) before the
singular nouns, joined. **41 hits**, among them
`sweep::test_support::prism`'s *"The twelfth copy of this four-line
helper … was what got it homed"*.

**Folded here, every hit in the two files this row names**:
`body.rs` (1) and `sweep/src/test_support.rs` (7: the module header,
`block`, `revolved_about_y`, `prism`, `stacked_at`,
`assert_promises_either_side`, `assert_full_revolve_rim` — each
rewritten to the door's reason, with no count). In the other files
this PR had open, the hits are **named enumerations, not counts of a
drifting population**, and stay: `chart_region.rs`'s *"three named
places"* and *"exactly THREE places"* (each followed by its list of
arms in the same function), `boolean/join.rs` and `merge_faces.rs`'s
*"three sites"* of the ring-winding predicate (named in full at the
canonical statement), `test_support_fixtures.rs`'s *"three axes"*
(listed below it), `euler_ring.rs`'s *"third spelling of the second"*
(an argument, not a count).

The rest of the population is outside this row's two doors and is
filed as
`work/dup/rustdoc-count-sentences-outside-the-two-doors-are-unguarded.md`.

## Closed (2026-09-24, PR #3152)

Both doors now make no census claim, and neither does any other door
in the two files this row names. The population outside them is the
row filed above, with its instruments and hit list.
