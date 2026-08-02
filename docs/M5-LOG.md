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

**S9 DISPATCHED (2026-07-31, opus — block-10 remainder; A/B row
owed at next table touch, difficulty S logged here pre-
assignment): the chord_spec azimuth-window repair.** Spec
docs/M5-S9-SPEC.md: the divided face's azimuth window from the
run structure replaces sample-membership; exact interval
containment selects; neither/both refuse typed; in-band window
boundaries escalate (azimuth×radius arm); the belly refusal row
FLIPS to certified pass + the PR 6 mint-on-repaired-bodies row
closes the loop with the machinery that found the defect.
Review charter: construct any old-vs-new disagreement outside
the belly class (spec claims none); attack seam-placement
independence. PR 7 spec drafting begins now in parallel
(orchestrator; OQ4 checkpoint per the standing note).

**M5-PR7-SPEC.md DRAFTED (2026-07-31).** OQ4 DISCHARGED at spec
time, carrier-primary stands — the ℝ⁴ trace's shared parameter
is exactly PR 6's ratified parameter-identity contract (both
pcurves as projections of one traced object; the 3-D carrier
stays the certified authority; no re-plumbing) — no fork goes to
Evan. Binding content: Hoffmann §6.2 stepper (untrusted, the
PERF-PLAN dual-code pilot with the idealized tiny-h suite in CI
from day one), fixed-shape SVD joins linalg here, ℝ⁴ trace,
per-arm trace shapes in the table, the FULL three-limb C2
certificate (OQ2: both always), in-op exhaustiveness doubling as
seed generation (brute-force cells OK; BVH swap rides the merged
differential suite), σ₂-sliver → C7 refusals, shape (iv)
small-loop found-or-typed as the signature acceptance + a
directly-authored NURBS wall for shape (iii)'s substrate.
Dispatch: next freed lane (block-11 draw at dispatch; S9 opus in
flight now).

**S9 implementation COMPLETE (f45d4e1, pushed) — and it found a
SECOND belly-class member shipping silently at main:**
on_endpoint_belly_cut stored complement arcs (5.8958 rad,
z∈[−1.860,3.360] on height-1) and PR 6's mint pass ACCEPTED the
bodies — unlike member 1, this one produced a wrong body TODAY
with no refusal anywhere. Repaired along with the tilted case.
The exactly-two claim is instrumented (both rules diffed across
all 28 mints in topo+sweep, both lanes: 2 changed, both belly,
16 SAME). Window = hull of run half-edges' exact chart extents,
branch-pinned by PR 6's loop walk; selection = exact interval
containment with four metered margins; deviations incl. the
centre-reduction fix (edge-anchored reduction straddled the
period boundary and widened interval enclosures to a full
period) and the width≥τ early BothContained arm. Belly rows
flipped to certified passes; rotation-invariance row (total
sweep invariant under 0.7 rad rotation); repaired bodies mint
certified pcurves. **Review DISPATCHED** — merge-base
verification of member 2 is load-bearing (main ships a wrong
body); charter attacks the mixed-run/seam-crossing walk, the
centre-reduction, and the exactly-two claim with constructed
counter-candidates.

**PR 7 DISPATCHED (2026-07-31, opus — block-11 draw, difficulty
L logged first; remainder fable to the next unit).** Clone
m5-pr7 from post-#144 main, spec imported; prompt carries the
OQ4 discharge, the untrusted-stepper contract, the dual-code
pilot obligation, the three-limb certificate, the S9
fold-mid-flight note, resilience checkpointing, and the full
consumed-machinery reading list (PR 2 hulls, PR 4 fit/projection
/compose, PR 5 table, PR 6 doors). Two lanes: S9 review + PR 7
implementation. S4 remains the lull-queue unit.

