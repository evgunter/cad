# VACUITY — the plan

rows that cannot fail: assertions on their own codomain, probes that print, floors that count what they discard

Opened 2026-09-20 by TINT's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**22 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `random-integer-rays-search-trips-at-eps-1e-6-on-one-run` | H | review_gui1_r1::random_integer_rays_match_the_exact_oracle failed once at eps = 1e-6 on a commit that passed it before and after |
| P3 | `anti-vacuity-floor-cannot-go-red-on-degradation` | E | The GUI0-R1 walk's anti-vacuity floor survives deleting either refusal arm: a guard 16x looser than it reads |
| P3 | `corpus-result-node-loops-skip-silently` | E | m4_pr8_corpus.rs's result-node loop can skip every document with nothing counting it |
| P3 | `e2-recut-probe-has-no-residual-independence-left` | E | review_arceval's E2 now re-executes the shipped row's fixture, constant and predicate, and can no longer disagree with it |
| P3 | `fuzz-rows-discard-trials-against-a-floor-that-counts-them` | E | A fuzz row's continue shrinks its own sample while its coverage floor is written against the attempted trial count: twelve candidate files |
| P3 | `m5-s13-pips-union-escalation-arm-runs-at-no-gated-eps` | E | m5_s13_pips_interval's union escalation arm executes at none of the three gated eps rows |
| P3 | `mate6r1-shared-has-eleven-tests-and-no-assertions` | E | Three mate/fix suites print unasserted probe answers; mate6r1_shared's eleven assert nothing at all and its in_part named the wrong node for its whole life |
| P3 | `phantom-turn-row-stands-down-outside-the-tree-s-door` | E | m10_5_r1 phantom-turn row asserts nothing in three of four arms and announces through a bare println!, outside the stand-down door |
| P3 | `pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` | E | review_gui1_r1's random-ray oracle sweep asserts "some draw hit the cube" over 120 integer rays at effort 1 — a probabilistic guard that reds CI on a bad seed |
| P3 | `rounded-plate-bulges-are-asserted-by-nothing` | E | The conic corpus's rounded plate can lose both its bulges and no row reds |
| P3 | `sign-asserts-on-structurally-zero-measurements` | E | Test-integrity class - sign/positivity asserts riding cancellation noise on structurally-zero measurements (the washer donut instance) |
| P3 | `topo-cylinder-sheet-geomsources-are-asserted-by-nothing` | E | mate5's cross-instance GeomSource fingerprint — node, minted index and pairwise distinctness — is documented at three suites and pinned by no row |
| P3 | `torus-tangency-shell-floor-does-not-scale-with-k` | E | the_clamp_floor_clears_the_torus_tangency_shell asserts a fixed floor against a shell that grows as K^(1/3) — red at CAD_AMBIGUITY_K=30 on main |
| P3 | `unexplained-literal-seeds-do-not-say-which-shape-they-are` | E | Fourteen test-side PRNGs take an unexplained literal seed: none says in-file which shape licenses it |
| P3 | `verbs-f7-r2-probes-brick-rows-print-without-asserting` | E | verbs_f7_r2_probes' two brick-operand rows print their boolean outcome and assert nothing |
| P4 | `cert10-strict-gap-floor-gates-on-a-varying-seed` | E | nurbs_cert_fuzz's cert10 strict-gap floor gates hosted CI over a per-run varying seed and fails at some seeds |
| P4 | `folded-slab-strands-a-hand-derived-volume-and-a-silent-refusal-arm` | E | review_s12_adv's log probe restates the slab's volume by hand and swallows a refusal, so no fixture change can red it |
| P4 | `malformed-ambient-eps-reds-review-m2-pr7-k` | E | local cargo test — review_m2_pr7_k reds under a malformed ambient CAD_TOLERANCE_EPS |

## Order

`random-integer-rays-search-trips-at-eps-1e-6-on-one-run` FIRST, and
not because it is the biggest: it is a test that actually failed, and
it may be reporting a real pick defect rather than its own weakness.
Until that is settled it is not safe to treat the rest of this slate as
test hygiene.

Then the rows that assert nothing at all
(`mate6r1-shared-has-eleven-tests-and-no-assertions`,
`verbs-f7-r2-probes-brick-rows-print-without-asserting`,
`phantom-turn-row-stands-down-outside-the-tree-s-door`) — one unit,
not three. The floors and the seeds behind them.

## Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
