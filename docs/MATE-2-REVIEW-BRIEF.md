# MATE-2 stored symmetric review brief (substitute {R}, {PROBES}, {OTHER})

You are a blinded adversarial REVIEWER (label: {R}) for unit MATE-2
of the S-MATE program in this CAD kernel repo (evgunter/cad).

UNIT UNDER REVIEW: PR #1417 — "MATE-2" (issue 1032, declared
cylindrical Rest without a planar Rest beside it). The FROZEN review
head is commit `c27ecb5afac7af66e27a2ced99eaee13fac835c0` on branch
`mate/2-cyl-rest`. Fetch and check it out; review THAT head; if the
branch moved past it, ignore newer commits and say so. Hosted CI run
33439015958 gated this head at the ASKED-FOR point {interval,
eps=1e-12} (trailer), both test shards green; the run's one red job,
k-lint (gate), is repo-wide and inherited from main (issue 1418) —
NOT this unit's; do not investigate it beyond confirming the
inheritance claim.

READ FIRST, by path: docs/prompts/reviewer-style-lane.md (binding
structural lane) and docs/MATE-2-SPEC.md (the binding spec; both on
main). Issue 1032 is the primary specification — its measurement
stands, its mechanism paragraph was a hypothesis the PR claims to
have FALSIFIED. Read the PR #1417 body in full.

CLAIMS TO FALSIFY (execution outweighs inspection):
1. The measured mechanism: on main, the spelling (3) fixture enters
   the declared-cover rung (covered == true — the issue's coverage
   hypothesis false) and refuses because `vertex_on_curved_face`
   collapses a certified Out-of-THIS-face verdict into the same
   Ok(false) as "no verdict". Re-trace it yourself on main at the
   fixture; confirm the quoted refusal and the covered flag.
2. Soundness of the fix's `Placement::Elsewhere` arm: an endpoint
   certified Out of this face is treated as eventless AT THAT FACE
   on the argument that the event surfaces via the neighbouring
   face (the planar coplanar arm's own posture). ATTACK IT: build a
   configuration where a real event is now dropped everywhere — a
   seam vertex whose neighbouring face does NOT surface it, a
   degenerate/short arc, a declared pair whose neighbour is not in
   the declaration. This is the unit's one silent-wrongness shape.
3. `Undecided` and nothing-recorded still keep the frontier; the
   UNDECLARED arms are byte-identical in behavior — execute
   undeclared fixtures on both trees.
4. The four spellings: red-first quotes reproduce from main; the
   control (fixture (i)'s shape) pinned; the PR's deviations about
   spellings (2) and (4) (their main-state differs from the
   issue's claims) re-measured by you.
5. The lily wall-probe-12 pin is bit-identical and UNTOUCHED, and
   the narrower-class claim holds: the corm's full-period revolve
   face answers None (not Out) at `curved_face_containment`, so
   azimuth-split carriers are fixed and full-period ones are not
   (issue 1416's framing accurate).
6. The M9-3 R1 probe allow-list widening (+1 typed variant; the PR
   says the fix moves that probe's frontier downstream to the F7
   merge door, issue 1415): is the widening sound and honestly
   framed, or does it bless a wrong outcome?
7. The 8-ULP volume-additivity assertion (was bitwise in the
   issue's control): justified, or a loosening that hides drift?
8. Sweep genera A and B: spot-verify at least two not-this-unit
   dispositions in each; are the stated blind spots real?
9. ε posture: the unit asked eps=1e-12 (lane drawn). Is the
   band-sensitivity argument sound, and do the new rows hold at
   default ε (run them)?

METHOD AND RULES:
- Own worktree, own default target/; sibling lanes share the
  machine — slow builds are expected; foreground, one at a time;
  never end a turn with background work active.
- Commit probes to branch {PROBES} and push. Do NOT push to
  mate/2-cyl-rest, do NOT comment on the PR, do NOT open PRs.
- ISOLATION: until your report is delivered, do not fetch, read,
  or check out the other review lane's branches or artifacts
  (anything named like {OTHER}), and do not read mate/ab-state or
  any MATE-AB-STATE file. Disclose any accidental glimpse.
- BLINDING: never speculate about which model implemented the
  unit; no model names anywhere.
- Structural lane (reviewer-style-lane.md): duplication; rows that
  red only at a chosen fixture; invalidated premises; comment
  truth.

REPORT (final message, ≤150 lines): verdict (MERGEABLE /
MERGEABLE-AFTER-FIXES / NOT-MERGEABLE), findings MAJ/MIN/NOTE each
with its demonstration, silent-deviation count, rubric triple
idiom/tests/docs (1–5), claims-to-falsify outcomes one line each,
probe branch contents, isolation/blinding disclosure.
