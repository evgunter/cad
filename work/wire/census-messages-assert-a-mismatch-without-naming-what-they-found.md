---
id: census-messages-assert-a-mismatch-without-naming-what-they-found
kind: issue
title: A CLASS - three census clauses report a mismatch without naming what they found, in a file whose sibling clause does
status: closed
opened: 2026-09-12
branch: wire/census-localise
pr: 2501
closed: 2026-09-13
---



(WIRE orchestrator, 2026-09-12) From the light review of PR 2447. The
PR filed one instance
(`work/wire/wire-roundtrip-census-localises-nothing.md`) and fixed a
second; the review found a third and a limit on the fix. Filed as the
class so the next taker sees the shape rather than one clause.

All four live in `crates/editor-core/tests/switch_program_vocabulary.rs`
— a file whose whole job is to catch a vocabulary member that fails to
reach a downstream spelling. Every one of them **fires correctly**;
what they do badly is say what they found, which is the half a reader
needs at 3am with a red shard and no local repro.

## The instances

**1. The mode census's laundering clause (`:471-474`) — worst, because
its sibling shows the fix.** It asserts `spec.mode() == *mode` and
prints *"the document spec for Radius resolved to a different mode"*
**without printing which mode it resolved to**. Fifteen lines up, the
target census (`:377-381`) does exactly this comparison and DOES print
what it got:

> `the document target {name} resolved to {got_name}`

Two clauses written to the same model, fifteen lines apart, one a notch
less useful. Measured live by the review: changing `res_spec`'s
`Radius` arm to construct `ArcData::Sweep` reddens this clause and
nothing else in the tree — so it is the **only** catch for the
`res_spec` hop that #836 and S195 both call the hop that matters, and
it is the one that will not tell you where you landed.

**2. The wire round-trip clause** —
`work/wire/wire-roundtrip-census-localises-nothing.md`, filed by 2447's
lane. Fires correctly on a laundered vocabulary member, then prints two
whole-corpus `Debug` renderings, thousands of characters each.

**3. The bijection count clause — fixed in PR 2447, but only half.**
The fix names the offending chain step and its **verb**; it never names
the **arc mode**, because `variant_name` (`:279`) takes only the
leading identifier of a `Debug`. In a corpus where `ArcTo` appears
seven times — once per `ArcMode::ALL` entry — *"chain step 16 (ArcTo)
has 3 expressions and enumerates 0 slots"* still leaves the reader
counting `chain_steps()` to find which mode's arm is short. The fix's
own doc states the defect as a message *"naming neither the step nor
the vocabulary member"* and closes one of the two.

**4. The same fix's localizer re-derives its subject.**
`steps_whose_slot_count_disagrees()` (`:552`) calls `chain_steps()`
afresh, while the clause it explains asserts over `corpus()` (`:261`).
They agree today only because `corpus()`'s first loop happens to be
`LoopProgram::Chain(chain_steps())`. Add a second chain loop or reorder
the loops and the helper localises confidently against the wrong
program — and it prints `chain step {i}` and never `loop_`, though
`SlotId::Profile` carries one and the helper's own doc justifies the
index by saying it is *"what a `SlotId::Profile` carries"*.

## What a taker owes

The cheap version is four message edits and is worth doing on its own.
The version worth thinking about first is whether these clauses share a
**localiser** — they all answer the same question ("which member of
which vocabulary, at which position") and they all answer it
differently or not at all. This file already has one narrowing helper
per census and a `variant_name` that every clause reaches for; a single
"name the offender" door would be the fourth projection of the same
idea `ALL` gave these censuses in the first place.

Instance 4 is the constraint on that: the localiser must read the
program under test rather than rebuild its own, or it becomes a second
corpus kept in step by hand — which is the defect this file exists to
catch, one level up.

## Closed 2026-09-13 (PR 2501)

All four instances fixed, plus five more the file-wide sweep found.
The class's own answer was the one this row asked for: `step_label` is
the single "name the offender" door, a verb plus every vocabulary
member riding it, and every clause that localises a failure to a step
says it through there.

- **1** — the mode census's laundering clause now prints
  *"the document spec for Radius resolved to Sweep"*, on the target
  census's model verbatim.
- **2** — `every_document_verb_survives_the_wire` localises; see
  `wire-roundtrip-census-localises-nothing.md`.
- **3** — the bijection count clause's localiser names the mode:
  *"loop 0 chain step 15 (ArcTo(Sweep)) has 2 expressions and
  enumerates 1 slots"*.
- **4** — the localiser takes the program under test as an argument
  and walks ITS loops, so the indices it prints are that program's, it
  prints the `loop_` a `SlotId::Profile` carries, and the carrier
  loops are walked too (the "empty means a carrier" caveat is gone
  because carriers are no longer uncovered).

Five more of the same shape, found by sweeping the file's assertions
rather than the three names: the per-position verb clause (named a
verb in a corpus where a verb is not an identity), both witness shape
panics (said what the step was NOT), the duplicate-address clause (did
not name the slot it collided with) and the dimension clause (named
neither dimension). Each was driven red with its new message.

## R1 addendum: instance 4's class had a second member

The review found the verb census doing exactly what instance 4
described, fifteen lines from where this row's fix argued against it:
`let authored = chain_steps()` is a second freshly-built corpus, lined
up with the resolved loops only because `corpus()`'s first loop happens
to be `Chain(chain_steps())`. It now reads the authored side OUT of the
program it resolves and walks EVERY chain loop, so the assumption is
gone rather than restated, and it prints the `loop_` the localiser
prints:

    loop 0 chain step 8 (LineTo(Point)) lifted to Verb::ContinueTo

`position_label` is the one place that prefix is spelled, for the reason
`step_label` is the one place a step's members are named.


## Closed 2026-09-13 (PR 2501)

Nine clauses now name the value that made them fire; six already did.
Every message was **captured from a planted mutant** rather than
predicted — the form of evidence this program settled on — and the file
was swept for the shape rather than the three named instances, with the
hit list and its dispositions in the PR.

Instance 1 was the one that mattered out of proportion to its size: the
mode-laundering clause was measured by PR 2447's review as **the only
catch in the tree** for the `res_spec` hop, and it was the one clause
that would not tell you where you landed. Its fix was written fifteen
lines above it the whole time, in the target census. It now says
*"the document spec for Radius resolved to Sweep."*

Instance 3's half-fix is finished: the localiser named the chain step and
its verb but never the **arc mode**, because `variant_name` took only the
leading identifier — so in a corpus where `ArcTo` appears once per
`ArcMode::ALL` entry, naming the verb narrowed nothing.

**The sweep's stated blind spot is the durable part**: the pattern read
every `assert!`/`panic!` for whether its message names the value that
fired it, and **cannot see a clause whose message is fine but whose
SUBJECT is wrong.** One instance was found that way by reading, not by
the pattern — so the hit list is a floor on what is there, not a ceiling.
