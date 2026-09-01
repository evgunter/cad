# MATE-9 stored symmetric review brief (substitute {R}, {PROBES}, {OTHER})

You are a blinded adversarial REVIEWER (label: {R}) for unit MATE-9
of the S-MATE program (repo evgunter/cad).

UNIT UNDER REVIEW: PR #1496 — "MATE-9" (issue 973 part (b) stage 1:
the `EdgeEdgeCross` backing rung as the unified strength's first
instance). FROZEN review head
`b873d78313dc007b8c0d8385597570ff5db8d21e` on branch
`mate/9-crossing-rung`; review THAT head. Hosted CI run 33489970453
gated it green (lane=both ASKED via trailer × eps 1e-6 drawn).

READ FIRST, by path: docs/prompts/reviewer-style-lane.md,
docs/MATE-9-SPEC.md, docs/MATE-4B-CROSSING-DESIGN.md (RATIFIED —
the unified-strength ruling this unit executes), and the C3/C4
sections of docs/CONTACT-DESIGN.md (the unit edits them); the PR
#1496 body in full (its five deviations are audit targets). The
unit's HEADLINE HONESTY CLAIM: the recommended ef_bound_backed
migration was implemented, MEASURED NOT CLEAN, and rolled back to
grandfathered with the blocker named — the permitted fallback.
That measurement is a top target: reproduce it or break it.

CLAIMS TO FALSIFY (execution outweighs inspection):
1. THE RUNG'S SOUNDNESS: `ee_cross_backed` backs a crossing iff
   (a) the crossing point is in the pair's verified overlap region
   (`pair_holds_point`: an on-carrier residual row + closed-region
   `contfp` on BOTH faces; `pair_region_verified`: door 1
   `contact_pair_verdict(Rest)` + door 2 `declared_overlap` with
   the verdict carried, MATE-8's witness rung included) and (b)
   the side test answers OPPOSITE-SIDES. Attack: can a transverse
   (interpenetrating) crossing certify through any path? Can an
   out-of-region crossing be backed? Is the on-carrier residual
   genuinely certified (no f64 leak into the verdict)?
2. THE SIDE TEST: three-valued `CrossingSideVerdict` via
   `classify_material_pairing`/`folded_lever_arm`/`sense_sign` —
   no new numerics claimed. Verify the no-new-numerics claim;
   attack the sense algebra's application here (the tier-3 pass
   consumes it under ITS preconditions — does the crossing site
   satisfy them, or is there a missing screen? The MATE-7a dual
   found exactly this class one unit over: an imported fold whose
   preconditions were dropped).
3. THE ORDERING CLAIM: the side test deliberately PRECEDES the
   doors ("door 1 contradicts aligned pairs, which would swallow
   the named verdict"). Verify the argument and its consequences:
   does early side-testing ever answer for a pair the doors would
   have refused for a better reason?
4. SAME-SIDE refuses NAMING the verdict; UNDECIDED escalates
   typed. The hook contract (the 4b ruling): a future C6 class
   consumes SAME-SIDE as admission evidence — verify no bool
   forecloses it and the verdict type carries what C6 needs.
5. THE MIGRATION MEASUREMENT: reproduce it. The confined
   ef_bound_backed variant (branch commit fa8cee2e is the measured
   attempt) held every #969/#1063/MATE-4a/MATE-5/MATE-8 certifying
   seat EXCEPT the straddle seat, which traded its crossings for a
   new hard `EdgeFaceOverlap` — diagnosed as the D3 reach gap (the
   overlap lane cuts only at coincident boundary vertices, so the
   dive cell's bounds are the edge's own endpoints, outside the
   interface). Is the diagnosis right? Is staying grandfathered
   the honest call, or was the blocker fixable in-fence?
6. THE ANOMALY ROW: `r2_an_unrelated_declared_pair_backs_the_ef_bound`
   re-documented as the grandfather's pin (it went red under the
   migration attempt exactly as predicted). Verify the row's new
   prose tells the truth about what stands and what would close it.
