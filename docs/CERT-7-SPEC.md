# CERT-7 — the offset_fit family: issues 1005, 1008, 1007

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issues 1005, 1008 and 1007 are the primary specifications; this
document fixes scope and sequencing. One unit, three commits' worth
of coherent scope, all in `geom-brep/src/offset_fit.rs`.

## The three pieces, in order

1. **Issue 1005 — the weighted composite**: the rational/weighted
   composite fit, with the two reviewer rows that were flipped away
   from containment flipped BACK to containment (the issue records
   which rows and why they were weakened; the fix makes them honest
   containment assertions again — red-first at the current
   non-containment if the defect is live at your merge base).
2. **Issue 1008 — per-cell recentring**: recentre the fit per cell;
   re-measure the small-|d| row the issue names (its number is a
   measurement, not a target — re-derive with the argument in the PR
   body if it moves).
3. **Issue 1007 — directional refinement with the stall guard**: the
   refinement loop refines in the direction the residual says, with
   a stall guard so a non-converging direction terminates in a typed
   refusal rather than a budget spin. The guard's admission set is
   D2-addendum ground: classify what it refuses.

## Explicitly NOT here

- **Issue 1006 is CERT-10's** (the patch-hull consolidation, under
  the Q2 ruling, sequenced after CERT-5 and CERT-7). Do not start
  the shared-home move, the whole-face-arm collapse, or the
  magnitude-reading retirement — even where this file tempts it.
  If your diff wants a helper 1006 would also want, home it locally
  and note it for CERT-10.

## Fences

- This unit's file is `offset_fit.rs`; the plan sequences CERT-10
  after this unit precisely because both edit it — keep the diff
  scoped so CERT-10's seam stays clean.
- Sibling lanes may be in flight on disjoint files; the build-slot
  mutex binds. Merge origin/main before opening the PR and again if
  main moves.
- Track M trait ground per H-R3/#867: consume `CertifiedBounds` as
  it stands.

## Order of work

1. Red-first rows per piece (the containment flips for 1005; the
   small-|d| measurement for 1008; a stall fixture for 1007 that
   currently spins or exhausts wrongly).
2. The three fixes as three coherent commits (reviewable seams; one
   PR).
3. Blast radius: touched-crate suites at default + interval; margin
   populations re-derived not re-baselined; k-lint per runbook.

## Acceptance

- All three issues' asks discharged with red-first digits in the PR
  body; the 1005 rows assert containment again.
- The stall guard's refusal typed and classified per the D2
  addendum.
- Sweep obligation: the containment-weakening shape (an assertion
  flipped from containment to something monotone-easier) swept
  across this file's test surface — hit list with dispositions;
  state what the pattern cannot match.
- Hosted CI green on the head; drawn point stated;
  `CI-Config: lane=both` if the interval claims need it.
- ε-three-outcome honesty; deviations stated; no `Co-Authored-By`;
  "issue 1005/1007/1008" spelled out (orchestrator closes after
  merge).
