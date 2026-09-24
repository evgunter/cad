# S-TINT — process observations

Not a slate and not a log. Three things this program has learned about
how its own work goes wrong, each with the evidence that made it more
than a suspicion. All three are about S-TINT's own work, not about the
tree it repairs.

## 1. A unit that closes a hand-written mirror mints a fresh one inside its own fix

**Seven instances across five of the six units run so far — TINT-5
produced two and TINT-6 produced six at once. The outside reader caught
four; a sibling lane's base merge caught one; and on TINT-6 **the lane
caught its own, all six, before it pushed**. Every spec that warned
against the trap got the warning obeyed and the trap sprung anyway, one
shape sideways — until a lane stopped relying on the warning and read
its own diff cold instead.**

**TINT-1** (`assert_f6` ban lists). The spec devoted a section to
refusing a source-scanning census because it would mint an instance of
`source-scanning-censuses-are-a-tripwire-on-ordinary-rust`, a live row on
this slate. The lane obeyed that — and two lines away wrote match arms
returning **hand-typed identifier strings** beside a hand-written roster.
rustc checks a match's patterns and never its strings, so an arm reading
`ParseError::UnknownUnitSymbol { .. } => "UnknownUnit"` (what a RENAME
produces) left the suite green while banning a dead identifier and
leaving the live one unbanned. The review demonstrated it on the pre-fix
file.

**TINT-2** (the stand-down channel). The spec required the unit to state
what its guard does not enforce, precisely because of TINT-1. The lane
did that honestly — and replaced eight hand-kept row lists with a
**template pasted into nine files**, one of which was **false on
arrival**: `crates/topo/tests/m6_2_fitted_at_rest.rs` was told it was an
otherwise-empty interval binary when it carries three ungated rows, and
the false sentence REPLACED an accurate one. The fix pass then found two
more (`error_display.rs`, 15 ungated rows; `panel_display.rs`, 16) in a
variant nobody had checked.

**What does not work: naming the trap.** Both specs named it. Both lanes
read it. Both minted one anyway, in a shape the warning did not
literally cover — a different copy mechanism each time. A warning tells a
lane which instance to avoid; it does not make the lane ask *"is what I
am about to write an instance?"*

**TINT-4** (a header roster). The spec required the lane to state that
the weld holds NAMES and never PROSE, and the lane did — then wrote
seven sentences into the new, explicitly-unchecked prose column, of
which **four were wrong or misplaced on arrival**, including a
fabricated citation in the very entry the unit existed to correct. The
column was disclosed exactly as instructed and was wrong anyway.

