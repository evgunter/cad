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
