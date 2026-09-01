# MESH-5 — issue 685: the `nu == 1` sizing-intent decision

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S/M, recorded numeric M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 685 is the primary specification; the slate entry
(`docs/S-MESH-PLAN.md`, MESH-5) is the ruling on shape.

## Situation

When the v-schedule computes `nu == 1`, the sizing intent is
ambiguous and the issue holds BOTH readings defensible: either the
v-schedule is honoured (rows emitted) or one triangle strip is
right for a ruled patch and the code should say so and stop
computing a schedule it discards. The unit DECIDES BY MEASUREMENT —
the π/6 cone wedge δ-sweep with the budget instrument in hand — and
writes the decision at the site. Explicitly NOT conflated with
issue 678's `nu == 2` pole floor.

## Deliverables

1. **The measurement first**: the π/6 cone wedge δ-sweep through
   the budget instrument, recording where the two readings diverge
   (element counts, budget rows, watertightness, and any quality
   metric the instrument already reports) across the δ range the
   suites exercise. The measurement decides; the PR shows it.
2. **The decision written at the site**: whichever way it lands,
   the code either honours the schedule at `nu == 1` or states at
   the site why one triangle is right for a ruled patch AND stops
   computing the schedule it discards (no dead computation kept
   "for symmetry"). The S29 smell instance retires with the
   decision.
3. **D9 discipline**: if the decision changes any emitted mesh,
   that is a change to `f(body, δ)` — enumerate which in-tree
   bodies/δ change (a two-build byte instrument over the tour
   bodies is the established shape; both MESH-3 review instruments
   are in-tree to reuse), state the direction, and pin the new
   shape red-first. If it changes nothing in-tree, pin THAT.
4. **Rows**: red-first on the decided behavior at the `nu == 1`
   boundary; a row proving `nu == 2`+ is untouched (the 678 fence);
   the wedge fixture pinned at the δ values the measurement used.
5. **ε/δ posture** (issue-1356): the schedule read is δ-driven —
   state which reads are band-sensitive and pin per-band where the
   suites' ε rows interact with the sizing decision at all (expected:
   none — say so if so).
6. **Class sweep** (discipline §5): every `nu == 1` / `nv == 1` /
   one-element special case in `crates/mesh`'s sizing and patch
   emission paths, dispositioned in the PR (the 678 pole floor is
   NOT this unit's — record and leave).

## Stop-and-report

If the measurement shows the answer belongs to the sizing-POLICY
conversation (a global budget/quality trade the site cannot decide
locally), STOP and report with the measurement rather than
patching. That is an acceptable unit outcome per the slate.

## Acceptance

- The measurement in the PR; the decision at the site; S29 instance
  retired; red-first rows; D9 disposition stated with evidence.
- Hosted CI green on the final head; gate record per head in the PR.

## Hard rules

- NO `Co-Authored-By` trailer, no model names in commits. "issue
  685" spelled out, no closing keywords (the orchestrator closes at
  merge if the decision closes it; the stop-and-report branch does
  not close it).
- Scope fence: `crates/mesh` sizing/schedule and the patch-emission
  path that consumes it, mesh suites, the wedge fixture. NOT:
  `walk.rs` classification (MESH-3 just landed there — rebase on
  current main), issue 678's pole floor, `docs/MODEL-AB-LOG.md` /
  `docs/S-MESH-*.md` / SMELL edits.
- Re-merge main before opening the PR.
