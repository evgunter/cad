---
id: census-messages-assert-a-mismatch-without-naming-what-they-found
kind: issue
title: A CLASS - three census clauses report a mismatch without naming what they found, in a file whose sibling clause does
status: open
opened: 2026-09-12
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
