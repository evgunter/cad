---
id: test-headers-name-fns-that-exist-nowhere
kind: issue
title: Two test file headers name retired fns that exist nowhere in the tree
status: closed
opened: 2026-09-15
closed: 2026-09-15
---


(S-TINT orchestrator, 2026-09-15) Found by re-derivation lane C. Charter
shape 4 — citations and names that describe a tree that has moved — in
its cheapest form.

## The sites

- `crates/step-export/tests/m5_pr13_curved.rs`'s header names **three**
  `fn`s that exist nowhere in the tree:
  `no_body_at_rest_carries_a_nurbs_carrier_or_face`,
  `no_export_corpus_body_carries_a_nurbs_carrier_or_face`,
  `nurbs_geometry_appears_exactly_where_the_kernel_put_it`.
- `crates/topo/tests/review_mate4a_r2_probes.rs`'s header names
  `pm_census_ee_parallel`, which likewise exists nowhere.

Both are `//!` headers describing rows the file is supposed to contain.
Four names, two files, zero resolving.

## Why this is filed at all, given `D113`

`D113` closed on the ruling that a citation FORMAT is not a lever, and
that a one-time cleanup of unresolving names with no guard behind it
re-rots. **That ruling is about the ~62 identifier citations rustdoc
found, and it applies here too** — so this row is NOT a request to sweep
the tree for stale names.

What makes these four different from those sixty-two is what they are
IN. A `//!` header that enumerates the rows its file contains is a
**roster**, and a roster that names a row which does not exist is the
same defect as
`work/tint/r2-m10-6-header-roster-omits-the-suites-heaviest-row` and
`work/tint/interrogate-ladder-header-claims-every-rung-and-pins-five`,
both of which are live on this slate. Those two are about rosters that
UNDER-count; these are rosters that OVER-count. **One class, two
directions**, and the fix for all four is the same mechanism, not four
edits.

So this row exists to be **merged into whichever row takes the roster
class**, not to be taken alone. If that class gets an executable home —
a check that a file's header roster names rows the file actually has —
it closes all four of these for free and the names never need touching
by hand. If it does not, these four are prose fixes of exactly the kind
`D113` declined to schedule, and this row should close unfixed with that
said out loud.

## Blind spot

Found incidentally, not swept. The instrument that would find the rest
is a roster check, which is the fix — so the population is unknown and
**four is a floor with no ceiling measured**. Nobody should read four as
the count.

## Closed — the premise was wrong (S-TINT orchestrator, 2026-09-15)

**This row should not have been filed, and the error was this seat's.**
It was filed the same day out of a re-derivation lane's incidental
finding, asserting that four `fn` names in two test headers "do not
resolve" and belonging to the roster class. A feasibility probe on that
class checked the sites and both are **deliberate lineage, not stale
rosters**:

- `crates/step-export/tests/m5_pr13_curved.rs` names its three under an
  explicit heading — *"**Lineage**: `CENSUS` succeeds
  `no_body_at_rest_carries_a_nurbs_carrier_or_face`,
  `no_export_corpus_body_carries_a_nurbs_carrier_or_face` and
  `nurbs_geometry_appears_exactly_where_the_kernel_put_it`."* They are
  named BECAUSE they are retired; that is the sentence's whole subject.
- `crates/topo/tests/review_mate4a_r2_probes.rs` names
  `pm_census_ee_parallel` as a source of escalations, not as a row the
  file contains.

So the count "four names, zero resolving" was true and **irrelevant** —
the header never claimed those rows were in the file. A roster mechanism
would actively FORBID naming a retired row, which is the wrong answer
for a deliberate lineage note.

**What this row got wrong, stated so the mistake is legible**: it read a
population off a pattern (backticked identifiers in a `//!` header that
resolve nowhere) and assigned it a subject (a roster that over-counts)
without reading what the sentences around the names actually said. That
is the same defect this program exists to find, committed by this
program's own orchestrator while filing against it — the class in
`work/tint/process-observations.md` observation 1, one level up from the
code.

**One real question is left and is deliberately NOT taken.**
`docs/prompts/implementer-discipline.md` §4 says comments state the
invariant, not the history — *"no retired-type archaeology … an argument
about how the code used to work belongs in the PR description"* — so the
lineage note is arguably against that discipline. Taking it would be a
prose edit across two files with **no guard behind it**, and Ev's ruling
on `D113` settles that shape: *a one-time cleanup with no guard re-rots*.
Nothing is scheduled. If the §4 reading is ever pressed, it is a question
about the discipline's scope for test-file headers and belongs on an
`[ev]` PR, not on this slate as implied work.
