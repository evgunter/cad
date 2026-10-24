# Review of PR 5636 (INTENT D′3) at frozen head `d9799aca547b`

**Verdict: NOT-MERGEABLE-AS-IS.** Three things I demonstrated by running probes break the spec's central property, "a meeting inside an operand is carried, never re-decided" (test 25's *Breaks if*). One of them is a regression against main for the spelling the spec requires the corpus to use: a list placement becomes a `⊔` placed once. All three trace back to Deviation 1: the census runs when a `⊔` is gathered, not inside its wire, so a `⊔`'s value carries no record. That deviation is disclosed and filed. What the PR body doesn't state is these consequences, and the acceptance rows are built so they never reach them.

## MAJOR

**M1 (claim 1, 4). A seated sub-assembly placed once refuses at rest; main accepts the same scene.** (`product::gather`, `eval/wire.rs` `wire_assemble`.) Bolt on plate as `bolt ⊔ plate`, one world placement, product = that placement. On head, `assemble` returns `AtRest{UnrecordedContact{ConformalPatch …}}`. On main, the same scene as the list placement `stood(vec![bolt, plate])` gives `assemble` Ok. Cause: the product census treats the copy as one arrival and finds the bolt–plate touch again, with no record to answer it. The filed `a-boolean-over-an-assemble-cites-none-of-its-meetings` says "re-finds the touch" but not that the product then refuses; the test helper `produced` has the slot read the `⊔` itself because of exactly this. Execution (red probe) plus main differential. **sure**.

**M2 (claim 1). An outer `⊔` decides a meeting inside its operand again, and the verdict flips.** `p ⊔ b ⊔ n` gives 1 row, census 0, `assemble` Ok. `(p ⊔ b) ⊔ n` gives outer census 1 (`UnrecordedContact ConformalPatch` bolt/plate) and `assemble` refuses AtRest. With overlapping `a, a'`, the outer census holds the same 21 findings the inner gather already holds; run-checks findings differ (flat 1, nested 0). Execution. **sure**.

**M3 (claim 2). A cut that bridges nothing refuses when its tool crosses a seat.** `(bolt ⊔ plate) − t`, with `t` through bolt and plate at their meeting, refuses `Boolean(VertexReadTwice{operand: B, …})`, for two different tools. Controls pass: the bolt lifted 0.05 clear gives Ok(2); a corner cut, `∩ big` and `∪ far` give expected counts. The result would be two solids touching each other, not a self-touch; claim 2 says nothing but a self-touch refuses on that path. `a_cut_through_an_assemble_keeps_its_pieces` uses pieces 1.0 apart (Q3). The filed issue asserts the meeting "is not decided again", which this contradicts. Execution (red). **sure**.

Also: the spec's *Done when* row "`(bolt ⊔ plate) ∪ bead` keeps the seat as the `⊔`'s record, cited, and refuses typed" is not met. Disclosed and filed (P1); the row asserts only `NodeErrorKind::Boolean(_)`, so any boolean error passes it.

## MINOR

- **m1 (claim 3/7). The load door doesn't check the product slot.** `persist/mod.rs` reads `SnapshotWire.product` with `#[serde(default)]`, and `validate_snapshot` never looks at it. A stored doc whose product points at a construction-space body loads and gathers. A dangling id is refused, only as a parse error. Execution; **sure**.
- **m2 (claim 3). The retired gather survives under other names**, deriving the product from `Doc::placements` filtered to the world: `editor_core::test_support::world_product_edits` (public `pub mod`); `demos/tour/src/world.rs` `world_is_the_product`, used by every tour scene; the viewer tests' `world_is_the_product`. The tour now teaches deriving the product rather than naming it. Inspection; **sure**.
- **m3 (claim 5). The pick door doesn't check paths into an instance.** In `edit.rs` `pick_fault`, an `InstantiatePart` source returns no fault, so any path is admitted (through the part's `∪`, even `[WORLD]`); evaluation then refuses `NoSuchPiece`. Undisclosed deviation from "the door refuses a path through any other operator". Execution; **sure**.
- **m4 (claim 5). A pick into a `⊔` that reads one body twice silently takes one piece.** A pick of a copy of `A ⊔ A` by `[A]` is admitted and evaluates to 1 solid; ambiguous, fail-loud would refuse. Not filed. Execution; **sure**.
- **m5 (claim 3). The viewer can't set a product, so every viewer-authored doc's export refuses `EmptyProduct`.** User-visible regression, filed only at P2 (`the-viewer-sets-no-product`). Inspection; **likely**.
- **m6 (claim 1/4). The host's census decides the overlap inside an instance's copy again.** For a part whose product overlaps itself, host product has `census` 21 beside `carried_at_rest` 21; the gate's `census_pairs` subtraction hides the double count. Test 26 asserts `host.census.is_empty()` only on a touching part. Execution; **likely**.

## NOTE

- **N1 (premise correction, claim 7).** No persisted wire version, by design. Main's five stored docs all refuse on head, typed ("unknown field `shapes`…regenerate"). Instance-only main doc untested. Execution; **sure**.
- **N2 (premise).** The `demos` job on this head was still in progress; the red is from an earlier head. Test, lint, python, viewer green. **sure**.
- **N3 (claim 6).** Re-derived: perf12 census goldens (all 3 ε) match main on structure (699 lines, same leading tokens, 11 `NamedCoincidence` each side); `rebuild-latency.json` +1 node and +1 cone on exactly five named documents. Inspection; **likely**.
- **N4.** The PR body says "the slot is in the content key"; `eval::content_key` doesn't include it; the `DocRef` pin does. **likely** that's what was meant.
- **N5 (claim 5).** The spec's grep has 3 prose hits (`docm3_union.rs:568`, `name_words_corpus.rs:155`, `:173`), no code hits. **sure**.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7; Q8 partial; Q6: Deviation 1 scheduled (P1), not an improvement.
- Q5: `Node::Assemble`'s doc and `pncad.pyi` say "its door runs the at-rest census… carries every member's own records"; `wire_assemble` does neither. **sure**.
- Q3: acceptance rows' premises miss each failing mode (disjoint pieces for the cut; slot reading the `⊔` not a placement; `Boolean(_)` accepting any refusal; touching not overlapping part). **sure**.
- Q1: one product derivation, three copies (test_support, tour, viewer tests), already drifting (tour skips `VarKind`). **sure**.
- Q4/Q5: stale prose about retired semantics — `pncad.pyi` `inline` docstring; `refactor.rs` `InlineError::UnplaceablePart.why` and `InstanceReadUncarried`; `viewer/src/session/op.rs:801` (`StableName::copy_of`); `pncad/tests/all.rs:3955`. A class. **sure**.
- Q7: `InlineError::InstanceReadUncarried` and `Uncarried::NoSuchPiece` never constructed; dead public arms mapped in pncad-py. **sure**.
- Q7: `let _ = tol;` in `wire_assemble`. **sure**.
- Q2: fixture `resolver::InPart { at, name }` keeps the retired segment's name. **likely**.
- Q7: `ProductRead` covers the spec's `SpaceMismatch` and refuses an `Operand::Pick` as `Unresolved`. **unsure**.
