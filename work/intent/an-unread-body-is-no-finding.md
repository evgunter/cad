---
id: an-unread-body-is-no-finding
kind: issue
title: An unread body is no finding: DESIGN [operations]' 'an unread body is an unread variable, a finding' has no check
status: dispatched
opened: 2026-10-23
priority: P0
cost: M
refs: [assemble-is-an-operator-and-the-product-is-a-slot]
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`), building test 26 (`docs/INTENT-STAGE3-SPEC.md` §11, row 26: "an unread body is a finding and is not exported").

`docs/DESIGN.md` `[operations]` ("The product", line 1220): "Nothing is derived from which bodies go unread: an unread body is an unread variable, a finding, and a document whose slot reads nothing has an empty product." D′3 builds the slot, so an unread body is not exported (`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`, `the_product_is_what_the_slot_reads`). But no check reports it: `CheckId` (`crates/editor-core/src/checks.rs:80–115`) holds `Connectedness`, `Separation`, `ChartCoherence` and `UnprovenCoincidence`, and nothing reads which `Body` variables no reader and no slot reads.

The spec does not settle the check's shape, so this is a question rather than a build:
- Which variables count. A body nothing reads (a construction whose only reader is deleted, a placement's copy the slot does not read)? Every unread variable of any kind, as the sentence says, or bodies alone?
- Its honesty label and default severity (DS6): it is exact, so certified, but is it `Warn` or `Off` by default? A sketch kept beside the product for reference is unread on purpose.
- Its subject: the variable, or the node defining it.

**Re-priced P0 (2026-10-30).** The sentence is D10 text, ratified in `[ev]` D′ (commit 679b4abbf7, 2026-10-22), so D10 is not built until the check exists. D10's umbrella now waits on this row. It is `design: true`, so the designer pair weighs it first.

## The answer (Ev, PR #6075, 2026-10-30; DESIGN `[operations]` "The product" and `[variables]` "Names" as merged)

**The rule:**
- **Deliberate** means what the person inserted (an operation) or named (a variable, a named output included).
- **Byproduct** means what a door minted on their behalf: an output nobody named, or an anonymous variable. It is never a finding.
- **A use** is a read, being the product, or being what an assertion states. The product is a use, not a reader, so `Doc::reader_counts` is unchanged.
- **Reported:** something deliberate that nothing uses, at the tip of a dead chain. Delete the tip and the next one surfaces.
- **Exempt:** only the world. A `Measure` nothing asserts is reported, per Ev: "simplest … maybe we'll drop it later if it mostly turns out annoying".

**The build:**
1. **A recipe query on `Doc`** (no evaluation, `reads_subject = false`): `Doc::unused()` returns the tip operations and tip named variables. Put it beside `shared_unnamed_vars`. Optionally add `Doc::in_product(var)`, the transitive reach for the viewer's tint, if it falls out of the same graph walk; it is CHROME's to consume, so don't wire the viewer here.
2. **One check resident, `unused`:**
   - certified;
   - `ChecksConfig::unused: Advisory` (Off/Warn), default **Warn**;
   - subjects `FindingSubject::Node`, and a new `FindingSubject::Var(VarId)` spoken by name;
   - story: "<subject> is used by nothing: not the product, not an assertion, and nothing reads it. Recourse: use it, or delete it."
   - A part's unused things are the part's own findings and are not carried into a host.
3. **Before the default lands,** count the findings over the corpus and the demos, and put the count in the PR body. If the count is dominated by one idiom, report it rather than exempting it.
4. **Rows:**
   - a split's unnamed half is quiet; named, it fires; un-named again, it is quiet;
   - a construction's `space` and a revolve's `axis` are quiet;
   - an unplaced fresh construction fires;
   - a named parameter nothing reads fires;
   - a reference body read only by an assertion is quiet;
   - an unasserted `Measure` fires;
   - a dead chain reports only its tip, and deleting the tip surfaces the next;
   - an indexed family is one subject;
   - a part's finding stays in the part;
   - the world is quiet.
5. **Docs:** CHECKS.md gains the resident. `crates/editor-core/VARIABLES.md` VR7 says a named variable may stand unused and is then reported, never removed, and an output is a byproduct unless named. `docs/INTENT-STAGE3-SPEC.md` §5′.3 row 193 and test 26 follow. The row's citation of a `Separation` resident is stale: the residents are Connectedness, ChartCoherence, UnprovenCoincidence and AtRest.

**Review tier:** single FULL review. Dispatched 2026-10-30 to cloud lane `session_01VyvYnMLncpSexG9CKDmPfU`, branch `intent/the-unused-check`. The rule is settled, and the claims to falsify are the graph walk's tips and the byproduct/deliberate line.
