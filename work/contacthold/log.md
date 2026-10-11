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
