# M5 orchestrator log

Running record of the M5 milestone (curved geometry: NURBS depth,
SSI, fillets). Plan: `docs/M5-PLAN.md`; design contract:
`docs/CURVED-DESIGN.md` (C1–C12, ratified #85). Convention as in
M0–M4 logs: newest entries append at the bottom; the tail snapshot
is the resumption contract for any successor.

## Session start (2026-07-27 — cad-implement-m5 orchestrator)

Handoff from cad-implement-m3-6plus received at the drained M4→M5
seam (M4 CLOSED: exit walk 12/12, #119/#121 merged, final snapshot
at M4-LOG tail). Worktree fast-forwarded to main `2f2b3b3`;
monitors installed from `scripts/monitors/` and armed (away-channel,
disk watchdog, hourly check-in); sign-off watchlist empty;
predecessor stopped via `mngr stop` after orientation confirmed.

**M5-PLAN drafted and opened without overnight block** per Evan's
explicit authorization at handoff (2026-07-27: "it seems safe for
it to just write that and then go … i don't want it to wait
overnight just for my ok"; patchable-not-fatal stance invoked by
him). The plan is sequencing-only — CURVED-DESIGN C1–C12 is the
design layer and every OQ1–OQ9 carries a #85 DECIDED entry; the
plan resolves only the planning residue R1–R6 (cone-arm staging,
ring-vs-adoption split, schema v2 mechanics, banked-opener
scope-boxing, the acceptance-shape set, scope guards). PR 0
obligations folded into the plan PR: the D2 sharpening
(`TangencyLocus` → `TangentIntersection` rename sweep in
DESIGN.md), the inari quarantine transition-state text +
LGPL-before-publish exit condition, this log seeded.

Immediate dispatch order (fork-independent first, per handoff):
PR 1 (interval-transcendentals adoption, green-lit), PR 2 (C9
ring), PR 3 (linalg + NURBS substrate part 1), S3 (curvo audit —
must report before PR 4). A/B block 4 opens with the first
implementation dispatch (pre-flip difficulty logged per protocol
v2; MODEL-AB-LOG is the log).

**Openers dispatched (2026-07-27, same session)**: #123 opened
with the sign-off comment watchlisted. Two seam surveys ran
(interval seam: swap surface is exactly geom-core/src/interval.rs
+ two Cargo.tomls, zero inari use elsewhere; curve/surface
substrate: no Curve2 exists, Surface::Nurbs is a unit placeholder,
Real trait confirmed ring-complete for de Boor, no linear-solve
code anywhere). **S3 curvo audit COMPLETE** (same day —
docs/CURVO-AUDIT.md @ 1fbf0ad): Q5 default stance CONFIRMED,
vendoring REJECTED on all four candidate areas (hardcoded
epsilons/fail-quiet branches at every invariant-relevant surface;
no A9.10 stack and NO SSI in curvo at all — the DESIGN.md
landscape row's "incl. SSI" was wrong, corrected); oracle scope =
evaluation/basis/derivatives, pinned at 47d19d5. Q5 lean revision
applied to DESIGN.md per the OQ9-decided path. Plan amendments at
spec time (recorded in #123): PR 1 takes a path-dep (crate keeps
its own workspace so the gmp dev-oracle stays out of kernel
builds); C12.8's LSQ/SVD move to their consumer PRs (4/7).
Binding specs committed: M5-PR1-SPEC.md (API mapping, D1-D8
divergence handling incl. the D8 floor pin flip, width-pin
inventory), M5-PR3-SPEC.md (geom-core::spline home, Arc payloads
with accepted Copy-loss, the SpanLocate seam, clamped-v1,
Tiller-bound honesty test). **PR 1 implementer DISPATCHED (A/B row
11: M, OPUS — block-4 draw byte 63) and PR 3 implementer
DISPATCHED (row 12: L, fable remainder)** — the two cargo lanes;
PR 2 (C9 ring) queues for the next free lane. Branches:
ev/m5-pr1-interval-adoption, ev/m5-pr3-nurbs-substrate.

**M5-PLAN RATIFIED (2026-07-27): Evan "lgtm!" on #123 → MERGED to
main `4642619` on 18/18 green**, with one rider folded in pre-merge
(R3: the v1→v2 migration is NOT a commitment — no users yet; clean
break if cleaner; the PR 10 spec records the call). Same sweep:
Evan triaged open issues — #95 verified landed at #102 (memo.rs
recursive naming key + grandparent pin) and CLOSED as a
bookkeeping slip; #89 answered open-by-design until the M5 exit
K-snapshot; #104 answered unresolved/banked for v2 (offered an M5
design-unit slot if wanted). His `test (interval)` CI-duration
question answered with run data: 8–12m is the historical norm
(feature-set recompile + gmp on cache miss + genuinely slower
interval instantiation); recommendation on record — no split
before PR 1's gmp removal lands, shard per-crate only if it still
dominates after. One waiter-parking incident: the PR 3 implementer
stopped on background builds mid-battery; nudged back to
foreground rows with the cd-prefix rule (standard playbook), lane
recovered and progressing.

**PR 1 implementation COMPLETE (2026-07-27, ev/m5-pr1-interval-
adoption @ 2dc8860 + report) — opus row 11.** Shape A: ring ops +
transcendentals both from the in-repo crate; inari/gmp/rug gone
from every kernel path incl. Cargo.lock. Headline: ONE predicate
verdict flip found and fixed at the ROOT — split_join_order_u
escalated because div_lo/div_hi padded exact quotients (axis-
aligned v/||v|| stopped being exactly unit); fix = division
exactness witness (fma residual == 0 above the 2Prod floor)
mirroring mul_exact, D1 updated. 5 reported deviations, 0 silent
(div witness beyond sanctioned scope; oracle-computable moved to
sibling crate — cargo resolves path-dep manifests even disabled,
would break builds without Evan's ~/projects/computable checkout;
root exclude addition; acceptance item 3 skipped to avoid
reintroducing the LGPL dev-dep — ruled ACCEPT conditional on
reviewer verifying the wrapper is a pure delegating newtype;
consts path nit). One tripwire consciously inverted:
powi_diverges_from_the_tight_enclosure → containment pin (pad
backends contain their own f64 lane; flagged for review
assessment). Battery: 1342-1343/0 ×3ε default, 1498/0 ×3ε
interval, clippy clean, certify 12/0 vs warm-cache inari oracle.
Build delta measured PAIRED: workspace interval clean 665→478s
(1.39×), isolated geom-core 280→47s (6.0×) — #115's 93× was
crate-vs-crate, not claimed. **Blinded reviewer DISPATCHED**
(F1-F10: div-witness soundness attack incl. subnormal/2Prod-floor
edges, containment fuzz, with_dec_capped laundering, verdict-flip
repro both directions, tripwire-loss assessment, 4 pin
re-derivations, LGPL-free repro, doc-claims audit, e2e consumer
run, battery honesty).

