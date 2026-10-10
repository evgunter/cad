# FLUSH — the plan

Planar and declared-contact correctness at the join: the join's
remaining wrong or refused bodies on planar and declared-flush poses,
and the tier-3′ undeclared-contact failures.

Opened 2026-10-10 by JOIN's close (`docs/doc-ledger/join-leaves-the-tracker.md`;
Ev approved the plan in chat on 2026-10-10). Every row here moved from
`work/join/` by `git mv` with its id and body unchanged. Nothing is
dispatched.

## The slate

**32 budget points** of dispatchable work against a ceiling of 32
(`work/flush/program.md` states why it is not 30).

| pri | item | cost | title |
|---|---|---|---|
| P0 | `a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime` | M | box ∩ cylinder through its corner edge: the right volume, and tier 3′ fails |
| P0 | `a-result-that-is-only-a-sliver-shell-passes-the-door` | M | a result whose only lump is a sliver ships: check 7 exempts its in-band sign, check 10 reads no one-shell solid |
| P1 | `an-l-prism-top-edge-exactly-in-the-cube-face-plane-ships-an-undeclared-contact` | M | an L-prism's top edge in the cube's face plane ships an undeclared contact |
| P1 | `a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync` | M | a declared-flush wedge sunk in a block: its intersect refuses `JoinDesync` |
| P1 | `reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop` | H | the 315° reflex corner's edge-in-face poses zip a ring parallel to its section loop |
| P2 | `declared-flush-intersect-refuses-in-one-operand-order` | M | a declared-flush intersect refuses `JoinDesync` in one operand order |
| P2 | `a-curved-face-at-a-shared-pinch-vertex-builds-bodies-the-census-and-the-reuse-check-refuse` | M | a curved face at a shared pinch vertex: bodies tier 3′ escalates and a further union refuses |
| P2 | `a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face` | M | `Joint::Hole` cannot tell a figure-8 hole from an island face |
| P2 | `undeclared-anti-parallel-touch-builds-a-scaffold-at-rest-in-one-op` | M | an undeclared anti-parallel touch at a wedge builds a scaffold at rest, one op and order per pose |
| P3 | `the-reflex-probes-run-b-minus-a-under-declarations-keyed-for-a-b` | E | the reflex probes run `b ∖ a` under declarations keyed for `(a, b)` |
| P3 | `the-turn-sense-divisor-is-nonzero-by-prose` | E | `Turn.sense`'s nonzero divisor is held by call order, not by type |
| P3 | `two-pinch-poses-escalate-at-eps-1e-6` | E | two `pinch_faces_tessellate` poses escalate at ε 1e-6 |
| P4 | `a-curved-boolean-refusal-cites-the-germs-recorded-face-after-a-mef` | E | `CurvedBooleanUnsupported` cites the germ's recorded face, which a `mef` may have divided |
| P4 | `levered-sign-as-bool-has-no-shared-home` | E | a levered sign read as a bool has no shared home in `boolean/` |
| P4 | `sibling-batteries-judge-sound-weaker-than-differential-outcome` | E | four batteries judge SOUND weaker than `differential::outcome` |
| P4 | `strut-side-labels-are-provisional-in-join-and-derived-in-vtxfac` | E | `join.rs` calls strut labels provisional; `vtxfac` mints them as derived |

Parked, and not counted:

| pri | item | cost | waits on |
|---|---|---|---|
| P0 | `a-corner-pair-with-an-edge-in-the-partners-face-plane-builds-with-undeclared-contacts` | M | `intent-stage4-is-built` |
| P0 | `a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded` | M | `boolean-door-runs-the-census-over-its-result` (REACHHOLD) |
| P1 | `the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone` | H | `intent-stage4-is-built` |
| P2 | `a-pinch-the-seams-do-not-link-keeps-its-cones-on-separate-keys` | M | `intent-stage4-is-built` |

Three rows were unpriced on JOIN and are priced here:
`undeclared-anti-parallel-touch-…` (P2, M: not root-caused),
`a-curved-boolean-refusal-cites-…` (P4, E: cite the face that holds the
segment) and `levered-sign-as-bool-…` (P4, E). Two P0/P1 rows were open
on JOIN with a `blocked_on` naming the unbuilt INTENT stage 4, and are
parked here.

## Order

1. **The P0 wrong bodies.** `a-box-and-a-cylinder-through-its-corner-edge-…`:
   find which contact or edge the census reads as non-manifold at the
   corner ruling, and decide whether the body or the census is wrong.
   `a-result-that-is-only-a-sliver-shell-passes-the-door`: the door reads
   check 7's in-band arm through `RoleUnread::certified_by` and types it
   `Escalated { ShellRole }`. TALLY holds the sliver row's three
   siblings on the door's in-band typing
   (`the-door-gates-other-in-band-findings-are-typed-the-kernels`,
   `a-threshold-straddling-in-band-shell-is-typed-the-kernels`,
   `the-shell-role-refusal-locates-no-piece`), which touch the same
   `finding_arm`; a lane here announces the seam on TALLY's log.
2. **Reader's closes.** Two rows say on their release from the D10 hold
   (2026-10-08) that they may already be done:
   `reflex-corner-edge-in-face-poses-…` ("already fixed by PR 3900 …
   it can close now") and `a-declared-flush-wedge-sunk-in-a-block-…`
   ("what is left is a reader's close"). Re-measure each on main and
   close it with the evidence, or price what is left.
3. **The undeclared contacts.** `an-l-prism-top-edge-…` and
   `undeclared-anti-parallel-touch-…`: an undeclared coincidence the op
   neither refuses nor records. Each lane reads the row against the
   hold below first.
4. **The declared-flush and pinch rows**:
   `declared-flush-intersect-refuses-in-one-operand-order`, then
   `a-hole-weld-cannot-tell-…` and `a-curved-face-at-a-shared-pinch-vertex-…`.
5. **The E tail**, batched by file into one or two lanes:
   `the-turn-sense-divisor-…`, `strut-side-labels-…`,
   `a-curved-boolean-refusal-cites-…` and `levered-sign-as-bool-…` in
   `boolean/`; `two-pinch-poses-…`, `the-reflex-probes-run-b-minus-a-…`
   and `sibling-batteries-…` in the sweep tests.

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
