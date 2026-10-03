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

## A name's words tell it apart, within a readable sentence

Measured on PR 3886's build over the corpus (every name in every node's table): 316 groups of distinct faces read alike, and a refusal naming two real names runs to 134 words against the 75-word budget. 314 of the 316 are copies of one master: a `Transform` passes names through, so which copy a face is shows only in the carry that brought it into the body, and the renderer reads every carry as silent.

The words of a name:

- **Shape.** `<role> of <feature>[, <join>…][, on <node>]`. The feature is the node that made the leaf. "on <node>" names the node whose output holds the name, said only where the sentence is not already about it. A name a role cites is said the same way, so it keeps its feature. The "name minted by" frame goes.
- **Joins.** A carry through a primary operand (a Boolean's A, a fillet's target) is the body's own continuation and is silent. A carry through a secondary operand (a Boolean's B, a union member) is a join, said with its node, outermost first: "the end cap of Extrude e548, cut in at Subtract 1669". Over carry chains this is injective: two names in one table first differ at a node where one went through B.
- **Detail.** Below that core, how deep cited names and how wide neighbour lists are said is chosen per speaker: a speaker holding the body's name table (the viewer's pick readout, refusals raised at evaluation) says the least detail unique in that table. A speaker holding no table says {the full form | a fixed default detail} (the open choice).
- **Gates.** Over the corpus: the full form is injective; table-scoped words are unique per body except ties; every refusal that forwards a name meets the budget with the corpus's longest scoped names; `PairInBand`'s own prose meets the refusal standard.

## Ruled (Ev, PR 3906, 2026-10-03)

The shared core as recommended (joins said, one sentence shape, table-scoped detail, corpus gates), and on the one split the full form for a speaker holding no table. PR 3886 builds it in its fix pass.

## Built (2026-10-03, PR 3886)

The fix pass builds the #3906 ruling.

- **Shape.** A name in words is `<role> of <feature>[, <join>…][, on <node>]` (`names::words`), and `StableName`'s `Display`, `Speaker::name` and `SpokenName` all say it; the "name minted by" frame is gone. A carry through a boolean's B or a union's member is a join ("joined at Boolean X", "joined at Union U from Transform M"), outermost first; A and a fillet's target are silent (`CarriedAs::Primary`/`Secondary`). A cited name keeps its feature. Every number counts from zero; a cut is a "part", a profile piece a "piece". A step is said with its profile wherever the feature does not read that profile alone. The sentence is built from an explicit stack.
- **Detail.** A speaker holding the evaluation (`Speaker::within`, `NameTables`) says each name at the least detail that tells it apart in the table of the node that holds it: citations opened one at a time, fewest-rivals first, then needless openings shut. A speaker holding no table says the full form.
- **Gates** (`editor-core/tests/name_words_corpus.rs`, slow set): the full form is injective over every corpus table, from the document and by tag; the scoped words are unique per body (no admissions); `respoken_after_a_dropped_step`. The budget gate is a ratchet: seven of the eight forwarding rows overrun 75 words with the corpus's longest scoped names, filed as `refusals-with-the-longest-scoped-names-overrun-the-budget`.
- **Review fixes.** A dropped step lets go of its row in `HeldNodes::respoken`; `PairInBand`'s prose meets the refusal standard on its own; `SelectionRefusal::Unresolved`/`NotAFace` carry the name and say it through the speaker; the stranded sentence names the node that went.
- **Remains.** The viewer's pick readout speaking within the landed evaluation is DOORS' `face-pick-cannot-name-which-face`. A strand row's kept step at its old row is `a-strand-row-says-a-kept-steps-old-row`.

## Built (2026-10-03, PR 3886, second fix pass)

- **Speakers holding the evaluation now say names within it.** The kernel refusals raised against an evaluation are `NodeError`, `ResolveError`, `SelectRefusal` and `HitTestError`. Their `spoken` now takes the evaluation (`spoken(doc, evaluation)`, through `spoken_within`), so no caller can say them in full by default. Two families of caller use it:
  - The viewer's tree rows, pick refusal, picking-paths notice, index badge and tool notices speak through `Speaker::of(landed).within(evaluation)`.
  - The Python surface's node failure, poisoning, select, resolve and hit-test errors pass the evaluation they were raised against.
- **Production gate.** The budget rows are said through those `spoken` doors. The resolve rows forward a name the evaluation does not hold, so they are said in full. Measured: 263, 136, 96, 81, 76, 137, 127 and 133 words. The rows are ratcheted, and a second ratchet (`SAID_WORDS`) pins the words of every corpus name. Re-filed with these numbers as `refusals-with-the-longest-scoped-names-overrun-the-budget`, `needs_ev`.
- **Scoped uniqueness** is gated per whole table, ties and every body included. Documents built outside the corpus are a test of their own.
- **The scoped detail** is re-checked against every name of the table before it is given (`unique_detail`). The docs say it is greedy, not the least.
- **Words.** A Boolean join is said by its operation ("cut in at Subtract 1669"). `head` has words for every segment, and no silent arm: a path no operation mints, and a seam junction, say each segment.
- **The strand row** says a kept step at its new row; `a-strand-row-says-a-kept-steps-old-row` is closed.
- **No role path reaches the status line's pick refusal.** `idpass::NameAndPath` keeps the path for the picking-paths bug report (`Disagreement`, `IdAnswer`) alone.
- **Remains.**
  - The DOORS face composer (`face-pick-cannot-name-which-face`) speaks within the landed evaluation when it lands.
  - The budget is Ev's question.
  - Six rows' unmarked recourses are WIRE's `refusals-forwarding-a-name-state-no-marked-recourse`.