**PR 3 implementation COMPLETE (2026-07-27 late, ev/m5-pr3-nurbs-
substrate, 4 commits) — fable row 12.** geom-core::spline
(knots/basis/locate/algebra as structure PLANS + generic
appliers); NurbsCurve2/3 + NurbsSurface (SurfaceJet, per-column
u-algebra, transpose trick for v); Arc payloads landed, Copy-loss
rippled (~30 clone sites), placeholder constructors for the old
unit-variant construction sites (poison-valued valid structure,
claimed bit-identical). ONE structural deviation, well-argued:
SpanLocate seam unimplementable as crate-private-in-geom-curves
(open-generic T: Decide callers can't name it; the geom crates
can't share a private trait) → shipped public-but-sealed in
geom-core::spline with Decide: SpanLocate supertrait; remaining
T: Real eval call sites became T: SpanLocate sole-bound.
Removal bound = Eq 9.81 mechanism in projective form; planted
Tiller-honesty case: bound 2.094e-3 vs realized 1.074e-3
(ratio 1.95, containment 801/801). Mid-flight interval-lane
catch: knot-straddling boxes hulled EMPTY spans (repeated
interior knots → zero denominators → poisoned enclosures); fixed
by empty-span poison + hull skip. Battery: 1365-1373/0 ×3ε
default, 1529/0 ×3ε interval, clippy/doc/demos/tripwires clean.
**Blinded reviewer DISPATCHED** (F1-F10; sharpest: the
empty-span-skip containment attack — "[u,u] holds a parameter" —
the Dual independent-channel hull soundness at C0/C1 kinks,
rational deriv2 differential vs circle closed form + num-dual,
adversarial removal-bound fuzz with wide weights, and the
SpanLocate-home unimplementability premise verified by attempt).
Lanes: both reviewers running = the two cargo lanes; PR 2 stays
queued; S5 drafting next orchestrator work.

**Third limit outage + recovery (2026-07-27 ~20:10Z → 2026-07-28
~01:2xZ)**: Fable limit killed BOTH blinded reviewers mid-battery
(PR 1's post-certify-lane/pre-division-fuzz; PR 3's mid
test-binaries + surface C0-kink attack). Evan re-logged-in; both
resumed from transcript with surviving clones (standard ladder,
cwd-reset guard in the resume messages). No work lost; both
implementations were already pushed pre-outage.

**S5 PATHS-DESIGN drafted + PR #124 OPENED (2026-07-28)** —
design-conversation PR, WAITS for Evan (watchlisted). J-core
recommendation (legs + per-junction resolver {Sharp, TangentDirect,
TangentArc(r)}), forward-sugar table as requirement, elaboration
semantics with typed failure vocabulary, PQ1-PQ3 forks with firm
recommendations. S2 sequenced after it.

**computable oracle DROPPED (Evan, in-session 2026-07-28)**: "we
can drop the computable oracle — i doubt it adds anything over
inari" — record check confirms zero computable-only catches
(its lane was 4 functions / ~1.5k cases vs inari's 12 families /
~4M; value was theoretical MPFR-common-mode independence only).
Action: PR 1 fix pass DELETES the oracle-computable sibling crate
+ feature plumbing (simpler than the relocation PR 1 shipped).
Caveat on record: if inari-as-dev-oracle is ever dropped too, a
second independent oracle should be reconsidered.

**BOTH reviews returned (2026-07-28): APPROVE-WITH-FIX-PASS ×2,
zero MAJORs, both fix passes DISPATCHED.**
- **PR 1 (opus row 11): 0 MAJ / 3 MIN / 2 NOTE, 1 silent omission
  (stale-claims sweep left 6 live-rustdoc inari mentions).**
  Review highlights: div witness attacked with a 17.5M-case
  exact-rational fuzz (zero unsound firings, zero containment
  violations; subnormal floor correctly refuses); outcome SETS
  compared main-vs-branch — identical except designed changes;
  MUTATION experiment proved the inverted powi test is NOT a pad
  tripwire (the certify lane is; it becomes a gmp-free kernel CI
  row per fix pass); deviation-2's cargo-necessity claim did NOT
  reproduce (harmless move, wrong rationale — mooted by the
  computable deletion); with_dec_capped laundering impossible
  (min-only); e2e rotated-cutter interval boolean green. Rubric:
  idiom 5 / tests 4 / docs 4. Fix pass: 3 MINs + NOTE-1 CI row
  (watcher floor 18→19) + computable deletion + adopt 3 reviewer
  suites.
- **PR 3 (fable row 12): 0 MAJ / 2 MIN / 5 NOTE, 0 silent.**
  Review highlights: empty-span skip HOLDS under degenerate
  [u,u]/straddle boxes at multiplicity-p kinks (wording loose —
  right mechanism: find_span always selects the nonempty span);
  Dual kink conventions consistent (both-tangent enclosure under
  straddle verified — independent-channel hull vindicated);
  deriv/deriv2 + full surface jet FD-verified independently (the
  implementer's circle oracle shared the quotient-rule shape —
  the reviewer's FD is the independent check); removal bound
  sound under adversarial weights/multi-pass, WeightCollapse
  honest; SpanLocate sealing compile-fail-probed +
  unimplementability premise verified by attempt; interval count
  re-derived EXACTLY (1550 − 21 reviewer tests = 1529). Rubric:
  idiom 5 / tests 4 / docs 3. Fix pass: 2 doc MINs + wording
  NOTEs + adopt 21 reviewer tests across 5 suites.
- S5 meanwhile: Evan caught his own fillet(continuation) example
  ("shouldn't land on c→d"); orchestrator laid out the 4 semantic
  options (overdetermined single arc / land-on-carrier /
  coincident-corner fillet / extend-to-intersect) and recommended
  v1 = coincident-nominal-corner fillet(r), uniform across
  in-chain, seams, close; gap forms refuse typed naming
  BiarcJoin/extension as future doors. Awaiting his pick.

**PR 3 fix pass COMPLETE + PR #125 OPENED (2026-07-28)**: all fix
items landed (doc-attachment MINOR, 5 stale surface-doc lines —
one more than the reviewer's 4, honest catch; empty-span wording;
unit_segment(0) + λ-overflow comments; allow-count corrected to
6+0 with the unused_imports allow REMOVED as unneeded); all 21
reviewer suites adopted verbatim from the salvaged scratch (2
marked mechanical lint adaptations; zero discrepancies vs the
review's results). Provenance note for the record: the fix-pass
harness flagged the salvaged-scratch adoption as untrusted-code
integration — the chain is fully in-session (blinded reviewer
authored → orchestrator salvaged from the review clone pre-
reclaim → implementer adopted, tests-only, results matching the
review line for line); no action needed. Post-fix battery:
1387/0 + 1550/0 interval, clippy/doc/fmt clean. Main merged into
the branch (no conflicts); watcher armed (18-row floor). A/B rows
11/12 filled (row 11 fix-pass columns pending). DISK: watchdog
WARN 14G → salvaged scratch + reclaimed both review clones +
curvo clone → 75G.

**S5 round 4 direction (in-session, 2026-07-28)**: Evan proposed
TYPED path ends — ends with/without direction as different types;
fillet consumes only DIRECTED-OPEN ends so `.line_to(p).fillet(r)`
becomes a type error (authored points always lie on the final
path; carriers authored by direction; the trimmed corner is
unauthorable). Orchestrator spelled out the end-state table
(Point / Directed / OpenRay), the .tangent()/.angle() director
unification (arc_tangent_to and start_dir dissolve), the
Path::through carrier form (all-rounded square with no authored
corner), and flagged closure typing (seam-at-junction v1 rule;
mid-carrier seam trips same-carrier rules) as the remaining
sub-fork. Awaiting Evan's nod to fold as round 4 (supersedes the
pending-resolver surface story; the pending resolver survives
underneath).

**PR 1 fix pass COMPLETE (2026-07-28, 12 commits pushed)**: all
items landed — powi comment honesty (names the real tripwires),
7 stale attributions retargeted (found one more than the review's
6), computable oracle DELETED per Evan (the implementer also
produced the missing repro: the disabled-optional-path-dep failure
is real on `cargo build`, not metadata/check — moot now), the
gmp-free crate tripwire row (inari → optional dep behind
oracle-inari; edges.rs split in-place; hosted jobs 12→13; NO
in-repo watcher-floor constant exists — floor 19 lives in
orchestrator watcher invocations from here), NOTE-2 narrowed,
reviewer suites adopted verbatim (fuzz at 1/35 default scale with
floors scaling, full sweep #[ignore]). NEW FINDING from the
adopted harness: Dual::powi(0) launders derivative-channel poison
— M0-era, backend-independent, topology-safe (Decide reads value
only); pinned as-behaves; FILED #126 (Dual contract decision,
non-blocking). Battery: 1343/0 default, 1503/0 interval ×3ε,
clippy/fmt clean, crate 21/0 gmp-free in 1.97s clean build,
certify 12/12 with the oracle feature. PR 1 opens after #125
merges (both branches touch interval.rs — the PR 1 clone resolves
the merge with its warm context).

**PR 1 MERGED (2026-07-28): #127 → main `bfab91f` on 19/19 (the
interval-backend row joins the matrix; floor is 19 from here).**
THE KERNEL IS COPYLEFT-FREE IN EVERY BUILD CONFIGURATION — inari/
gmp/rug gone from the tree; quarantine issue #4's exit condition
is met by removal. The pre-merge main-merge was conflict-free
(disjoint interval.rs regions); a seventh stale attribution
caught at the merge (grep-by-enumeration lesson noted in the PR).
A/B row 11 CLOSED: opus, M — 0/3/2, 1 silent omission, 5/4/4,
moderate fix pass. Both implementer clones reclaimed (disk 123G).
PR 3 clone also reclaimed. Next: DESIGN.md quarantine-text
retirement rides the next state-sync; PR 2 (ring) in flight
(opus, row 13); PR 4 spec (fitting stack) is next orchestrator
work once PR 2 lands; S5 at round 6d, converged, awaiting Evan's
#124 👍.

**Design threads (2026-07-28, in-session with Evan)**:
- **S5/#124 converged at round 7** (six design rounds in ~one
  day): the binding lattice (Open/Point/Angle/Directed = which of
  {position, angle} the tip has bound; two point flavors — plain
  vs directed — making .tangent()'s typing and the Sharp check
  structural), .at/.angle as dual binders, argument-minimal
  .fillet(r) with either-order arrival binding, Open as the entry
  (close symmetry — no privileged first side), all point-targeting
  constructors as sugar, §5b one-Option-pair implementation note.
  PR description rewritten to the converged state. Awaiting 👍.
- **The two-tolerance principle: #129 OPENED** (from Evan's #124
  inline comment + "a more general principle we should adopt"):
  D4 ¶1 addendum PROPOSED — eps_precision ("what we represent")
  vs eps_input ("least precision the user might care about"),
  v1-bound eps_input = K·ε, uniform user recourse below eps_input
  (kernel semantics untouched), D7's ε_in becomes an instance.
  Backed by a 10-pair fork-site audit (incl. two single-decide
  adjacent-arm forks: bool_plane_offset, census gap_is_zero; the
  census escalated arm also LOSES its recourse sentence via {:?}
  — sweep fixes regardless); merge_coplanar_faces is the in-repo
  unification precedent. Rework sweep = M5 side unit on
  ratification. Watchlisted.

