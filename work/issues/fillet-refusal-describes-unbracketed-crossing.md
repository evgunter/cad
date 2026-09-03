---
id: fillet-refusal-describes-unbracketed-crossing
kind: issue
title: An arc-carrier fillet refusal can describe the crossing the author did not bracket
status: open
opened: 2026-08-30
github: 1281
refs: [1267]
---

## From GitHub issue 1281

opened 2026-08-30, 0 comments.

A **pre-existing attribution gap** in the arc-carrier fillet's refusal channel, measured during the BLEND-7 review (PR #1267) and disclosed there rather than changed by it.

## The mechanic

`path::arc_fillet`'s resolve enumerates every derived corner of the carrier pair, runs the ratified construction at each, and keeps the FIRST construction refusal it sees (`build_refused`, `crates/profile/src/path/arc_fillet.rs`). When both crossings of the pair sit inside the anchors' advance/reach windows and both refuse, the sentence the author reads is the first corner enumerated — which need not be the corner they bracketed. Every refusal that rides that channel inherits this: `NoCornerForFillet`, `AnchorOutsideTrimmedExtent`, and now `FilletEnclosesLegCarrier`.

## The measurement

A reviewer's 4 032-corner sweep found the reported refusal describing the other crossing in **12%** of refusals. Both reviewers also measured how far away that other crossing sits — on the enclosing fixtures it is a corner up to 0.83 m from the one the anchors bracket, i.e. not a near-neighbour whose numbers would be interchangeable.

## Why it is not fixed in PR #1267

The channel predates that unit, is shared by three refusal families, and choosing an attribution rule (nearest-to-the-anchors? the corner the gates admit most cleanly? report both?) is a design decision about what a refusal is *about*. The unit's fence explicitly excluded re-ranking or widening the lattice ladder.

What that unit did instead: the new refusal's deixis is written to what is honest at the site — *"a radius-r m fillet cannot round a corner of these carriers"*, not "this corner" — and the Display arm cites this issue at the deixis.

## What would close it

An attribution rule for the refusal channel, or a payload that names which crossing the refusal is about (the corner point is known at the site). Either makes the confident deixis honest again.

## A related observation, recorded here because it is the same surface (reviewer NOTE, not scheduled)

**The refusal taxonomy is gate-shaped rather than author-shaped.** The names an author meets — `NoCornerForFillet{OffsetCarriersDisjoint}`, `NoCornerForFillet{NoCornerSideCandidate}`, `AnchorOutsideTrimmedExtent`, `FilletOffsetLeverTooShort` — partition the space by *which gate fired*, which is the construction's own anatomy, not the author's. One authored mistake ("this radius is too big for this corner") reaches them through three different names depending on how far past the bound it sits, and BLEND-7's measurement (PR #1267 §1.B) shows all three on one radius sweep of one fixture. An author-shaped taxonomy would name the situation and let the gate ride the payload.

Not a defect on its own — each name is true where it fires — and not BLEND-7's to change: it is a question about the whole fillet refusal family, adjacent to the attribution rule above, which is why it is parked beside it rather than in its own issue.

— Filed by the BLEND-7 implementer lane, adjudicating both blinded reviews.

## Home

`work/issues/` — `crates/profile/src/path/arc_fillet.rs`'s refusal channel is S-BLEND-era ground and S-BLEND is closed; S-BOOL's `crates/profile/*` glob is for boolean reach, not this channel.
