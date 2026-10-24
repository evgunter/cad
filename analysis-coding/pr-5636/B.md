# Review of PR 5636 (INTENT D′3), head `d9799aca547b2e3018164a33b1fd8c5209472f59`

**Verdict: NOT-MERGEABLE-AS-IS.** Three of the brief's claims fail when run: a nested `⊔` decides a seat a second time and refuses it, a cut through two touching pieces refuses although nothing touches itself, and the product slot can be moved out of the world with no report. Claim 3's retired gather also survives in two helpers. None of these is a regression from main, which refused any boolean over a product. Each looks fixable or fileable inside this PR.

CI on this head: test, lint, python and viewer green; demos still in progress at report time.

## MAJOR

**M1. A `⊔` decides the meeting inside a `⊔` operand again, and refuses it** (claim 1; `product.rs` `gather` / `gather_sources`; `eval/wire.rs` `Node::Assemble`). With only `p ⊔ bolt` (bolt resting on plate) as the product: 1 coincidence, 1 contact, census 0, `assemble` succeeds. With `(p ⊔ bolt) ⊔ far`: 0 coincidences, census 1, `assemble` fails `AtRest { UnrecordedContact { ConformalPatch … Rest } }` on the same faces. A placement of `p ⊔ bolt` shows the same: census 1, at-rest refusal, plus `Connectedness` and `UnprovenCoincidence` findings. The placement case is disclosed (deviation 1, `a-placement-carries-its-sources-at-rest-result`); the nested-`⊔` case is not disclosed or filed. The acceptance rows step around both: `produced` has the slot read the `⊔` itself, and `three_pieces` uses non-touching pieces. Execution (red). sure.

**M2. A cut through two touching pieces refuses, though the result is no self-touch** (claim 2). Blocks `a`, `b` stacked face to face: `a − through` 1 solid; `(a ∪ b) − through` 1; `(a ∪ b) − hole` 1; `(a ⊔ b) − low` (not reaching the shared face) 2; `(a ⊔ b) − through` → `Boolean(VertexReadTwice { operand: B, reads: [Pierce(1v1), Pierce(8v1)] })`; `(a ⊔ b) − hole` the same. Drilling through a seat is the basic assembly operation. The filed `a-boolean-over-an-assemble-cites-none-of-its-meetings` covers only the bridging union, not a cut. `a_seat_unioned_with_a_bead_keeps_its_pieces` accepts any `NodeErrorKind::Boolean(_)`. Execution. sure.

**M3. The product slot can be moved out of the world with nothing reported** (claim 3; `edit.rs` `product_read` guards only `SetProduct`; `product.rs` `gather` uses `space_of(var).unwrap_or(VarId::WORLD)`; `persist/check.rs` has no product rule). Set the product to a world copy; `SetParam` the copy's `OperandSlot::Space` to a `Node::Space`; admitted with empty maintenance; `space_of(product)` is the new space, yet `product()` returns 1 solid, and save then load succeed. The rule holds only at one door; export writes this body too. Execution (red). sure.

## MINOR

**m1. The retired gather survives under another name** (claim 3). `editor_core::test_support::with_world_product` / `world_product_edits` and the tour's `world.rs::world_is_the_product` both set the product to the `⊔` of `doc.placements()` filtered to world targets — main's `product_in` rule re-spelled as an edit. `fixture::insert` calls it after every world placement (~1,500 test call sites); the viewer's `test_support::placed_read` calls it; the tour from 14 sites in 10 files. The suite rarely sees the slot as something an author writes; the tour does not say its products. sure.

**m2. The pick door's walk steps through a pick as if it were that pick's placement** (claim 5; `edit.rs` `pick_fault`, `source` via `operation_of`). A placement whose source is the pick of the bolt, then picked for the plate: the door admits it, evaluation refuses `NoSuchPiece { at: 0 }`. Typed, but at the wrong door. Other walks hold (through `−`, past a leaf, empty path, pick of a `⊔` itself). Execution. sure.

**m3. The spec's grep is not empty** (claim 5): `docm3_union.rs:568`, `name_words_corpus.rs:155`, `:173`. The `docm3_union.rs` hit states the retired mechanism as current. The PR body says the grep is empty. sure.

## NOTE

- **n1. Correction to claim 7's premise.** No persisted version, on purpose; the 19→20 bump is the content-key `VERSION`, never persisted. Main's `golden.cad` refuses `Unreadable` ("unknown field `shapes`") with the regenerate recourse. `product` is `#[serde(default)]`; a current-format file without it loads as an empty product; no silent path found. likely.
- **n2. Claim 6 re-derived, holding.** perf12 census ×3, ids masked and names cut, line-for-line identical. Exactly five corpus documents have their slot read a `⊔` (heat_sink 6, declared_tangency 2, kitchen_sink 8, cut_cylinder 2, part_select 2), the five rebuild-latency rows that gained one node. sure.
- **n3. Content pin** differs when only the slot differs. sure.
- **n4. Claim 4's premise may be vacuous.** No multi-shape placement found in main's corpus or stored documents except `pncad-py/tests/ty_fixtures/legal.py:559`. unsure.
- **n5.** Demos job unfinished on this head; teapot touch unverified.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6; Q8 partial; Q7 only s7.
- s1 (Q1) `test_support.rs` `world_product_edits` and tour `world_is_the_product`: two hand-written copies of one derivation, each with its own "a `⊔` I wrote before" rule. sure
- s2 (Q3) `produced` and its doc exist to keep the slot off a placed `⊔`; `three_pieces`' pieces don't touch. likely
- s3 (Q3) `a_seat_unioned_with_a_bead_keeps_its_pieces` passes on any boolean refusal, including M2's. sure
- s4 (Q4) `viewer/src/session/op.rs` `AddFillet`'s doc cites retired `StableName::copy_of`; `name_words_corpus.rs:155` "D′ re-tightens this pin" though this PR does it; other viewer `///` text saying "world placements re-pointed". sure
- s5 (Q2/Q5) `product.rs` module doc still speaks of roots (filed `the-world-still-speaks-of-roots`); `space_of(var).unwrap_or(VarId::WORLD)` a silent default (M3's mechanism). sure
- s6 (Q2) `persist/check.rs` now skips every name read in a body reaching an instance; a broad loosening, disclosed under deviation 4. unsure
- s7 (Q7) `assemble_gathers` re-grafts and re-censuses every `⊔` on each `product_recorded` call. unsure
- s8 (Q6) Deviation 1 narrows the spec; the filed item's scope misses M1's nested case and M2's cut. likely
