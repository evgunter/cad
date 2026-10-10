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