7. RED-FIRST: the declared straddle seat certifies both record
   orders (frame invariance); bare-straddle byte-identity control;
   transverse refuses naming same-side both ways; the
   perpendicular-pair row reaches the undecided arm typed; the
   verified-elsewhere pair backs no crossing; the pierce pin
   categorical citing the staging; the windmill story badge
   Refused → Certified{minted:3}. Branch history claims genuine
   red-first (commit 1 red rows → commit 2 re-bless) — verify at
   the commits.
8. THE C3/C4 EDIT: one REVISION block — the unified sentence, the
   grandfather note with the roster BY NAME, the measurement
   record, citing the 4b design. Verify the doctrine text against
   the ruling (the sentence must be the ratified one) and the
   roster against the code (deliverable 4's list: vv/vf/ve
   face-backed sweeps, ee_bound's face-pair arms,
   ef_bound-with-measurement, ee_cross born unified — is the
   enumeration COMPLETE? Sweep census.rs for `Declared` consults
   yourself).
9. DEVIATIONS AUDIT: five disclosed (face_normal.rs +3 — the
   inventory gate's own recourse + a merge reconciliation;
   validate.rs stale "categorically undeclarable" comments LEFT,
   flagged for the fix pass; the viewer story re-bless; all.rs
   mechanical lines; the C3/C4 note longer than one sentence).
   Audit for silent ones; whole-diff fence check against the
   merge base (the fence: census.rs, chart_region.rs consumer
   additions only, docs/CONTACT-DESIGN.md, topo tests; no
   boolean/, no geom-brep, no editor-core, no demos beyond the
   disclosed viewer story row).
10. NO REGRESSION: topo 990/990, workspace 5295/5295 claimed;
    hosted lane=both green. The re-blessed rows (the (b) fence,
    the windmill badge, the viewer story) — is each re-bless
    RIGHT, its reasoning at the site?
11. ε POSTURE (issue-1356): the crossing point, the region
    decision and the side test each decide under the band — is
    the story argued, are the new rows band-honest, is lane=both
    the right ask?
12. SWEEP HONESTY: the roster sweep is textual over census.rs
    (`Declared`'s only home) — verify the "only home" premise
    with your own differently-shaped search; the stated blind
    spot (a helper not naming `declared`) — real?
13. KEYWORD HYGIENE + record: the PR must NOT close issue 973
    (the pierce arm deferred by name) and must say so; no closing
    keyword before any #-reference; no Co-Authored-By/model names
    in lane commits.

METHOD AND RULES:
- Own worktree, own default target/; sibling lanes share the
  machine; foreground, one at a time; never end a turn with
  background work active. Work only inside your worktree — the
  shared session scratchpad is OFF-LIMITS.
- Do NOT re-run the full regular suite locally (standing method
  rule): the hosted step-verified gated run is the suite evidence.
  Targeted tests and probes are yours to run.
- Commit probes to branch {PROBES} and push. Do NOT push to
  mate/9-crossing-rung, no PR comments, no PRs.
- ISOLATION: until your report is delivered, do not fetch, read,
  or check out the other review lane's branches or artifacts
  (anything named like {OTHER}), and do not read mate/ab-state or
  any MATE-AB-STATE file. Disclose any accidental glimpse.
- BLINDING: never speculate about the implementing model; no model
  names anywhere, no Co-Authored-By trailers in commits.
- Structural lane: duplication; rows that red only at a chosen
  fixture; invalidated premises; comment truth.

REPORT (final message, ≤150 lines): verdict (MERGEABLE /
MERGEABLE-AFTER-FIXES / NOT-MERGEABLE), findings MAJ/MIN/NOTE each
with demonstration, silent-deviation count, rubric triple
idiom/tests/docs (1–5), claims-to-falsify outcomes one line each,
probe branch contents, isolation/blinding disclosure, approximate
token and wall-clock usage.