**S9 review returned (2026-07-31): APPROVE-WITH-FIX-PASS — 0
MAJ / 3 MIN / 3 NOTE, rubric 4/5/4, no re-review.** F2 CONFIRMED
INDEPENDENTLY at merge-base 5fab705: the on-endpoint belly
stores complement arcs and validate_pcurves returns ZERO errors
— main ships a silent wrong body today. F3 supported with the
right property: any constructible old-vs-new disagreement
outside the belly class refuses (NeitherContained) rather than
picking wrong geometry. F4/F5 soundness verified with explicit
headroom arithmetic. MIN-1 is a nice catch: the new
SectionArcWindow definite arms miss the S6 message shape while
their band-adjacent Escalated arm carries it — the exact fork
the sweep unified, recreated by fresh code (process note: new
error sites need the two-tolerance shape called out in specs;
S9's spec said so for escalation but not the definite arms).
**Fix pass DISPATCHED; S9 merges with PRIORITY on its report**
(main is wrong until then). PR 7 implementing in parallel.

**S9 fix pass COMPLETE (bbb19f4), PR OPENED as #145 with MERGE
PRIORITY (2026-07-31)** — the writeup leads with the fact that
main ships a silent wrong body until this lands. All fix items
shipped: SectionArcWindow definite arms carry the two-tolerance
shape (width-τ-exact vs τ−5e-9 message pair pinned);
centre-reduction comment states the true (τ−width)/2 bound;
interval belly row completed (span sums + bit-replay at
Interval); short-circuit metering + uncertified-input rustdoc;
reviewer's disagreement probe committed verbatim; the
merge-base silent-acceptance measurement recorded as the history
note explaining WHY loop-closure isn't a wrong-arc detector.
Watcher armed. PR 7 implementing in the other lane.

**#145 MERGED (2026-07-31): 18/18 — the silent wrong body is
FIXED on main.** S9 done; sixteen PRs this milestone. PR 7
implementer notified to fold main (chord_spec signature change +
SectionArcWindow). Sole lane in flight: PR 7 (SSI, opus). Queue:
S4 on a lull; PR 9 spec after PR 7's shape firms; A/B table rows
21-25 owed (PR 6, S7, S8, S9, PR 7 draws recorded in log
entries).

**PR 7 implementation COMPLETE (2026-07-31, 3e4a201, pushed) —
SSI IS REAL.** Highlights: fixed-shape Householder+Jacobi SVD in
linalg (min-norm solve IS Frenet's γ₂=0 — documented derivation
unifying both trace shapes); jet-based RHS (Poly3, no derivative
tensors); the full three-limb certificate incl. a NEW
geom-surfaces::projection (surface foot points didn't exist) and
the box-chain graph criterion with a LADDER-searched tube
radius; exhaustiveness with an asserted receipt (examined ==
excluded+accounted+refined) doubling as seeding. Shape (iv)
signature met: sphere×0.08-cylinder whose ENTIRE locus is two
interior polar loops — boundary seeding reaches nothing, found
by subdivision (166 seeds), floor-refusal variant pinned. 13 new
ssi_* predicates. cylinder×sphere retired (all limbs). A real
interval-cancellation bug found+fixed en route (w = q − â(q·â)
form recovers ~0.8 m² of hull width; exclusion never fired
before). FIVE numbered deviations, notably (1) plane×NURBS NOT
retired — limb 2 needs tensor-product Bernstein composition
(compose is curve-only); shape (iii)'s substrate row UNMET,
refusal names the blocker; disposition (accept+bank as a unit vs
fix) rides the review's feasibility assessment. (3) pcurve
storage variant deferred to PR 9's zip with the identity
demonstrated by SSI's own (stronger) limbs. S9 fold was clean
(split.rs vs join.rs — no overlap). **Review DISPATCHED**
(charter: hand re-derivation of the graph criterion — can a
disjoint branch thread the tube?; exclusion-cannot-lie probes;
independent SVD differential; per-deviation verdicts).

**PR 7 review returned (2026-07-31): APPROVE-WITH-FIX-PASS — 2
MAJ / 6 MIN, rubric 4/4/4.** The core HELD: 8000-matrix
independent SVD differential clean; min-norm-is-Frenet
re-derived exact; w-form cancellation verified 30× with
exclusion-cannot-lie probes (~1400 boxes incl. locus-planted);
the reviewer's own adversarial loop pair SMALLER than the seed
floor was FOUND; receipt asserted; h_fit-cap reasoning
confirmed; tube ladder deterministic. M1 = std powf(0.25) in the
step rule (one line, → sqrt().sqrt()) + latent jet sin_cos fork.
**M2 RULED (orchestrator): ACCEPT-AND-BANK as PR 7b** — the
reviewer confirmed the centered second-order tightening is
constructible but likely not ε-practical; tensor-product
Bernstein composition is the clean fix and is its own reviewed
unit. PR 7b = tensor compose + plane×NURBS retirement + shape
(iii) substrate row; MUST land before M5 exit (shape (iii) is
exit-gating); sequence alongside/after PR 9. Best MIN: m3 —
limb 3's doc claims bare IFT but the code supports the stronger
mean-value/convexity argument; disjoint-component exclusion is
the ACCOUNTING's job (stated honestly now). m4 = refined-seed
dedup (duplicate SsiBranch possible). **Fix pass DISPATCHED.**

**PR 7 fix pass COMPLETE, PR OPENED as #146 (2026-07-31).** All
nine items: powf → sqrt·sqrt + jet sin_cos route unified (M1);
limb-3 doc states the real mean-value/convexity theorem and the
uniqueness/completeness division of labor (m3); refined-seed
dedup with the distinguishing fixture (m4); BoundaryInBand
labeled honestly (m5); trios + SelfCrossingLocus row + k-funnel
verdict-log row (m6 — note: Probe's margin sink is generic-T,
the SSI lane is f64-only, so the verdict log is the recorder);
chart-lane tube margin ÷ chart stretch, upper bound = safe
direction (m7); the split_edge meter boundary pinned in topo
with PR 9 named as the end-to-end trigger (m8 partial —
unconstructible until the zip); PR 7b citations everywhere (M2);
all three review probes adopted verbatim (8000 matrices in
1.0s). Watcher armed on #146. When it merges: the M5 spine
through SSI is DONE; next = PR 9 spec (curved booleans + 
tangency) and PR 7b spec, then S4 on a lull.

**#146 gate RED (2026-07-31): the multi-ε battery caught the new
SSI suite hardcoding ε=1e-9 assumptions** — corruption magnitude
4.889e-7 correctly ACCEPTED at eps=1e-6 (the test's premise
wrong, not the kernel); differential band and trio in-band
probes placed at default-band scale. The narrowed local battery
runs touched crates at DEFAULT ε only, so this class now
surfaces at the gate — by design, and it worked. Implementer
redirected: scale probes/corruptions from the resolved band (the
multi-ε-green suites' idiom), explicit skip-with-reason only
where scaling is dishonest, verify locally at 1e-6/1e-12/
interval before re-push.

**#146 ε-fix pushed (2026-07-31).** All test placements scaled
from the resolved band (definitely_positive = escalate·100,
in-band = midpoint; corruption = definitely-outside at every ε;
tangent-arc sample count derived from the ε^(1/4) law). One
resource-bound case became a KERNEL budget, not a test skip:
SSI_MAX_FIT_SAMPLES=1200 with typed FitSampleBudget (the
ε=1e-12 row wants ~4015 samples and a cubic-cost fit; the row
stands down on the typed refusal, pinned by its own row —
scaling the fixture instead would hold r/ε constant, an 80 m
"small loop"). Bonus find: the differential's distance probe
was converging to a clamped domain end (5.6e-4 m reported where
a dense scan puts ~0) — replaced with scan+seeded-projection
min. Local: 21/21 at 1e-6/1e-9/1e-12/interval. Watcher
re-armed.

**#146 MERGED (2026-07-31): 18/18 — SSI IS ON MAIN. The M5
spine through PR 7 is DONE.** Seventeen PRs this milestone. The
kernel now: represents/fits/certifies NURBS (PRs 2-4), cuts
exact conics through the exhaustive table (PR 5 + S9), stores
certified pcurves (PR 6), spatially indexes (PR 8), marches and
certifies general curved intersections with proven
exhaustiveness (PR 7) — plus the planar side units (S1 REST
zip, S2/S8 arc fillets with the equivariant ladder, S6
messages) and the process/CI improvements (S7, #133, #139).
Remaining to exit: PR 7b (tensor compose, exit-gating), PR 9
(curved booleans + tangency), PR 10 (sweeps/lofts), PR 11
(tessellation/props — the demo moment), PR 12 (fillets/die),
PR 13 (STEP), PR 14 (exit sweep + #89 K-snapshot), S4
(non-gating). Next orchestrator drafting: PR 9 spec + PR 7b
spec; board currently idle.

## HANDOFF SEAM (2026-07-31): successor orchestrator briefing

Predecessor (cad-implement-m5 session) hands off at the
spine-through-PR-7 seam by Evan's suggestion. State: 17 PRs
merged, board idle, no lanes in flight, all clones deletable
(verify unpushed-commit-free first per standing rule).

**Immediate work order for the successor:**
1. Draft docs/M5-PR9-SPEC.md (curved booleans end-to-end + the
   C7 tangency regime — TangentIntersection, second-order sector
   classification, the K-funnel predicate family; consumes
   PR 5/6/7 + the S1 zip precedent; PR 9 also triggers the
   pcurve storage variant deferred at PR 7 deviation 3 and the
   end-to-end Nurbs split row at m5_pr7_split_meter).
2. Draft docs/M5-PR7B-SPEC.md (tensor-product Bernstein
   composition in geom-core; limb-2 tight bound; plane×NURBS arm
   retirement; the shape (iii) substrate row — EXIT-GATING).
3. Dispatch both to fresh A/B draws (block 12; log difficulty
   FIRST; blocked pairs; MODEL-AB-LOG rows 21-25 need their
   table entries — draws recorded in log entries above).
4. Then per plan: PR 10 (sweeps/lofts + the R3 migrate-vs-break
   consultation with Evan), PR 11 (tessellation/props — the
   DEMO moment, Evan's ruling above), PR 12 (die pips), PR 13
   (STEP), PR 14 (exit + #89), S4 in any lull.

**Standing process (all verbatim-binding, sources in memories/
and this log):** one implementer + one blinded adversarial
reviewer + one fix pass per unit; binding orchestrator specs
BEFORE dispatch; OUTPUT DISCIPLINE headers; the foreground
clause verbatim + "THAT NOTIFICATION WILL NEVER ARRIVE" + the
pgrep -x (never self-matching -f) poll pattern; cwd-reset guard
in every prompt AND every resume; narrowed local battery
(touched crates, default ε + interval; CI is the gate —
memories/local-battery-scope.md); resume-vs-fresh rule
(memories/resume-vs-fresh-subagent.md); push-per-unit; clones
under ~/.local/share/cad-work/; two cargo lanes max; tier-aware
merge watchers (no-fail + no-pending via gh pr checks loops);
monitors re-armed at session start (scripts/monitors/); A/B
blocked pairs with blinded reviewers (fix pass inherits arm;
design/specs/reviews stay Fable); state-sync PRs at seams;
disk watch (~30G per active lane); new error arms follow the
two-tolerance shape INCLUDING definite arms (S9 lesson);
equivariance principle (memories/equivariance-principle.md).

**Open with Evan:** Q9 (name), #131 (cusps), PQ4, #89 display
half, the PATHS cusp-variant split (divergence 3, unruled), the
PR 10 schema consultation (commitment: consult BEFORE executing
either migrate or break).

**Watch out for:** spend/usage limits (8 outages this session;
the recovery ladder is in this log; Evan re-logs-in), waiter
parking (the verbatim clause prevents it), sed on MODEL-AB-LOG
(use Edit — two ordering slips this session), away-channel
echoes of your own comments (not Evan), the demo ruling (M5
showcase rides PR 11; an S2 arc-fillet demo stop is ripe
anytime).

**NEW ORCHESTRATOR PICKUP (2026-07-31).** Monitors armed
(away-channel/disk/hourly, re-installed from checkout); 14
merged-lane clones deleted (~89G freed, disk 124G free) via NEW
scripts/clean-lanes.sh (Evan's suggestion: per-directory
check-then-delete adjacency — no stale global audit; 19-case
fixture matrix + shellcheck clean; committed with a
worktree-disk-hygiene memory pointer). A/B rows 21-25 backfilled
to MODEL-AB-LOG from the log entries. Handoff discrepancy resolved
against the draw record: "fresh block-12 draws for both"
contradicted PR 7's dispatch entry — the block-11 FABLE remainder
was never consumed (only fix passes followed, and they inherit
arms); the protocol wins, the remainder goes to the next unit.
Ops note: clean-lanes.sh and session ops scripting sit OUTSIDE the
A/B experiment (no blinded lane — the scripts/monitors/ precedent).

**SPECS DRAFTED + BOTH LANES DISPATCHING (2026-07-31).**
docs/M5-PR9-SPEC.md: zip with per-arm CurvedBooleanUnsupported
retirement; rung-3 edges at rest (EdgeCurve::certify Nurbs flip +
the m8 end-to-end split row); the pcurve storage variant (PR 7
deviation 3, Copy drops per the Surface precedent); C7
TangentIntersection by classification with the jet schedule;
second-order sector trilean family (K funnel's second
ill-conditioned crop, telemetry from birth); OQ7 mark +
jet-determinate must-carry pinned both directions; C12.5 cosurface
merge; C12.4/OQ5 census text + 3′ frontier; the (Plane,Nurbs)
boolean arm wired one-7b-flag-flip-from-live. docs/M5-PR7B-SPEC.md:
tensor-product Bernstein composition with the difference formed at
the coefficient level (the cancellation IS the content — review M2
made a doc obligation); ssi_hull_sup_chart keeps its K name across
the swap; plane×NURBS arm retires with its proof (C12.1); shape
(iii) substrate row lands GREEN — EXIT-GATING. **Difficulties
logged pre-assignment: PR 9 = L, PR 7b = M.** Arms: PR 9 = FABLE
(block-11 remainder; dispatched first per handoff order); block-12
draw (byte 172, coin 1) = (fable, opus) → PR 7b = FABLE (slot 1),
OPUS remainder owed to the next unit (PR 10 or S4). Two cargo
lanes = at cap; reviews stagger behind implementations.

## PR 10 (2026-08-01): sweeps/lofts as definitional feature nodes; schema v2

Branch `ev/m5-pr10-sweeps-lofts`. Spec `docs/M5-PR10-SPEC.md`
(binding). Shipped §1 (the `Loft`/`Sweep` node vocabulary), §2
(the §10.3/§10.4 definitional geometry) and §4 (the schema-v2
clean break) in full; §3/§5's SOLID is frontier-blocked and the
refusal is pinned, not skipped.

**Schema v2, as landed.** `SCHEMA_VERSION = 2`. The migration
chain became a real (empty) step TABLE, `migration_step(from) ->
Option<MigrationStep>`, so "no step exists" is one fact in one
place; the loader walks it for AVAILABILITY before it parses a
byte of body, and a gap raises the new typed
`PersistError::SchemaTooOld { found, supported, missing }`. Its
message names all three and ends on the one shared
`REGENERATE_RECOURSE` carrier, composed exactly once (version
comparison is exact integer arithmetic, so the arm has no in-band
twin and the two-tolerance discipline explicitly does not apply —
stated in the docs so the omission reads as a decision).
`PersistError` gained a full `Display`/`Error` impl on the way.
The single in-tree v1 file (`tests/golden/v1_golden.cad`) was
REGENERATED as `v2_golden.cad` and the v1 bytes were KEPT — as
the refusal fixture, because a break nobody can demonstrate is a
break nobody can trust.

**The geometry.** `sweep::skin` (numbered deviation 1: it lives
in `sweep`, not `geom-surfaces`, because `geom-surfaces`
deliberately does not depend on `geom-curves`). Exact analytic →
NURBS section conversion (lines degree 1; arcs the standard
rational quadratic split into quarter-turn sub-arcs — the carrier
circle, not a fit of it); §5.5 degree elevation + §5.3 knot
merging for compatibility; §10.3 skinning as ONE homogeneous ℝ⁴
collocation solve through the new
`geom_curves::fit::interpolate_columns` (all columns as
simultaneous right-hand sides, so they cannot drift relative to
each other); §10.4 by instantiate-and-skin with a path-FOLLOWING
frame that refuses typed on a reversing tangent. Structure
selection is `f64` and the produced surface lifts to any scalar
(`lift_surface`) — Q8 says the control bits ARE the definition,
so every lane encloses the same surface.

**The frontier (numbered deviation 2).** §3 asks the lofted body
to validate at tier 3. It cannot at this PR's merge time and the
blocker is not this PR's geometry: `topo`'s tier-3 check 1
refuses `Surface::Nurbs` BY KIND (`UncertifiableSurface`) and
`geom_brep::EdgeCurve::certify` refuses NURBS carriers and
NURBS-naming descriptions as `Unimplemented`. That flip is PR 9's
charter, verbatim (`docs/M5-PR9-SPEC.md:36`: "`EdgeCurve::
certify`'s Nurbs-carrier refusal FLIPS — this PR mints the
kernel's first rung-3 edges at rest"), and PR 10's own spec says
it does not depend on PR 9. So the node BUILDS its definitional
walls and then refuses
`NodeErrorKind::CurvedSolidFrontier { what }` — a named
sub-frontier on the `RestZipUnsupported` precedent — and
`sweep/tests/m5_pr10_frontier.rs` DEMONSTRATES the blocker on a
real tier-3-valid extrusion with one wall's surface swapped for a
real skinned NURBS. Consequences: shape (iii)'s loft-body row,
the plane-CUT-of-loft row, the Band 4 corpus row and the render
all move to the PR that closes the frontier; the corpus tally's
`NODE_KINDS` gained `Loft`/`Sweep` so they report at ZERO rather
than reading as covered. The demo stop
(`demos/tour/src/skinned.rs`) narrates and MEASURES the walls,
then pins the frontier with a retire-on-closure panic carrying
the flip instructions — the `curvedcut` pattern.

### PR 10 fix pass (2026-08-01)

Review APPROVE-WITH-FIX-PASS, 1 MAJ / 4 MIN / 4 NOTE, rubric 5/4/4.
The math held under independent attack: the skinning matched a
closed-form rational loft BETWEEN sections, Eq. 10.8 matched an
independent derivation exactly (including the cross-row average and
the pinned-row abstention), the schema break survived all eight header
attacks, and Interval containment held on a dense grid over both loft
and swept walls. All four reviewer probe files were ADOPTED into the
PR (`review_m5_pr10*.rs`, 21 rows) and extended.

**MAJOR-1, the Sweep node's dead lane.** `wire_sweep` carried a
single-segment path lane that CANNOT run: a `Node::Sweep`'s path
operand is a profile, a validated profile's loop is closed, and a
closed chain has ≥ 2 segments — even the minimal two-vertex loop is
two half-turn arcs. The reviewer's executed witness refused on both a
rectangle path and that minimal circle. Ruled: collapse rather than
invent expressibility. The node now runs every RECIPE door (both
structural slots, both operands) and then refuses ONE arm naming a
joined-path composition lane — no PR number, because none is
scheduled to build one. `sweep::sweep_geometry` stays live and
exercised through the library API, the demo stop and the acceptance
suites; it is the NODE lane that is gated.

**MIN-1, dead error arms.** `SkinError::Escalated` was never
constructible — this module makes no banded decisions, every
comparison in it is exact-`f64` structure selection (C6) — so it was
DELETED rather than advertised. `DegenerateSection`'s docs now say
where the banded half of its user situation actually lives
(`profile::validate`'s `vertex_separation` /
`segment_straightness` / `arc_diameter_clearance`, one layer up) and
why it still names that door's recourse. `OpenClosedMixed` was wired
GENUINELY: `loft_geometry` now compares chain closure per loop across
sections and refuses when they disagree, with the loop index and the
expected closure in the payload. Reachable at the library door (a raw
`SectionSegments` chain can be open) though not at the recipe layer;
rows for both orientations.

**MIN-2, the tangent claim.** "A reversing tangent refuses typed" was
false in float: at an exact half-turn `|t₀ × t₁|` evaluates to
≈ 1.2e-16, so the anti-parallel arm does not fire and the frame is
built from an ill-conditioned axis. Doc and log now claim only what is
true — a VANISHING tangent refuses — and the knife edge carries a C6
comment. No band was asserted: `sin` is a dimensionless sine, not a
length, and converting it needs a lever arm this construction does not
have (D4 ¶1 forbids inventing one). Two rows pin the executed
behaviour, and the half-turn row FAILS LOUDLY the day a real angular
predicate lands.

**MIN-3/MIN-4.** Spec §2's numbered note on dense-collocation size
limits landed as deviation 5 (below). `interpolate_columns`'s ragged
rows got a shaped `FitError::RaggedRows { row, width, found }` instead
of widths stuffed into `ParamCountMismatch`'s parameter/point fields.

**NOTE.** The break's honest edge is now stated in `persist`'s module
docs and pinned: v2 changed the VOCABULARY, not the wire format, so a
hand-edited `schema: 2` header over a v1 body loads. Inherent to a
break with no format change; not a gap the door can close.

**Deviations, complete (3–5 backfilled).** 1: `skin` lives in `sweep`,
not `geom-surfaces` (which deliberately does not depend on
`geom-curves`). 2: no tier-3 loft body; the frontier is pinned and
demonstrated. 3: the construction is `f64`-structure + `T`-lift, not
`T`-generic — fitting is `f64`-only across this codebase by design and
Q8 makes the control bits data, so "evaluation generic over `Real`"
means evaluation, not construction. 4: the Sweep NODE is fully
frontier-gated (was: "single-segment paths only" — retracted at the
fix pass as unreachable). 5: the §10.3 solve is DENSE — `k × k` in the
section count, `O(k³)` plus `O(k²·n)` substitutions, no banded solver
because PR 4's stack provides none; realistic lofts sit at `k` in the
single digits to low tens where this is unmeasurable, and banding
starts to matter around `k` in the high hundreds (M7 scattered-data
territory). The matrix IS banded at half-bandwidth `q`, so the upgrade
is a solver swap behind the same entry.

**Post-merge correction.** PR 9 (#152) merged during this fix pass and
partially opened the NURBS-carrier door — `EdgeCurve::certify` now
accepts a NURBS carrier under an `Intersection`/`TangentIntersection`
description of two ANALYTIC surfaces. Tier 3's `Surface::Nurbs` KIND
gate is untouched, and NURBS-naming descriptions plus NURBS carriers
under CONVENTIONAL descriptions — exactly a loft's iso-parameter seams
— still refuse. Both frontier messages were rewritten to that truth
rather than left to describe a main that no longer exists.

## PR 9c (2026-08-01): the banked curved-boolean completions — item 1's
## containment/pierce half lands; items 2–6 return executed blockers

Binding spec: `docs/M5-PR9C-SPEC.md` (six banked lanes). Branch
`ev/m5-pr9c-completions` from #151/#152's merged main (2c17686).

**Landed.** The SPHERE containment/pierce doors of
`topo::boolean::point_in_solid` (spec item 1's second half). `face_geo`
resolved `{Plane, Cylinder}` only; a sphere operand could not be
classified at all, which is the door the cylinder×sphere arm stands on.
The arm covers the CLOSED sphere class and says so structurally: the
faces sharing one sphere surface must close on each other (every
boundary edge's mate lands on a face of the same surface), which makes
their union the whole chart and removes the need for a per-face chart
trim. That is exactly what the M5 inventory mints — the `ball`
acceptance's V2 E2 F2, two half-bands on ONE sphere key joined along
the seam meridian and its angle-π copy — and it is why a per-FACE
fullness test (the first design, "every boundary edge is this face's
own `Seam`") was wrong: no M5 body has such a face. Arms act only for
the group's representative face, so one sphere folds exactly one
crossing pair per ray; a per-face arm would fold the same root twice
and tie itself into a permanent graze.

Predicates added (K funnel, metres): `bool_ray_sphere_disc` — the
ray/sphere discriminant metered as a length (`disc / 2r`; `√disc` is
the half-chord). Zero ⇒ tangent ray ⇒ graze/retry; Negative ⇒ definite
miss. The outward sign at each root needs NO second predicate:
`d·(p − c)/r = (w·d + t)/r = ±√disc/r`, so the near root enters and the
far root exits, read off the discriminant already decided definite. The
boundary pre-pass reuses `bool_point_in_solid_plane` on the linearized
radial residual `(|q − c|² − r²)/2r`, the same metre-valued form the
cylinder arm and the certification layer classify. Both squares go
through `powi(2)` (interval-square-poison rule — these straddle zero on
a probe near the wall).

`PointInSolidError::PartialSphereFace` is the new typed refusal, and it
is RE-PINNED as its own construction row (the S9 flip pattern): a
partial revolve caps the sphere band with planar fan walls, the group
stops closing, and the door refuses rather than guessing where the trim
runs. Its message carries the two-tolerance shape for a DEFINITE arm —
it states that the arm is structural (an exact-`f64` scan of arena keys
and mate adjacency, C6), has no in-band twin, does not move with ε, and
names the recourse.

**Deviations (numbered, with the executed blocker).**

1. **Item 1's fitted-chord join lane is NOT landed.** Only the
   containment/pierce doors are. The germ-pair join dispatch
   (`boolean/join.rs:402`) wires exactly one arm, `(Plane, Cylinder)`,
   through `JoinLane::BoolPlanar` with an azimuth window from
   `face_azimuth_window`. A cyl×sphere arm needs its own window analog,
   and the blocker under it is deviation 2: fitted carriers have no
   closed-form chart image, so `chart_pcurve` — which
   `run_azimuth_window` calls per run edge — refuses them, and there is
   no stored pcurve to read instead until `Pcurve::Fitted` exists.

2. **Item 2 (`Pcurve::Fitted`) is blocked on the SSI enclosure
   machinery being `f64`-only.** The storage half is small (the
   `Copy`-drop ripple is ~35 sites: `pcurve_cache.rs` 24, `topo/body.rs`
   7, `topo/pcurves.rs` 2) and the sampled-residual limb
   (`|S(P(tᵢ)) − C(tᵢ)|`) is already generic over `T`. What is not: the
   BETWEEN-SAMPLES envelope. `PcurveCache::recertify` must RE-DERIVE the
   whole certificate at rest — never trust the stored one — and for a
   fitted pcurve the only honest envelope is the C2.2 hull bound the
   spec names, which is computed by `ssi::enclose`/`ssi::certify`. Those
   are `f64`-only by type (`Box3` is built on `Point3<f64>`;
   `NurbsSurface::project` is `impl NurbsSurface<f64>`). Admitting
   `Pcurve::Fitted` therefore requires first lifting the SSI enclosure
   stack to `T: Real`, or accepting an f64-only certification lane that
   silently dies in the Interval lane. Neither is a PR 9c edit; sized as
   its own unit.

3. **Item 3 (curved revert) is blocked on there being NO representation
   for an orientation-reversed curved surface.** This is the deepest of
   the five and it is not a sizing miss — it is a contract gap. A face's
   outward normal IS its surface's chart normal (`topo::Face` carries no
   sense flag), and the per-kind statement is (SCOPED at the fix pass by
   the review's F1 — the first draft's blanket "always outward" was
   FALSE for the sphere):

   - **Cylinder, cone, torus**: the chart normal is ODD in the radius
     (`∂u × ∂v = r·radial(u)` for the cylinder, analogously for the
     other two), so it is OUTWARD for either sign — a negative radius
     moves the point to `radial(u+π)` and the normal with it. Negating
     `axis` flips `v_ref = axis × u_ref` too and merely reparameterizes
     `u ↦ −u`. Nothing to write.
   - **Sphere**: `∂u × ∂v = r²·cos v·n̂` is EVEN in the radius. The
     reviewer's executed probe: `Sphere { radius: -1.0, .. }` evaluates
     with an INWARD chart normal, while a cylinder at `r = −1` stays
     outward. So a negative-radius sphere IS a de facto reversed sphere.
     It is REJECTED as a representation rather than adopted: it violates
     the variant's ratified `radius > 0` convention, and every consumer
     that meters a sphere residual by `2r` reads the sign backwards —
     including this very unit's `point_in_solid` sphere arm, whose
     boundary residual `(|q−c|² − r²)/2r` and `bool_ray_sphere_disc`
     metering both divide by `2r` (a definite miss would read as a hit).

   The conclusion is unchanged: under the current contracts `revert` has
   nothing it may write on a cylinder, cone, sphere or torus. Three
   candidate designs, all
   ratification-scope: (a) a `sense` field on `topo::Face` — 82 `Face {`
   literals plus every consumer that asks "which way is out" (mesh,
   step-export where STEP's `advanced_face.same_sense` is the natural
   home, certify, boolean classification, props, validate); (b) a
   `Surface::Reversed(Arc<Surface>)` wrapper — 66 files match on
   `Surface::` and every exhaustive match breaks, and it is a D3 closed-
   enum change; (c) convert reverted curved surfaces to NURBS with
   reversed parameterization — loses analytic identity, forces a pcurve
   re-mint onto a different chart, and would hand PR 12's die pips a
   NURBS dimple face. A fourth option surfaced and was REJECTED by the
   review: (d) adopt the negative-radius sphere as the reversed form —
   it is the only reversal the enum can already express, and it is still
   wrong, for the convention and `2r`-metering reasons above. The
   `CurvedOpUnsupported` front door therefore STAYS, and PR 12's curved
   subtract stays gated. Recommendation: (a), discussed with Evan before
   any code.

4. **Item 4 (cyl×cyl equal-radius germs) not landed** — same blocker
   as deviation 1: the join dispatch's `(a_s, b_s)` fallthrough refuses
   typed and the equal-radius ellipse pair needs its own two-cylinder
   window story in `JoinLane`. `geom_brep::intersect`'s classification
   arm (`intersect.rs:241`, `:658`) is live; the TOPOLOGY side is the
   missing half.

5. **Item 5 (edge×NURBS-face sweep layer) not landed**, and item 6
   depends on it, so the shape-(iii) cut-loft row stays pinned refused.

6. **Item 6 (the loft/sweep body assembly) is blocked at tier-3 check
   7, the +V invariant.** The assembly design was executed to the point
   of the blocker and is recorded here so the next unit does not
   re-derive it. (i) The topology is extrude's, with different geometry:
   `LoftGeometry.walls[loop][segment]` are the wall surfaces, cap rims
   are section 0 / section k−1's segment curves, struts are the walls'
   `u = const` iso-curves. (ii) Cap-wall edges need NOTHING new: the
   wall's `v = 0` iso IS the placed sketch segment (degree elevation and
   knot refinement are exact), so the carrier stays `Curve3::Line`/
   `Circle` under `MappedCurve::PlacedSegment` and certifies today.
   (iii) Wall-wall seams are the genuinely new class; they cannot go
   through `Intersection` with a widened `resolve()` as the spec
   sketched, because `implicit_residual(Nurbs)` and
   `curvature_lever_arm(Nurbs)` are poison and a foot-point gradient is
   `f64`-only — so `classify_dihedral` cannot run. The workable shape is
   a new `EdgeGeometry::IsoCurve { surface, u, v0, v1 }` (Copy-
   preserving, resolves through the surface arena) whose residual is the
   genuinely metric `|carrier(t) − S(u, v0 + (v1−v0)t)|` at the CERT
   schedule, with adjacency read as `surface ∈ {fs_plus, fs_minus}`, and
   tier-3 check 4 exempt BY KIND for Nurbs-adjacent edges (the `Seam`
   exemption idiom; a definitional wall junction's contact class is the
   profile's declared corner structure, Q8/C11, not a derived one). (iv)
   What none of that fixes: `validate_geometric` check 7 calls
   `mass_properties`, which routes a Nurbs face to
   `geom_brep::props::curved_face` → `PropsError::Unimplemented` →
   `ValidationError::VolumeUncomputable`. NURBS-patch flux needs
   surface quadrature (PR 11), and the AREA half has no closed form at
   all for a rational patch. So a loft body cannot be tier-3 valid in
   this build no matter how well the assembly is written, and shipping
   the assembly without it would replace one honest frontier with a body
   that fails validation — strictly worse than PR 10's pinned refusal.
   `wire_loft`'s `CurvedSolidFrontier` therefore STAYS, and shape (iii)'s
   loft-body row stays pinned.

**Spec-vs-code correction.** The spec says the tier-3 Nurbs kind
refusal is "duplicated in `tier3_local_checks` AND
`tier3_local_checks_marked` — flip BOTH". There is only ONE copy:
`tier3_local_checks` (validate.rs:1512) delegates to
`tier3_local_checks_marked` (:1546), which holds the single check-1
loop. Nothing to flip twice.

**Battery.** Touched crates `topo` + `sweep`, default ε: 42 + 39 green
result lines, 0 failures. Interval lane (`topo/interval`,
`sweep/interval`): 81 green result lines, 0 failures. `cargo fmt --all
--check` clean; `clippy --all-targets` clean on both crates in both
feature sets. Interval-square tripwire self-check on the diff: no
`x * x` on a generic scalar (both new squares are `powi(2)`).

**Message hygiene (the PR 10 fix-pass rule applied to ourselves).**
Three in-code frontiers said "banked as M5 PR 9c". PR 9c has now run,
so leaving them would be a promise describing a main that no longer
exists. All three were rewritten to the executed finding and PINNED by
acceptance rows: `RevertError::UnsupportedSurface` now carries the
"not merely unimplemented" PROOF (the chart normal of every
axisymmetric variant is always outward, and neither an axis flip nor a
negative radius moves it); `BooleanError::CurvedOpUnsupported` quotes
that finding so a caller who never touches `revert` still learns why
curved subtract is gated; `CurvedBooleanUnsupported` names what PR 9c
did land (the sphere half of the containment/pierce door) and what it
did not (the fitted-chord join lane, behind `Pcurve::Fitted`);
`LOFT_FRONTIER` names the +V/quadrature door behind the description
doors it used to stop at. The die-pips shape (a sphere bitten out of a
slab) is exercised as a SMOKE row in `m5_pr9c_sphere_doors.rs` and
pinned at its typed front-door refusal — the honest form of "ahead of
PR 12" when the op itself is gated.


### PR 9c fix pass (2026-08-01)

Review verdict: APPROVE-WITH-FIX-PASS. The sphere group-arm design was
verified sound (the clopen-coverage argument holds for any face count),
the double-fold claim was EXECUTED-CONFIRMED (a scratch de-guarded
variant dies `RayExhausted`), the `disc / 2r` metering was D4-endorsed,
and the boundary pre-pass was checked correct at both the seam and the
poles. All five blocker proofs confirmed. Zero landed-code MAJORs
outside the proof TEXT.

**F1 (MAJOR, proof text).** The outward-normal proof's negative-radius
leg is false for the sphere — `∂u × ∂v = r²·cos v·n̂` is EVEN in `r`.
Every pinned copy (revert.rs docs + `Display`, `boolean/mod.rs`
`CurvedOpUnsupported` docs + `Display`, deviation 3 above) is now scoped
per kind, and option (d) — adopting the negative-radius sphere as the
reversed form — is recorded as REJECTED with its two reasons. The
acceptance row was flipped from asserting `"always outward"` to
asserting the corrected per-kind text AND the ABSENCE of the overclaim,
so the first draft's statement cannot come back.

**F2 (MINOR).** `boolean/reduce.rs`'s NURBS-wall pre-refusal still said
the crossing layer was "banked as M5 PR 9c". Rewritten to the executed
finding: PR 9c was that unit and did NOT land it, because the residual
sides a crossing layer needs (`implicit_residual`, `classify_dihedral`)
are poison on a NURBS surface and the only non-poison substitute is a
foot-point projection that exists at `f64` only.

**F3 (MINOR).** The sphere arm's comment claimed it metered `disc` by
`2r` "exactly as the cylinder arm meters its own"; the cylinder arm
divides by `(2r)²`, which is dimensionless. The comment now states that
the sphere's length-dimensioned form is the D4-honest one and FLAGS the
cylinder arm for normalization by a unit that can re-pin its margins —
deliberately not changed here, since the PR 9 acceptance rows pin them.

**F4 (NOTEs, both taken).** A tangent-schedule-ray row (zero
discriminant ⇒ graze ⇒ the retry schedule answers, never a parity
guess) and a two-ball MULTI-SHELL row. The latter builds its body with
the live curved UNION of two disjoint balls, so it pins the group
rule across SURFACES *and* pins the new arm driving the boolean's own
no-intersection containment fallback — not just a direct query.

## S10 (2026-08-02): face orientation sense — the ratified fix for PR 9c
## deviation 3 (`Face::sense`), the consumer audit, and one live defect

Evan ruled option **(a)** — a `sense: bool` on `topo::Face` — over
(b) `Surface::Reversed`, (c) NURBS conversion, and (d) negative-radius
spheres. The ruling was the sign-off for the DESIGN.md D1 amendment,
which is in this PR: a face's outward normal is `sense_sign · n_chart`,
orientation reversal is exact structure (never a decide), the bit IS
STEP's `advanced_face.same_sense`, and persistence is untouched
(bodies re-derive; `serde` appears in exactly one crate manifest,
`editor-core`, and the save is the recipe).

**Scope discipline.** S10 is the contract plus the consumer audit.
Wiring `revert` to flip the bit is the follow-on unit, so `revert` and
the boolean front door keep their typed refusals — with messages
rewritten from "unrepresentable" to "the representation gap is closed,
the WIRING is not".

**The audit's governing distinction** (this is the reviewable claim):
orientation is now stored in TWO places, and they must not both be
applied. *Chart reads* — a site that takes a surface's chart normal and
calls it the face's outward normal — get `× sense_sign`; the chart is
the only encoding there. *Winding-derived* sites — loop vector areas,
Newell/shoelace sums over stored traversal, emitted triangle order —
already carry the orientation, because `revert` reverses loops AND
flips `sense` in the same step; multiplying those by the bit would
negate the volume twice. The bit enters a winding-derived layer at
exactly one kind of site: where there is no winding to derive from (the
rimless sphere band's hardcoded `s_f = +1`, the tessellator's
"assumes outward-oriented shells"). The AGREEMENT of the two encodings
is a tier-3 obligation — the validator's loop-role winding check is now
the S10 gate, and a body whose bit disagrees with its winding is
inside-out and refused.

**Deviation 1 (MAJOR, returned, not fixed).** The spec's premise —
"at M5 every constructor mints material-agrees-with-chart faces" — is
FALSE, and was false before S10. `extrude` mints a cylinder wall per
arc segment, and a cylinder's chart normal is unconditionally the
radially-outward radial; for a **concave** arc the material lies
OUTSIDE that cylinder, so the face's true sense is `false` while this
build stamps `true`. Executed consequence: `point_in_solid`'s cylinder
door reads the chart-outward radial as outward, so on the
`review_m2_pr4` mixed-turn-arc fixture it reports `In` throughout the
notch the concave arc cuts (true boundary at `x = 1` is
`y = 2.5 − √2 ≈ 1.086`; the door does not turn over until `y ≈ 1.5`).
Both halves are pinned as `finding_concave_arc_wall_sense_is_wrong_today`
in `crates/sweep/tests/m5_s10_face_sense.rs`. This is NOT improvised
away here: fixing it means the sweep constructors must mint
`sense: false` on concave arc walls, which changes behaviour across the
boolean layer and is its own unit. It is a **required predecessor of
the revert-wiring unit** — reverting a body whose senses are already
wrong flips a lie into another lie.

**Deviation 2 (minor, count).** The spec estimated ~82 `Face { … }`
literals; the actual population is 24 (the looser grep that produced 82
also matches `MassPropsError::Face`, `EulerOpError::NotSameFace`, and
the other error variants named `*Face`).

## S11 (2026-08-02): concave/inward walls mint `sense: false` — the
## S10 deviation-1 fix; the pellet-swallow union dies here

Merge-priority unit (S9/du_of_rims precedent): until this PR, main
misreported containment on concave notches AND revolve bores, and the
public `union` silently swallowed a disjoint pellet placed in a notch
(volume 3.000 for 3.008, one shell for two, no refusal). Required
predecessor of the revert wiring.

**The criterion, exact structure only.** The profile's canonical
winding is material-left (outers CCW, holes CW), so a wall's material
side is the left of its canonical traversal. Extrude: an arc's carrier
center lies left iff its stored turn is `Positive`, and the cylinder
chart normal is unconditionally the outward radial ⇒ **wall sense =
(canonical turn == Positive)**; concavity is a property of the 2-D
region against the carrier circle, so the swept reversal never enters.
Revolve, derived in the (r, z) frame (orientation-preserving, r ≥ 0):
sphere/torus walls take the same turn-sign rule; cylinder AND cone
walls take **sense = (canonical Δz > 0)** (the cone's nappe dependence
algebraically collapses to the axial sign); plane annuli (chart normal
fixed at `+a₃`) take **sense = (canonical Δr < 0)**. All signs come
from the named `axis_line_radial`/`axis_line_axial` decide funnel —
the cylinder arm now decides `axis_line_axial` too. Attachment is the
new constructor-facing door `Body::set_face_sense` (the mint cannot
know the material side; the mirror of `set_face_surface`).

**Widened scope (deviation).** The audit found revolve's line walls
carried the same defect class: bore cylinders, inward cones, and the
UNDER-side plane annulus. The annulus claim is **at-rest class
membership** — its chart normal (`+a₃`) points into material — not an
executed door witness: the containment ray schedule never decides
through that face (review probes on main read `Out` from below), and
the executed misreports on main were the notch, the hole plate, and
the bore. Note the annulus is also invisible to tier-3 check 6, whose
loop-role gate is line-bounded-planar only: an arc-bounded reversed
planar face passes at rest, so the constructor criterion is the ONLY
guard there.

**Rows.** The two S10 `finding_*` rows flipped to construction rows
(door reads `Out` through the notch; `union` keeps the pellet: 3.008,
two shells). New per-wall-kind acceptance in
`crates/sweep/tests/m5_s11_concave_sense.rs` (+ interval twin), the
watertight/convergence rows in `crates/mesh/tests/`, and the first
real `.F.` emission + typed-refusal rows in `crates/step-export/tests/
m5_s11_same_sense.rs` (the planar-only writer cannot yet carry a
curved reversed wall — the spec'd same_sense e2e lands with the
exporter's curved arms). Pre-existing, sense-independent door
limitations pinned typed: full-period wall trims
(`bool_wall_trim_period`) and rimmed sphere bands
(`PartialSphereFace`).

**Banked hazard (review MIN-1).** Boolean splitting's `mef` re-mints
stamp `sense: true`, so splitting a reversed face would silently reset
the bit; unreachable today (curved lanes refuse typed first) and
pinned by the adopted guard
`review_s11_adv::adv_touching_union_with_reversed_faces_refuses_typed`.
The parent-sense inheritance fix MUST land with the unit that makes
those splits reachable. Recorded at `Body::set_face_sense` and the
`splitting/join.rs` mint site.

**Review.** Blinded adversarial review: APPROVE, 0 MAJOR / 2 MINOR /
3 NOTE (rubric 5/5/4); the criterion survived all six adversarial
constructions unmodified (mixed hole arcs, fillet-cornered eye slot as
outer and hole, per-carrier downward invariance, reversed authoring,
bore-groove torus, touching-union guard — adopted verbatim); merge-base
reproductions confirmed for bore, hole plate, and pellet.

## S12 (2026-08-02): curved `revert` is wired — the sense flip, the
## split-fragment inheritance, and curved ∖/∩ live on the cylinder class

The unit S10 and S11 were predecessors of. S10 ratified `Face::sense`
and threaded the consumers; S11 made the constructors write honest bits;
S12 makes `revert` FLIP them, makes splitting's re-mints INHERIT them,
and narrows the wholesale curved subtract/intersect front door.

**(a) The revert arm, and the design call inside it.** Two encodings
exist for "this face's outward normal is negated": negate the `Plane`'s
stored normal (M3's arm), or flip the face's `sense` bit (S10's). S12
keeps BOTH and makes them **exclusive by surface kind** — planes take
the first, every other class takes the second — so each face is flipped
exactly once. The alternative (flip every face's bit, touch no surface)
is the more uniform statement and is what D1's amendment reads most
naturally as; it was rejected on the RISK that a reverted planar operand
would reach `merge_coplanar_faces`, `plane_eq` and the sector tables with
unchanged normals and `sense: false` for the first time, where any
consumer S10's audit missed would silently read the wrong side.
**Corrected at the fix pass (review MIN-1): that risk did not
materialize.** The reviewer IMPLEMENTED the uniform flip and ran the
pinned planar lanes — 310/310 topo lib rows plus every M3
boolean/surgery suite, the A∖B ≡ A∩revert(B) oracle included — ALL
GREEN. So the split is **chosen for bit-for-bit planar conservatism**
(no pin was willing to move without a design conversation first), not
forced by moving pins; the evidence says both encodings work, and the
one that leaves M3's planar behaviour byte-identical is the one a revert
unit is entitled to ship. The chosen split confines the new encoding to
the faces that never had one. `RevertError::UnsupportedSurface` is RETIRED — the flip is uniform
across cylinder/cone/sphere/torus/NURBS, so no per-class residue is left
inside `revert` — and its parity record (PR 9c's ODD/EVEN-in-the-radius
finding, F1-scoped) is kept as prose on the enum rather than as an
unreachable variant.

**(b) Split-fragment sense inheritance** (the S11 banked hazard, fixed
in the same PR that makes the splits reachable, as S11 required).
`mef`'s `mint_loop_and_face` and `mfkrh` now take the new face's bit
from the OLD FACE when the new face lands on the old face's surface KEY,
and keep minting `true` otherwise (a `New`/foreign `Shared` surface is a
different region — the caller attaches the honest bit, as the sweep
constructors do). Exact structure: key equality, no numeric compare, and
`mef` gains no material-side knowledge it does not have. Executed as
load-bearing: with the inheritance disabled, the mixed-sense split row's
∩ arm comes back with every wall fragment `sense: true`.

**(c) The front door, narrowed per class (C12.1), not retired
wholesale.** ∖ and ∩ are open on operands whose faces are `Plane` or
`Cylinder`; `Sphere`/`Cone`/`Torus`/NURBS still refuse
`CurvedOpUnsupported`, with the message rewritten from "revert has no
representation" to the blocker that actually remains — the germ-pair
join dispatch wires exactly one arm, `(Plane, Cylinder)` (PR 9c
deviation 1), behind which sit `Pcurve::Fitted` (deviation 2) and the
edge×NURBS crossing layer (deviation 5).

**What went live, with volumes** (all tier-3, both sweep strategies
bit-identical): blind hole `plate(3×3×0.8) ∖ boss(r 0.35, z 0.3→1.3)` =
7.00757744996763 = 7.2 − πr²/2, on the 3-arc AND 2-arc authorings;
through hole = 6.89212391994820; the complement `boss ∖ plate` = two
shells, 0.15393804002590; `plate ∩ boss` = 0.19242255003237 with
`V(A∖B) + V(A∩B) = V(A)`; and the mixed-sense trio on the S11 notched
plate (∪ 9.56438055098077, ∖ 7.96438055098077, ∩ 0.64292036732051),
each exact against the closed form, each keeping a `sense: false`
fragment of the split concave wall.

**Deviation 1 (numbered, executed, NOT fixed here) — the containment
fallback is vertex-probed, so it is unsound for curved boundaries, and
∪ is wrong today because of it.** A unit ball half-buried in a 4×4×1
slab pokes out of both faces, but its only two vertices are the revolve
poles, which sit inside; no crossings are found for the sphere class,
the pipeline falls through to per-shell vertex-in-solid containment, and
the ball is classified as wholly contained. `union` therefore meters
16.0 where the truth is 17.30899693899575. **Reproduced on the merge
base** (3ef715e) by the same call, so it predates S12 and no part of
this unit touches that path (∪ was never behind the curved door; the
ball's bits are `sense: true` in both builds). S12's response is the one
this unit is entitled to: refuse the class up front for the two ops it
is OPENING, rather than let them inherit a silent wrong answer, and pin
the ∪ defect as `finding_sphere_class_containment_fallback_is_wrong_today`
(asserting the WRONG value on purpose, so the fix fails it loudly).
Re-cutting the containment fallback — the honest fix is a curved-extent
test, not a vertex probe — is its own unit and is what the die-pips
class waits on together with the join lane.

**Deviation 2 (scope, minor).** The unit's acceptance was specified for
`crates/topo/tests/m5_s12_curved_revert.rs`; it lives in
`crates/sweep/tests/m5_s12_curved_ops.rs` instead, because `topo` has no
dev-dependency on `sweep` and therefore cannot build a body with an
analytic surface at all. The topo-side row is the in-module construction
row on the `Nurbs`-surfaced `ops_cube` (the exact fixture that used to
refuse).

**Rows flipped from refusal pins to construction rows** (the S9
pattern): `revert::revert_refuses_non_plane_surfaces` →
`revert_flips_sense_on_non_plane_faces_instead_of_refusing`;
`review_m3_pr1_sweep::revert_curved_body_refuses_typed` →
`revert_curved_body_reverts_via_the_sense_bit` (which keeps the whole M3
contract — operand untouched, tier-2 valid, tier-3 exactly
`NegativeVolume`, bit-negated volume, bitwise involution and
determinism — on a curved body);
`m5_pr9c_sphere_doors::curved_revert_refusal_states_the_wiring_blocker`
→ `curved_revert_reverts_the_ball_instead_of_refusing` (the sphere chart
is untouched and its `radius > 0` survives, which is what the parity
finding demanded);
`review_m5_pr9_boss_probe::my_boss_subtract_makes_a_blind_hole_honestly`
→ the same name, now an audited construction row (volume, tier 3,
intrinsic seam arcs, pcurve coverage) plus the ∩ twin and additivity.
`m5_pr9c_sphere_doors::curved_subtract_front_door_quotes_the_same_finding`
→ `the_die_pips_shape_still_refuses_at_the_narrowed_per_class_door`: the
die-pips row STAYS a refusal pin, honestly, at a door that now names the
join lane instead of `revert`.

**The S11 guard row's fate.** `review_s11_adv::adv_touching_union_with_
reversed_faces_refuses_typed` still REFUSES (its washer/box pair takes
the annulus-touching door, which S12 does not open), so it stays green
as a refusal — but its panic-on-answer is stale now that inheritance is
implemented, so the row was re-aimed: an answer is AUDITED against the
additive closed form instead of rejected. The mixed-sense split S12
genuinely made reachable is pinned with exact volumes in the S12 suite.

**Battery.** Touched crates `topo` + `sweep`, default ε; new rows also
at the Interval band. `cargo fmt --all --check` clean; `clippy
--all-targets` clean on both crates. Interval-square tripwire on the
diff: no squares added at all (`powi(2)` rule vacuous here — the unit
adds a `bool` negation, a key equality and an arena scan, and decides
nothing numerically).

## PR 13 (2026-08-02): the curved STEP subset — conics and elementary
## surfaces as EXACT native AP214 entities, and the corpus that proves
## an outside reader reconstructs them

Plan line 13. The writer's two closed matches grew from
`Plane`/`Line` to the whole kernel geometry vocabulary. **Every arm is
a native entity, and every arm is exact**, which is the entire reason
this writer is in-house:

| kernel | AP214 | exactness |
|---|---|---|
| `Plane` | `PLANE` | identity (M4) |
| `Cylinder` | `CYLINDRICAL_SURFACE` | identity |
| `Cone` | `CONICAL_SURFACE`, apex placement + `radius = 0` | identity as a LOCUS; `v` differs by cos α |
| `Sphere` | `SPHERICAL_SURFACE` | identity |
| `Torus` | `TOROIDAL_SURFACE` | identity |
| `Line` | `LINE` | identity (M4) |
| `Circle` | `CIRCLE` | identity |
| `Ellipse` | `ELLIPSE` | identity |
| `Nurbs` (curve) | `B_SPLINE_CURVE_WITH_KNOTS`, rational complex instance when weighted | structure for structure |

"Identity" is literal and tested: each kernel frame `(origin, axis,
u_ref)` is ISO 10303-42's `axis2_placement_3d` field for field, so the
two PARAMETERIZATIONS agree, not merely the point sets. The acceptance
suite compares emitted reals with the body's stored reals using `==`
(the float printer round-trips to identical bits), so a renormalized
axis, a rotated seam, or a cone placement offset down the axis — all
of which import perfectly — fail.

**The conic question the spec asked, answered: native, never the
rational-quadratic form.** CURVED-DESIGN keeps the NURBS Book §7.3–7.4
form (shape factor `k = w₀w₂/w₁²`) as the declared export/tessellation
form for conics, and it IS exact. But AP214 has `CIRCLE` and `ELLIPSE`,
so taking that road would be an equally exact and strictly worse
encoding: it discards the axes and centre every reader consumes,
reparameterizes for nothing, and trades five reals for a
control/weight/knot triple. The infinite-control-point machinery for
arcs ≥ 180° (§7.4) never comes up. Both kernel conic rungs are native.
There is no curve kind in the kernel that NEEDS the rational form.

**Two deviations from the spec, numbered.**

1. **`B_SPLINE_SURFACE_WITH_KNOTS` is not implemented; `Surface::Nurbs`
   still refuses typed.** The spec lists it under writer growth but
   also puts "NURBS FACES at rest" out of scope and hands them to the
   loft-assembly unit. No body at rest carries a NURBS face, so the arm
   would have been an untested code path guarded by nothing. The
   refusal message now distinguishes the mvfs "no description yet"
   placeholder from a described NURBS surface and names the unit that
   brings the entity. The CURVE arm *is* implemented (the entity is
   part of the named subset and its record text is pinned in
   `writer.rs`'s unit tests, rational and non-rational, including the
   exact-equality knot run-length encoding) even though no at-rest body
   carries a NURBS carrier either — the difference is that a rung-3
   SSI carrier is a thing the kernel already MINTS, just not through a
   public constructor.
2. **The outward/void classifier did not grow curved closed forms.** It
   is now NARROWER than the emitter: a MULTI-shell solid carrying
   curved geometry refuses, even though every one of its faces has a
   printer. Its divergence-theorem reduction (`p·n̂` constant over a
   face) is a planarity identity with no closed-form curved
   counterpart, and its output is a material-vs-void SIGN — the one
   place an approximation is a silent lie rather than a roundoff. The
   refusal got its own variant, `CurvedShellClassification`, whose
   message says the emitter is fine and the classifier is not; the old
   behaviour would have reported `UnsupportedSurface`, which is now
   false. Only S12's two-stub `boss ∖ plate` complement is affected —
   every other curved body at rest is single-shell.

**The corpus, and what FreeCAD said.** Seven fixtures joined the three
planar ones, chosen so every new arm has a body behind it:
`cut_cylinder` (the only `Ellipse`), `boss_union` (curved boolean),
`notched` (the S11 concave wall), `washer` (genus 1, two `.F.` faces,
full-2π seam encoding), `ball`, `cone`, `donut`. All ten import into
FreeCAD 1.1.2 as valid solids. **The volumes are not approximately
right, they are exactly right**: every one agrees with the closed-form
analytic value to ≤ 4e-15 relative — because the surfaces crossed the
wire as surfaces, not as facets. OCC keeps the kernel's topology
exactly on five of the seven; on `ball` and `cone` it ADDS degenerate
pole/apex edges (2→6 and 6→8) that its own face model requires, with
faces, vertices and volume unchanged. The `.expect` sidecars carry the
analytic volume and say which counts are OCC normalisation. CI's
`step-import` job globs the fixture directory, so all seven are hosted
with no workflow edit.

**The S11 row flipped, as its doc comment instructed.**
`m5_s11_same_sense`'s row 2 was written as a typed-refusal pin with an
explicit instruction for what to become once the curved arms landed.
The notched body now exports with exactly one `.F.` `ADVANCED_FACE` —
and the row resolves that face's surface reference and demands a
`CYLINDRICAL_SURFACE`, of which the body has TWO (one convex, one
concave), so flipping the wrong wall or both fails.

**Orientation: the composition rule, and FreeCAD's measured
blindness.** With `.F.` faces finally reaching the emitter, the S10
review's rule became testable on real output. Two pins aim straight at
the double-composition bug: every bound orientation is `.T.` on every
fixture including the reversed ones, and every `.F.` face's surface
axis equals the body's stored CHART normal bitwise (a writer that
negated the axis instead — the other half of the same bug — fails).

The external oracle cannot arbitrate this. `revert(ball)` and
`revert(washer)` — genuinely inside-out solids, every face `.F.` —
import as `valid: True` with the SAME positive volumes as the
un-reverted bodies. OCC's ShapeHealing rectifies silently, exactly as
M4's review found on `cube.step`. The plan anticipated the fallback
("else pin the emitted text"), so the text is pinned, and the
orientation oracle gained a **curved-agnostic** companion to the
planar signed-volume walk: **edge-use coherence** — in a coherently
oriented closed shell every edge is traversed once in each direction,
counting the whole shell's loops. That is the same boundary-winding
datum the volume oracle reads, stated locally, and it needs no
planarity. It is pinned on all ten fixtures with per-shell edge counts,
and it has three controls: a double-composed face (INCOHERENT — this is
the row FreeCAD would otherwise have owned), one inverted curved face
(INCOHERENT), and a uniformly reversed shell (COHERENT — said out loud
so the oracle's scope is not overclaimed; that case belongs to the
volume oracles).

**ε.** Exports are exact structure. The only ε-dependent byte in the
whole document is the `UNCERTAINTY_MEASURE_WITH_UNIT` value, pinned by
a row that exports the washer at two tolerances a thousand-fold apart
and asserts the texts differ in exactly one line. Every other row
compares emitted floats to the body's own stored floats and never reads
a distance. The one new refusal arm is reached by a type-level match
before any arithmetic, and is run at two tolerances to check that
rather than assert it; the ambient axis is CI's `CAD_EPS` matrix.

**The demo tour.** All 26 bodies now export STEP (nine of them curved,
six carrying `.F.` faces); the narrated curved refusals are gone and
`step_expected` is true everywhere, so a refusal anywhere in the tour
is now a hard failure. All nine curved exports import into FreeCAD as
valid single solids, and their OCC volumes agree with the kernel's own
STL tessellation to within faceting error (3e-4 to 1.4e-2 relative,
signed the way inscribed/circumscribed faceting predicts) — an
independent end-to-end check that OCC reconstructed OUR solid rather
than a healed neighbour.

**One walk-order trap, banked.** The suites that match emitted records
against kernel entities must walk the WRITER's traversal, not
`Body::faces()`/`Body::edges()`. Arena order coincides with the walk on
simple extrusions and diverges on boolean results, so a helper using
arena order silently compares the wrong pairs on precisely the most
interesting bodies — it was caught only because `boss_union` failed
while every swept primitive passed. `common::walk_order` mirrors the
documented traversal and says this in its doc comment.

**Battery.** `step-export` at default ε: lib 8, `export` 14,
`m5_pr13_curved` 11, `m5_s11_same_sense` 2, `orientation_oracle` 6 —
41 rows, all green. Planar goldens byte-UNCHANGED (no planar-only body
at rest mints a reversed face, so the S10/S11 wiring costs the M4
fixtures nothing). `scripts/check_step.sh` green on all ten fixtures
locally with `REQUIRE_FREECAD=1`. Demo tour built and run end to end.
`cargo fmt --all` clean; `clippy --all-targets` clean.

### PR 13 fix pass (2026-08-02)

Review: APPROVE, 0 major / 2 minor / 3 notes; every attack absorbed.
Two of the reviewer's own findings are worth banking beyond the items.
First, FreeCAD was proven blind even to **double composition** (not
just to a uniform inversion), independently reproduced — so the
edge-use-coherence oracle is the only working guard on that axis, which
raises the stakes on keeping its negative controls. Second, the
rational complex-instance record was validated end-to-end by splicing
it into a wireframe and importing it, which is a check this PR did not
have and now does.

**F1 (MIN-1).** `m5_pr13_curved.rs`'s NURBS-frontier row contained a
loop that iterated the edges and did nothing. It now asserts the
kernel-level fact the text-level `B_SPLINE` grep only implies: every
carrier of every corpus body is line/circle/ellipse and every surface
is non-`Nurbs`, so the claim is "the bodies do not have them", not
"the text does not mention them".

**F2 (MIN-2).** The DESCRIBED-NURBS-surface refusal message was dead by
construction (no body at rest carries a NURBS face) and therefore
untested. `NurbsSurface::new` is public and validating, so a described
bilinear patch is constructible in a unit test even though it cannot be
attached to a face by any public constructor: the two `Surface::Nurbs`
states are now pinned to their two different messages, and to being
different from each other.

**F3 (NOTE-1).** The orientation-oracle header said the cone's reversed
faces were "the cone's two faces", implying the conical bands. Wrong:
both `CONICAL_SURFACE` faces write `.T.`, and the two `.F.` faces are
the PLANAR base-disc halves. Corrected, with the general shape of S11's
rule stated — the reversed face is whichever one has material against
its chart normal, and on a revolved solid the under-side cap is such a
face even though it is flat. (`lib.rs` already had it right.)

**F4, three reviewer probes adopted.**

- **The wireframe splice** (`tests/fixtures/nurbs_wireframe.step` +
  `.probe.py`). A `GEOMETRIC_CURVE_SET` document carrying the writer's
  `RATIONAL_B_SPLINE_CURVE` complex instance verbatim — the Eq. 7.33
  exact quarter circle, weights (1, 1/√2, 1). It is the arm's ONLY
  reader-level validation before the loft-assembly unit, since no body
  at rest produces the record. OCC returns a RATIONAL degree-2 B-spline
  with **bit-identical weights**, and every one of 1001 sampled points
  sits on the unit circle to **3.4e-16 relative** (arc length π/2 to
  1 ulp) — better than the reviewer's 2.3e-13 measurement, and four
  orders tighter than any non-rational approximation of a conic could
  reach, which is what makes the check discriminating. A reader that
  parsed the record but dropped the weights would trace the control
  polygon's parabola and miss by ~8%.
  `check_step.sh` grew a **generic** hook for this: any
  `<fixture>.probe.py` beside a `.step` runs under the same
  interpreter with `$STEP_FILE` set, for geometric facts the generic
  count/volume checks cannot state. A Rust row asserts the spliced
  record is byte-identical to `writer.rs`'s pin, so emitter and fixture
  cannot drift apart.
- **A3, the same_sense-only corruption**, adopted as a documented
  KNOWN BLIND SPOT rather than a fix. Flipping only the
  `ADVANCED_FACE` flags leaves every winding untouched, so edge-use
  coherence reads green — correctly, since it is a statement about
  traversal directions. FreeCAD is blind to it too, measured: the
  corrupted `notched.step` imports `valid: True` with 6/12/8 and
  volume 3000000000.0 mm³, every figure identical to the honest file.
  So this lie has no external witness at all, and the only guard is
  the kernel-side identity `same_sense == Face::sense` asserted PER
  FACE in `m5_pr13_curved.rs` — which is why that row compares faces
  rather than counting `.F.`s. Recorded so the gap is stated instead
  of discovered.
- **The arena-vs-walk divergence probe**, adopted as a `walk_order`
  regression guard. It pins both halves of the trap: the two orders
  DIVERGE on `boss_union` (a boolean result) and AGREE on `washer` (a
  swept primitive), same multiset either way. The agreement half is the
  point — a suite built only on primitives passes with the bug in
  place.

**Battery after the fix pass.** `step-export` default ε: lib 9,
`export` 14, `m5_pr13_curved` 13, `m5_s11_same_sense` 2,
`orientation_oracle` 7 = **45 rows**, all green. `check_step.sh` 11/11
(the ten corpus fixtures plus the wireframe, whose geometric probe runs
inside the same job). `fmt --all --check` and `clippy --all-targets`
clean.
