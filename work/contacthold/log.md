# CONTACTHOLD — the log

## 2026-10-09 — opened

Cut out of CONTACT by CONTACT's orchestrator when it closed (Ev, in
chat 2026-09-29: close CONTACT and move all remaining rows to 2-3
successor programs). CONTACT measured 99.5 budget points against 30;
its units CONTACT-10, -11 and -12 merged (PRs 4363, 4368, 4372) and
CONTACT-13 parked on the D10 hold. Every row here moved by `git mv`
with its id, body and history unchanged. Rows dispatchable: 0, for 0 points.
— (CONTACT orchestrator)

## 2026-10-10 — picked up; the four record rows re-park on B2

INTENT stage 4 B (`coincidences-are-recorded-at-one-door`, PR 4354)
merged, which fired five rows' trigger. Read against the D10 hold:

- Four are about the shape or carrying of `ContactRecords`
  (`boolean-vertex-contact-records-are-inferred-from-values`,
  `curve-contact-names-one-face-where-its-witness-edge-lies-in-two`,
  `topo-surgery-verbs-drop-a-bodys-contact-records`,
  `contact-records-carry-operand-labels-into-the-at-rest-currency`).
  B landed the `Coincidence` record and left `ContactRecords` as it was
  (S4-B ruling, option (b)); B2 (`contact-records-cite-their-decision`,
  live on `intent/s4-b2-records-cite`) rewrites every record to cite its
  decision and folds `push_vv`/`contacts.vf` into Coincidences. Building
  on the record now would collide with B2 and be built twice, so all four
  re-park on B2. The first likely closes with B2 (each value-decided
  vertex record then cites a recorded, linted coincidence — D10's answer
  to its design question); the other three are re-read against B2's
  record. Alternative weighed: dispatch them now and restate after B2 —
  rejected, B2 touches ~99 files over the same types.
