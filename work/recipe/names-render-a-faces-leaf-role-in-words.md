---
id: names-render-a-faces-leaf-role-in-words
kind: issue
title: The names layer renders a face's leaf role in words through one public renderer, and StableName's Display carries it (Ev, #3571)
status: open
opened: 2026-10-01
priority: P1
cost: M
---


**Ruled by Ev on #3571 (2026-10-01).** The decision record is `work/doors/face-pick-cannot-name-which-face.md`, its "Which face, in words" and "Ruled" sections. The designers' reports are on #3571. Fork-log row 23.

## What to build

- **One public renderer for a name's leaf role**, promoted from `editor_core::resolve::role_words` (with `piece_words` and `descent_leaf`). It lives in `names` and is re-exported on the `pncad` facade (`select`).
  - It matches exhaustively over `RoleSeg`, so a new segment gets words or fails to compile.
  - It looks through the carry-over wrappers (`FromA`, `FromB`, `FromMember`, `FromTarget`) to the leaf.
  - Where one more level is what tells two faces apart, it adds that level: a piece of a cut face "bordering" its neighbour (`Qualifier::Borders`) or "piece k of n" (`OrderAlong`); a fillet face "over" its rim.
- **`Display for StableName` carries the role:** kind, minting node and leaf role, the shape `Cutter` already prints. Every refusal that forwards a name then tells faces apart. `idpass::name_and_path` keeps its `Debug` path, as the one operator diagnostic, and says so.
- **The comment** on `Display for StableName` (`names/role.rs`), written by an agent in #1454, becomes: "the path as a structure is the machine channel; a person reads its leaf in words". `display_contract.rs`'s ban test flips to assert the leaf, and still asserts that the path's structure is absent.
- **Second spellings to retire:**
  - `SelectRefusal`'s hand-written `named` helper (`names/geompred.rs`) forwards to the name's `Display`. Its `PairInBand` then stops printing a flush pair as two identical phrases.
  - `descent_leaf` derives from `names::attribute`'s `SegOrigin` classifier rather than re-listing the carry-through wrappers.

## Open within the unit

- **How a profile step is spelled.** `piece_words` prints "the profile step minted #k", a step id no screen shows. One designer recommends the loop and row the profile editor shows (from the program's `step_ids`). Check what the profile pane spells, and use that.
- **Blend, shell and split face wording.** `role_words` prints these as "a blend face" and similar, which tells nothing apart. Each needs its one level of recursion designed. The designers rated this unsure.

## Cost

Re-baselining every golden that embeds "name minted by node": editor-core display contracts, viewer refusal tests, and possibly `pncad-py` prose censuses.

## Ground

EDIT: `names/`, `resolve/mod.rs`. Also EMIT, on `role.rs`. The viewer half is AUTHOR's `face-pick-cannot-name-which-face`, blocked on this row.

Filed by the AUTHOR orchestrator on Ev's ruling.
