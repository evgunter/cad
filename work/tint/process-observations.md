# S-TINT — process observations

Not a slate and not a log. Two things this program has learned about
how its own work goes wrong, each with the evidence that made it more
than a suspicion. Both are about S-TINT's units, not about the tree they
repair.

## 1. A unit that closes a hand-written mirror mints a fresh one inside its own fix

**Twice out of two units. Both times the outside reviewer caught it, and
both times the unit's own spec had warned against exactly that trap in a
paragraph of its own.**

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

**What did work, both times: a reader who did not write the fix.** That
is `docs/prompts/reviewer-style-lane.md` §1's standing claim
(*"Only a reader who did not write the fix has ever caught it, which is
you"*), and this program has now paid for it twice on two units. It is
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

## What these do NOT license

Neither observation is a reason to stop writing specs, to stop deciding
the fix shape in them, or to soften the standing discipline. A spec that
decides badly is still better than a lane deciding in the dark — both
units' fix shapes were arrived at faster for having something to argue
against. And neither observation is evidence about any OTHER program:
S-TINT's units are unusual in that the thing being repaired and the
thing doing the repairing are the same kind of artifact, which is
plausibly why its fixes keep becoming instances of their own subject.