**TINT-5** (the F6 weld's home), twice, and the second is a new shape.
First: the unit promoted `set_difference` into `test-utils` while
TINT-4's `roster::violations_against` was already spelling a
both-direction set comparison inline there — **two comparators, four
days apart, by two units of the program whose subject is one claim in N
copies**, in the crate whose job is to hold one of each thing. The lane
FILED it on three stated blockers; the review executed the substitution
and all three were false. Second: two of this unit's own filed rows
**misstated their own defects** — one claiming three identifiers were
dead that are all live, the other (the row the unit was cut from)
claiming a `Debug` regression would have passed when the brace ban
already caught it.

**The fifth was not caught by a reader at all.** The two-comparators
collision surfaced when a sibling lane merged main and saw both spellings
land in one crate. That is worth naming because it is the first
detection in this program that no amount of reviewing would have
produced: neither unit's diff contains the defect — it exists only in
their union, and only after both landed.

**TINT-6** (the interrogate ladder), and this one breaks the pattern.
The lane minted six: a `READ_DOORS` table typing each door's NAME as a
string beside its path — TINT-1's exact defect, in the unit that
inherits TINT-1's mechanism; a hand-written array length plus two prose
sentences restating it, **in the unit whose subject is a header that
counted and counted wrong**; a helper shadowing a fixture of the same
name in the same file; an `assert_ne!` over two `&'static str`s, fixed
at compile time; a doc citing `work/tint` by a path that is deleted when
the program closes — TINT-4's dangling-citation shape; and "two of its
arms' payloads" where there is one arm and two payloads.

**All six were caught by the lane, not the reviewer**, on two cold reads
of its own diff — three on the first pass and three more on a second
read taken off `git diff --cached`. That the second read found as many
as the first is the part worth keeping: one pass is not the method, two
passes at different framings is.

**What did work, four times out of five, and what finally worked once
without one: a reader who did not write the fix.** That
is `docs/prompts/reviewer-style-lane.md` §1's standing claim
(*"Only a reader who did not write the fix has ever caught it, which is
you"*), and this program has now paid for it on four of the five units it
has run (TINT-3 is the one that did not mint; its review found an
undisclosed cost instead). It is
the argument for keeping one style review on every unit of this program
however small the diff looks — TINT-2's diff was prose and deletions,
with no executable line, and it still shipped a false claim.

**What may work better, untested: make the spec state the mechanism's
LIMIT as a claim to be falsified**, rather than stating the mechanism as
settled. See observation 2 — it is the same failure from the other end.

## 2. Both specs so far have misled their lane, and the correction came from the review

**TINT-1's spec said *"use the compiler"* and *"a variant added tomorrow
makes this file fail to COMPILE"*.** rustc checks patterns, not strings,
so that overstated what the design could deliver — and the phrase
*"rustc is the census"* was copied out of the spec into a committed doc
comment, where it was simply false. `docs/DOC-LEDGER.md` records that
spec as **wrong rather than superseded**.

**TINT-2's spec leaned to *"make a stand-down a fact the suite can
floor"*.** The lane refused and the review verified why behaviourally: a
two-row probe under the pinned `cargo-nextest 0.9.140` reports **pids
12156 and 12157** with a `static AtomicUsize` reading 0 in both. nextest
is process-per-test, so nothing one row records is readable by another.
**That option had no mechanism inside the fence and was never
available.**

Two for two. The pattern is not that the specs were careless — each was
argued at length — but that **an orchestrator writing a spec states a
mechanism it has not executed**, and a mechanism nobody has run is a
hypothesis wearing a decision's clothes. A lane then either obeys a
hypothesis that does not work, or spends its own effort refusing it.

**The correction this program is adopting**: a spec that names a
mechanism also names the ONE measurement that would show the mechanism
cannot work, and says the lane is expected to take it first and report
back if it fails. TINT-2's lane did that unprompted; it should not have
had to.

## 3. The orchestrator read a green light on a wire that was not connected

**`"merged": true` is a claim about a PR's base. It was read as a claim
about `main`.** TINT-3's PR was opened with base
`tint/2-stand-down-channel` — the lane had cut its branch off TINT-2's
head to build on unmerged work, and TINT-2 landed on `main` two minutes
before the PR existed, which nobody noticed. The merge API then did
exactly what it was asked and said so. The unit's merge commit,
`e4adf05a1`, was reachable from that branch and from nothing else.

The log entry declaring the unit landed was written from `merged: true`
and a green CI run id. **Neither is a fact about `main`**, and there is
no state in which that API call reports a wrong base, because to the API
there is no wrong base. A check that cannot go red is not a check —
S-TINT's charter sentence, and this seat wrote its own instance of it
while running the program named for it.

**Caught by TINT-4's style review, as a note, while reviewing a
different unit.** Third time in this program that the outside reader
found what the working seat could not. It is the same standing claim
observation 1 rests on, reaching one level further out than the code.

**The correction**: a unit is landed when
`git merge-base --is-ancestor <merge-sha> origin/main` returns true, and
the log entry saying so is written after that command rather than before
it. Cheap, mechanical, and it is the first thing in this program that
has closed one of these by a command rather than by a warning.

## What these do NOT license

None of the three is a reason to stop writing specs, to stop deciding
the fix shape in them, or to soften the standing discipline. A spec that
decides badly is still better than a lane deciding in the dark — both
units' fix shapes were arrived at faster for having something to argue
against. And none of them is evidence about any OTHER program:
S-TINT's units are unusual in that the thing being repaired and the
thing doing the repairing are the same kind of artifact, which is
plausibly why its fixes keep becoming instances of their own subject.
