# ATREST-8 — the at-rest findings fit the viewer and name the right recourse

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-8.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/8-prose`. Rows carried:
`work/atrest/atrest-refusal-prose-outgrows-the-viewer` (Ev's concision
request, P1), `work/atrest/census-lane-unsupported-display-names-the-scalar-not-the-door`,
`work/atrest/census-witness-string-repeats-the-subject`.

## The standard

Stated once, in `work/chrome/error-and-check-text-overflows-its-region.md`
§"The standard a refusal is rewritten to". Read it in full; do not
restate it. In short: what could not be done, the short reason, and what
the user can do — **the recourse is never dropped**; developer detail
(arena keys, predicate names, doc paths, issue numbers) lives in the
rustdoc or the `Debug` payload; **75 words on the RENDERED text**;
prose only, types and payloads unchanged; a test on the old text is
re-baselined, never weakened.

## What to do

1. **The five long `ValidationError` arms** the carried row lists
   (`LaminaWedge`, `TangentNotIntrinsic`, `InstanceInterference`,
   `ScaffoldAtRest`, `CensusLaneUnsupported`) — re-take the census
   first on today's tree (ATREST-3..7 may have added or changed arms;
   include any new arm over budget), then rewrite each to the standard.
2. **`CensusLaneUnsupported` names the wrong thing twice** (the second
   carried row, in full): the refusal names the SCALAR where the fact is
   the DOOR's — the `_structural` doors hold no certified chart lane at
   any scalar by H5 ruling 3 — so its recourse becomes the certified
   `validate_pseudomanifold` family at a certifying scalar; and it names
   "the conformal face-pair arm" where the declared-record confirm arm
   raises it too. Fix the `Display`, the variant's rustdoc, and
   `census::tests::lane_unsupported_sentence`. The door's VERDICT is
   ruling 3's letter and does not change. The same class outside `topo`
   (`crates/editor-core/src/assembly.rs`, named in the row) — fix it if
   it is prose, or file it on its owner's slate if it is more.
3. **The census witness strings** (the third carried row): the two
   `census.rs` sites that fill the `witness` slot — documented as a
   POSITION and rendered after "at" — with a repeat of the face pair.
   Put the witnessing position there, as the other seven sites do; if
   no position exists at a site, say what the slot then carries and
   make the `Display` read correctly for it.
4. **A guard.** The budget is enforced today for the Boolean family
   only (`editor-core/tests/refusal_concision.rs`). Add every
   `ValidationError` arm the viewer's checks window renders to a budget
   row of the same shape, on a representative payload, so the next
   arm that outgrows the window reds. Say what the row cannot see (an
   arm the census did not reach).

## Concurrency

ATREST-3..7 touch `validate.rs` and may add variants or edit variant
docs. Merge `origin/main` whenever it moves; a new arm that lands during
this unit is in scope for step 1.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live. Do not touch `work/atrest/log.md`. Do not merge. Set the
three carried rows to `review` when you open the PR.

## Review tier

**Single, STYLE**: prose at the source plus a guard; the one code change
(the witness slot) is small and readable. Class S / PROSE.
