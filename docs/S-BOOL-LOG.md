# S-BOOL log — boolean reach and containment

Narrative record; the plan is `docs/S-BOOL-PLAN.md`, the charter
`docs/WORK-STREAMS-2026-08.md` §S-BOOL. Convention as in the other
programs: seam entries at pipeline seams, unit entries at merges, the
tail is the live state.

## Opening state (2026-08-31)

Opened on Evan's direction (in-chat: "you can also take S-BOOL if
that's not claimed yet" — verified unclaimed against docs, branches
and open PRs), by the S-MESH orchestrator in the same opening PR. The
plan is a DRAFT design conversation for its **Rulings sought**
section; BOOL-1 is dispatchable pre-ratification as a charter-named
defect fix whose reproduction is already pinned `#[ignore]`d on main
(recorded here as a unilateral decision).

**Operational facts, recorded once:**

- **Branch prefix (the #396 convention): `bool/`** — unit branches
  `bool/<unit>-<slug>`, orchestrator branch `bool/orchestrator` (the
  opening PR rides the S-MESH session branch; see that log).
- **A/B ordinal band: S-BOOL = 1100–1199**, claimed in
  `docs/MODEL-AB-LOG.md`'s banding entry in the same commit as
  S-MESH's 1200–1299 (same orchestrator; renumbered twice after
  the GAUTH and SEAT collisions — see `docs/S-MESH-LOG.md`; the
  1100–1199 band was fixed on main by the ordinal-1100 claim at
  BOOL-1's review dispatch). Implementer
  blocks are named `BOOL-B1, BOOL-B2, …` (`BOOL-<n>` are unit names).
- **Container and process facts are S-MESH's** (`docs/S-MESH-LOG.md`,
  Opening state): one remote container, one lane budget shared by the
  two programs, dispatches interleaved. Away-channel tag `(S-BOOL
  orchestrator)`.

**Sweep at opening**: all seven charter issues open with zero
comments; the VERBS fence is confirmed from VERBS' own plan ("S-BOOL's
honest remainder … was never VERBS'"). #1152's reproduction is
committed `#[ignore]`d with un-ignore instructions at the site;
#1011's two red-on-landing pins are torus-shaped and flip with the
torus arm (VERBS-authored file — coordinated flip); #542 sits on
Track R fence ground (seam recorded in both plans); #433's proposal
rides PR #576's body and is retrieved before the conversation opens.
Track Q is current in §D (16 rows, re-derived 2026-08-31; D285/D286
left with CERT-2); the S112 member-(e) pointer to the landed D282 is
deleted in the opening PR. Carve-outs: D283 (Evan's), S83/D36 (wait
on P-2/#1177), H11's third door (N's ground, filed not edited).

## BOOL-1 merged (2026-08-31) — issue 1152 closed; coplanar splits carry adjacent citations

PR 1378 at fix head `3d8b4344` (merged with current main). Gates:
impl head `3f14f3c4` green (interval/1e-6 drawn); fix head green
(interval/default drawn — no trailer, the band-relative regime row
makes any drawn ε the full shape). Root cause was STALE, not absent:
extrude-time operand citations whose partners moved to the `above`
product; the empty smooth arm kept them. The fix keeps only
descriptions drawn in an adjacent chart (seam clause matching the
siblings) and restates the rest on their own carriers.

**The dual (ordinal 1100, sample at the row)**: R1 A-W-F 2/3/3 —
both MAJORs in the PR's CLAIMS, not the code: band-independence
falsified by a dy ladder (the arm fires on within-band flush pairs;
a NEW ε-dependent `ChartResidual` refusal, safe direction — replaces
a body main accepted with red tier 3), and the refusal-vacuity
sentence wrong; plus 6/6 arm mutations surviving the battery. R2
APPROVE 0/1/4. R1's MAJOR-1 is a v6 TALLY CANDIDATE (unilateral,
executed). Fix pass IMPLEMENTER-INHERITED, all 10 union items: the
four-regime band contract measured and pinned per-band at three ε
points; D2 classification written; three mutants killed (the conic
boss-on-plate fixture breaks the axis-aligned rebuild degeneracy),
three site verdicts recorded; the unreachability comment corrected
to the mechanism that actually fires (`Join(DegenerateSection)`).

Issues from the cycle: 1382 (boolean rebuild arm), 1390 (extrude
cap-rim arm on a falsified premise), 1391 (three drifted spellings
of the staleness ladder want one home). Slate next: BOOL-2 (the
point_in_solid cone arm) after the MESH-1 cycle concludes — the two
programs interleave dispatches per the shared lane budget.

## BOOL-2 merged (2026-09-01) — the cone arm lands; issue 1011 stays open for the torus half

PR 1425 at fix head `95ca01b3`. Gates: interval lane trailer-asked
on both gated heads (1e-6 drawn), all six matrix points local at
both. point_in_solid answers cone-bearing solids: quadratic + nappe
+ axial trim, apex OnBoundary with its √(K·ε·v_ext) escalation
shell documented, grazing typed-escalating at three sites. The
delivered head's k-lint red was main-inherited (PR 1351's lofts
renames never re-cut into the tess-budget baseline) — repaired
orchestrator-direct at PR 1428 per the S-CERT PR-1257 precedent,
with the twopeg dead-const clippy red the forced row exposed ported
in the same PR.

**The dual (ordinal 1101, sample at the row)**: twin A-W-F (R1
0/4/2, R2 1/5/3), the load-bearing finding BILATERAL at split
severity — the clamp floor's "five orders" derivation wrong (the
shell scales as √ε; K=1000 turns the row red) — so no tally
candidates. R2's analytic oracle put 2000 points through the arm
with zero mismatches. The planar-cap misread (every full-revolve
cap; the cone base answers In) is homed on issue 1076 with both
lanes' measurements and an #[ignore]d reproduction. Fix pass
IMPLEMENTER-INHERITED, all 11 items; the shell law is now guarded
by a row that re-measures it.

Recorded for BOOL-3's spec: the wrapped_cone_group/
closed_sphere_group scan wants one home before the torus copy
lands; the three per-suite probe-offset spellings likewise. Slate
next: BOOL-3 (the torus arm) or BOOL-5 (#542) per lane budget;
issues 1434, 1401, 1402 filed en route this wave.