**PR 2 implementation COMPLETE (2026-07-28, ev/m5-pr2-interval-
ring, 7 commits) — opus row 13.** RingInterval (always-compiled,
outward-padded ring, NOT a Real — the two-role split held);
Enclosure seam (spec option (a), blanket over Bounds, E0034
hazard documented); spline::hull primitives (span/domain/rational
with positive-weight re-check, ring-side derivative coeffs,
sup-norm helpers). Numbers: 9.7M-case exact fuzz ZERO violations
(comparator generalized to a 72×64-bit fixed-width integer — ±
alignment needs ~2100 bits, PR 1's u128 technique cited); ~3M
differential comparisons vs BOTH in-repo interval impls, max
disagreement 1 step; C2.2 rehearsal bound 2.9× densest sampled
max on an exactly-zero case (sphere limb 2.8e-14 m² = 20 ulps of
r²); planted corruption 12k/12k refused with 59% INVISIBLE to a
9-point schedule (OQ2's argument, measured). 7 reported
deviations, most notably TWO algebraic rules beyond spec (sign
clamp + zero annihilator — claimed ℝ-facts required for the
even-power rule; the interval-square-poison lesson resurfacing).
**Blinded reviewer DISPATCHED** — F1 (the crux): attack the sign
clamp's soundness with an independent comparator (any true-range-
below-zero clamp firing is MAJOR, repair-or-remove never keep);
zero-annihilator × infinity/poison; independent containment fuzz;
rational hull-bound derivation re-check (ratio-of-hulls subtlety);
C2.2 pipeline algebra verification; e2e corrupt-control-point
catch at 2ε.

**PR 2 review returned (2026-07-28): APPROVE-WITH-FIX-PASS, 0 MAJ
/ 2 MIN / 2 NOTE, all 7 deviations reported, 0 silent.** The crux
HELD: sign clamp proof-checked (cannot fire on straddlers; -0.0
correctly nonneg) + 6.8M independent boundary verdicts, zero
violations; zero-annihilator ordering verified (poison → straddle
→ annihilate); rational hull bound re-derived — control-value
convex combination, never ratio-of-hulls; 11.7M total independent
exact verdicts; C2.2 rehearsal algebra hand-verified, numbers
bit-reproduced; e2e plane-residual consumer certifies clean /
refuses 2ε corruption exactly. MINs: from_bounds admits
[+inf,+inf] (no-real bracket, public ctor — poison it); the
branch now REALLY conflicts with main (PR 1 merged post-base —
deviation 7's trivial-merge claim went stale; differential lane's
inari-backed comment too). Rubric: idiom 5 / tests 5 / docs 4.
Fix pass DISPATCHED (main merge + inf gap + comment fixes + 3
reviewer-suite adoptions; clamp/annihilator/hull code frozen as
verified). Meanwhile PR 8 (BVH, fable row 15) implementing on the
other lane; PR 4 dispatches on PR 2 merge.

**PR 2 MERGED (2026-07-28): #130 → main `8e2a610` on 19/19.** The
C9 ring + Enclosure seam + hull primitives are in — fitted-cache
certification substrate complete. A/B row 13 CLOSED: opus, M —
0/2/2, 0 silent, idiom 5 / tests 5 / docs 4, moderate fix pass
(main-merge conflicts + inf gap + 3 suite adoptions). PR 2 clone
reclaimed. **PR 4 DISPATCHED (fable row 14, L)** on the freed
lane: LSQ + projection-with-certified-orthogonality + A9.10-shape
fitting under C6 + the compose module promoted from the rehearsal
(ev/m5-pr4-fitting). Lanes: PR 4 + PR 8 (BVH, mid-implementation,
branch cc25cdb). S5 at ROUND 9 (Start token + NURBS legs + two
tiers — Evan iterating live; rounds 7b/8/9 all his design calls
folded same-session). #129 two-tolerance awaiting 👍.

**Fourth limit outage — MONTHLY SPEND LIMIT (2026-07-28 ~09:30Z →
~11:15Z)**: both implementer lanes (PR 4 mid-battery group C, PR 8
mid-battery chunk C) killed by the account monthly spend cap (a
different mechanism from the Fable usage limits — only Evan's
settings or month rollover clears it). Both resumed from
transcript ~11:15Z and accepted work, so the limit is evidently
cleared; clones survived, no work lost (per-unit push discipline
held). Also this outage window: two PR 4 waiter-parking incidents
pre-outage (the endemic pattern; second nudge needed the explicit
"no notification will ever come" wording — future implementer
prompts get that sentence verbatim).

**Spend-limit outages #2 and #3 (2026-07-28 ~16:30Z, then a long
one ~17:15Z → 2026-07-29 ~00:30Z, taking the ORCHESTRATOR down
too — 7 hourly ticks batched on recovery).** Both lanes resumed
each time from transcript; per-unit pushes held; PR 8's battery
was fully green pre-outage (1582/0 ×3 interval) with only
clippy/doc/fmt remaining; PR 4 mid-interval-row.

**S5 rounds 10–11 (2026-07-28, twelve + five inline comments from
Evan)**: inline-authored NURBS (control polygon absorbs junction
constraints; departure-frame rule made precise and GENERAL —
internal shape data relative, anchors absolute); reverse-tangent/
cusp class (refuse-by-value; NO v1 declaration door — the
material-wedge tension; Evan TABLED the higher question → issue
#131 filed); .turn(δ) sugar; .to(dp) combined binder (Start
always a full directed point); close() dropped entirely;
tangent_arc_to rename; nurbs_mirrored; write-once outgoing /
read-only incoming made explicit; runtime-checks note; PQ1
DELETED (TangentAt superseded; Smooth unplanned — docs don't
reference unplanned things per Evan); PQ2 DECIDED AGAINST mixing
(Evan, associativity); **then() DROPPED** (round 11: it
reintroduced the value-matching seam Start killed; builder
functions cover reuse); fillet(r,dd) sugar dropped; arc-vs-fillet
recorded as one ArcLeg + three binding modes. Evan doing a final
pass.

**#129 simplified per Evan**: eps_input = SYNONYM for K·ε, K the
one knob, decoupling license dropped. **#126 DECIDED (a)** (powi(0)
derivative-poison propagation) — folds into PR 4's fix pass.

**PR 8 implementation COMPLETE (2026-07-29, ev/m5-pr8-bvh, 14
commits) — fable row 15.** crates/bvh BELOW the geom crates (box
constructors in geom-curves/surfaces so PR 7's SSI can consume
the tree — the layering call reported and sound); median-split/
total-tie-break deterministic tree, poison-never-prunes;
span-aware circle-arc boxes (trig in the inclusion TEST only,
values through an outward bracket type); control-hull NURBS
boxes; sweep_pad derived; reduce wired candidates-ascending
(subsequence of the brute scan). Pins: superset 536⊇514 over 43
corpus boolean nodes + disjoint-pruning-engages; bit-equal
bodies/contacts/names/keys f64+Interval kernel+corpus with
EXACTLY the verdict log scrubbed; planted degradation caught.
Latency: die full −29%, corpus total −21% (contended, box-
relative). Battery 1407/0 ×3 + 1582/0 ×3, clippy/doc/fmt clean.
**THE FINDING FOR EVAN (production-path, honest)**: verdict-log
populations are candidate-set-dependent and N5's vdiff consumes
them — vanish diagnoses at interaction boundaries can degrade
from PredicateFlip to the documented evidence-free fallback
under pruning. Shipped: engine-semantics tests pinned under
Idealized; one banked test runs BOTH strategies (realized admits
flip OR exactly the documented fallback). Touches N5's ratified
diagnosis story → goes in the PR writeup for Evan; candidate
future rungs recorded (divergence-channel read; PERF-PLAN
shadow-exec scalpel). Sweep-lane bound became Decide + Bounds
(L7-licensed; Dual booleans statically excluded, never used).
**Blinded reviewer DISPATCHED** (crux F1: independent bit-equal
verification + diagnosis-regression blast radius; F3 circle-box
extremal-inclusion attack; F4 sweep_pad re-derivation; F7
error-channel divergence repro).

**PR 4 implementation COMPLETE (2026-07-29, ev/m5-pr4-fitting @
7a17976) — fable row 14.** LSQ (Cholesky normal + no-pivot LU,
D9-refusal-not-reorder pinned); projection carrying BOTH
residuals — the wrong-branch fixture converges at the antipode
with clean orthogonality and damning DISTANCE, the domain-clamp
fixture the mirror (both laundering modes covered); the Type-2
fit loop with a DIRECT union-refined deviation bound (reported
deviation: PR 3's per-removal accumulation measured
order-too-conservative — 190 ctrl pts from 65 samples;
replacement certifies tighter: 30 pts, worked numbers pinned);
compose module (rehearsal refactor BIT-IDENTICAL, to_bits pins);
the OQ2 standing pin (planted excursion: 9-point schedule
bit-blind, hull catches at 17× band). Battery 1440/0 ×3 +
1615/0 ×3, clippy/doc/fmt clean. CI caveat flagged: curvo as GIT
dev-dep = network on test lanes (review assesses). **Blinded
reviewer DISPATCHED** (crux F1: f64-understatement attack on the
union-refined bound + the bound-vs-locus honesty at the public
surface; projection stagnation-laundering; torus composite
hand-check; curvo dep-hygiene ruling input).

