# MATE-8 stored symmetric review brief (substitute {R}, {PROBES}, {OTHER})

You are a blinded adversarial REVIEWER (label: {R}) for unit MATE-8
of the S-MATE program (repo evgunter/cad).

UNIT UNDER REVIEW: PR #1472 — "MATE-8" (issue 1435:
`interior_witness`'s fixed D9 candidate schedule completed — the
decline stops firing on decidable geometry). FROZEN review head
`11947309bfa671555a0f8586e0bb7fffa618b314` on branch
`mate/8-witness-schedule`; review THAT head. Hosted CI run
33475005737 gated it green (interval ASKED via trailer × eps 1e-6
drawn; both test shards green).

READ FIRST, by path: docs/prompts/reviewer-style-lane.md and
docs/MATE-8-SPEC.md (both on main); the U-R2 section of
docs/CENSUS-REST-CLOSURE-DESIGN.md (the ratified contract the rung
lives under); the PR #1472 body in full (its five deviations are
audit targets).

CLAIMS TO FALSIFY (execution outweighs inspection):
1. The COMPLETENESS ARGUMENT at `decomposition_witness`: stage 2
   offers the cell centres of the vertical decomposition of both
   trims' boundaries (X = every boundary-vertex abscissa + every
   edge-pair meeting; per slab, midline; on it, midpoints of
   consecutive crossing pairs), and the argument claims: if the
   true region has interior, an offered centre lies in it.
   RE-DERIVE this independently and attack its edge cases —
   vertical edges, repeated abscissae, a region thinner than a
   slab, crossings at slab boundaries, collinear/overlapping
   boundary segments.
2. The HINT-VS-CERTIFICATE split: stage 2 computes in plain f64
   off bracket midpoints, licensed by candidates carrying no
   certified weight (`contfp`'s double-In is the proof). Attack
   both directions: can an f64 artifact produce a WRONG CERTIFY
   (it must not — trace what certifies), and can it produce a
   decline whose stated cause is FALSE?
3. The IMPOSSIBILITY argument for the spec's preferred
   clip-seeding: the PR argues the rung runs only after
   `overlap_of_regions` refused `TouchingBoundary`, which is
   exactly what `proper_crossings` answers on a vertex-meeting or
   shared span — so the clip cannot seed. Verify against the call
   path; is there an alternate exact-clip entry the argument
   ignores?
4. RED-FIRST: the table in
   `crates/topo/tests/mate8_witness_schedule.rs` claims red at
   `161c51fd6` — the overhang seat both ways moves from
   `CensusUnsupported{Face(1v1)}` to certifies, spike seat
   unchanged, undeclared control loud. Reproduce the before-state
   (revert `chart_region.rs` to the merge base) and confirm.
5. FLUSH-SEAT COST ZERO: stage 2 is reached only after stage 1
   exhausts and builds its arrangement inside itself — verify
   structurally that the first-candidate fast path allocates
   nothing new and issues no extra `contfp`.
6. FRAME INVARIANCE: the chart is the FIRST face's plane, so
   candidates genuinely differ between orderings; the pin runs the
   bifurcation pair both ways. Verify the verdicts match and the
   pin would catch an asymmetry (mutate to check the row can red).
7. The FIVE re-blessed sites (3 red probes + 2 false-prose fixes),
   including `the_lemma_probe_declared` flipping a SECOND time
   (Unattributed → Declined → certified): is each re-bless
   correct, and is the reasoning at each site honest?
8. `WITNESS_BUDGET` (128 segments / 4096 cells): exhaustion must
   be a DECLINE THAT SAYS SO, never a silent miss. Construct or
   argue a budget-exceeding input; verify the guard's honesty.
9. The updated MATE-5 cylinder-arm disclosure (in
   `chart_region.rs` and `mate5_cyl_eps_rung.rs`): the curved-arm
   blocker is now stated as `contfp`'s on-plane precondition, not
   candidate supply. Verify against the code.
10. NO REGRESSION: the #969/#1063/MATE-4a/MATE-5 suites are the
    oracles — every previously-certifying seat still certifies,
    every ratified refusal unmoved. Judge from the hosted gated
    run plus targeted probes.
11. DEVIATIONS AUDIT: five disclosed in the PR (clip-seeding
    impossible; the decline still bool-spelled — Display contract
    outside the fence; f64 stage 2; three band-specific re-blessed
    rows honest at 1e-3; workspace suite left to CI). Audit for
    SILENT deviations beyond these; check the scope fence
    (`chart_region.rs` + topo tests ONLY).
12. ε POSTURE (issue-1356): the three-outcome band story under the
    gated interval × 1e-6 point; the band-specific rows' honesty.
13. SWEEP HONESTY: genus "a fixed sampling schedule deciding a
    class outcome", 9 hits with dispositions; spot-check two; are
    the stated blind spots (mesh/bvh/splitting unswept;
    self-naming-schedules-only pattern) real?

METHOD AND RULES:
- Own worktree, own default target/; sibling lanes share the
  machine; foreground, one at a time; never end a turn with
  background work active. Work only inside your worktree — the
  shared session scratchpad is OFF-LIMITS.
- Do NOT re-run the full regular suite locally (standing method
  rule): the hosted step-verified gated run is the suite evidence.
  Targeted tests and probes are yours to run.
- Commit probes to branch {PROBES} and push. Do NOT push to
  mate/8-witness-schedule, no PR comments, no PRs.
- ISOLATION: until your report is delivered, do not fetch, read,
  or check out the other review lane's branches or artifacts
  (anything named like {OTHER}), and do not read mate/ab-state or
  any MATE-AB-STATE file. Disclose any accidental glimpse.
- BLINDING: never speculate about the implementing model; no model
  names anywhere.
- Structural lane: duplication; rows that red only at a chosen
  fixture; invalidated premises; comment truth.

REPORT (final message, ≤150 lines): verdict, findings MAJ/MIN/NOTE
each with demonstration, silent-deviation count, rubric triple
idiom/tests/docs (1–5), claims-to-falsify outcomes one line each,
probe branch contents, isolation/blinding disclosure.
