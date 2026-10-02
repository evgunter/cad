---
id: names-render-a-faces-leaf-role-in-words
kind: issue
title: The names layer renders a face's leaf role in words through one public renderer, and StableName's Display carries it (Ev, #3571)
status: open
opened: 2026-10-01
priority: P1
cost: M
needs_ev: true
---


**Ruled by Ev on #3571 (2026-10-01).** The decision record is `work/doors/face-pick-cannot-name-which-face.md`, its "Which face, in words" and "Ruled" sections. The designers' reports are on #3571. Fork-log row 23.

## What to build

- **One public renderer for a name's leaf role**, promoted from `editor_core::resolve::role_words` (with `piece_words` and `descent_leaf`). It lives in `names` and is re-exported on the `pncad` facade (`select`).
  - It matches exhaustively over `RoleSeg`, so a new segment gets words or fails to compile.
  - It looks through the carry-over wrappers (`FromA`, `FromB`, `FromMember`, `FromTarget`) to the leaf.
  - Where one more level is what tells two faces apart, it adds that level: a piece of a cut face "bordering" its neighbour (`Qualifier::Borders`) or "piece k of n" (`OrderAlong`); a fillet face "over" its rim.
- **`Display for StableName` carries the role:** kind, minting node and leaf role, the shape `Cutter` already prints. Every refusal that forwards a name then tells faces apart. `idpass::NameAndPath` keeps its `Debug` path, as the one operator diagnostic, and says so.
- **The comment** on `Display for StableName` (`names/role.rs`), written by an agent in #1454, becomes: "the path as a structure is the machine channel; a person reads its leaf in words". `display_contract.rs`'s ban test flips to assert the leaf, and still asserts that the path's structure is absent.
- **Second spellings to retire:**
  - `SelectRefusal`'s hand-written `named` helper (`names/geompred.rs`) forwards to the name's `Display`. Its `PairInBand` then stops printing a flush pair as two identical phrases.
  - `descent_leaf` derives from `names::attribute`'s `SegOrigin` classifier rather than re-listing the carry-through wrappers.

**Note (emit, #3821): `StableName`'s `Display` is no longer the one spelling.** `spoken::Speaker::name` and its `SaidName` (`editor-core/src/spoken.rs`) spell `<kind> name minted by <node>` a second time, and every sentence spoken from a document goes through them: `SpokenName`, the kernel refusals that forward a name, and the viewer's pick path (`idpass::NameAndPath`, so the disagreement notice and `frame::pick_refusal`'s tie). Changing `StableName`'s `Display` alone leaves all of those saying the old shape, so the ruled change reaches `SaidName` too. `StableName`'s `Display` is that same sentence by tag, so the two can be one spelling.

## Open within the unit

- **How a profile step is spelled.** `piece_words` prints "the profile step minted #k", a step id no screen shows. One designer recommends the loop and row the profile editor shows (from the program's `step_ids`). Check what the profile pane spells, and use that.
- **Blend, shell and split face wording.** `role_words` prints these as "a blend face" and similar, which tells nothing apart. Each needs its one level of recursion designed. The designers rated this unsure.

## Cost

Re-baselining every golden that embeds "name minted by node": editor-core display contracts, viewer refusal tests, and possibly `pncad-py` prose censuses.

## Ground

EDIT: `names/`, `resolve/mod.rs`. Also EMIT, on `role.rs`. The viewer half is AUTHOR's `face-pick-cannot-name-which-face`, blocked on this row.

Filed by the AUTHOR orchestrator on Ev's ruling.

## A name's words tell it apart, within a readable sentence

Measured on PR 3886's build over the corpus (every name in every node's table): 316 groups of distinct faces read alike, and a refusal naming two real names runs to 134 words against the 75-word budget. 314 of the 316 are copies of one master: a `Transform` passes names through, so which copy a face is shows only in the carry that brought it into the body, and the renderer reads every carry as silent.

The words of a name:

- **Shape.** `<role> of <feature>[, <join>…][, on <node>]`. The feature is the node that made the leaf. "on <node>" names the node whose output holds the name, said only where the sentence is not already about it. A name a role cites is said the same way, so it keeps its feature. The "name minted by" frame goes.
- **Joins.** A carry through a primary operand (a Boolean's A, a fillet's target) is the body's own continuation and is silent. A carry through a secondary operand (a Boolean's B, a union member) is a join, said with its node, outermost first: "the end cap of Extrude e548, cut in at Subtract 1669". Over carry chains this is injective: two names in one table first differ at a node where one went through B.
- **Detail.** Below that core, how deep cited names and how wide neighbour lists are said is chosen per speaker: a speaker holding the body's name table (the viewer's pick readout, refusals raised at evaluation) says the least detail unique in that table. A speaker holding no table says {the full form | a fixed default detail} (the open choice).
- **Gates.** Over the corpus: the full form is injective; table-scoped words are unique per body except ties; every refusal that forwards a name meets the budget with the corpus's longest scoped names; `PairInBand`'s own prose meets the refusal standard.
