# FLUSH — the plan

Planar and declared-contact correctness at the join: the join's
remaining wrong or refused bodies on planar and declared-flush poses,
and the tier-3′ undeclared-contact failures.

Opened 2026-10-10 by JOIN's close (`docs/doc-ledger/join-leaves-the-tracker.md`;
Ev approved the plan in chat on 2026-10-10). Every row here moved from
`work/join/` by `git mv` with its id and body unchanged.

## The slate

**31 points** of dispatchable work against a ceiling of 32
(`work/flush/program.md` states why it is not 30). INTENT stage 4 E
released three rows on 2026-10-10 (each says so in its own section);
step 2's reader's closes took back 10 and filed one P1 residue, and the
lanes in flight are expected to close more, so the track is not split.

| pri | item | cost | title |
|---|---|---|---|
| P2 | `a-thin-walled-hollow-ships-while-a-slab-as-thin-refuses` | M | a thin-walled hollow ships while a slab as thin refuses: retire PR 4636's sole-shell gate; thinness is judged by pairs alone (Ev, PR 4690) |
| P2 | `a-curved-notch-at-a-corner-pinch-refuses-join-desync-on-a-section-vertexs-null-edge-copies` | M | a curved notch at a corner pinch refuses `JoinDesync` on a section vertex's null-edge copies (15 lines, 5 poses) |
| P2 | `a-cube-less-a-pinched-operand-piercing-its-face-refuses-the-split-cones-re-pair` | M | a cube less a pinched operand piercing its face refuses `ZipCorrespondence` at `split_cones`' re-pair (36 `dbl` lines, typed since PR 4728) |
| P2 | `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys` | M | a pinch whose keys no seam ties keeps its cones on separate point keys |
| P3 | `two-pinch-poses-escalate-at-eps-1e-6` | M | pose 1 at ε 1e-6: can a cross-germ tie between pairs that share no germ change the greedy matching (a proof task; pose 2 is honest) |

Parked, and not counted:

| pri | item | cost | waits on |
|---|---|---|---|
| P0 | `a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded` | M | `boolean-door-runs-the-census-over-its-result` (REACHHOLD) |

## Order

1. **The P0 wrong bodies** (done). The corner ruling, its corner-pair
   sibling and the L-prism closed with PR 4650: a record follows its
   divided face to the piece that holds it. The sliver-only result
   closed with PR 4636; Ev ruled on its gate in PR 4690 (thinness is
   judged by pairs alone). The P0 residue,
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
   no pierce), `a-cube-less-a-pinched-operand-…` (PR 4728's typed residue) and `a-curved-notch-at-a-corner-pinch-refuses-join-desync-…`
   (the residue of the curved shared-pinch re-read, PR 4698).
5. **The E tail** (done, PR 4714: five rows closed; `two-pinch-poses-…`
   stays open as a proof task about pose 1's matcher tie).

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
