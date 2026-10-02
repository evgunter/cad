---
id: selection-door-refusals-speak-the-node
kind: unit
title: The resolve, pick, naming, standing, assembly and export refusals speak the node with its label where a door holds the document
status: review
branch: emit/selection-speak
pr: 3760
opened: 2026-10-01
priority: P2
cost: H
parent: node-labels-are-document-data
---


Split from `kernel-door-refusals-beyond-edit-speak-the-node`. The rule is DESIGN.md Band 1, "Node labels": a refusal raised by a door that holds the document holds a `SpokenNode` built at the raise (`Doc::spoken`), a name speaks its minting node through `Doc::spoken_name`, and a machine channel keeps the full id. Each type needs a ruling first: is it raised at a door that holds the document, or memoized or raised where no document is at hand (then it keeps the tag)? Several of these are raised during evaluation (`NamingError` lands inside `NodeErrorKind`), and those belong to `memoized-refusals-speak-inner-nodes-through-the-frame` instead; say so here when ruled.

## The hits (node fields counted at the parent row's sweep)

- `resolve/mod.rs`: `ResolveError` (3 names) and `Diagnosis` (1 name), and its `node {…}` sentences (12 format strings).
- `NodePickError` (`resolve/pick.rs`), 2; `resolve/hit.rs`'s one sentence.
- `NamingError` (`names/emit.rs`), 4 nodes and 2 names.
- `SelectRefusal` (`names/geompred.rs`), 1 node and 3 names.
- `NodeStanding` (`eval/mod.rs`), 5. Its `Display` is also the opening of the Python `poisoning` message (`pncad-py/src/py/value.rs`).
- `AssemblyError` and `MintRefusal` (`assembly.rs`): `MintRefusal` holds 2 nodes and 1 name.
- `ExportError` (`pncad/src/export.rs`), 2.
- `pncad-py/src/py/checks.rs` (`__repr__`, `new`): a `repr` is a machine channel and keeps the full id; say which of the two is a sentence.

## Ruled

Every type here holds bare ids, and none is spoken at the raise. Each is one of two cases:

- **Memoized**: `NodeStanding` (inside `ProductError::Root` and `ClearanceRefusal`), `ResolveError` and `NamingError` (inside `NodeErrorKind`), `MintRefusal` (inside a part's `PartValue::unminted`).
- **Raised from an evaluation alone**: `NodePick::build`, `pick_face`, `entity_name`, `select_where`, `find_flush_candidates`, the read-back doors and `step_for_node` take an `Evaluation` and no document. `AssemblyError` and `ExportError` are also raised by doors that do hold the document (`assemble`, `export_document_step`), but the same type comes from doors without one (`assemble_gathered`, which takes a gathered `Product`, and `step_for_node`, which takes an evaluation) and carries the memoized payloads above. Speaking at the raise would need `absent` for ids the door cannot look up, which the rule forbids.

So each is spoken by the frame that hands it out, the `NodeError::spoken` pattern: the type writes its sentence once over `spoken::Speaker`, its `Display` says each node by tag, and `spoken(doc)` says each as the frame's document holds it.

`NamingError` is raised only inside evaluation and is carried whole by `NodeErrorKind::Naming`, so it is `memoized-refusals-speak-inner-nodes-through-the-frame`'s, as listed there.

`pncad-py/src/py/checks.rs`: `__repr__` (`CheckFinding(…, node <full id>, …)`) is a machine channel and already prints the full id. `ChecksConfig.new`'s `ValueError` ("expected_components states subject (node …) twice") is the sentence. A configuration holds no document, so it keeps the tag.

## Built

- `spoken.rs`, public and re-exported by `pncad::document`: `Speaker` (`Speaker::TAG`, or `Speaker::of(doc)`), `Say`, `Said` and `spoken_by`. `Speaker::node_as` keeps a sentence's own noun (`mate <tag>`) for a node said by tag or not held. pncad's `ExportError` is said by the same speaker.
- `spoken(doc)` on `NodeStanding`, `NodePickError`, `NameLookupError`, `HitTestError`, `UnnamedEntity`, `SelectRefusal`, `InterrogateError`, `ResolveError`, `Diagnosis`, `ResolveIndeterminate`, `AssemblyError`, `MintRefusal`, `ChecksError` and pncad's `ExportError`. The inner clauses (`RecipeEditRef`, `UpstreamCause`, `GroupCutters`, the cutter and wall lists, `RefusedRef`, `Attribution`, `RootStanding`) are said by the same speaker.
- `AssemblyError::spoken` says this document's rows from it. A carried row (`CarriedMintRefusal`, `Attribution::Carried`) is spelled in a part's ids, so it keeps its tags; so does `ExportError::UnplacedBelow`'s.
- The Python frames speak from the document the evaluation is of (`Evaluation.doc`): `value`'s standing and poisoning messages, `NodePick.build`/`build_all`, `pick_face`, `patch_names`/`boundary_names`, `select_where`, `find_flush_candidates`, the read-back doors, `resolve`'s `detail`, `step_string`, `assemble` and `run_checks`. The payload fields keep the full id. The rows an `AssemblyError` hands out (`MintRefusal`, `AtRestFinding`, `Attribution`, `RefusedRef`) speak their `__str__` from the same document; a `CarriedRefusal` and the `MintRefusal` it carries keep the part's tags.
- `ResolveError::NodeGone` now reads "the <kind> name minted by <node> is stranded: its minting node was deleted" (or "was never minted by this document"), saying the node once. The old "is no longer in the document (node … was never minted …)" contradicted itself on a foreign node.
- `Diagnosis::StructuralParam` and `UpstreamCause::StructuralParam` read "slot <slot> of <node>", so a spoken node is not nested in parentheses.

## Filed

- `check-findings-speak-their-root-by-tag`: `CheckFinding`'s root, a report's rendering.
- `a-carried-rows-route-says-its-first-instance-by-tag`: a `Route`'s first hop is this document's instance.
- On `viewer-refusals-speak-the-node`: the viewer frames that draw these types.
- On `memoized-refusals-speak-inner-nodes-through-the-frame`: `NamingError`, and the memoized types above that now have `spoken`.
