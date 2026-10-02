---
id: names-render-a-faces-leaf-role-in-words
kind: issue
title: The names layer renders a face's leaf role in words through one public renderer, and StableName's Display carries it (Ev, #3571)
status: review
opened: 2026-10-01
priority: P1
cost: M
branch: recipe/leaf-role-words
pr: 3886
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

## Built (2026-10-02, PR 3886)

- `names::words` holds the one public renderer (`LeafRole`, `leaf_role`, `role_leaf`), re-exported on `pncad::select`. Its descent comes from `attribute`'s `SegOrigin`, which now says how each carry was carried (`CarriedAs`). `resolve`'s `role_words`, `piece_words`, `descent_leaf` and `Cutter` are gone.
- `Speaker::name` and `StableName`'s `Display` are one sentence: kind, minting node, then the leaf role in words, adding the leaf's node where it differs. `SpokenName` keeps the nodes and steps its words say.
- A profile step is said as `loop L step S`, both from zero, as the profile pane numbers it. Where no document is at hand, it is said by its tag.
- Blend, shell and split faces are each said over what they were made against, one level down. Fragments and non-whole carries are said around the leaf.
- `SelectRefusal`'s `named`, the three kind refusals in `eval`, `MeasureRefUnreadable` and `CrossingUnverified` all forward the name's sentence.
- Remains: the viewer half is DOORS' `face-pick-cannot-name-which-face`, which this row unblocks.
