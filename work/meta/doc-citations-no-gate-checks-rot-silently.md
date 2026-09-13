---
id: doc-citations-no-gate-checks-rot-silently
kind: issue
title: A CLASS - hand-written citations nothing checks: line-numbered file.rs:NNN citations across work/ (seven rot in one doc-only PR) and test-function names in doc comments, which cannot be intra-doc links
status: open
opened: 2026-09-12
---


## Finding

Filed by WIRE's `placement-prose` lane (PR 2475), on META's slate because
both arms cross every program's files and neither belongs to the code the
citations point at. `docs/prompts/implementer-discipline.md` §7 already
states the rule — *"Cite by name; line numbers rot. A number may ride
along beside the name and is allowed to go stale; a bare `file.rs:NNN` is
not a citation"* — so this row is not asking for a decision. It is asking
for an instrument, because the rule is stated and unenforced and the
measurement below is what that costs.

Two arms. They share a shape — **a citation a human wrote and no gate
reads** — and differ in instrument, so closing one is not closing the row;
a taker who does one says which.

### Arm A — `file.rs:NNN` in `work/`, rotted by an unrelated PR

PR 2475 grew `crates/editor-core/src/placement.rs` from 430 lines to 569
**without changing a single line of behaviour** — doc prose, attributes
and two test rows. That alone left **seven** line-numbered citations
across five programs' slates pointing at the wrong thing. Checked, not
assumed: each number below was read against the post-PR file.

| citation | now points at |
| --- | --- |
| `work/fix/unit-admits-non-finite-direction-norm.md:104` — `placement.rs:76` | a bare `///` |
| `work/wire/placement-lifts-its-affine-by-hand-beside-affine3-map.md:16` — `:202` | a closing `})` |
| `work/props/affine3-try-map-the-fallible-walk-has-no-kernel-door.md:63` — `:169`–`:176` | mid-sentence in `rotate_then_translate`'s doc |
| `work/lib/the-value-records-hash-by-hand-…​.md:31` — `:74` | a bare `///` |
| `work/wire/frame-linear-generic-door-has-no-consumers.md:17` — `:230-232` | a `#[must_use]` attribute |
| `work/wire/plan.md:52` — `:202,215-218` | as above |
| `work/wire/plan.md:66` — `:230-232` | as above |

**The code is right in every case.** This is doc rot, not drift: nothing
described has moved or changed meaning, and a reader who follows the
number lands somewhere arbitrary and has to re-find the subject by name
anyway. That is the whole cost, and it is paid by every reader, every
time, for as long as the row is open.

**Instrument** (the thing this row wants built, not run by hand): any
`<file>.rs:NNN` citation in `work/` or `docs/` whose file a merged PR has
changed the length of. `scripts/work.py lint` is the natural home — it
already reads every tracker file, and the check is *resolve the path,
count the lines, flag a citation past the end or (the useful half) one
whose named symbol is not within a few lines of the number*. A weaker but
free version: flag a bare `file:NNN` with no adjacent backticked symbol
name, which is exactly what §7 forbids.

**Do not fix these seven from a lane that is passing through.** One file,
one item — seven edits across five programs' slates is seven merge
conflicts. Two of the seven are on WIRE's own slate and were deliberately
left, so that this row measures the class rather than a residue.

### Arm B — a test-function name in a doc comment cannot be a link

A doc comment that cites a test by name — *"the guard is
`some_test_name`"* — is a hand-written citation with strictly less
checking than arm A: a test function is **not an intra-doc link target**
(it is `#[cfg(test)]`, invisible to rustdoc), so it cannot be spelled
`[`…`]`, and the rustdoc gate is silent on a rename. The citation is dead
the moment someone renames the test, and nothing anywhere says so.

Found in this PR's own diff, which is why it is filed rather than
grumbled about: the fix pass' first draft listed four guard names in
backticks inside `Frame`'s doc — a lane closing a hand-written census
having added a hand-written census. Three of the four were removed by
inverting the reference (each test row now names the claim it keeps, and
the type doc points at the module rather than at names). The fourth
could not be: `r1_the_placement_frame_matches_the_transform_node_bit_for_bit`
lives in `crates/editor-core/tests/asm2a_instantiate.rs`, because the
claim it guards needs a whole document, and there is no way to reach it
from a doc comment. It now carries the *"unguardable, and here is why"*
sentence at the site per `docs/prompts/reviewer-style-lane.md` Q6.

**Inverting the reference is the general fix** and it is free: let the
guard name its claim rather than the claim name its guard, so a rename
moves the only copy. It only fails across a file boundary, which is where
an instrument is owed — the cheapest being a doc-gate grep for a
backticked `snake_case_identifier` in a doc comment that matches no `fn`
in the tree.

**How wide arm B is, is not measured.** A grep for the shape over
`crates/*/src` would over-fire on every ordinary function citation and
under-fire on a test cited without backticks, so the honest statement is
that this row names one confirmed instance and one mechanism, and the
population is unknown. Whoever takes it measures it first.

Citations accurate at `7d5782045`.