**PR 8 review returned (2026-07-29): APPROVE-WITH-FIX-PASS — 2
MAJOR (both DESIGN, held for Evan) / 6 MINOR / 4 NOTE.** The
review independently verified the bit-equal pin (9 docs / 174
nodes, every field, exactly verdicts differing on the 43 boolean
nodes) and quantified what the report had not: verdict volume
falls −51% under pruning. MAJOR-1: the N5 diagnosis regression is
REAL on the production path (4 pins fail under Realized incl. the
diagnosis-corpus golden; flip-vanish degrades to the fallback;
the fallback arm exists in implementation docs but NOT in
ratified N5, and the fixture re-pointing made the golden's
byte-identity a consequence, not evidence). MAJOR-2: the
Decide+Bounds widening violates L7's sole-bound rule at ~16 sites
incl. editor_core::evaluate, self-licensed in comments, with the
CI grep structurally blind to it. Both go to Evan in the PR 8
writeup with recommendations. Also found: sweep_pad is prose-only
(pad=0 passes the whole suite); circle_arc_aabb unsound for
bracket half-widths > ANGLE_SLOP (unchecked precondition, MINOR
— unconsumed constructor); the strategy-flip memo test is
gold (0 recomputes across all 9 docs). Rubric: idiom 5 / tests 3
/ docs 4. **Mechanical fix pass DISPATCHED** (pad pins incl.
pad=0-must-fail, testing-feature gate on the injector,
bracket-corner atan2, pin-2 vacuity, scrubbed() coverage, poison
proptest, F7 value-channel pin, §4.4 note); the two MAJORs held.

**PATHS-DESIGN RATIFIED (2026-07-29): Evan "lgtm!" on #124 after
THIRTEEN design rounds in ~2 days** — the deepest design
conversation of the project so far, all of it Evan-driven
iteration: from his forward-consuming sketch to the binding
lattice (which-bits-bound typestates), dual binders, directed
points, argument-minimal fillets, the Start token (structural
closure), NURBS legs with implied junction-owned control points
(both ends), the v2 core-representation affirmation, and every
safety property migrated from runtime rules into types.
Merging on green; S2 (arc-leg sugar) unblocks with its
sugar-over-the-algebra shape constraint now ratified.

**#124 MERGED (2026-07-29): main `5bb2d51` — PATHS-DESIGN is
ratified text.** Merged on the docs-only fallback: Evan's lgtm +
prior full-green run on the same branch, delta verified as
docs/PATHS-DESIGN.md only (the 19/19 check failure was the
GitHub ACTIONS BUDGET exhausting — 2-10s refusals, not tests;
Evan confirmed in-session). **Merge-gate posture until the
Actions budget recovers: gate.sh locally per merge-ref for code
PRs** (the standing fallback); docs-only PRs merge on sign-off +
verified doc-only delta. Watchers stand down; gate runs queue
behind the active lanes (PR 8 fix pass, PR 4 review).

