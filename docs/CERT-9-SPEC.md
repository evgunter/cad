# CERT-9 — issue 303: signed_volume recentring

**Binding at dispatch** (S-CERT program, `docs/S-CERT-PLAN.md`;
difficulty logged at spec: **S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 303 is the primary specification; this document fixes scope.
Mesh-fence ground with no live claimant — taken because the merged
cut assigns it; keep the diff to exactly this defect.

## The defect and the fix shape (the issue's own)

`signed_volume` folds tetrahedra anchored at the world origin, so a
body far from the origin pays catastrophic cancellation proportional
to its distance. Recentre on an interior point — the mesh's bbox
centre — so the fold's operands scale with the BODY, not its
placement. The recentring subtraction must not silently change what
is being measured: over the reals the signed volume is
translation-invariant, so the fix is exact in ℝ and the argument is
one sentence — write it at the site.

## Order of work

1. **Red-first**: the huge-offset probe (the issue's own case)
   pinned as a row that FAILS on the current spelling — measure the
   error at a large placement and pin the recentred truth; a
   translation-invariance row (volume at origin vs at 1e3/1e6
   placements agreeing to a stated, argued tolerance).
2. The recentring.
3. Blast radius: mesh suite + consumers at default and
   `--features interval` if the function is generic (check —
   if f64-only, say so and skip the lane); any moved
   baseline re-derived with the argument, never re-baselined
   silently; k-lint per runbook if it fires.

## Fences / posture

- Mesh fence, no live claimant; do NOT widen into mesh::walk's
  `closing_column` (S-MESH's future owner inherits that note) or
  any other mesh ground.
- ε posture per the issue-1356 practice: if your rows' premises are
  band-sensitive, pin the ε row by trailer and state per-band
  premises; a pure f64 invariance row that consults no tolerance
  says so.
- `CI-Config: lane=both` only if your claims need the interval
  lane; a purely f64 mesh fold may not — decide and say why.
- No `Co-Authored-By`; "issue 303" spelled out; push early to
  `cert/9-signed-volume`; the gate runs when the orchestrator opens
  the PR — report local evidence as local.

## Acceptance

- Red-first digits in the PR body; the invariance row a permanent
  gate; the one-sentence exactness argument at the site.
- Sweep obligation (assume it is a class): other origin-anchored
  folds in `mesh/` (the same shape — a quantity translation-
  invariant over ℝ paid as cancellation by a world-origin anchor);
  hit list with dispositions; state what the pattern cannot match.
- Deviations stated; any refusal minted/changed classified per the
  D2 addendum (none expected).
