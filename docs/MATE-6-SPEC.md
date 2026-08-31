# MATE-6 — issue 946: minting moves to evaluation (the Q1 ruling executed)

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged in the plan's opening commit: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the RULING — `docs/S-MATE-PLAN.md`
§Rulings item 1 (Evan, in-chat 2026-08-31) — which is a
DRIFT-CLOSURE against ASSEMBLY-DESIGN A3's own ratified sentence:
*"Evaluation carries each mate's declaration into the evaluated
body's contact record set."* The code implemented minting in
`assemble` instead, and the instantiation seam inherited the gap
issue 946 documents. ASSEMBLY-DESIGN needs no edit; the drift was
the code's (the issue-945 shape).

## Situation

`assemble` = `product_recorded` + `mint` + tier-3′
(`crates/editor-core/src/assembly.rs`; minting near line 435). At
the instantiation seam only the gather runs
(`eval::parts::resolve_and_evaluate` → `product::product_recorded`),
so a sub-assembly's boolean-minted records survive (R2-b row 1's
descendant-map pin) while its MATE-minted declarations are lost —
the outer gate then reports `UndeclaredContact`, the F1 hard error,
for contacts the inner document declared perfectly well
("three identical stands in a row"). Note the inner SOLVE already
runs at the seam (the instantiated geometry arrives correctly
placed): minting is the one act missing, and everything it needs is
already computed there.

## The ruling, restated as the contract

Minting moves into the product gather UNIVERSALLY — every evaluated
product carries its mate-minted records, `assemble` = product +
tier-3′. Construction composes; verification runs once, at the
outermost gate. Soundness: the outer census re-verifies everything
it consumes — crossings re-verify (#591), and a carried declaration
the outer geometry refutes lands `StaleContactDeclaration →
Refuted` naming its mate.

## Deliverables

1. **The persistence premise, verified FIRST.** The ruling was
   taken on a by-eye check that minted records are evaluation-side
   only and the persisted recipe carries the Mate node (class +
   alignment), so no schema change is implied. Verify against the
   code. If minted records reach ANY persisted state, STOP and
   report — schema is contested territory (M10 holds the live
   versions) and the ruling gets revisited, not stretched.
2. **Move `mint` from `assemble` into the product gather** so the
   gather's output — every consumer's product — carries mate-minted
   declarations; `assemble` keeps tier-3′ and its refusal surface,
   minting nothing of its own.
3. **The seam, red-first from issue 946's own shape**: an inner
   document whose validity depends on its mates, instantiated ×3
   into an outer document. On main: the outer gate's
   `UndeclaredContact` hard error (quote it in the PR). After: the
   inner declarations arrive under the graft's descendant map and
   the outer gate is green. One nesting level deeper (outer–mid–
   inner) gets a row if cheap; otherwise its status is STATED.
4. **Outer re-verification with teeth**: a carried declaration the
   outer geometry refutes (move the outer placement so the inner
   seat no longer holds) lands `StaleContactDeclaration → Refuted`
   naming its mate — its own row.
5. **Behavior invariants**: a document with no mates gathers
   bit-identically; single-document `assemble` outcomes unchanged
   (the existing assembly/gate suites are the oracle); the
   MATE-1-adopted probe rows stay green untouched.
6. **Class sweep** (discipline §5): the genus is "a consumer whose
   correctness assumed records are minted only under `assemble`" —
   sweep product/gather consumers (validation, census entry,
   update re-verification, split/inline, persist round-trip,
   pncad façade readers) for sites that would double-mint, drop,
   or double-count now that the gather mints; hit list with
   per-hit disposition, blind spots stated.

## Acceptance

- Red-first demonstrated from main; the seam rows and the Refuted
  row green; existing editor-core suites green; no demo edits.
- The retired refusal reach classified against the D2 addendum
  (row 2: the seam's `UndeclaredContact` misfire) in the PR body.
- ε posture (issue-1356): structural unit — argue
  band-independence or say where a numeric enters; state which
  point gated, drawn or asked.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 946" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the
  issue after merge.
- Scope fence: `crates/editor-core/src/` — `assembly.rs`, the
  product gather path (`product.rs` GATHER ONLY: the Dual arms are
  M10-4's ratified territory — touch nothing Dual; merge main
  frequently), `eval/` where the seam requires it, and
  `crates/editor-core/tests/`. Nothing else — no `crates/topo`,
  no schema files (item 1's STOP rule), no `pncad`/`pncad-py`
  surface changes (record any façade gap as a finding for LIB),
  no `docs/` beyond nothing (ASSEMBLY-DESIGN needs no edit — that
  is the ruling's point), no `docs/MODEL-AB-LOG.md`, no
  `docs/S-MATE-*.md`.
- Sibling implementer lanes are running concurrently on disjoint
  files; builds may be slow, and you MERGE MAIN before opening the
  PR and whenever it moves.
- Commit and push after every coherent unit of work (branch
  `mate/6-mint-to-eval`).