**CI closure filter COMPLETE (2026-07-29, on #133) — opus row 16.**
Tool verdict: hand-rolled beats both candidates (determinator =
library-no-CLI; nextest = runner swap + doc-test loss). Shipped:
scripts/ci-filter.py (~40-line cargo-metadata reverse-dep walk,
allowlist ⇒ fails closed), three tiers (docs / closure / all —
any member manifest ⇒ all for feature unification), per-job roots
(watertight rooted at stl with the dev-dep-graph argument; k-lint
deliberately always-on as the sole demos/tour compiler),
ci-local.sh filtered by default with --full documented (Evan's
local-equivalence refinement — the same-soundness argument won
over my degraded-context caution), convention-3 floor text
amended to per-tier row sets. Awaiting Evan's 👍 on #133
(convention amendment) + green. Watcher semantics updated
tier-aware (no-fail + filter-pass, not row-count floors).
Also: #104 CLOSED (resolved by ratified #124); #89 re-confirmed
open-by-design; PR 8's two MAJORs put to Evan directly in-session
(N5 posture; L7 Bounds amendment) with recommendations.

**PR 8 MAJORs RULED (Evan, in-session 2026-07-29): N5 = option
(a)** (accept + document: reachability enumeration + N5 amended
to name sweep pruning; acceptance goldens re-pinned on the
PRODUCTION path with the degraded row explicit; engine-semantics
tests may stay idealized with header notes; shadow-exec recovery
rung BANKED as #134) **and L7 = ratify the narrow amendment**
(real.rs rule text gains the named seams; CI grep refined with a
file allowlist failing on new unlisted `+ Bounds` sites; bvh
charter wording ratified-not-self-asserted). Mechanical fix pass
landed first (pad genuinely pinned — pad=0 now FAILS the suite;
corner-evaluated arc-box extremal interval; injectors behind
sweep-testing feature; pin-2 de-vacuoused; scrubbed() widened;
poison proptest; grazing-plane divergence pinned WITH a
sharpening: the value channel never diverges on the fixture —
only the refusal SITE moves, sweep predicate vs containment
predicate). Ruling increment dispatched to the same lane. Also:
branch cleanup per Evan (31 merged remote branches deleted;
remote now 6); #133 final head GREEN awaiting his 👍; a mid-turn
text-delivery miss re-sent (decision elaborations had been
written between tool calls — lesson: final-message-only for
user-facing content).

**PR 4 review returned (2026-07-29): APPROVE-WITH-FIX-PASS — 0
MAJ / 2 MIN / 5 NOTE, no substantive silent deviations.** The
crux HELD: the direct fit bound survived ~1M-sample randomized
falsification (worst dense/bound ratio 1.0000 — touched: tight
AND sound); rehearsal bit-identity independently reproduced from
origin/main; all five implicit composites hand-derived (incl. the
degree-4 torus with exact |a|² ring normalization); projection
fuzz 400 cases — 0.5% honest interior local-min feet whose
carried distances fail bands (the C2.1 pair working as designed).
Best find: binom_row's exactness claim is FALSE at C(55,26)
(recurrence intermediate exceeds 2^53 — latent soundness edge at
composite degree ≥ 55, unreachable today; MIN-1). MIN-2 curvo
hermeticity RULED by orchestrator: accept-with-stated-risk, the
excluded-oracle-crate pattern recorded as the escape hatch.
Rubric: idiom 5 / tests 4 / docs 4. **Fix pass DISPATCHED**
(binom fix + pin; curvo header; speed-0 cosine doc; #126 OPTION
(a) folded per Evan with the launder-test flip; NOTE renames/pins;
adopt 4 reviewer harness tests — file salvaged to
review-scratch/pr4). PR 4's PR opens on its report; PR 8's on the
ruling-increment report. A/B rows 14/15 rubrics now fillable.

**PR 8 OPENED as #135 (2026-07-29)** after the ruling increment
landed (a391f4a): N5 amended + golden re-pinned on the production
path (0x9d9b_b962_4cac_3156, one-row flip-vanish→fallback change
documented at the pin), L7 Bounds amendment ratified in real.rs
with the CI/ci-local allowlist grep (verified to catch a
synthetic site), bvh charter cites the ratified rule. #134
confirmed filed. Writeup features both ruled MAJORs, the N5
−51%-verdict characterization, and the latency wins (die −29%,
corpus −21%). Tier-aware merge watcher armed (no-fail +
no-pending, then merge). PR 4 fix pass still in flight.

**#135 GREEN (21/21) but merge REFUSED — token scope.** The gh
OAuth token lacks `workflow` scope, and #135 touches ci.yml (the
L7 allowlist grep), so both GraphQL and REST merges 403. Same
reason #133 needed Evan's hand. Recourse: Evan merges in the web
UI, or grants scope once (`gh auth refresh -s workflow`) so
workflow-touching PRs self-merge from here on.

**#135 MERGED (2026-07-29)** after Evan granted `workflow` scope
(`gh auth refresh -s workflow`) — workflow-touching PRs now
self-merge. C10 complete. **A/B block 7 drawn** (difficulty
logged first: S6=S, S2=M; urandom coin): **S6=fable, S2=opus**.
MODEL-AB-LOG rows to follow at dispatch. docs/M5-S6-SPEC.md
committed (message-level sweep of the ten #129 pairs, variants
stay, shared Indeterminate carrier, census {:?} fix; collapse
candidates banked not executed). S6 dispatched to the freed lane
(clone m5-s6); S2 waits for the PR 4 lane (two-cargo-lane cap).

**M5-PR5-SPEC.md DRAFTED (2026-07-29)** while both cargo lanes
run (PR 4 fix pass, S6). Binding content: Curve3::Ellipse (a>b
strictly — a=b refused at construction, one kind per
configuration; corner-evaluated arc-AABB intervals per PR 8's
wedge fix; persistence variant arm, no migration per R3); the
exhaustive C5 table with NO wildcard arms, M2 pairs bit-invisible,
trileans-before-rungs with named lever arms, rung-3 arms refuse
typed until PR 7; closed forms = tilted plane×cylinder,
equal-radius intersecting cylinder×cylinder (radius equality
structural/declared ONLY), plane×cone exact-degenerates with
generic tilt routed to rung 3 permanently (R1); split_edge conic
lane; carrier-side conic pcurve constructors (storage = PR 6).
New-error Display text must follow the D4 two-tolerance shape —
coordination note with in-flight S6 written into the spec.
Dispatch when a lane frees (S2 first per queue; PR 5 = block-8
draw).

**Spend-limit outage #5 (2026-07-29, ~08:30–16:30Z, ~8h):** both
lanes killed mid-run; Evan enabled usage credits and restored the
session. Damage: NONE — PR 4's fix pass had fully committed and
pushed before dying (through 65318c3: all review items + #126(a)
+ the 13-test adversarial harness adopted untrimmed; only battery
re-verification was in flight); S6's 14-file sweep WIP survived
uncommitted in its clone (verified intact). Both lanes RESUMED
from transcripts with cwd-reset guards; S6's first order is
commit+push. Session-restart side effect: all three monitors were
lost — re-armed (away-channel, disk watchdog, hourly heartbeat).
Push-per-unit held for the fifth time.

**PR 4 fix pass COMPLETE, PR OPENED as #136 (2026-07-29).** All
six items shipped: binom_row capped at BINOM_EXACT_MAX=54 with
all-NaN poison rows for n≥55 (ring quotients REJECTED in-code:
RingInterval::Mul widens unconditionally, would break the
rehearsal bit-identity pin) + u128 frontier pin at exactly
C(55,26); curvo ruling recorded in the test header; speed=0
cosine caveat (band the PAIR); #126(a) via
KinkJacobian::powi_zero_deriv_factor with the launder pin FLIPPED
to pin the fix; Underdetermined dual-use doc; RefitSkip surfaced
on FitOutcome with the worked example pinned DegenerateSystem;
reviewer harness adopted UNTRIMMED (13 tests, ~27s). Battery
1454/0 + 1629/0, clippy/fmt clean, doc-neutral. Trivial main
merge conflict (boxes vs fit module lines — kept both). Merge
watcher armed. Row 14 rubric: fix pass moderate, complete.

**S2 DISPATCHED (2026-07-29, opus per block-7 remainder — row 18
at next A/B-log touch).** docs/M5-S2-SPEC.md committed: offset-
carrier center construction for line×arc/arc×arc corners, exact
tangent points, declared-tangency-by-construction, deterministic
branch rule (zero candidates AND ambiguous-two both refuse
typed), angular setback extending the reified fillet_leg_fit in
the same exact-order band, message shape coordinated with the
in-flight S6 carrier, PATHS §3-4 lowering consistency required
with divergences reported. Clone m5-s2. Both cargo lanes now
occupied (S6 + S2). ev/m5-state merged main (post-#135 sync).

**#136 MERGED (2026-07-29): 21/21 green.** PR 4 lane closed —
and with it THE ENTIRE M5 SUBSTRATE PHASE: PRs 1, 2, 3, 4, 8 all
merged (interval backend, C9 ring, NURBS types, fitting stack +
projection + LSQ, BVH). PR 5's dependency set [3; 4] is
satisfied; it dispatches to the first freed lane (spec already
committed). In flight: S6 (fable) + S2 (opus).

**529-outage on both lanes (2026-07-29 ~19:15Z, Anthropic
server-side); recovered ~21:15Z with FRESH FINISHER agents per
Evan's new resume-vs-fresh rule** (memories/resume-vs-fresh-
subagent.md: stopped >1h + remaining work fully specifiable →
fresh agent, not transcript resume). State at interruption: S2's
implementation was COMPLETE and pushed (079dd04 — construction,
gates, taxonomy, K-funnel test for seven fillet gates, main
merged); only battery + report remained → fresh opus finisher
(arm inherited per A/B protocol). S6's sweep was committed
(dbb3c00) but coverage unverified → fresh fable finisher with an
explicit audit-the-diff-against-the-ten-pairs step before
battery. Both prompts carry the full foreground/cwd/discipline
headers. First real application of the rule; saves two ~400k
context replays.

**PROCESS CHANGE (Evan, 2026-07-30): local batteries stop
duplicating CI.** The full-local-matrix discipline (a relic of
the Actions-budget era) is retired: implementers now run only
touched-crate tests (lanes as relevant), fmt, and touched-crate
clippy; hosted CI on the PR is the gate and proves the full
matrix. Persisted to memories/local-battery-scope.md; all future
dispatch prompts carry the narrowed battery clause. Both in-
flight finishers redirected mid-run (S6: four touched crates;
S2: profile only) — this also resolves the 6-hour-battery
pathology (cold interval targets × two concurrent workspace
builds on 9G, compounded by S6's self-matching pgrep poll loops,
killed by the orchestrator; poll pattern corrected to pgrep -x).

**S6 finisher REPORTED (2026-07-30): the interrupted
implementer's sweep was ALREADY COMPLETE** — all ten pairs
correct in dbb3c00; finisher added only the main merge, one fmt
fix, and the battery (touched crates green both lanes locally;
multi-ε interval rows left to CI per the new process). Census
{:?} bug fixed with a no-Debug-leakage regression test; shared
COINCIDENCE_RECOURSE carrier with Indeterminate::payload() view
against double-composition; both collapse candidates BANKED as
spec required; deviations: none substantive (+1 out-of-scope
tone-pass observation on SliverVertex/SliverSector, +1 internal
doc note). **Blinded adversarial review DISPATCHED** (fresh
clone m5-s6-review; charter: exactly-once recourse property
probes across all ~20 arms, no-semantic-change proof, census
leak fix, e2e triggers on three pairs; narrowed local-test scope
per the new CI rule).

**S2 finisher REPORTED (2026-07-30), branch green+pushed at
7ad2995.** The opus implementation shipped complete:
fillet_corner with carrier-named arc legs (trim-invariant datum,
not bulge), signed-radius offset construction (ρ = R − σ·τ·r —
one formula carries internal/external/enclosing), algebraic
bulge (major-arc-correct, no transcendentals), data-independent
four-classification branch rule, seven K-funnel gates with
escalation/refusal recourse parity, 22+3+1 test rows incl. the
vesica ambiguity fixture and the #100 bracket with a √10 arc
leg. Finisher fixed one real bug (discipline-grep trip on scalar
bounds — helpers moved onto typed impls) and added one arc-leg
degenerate row. Reported deviations: 2 pre-existing line-line
tests textually touched (new carrier field; values unchanged),
no both-external arc×arc fixture (argued covered), PATHS
divergences named incl. AmbiguousFilletBranch unnamed in
PATHS-DESIGN (flagged for v2). Full workspace battery ALSO ran
pre-process-change: 1492/0 default + 1672/0 interval. **Blinded
adversarial review DISPATCHED** (fresh clone m5-s2-review;
charter: hand re-derivation + fuzz of the signed-radius
construction, bulge vs atan2 differential, verdict-sequence
invariance, escalation parity, straight-leg unreachability of
the ambiguity). Both reviews now run in parallel.

**S6 review returned (2026-07-30): APPROVE-WITH-FIX-PASS — 1
MAJ / 2 MIN / 3 NOTE, rubric 4/4/5.** MAJOR-1 is the good catch
of the unit: the boolean pair's exactly-on arm embeds plane_eq's
synthesized MarginDiag::Invalid verbatim, so a clean undeclared
flush stack renders "margin is invalid (NaN or a poisoned
enclosure)" — actively false. MINOR-1: Invalid-margin escalated
arms carry ZERO recourse (Indeterminate's Invalid Display omits
the carrier; bool Escalated dropped its old lever) — the
exactly-once property holds only for Value/Enclosure margins.
MINOR-2: two definite arms (VertexCrossesAxis at any magnitude,
SplitParamNotInterior) carry band-recourse that is nonsense far
from the band. F1 no-semantic-change PROVEN (full diff read: only
Display/view/re-export/tests). Four pairs triggered e2e. **Fix
pass DISPATCHED to the S6 finisher** (fable inherited; resume
justified — fresh context): render the honest definite statement
at the Invalid exactly-on arm (decision machinery untouched),
carry the carrier through Indeterminate's Invalid arm, rephrase
the two far-honest sites without value forking, adopt the three
reviewer probes + count==1 tightening. NOTE-1 (collapse
candidates in the PR description) is the orchestrator's.

**S2 review returned (2026-07-30): APPROVE-WITH-FIX-PASS — 1
MAJ / 2 MIN / 3 NOTE, rubric 5/4/4.** The construction crux HELD
completely: hand re-derivation of ρ = R − σ·τ·r for every sign
combination (incl. enclosing r>R), 20k-corner fuzz with 12,714
accepted constructions and zero wrong circles, bulge matched an
independent atan2 oracle on all cases, verdict-sequence
invariance probe-confirmed, straight-leg ambiguity unreachability
proven structurally. MAJOR-1 (diagnostics cluster): arc-leg
setback wraps mod 2π — behind-corner tangent points read as huge
positive setbacks, reach can never classify Negative on arc legs,
NoCornerSideCandidate unreachable for arc×arc, and
FilletDoesNotFit can render the WRONG candidate's numbers
(vesica repro). MINOR-1: one of seven trios breaks recourse
parity (reach escalation renders the fit const). MINOR-2: the
reversed/cusp arm advises declare_tangent — wrong per #131; also
the one PATHS divergence the implementer missed (its two
reported ones verified). **Fix pass DISPATCHED to the opus
finisher** (resume justified — deep sugar.rs context): signed
arc setback, trio parity, honest cusp text, NOTE docs, major-arc
bulge row, probe adoption. Both fix passes now run in parallel;
PRs open on their reports.