- `a-tube-rim-vertex-touching-a-face-along-its-tangent-unions-past-tier-3-prime`
  said "measure first at release": a measure lane (no fix, no review
  tier yet — it builds nothing) on `contacthold/tube-rim-measure` decides
  whether its seat is the vertex-on-face record (B2/E's ground) or vertex
  classification (this track's). The tier is set if it comes back with a
  fix to build.

Track `active`. — (CONTACTHOLD orchestrator)

## 2026-10-10 — the E/F-parked rows read against E's branch; three lanes out

A read-only lane checked the 12 rows parked on E and F against E's
branch (PR 4496) and F's spec. **E doesn't rewrite the contact
verifiers.** `glue.rs::decided_declarations` adds each Zero-decided
undeclared pair to the op's `BooleanDeclarations`, and it then runs the
existing declared path. So `contact_verify::tangent_locus_relation`
becomes the undeclared tangent glue decision too, and the census's
record walks outlive F (they read op records, not declarations). Most
"rewired by E" parking notes don't match E's diff. I checked B2's diff
myself: it touches `rest_pair_verdict` and
`census::confirm_curve_and_patch_records`, not `tangent_locus_relation`
or the census's edge passes.

Dispatched (the rows' own files update on their lanes' branches):

- **contacthold/tangent-locus**: `contact-verify-on-surface-residual-subtracts-its-sag`,
  `contact-tangent-relation-decides-offsets-and-tilt-per-sample`,
  `contact-verify-logs-a-second-order-escalation-its-outcome-overruled`
  (bullets 1 and 3; bullet 2 is the normal case after E). All three are in
  one function, so one lane. Review tier: **full single**. A bound's
  direction per verdict is exactly the class CONTACT's reviews kept finding.
  Seam: E's tangent goldens move; whichever lands second re-baselines.
- **contacthold/census-edge-pass**: `census-edge-pass-reads-no-line-conic-crossing`,
  then `ef-bound-backed-migrates-to-region-confinement`. Its "moot at F"
  premise is wrong: the arms read op records. Review tier: **full single**.
  The lane keeps out of B2's census regions.
- **contacthold/side-code-measure**: `boolean-conic-side-code-zero-is-first-order`.
  Measure only; it also runs on E's branch, where undeclared verified
  tangents reach `tangent_lump`.

Stays parked: `census-declared-sites-read-a-torn-record-as-absent`
(code survives, but B2 rewrites one of its three sites; re-parked on B2);
`a-same-operand-f7-…` (closes with E); `a-bridge-union-…` (closes at F);
`carrier-escalation-…` (flush half dies at F, Rest half sits in E's and
B2's one contact_verify hunk). Held for designers:
`contact-verify-lane-gate-answers-a-crossing-not-certifiable` (same
function as the tangent lane; weigh it once that lane lands) and
`declared-faces-has-no-cross-solid-check`. — (CONTACTHOLD orchestrator)

## 2026-10-10 (afternoon) — E and B2 merged; lanes resumed

All four lanes stopped at about 07:00 on the account's weekly usage
limit, before any of them pushed. While they were stopped, INTENT's E
(PR 4496) and B2 (PR 4533) merged, and INTENT's own tracker commits
released the rows parked on them. I resumed the four lanes from their
clones rather than starting fresh, since their reads of the code were
still useful. Each was told to merge main first, because E and B2 rewrote
the ground under them:

- **tangent-locus**: unchanged scope. A padding change now also moves
  which undeclared tangents glue (E), so goldens may re-baseline.
- **census-edge-pass**: takes `census-declared-sites-read-a-torn-record-as-absent`
  too. B2's census rewrite is on main, so the keep-out is lifted. Its two
  original rows are still `parked` on F on main, and its PR un-parks
  them (they read op records, not declarations).
- **side-code-measure** and **tube-rim-measure**: re-measure on main.

A read-only lane re-reads the four B2-released record rows
(`boolean-vertex-contact-records-…`, `curve-contact-names-one-face-…`,
`topo-surgery-verbs-drop-…`, `contact-records-carry-operand-labels-…`)
against B2's merged record: closed, narrowed, or a question for
designers. — (CONTACTHOLD orchestrator)

## 2026-10-10 — the B2-released rows read; one design fork; wording lane

A read-only lane read the four B2-released rows against merged main:

- `boolean-vertex-contact-records-are-inferred-from-values`: answered by
  B2 (each value-decided vv/vf record cites a `VertexFusion` coincidence
  the lint walks; pinned by `records_cite_their_decision.rs`). What's
  left is wording: DESIGN.md tier 3's "declared contacts ride beside it",
  following B2's approved change, and about 25 "declared-contact record"
  comments. Dispatched to `contacthold/declared-wording`. Review tier:
  **orchestrator's read** (mechanical wording). Its owed audit of other
  Zero-to-distinct-cell doors gets its own row in that PR.
- `curve-contact-names-one-face-…` and `contact-records-carry-operand-labels-…`:
  both are questions about what the record a body carries at rest says, so
  they go to **one designer pair** with one problem statement. The blinding
  byte is on `analysis/design-fork/contacthold-at-rest-record`. C3
  (`crates/topo/README.md`) is ratified, so the outcome is probably an
  `[ev]` PR.
- `topo-surgery-verbs-drop-…`: buildable on B2's pattern, but its
  re-citing addresses `rows()` order, which the fork may change, so it
  parks on the fork.

Held for designers once the tangent lane lands:
`contact-verify-lane-gate-answers-a-crossing-not-certifiable` (same
function as the tangent lane) and `declared-faces-has-no-cross-solid-check`.
— (CONTACTHOLD orchestrator)

## 2026-10-10 — the at-rest record fork goes to Ev (PR 4547)

The designer pair converged on first reports: the at-rest record is an
unordered pair of the body's cells plus its citations, in one list, with
no operand labels, stored granularity or witness. The one difference
(whether a straight edge in a plane gets a stored row) goes to Ev as a
sub-question. C3 and D1 change, so this is `[ev]` PR 4547 (fork log row
108; blinding on `analysis/design-fork/contacthold-at-rest-record`).
Both rows `needs_ev`. Their shared side finding (an operand's records
cross a boolean only via declarations, and F deletes that channel with no
replacement) went as seam notes to INTENT's and WIRE's logs.
— (CONTACTHOLD orchestrator)

## 2026-10-10 — tube-rim measured; the census backstop claimed

The tube-rim lane found the union's record right (a cited `VfContact`)
and the refusal in `census::sweep_cross_solid_backstop` arm 1, which
clears a curved pair only along world axes. That is a filed P1 defect
held twice: RESTREAD's `census-backstop-separates-curved-pairs-only-along-world-axes`
and ORBIT's twin, neither with an orchestrator. I claimed the RESTREAD row
(moved by `git mv`), closed ORBIT's as a duplicate, and made the
tube-rim row its riding witness. Notes are on both logs and both plans
are updated. A lane follows once this is on main.
— (CONTACTHOLD orchestrator)

## 2026-10-10 — wording merged; side-code measured and dispatched

- PR 4545 merged on my own read (comments, one DESIGN.md sentence, one
  panic message). It closed `boolean-vertex-contact-records-…` and filed
  `zero-to-distinct-cells-on-one-point-doors-audit`; the census/validator
  wording tied to `UndeclaredContact` went to INTENT J's row as evidence.
- The side-code measure reproduces on main: a tangent arc that curves off
  is read On to first order (`sectors::side_code`, `Reach::Extent`), and in
  the dip case that makes `sectors::along` refuse a germ with both bounds
  "on" the face. E made the undeclared path reach it. The same lane now
  builds the second-order reading on `contacthold/side-code-order2`.
  Re-priced to cost H (certified second-order bounds). Review tier:
  **dual** (class H, per this plan's posture), under
  `docs/DUAL-REVIEW-PROTOCOL.md`. Its side finding (flush and corner
  postures refuse `odd number of surviving crossing records`, not this
  row's cause) is filed in that PR.
- Friction: the session's disk allowance (about 40 GB) holds about four
  building lanes. One lane's target reached 11 GB, and free space fell to
  5.8 GB. — (CONTACTHOLD orchestrator)

## 2026-10-10 — PR 4550 reviewed (full single); fix pass

The reviewer found the new clear sound (cylinder, cone, sphere, torus and
NURBS reaches enclose their faces in an aimed frame; a `None` reach never
clears) and that it only ever clears more. It added rows: a torus, and
1e-7/1e-6 cuts. **MAJOR**: the row asked to reword the README's
touch-analysis sentence to match the code, and that sentence is C1 of
Ev's PR #3422 ruling, ratified and unbuilt. My brief passed the row's ask
on unchecked, and the lane's `git log -S` missed the line-wrapped phrase.
Class: **a row's request to reword "stale" text is a claim about
ratification, and the brief must check it before passing it on**. The
hunk is reverted; the two parked rows that carry C1 stay. The minors are
in the fix pass: doc placement, the restored planar-first check in
`operand_axes` (it had added extra reach reads to the boolean), the
census passing `sweep_pad` (the aimed frame's rounding contract), and two
filed rows (torus extent overshoot; `bool_pair_gap` missing from the K
ledger). An independent verifier follows the fix pass. — (CONTACTHOLD orchestrator)

## 2026-10-10 — Ev ruled the at-rest record (PR 4547, merged)

"this sounds good. i have no strong opinion on the sub-question;
whatever is more elegant/uniform works." I took the uniform answer:
every pair whose interiors meet is recorded. Fork log row 108 is filled
(A = Opus, B = Fable; both matched Ev). The build is the unit
`at-rest-records-are-unordered-cell-pairs` (H, dual review). The two fork
rows ride with it, and `topo-surgery-…` and `declared-faces-…` park on
it. It's dispatched once disk and slots free up. PR 4558's review came
back with no MAJOR, so a fix pass is out. PR 4550's verifier passed it,
and it merges after its last README word. — (CONTACTHOLD orchestrator)

## 2026-10-10 — PR 4559 verified; PR 4575 to a concurrent dual

- PR 4559's delta review passed (mutations re-run; the line × conic guard
  rows still refuse with PR 4550 merged, held by backstop arm 1's
  `CurvedWithinReach` for the three rows with no overlap cell). I made
  its doc and row nits myself (comments and tracker only). It merges on
  green.
- PR 4575 (side-code order 2, class H): a **concurrent dual pair** on
  frozen head `5ca970782b` under protocol `da5dbd1532`, with identical
  briefs and claims (the In/Out lift bound, the On height bound, the
  narrowing's two regression measurements, the pins, `ConicBend`'s wiring,
  the dip correction). R1 runs in the implementer's clone, R2 in its own;
  the method is the same. The census lane's space is reclaimed for R2.
  — (CONTACTHOLD orchestrator)

## 2026-10-10 — PR 4559 merged; PR 4575's dual adjudicated; the record build dispatched

- PR 4559 (census) merged after a verified fix pass. Its three rows are
  closed.
- PR 4575 dual (concurrent, frozen `5ca970782b`): R1 and R2 each gave
  APPROVE-WITH-FIXES with no MAJOR, so there are no tally candidates.
  Bilateral findings: the certified lower bound is unpinned (surviving
  mutants); `ConicBend`'s text is false; the dip's HONE attribution
  doesn't hold; the residue row has no reaching reproduction; the
  splitting sibling `enters_material_order2` certifies less; the refusal
  comes before the lump. The pair is flagged: R1 glimpsed R2's command
  line (method only). R2's `ConicBend` census was not run (build slot),
  so the fix pass runs it. The union went to the implementer's fix pass;
  the pre-note is held for the DR row at merge.
- Disk freed (R2, census and tangent lanes reclaimed: 20 GB), so
  `at-rest-records-are-unordered-cell-pairs` is dispatched to
  `contacthold/at-rest-cell-pairs` (H, dual review). — (CONTACTHOLD orchestrator)

## 2026-10-10 — PR 4558 merged

The tangent locus PR merged after a full single review and a verified fix
pass, which closes its three rows. The lane pushed one empty commit to
re-trigger CI before it found the merge conflict that was blocking the
run. That is a rule slip, left in history (merge-only). The plan now
lists the rows this session's reviews filed. — (CONTACTHOLD orchestrator)

## 2026-10-10 — the tangent-tolerance fork goes to Ev as a split (PR 4595)

A designer pair weighed `tangent-offset-and-tilt-are-two-epsilon-checks-…`
(blinding byte 123 on `analysis/design-fork/contacthold-tangent-one-displacement`).
Both rejected the filed premise (the offsets are perpendicular, √2ε; no
sum). They crossed three times over rounds 2–4 on whether to add a
per-sample separation row, and the question beneath both got named: does
a served `Tangent` claim the pair touching within ε (a relation, like
`Rest`) or a witness within ε of each surface (like `Intersection`)?
That stated split is `[ev]` PR 4595 (fork log row 111); the diff carries
the text-only option's wording. Both found
`rim_wedge::same_circle`'s three-row rim identity, filed on HONE.
— (CONTACTHOLD orchestrator)

## 2026-10-11 — the container restart; PR 4575's fix pass; the record stack up

- A container restart stopped both lanes. The record build resumed from
  its transcript. PR 4575's fix-pass agent was lost, but its worktree
  survived with half the fixes uncommitted, and a fresh lane finished
  them. It found and fixed a soundness hole in the earlier circle height
  bound. It briefly pushed the old lane's census logging while
  recovering the WIP, then removed it in the next commit. The census
  found no existing row newly refusing `ConicBend`. R1 is verifying.
- The record unit landed as a stack: PR 4598 (the type, every producer
  and consumer, the census clearing exactly what it did) and PR 4604
  (the census reading carriers, edge–face and face–face rows,
  `CurvedRestUnrecorded` retired). Both are green. Each gets a concurrent
  dual, one after the other: disk doesn't hold four reviewer targets at
  once, and the protocol (rule 4) says to reduce simultaneous duals rather
  than narrow a method. PR 4598's pair dispatches once PR 4575 merges and
  its target is reclaimed. — (CONTACTHOLD orchestrator)

## 2026-10-11 — Ev chose (a) on the tangency claim; PR 4598's dual; my two slips

- **PR 4595 merged.** Ev asked what (a) would cost. I answered: one
  per-sample row beside the two residual rows in each decider, one
  `CertCheck` variant, σ from the gradients in the certifier, about
  50–80 lines. Ev chose (a): a served `Tangent` claims the pair touching
  within ε. C7/C4 state it, and row 111 is filled (B's final position).
  The row `tangent-offset-and-tilt-…` is now the build, dispatched to
  `contacthold/tangent-gap` (review tier: **full single**; it moves two
  deciders' contradiction recourse).
- **PR 4598's dual** (frozen `12cb9a6cf0`): R1 and R2 each gave
  APPROVE-WITH-FIXES with no MAJOR, so there are no tally candidates. Both
  showed by execution that no clear moved and that citations stay
  faithful. Both found the same gaps: no citation row past index 0; the
  census incidence arms unguarded (R1 executed a surviving mutant that the
  merge base killed); the cusp half dropped on a false premise; stale
  curve-record prose. R1 alone: `rekeyed` can build incident and duplicate
  pairs. No glimpse either side. The union went to the implementer's fix
  pass; PR 4604's dual follows it.
- **PR 4575 (side-code)**: DR row renumbered twice as other PRs merged
  theirs first (153 → 157 → 158). **Two slips of mine**: I pushed a
  merge-with-main resolution without compiling it, and main's new
  `vtxfac` test then failed to build. That one is fixed and compiled
  locally. Then a DR-log conflict silently stopped CI, which I caught on
  a zero-checks read. Both later merges were compile-checked before
  pushing. — (CONTACTHOLD orchestrator)
- 2026-10-11 `boolean-conic-side-code-zero-is-first-order` closed: PR 4575 merged (DR-164); the residue rides `conic-ties-outside-the-vertex-on-face-pass-still-read-first-order`.
- 2026-10-11 `at-rest-records-are-unordered-cell-pairs` closed: PR 4598 and PR 4604 merged (both dual-reviewed; 4604's crescent MAJOR tallied). Passengers `curve-contact-names-one-face-where-its-witness-edge-lies-in-two`, `contact-records-carry-operand-labels-into-the-at-rest-currency` and FUSE's `a-record-on-a-curved-edge-resting-on-a-face-refuses` closed with it; `topo-surgery-verbs-drop-a-bodys-contact-records` and `declared-faces-has-no-cross-solid-check` unparked.
- 2026-10-11 — Note from FLUSH: filed `a-plate-at-a-frustums-support-unions-into-two-disjoint-lumps` (P0) on this slate. GERM's `operand_gate_support_plates` slow row has been red on main since one of #4667, #4598, #4597, #4636, #4573, #4604, #4671: a union of a plate touching a frustum or torus at its support builds two disjoint lumps. #4604/#4598 are the likely suspects from the symptom, but it is not bisected. Re-home it if the bisect lands elsewhere. (FLUSH orchestrator)
- 2026-10-11 `tangent-offset-and-tilt-are-two-epsilon-checks-where-rest-is-one-displacement` closed: PR 4640 merged (single review, APPROVE-WITH-FIXES; delta clean). A served Tangent reads the touch s = r₁ − σ·r₂ in both deciders. Residue filed: `tangent-parallelism-between-samples-is-certified-by-no-row` (P4).
- 2026-10-11 PR 4694 (`topo-surgery-verbs-drop-a-bodys-contact-records`): dual tier (one carry API across every surgery verb, hard to change later); letter M, byte 97 → SEQUENTIAL. Main's support-plates slow-set red bisected to PR 4604's own diff (one-point rim EdgeFace records now ship as Assembly); measurement and reading posted on GERM's row, CLEAVE holds the fix.
- 2026-10-11 `topo-surgery-verbs-drop-a-bodys-contact-records` closed: PR 4694 merged (dual tier, sequential arm, one review, no MAJOR; delta clean). Hotfix PR 4716 merged: main did not compile topo (4613 × 4650, `piece_holds`). Filed from the same-side fork's designer pair: `the-boolean-records-no-line-touch-and-the-census-does-not-demand-one` (P2), `a-touch-only-rim-with-no-derivable-locus-refuses-every-op-as-a-cusp` (P3, measure first), `a-same-side-slab-past-the-round-stops-at-curved-boolean-unsupported` (P3). Sequencing call: the same-side build lands the short-circuit's removal and the `ee` writer in one PR (designer B's shape); the alternative, the removal alone, matches the planar state today and was not taken because the row is owed and the writer sits in the same function.
- 2026-10-11 PR 4704 merged (holdout dual; R1's same-side MAJOR bilateral, 0 tallied): `a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex` and TANG's moved `the-capsule-rod-ending-on-the-joint-parts-by-member-order` closed. The same-side fork went to two designers (byte 60), who converged after one round: delete the short-circuit, ∪ two solids with an edge–edge record, no refusal, C7 routes a junction only. Now `[ev]` PR 4723, fork log row 116.
- 2026-10-11 `surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key` closed: PR 4730 merged (single review; re-routed onto main's `confirm_records` rename before merging; CI red only on main's sweep serial-golden test, P0 on NURBS with its fix in PR 4684). Filed from the PR: `a-cut-through-a-recorded-touch-mints-a-pair-no-record-backs` (P1, design), `the-boolean-and-the-surgery-carry-place-a-divided-cells-record-by-two-routes` (P1, design), `the-shell-seam-piece-carry-has-no-fixture` (P3). The two design rows go to one designer pair together.
- 2026-10-12 `a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector` re-measured and parked on `[ev]` PR 4723: since PR 4604's crescent read the glue door refuses the dip's overlapping tangency (`contact_tangent_overlap`), so the round's wall sector is undeclared and C8 refuses `CurvedBooleanUnsupported` at every op; the old `sectors::along` refusal existed only under a false `Tangent` auto-declaration. How an overlapping tangency is decided is a design question on C7, weighed after 4723's ruling.
- 2026-10-12 `the-boolean-records-no-line-touch-and-the-census-does-not-demand-one` closed: PR 4740 merged (holdout dual, byte 3; no MAJOR, 0 tallied; one fix pass, delta read by the orchestrator). The record-carry fork (three rows: the cut-through-a-touch pair, the two placement routes, a consumed cell) went to one designer pair together (byte 13); they converged on one lineage and one carry with consumption on declared removal, split on whether a pair an op mints on a touch needs its own decision: `[ev]` PR 4749, fork log row 117. Filed: `a-divided-faces-holder-drops-a-record-whose-vertex-is-on-a-piece-boundary` (P2, from the fork), and on INTENT `a-wide-loft-section-escalates-chord-side-at-1e-12-on-main` (P0, main red from PR 4741 met on PR 4740's CI). The CONTACTHOLD dual-review rows since PR 4598 named protocol `da5dbd1532`; corrected to `24b76fff0e`, the protocol in force.
- 2026-10-12 `a-plate-at-a-frustums-support-unions-into-two-disjoint-lumps` and `a-curved-rest-union-builds-as-disjoint-since-4604` closed with GERM's `main-reds-on-plates-at-the-exact-support-of-a-frustum-and-a-torus` (closed, CLEAVE's PR 4707): the frustum's rim touch is excluded under ∪ as under ∩, so the union ships two solids and a record (D1's pieces rule), which the row now asserts.
- 2026-10-12 Ev approved `[ev]` PR 4723 ("sounds good!"): two operands touching only along a tangent line answer every op (∖ the operand, ∩ empty, ∪ two solids with the edge–edge record), the same-side short-circuit goes, C7 routes a junction only. Fork row 116 filled in: both designers matched. The build is dispatched on `a-same-side-tangent-touch-refuses-at-every-op`; `a-touch-along-a-glued-tangency-cites-no-tangency-decision` becomes live with it.
- 2026-10-12 Ev approved `[ev]` PR 4749 in the per-op form (the split cites the record, the boolean decides, others refuse; the cutter field deferred until needed, the conversation revisited then): fork row 117 filled in. Filed: `a-minted-pair-is-decided-per-op-with-no-cutter-field` (P4, design, the revisit trigger) and `a-boolean-coincidence-names-a-minted-vertex-by-its-input-edge` (P3).
- 2026-10-12 The overlapping-tangency fork (the dipping round, unparked once PR 4723 merged) went to a designer pair (byte 117), who converged in the first round: one claim-free `Tangent` glue class, `Contact(Tangent)` and `Seam` collapse, consumers derive the side. `[ev]` PR for C4's seam text, fork log row 118. Filed from the pair: four P3 rows (the glue door swallowing escalations, `vtxfac` offering a retired declaration, the at-rest shared crescent's verdict, `verify_declared_contacts`' declared bit).
- 2026-10-12 Ev approved `[ev]` PR 4775 ("yes this sounds good!"): the glue door decides a tangency with no material claim; fork row 118 filled in, both designers matched. The build is dispatched on the dipping round's row.
- 2026-10-12 `zero-to-distinct-cells-on-one-point-doors-audit` closed: PR 4765 merged (dual tier, sequential arm, byte 212; one review, no MAJOR; one fix pass, delta read by the orchestrator). The one defect, a boolean pinch recording no coincidence row, is fixed (`ops::pinch_rows`). Filed from it: `an-inherited-pinch-decided-onto-the-ops-face-records-nothing` (P2, design); WIRE's n-ary pinch row widened to intersect and raised to P2. The one-carry build for PR 4749's ruling is dispatched on top of it.
- 2026-10-12 PR 4781's CI drew a second seed in GERM's in-band sweep fuzz class (`0x50cc8da379c68fde`, replays on main); added to GERM's row.
- 2026-10-12 `a-same-side-tangent-touch-refuses-at-every-op` closed: PR 4781 merged (holdout dual, byte 189; no MAJOR, 0 tallied; one fix pass, delta read by the orchestrator; main merged in twice over conflicts). Two operands touching only along a tangent line answer every op per Ev's ruling on PR 4723. The claim-free `Tangent` build (PR 4775's ruling) is dispatched on top of it, with the dipping round's row.
- 2026-10-12 `two-second-order-conic-readers-certify-one-fact-differently` closed: PR 4826 merged (single review, APPROVE-WITH-FIXES; one fix pass, delta read by the orchestrator). Both lanes read a conic's side through one certified reading in `geom_brep::conic_bend`; the fix pass reads the lift at arc length, so the review's circle window answers. The first implementer stalled silently for ~8 h (no file change after 00:24, no reply to a status ping); it was stopped and its WIP saved and resumed by a fresh lane.
- 2026-10-12 PR 4809 (DR-207) and PR 4820 (DR-208) merged: the one carry, per Ev's ruling on PR 4749. Holdout pair, one tallied MAJOR (R1's pit regression), an M-tier miss: short readout `m-tier-miss-at-dr-208-the-booleans-carry-refused-a-pit-under-a-carried-rest` to Ev. Main merged in over FUSE's PR 4868 (its checked lineage lists kept; one walk, `Descent`).
- 2026-10-12 PR 4831 merged (sequential, byte 4; one review, no MAJOR; two fix passes, each delta-verified by the reviewer — the first was not clean, a short-edge `ClassificationInvariant` regression, fixed by reading each bound over its reach). The glue door decides a tangency with no material claim, per Ev's ruling on PR 4775; the edge–sector lane lumps a glued tangency. Closed: `a-same-side-slab-past-the-round-stops-at-curved-boolean-unsupported`, `verify-declared-contacts-bridges-an-exact-zero-for-auto-glued-pairs`, `vtxfac-offers-a-tangent-declaration-on-a-boolean`. Open with evidence: the dipping round (`along` remainder; the bore in a disc), the escalations the door swallows (C8 unmet for in-band pairs), the bisector row (P3), the declaration-name row.
- 2026-10-12 The self-touch fork (the bridge row) went to a designer pair (byte 70), who converged on the record form in one round: `[ev]` PR for the D1/C7 wording, fork log row 129. The face–face row build is dispatched.
- 2026-10-12 Ev took the self-touch record form on `[ev]` PR 4914 ("sounds good!"); fork row 129 filled.
- 2026-10-12 The self-touch fork row is 129 (main took 127 and 128 while PR 4914 waited).
- 2026-10-13 The carried-unread-record fork converged on read-at-mint, carry-by-lineage and a point-touch reading; `[ev]` PR for the D1/C3 text, fork log row 132.
- 2026-10-13 PR 4921 merged (the face–face touch row, the carried ef/ff kinds); the bridge row closed. The carried-unread-record fork went to a designer pair (byte 127), who converged in two rounds.
- 2026-10-13 PR 4918 merged (the glue door escalates in-band tangencies typed; the identity ladder's one home): its row, the sag-route row and TANG's capped-tube row closed.
- 2026-10-13 Ev on `[ev]` PR 4951: point-touch reading and lineage carry as recommended; the mint-time confirm folds into un-parking the door's result gate. Fork log row 132.

- 2026-10-13 — **Red main, from a semantic conflict between PR 4918 (contacthold) and PR 4197 (SYM-17).** Main at 0c28d7f4 and at 4dce8298 does not compile sweep's tests.
  - 4918 (merged 13:45Z) added `sweep/tests/a_circle_of_one_radius_elsewhere_is_no_rim.rs` and `a_face_off_the_tangent_ruling.rs`, which call `topo::transform_rigid` with an `&Affine3`.
  - 4197 (merged 15:35Z, from a branch whose last main merge was 11:54Z) changed it to take `&RigidMotion`.
  - **Fix:** PR 4992 (`nurbs/fix-main-sweep-rigid-tests`), test-only. It wraps the four sites in `geom_core::test_support::rigid`. It merges on CI.
  - **Class:** a signature change's re-spelling pass covers only the callers its branch can see. A branch that last merged main hours before it lands misses callers added since. — (NURBS orchestrator)
- 2026-10-13 PR 4956 merged (the census reads point touches) and PR 4986 merged (review_4781's eps pins scaled to the band; the corner-sample defect filed): their rows closed.
- 2026-10-17 — Seam note from RESTREAD: six census record/finding rows moved here parked on `d10-one-way-to-say-intent-is-unbuilt` (RESTREAD's plan: a row needing declared pairs or contact parks and moves to CONTACTHOLD) — `an-escalated-census-finding-names-no-pair`, `census-face-rungs-for-a-collinear-overlap-are-unconfined`, `a-two-carrier-curve-touch-has-no-bounded-locus`, `an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`, `census-face-pair-confirm-is-a-trial-ladder`, `the-two-carrier-face-pair-confirm-escalates-where-its-second-face-reads-the-point-out`. Your PRs 4982/4996/5006 sit on the same confirm and carry code. (RESTREAD orchestrator)
- 2026-10-17 — Seam note from WIRE: `a-boolean-drops-its-operands-curve-and-patch-records` moves here from WIRE, parked on `the-at-rest-census-is-a-check`. Its title is overtaken (`ContactRecords::carried` files all six kinds). What remains is a cut's implied end records and the three-member witness, which waits on your `a-union-fold-step-mints-no-face-face-row-for-a-given-tangency`. Every file it touches is topo's. (WIRE orchestrator)
- 2026-10-18 — Seam note from FUSE, which closes (`docs/doc-ledger/fuse-leaves-the-tracker.md`): the carried-record rows `a-booleans-records-are-complete-only-relative-to-the-records-passed-in` and `a-carried-line-contact-an-op-cuts-is-dropped`, and `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`, parked on this slate's `an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`, moved here by `git mv`, ids kept and bodies unchanged but for re-pointed `work/fuse/` citations. (FUSE close lane)
- 2026-10-19 — Seam note from RESTREAD (`restread/cross-solid-apart`): `census::sweep_cross_solid_backstop` arm 1 gains a third clear after the world axes and `backstop_apart`, a gap in depth about an axis, a centre or a core circle (new module `census/depth_gap.rs`, one line at the `CurvedWithinReach` push). Touching pairs keep `CurvedWithinReach`: the record/contact half stays on [ev] PR 4666. Pins that admitted `CurvedWithinReach` on nested bores and plugs now demand `Ok` (`s16_box_soundness`, `an_annular_tube_through_a_plate`, `a_plane_across_a_one_face_wall`); `touch_union`, `tube_rim_touch_rows` and `bisector_nodes` are unchanged and green. (RESTREAD lane)
- 2026-10-19 PR 5006 merged (one tangent-interiors reading for op and census, cut at boundary crossings, true extents): its rows closed.
- 2026-10-19 — The nightly's reds of 2026-10-17 to 10-19 bisected (`contacthold/nightly-reds`). None of them is CONTACTHOLD's. The one on CONTACTHOLD's own probe, `review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch` at 1e-12, dates from PR 5162 (`fuse/curve-pair-reader-c2`). It stays on CPAIR's `a-same-side-touch-pose-escalates-the-curve-pair-extreme-at-1e-12`, with the bisect added. New rows were filed on PCERT and FLUX. The others were existing rows, which now carry their culprit merges, or were already fixed on main.
- 2026-10-19 PR 4996 merged (the carry places by lineage; an image that no longer holds its cell's geometry is read): its row closed.
- 2026-10-19 PR 4982 merged (a union fold step mints the face–face row of a given tangency, carrying the pass's own line): its row closed. It also listed `d1_tabs_a_few_bands_wide` once in the slow set, clearing main's red name check.
- 2026-10-19 `a-rim-locus-tangency-through-two-faces-interiors-mints-no-face-face-row` parked behind TANG's `dev1-cylinder-sphere-circle-locus-arm`: no circle witness locus exists, and the rim arm's `Ok(false)` is a theorem of its routing; probe poses recorded on the row; the touch-only consumer noted on TANG's row.
- 2026-10-19 `[ev]` (branch `contacthold/ev-pinch-crescent`): one solid pinched along a tangent line through two faces' interiors, weighed by a designer pair (fork-log row 182). Both agree, and the diff carries it: same-solid tangent-touch discovery is owed (C2), and a crescent shared by two distinct solids is interference (C4). Split for Ev: the material-crescent pinch (plate ∖ a bore tangent from inside) as one face–face record (B) or two coincident cusp edges (A). The same-solid discovery row is flagged `needs_ev`, `design`. Off-question defects filed: `the-at-rest-face-face-confirm-bridges-an-in-band-relative-curvature` (here), RESTREAD's `confirm-face-pair-reports-a-face-face-record-as-a-declared-tangent-contact` and `the-edge-edge-and-point-record-confirms-read-no-material-sides`, RESTFRONT's `validate-rs-reads-the-doubled-cusp-as-the-kissing-union-two-slit-edges`; a supersession note on CLEAVE's closed `tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line`.
- 2026-10-19 PR 5389 merged (two tubes in the zero band answer alike in either order: a covered circle with no end on the carrier defers to the settle stage): its row closed.
- 2026-10-19 PR 5390 merged (a face–face record on divided faces lands on the piece pair holding its line; line-then-table held by type): its row closed. The bore row it filed waits on the `[ev]` pinch-crescent fork (needs_ev).
- 2026-10-19 Track handed back (`ready`): no dispatchable P0 or P1 (the P1 waits on `[ev]` PR 5395; the P0s on intent stage 4). Plan slate rewritten.
- 2026-10-19 Pinch fork round 3 (measured, on Ev's framing) added to `[ev]` PR 5395 and fork-log row 182: converged on point touches (records; a shared crescent at a point legal inside one solid, so C4 states it); the line case stays split on one reading of PR 4795. W1′ witness (seam on the ruling: edge–face record, 3′ passes; off: face–face record, 3′ refuses) added to the bore row and to RESTREAD's `the-edge-edge-and-point-record-confirms-read-no-material-sides`; the filed rows' line citations refreshed for main.
- 2026-10-19 Pinch fork round 4 (merits only, after Ev rejected the reading-of-PR-4795 frame) added to `[ev]` PR 5395 and fork-log row 182: both hold, both now "likely, close"; the trade-off is symmetric (the record form splits faces at a disconnecting far cut, the edge form re-pairs at a complement). The through-bore slot witness (unmeasured) noted on the bore row.
- 2026-10-20 Ev decided the pinch fork on `[ev]` PR 5395: the edge form (a material crescent along a line is two coincident cusp edges, no record; void crescents and point touches are records; a shared crescent at a point inside one solid is legal). Designer B withdrew its dissent in round 5. Written into D1 tier 3, C7 and C4; the bore, discovery and shared-crescent rows answered; filed `a-material-crescent-tangent-line-builds-two-coincident-cusp-edges` and `a-material-crescent-face-face-record-refuses-typed-at-the-op-and-the-carry`; evidence on RESTREAD's `census-passes-a-face-inside-an-edges-material-wedge` (the disjoint-sector check, already owned there).
- 2026-10-20 `[ev]` PR 5395 merged (the edge form; fork-log row 182). The track is held again: the P1 same-solid discovery is dispatchable.
- 2026-10-20 Nightly-reds lane on the 2026-10-20 nightly (run 39375288176): none of the five new reds is CONTACTHOLD's. Bisected: `ellipse_constructor_trios` and the meeting-axes pose → GERM PR 5156 (ellipse re-pinned by ENCL PR 5451; pose filed on GERM); `reader_census` and the tier-1 roster → ENCL PR 5418 (census filed on ENCL; tier-1 already filed); the reshaped rod's census → PATHS PR 5068 (evidence added to CENSUS's existing row).
- 2026-10-20 PR 5456 merged (the same-solid discovery, line lane; `CuspEdgesMissing`): the P1 closed.
- 2026-10-20 Track handed back (`ready`): no dispatchable P0 or P1.
- 2026-10-23 — Seam note from INTENT (PR 5605, `intent/census-planar-face-pair`): the census confines face pairs by location (`docs/DESIGN.md` `[assertions]`, Ev). A touch of two confirmed faces' boundary cells is the pair's only in the closure of its overlap, and elsewhere a pair of its own. `vv_face_backed`, `ve_face_backed`, `sides_face_backed`, `ee_face_backed` and `vf_face_backed_confined` are gone. `census-face-rungs-for-a-collinear-overlap-are-unconfined` is closed by it. `declared-only-meetings-clear-at-the-census-gate-unread`'s premise is restated: the meetings are location-confined now, but still on the pair's word. (INTENT lane)
- 2026-10-25 — **Band rule (work/README.md, Priority):** `an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane` banded P2 (was unbanded): it gates the parked P2 `a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only`; its own blocker, the D10 hold, is P0. (band-rule triage)
- 2026-10-27 — (seam, CLEAVE PR 5842) The volume backstop left the boolean's operation path under `[ev]` PR 5780: closed `a-round-lifted-within-band-off-the-rim-refuses-its-union-on-the-volume-bound` — the in-band lifted unions build within `1e3·ε` of the closed form (`r1_turned_about_the_rim`, measured).
- 2026-10-27 — `intent-stage4-is-built` closed on 2026-10-24, but no one re-read the two P0s parked on it. They are `CONTACT-13` and `declared-only-meetings-clear-at-the-census-gate-unread`.
  - Their wrong clears were built by hand-pushed declaration records, which main no longer allows.
  - What is left is the census's record-only reading, which is ATREST's ground.
  - Both moved to `work/atrest/` by `git mv`. `CONTACT-13` closed into the issue row, which is now one P2 re-read (Ev in chat).
- **2026-10-30, picked up by (INTENT orchestrator)** for its one P0, `a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band`. It is the last row D10's umbrella (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`) waits on, and stage 6 is closing.
  - The row is `design: true`, so the designer pair is dispatched first (one Opus, one Fable, identical briefs, 05:16Z). Its blinding byte is on `analysis/design-fork/glued-pair-reread`.
  - An `[ev]` PR follows if the fork is Ev's. The track is handed back once the P0 is done.
- **2026-10-30, designers agreed** on the P0 (no `[ev]`; the answer is in the row). It was dispatched as one unit, with the at-rest confirm, declaration names and README C4 rows folded in. Review tier: dual H.
- **2026-10-30, the P0 merged (#6079)**, dual H, tally 0 (row `PR-6079.md`). Glued pairs read their record; the declared posture, the public declaration surface and the declaration names retire.
  - Filed from review: `a-fold-step-glues-a-pair-its-door-reads-in-band` (P1, design) and `the-declaration-read-routing-survives-the-declarations` (P3).
  - The track is handed back `ready` by the INTENT orchestrator; its P1 design row is next, for a designer pair.
- 2026-11-01 — Note from TANGRIM: claimed a row from this program by `git mv` into `work/tangrim/` (the one bound-side reading decided for the tangent crescent; fork row 229). See TANGRIM's log. (TANGRIM orchestrator)
