# FLUSH — the plan

Planar and declared-contact correctness at the join: the join's
remaining wrong or refused bodies on planar and declared-flush poses,
and the tier-3′ undeclared-contact failures.

Opened 2026-10-10 by JOIN's close (`docs/doc-ledger/join-leaves-the-tracker.md`;
Ev approved the plan in chat on 2026-10-10). Every row here moved from
`work/join/` by `git mv` with its id and body unchanged.

## The slate

**45 points** of dispatchable work against a ceiling of 32 (2026-10-31, after PR 5781 closed the aims' ties and filed its residue), so the slate is over budget and splits along its seam: the five P2 rows PR 5781 left on the side reader's ties go to a track of their own, in the next tracker PR. FLUSH measured 41.5 when PR 5368 closed its P1, so it was cut along its priority seam: the eight P3 rows and the one P4 row moved to **FLUSHTAIL** (`work/flushtail/`) by `git mv`, with ids unchanged. Three older rows are unpriced. `work/flush/program.md` states why the ceiling is not 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side` | M | an aimed ray started far past a seam split's ellipse loses its wall roots, and a correct split refuses inside out (live on main, PR 5781) |
| P2 | `a-wall-root-bounded-by-an-oblique-section-still-walks-and-ties-at-the-nominal` | H, design | `cut_cylinder`'s 22 exact ties (PR 5781's residue) |
| P2 | `cone-sphere-patch-and-torus-patch-roots-still-walk-their-regions` | H | PR 5781's residue |
| P2 | `a-torus-face-aims-at-a-fixed-chart-point-that-a-symmetric-pose-holds` | M | PR 5781's residue |
| P2 | `the-census-ladder-asks-at-symmetric-points-unmeasured` | M | PR 5781's residue |
| P2 | `the-ruled-aims-on-axis-fallback-hulls-at-interval` | M | PR 5781's residue |
| P3 | `a-fallback-cut-refuses-beside-a-ring-a-seam-or-a-pinch` | H | PR 5706's residue |

In flight: `main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond` (P1 M).

Parked, and not counted:

| pri | item | cost | waits on |
|---|---|---|---|
| P0 | `a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded` | M | `boolean-door-runs-the-census-over-its-result` (REACHHOLD) |
| P0 | `a-result-that-is-only-a-sliver-shell-passes-the-door` | M | `boolean-door-runs-the-census-over-its-result` (REACHHOLD): under pairs only (PR 4690), its residue is the door census; 11 needles at ε 1e-9 and 34 `rc_wide` lines at 1e-6 ship (PR 4783) |

## Order

1. **The P0 wrong bodies** (done). The corner ruling, its corner-pair
   sibling and the L-prism closed with PR 4650: a record follows its
   divided face to the piece that holds it. The sliver-only result
   closed with PR 4636; Ev ruled on its gate in PR 4690 (thinness is
   judged by pairs alone), and PR 4783 retired the gate. Its residue is
   parked on REACHHOLD's door census. The P0 residue,
   `an-apex-edge-in-the-partners-face-plane-…`, closed with PR 4737:
   a line edge in a plane face records its span.
2. **Reader's closes** (done, PR 4635: all three closed with pins
   through the plain booleans; the residue,
   `a-wedge-flush-on-a-block-top-…`, closed with PR 4709 as D10's
   correct refusal, pinned). Two rows said on their release from the D10 hold
   (2026-10-08) that they may already be done:
   `reflex-corner-edge-in-face-poses-…` ("already fixed by PR 3900 …
   it can close now") and `a-declared-flush-wedge-sunk-in-a-block-…`
   ("what is left is a reader's close"). Re-measure each on main and
   close it with the evidence, or price what is left.
3. **The undeclared contacts** (done). The corner pair and the L-prism
   closed with PR 4650. The anti-parallel touch closed as fixed by
   PR 4496 (PR 4649).
4. **The declared-flush and pinch rows**:
   `declared-flush-intersect-refuses-in-one-operand-order` (folded into
   step 2's measuring lane), then the pinch rows:
   `the-pre-zip-pinch-weld-retires-…` and `a-hole-weld-cannot-tell-…` (done,
   PR 4728); `a-pinch-the-seams-do-not-link-…` (vertex-vertex pinches with
   no pierce; ruling A, fork-log row 133: two parts' value-decided
   coincident vertices share one point key; done, PR 4823),
   `a-cube-less-a-pinched-operand-…` and `a-curved-notch-at-a-corner-pinch-refuses-join-desync-…`
   (done, PR 4777).
5. **The E tail** (done, PR 4714: five rows closed; `two-pinch-poses-…`
   closed with PR 4769: among straight, mutual germs with K > 2 the
   matcher's pair order cannot move the matching).

## The intent-refactor hold (Ev, `[ev]` PR 3990, 2026-10-03)

Rows that meaningfully use declared contact, declared flush pairs,
continuations or the REST zip wait for INTENT stage 4
(`work/intent/plan.md`). INTENT stage 4's PR A (PR 4364,
`the-join-builds-what-the-rest-zip-builds`) merged 2026-10-09: the join
now builds what the REST zip built, and the REST zip is gone. The rest
of stage 4 is unbuilt, so the rows gated on it stay parked on
`intent-stage4-is-built`: the corner pair with an edge in the partner's
face plane, the pre-zip pinch weld, and the pinch whose seams do not
link.

The declared-flush rows on the open slate were released from the hold
by INTENT's re-homing on 2026-10-08, each with its reason in its own
`## Released from the D10 hold` section: their open step is the join's
own topology, which D10 keeps. A lane that finds its row does need
declared pairs or contact after all parks it on its INTENT stage row.

**What is in flight on the same ground (2026-10-11, after F).** INTENT
stage 4 F (PR 4641) merged. The declared surface is gone:
`declare`, `DeclaredPair`, `BooleanCoincidence`, `BooleanDeclarations`,
the flush detector and the `*_with` booleans. A boolean's one other
input is `BooleanInputs { given, records_a, records_b }`, so FLUSH lanes
use the plain booleans. CONTACTHOLD's PRs 4598 and 4604 (merged) made
`ContactRecords` one list of unordered cell pairs and rewrote how the
census reads a touch. FLUSH's PR 4650 changes how a record follows a
divided face (`Descendants::holder`, which includes the pinch welds'
fragments), so the pinch cluster is measured after it merges. Stage 4
C/D (`intent/s4-cd-symbolic-door`) moves the door's coincidence pass.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md` and `docs/DUAL-REVIEW-PROTOCOL.md`
(an H unit draws a concurrent pair, an M unit a urandom draw); the log
names each tier and its reason. The A/B experiment is suspended, so the
track carries no band.

A lane measures its row's repro on current main before it builds.

## Exit

The slate is closed or re-homed. This plan sets no `## Exit criteria`,
so the program closes without an exit walk.