**S2 fix pass COMPLETE (342651b), PR OPENED as #137
(2026-07-30).** MAJOR-1 fixed with the signed fold x − τ·⌊x/τ+½⌋
— deliberately NOT atan2 (tried, broke exact-fit trios by one
ulp): bit-identical to the shipped expression on the corner side,
changing values only where wrong; fuzz still accepts exactly
12,714. NoCornerSideCandidate now REACHABLE for arc×arc (492/60k
search hits, clean pinned fixture); misattribution repro now
names the author's own candidate. Trio parity restored (reach →
no-corner recourse both ends + negative assertion); cusp arm got
FILLET_CUSP_CORNER_RECOURSE naming #131 (judgment call flagged:
in-band turn escalation names BOTH doors — it hasn't decided
which class); major-arc branch proven structurally unreachable
(200k search, max θ = 0.987π) and unit-pinned instead of a fake
e2e row; review probes adopted (review_s2.rs, 0.9s). -p profile
124/0 + 134/0. This is the first PR gated by CI under the
narrowed-local-battery process. Watcher armed. Row 18: fix pass
moderate, complete.

**S6 fix pass COMPLETE (0e7acef), PR OPENED as #138
(2026-07-30).** All items: dishonest exactly-on payload fixed by
branching on payload SHAPE only (Invalid → the honest definite
statement; decision machinery untouched; verified via the
reviewer's flush-stack repro); Indeterminate's Invalid arm now
explains the poison AND ends with the carrier (count==1 pinned
for both margin shapes); far-honest rephrasing at
VertexCrossesAxis/SplitParamNotInterior (unconditional lever
first, coincidence levers conditionally phrased, no value
forking); 3 probe suites adopted; ALL S6 contains-pins tightened
to exactly-once (16 assertions). Local touched-crate rows green
both lanes. PR description carries the banked collapse
candidates (review NOTE-1 discharged). Watcher armed. Both S
units now gate on CI in parallel (#137 profile-only — the
closure filter's first real selectivity test; #138 touches
geom-core → full matrix expected).

**#137 MERGED (2026-07-30): 21/21.** S2 done — arc-leg fillet
corners shipped; row 18 columns filled. NOTE on the closure
filter: the full matrix ran for a profile-only change, which is
CORRECT closure behavior — profile feeds sweep→topo→editor-core,
so its reverse-dependency closure is nearly the whole workspace.
The genuine selectivity demo still awaits a leaf-crate or
docs-only PR (the next state-sync PR will be the docs-tier
case). A/B rows 17+18 now filled through fix pass; block 7
complete.

**PR 5 DISPATCHED (2026-07-30) — block-8 draw (difficulty logged
first: PR5=L, PR6=M; urandom coin 0→wait, printed draw): PR5=
fable, PR6=opus.** Clone m5-pr5 from post-#137 main, spec
imported; prompt carries the S6-carrier coordination (merge main
when #138 lands), the R1 permanent cone routing, M2 bit-identity
obligation, no-wildcard discipline, the narrowed local-battery
scope, and the self-match-proof poll pattern. Row 19 to
MODEL-AB-LOG at next touch. #138 still gating.

**#138 MERGED (2026-07-30): 21/21. Both block-7 side units are
on main.** The two-tolerance message unification is live
kernel-wide (ten pairs, one carrier, exactly-once pinned). PR 5
implementer notified to merge main (carrier dependency
satisfied). Side-unit queue: S1 (REST-contact) and S4
(validator consolidation) remain; PR 6 spec (pcurve caches,
opus) is the next orchestrator drafting work once PR 5's shape
firms up. Row 17 final columns filled at next A/B touch.

**State-sync #139 MERGED docs-only — THE DOCS TIER DEMONSTRATED
LIVE**: only `change filter` + `docs-only ok` executed; the 13
build rows were skipped matrix stubs. Zero build minutes for a
docs PR vs 21 executed rows on kernel PRs. All three filter
tiers now proven in production. (The 6-line monitor-script fix
was deliberately excluded — scripts/** fail-closes — and rides
the next mixed sync.)

**S1 DISPATCHED (2026-07-30) — block-9 draw (difficulty logged
first: S1=M; coin): S1=fable, remainder=opus (next unit, S4 or
PR 7).** docs/M5-S1-SPEC.md committed: union-only declared-REST
zip at the join stage (contact patches removed as interior, seam
minted once, splitting reused — sub-frontiers refuse typed with
honest records), undeclared door pinned unchanged, tripwire flip
per the wire's embedded instructions + demos/tour crosslap
upgrade + tier3prime revisit, exact dyadic volume additivity,
purely-structural-or-trios requirement. Two lanes now: PR 5 +
S1. Row 19/20 to MODEL-AB-LOG at next touch.

**PR 5 implementation COMPLETE (2026-07-30, e7a2ec8, 8 commits;
survived one transient 529 with an immediate resume).** The C5
table EXISTS: 36 arms, zero wildcards, trileans-before-rungs,
rung-3 arms refuse naming the routing; splitting executes
Plane+Cylinder; boolean gate stays plane×plane until PR 9. Shape
(i) green e2e: disc cut by tilted plane → exact Ellipse arcs
both parts, bit-identical replay, corpus doc cut_cylinder in
Band 4. 14 new predicates (named lever arms) via k_stats.
Declared-only radius equality PINNED (bit-equal w/o declaration
→ rung 3). M2 bit-identity pinned across the m3/m4/m5 suites
both lanes. 8 reported deviations (notably: no persisted Curve3
in schema v1 → D6.1 rows instead; tilted-cut volume refuses
typed to PR 11; near-circular double gate documented).
**Adversarial review DISPATCHED** (fresh clone m5-pr5-review;
charter: hand re-derivation + fuzz of both closed forms, table
exhaustiveness attack, 6-predicate trio sample incl. the
declared-equality door, independent M2 bit-identity, thin-ellipse
winding attack, deviation re-verification esp. 1/2/8). S1 lane
still implementing (last activity minutes ago).

**S1 implementation COMPLETE (2026-07-30, 8 commits pushed;
survived one 529 with immediate resume).** Root cause found: the
join gated NOT on missing chord partners but on germ-meta
inconsistency — at a REST site the seam direction lies in FOUR
coincident planes, and recl's per-site record survival gives a
segment's two end germs DIFFERENT (a_face,b_face) meta, so
find_match's germ-identity test never fires. The zip
(boolean/rest.rs, ~900 lines): triggers only AFTER a typed join
refusal on a declared ∪; germ rematching with the ambiguous
identity dropped; every declared pair re-verified
(DeclarationContradicted on false pairs — deviation 2 makes
false RESTs never-silent); strut undo in reverse mint order;
seam realization via standard chords; patch discovery by
antiparallel cycle congruence; graft-B-whole ⇒ exact dyadic
volume additivity; slit-zip for adjacent pairs; sub-frontiers
refuse typed RestZipUnsupported (annular pinned). TRIPWIRE FIRED
with the exact volume (1.875); wire retired to certified pins;
demos crosslap ships the glued union; tier3prime pin FLIPPED;
four more embedded-instruction pins re-derived incl. the BVH
diff expected-refusal row now byte-equal Ok. NO new numeric
predicate (purely structural — the ladder is law). ∖/∩ turn out
VACUOUS (classification resolves them: OperandA/Empty — pinned
instead of dead door text). Local rows green both lanes.
**Review dispatch in progress** (clone re-running after a
network timeout).

**S1 review returned (2026-07-30): APPROVE-WITH-FIX-PASS — 1
MAJ / 2 MIN / 3 NOTE, rubric 4/4/4.** MAJOR-1 is the milestone's
most serious catch: hole-creating declared merges (kept+absorbed
sharing two disjoint seam runs) invert outer/ring loop roles —
gates pass, volume exact, but tessellation fails
MismatchedWinding and STL exports SILENTLY CORRUPT (1.8333 vs
5.5). Pre-existing machinery, newly reachable through the lane;
the volume backstop is role-invariant so no gate catches it.
Root-cause claim (germ-meta inconsistency) CONFIRMED empirically
at the merge-base with dbg-join instrumentation; all six pin
flips verified faithful; mispairing attacks failed safe;
deviation-2 stricter semantics regressed nothing. **Fix pass
DISPATCHED** (fable implementer resumed): close the silent class
(correct roles or refuse typed, plus a loop-role tier gate if
contained), narrow the ∖/∩ claim (three-wall counterexample
pinned), narrative alignments, adopt 6 probe files. PR 5 review
still in flight.

**PR 5 review returned (2026-07-30): REJECT — the milestone's
first. 3 MAJ / 3 MIN / 3 NOTE, rubric 5/4/4.** The geometry HELD
(hand re-derivation of all three closed-form families + 500 fuzz
configs, residuals ≤5e-12·scale, interval enclosures contain 0;
table exhaustiveness verified with E0004 break; declared-only
pin real; M2 bit-identity independently confirmed by source-path
walk). The REJECT is M1: even-count conic crossings between
same-side endpoints are INVISIBLE to the endpoint-verdict rule
and the un-cut fallback silently loses a sliver (reviewer repro
on the PR's own corpus geometry: disc cylinder + offset plane →
above=Empty, the 0.25<y≤0.5 sliver gone) — the exact
never-silence class, and no committed test covers it. M2: D9
violation — std trig in pcurve.rs (4 sites) that PR 6 would
persist. M3: split-lane trileans shipped untested + 4 missing
in-band rows. **Fix pass DISPATCHED (fable implementer resumed);
a RE-REVIEW gates the PR** — this unit does NOT go to
fix-then-merge; reviewer's verdict was fix + re-review, then
approvable. S1 fix pass also in flight.

**S1 fix pass COMPLETE, PR OPENED as #140 (2026-07-30).**
MAJOR-1 closed BOTH ways: normalize_merged_roles (Newell winding
through the existing bool_ring_run_winding funnel — outer = the
unique positively-wound cycle; no unique positive ⇒ typed
MergedFaceRoleAmbiguous) PLUS tier-3 check 6 (planar loop-role
winding gate, LoopRoleInverted — fills the battery's own
documented deferral, scoped by the corpus: digon exemption,
line-bounded loops only, curved loops stay deferred/undecidable).
Bridge fixture: exact 5.5, watertight, STL 5.5. Claim narrowing,
comment/narrative fixes, 6 probe files adopted (F1 merge-base
repro as history note). topo 300+568/0 default, 300+296/0
interval; tour green (22 bodies; a chute regression caught and
fixed mid-pass). Watcher armed. Session-limit outage #7 (both
lanes, ~1h) recovered with immediate resumes at the 5am reset.
PR 5 fix pass still in flight (re-review gates it).

**PR 5 fix pass COMPLETE (b08beed), RE-REVIEW DISPATCHED
(2026-07-30).** M1 fixed root-based (conic_crossing_roots;
split_conic_belly_graze R−|D| in meters; per-end
split_conic_crossing_root with typed CrossingEscalated;
split_conic_root_order; lines keep M3's lane bit-identically) —
and the fix EXPOSED TWO further downstream defects, both fixed:
orbit entries now classify by outgoing-tangent side
(split_conic_departure; far-vertex verdicts misread belly arcs)
and adjacency-skip guards verify the in-between edge in-plane
(split_conic_inplane_mid). Belly audit complete: ON-endpoint two-
sided row, tilted 4-ellipse+2-chord row, exact-graze typed row,
and the seam-coincident y=0 cut UPGRADED from refusal to correct
seam-split. M2: libm at all four sites, no committed bit row
moved. M3/m1/m2/m3/n2 all closed (9 unrelated latency rows
reverted verbatim). Re-review focuses on attacking the new
crossing lane (multi-period arcs, start-azimuth straddles, the
seam-cut upgrade's soundness) + regression re-runs.

**#140's only red row = the L7 allowlist grep CATCHING ITS FIRST
REAL SITE (2026-07-30)**: boolean/rest.rs carries the boolean-
seam Decide+Bounds compound bound but was not in the per-file
allowlist — the grep refused, exactly the deliberate-step design
from Evan's L7 ruling. Orchestrator judgment: rest.rs IS the
boolean seam (sibling of ops/reduce/mod, called from the ops
door), so it joins the allowlist in ci.yml + ci-local.sh with a
doc note at the bound site citing the ratified real.rs rule.
Pushed; watcher re-armed.

**PR 5 RE-REVIEW: APPROVE (2026-07-30, rubric 5/5/5 — up from
5/4/4).** All three MAJORs verified closed adversarially; fresh
attacks on the root-based lane held (seam-straddling bellies,
steep tilts, 25ε slivers, grazes both sides); multi-period spans
proven unreachable via the span≤τ gate; the seam-cut upgrade
proven sound (the old refusal guarded a classification gap the
departure lane closes, not a certification); whole_body_side
honest again. **#141 OPENED** with the full REJECT→fix→APPROVE
story. **#140 MERGED (21/21)** — S1 done; the crosslap frontier
closed after living through three milestones. Post-#140 main
merged into #141's branch (clean; topo lib 306/0 sanity row);
watcher armed. A/B rows 19/20 filled; row-16 ordering slip
fixed. Queue: PR 6 spec (opus, block-8 remainder) is next
orchestrator drafting; S4 takes block-9's opus remainder after.

**#141 MERGED (2026-07-30): 21/21 — PR 5 is on main.** The
kernel's first curved boolean: exact Ellipse carriers, the
exhaustive C5 table, root-based conic crossing detection. **PR 6
DISPATCHED (opus, block-8 remainder — A/B row 21, difficulty M
logged at block-8 draw time)**: clone m5-pr6 from post-#141
main, spec imported; prompt names PR 5's constructors as
sources, the meridian-unwrap history, the tier-gate consumer,
and the meters-only certification rule (UV tolerances = review
defect). M5 board: PRs 1-5,8 + S1,S2,S5,S6 + plan + CI filter
merged; PR 6 implementing; then PR 7 (SSI — the milestone's
heart), S4 on lulls, PRs 9-14 beyond.

**S7 DISPATCHED (2026-07-30, opus — block-9 remainder, A/B row
22, difficulty S logged pre-assignment): Evan-directed CI/docs
hygiene.** Three rulings from chat: (1) retire the stale
gmp/LGPL/copyleft-free campaign language (present-tense docs;
factual record stays once; logs untouched); (2) DROP the ε=1e-9
rows from all matrices — 1e-6/1e-12 straddle it — and sync the
"3ε" convention text (ruling supersedes the 3ε battery
convention); (3) keep the interval feature/job but verify the
rust-cache key separates feature sets + optional pure-YAML
split. Demo ruling also recorded: the M5 showcase demo rides PR
11/12 (tessellation/props make the ellipse cut visible; die pips
are PR 12's acceptance); an S2 arc-leg fillet demo stop is ripe
now and rides opportunistically. PR 6 (opus) implementing in the
other lane.

**Branch cleanup (2026-07-30, Evan-prompted):**
ev/interval-transcendentals confirmed superseded — main's crate
is strictly newer where they differ (round.rs exactness-witness
division vs the branch's always-pad; certify.rs evolved; the
branch's unique content is the DROPPED computable oracle) — so
it was archived as tag archive/interval-transcendentals-v1
(commits stay reachable, merge-only ethos) and deleted.
ev/m5-state-sync (merged #139) deleted. Remote is now just main
+ ev/m5-state + live work branches as they open. Interval-lane
ruling recorded: the feature flag stays (build-cost boundary,
not quarantine); S7 handles the cache-key question.

**S7 implementation COMPLETE (2026-07-30, 2 work commits).** Key
finding: DEFAULT_EPS = 1e-9 (tolerance.rs:49) — the retired rows
were RE-RUNS of the unparameterized rows; no coverage lost.
Hosted rows 21 → 18. Cache key verified CORRECT against
rust-cache source (add-job-id-key defaults true; no shared-key
anywhere; interval job doubly keyed) — the interval lane's cost
is genuine compile, and the freed 1e-9 runners shorten its queue
wait at zero cost; optional split declined (10G LRU budget
competition, cold first run — didn't clear cheap-and-safe). Four
honest deviations (grep-binding manifests prose-only; LGPL
workspace-exclusion note kept; k-lint/k-probe 3ε inputs are
baseline-matched telemetry, deliberately untouched).
**Lightweight review dispatched** per spec §5. Note: my merge
watchers are tier-aware (no-fail+no-pending), so the 21→18 row
change needs no watcher edits.

**S7 review APPROVE (0/0/2), PR OPENED as #142 (2026-07-30).**
The pre-existing README shorthand (inari as "dev-dependency" —
actually an optional regular dependency behind oracle-inari)
fixed on-branch by the orchestrator per the NOTE. DEFAULT_EPS
justification independently confirmed (no env override on
unparameterized rows). #142 is workflow-touching (workflow scope
available) and classifies tier `all` — its own run demonstrates
the 18-row battery. Watcher armed.

**RESUMPTION CONTRACT refresh (2026-07-30, Evan warns usage
limit imminent).** In flight: PR 6 implementer (opus, clone
m5-pr6, branch ev/m5-pr6-pcurves, spec docs/M5-PR6-SPEC.md;
nudged to checkpoint-push NOW); #142 (S7) gating with a local
watcher that survives API outages (pure bash+gh — it will merge
on green even during one). Monitors are local processes and
survive. Recovery ladder (memories/resume-vs-fresh-subagent.md):
stopped <1h or context-useful → SendMessage resume with cwd
guard; >1h + fully-specifiable remainder → fresh finisher with
clone path + pushed-commit state + narrowed battery + report
format. After #142 merges: next orchestrator work = PR 7 spec
(SSI, C2/C3; watch for OQ4 pressure → recommendation to Evan,
not unilateral flip); S4 (validator consolidation) queued for a
lull lane; PATHS amendment PR (ambiguity DOF + cusp variant
split) offered to Evan, drafts on request. A/B: row 21 = PR 6
(opus, M); row 22 = S7 (opus, S, review 0/0/2 APPROVE) — table
rows to add at next MODEL-AB-LOG touch. All state pushed.

**EVAN RULING (in-chat, 2026-07-30, on the ambiguous-fillet
plot ~/.local/share/cad-work/ambiguous-fillet.png): fillet
branch ambiguity resolves by NEAREST-THE-AUTHORED-CORNER.**
Grounds: the far tangent circle is always deliberately
authorable as the near fillet of the other corner (the second
carrier intersection), so pick-nearest loses no expressiveness
("is there a way to deliberately force the far one? if so i'm
ok assuming the near one"). Consequences: AmbiguousFilletBranch
retires; a named proximity predicate (total-setback comparison)
picks, with in-band ties escalating F6; PATHS-DESIGN §2 DOF note
amended (ruling = sign-off), resolving S2's recorded divergence
2; the cusp variant split (divergence 3) stays open. **S8
DISPATCHED (fable — block-10 draw, difficulty S logged first;
A/B row 23)**, spec docs/M5-S8-SPEC.md; review charter includes
attacking the sum-vs-max monotone-combination claim.

**RULING REFINEMENT (Evan, minutes later): near-ties do NOT
escalate — pick anyway.** "It is still safe to pick nearest,
since we only can't tell when they're the-same-up-to-epsilon":
both candidates are valid fillets, so the pick asserts no
geometric fact, and below eps_input the author cannot have meant
a distinguishable preference (D4 ¶1 applied to SELECTION, a
nice precedent: escalation is for decisions about truth, not
choices among valid constructions). Spec §1 amended: plain
deterministic selection rule (strict < on total setback, fixed
order, exact-tie broken first-classified), NO Q1 predicate, NO
K-funnel entry, NO new error; one determinism row replaces the
trio. Implementer corrected mid-flight; PATHS amendment text
carries the tie-pick rule.

**RULING REFINEMENT 2 (Evan): equivariant tie-break "at this
point may as well" + the recorded principle** "everything is
equivariant right now, so maintain that if it's free (if that
is indeed true)". S8's selection became a three-rung ladder:
total setback < → incoming-leg setback < (both isometry-
invariant ⇒ rungs 1-2 fully equivariant in ℝ) → enumeration
order ONLY where identical per-leg pairs make equivariance
impossible (candidate-swapping symmetry) — the kernel's first
knowingly-designed non-equivariant residual, documented as such.
Principle persisted to memories/equivariance-principle.md with
the unaudited-premise caveat honored; equivariance audit BANKED,
not assumed. Spec re-amended; implementer re-corrected
mid-flight (second amendment).

**#142 MERGED (2026-07-30): 18/18 — the S7 hygiene sweep's own
gate demonstrated the new two-ε battery.** Hosted rows now 18.
In flight: PR 6 (pcurves, opus) and S8 (equivariant fillet
selection, fable). A/B rows 21-23 to the table at next touch.

**Usage-limit outage #8 (~16:00Z 2026-07-30 → ~03:45Z
2026-07-31, ~11h; Evan re-logged in). Recovered.** PR 6 had
COMPLETED before the cut — implementation pushed at bfc531a
(main merged through #142). S8 died mid-probe (nothing pushed;
clone intact at spec-amendment-1; resumed with the amendment-2
reminder and its open question: 2M trials found NO
enclosing-involved two-survivor cases — probing whether that
class is structurally single-survivor).

**PR 6 report highlights:** SecondaryMap<HalfEdgeKey,
PcurveCache> with certify-only construction (uncertified caches
unrepresentable); CLOSED-FORM ENVELOPE certification (span{1,
cos t, sin t, t} exact coefficients — stronger in kind than the
C2.2 hull, which becomes the fitted-Nurbs limb in PR 7);
unwrap-by-unrepresentability (β ∈ {−1,0,+1}, τ jumps cannot be
expressed); persistence = re-derived on load (recipe-level
posture), pinned both ways. FIVE numbered deviations, notably
(1) PR 5's constructors NOT stored — parameter-non-affine
(~7mm/~0.5mm at-schedule mismatch, MEASURED); the Harmonic chart
image stored instead. **AND a pre-existing PR 5 DEFECT found:
chord_spec's arc-side rule stores the COMPLEMENT arc on the
tilted belly cut** (8 section arcs sweep z∈[−0.297,1.689] on a
height-1 wall; tier 3 blind — both surfaces contain every point
of the wrong arc; premise fails when the divided face spans more
azimuth than the chord). Branch posture: refuse typed
(LoopNotClosed) with a pinned evidence row; the repair (azimuth-
window containment — the same statement PR 6 certifies) needs
its own unit. **Review DISPATCHED** (fresh clone m5-pr6-review;
F7 requires independent merge-base verification of the defect).

**S8 implementation COMPLETE (6f5b1c0, pushed).** The ladder
shipped as a plain selection on the f64 diagnostic channel
(enclosure .lo() at interval, the FilletLegDegenerate
precedent). The monotone-combination question got a PROOF:
non-enclosing two-survivor candidates are mirror-symmetric about
the offset-centers line with same-side radial-projection tangent
points ⇒ componentwise dominance ⇒ sum/max/every monotone
combination agree; crossed setbacks would need enclosing
tangencies, and 27M trials (incl. targeted searches) found ZERO
enclosing two-survivor corners — observed-structural to the
non-enclosing lens class. AmbiguousFilletBranch retired
(no ripple beyond profile); vesica flipped to near-pick (exact
tangency); far-author row yields exactly the old far circle;
PATHS §2 amended dated (divergence 2 resolved, cusp split open).
3 honest deviations (stale spec §3/§4 text noted; rungs 2-3
unit-level with the dominance argument; equivariance memory
imported to the branch). **Review DISPATCHED** — charter: hand
re-derivation of the dominance proof, smarter-than-uniform
enclosing search (construct-from-circles), cross-lane pick
agreement, retirement grep.

**PR 6 review returned (2026-07-31): APPROVE-WITH-FIX-PASS — 0
MAJ / 3 MIN / 3 NOTE, rubric 5/4/4.** Envelope re-derived and
held (angle-sum identity verified symbolically; cancellation
probes never beat the formula); unwrap-unrepresentability
verified; deviation-1 reasoning CONFIRMED (parameter identity is
the binding contract; nothing lost). **chord_spec defect
INDEPENDENTLY CONFIRMED at merge-base** — exact complement arcs
(τ−0.305/τ−0.775), z∈[−0.30,1.70] on height-1, and tier 3's
only refusal (NotIsoRectangle) fires identically on the CORRECT
cut ⇒ genuinely defect-blind. No other configuration regressed
by the branch's typed refusal. Best MIN: certify()'s
snap-to-family admits an ε-shell where the stored envelope is
false by 7 orders (attach path only; minted caches exact);
fix = snap slack added to the envelope. **Fix pass DISPATCHED**
(opus implementer resumed). S8 review in flight in parallel.

**S8 review returned (2026-07-31): APPROVE-WITH-FIX-PASS — 0
MAJ / 3 MIN (doc-level) / 3 NOTE, rubric 5/4/4. The math got
STRONGER in review:** the reviewer's hand proof confirmed
dominance and went further — mixed enclosing/non-enclosing
two-candidate corners PROVABLY impossible (|ρ1|=r−R1 forces a
triangle-inequality contradiction in both στ sub-cases;
line×enclosing-arc likewise); both-enclosing never passed reach
gates in 3M targeted + guided hill-climb (violation→0 only in
the degenerate corner-on-L limit), and even if reachable the
mirror argument still gives dominance. ~160k independent
two-survivor fuzz cases, zero violations; 3 constructor
cross-checks picked the predicted winner. Honest MIN-1: the
cross-lane "same candidate" sentence is an overclaim (hairline
lens can legally split lanes — ruling-compatible, both valid);
fix = say what's true (deterministic per lane; agreement above
enclosure width). **Fix pass DISPATCHED (doc-level).** Both fix
passes now in flight; PRs follow.

**S8 fix pass COMPLETE (52ed526), PR OPENED as #143
(2026-07-31).** All six items landed: honest cross-lane wording
(each lane deterministic; agreement above enclosure width;
sub-width split harmless per ruling) + ulp-perturbed-lens
determinism rows both lanes; line×arc mirror proof written; spec
§3/§4 aligned; committed-evidence citations; reviewer probe
adopted as-is + trimmed 300k dominance-fuzz row (4.5s). Watcher
armed. Writeup carries the full ruling story incl. the
componentwise-dominance/impossibility results.

**#143 MERGED (2026-07-31): 18/18 — S8 done.** Nearest-corner
fillet selection live; AmbiguousFilletBranch retired; PATHS §2
amended; the equivariance principle recorded. Fourteen PRs
merged this milestone. In flight: PR 6 fix pass only (active,
last activity seconds ago). A/B rows 21-24 owed to the table.

**PR 6 fix pass COMPLETE (7feb37d), PR OPENED as #144
(2026-07-31).** Snap-slack fix shipped (r·(|pa.x|+|pb.x|+
|pl.x−β|·reach) added to the envelope; provably ZERO on every
minted cache — pinned; reviewer probes adopted with an
O(ε)-tightness bound so the slack can't blanket-pad); trim-
window doc states what IS certified; max_residual split from
the envelope accumulator; stale-row posture documented
module-level; seam probes adopted; belly merge-base probe = a
history note (unreachable at HEAD by design). Writeup features
the chord_spec finding prominently. Watcher armed. Next
orchestrator drafting: the chord_spec repair spec (azimuth-
window containment) + PR 7 (SSI).

**#144 MERGED (2026-07-31): 18/18 — PR 6 done, C4 discharged.**
Fifteen PRs merged this milestone. Board is CLEAR — no lanes in
flight. Next: chord_spec repair spec + PR 7 (SSI) spec, then
dispatch both (block-10 remainder = opus owed to the next unit;
block-11 draws fresh).
