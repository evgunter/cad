# SMELL-SCAN — a structural audit of things that look off (2026-08-18)

**Status: REPORT ONLY. Nothing here is ratified, nothing is a
commitment, no code was changed, and no finding proposes a specific
fix.** This is a survey, requested by Evan, of parts of the kernel that
play almost-but-not-quite parallel roles, code that feels overly complex
or indirect, and things that simply do not look like the way you would
do it. It is deliberately *not* a bug hunt (`/code-review` and the
review lanes cover that) and deliberately *not* a performance audit
(`docs/PERF-SCAN-2026-08.md` covers that).

**Scan base: `4258584` (main, 2026-08-18).** Line numbers are as of that
commit. Claims, not line numbers, are the content.

**Method.** Twenty parallel domain scans, one per scope, partitioned to
cover every `crates/*/src` file in the workspace. Each scan was
instructed to:

- anchor every claim at `file:line`;
- treat a long justifying comment as *mild evidence for* a smell rather
  than against it, since this codebase rationalises heavily in prose;
- grep repo-wide whenever it suspected a cross-crate parallel, because
  cross-crate near-duplication was the highest-value target;
- **not** propose fixes, and
- report uncertain findings rather than drop them.

Each scan carried its own confidence label (`sure` / `likely` /
`unsure`); those labels are preserved below. The coordinator deduped
across scans and ranked. **Findings that multiple independent scans hit
from different files are ranked highest** — that independence is the
strongest signal in the report, and it is noted where it applies.

**How to use this document.** Every finding has a stable ID (`S1`…`S48`).
Each carries a `**Verdict:**` line. Annotate in place; the IDs are stable
across edits so they can be cited in PRs, issues, and follow-up specs.
**Nothing here should be acted on before it has a verdict.**

Verdict vocabulary, as used below:

- **ACCEPTED** — Evan agrees the finding is real. Not yet a decision
  about what to do.
- **ACCEPTED IN PART** — some of the finding holds; the row says which.
- **ACCEPTED, SORT REQUIRED** — real, but the finding lumps together
  machinery that supports *planned* work with machinery that is
  *superseded*, and the two need separating before anything moves.
- **DISPUTED — REFRAME PROPOSED** — the finding over-reaches, and Evan
  has offered a different framing of the underlying question.

## Review status (2026-08-18)

**Tier 1 (S1–S15): verdicts recorded.** Tier 2 and Tier 3 are unreviewed
— their `**Verdict:**` lines are still blank.

**A steelman pass is in flight over all of Tier 1.** Evan's standing
instruction: *"for all of these we'd probably want an agent to steelman
the original / look for the original justification and be sure we're not
missing any basis for decision."* Eleven agents are reading git history,
PR descriptions (which is where this repo documents the logic of a
change — see `CLAUDE.md`), the milestone specs and logs, and `memories/`,
to answer for each finding: what was the original basis, what is the
strongest honest case for the current design, does the finding survive,
and what would acting on it cost. Their results will be recorded here as
a `**Steelman:**` line under each finding.

Until that lands, **an ACCEPTED verdict means "this looks real", not
"this is settled"** — several Tier-1 findings may turn out to rest on a
constraint the scanning agents could not see.

**What this report is not.** It is not evidence that any of this is
wrong. Several findings describe deliberate, ratified positions that a
scanning agent could not distinguish from drift — the assembly gate
(S24) and the `Interval` feature's build posture (S1) are the clearest
cases. A finding is a *question worth answering*, not a defect.

---

## Contents

- [Tier 1 — architectural, load-bearing](#tier-1--architectural-load-bearing) (S1–S15)
- [Tier 2 — significant](#tier-2--significant) (S16–S37)
- [Tier 3 — real but lower stakes](#tier-3--real-but-lower-stakes) (S38–S48)
- [Findings raised by the Wave-1 fix lanes](#findings-raised-by-the-wave-1-fix-lanes-2026-08-18) (S49–S58)
- [§A. Where I would start](#a-where-i-would-start)
- [§D. The schedule](#d-the-schedule) — live rows only, in tracks: **A** in flight, **B** the parallel orchestrator's, **C** unclaimed, **D** the audit's unscheduled rows plus B2/B3
- [§C. Process observations](#c-process-observations)
- [§B. Negative results and coverage](#b-negative-results-and-coverage)

---

# Tier 1 — architectural, load-bearing

## S1. Two interval arithmetics, and the ratified one is absent from shipped builds

- **Where**: `crates/geom-core/src/ring_interval.rs:107`,
  `crates/geom-core/src/interval.rs:126`,
  `interval-transcendentals/src/interval.rs:44`,
  `docs/GENERICS-BUILD-COST.md:95`
- **Confidence**: sure

`RingInterval` reimplements outward-rounded interval arithmetic — `±`,
`×`, `÷`, `powi`, `sqr`, `hull`, `from_bounds`, `lo`/`hi`, `contains`,
including its own `down1`/`up1` restatement of `interval-transcendentals`'
`round.rs` technique — that `DInterval` already provides. `RingInterval`
is always compiled and carries ~600 references across `geom-brep`,
`mesh`, `topo`, `geom-curves` and `geom-core::spline`. `Interval` — Q1's
*ratified* certification scalar — has 37 `cfg`-gated `src` sites and, per
the repo's own measurement, **zero symbols in a default build**.

The two disagree on semantics a caller must know: `RingInterval` has no
decoration channel, no empty set, poisons on a zero-touching divisor
where `Interval` clamps-and-decorates, and requires `sqr()` instead of
`x * x` (enforced by a repo-wide CI regex, see S13). The module doc's
"two interval roles, deliberately distinct" argues the split; what the
code shows is that the certification arithmetic actually running in
shipped builds is the second type, and the first never displaced it.

**Verdict:** ACCEPTED (Evan, 2026-08-18). "RingInterval probably isn't
earning its keep. Probably it should be removed and we should only use
the full `Interval` — but I'd want to double check that there's no hidden
cost." Steelman pass commissioned: what would deleting `RingInterval`
actually cost (feature-gating, build graph, the poison-vs-clamp semantic
difference at the ~600 call sites)?
**Steelman (2026-08-18): SURVIVES IN PART — the core claim is confirmed
and strengthened; two sub-claims are wrong; and the obstacle to acting
is not the one anyone expected.**

*Original basis — found, and it is unambiguous.* `CURVED-DESIGN.md`
OQ8/C9, signed off by Evan in PR #85 on 2026-07-24: *"Sign off on adding
a second interval type … **alongside inari** … This is a
**licensing**/architecture fork."* C9's stated constraint
(`:521`): interval arithmetic is load-bearing on the default build path,
*"which at the time meant dragging the `interval` feature's LGPL
transcendental stack (issue #4) into every consumer."* The binding spec
is blunter — `docs/archive/M5-PR2-SPEC.md:20`: *"NO feature gating (this
is the DEFAULT build path — **that is its reason to exist**)."*

*The premise died 2 h 44 m before the type merged.* PR #127 (inari →
in-house backend; *"kernel is now copyleft-free in every build
configuration"*) merged **2026-07-28T03:24:19Z**. PR #130
(`RingInterval`) merged **2026-07-28T06:08:41Z**. The ring's module doc
(*"always compiled, no feature gate — that is its reason to exist"*) was
authored 2026-07-27, while inari was still the backend, and never
revised. C9's retrospective header concedes *"C9's inari quarantine is
dead vocabulary"* — but retires the quarantine, not the second type the
quarantine was the reason for. **No document since #127 re-derives the
split on any other ground**, and there is no issue on it.

*A justification that postdates the thing it cites.*
`crates/geom-brep/src/ssi/enclose.rs:7` defends ring-only certification
with *"always compiled, no feature gate, **no LGPL**"* — committed
2026-07-30, two days after the LGPL left the tree.

*C9's technical premise is also falsified in the tree.* C9 argued the
ring needed *"±, ×, ÷ … no sin, no exp"*. The tree then hand-rolled,
over the ring: `cos_step`/`sin_step` as degree-8/9 alternating-Taylor
enclosures (`props/quad.rs:157`), `sqrt_enclosure` (`:829`), an
`AbsEnclosure` trait (`:383`), and three more ad-hoc
`sq.hi().sqrt().next_up()` sites. `DInterval` already ships every one of
those with a proven pad. The transcendental-free premise held about two
weeks.

*The keep-argument is circular.* `GENERICS-BUILD-COST.md:421` gives as
a reason to keep the feature gate that *"`ring_interval.rs`'s docs lean
on the gate"*; `ring_interval.rs:11` gives as the ring's reason to exist
that it is always-compiled because of the gate. Each artifact cites the
other.

**Two sub-claims of this finding do NOT survive, and both are my
errors:**

1. **"requires `sqr()` instead of `x * x`" is not a difference between
   the two types.** It is the general interval-lane rule; per
   `memories/interval-square-poison.md` it *originated on the `Interval`
   lane* (all four occurrences were found there), and the CI regex
   allowlists `interval.rs` and `ring_interval.rs` identically. I
   presented a spelling difference as a semantic one.
2. **"~600 references" overstates the blast radius.** Measured: **535
   `src` references in 15 files**, with five files carrying 60%
   (`props/quad.rs` 177, `mesh/nurbs_cert.rs` 66, `ssi/enclose.rs` 57,
   `spline/compose{,/tensor}.rs` 102). `topo` has 13, in one file. This
   is a 15-file change.

*The real obstacle, which is not build cost or licensing.* **The
decoration channel does not survive the `Enclosure` seam.** The seam is
`lo()`/`hi()` and nothing else (`real.rs:564`), and the certification
test is `residual.hi() <= eps`. `RingInterval` puts its failure state
*in the endpoints* (NaN), so poison cannot be lost. `Interval` puts its
failure state in the **decoration**, and `impl Bounds for Interval`
(`interval.rs:451`) forwards `self.0.lo()` **without consulting it**. A
one-signed divisor *touching* zero returns `dec = Trv` with one finite
side (`interval-transcendentals/src/arith.rs:89`) — e.g.
`[-2,-1]/[0,1] = [-inf,-1]`, `hi() = -1`. The ring poisons there. So a
blind type swap would **convert a refusal into a pass on exactly the
case the ring was specified to refuse**. That is not a mechanical
substitution.

*Costs that turn out not to be real.* Build cost is ~zero: measured on a
4-core box with `cargo clean` before each, `cargo build --workspace` =
**24 s**, `--features geom-core/interval` = **24 s**. The feature adds
~1,400 lines of pure-Rust libm-only code and no new external dependency
— `inari`/gmp is optional behind `oracle-inari`, which geom-core never
enables. **The LGPL risk is genuinely gone.** And every workspace member
is `publish = false` with no external consumers, so the ring's presence
in five crates' public API is an internal rename, not a break.

*Costs that are real:* re-pinning every certificate number computed by
this arithmetic (#130's `2.8e-14 m²`, the 12,000/12,000 planted-corruption
refusals, `mesh`'s sagitta budgets in `docs/tess-budget-data/`); ~6,600
lines of ring-specific tests across 12 files validating an arithmetic
that would no longer exist; ungating the feature is a `DESIGN.md` Q1
revision, i.e. a design conversation, not an agent-merged PR; and an
unmeasured runtime question — `DInterval` is 24 bytes vs 16 and performs
exactness-witness FMAs per op, inside the SSI subdivision and
per-triangle mesh certificates, **with no ring-vs-`DInterval` benchmark
anywhere in the repo**.

*A genuine subtraction on the other side:* `Enclosure` exists **only** to
admit `RingInterval` alongside `Bounds` (two impls: the ring, and a
blanket over `Bounds`). Delete the ring and `Enclosure` collapses into
`Bounds` — which also removes the `bracket<E: Enclosure>` laundering door
at `spline/hull.rs:98`. See **S41**.

*Could not determine:* whether the ~72k cases where the ring is tighter
(its nonneg sign clamp and zero annihilator, out of ~3M differential
comparisons) are rules `DInterval` should simply adopt; whether any of
the 15 sites depends on poison-on-zero-divisor in a way `Interval` would
change (settled by a differential run comparing **certify/refuse
verdicts**, not endpoints — the existing `ring_interval_differential.rs`
harness already does the endpoint half); and whether Evan ever
independently wanted a decoration-free certification substrate, or only
wanted LGPL off the default path. OQ8's sign-off adds *"doubling as the
seed of the eventual inari replacement"*, which #127 discharged three
days later by a different route.

## S2. `T: Real` genericity costs friction everywhere and monomorphizes at one-and-a-half types

- **Where**: `crates/editor-core/src/eval/mod.rs:865`,
  `crates/geom-core/src/dual.rs:200`, `docs/GENERICS-BUILD-COST.md:88`
- **Confidence**: likely

In a default build the only `Real` inhabitants that appear are `f64` and
`Probe` (a transparent `f64` newtype). `Interval` is 0 symbols; `Dual`
has **no** production consumer at all — every `Dual` use across
`geom-curves`, `geom-surfaces` and `geom-core` is inside `#[cfg(test)]`.
Meanwhile the cost is paid throughout: five-term bound stacks
(`T: Decide + ContentBits + geom_core::Bounds + Send + Sync +
topo::PropsQuadLane`) repeated at eight-plus signatures, plus the four
lane traits of S3.

`GENERICS-BUILD-COST.md` answers "does monomorphization hurt compile
times" (it does not). It does not address whether the abstraction earns
its signature-level friction, which is the more interesting question and
the one this finding raises.

**Verdict:** ACCEPTED IN PART (Evan, 2026-08-18). The `Interval` half is
moot — "interval is a live feature and per S1 may be in all builds soon."
**`Dual` is the live question**: "we need to investigate what's up with
Dual." Steelman pass commissioned on that specifically: is `Dual` planned
work with a scheduled consumer, or was it superseded by analytic
derivatives?
**Steelman (2026-08-18): SURVIVES IN PART — the "no consumer" framing is
wrong, but the position is worse than "planned work waiting its turn".**

*Original basis.* `Dual` is not accretion. Evan on PR #110 (2026-07-26):
*"the original purpose to making a `Real` instead of just using `f64` was
to support this feature, and `Interval` was a quasi-stand-in for a
uniform distribution."* It was slated in `docs/archive/M0-PLAN.md:50`,
implemented as M0 PR 5, and ratified with Evan's sign-off on PR #10; the
module docs are contemporaneous with the code (same commit `355e1fa`),
so this is a decision, not a rationalization.

*There is a named, dated, ratified consumer.* `docs/ERROR-DESIGN.md` is
RATIFIED (Evan, PR #110). **E4** (`:172`) is exactly the missing
consumer: *"∂m/∂pᵢ = evaluate the recipe at `Dual<f64>` with pᵢ
seeded"*. E5 consumes it for stackup reports; E7 (`:321`) uses
`Dual<Interval>` for monotonicity pruning. `DESIGN.md:1381` schedules
this as **M10**. `CONTACT-DESIGN.md:386` already writes contracts *for*
"the E4 dual lane".

*Not superseded.* All three candidate replacements were checked and
none covers E4: `ssi/jet.rs:11` declines `Dual` because it needs 2nd/3rd
derivatives and forward-mode duals do not nest; `derivative_coeffs`
computes sup-|C″| hulls, a different quantity; and `Surface::deriv_*`
are derivatives w.r.t. *surface* parameters where E4 needs them w.r.t.
*model* parameters through the whole recipe. Nothing in the kernel
computes that.

*`Dual` also earns its keep today, which my report missed entirely.* It
is the differential oracle for ~52 non-test call sites of analytic
derivative code — `first/second_derivatives_match_duals` cross-check
every surface and curve variant at 1e-12 including mixed-partial
symmetry by two independent routes. And
`topo/tests/review_m2_pr3.rs:218` builds an entire `Body<Dual64>` and
asserts bit-identity with the `f64` build — a live proof of the
value-channel contract and of Q1's "derivatives never branch" that no
other scalar can provide.

*The sharpest thing the steelman found, which upgrades this finding
rather than dismissing it:* **the planned consumer cannot be built
without first undoing a bound the kernel has already committed to.**
`eval/mod.rs:934` requires `T: … + geom_core::Bounds + …`; `Bounds` is
deliberately not implemented for `Dual`. So `evaluate::<Dual64>` **does
not compile**, and `grep -c Dual crates/editor-core/` is **0** — the
document layer where E4's recipe lives has never heard of duals. E4's
mechanism is structurally blocked today, and no spec, issue or PR
registers that blocker (all 39 open issues checked).

*Net position:* the cost is real and is `Dual`'s; the benefit is
deferred to M10; and the deferred benefit has an **unpaid, unregistered
structural prerequisite**. That is materially worse than "planned work",
and materially better than "dead weight".

*Hidden costs of dropping `Dual`:* it revises a doc Evan ratified with a
👍, re-opens the kink conventions settled over two rounds on #9/#10 and
already leaned on by `CONTACT-DESIGN.md:386`, forfeits the differential
oracle and the only end-to-end no-tangent-leak proof, and demotes the
L7 "evaluation code must not read brackets" rule from a compiler fact
(`Dual` is the one `Real` without `Bounds`, so a signature that reaches
for a bracket stops compiling) to a CI grep.

*Explicitly separable:* dropping `Dual` and narrowing the genericity are
**two different decisions and should not be bundled**. The load-bearing
genericity terms — the `Decide`/`Bounds` split, the `SpanLocate` seam,
the four-scalar test axes — are driven by `Interval`, not by `Dual`.

*Could not determine:* when M10 actually happens (no `M10-PLAN.md`, no
open issue); whether the `Bounds` blocker is known-and-unwritten or
genuinely unnoticed; and **whether a `Dual` test has ever caught a
defect in the analytic derivative code it checks** — that last one
distinguishes "real oracle" from "oracle that has never fired", and
would be settled by a mutation run against `deriv_u`/`deriv_uv` with the
dual tests as the only gate.

## S3. The "lane trait" — one cargo-culted pattern, four instances, three spellings of the same answer

- **Where**: `crates/topo/src/props.rs:360`,
  `crates/geom-brep/src/pcurve_cache.rs:907`,
  `crates/geom-brep/src/edge_nurbs.rs:214`,
  `crates/topo/src/chart_region.rs:252`
- **Confidence**: sure
- **Found independently by four scans** (topo-validation, pcurve-cache,
  geom-core-scalar, geom-brep-props) from four different files.

`PropsQuadLane`, `PcurveFittedLane`, `EdgeNurbsLane` and
`ChartRegionLane` are the same construct four times: a `Decide` subtrait
with one method, three impls (`f64`, `Probe`, `Interval`) each forwarding
verbatim to one shared `fn lane<T: Decide + Bounds>` body, plus a
refusing `Dual` impl and a `lane_name() -> &'static str` for
diagnostics. That is ~90 lines apiece, 16 impls in total, whose entire
information content is *"does this scalar carry a bracket?"* — a fact
`geom_core::Bounds` already names.

The pattern has not converged. Absence of a lane is signalled as
`Ok(None)` in `PcurveFittedLane` and `PropsQuadLane`, as
`Err(LaneUnsupported { scalar })` in `EdgeNurbsLane`, and as
`Option<Result<..>>` in `ChartRegionLane` — so each consumer handles the
same condition differently. `PropsQuadLane` is simultaneously an
instance of the pattern *and* a supertrait bundle of the other three,
and its doc comment spends ~25 lines arguing the bundling is justified
because they are "the same split, over the same four scalars, for the
same reason" — which is the observation that they are one concept,
offered as the reason to keep them as four.

**Verdict:** ACCEPTED (Evan, 2026-08-18). "This is a great candidate to be
collapsed." Steelman pass commissioned to find what actually blocks the
collapse — coherence across the `topo`/`geom-brep` crate boundary, the CI
`Bounds` gate, or nothing.
**Steelman cross-reference (from S2, 2026-08-18): two findings that
sharpen this one.**

1. **At the site this report cites (`eval/mod.rs:865`), the
   `PropsQuadLane` bound is inert.** The sibling `Bounds` bound already
   restricts `T` to `{f64, Probe, Interval}`, and all three have a
   certified quadrature lane. The lane trait buys nothing at that
   signature.
2. **The repo's own precedent says these collapse if `Dual` leaves
   `Real`.** When the same tripwire fired on the fillet battery the
   ruling was: *"no dual-scalar path can reach the fillet constructor …
   A `PropsQuadLane`-style static split would therefore have had an
   EMPTY refusing side: a dual impl refusing a call no dual scalar can
   make. So the seam is RATIFIED rather than split"*
   (`docs/archive/M5-LOG.md:2869`). All four lanes exist to carry a
   refusing `Dual` impl; remove `Dual` and every refusing side goes
   empty, so by that precedent all four become a plain
   `T: Decide + Bounds` seam.

Note also that the refusing impls are **Evan's own ruling**, not cargo
cult: `docs/archive/M5-LOG.md:3451` records *"Evan: it is not
semantically valid type-wise for duals to enter a pipeline that can only
refuse them — adopted."* So the question is not "why do these refuse"
but "should `Dual` be in `Real` at all" — i.e. S3 is downstream of S2.

**Steelman (2026-08-18): SURVIVES IN PART — not cargo-culted, the stated
obstacle is real, and a working collapse exists anyway (compiled).**

*Not accretion.* Four dated rulings. Instance 1 (`PropsQuadLane`, PR
#157) replaced an already-written runtime mechanism on **Evan's own
pushback** — `M5-LOG.md:3451`: *"it is not semantically valid type-wise
for duals to enter a pipeline that can only refuse them — adopted."*
Instance 2 (PR #176) states its blocker as **measured**: putting the
bound on `certify` cascaded through every public boolean door — *"7
compile errors at the first hop alone"*. Instance 4 was **pre-committed
a PR in advance** (`real.rs:474`). And the pattern was twice
**deliberately declined** (the fillet battery, `ssi/enclose.rs`).

*The real obstacle is the crate DAG plus coherence — not `Bounds`'
shape.* `PropsQuadLane`/`ChartRegionLane` name `topo` types;
`PcurveFittedLane`/`EdgeNurbsLane` name `geom_brep` types **and are
consumed inside `geom-brep`**. **No single crate can host all four
methods without a dependency cycle.** The agent then *tested* the
obvious blanket-impl collapse in a downstream crate and got
`error[E0119]: upstream crates may add a new impl of Bounds for Dual` —
so given the trait lives in `topo`/`geom-brep`, hand-enumerating the
four scalars is genuinely forced.

*`Dual`'s refusing impl is load-bearing.*
`sweep/tests/extrude_acceptance.rs:565` calls `validate_geometric` on a
`Dual64` body. Delete the impls and that **stops compiling** — a dual
body becomes un-validatable, contradicting Q1's ratified instantiation
set. A missing impl would not do the same job.

*But a collapse exists, and it compiles.* One trait plus a rank-2 job
callback, **both in `geom-core`** — where the blanket impl is legal
because `Bounds` and `Dual` are both local there — takes 4 traits × 4
impls down to **one trait and two impls**, keeps every call site's
arguments and crate, keeps the certified bodies at `T: Decide + Bounds`
in their already-allowlisted files, and preserves the static guarantee
that `Dual` instantiates none of the certified machinery. Verified on
stable (`rustc 1.94.1`), cross-crate: `Ok(2.0)` at `Interval`,
`Err(LaneUnsupported("dual"))` at `Dual`.

*Three defects confirmed at source:* `PcurveFittedLane`'s `Option` layer
is pure indirection — its sole consumer (`pcurve_cache.rs:2423`)
converts `Ok(None)` straight into the `EdgeNurbsLane` shape.
`lane_name()` is **6/8 dead** (the `f64`/`Probe`/`Interval` impls are
unreachable; nothing else in `crates/` reads it) — and only 2 of the 4
traits have it at all, so this report's "plus a `lane_name()`" is 2/4.
`ChartRegionLane`'s `Option<Result<..>>` routes `None` and `Some(Err)`
to the **identical** `CensusUnsupported` at both consumers.

*What does not survive:* "cargo-culted" as a characterisation, and
"entire information content is 'does this scalar carry a bracket?'" —
`PropsQuadLane`'s `None` also carries *"not attempted"*, a channel the
other three lack (`props.rs:230`).

*Hidden costs:* no new crate dependency (the collapse runs **downward**
into `geom-core`). But the CI gate gains a row and loses granularity —
four *narrow* trait names carrying the compound bound into a bounded
blast radius become one *workspace-wide* name, which would want its own
name-gate like `EvalScalar`'s. Net saving ~250–300 lines, not ~360:
each call site swaps forwarders for a job struct, and
`fitted_certificate` has **seven** arguments, six of them references.
`PropsQuadLane` also does a naming job (`EvalScalar`'s component) that
`BracketLane` cannot, so a `topo`-side alias trait may survive. And
`DESIGN.md:562` names a coming cascade — the mint pass needs this bound
on every constructor — so doing the collapse first makes it one bound
instead of two migrations over the same signatures.

*Could not determine:* **whether a `geom-core`-level mechanism was ever
considered.** Every recorded deliberation is about *whether to split at
all* or *parallel bound vs supertrait*. Nothing anywhere discusses
moving the dispatch into `geom-core`. It appears never to have been on
the table.

## S4. One vocabulary, N hand-synced copies — the dominant repo-wide shape

- **Confidence**: sure
- **Found independently by six scans.**

**The `units` row is FIXED by #646** (that row only — every other row of
this finding stands). There is no second spelling of the six units left in
any crate's `src`: the display-unit code stored in `Lit` is now the row's
POSITION in `quantity::UNITS`, so both directions go through the table.
**Not** "written once in the workspace" — two proptest generators still
enumerate the symbols by hand (`quantity/src/tests.rs:16` writes all six
in order, deliberately, as the vocabulary pin;
`editor-core/tests/u8a_parse.rs:366-369` writes the four length symbols
and both angle symbols). What is true is narrower and is the part that matters: no `src`
in the workspace holds a second opinion about the vocabulary, so a
**reorder** of `quantity::UNITS` reaches `editor-core` with no edit
anywhere, and an addition or rename needs a test edit but no `src` edit.
The remaining `src` copies are `pncad-py`'s six module bindings plus its
stub declarations, which are forced (PyO3 must name a module attribute at
its registration site; `IN` → `inch` is a documented fork) and generatable
at best, not collapsible — **and unpinned**:
`pncad-py/tests/test_stubs.py:95` checks only that `"mm"`, one of the
six, is among the top-level names, so the stub could lose five of them
silently.
`step-import`'s `UnitKind` is a different vocabulary, as the steelman
ruled. #291's MAJOR-2 measurement — the reason a CODE is stored rather
than the row — now has a mechanical guard of its own: `size_of::<Lit>()
== 16` is a compile-time assertion, and re-inlining the 32-byte row
(which takes the literal payload to 40) fails the build. (It was never
*entirely* unguarded — `large_enum_variant` is default-on in clippy's
`perf` group and CI runs `-D warnings`, the same detector that fired in
#291. What was unguarded is the **margin**: whether a regrowth
re-crosses that lint's 200-byte threshold depends on `DocEdit`'s size,
which nothing tracks, and `Lit` is a struct, so it trips no enum lint
alone.)

**`units` follow-up — #650 CLOSED by sealing, not by a check (this PR).**
#646 left one residue on the same row: `Expr::literal_with_unit`
validated the CALLER's `UnitDef.quantity` and then stored the table row
found by symbol, never re-checking, so a caller-built `UnitDef { symbol:
"mm", quantity: Angle, .. }` built an `Expr` that serialized into a
document editor-core's own load door refused — a round-trip break, not a
bad call. #650 offered a two-line whole-row check; the fix taken instead
is D2-addendum-shaped: **make the row unrepresentable**, so there is no
state for a check to catch. `quantity::UnitDef`'s three fields are
private with `symbol()`/`quantity()`/`factor()` accessors, there is no
constructor at all, and the only sources of a row are `UNITS`,
`unit_by_symbol`, and whatever hands one back.

Five things are worth recording — two because each was a way the seal
could have been cosmetic, one because the seal's own cleanup was nearly
missed, one because the residual was measured wrong before the work
started, and one because sealing MOVED where the coverage gap lives:

- **The second mint — now CLOSED by #686, see the `#669` note below.**
  `LengthUnit`/`AngleUnit` kept PUBLIC fields, and
  their `def()` was a public `const fn` — so `AngleUnit { symbol: "mm",
  factor: 1.0 }.def()` rebuilt #650's exact counterexample with `UnitDef`
  sealed. Sealing `UnitDef` alone would have renamed the hole. `def()` is
  now `pub(crate)`; the public route to a row is `unit_by_symbol`. This
  is the S4 lesson applied to a CONSTRUCTOR rather than a vocabulary:
  counting the copies of a spelling is not the same as counting the doors
  that can mint one.
- **The refusal the seal killed, and the apparatus that nearly outlived
  it.** `UnitSym::from_def`'s `None` arm — #646's newly-covered branch —
  is unreachable once no caller can build an off-table row. The first
  pass kept it and built the scaffolding to reach it: a `units-testing`
  cargo feature, a `#[doc(hidden)]` mint, a self-dev-dependency, an
  explicit dev-dependency in `editor-core`, and two tests. Review
  observed that **the same `impl` block was answering one question two
  ways** — `UnitSym::def`'s unconstructable index takes D2 addendum row
  4 (`unreachable!`) with the argument stated out loud, while
  `from_def`'s equally unconstructable row kept a typed refusal. Evan
  ruled it (2026-08-19): *"we definitely should not have any checks for
  states the type system excludes."* `from_def` is now total, the whole
  apparatus is deleted, and `DimensionError::UnknownDisplayUnit` keeps
  its ONE reachable raiser: `persist::wire`, where the symbol arrives as
  a string out of a file. **The general lesson for a sealing diff:
  sealing a type makes some downstream refusals unreachable, and the
  cost of keeping one testable is a cargo feature plus two dependency
  edges — price that before paying it, and check whether a neighbouring
  function in the same block already answered the question.**
- **The view of a sealed row — #669, FIXED by #686.** Sealing `UnitDef`
  moved the hole rather than closing it: the dimension stayed typed while
  the symbol and factor went free one level out, and that asymmetry was
  invisible until the residual was *executed* rather than reasoned about.
  It was neither formatter-only nor a display string, as #662's note had
  scoped it: `fmt.rs`'s own pin is `parse(fmt(x, unit))`, so a mislabelled
  view reached an `Expr` and a document along the route the module
  guarantees, and `25.0 * LengthUnit { symbol: "mm", factor: 1.0 }` was a
  25-METRE `Length` labelled `mm` — a wrong VALUE at the D6 boundary,
  across five public entry points (eight functions).

  **#686 made it unrepresentable, and the first attempt was not enough.**
  The views became newtypes wrapping the sealed row; review then
  demonstrated by mutation that the `compile_fail` guard was **already
  dead** — every view row failed `E0560` (*no field named `symbol`*), a
  SHAPE mismatch that stays true however open the seal becomes, so
  reopening the field left all seven rows green. It also planted
  `pub const MM_BAD: LengthUnit = length("mm", 1.0)` INSIDE `units.rs`,
  using the private helper the docs called the only place a row is
  written down, and reproduced #669's headline defect with every guard
  green: the real guarantee was *"constructed inside `units.rs`"*, not
  *"is a row of `UNITS`"*, which the rustdoc claimed in four places.

  What landed instead is what `editor-core` had already invented for this
  same table in `UnitSym(u8)`: **the view is a private INDEX into
  `UNITS`**, behind a private module, so `of_row` and `UnitDef::as_*` are
  the only mints *anywhere* — `units.rs` included. *"Is a row of
  `UNITS`"* is now true by construction rather than by a convention
  confined to one file, the duplicated `quantity` field is gone, and a
  seventh constant or a mislabelled one is a **const-eval** error rather
  than a lint. The guard rows are now refused on **privacy** (`E0423` /
  `E0616`), verified by mutation off-branch: opening the field reddens
  six view rows and correctly leaves the two `UnitDef` rows green.

  Two things the fix found that #669 did not name: a **sixth,
  non-public trust point** — `def()` built `UNITS` *from* the views, so
  the table's own rows were minted through the unsealed surface, which is
  why #662 had to demote it — and that the **Python mirror was never
  open**: `pncad.LengthUnit`/`AngleUnit` are `#[pyclass(frozen)]` with no
  `#[new]`, so the anticipated `pncad-py` work was six `.symbol` →
  `.symbol()` reads and no `.pyi` change. The lesson stands generalised:
  seal the ROW and the residue moves to the VIEW of the row — and a
  guard that pins the seal by quoting the PRE-SEAL spelling tests the
  shape, not the seal.
- **Sealing moves the coverage gap to the load door.** With construction
  closed by the type, the only production-realistic route to a corrupt
  document is a `.cad` file carrying a TABLED symbol on the wrong
  dimension — `{"dim":"Angle","unit":"mm"}`. Nothing tested that:
  editor-core's suite covered the unknown-SYMBOL arm and the CONSTRUCTOR
  arm, which is precisely the shape #646 and #650 both recorded as the
  reason the original defect hid ("the refusal reads as covered because
  a different construction site is covered"). A row was added. Generalise:
  after a seal, re-ask the coverage question at every door the sealed
  value can still ARRIVE at from outside the process — deserialization
  first.
- **In-crate code is not exempt.** `quantity/src/tests.rs` is a SIBLING
  module of `units`, not an inner one, so the private fields are private
  to it too; the first push was red on `clippy` for exactly that. Worth
  remembering when estimating a sealing diff: "same crate" is not the
  visibility boundary, "same module or below" is.

| Concept | Copies | Anchor |
|---|---|---|
| profile `Step` verbs | `profile::Step` / `ProgramStep` / `WireStep` / `StepArg` / content-key tag table — **5**, across 3 crates | `program.rs:64`, `persist/wire.rs:255`, `profile/src/path/program.rs:190` |
| `RoleSeg` → `SegTag` | kernel enum → editor-core fieldless mirror → `pncad` re-export → a **second** 40-variant py mirror → 40-arm `to_kernel` → 40-arm inverse tripwire → 1316-line `.pyi` | `pncad-py/src/py/select.rs:82` |
| node kinds | ~10 parallel match tables; `rg Node::Fillet` → 24 non-test hits in 10 files | `node.rs:423`, `eval/mod.rs:1325` |
| "which `RoleSeg` args are sub-names" | 4 sites | `resolve/mod.rs:969`, `refactor.rs:540`, `names/select.rs:296`, `eval/mod.rs:2040` |
| "which payloads carry a `StableName`" | 4 lists | `edit.rs:1096`, `node.rs:949`, `refactor.rs:801`, `resolve/mod.rs:911` |
| "node has no usable value" | 5 typed + 1 stringly | `resolve/mod.rs:266`, `resolve/hit.rs:23`, `resolve/vdiff.rs:69`, `appearance.rs:172`, `names/geompred.rs:488` |
| units | 7 spellings for 6 units | `quantity/src/units.rs:47`, `expr.rs:181`, `step-import/src/units.rs:76` |
| Euler vector per op | 3 (prose, arena delta, `ep_vector`) + a 4th divergent `Ledger`; the arena delta was an unnamed positional 7-tuple until **#625** made it `ArenaDelta` | `euler.rs:891`, `euler.rs:838`, `seqgen.rs:105` |

Two of these have **already drifted, observably**:

- `Node::named_nodes` lists `Declare`, `Mate`, `Fillet` and its comment
  states `Rebind` is the repair for all three. `DocEdit::Rebind`'s
  rewrite loop in `apply` matches only `Declare` and `Fillet` (`_ =>
  {}`); `refactor::payload_names` returns names only for those two. A
  mate head is validated at insertion, documented as repairable, and
  silently not rewritten.
- **FIXED by #632.** Three of the four `RoleSeg` sites carried comments
  insisting the match is exhaustive "so a future variant must be classified
  here or the compile breaks"; `select::name_args` was the fourth and
  wildcarded. All four are now exhaustive on both the `RoleSeg` and the
  `Qualifier` axis.

**The `BooleanOp` mirror is FIXED by #642.** It was the tell that these
copies are accretion rather than principle — one vocabulary **mirrored**
from the kernel while its neighbour `ContactClass` was **imported**, both
documented at length as correct under the identical constraint. There is
now one `BooleanOp`: `editor-core` re-exports `topo`'s and describes only
its BYTES, and the identity `From` match in `eval/wire.rs` is gone.
`topo` gained no serde dependency — the layering the mirror was minted to
respect is intact, and is now the thing a gate checks.

Byte-preservation was **proved, not asserted**: the whole model corpus
was `save()`d before and after and compared with `cmp` (389 741 B, 48
boolean operations — identical), and the three wire pins were written and
run green against unmodified `main` **first**, so they pin today's format
rather than describe the new one. The move of the modules under
`persist/` was re-proved the same way.

Two things a future reader will want, neither of which this finding
predicted:

- **The read direction could not be made compiler-exhaustive.** Safe Rust
  cannot tie an array literal to a variant list without a proc macro, and
  the workspace has none — so an operation added to the write table but
  missed in the read table would serialize fine and refuse on READ: a
  document this build wrote and could not load. The gap is closed **at
  the write door** instead. `serialize` round-trips the spelling through
  `untag` before writing and refuses typed if it does not come back, so
  the unloadable document is never created. Where a mirror's cost was
  paid in hand-syncing, an import's residual cost is paid here, and it
  is smaller and it is loud.
- **The technique now has a home.** `#[serde(with)]` modules for kernel
  types live under `editor-core/src/persist/kernel_wire/` — one doc for
  the technique, including the rule that a module's CASING is not a style
  choice but whatever its type's bytes already are (`BooleanOp` is
  capitalised *because* it replaced a derive; `ContactClass` is lowercase
  because it never had one). Previously the two copies sat at the bottom
  of a 1,320-line `node.rs` with a near-word-for-word duplicated doc, and
  the queued rows below would have made copies three and four.

**Verdict:** ACCEPTED (Evan, 2026-08-18). "Oh boy, good findings — these
look like they'll be a lot of work to fix but definitely worth it."
Steelman pass commissioned for the constraint map: how much of the
duplication the G1 no-serde-in-kernel rule actually forces, given that
the `ContactClass` `#[serde(with)]` remote derive shows the mirror was
avoidable at least once. Includes verifying the two observed drifts.
**Steelman (2026-08-18): SURVIVES IN PART — row by row, and the diagnosis of
the `BooleanOp`/`ContactClass` tell was wrong in a way that makes the finding
*stronger*.**

*The corrected tell — acted on, FIXED by #642.* The steelman's finding was
that `BooleanOp` was **never a serde decision**: PR #81 (M4 PR 1) forbade
`editor-core` from depending on `topo` **at all**, which is why the enum was
minted locally, and `git log -S` showed the dependency arriving the **same
day** (`baec1fd9`, M4 PR 2). The mirror had outlived its reason by four weeks
and its rustdoc defended nothing; the only text calling it a deliberate
pattern sat inside `ContactClass`'s doc, written by a different author three
weeks later. `ContactClass` meanwhile **was** ruled — `M9-1-SPEC.md:22`,
*"never a parallel enum… One enum, defined lowest, re-exported upward"* — and
#552 resolved the same collision with `#[serde(with)]`, a technique that did
not exist as a repo idiom when `BooleanOp` (2026-07-23) or `WireStep`
(2026-08-09) were written.

#642 collapsed it, and the collapse cost less than the row implied: the
`From` pair it removed had already become the identity, so **no conversion
had to be preserved anywhere** — the recipe node's operation now IS the
kernel operation the evaluator runs. Two things the row did not anticipate
turned up in the vocabulary sweep and are worth carrying to the remaining
mirrors: the split had also cost a **hole in the public prelude** (a curated
paragraph explaining why `pncad::prelude` could not carry `BooleanOp`, since
two types cannot share one name) and a **defensive alias at a call site**
(`demos/tour`'s `BooleanOp as NodeBooleanOp`). A mirror's price is not only
the hand-syncing; check `WireStep` and `SegTag` for the same downstream
surface tax before pricing them.

*Also corrected:* `RoleSeg` is **not** a kernel enum — it lives at
`editor-core/src/names/role.rs:226`. Both ends of that chain are editor-core.
And `DESIGN.md:1914`'s *"(layering enforced by CI grep)"* was **stale** — no
serde grep existed in `ci.yml` or `ci-local.sh`, the only mechanical check
being `profile/tests/seal.rs:87`, covering `profile` only. **FIXED by #642**,
which put a real gate behind the claim (`scripts/gates/kernel-serde-free.sh`)
and rewrote the DESIGN.md row to say what that gate proves and what it does
not. The gate's own history is the lesson: its first version WAS a grep, and
the reviewer falsified it twice — `serde.workspace = true` (the dotted form
already used for `edition`/`license`/`rust-version` in `topo`'s own manifest)
and `ser = { package = "serde" }` both walked past it and it reported OK.
Worse, its self-test planted only the one spelling its regex was written for,
so it could never have found its own hole. It is now a TOML parse over the
resolved package name of every dependency entry, self-testing five spellings
plus a negative control. A gate that scans a hand-shaped surface with a
hand-shaped matcher is S4's failure mode wearing an enforcement badge.

| Row | Verdict |
|---|---|
| profile `Step` verbs | **SURVIVES** — `WireStep`/`WireTarget`/`WireArcData` are field-for-field mirrors differing in **nothing**. Only `WireSide`/`WireWinding` wrap kernel-foreign types (two two-variant tags), plus `SketchPlane<f64>` needing `WirePlacement`. The scheduled RESPELL-TABLE unit does **not** reach these. |
| `RoleSeg` → `SegTag` | **SURVIVES IN PART** — three of four links are compile-enforced and the python lane runs in CI. Genuine gaps: the `.pyi`'s 40 members are **unpinned** (`test_stubs.py` parses only top-level names, never class bodies), and the py mirror is **forced by the orphan rule** — not collapsible, only generatable. |
| node kinds (~10 tables) | **DOES NOT SURVIVE as stated** — 10 operations over a 12-variant sum type is the design working. Re-scoped to *wildcard* arms it survives: `node.rs` 9, `eval/mod.rs` 5, `resolve/mod.rs` 4, `edit.rs` 3, `refactor.rs` 3, `persist/check.rs` 2. |
| `RoleSeg` arg sites | **SURVIVED IN PART; FIXED by #632.** The four answer four genuinely different questions and *should* differ — what survived was the fourth site's wildcard, now closed. |
| `StableName` payload lists | **SURVIVES** — see the confirmed drift below. |
| "no usable value" | **SURVIVES IN PART** — the four enums have genuinely different membership and closure (`RunStatus` is serde-persisted), but all four embed the identical triple, and the stringly fifth is a real fail-quiet. |
| units | **DOES NOT SURVIVE as counted; the residue FIXED by #646.** `parse.rs` uses the shared table; `step-import`'s `UnitKind` is a *different vocabulary* (STEP `SI_UNIT` names). Real duplicates: two-and-a-half, one of them **measured and justified** (PR #291 MAJOR-2: inlining the 32-byte row grew every `Expr` by ~40 bytes). #646 enumerated the two-and-a-half the steelman never named — (1) `expr.rs`'s `UnitSym` enum + its `def()` map, the measured one; (½) that file's *second* table, `from_def`'s six string literals, which the measurement never covered; (2) `pncad-py`'s six module bindings + stub lines, forced by PyO3 — and dissolved (1) and (½) together by making the code an INDEX into `quantity::UNITS`. The code is still one byte, so the measurement stands, and it now has a mechanical guard (a `size_of::<Lit>()` assertion) rather than only clippy's threshold-dependent `large_enum_variant`. (2) is untouched: forced — **and unpinned**, its stub pinned only at one of six names. A residue in `expr.rs` was filed rather than fixed: #650, `literal_with_unit` checks the caller's `UnitDef.quantity` and then stores the table's, so a mismatched pair builds an `Expr` the load door refuses. **#650 is now CLOSED STRUCTURALLY, not by a check** (see the `units`/#650 note below): `quantity::UnitDef` is sealed — private fields, no constructor at all, and `LengthUnit::def`/`AngleUnit::def` demoted to `pub(crate)` because a public `def()` on a still-public-fielded typed view was a *second* mint for the same illegal row. The vocabulary count is unchanged by that; what changed is that the shared table is now the ONLY source of a row, which is the property S4 was arguing for all along. |
| Euler vector | **SURVIVES IN PART; FIXED by #625 and #641.** The 6-vector and the 7 arena deltas are **different quantities** — Δh is not an arena count and cannot be derived from them — so they stay separate **by design**. What survived the steelman was the spelling, and it is now closed on both halves: #625 made the delta `ArenaDelta`, and #641 named every remaining positional carrier in the crate, collapsed `reassembly.rs`'s duplicate into `ArenaCounts` outright, and gave the parent-sense rule one home. The class was **four positional orders for one vocabulary inside a single crate** — S4's drift shape reachable without crossing a crate boundary at all — of which three were byte-identical to `ArenaCounts` and differed only in being separately declared. Residue: one copy is blocked purely by the `tests/`-is-another-crate boundary (**S52**), and the two drifted `Ledger`s are **S53**. Neither mechanism catches a transposition across correct names, which is why both conversions were checked component-by-component. |

*Drift (a) CONFIRMED, and it contradicts ratified design text.* Note the
contrast in the same file: `refactor::remap_node` **is** wildcard-free and
**does** handle `Mate` — the exhaustive sites stayed in sync; the wildcard sites
did not. **FIXED by #618**, which took that lesson as its shape:
`Node::payload_names` and its rewriting twin are the one answer to which
payloads carry a `StableName`, both list the nameless variants rather than
wildcarding them (a future name-carrying variant breaks the compile — verified
by a reviewer's probe variant, `E0004`), and the `Rebind` loop, the insert door,
the split's and inline's re-anchoring, edit-time name resolution and the load
check all read them. The sweep found two further mate-blind sites of the same
shape: the insert door silently ADMITTED a mate head naming no node, and
`persist/check.rs` never id-checked mate heads against the mint counter — a
file that got one in could previously be opened and salvaged by deleting the
mate, and now refuses to load at all.

*Drift (b) CONFIRMED, and **FIXED by #632*** — see the §D H5 row for what the
fix covered and what its review caught. The `Fragment(SideOf)` disagreement is
**documented and intentional** and was preserved. Two residues went to §D:
`resolve::apply_with_names`' `DocEdit` wildcard, correctly left alone but
unscheduled until now, and a verbatim triplication of the name-free variant
list that the fix grew (`select.rs`, `resolve/mod.rs`, `refactor.rs` — every
copy compile-enforced, so churn rather than rot, and collapsing it needs a home
`role.rs` does not have).

*One confirmation this report did not cite: the hand-synced tag table has
already produced a live measured bug.* `MODEL-AB-LOG.md:782` — *"**MAJOR-1 =
`Step::AtToward`'s memo content-key tag 28 COLLIDED with `ArcContinue`'s
existing 28 — latent memo collision, a hit would serve wrong geometry**"*.
Caught by a reviewer, not a type. S4's failure mode, realised.

*Ranked cheapest-to-hardest to act on:* (1) `BooleanOp` → import +
`serde(with)` — **DONE, #642** (cheaper than listed: the `From` pair was
already the identity, so nothing had to be preserved; dearer in one place
the ranking could not see — the read direction needs a run-time write-door
check, because safe Rust cannot make it exhaustive); (2) `name_args`' wildcard → exhaustive — **DONE, #632**; (3) the `Mate` arms —
small but a **behaviour** fix; (4) the Euler 7-tuple → named struct
(debug-only) — **DONE, #625**; (5) units — **DONE, #646** (and smaller than
listed: the only unforced `src` copy was inside one file; the residue it
found and filed rather than fixed, #650, is now **DONE** too — sealed
rather than checked); (6) `ProgramStep`/`WireStep` — cheap in isolation,
**expensive in sequence** (blocked behind OnArc + RESPELL-TABLE, and it
crosses the same files); (7) the "no usable value" core (blocked by a
persisted format); (8) `SegTag` (needs the workspace's first proc-macro
crate); (9) `profile::Step` re-parameterised — *"I would not do it."*

## S5. `splitting/` and `boolean/` are one pipeline built twice, with the shared core hosted inside one half

- **Where**: `crates/topo/src/splitting/join.rs:478`,
  `crates/topo/src/splitting/neighborhood.rs:69`,
  `crates/topo/src/boolean/sectors.rs:76`,
  `crates/topo/src/boolean/mod.rs:548`
- **Confidence**: sure

**MOSTLY FIXED by #647, #661 and #690; the remainder is #695.**

**#690 gave the three shared things homes of their own.** The `sector_face`
twins became one producer (`sector_face.rs`), the ch. 14 join core left
`splitting/join.rs` for `chord_join.rs` — 2,714 lines down to 400 — and
`SplitJoinError::Corrupt` gained a required `entity: EntityId` across its
sites, so the compiler proved that sweep complete rather than a grep. Eleven
sites that were **not** corruption were retyped (ten to the existing
`SectionInvariant`, one to `Band`), minting no sixth bug channel. What did
**not** move, on the steelman's reasoning: `SectorEntry` vs `BoolSector`
(a correct divergence — only the producer was duplicated) and the shared
error variants (every boolean twin carries `operand`, so unification touches
a public API re-exported into four crates).

**The fix's own review is the more useful record.** Two independent lanes
found what the diff hid:

*A load-bearing invariant was falsified inside the PR's own scope.*
`boolean/reduce.rs` stated as an INVARIANT that the planar sense flip lives
in one door *"so there is one flip, not two that could drift"* — and the new
shared producer re-derived it, making two. Fixed by **routing, not
rewording**: `face_normal.rs` is now the door's home (it had to leave
`boolean/`, since a crate-root module importing from one half is the same
wrong-way edge pointed the other way), and the invariant is a **standing
gate** — `the_planar_sense_flip_lives_in_one_place` walks `topo/src` and
fails if any file but the door both destructures a plane surface and mints
an `OutwardNormal`. Four blind spots are written out beside it, chiefly that
`normal * f.sense_sign()` is invisible to it — which is exactly what the
three known D6 sites do.

*The module built to end "built twice" contained a self-declared mirror.*
`chord_spec` and `bool_planar_chord_spec` sat 500 lines apart in the new
`chord_join.rs`, the second saying outright *"Selection logic mirrors
[`chord_spec`]'s S9 block deliberately"* — the C11 vocabulary §C11 says
nobody ever reads, in the file whose subject was that residue. Unified into
`section_case` and `select_arc`, with `Straight`/`Tangent` handed back
because that is the one thing the lanes genuinely disagree about. Its
adjacency-guard twin went the same way, as a class rather than an instance.
Also a standing gate: `the_arc_side_rungs_are_decided_in_one_place` **counts**
decide-sites and requires exactly one each — a cross-file guard that would
have been green throughout the mirror's life, with the rung names assembled
from parts so the guard's own file is subject to it.

**K-neutrality, reproduced twice — and the first instrument was blind.**
`probe_s5_sectors` reproduces 26541 rows at SHA `7c0e4ee0…` between base and
tip, matching #647's precedent. But that probe's fixtures are **all-planar
and contain zero arc-rung rows**, so it could not have seen the mirror
unification at all — the lane noticed and added a Band-4 corpus sweep at
ε=1e-9: 306,143 rows, `cmp`-identical, carrying 400 `split_arc_window`, 80
`split_arc_chart_orientation`, 64 `split_sphere_section_polar` and 16
`bool_between_arc_window`. *The general lesson: a reproduction is only
evidence about the rows its fixtures actually generate, and "the probe
reproduced" is not the same claim as "the change was neutral."*

**What is left, and it is more than the fix first estimated — #695.** The
pipeline is not "one built twice" any more, but two shared cores still live
inside one half: `carve`/`single_solid` (already imported by `boolean/`), and
`conic_plane_crossing_roots`, which `boolean/reduce.rs` calls on the
**production** path and which decides four `split_*`-named K predicates from
the boolean lane — the same *"bidirectional in fact"* shape this finding
cited, refuting the PR's own "nothing shared to extract" for the vertex
sweep. The gate's **edge** halves are token-identical modulo the error
constructor and want the same treatment. `DESIGN.md:1275`'s *"the boolean
engine and its splitting/census machinery"* is now **less** false and shown
to be unmakeable as written: the honest sentence names two peer lanes over a
shared core, which is a design conversation, not a lane's call.

---

*The sector-predicate fork, fixed earlier by #647 and #661:*
The vertex-neighborhood sector-shape rungs — the metering arm, the
wideness verdict, and the subdivision direction — are ONE
implementation, `crates/topo/src/sector_shape.rs`, a top-level sibling
of `boolean/` and `splitting/` that belongs to neither half and adds no
dependency edge between them.

*#647 merged the BODIES, K-neutrally.* Both lanes called it with their
own `SectorPredicates`, so all six K names and every recorded margin
were unchanged: 26541 recorded decisions across a boolean run and a
plane-split run reproduced byte-identically, same order, one SHA-256.

*#661 merged the NAMES* (Evan's ruling on **issue #652**, 2026-08-19,
now closed: *"go for the pool, it's internal tooling so a schema break
really doesn't matter"*). `bool_sector_{arm,reflex,straight}` and
`split_sector_{arm,reflex,straight}` became
`sector_{arm,reflex,straight}` — six census names to three, 233 → 230
for a sweep cut after it. The `SectorPredicates` parameter had nothing
left to vary and is gone, which also closes the residue #647 shipped (a
lane could have imported the name consts and re-implemented a rung under
them; the consts are now private to the module). The margins, bands,
outcomes and recorded order are untouched — only the `predicate` column
changes, and only for those six values. The committed CSVs under
`docs/k-report-data/` are left exactly as the sweep wrote them; because
the pooled names are NEW spellings rather than the 29:1-majority
`bool_sector_*`, the predicate column self-dates every row and no
committed row silently changes meaning (`docs/K-REPORT.md`, census note
2026-08-19).

One row goes red if a lane re-grows its own copy: the outside guard
walks the whole of `topo/src` at runtime, so a re-fork in a third file
is caught too, and it now also fails on any reappearance of the six
retired lane names. Its sibling — the inside guard, which ran every
shape under both name sets — was deleted with its subject rather than
left trivially green: there is no lane parameter left for the body to
branch on.

*What that reproduction is and is not.* The regenerating probe,
`crates/topo/tests/probe_s5_sectors.rs`, is **committed but not run by
CI**: it is `#![cfg(feature = "probe")]`, and nothing in
`.github/workflows/` runs `cargo test -p topo --features probe` — the
K sweep runs `-p editor-core --features probe`. So it is a *reproducible
hand-run* artifact, not a standing gate, and it can bit-rot green.
`tests/probe_census.rs` and `tests/probe_f34_review.rs` are in exactly
the same position, so this is a **class** (three uncompiled probe
suites in `topo`), pre-existing and not created here. The standing gate
over the same stream is CI's `k-lint`, which runs the full
`scripts/k_probe_sweep.sh` at three ε and lints the fresh rows.

**The rest of this finding stands.** Untouched: the `sector_face` twins,
the two forked `chord` helpers (genuinely NOT identical — splitting's
carries the C12.2 conic jet), the gate → sweep → array →
reclassification → join pipeline duplication, and the wrong-way
`splitting/join.rs` dependency with its `JoinLane::BoolPlanar`
reciprocation. Deliberately left alone on the steelman's own reasoning:
`SectorEntry` vs `BoolSector` (a CORRECT divergence) and the shared error
variants (unification means an optional field on a public API re-exported
into four crates). #647 handed back, rather than answered, whether the
two K names should become one population — with the evidence and the
`M3-LOG.md:264` counter-precedent, scheduled as issue #652 (the
steelman's K table above is corrected there and here). **#652 is
answered and closed: pool** (#661). The decisive evidence was coverage,
not size — see the K table below.

Both carry a gate → vertex sweep → sector array → reclassification →
join → finish pipeline. The halves drifted rather than unified:
`SectorEntry`/`SectorEntryKind` vs `BoolSector`;
`splitting::neighborhood::sector_face` vs `boolean::sectors::sector_face`
(the former's doc calls itself "the twin of" the latter, differing only
in error type and a sphere arm); and `SplitReduceError` /`BooleanError`
share `Band`, `CurvedBooleanUnsupported`, `CurvedEdgeUnsupported`,
`ScaffoldingOperand`, `CorruptOperand`, `CrossingInsertion` as separately
spelled variants.

Where sharing *was* attempted the dependency runs the wrong way:
`ChordJoiner`, `CutOutcome`, `SectionCtx`, `face_azimuth_window` and
`SplitJoinError` all live in `splitting/join.rs` and are imported by
`boolean/join.rs` and `boolean/solid_contain.rs` — and `splitting`
reciprocates by hosting a `JoinLane::BoolPlanar` variant plus a
`bool_planar_chord_spec` function that exist only for the boolean. A
three-way lane enum threaded through `chord_spec` (`Planar` / `Split` /
`BoolPlanar`) is the visible cost of a shared core that never got its
own home.

The vertex-neighborhood sector machinery is the sharpest instance: the
same orbit walk, the same carrier-aware chord/tangent extraction, the
same wide/reflex convex-subdivision algebra, with the predicates
**forked rather than shared** — `bool_sector_arm`/`split_sector_arm`,
`bool_sector_reflex`/`split_sector_reflex`,
`bool_sector_straight`/`split_sector_straight`. K-telemetry therefore
saw two populations for one question, and any future tolerance tuning
had to be done twice. (Both halves of that are now fixed: #647 the
bodies, #661 the names.)

(Note: `split.rs` is *not* part of this. `Body::split_edge` is an Euler
mid-edge split, a genuinely different job — only the name collides with
`splitting::split`. That name collision is S45.)

**Verdict:** ACCEPTED (Evan, 2026-08-18), together with S6 and S7 — "also
great findings!" Steelman pass commissioned: history of the backwards
dependency, whether unification is already scheduled (M9 = the C7 join
lane), and above all whether the forked `bool_sector_*`/`split_sector_*`
predicates are dimensionally identical — if so they split one K
population for nothing.
**Steelman (2026-08-18): SURVIVES — and the sharpest claim survives
exactly as written, with numbers.**

*Not a lane-isolation artifact.* `splitting/` landed as PR #55; the
boolean's `sectors.rs` was authored **6 h 45 m later**, on a branch that
had already merged the split branch. Its own module doc says *"PR 2's
geometry reused"* — what was reused was the **derivation**; the code was
re-typed.

*The backwards dependency was ratified, once, narrowly.*
`M3-PLAN.md:230` (RATIFIED #42) item 5: *"Ch. 14 join **reused** with
A↔B correspondence disambiguation"*. Item 4 — boolean reduction and
classification — carries **no reuse instruction at all**. That one
sentence is the whole origin: the plan named ch. 14's join as the shared
asset, ch. 14's join lives in `splitting/`, so the shared core was born
inside one half **by instruction**, not by drift. `JoinLane::BoolPlanar`
is likewise deliberate and argued (PR #152): the planar side of a curved
germ pair has no chart of its own, so the azimuth window must arrive by
value.

*The predicate fork is dimensionally IDENTICAL — not merely similar.*
Line-for-line, with `dir_a ↔ dir_end`: same `min` of the same two chord
norms for the arm; same `Margin::levered(u_start.cross(u_end).dot(n),
arm)` for reflex; same `.dot()` for straight; same bisectors; same spike
guard. `Margin::levered(x, arm) = x * arm`, documented as
*"bit-identical to the bare `x * arm`"*. Both arms are the same metres
from the same `edge_extent` machinery; both normals are `chart_normal *
sense_sign`. **The same computation on the same quantity under two
names.** `docs/predicate-dimension-audit.md:189` lists both families
with the same dimension column — a systematic sweep that saw both and
never noticed they were one population.

*The K cost is measured, not hypothetical* (`m7-eps-1e-6.csv.gz`).
**This table was CORRECTED on 2026-08-19 by #647's fix pass: the
original read a minimum *signed* margin as if it were an absolute one.
The corrected reading makes the case stronger, not weaker.** All six
rows, recomputed:

| predicate | samples | signed margin range | outcome split |
|---|---|---|---|
| `bool_sector_arm` | 1880 | [4.11e-2, 1] | 1880 positive |
| `split_sector_arm` | 64 | [1.95e-2, 0.941] | 64 positive |
| `bool_sector_reflex` | 1880 | [−0.25, 1] | 418 positive, 8 negative, 1454 zero |
| `split_sector_reflex` | 64 | **[0, 0]** | **64 zero** |
| `bool_sector_straight` | 1454 | [−1, −4.11e-2] | 1454 negative |
| `split_sector_straight` | 64 | [−0.941, −1.95e-2] | 64 negative |

One geometric question, two populations at **29:1**. What this passage
originally concluded — *"the 64-sample tail is the one that actually
reaches margin 0"* — is **wrong**: −0.25 and 0 were minimum SIGNED
margins, and by minimum ABSOLUTE margin both reflex populations reach
exactly 0, the boolean one **1454 times of 1880**.

The real asymmetry is **coverage**, and it is sharper than the size
ratio. **Every one of the 64 `split_sector_reflex` samples is exactly
zero**, so the splitting lane's wideness name has NO corpus coverage of
a definite convex-or-reflex verdict at all, while the boolean lane's has
**426** (418 positive + 8 negative). The two names do not merely differ
in population size — one of them is entirely degenerate on this corpus,
and that was invisible while each name was read as a single number. That
is a *coverage* argument for pooling the names, not just a bookkeeping
one, and it is the form in which the question was handed back (**issue
#652**) — and the form in which it was answered. Pooled, the rung is one
population carrying those 426 definite verdicts instead of two of which
one is entirely degenerate.

*The project already treats name/margin bijection as a correctness
property, in one direction only.* `M3-LOG.md:264` records PR #55's
review MINOR-1: two margins sharing one K name **had to be split** by
reviewer instruction. One margin under two names had never been
examined — until #652, which examined it and merged. The
counter-precedent exists in-tree and is what #652 leaned on:
`bool_planar_chord_spec` and `chord_spec` deliberately **share** the K
name `split_arc_window`, documented as *"same margins, same predicate
names"*. Both directions are now on the record, together with the test
that picks between them: whether the two names are the same computation
of the same quantity — which the split case was not, and this one
provably is.

*The wrong-way dependency is now bidirectional in fact.* A K predicate
literally named `bool_between_arc_window` is decided at
**`splitting/join.rs:1611`**.

*Drift confirmed with dates:* three cross-cutting sweeps had to edit
both files **in one commit** (S10 sense audit #155, clause-(i) `Margin`
migration #213, LB9 rename #270), and M5 PR 9's curved work landed as
two separate commits. The lanes are now unequal — splitting has the
C12.2 conic jet and refuses `Sphere`; boolean has the `Sphere` arm and
no jet.

*Weakened sub-claims:* the shared error variants are not simple
duplicates — every boolean twin carries `operand: Operand`, so
unification means an optional field on a **public API** re-exported into
four crates. And `SectorEntry` vs `BoolSector` is a **correct**
divergence: splitting classifies against one plane so resolves the side
eagerly; the boolean classifies pairwise so must retain the bound
geometry. Only the *producer* is duplicated.

*One quiet mismatch worth your eye:* `DESIGN.md:1275` describes the
crate as *"the boolean engine and its splitting/census machinery"* — the
ratified architecture names **one** engine with splitting subordinate.
The code has two engines with the dependency running the other way.

*Hidden costs, as first scanned — two of the four are now settled and
one was wrong.* (a) Merging the K names **was** a schema break in an
append-only dataset (`K-REPORT.md`'s 233-name census); it was taken
deliberately in #661 on Evan's ruling, and what the break actually cost
is written up in that report's census note (2026-08-19) — six names
out, three in, the committed CSVs left as the record, no row's meaning
changed. (b) ~267 tests across 36 files: still standing, still the bulk
of the remaining work. (c) Open issue **#561** — "the Python
refusal-tag *values* are pinned nowhere, so an enum reshape can
silently change strings the Python surface exposes" — was **checked
against this change and does not apply**: `crates/pncad-py/src/tags.rs`
maps kernel *enum variants*, contains no predicate name and no `sector`
string, and pooling reshaped no enum. (The check did find a real K-name
→ Python channel, `SelectRefusal.predicate`, and that was reported to
#561, which is its home; no sector predicate reaches it.) So #561 is
not a cost of the K-name half of S5, and the sentence above should not
be read as saying it is. (d) The sphere asymmetry still forces a
per-lane kind gate — unification **converts** the `JoinLane` cost
rather than eliminating it.

*Could not determine:* whether PR #62's implementer weighed extraction
and rejected it — no spec was committed and the log is silent, so this
reads as *"happened, then documented"*. PRs #62 and #65 have **zero
comments**; the fork was never surfaced to Evan.

## S6. FIXED by #710 — ten helpers duplicated across twelve sites, kept apart by a fork note whose own expiry condition had fired

- **Where**: the shared home is now `crates/sweep/src/swept.rs`
  (crate-level, sibling to the verbs) plus `crates/sweep/src/revolve/chain.rs`
  (revolve-internal); the copies were in `crates/sweep/src/extrude.rs`,
  `crates/sweep/src/revolve/{mod,surfaces,axis,upgrade,partial,full}.rs`.
- **Confidence**: sure
- **Verdict**: ACCEPTED (Evan, 2026-08-18), with a steelman pass that
  retracted four of the finding's claims. Executed as Track D unit **D1**.

**What was found — ten distinct helpers, twelve copies.** Each pair
named in the D1 row was extracted from `main` by brace matching and
compared under a normalisation that strips comments, path spellings
(`geom_core::`, `super::`, `geom_brep::`), the `_eff` parameter suffix
and the predicate-name literals. **Eight** came back identical to the
character: `cosurface` (532), `cap_points` (260), `arc_apex` (212),
`arc_span` (41), `turn_axis` (98), `SweptKind` (60), `decide` (39), and
— a ninth helper the row did not list — `SweptSeg::sketch_segment`
(179). Two more are identical only under a stated allowance, and the
allowance belongs in the record rather than only in the PR body:

- `face_surface_key` (106) is identical *once the return type is also
  normalised* — `ExtrudeError` against `RevolveError`. That difference
  is the substance of how it was unified, not an artifact of reading.
- `rim_spec`/`chain_spec` differ by exactly one piece of Rust sugar:
  extrude writes the field-init shorthand `place,` where revolve writes
  `place: place_eff,`. `_eff` is a formal parameter name; every caller
  passes the same argument it did before.

Sweeping for the **closed forms** rather than the names found two
further *copies* — not further helpers: `revolve/axis.rs`'s private
`apex_of(a, b, bulge)` is a **third copy of `arc_apex`**, and
`extrude.rs` had hand-inlined `face_surface_key`'s body at one site
rather than calling its own helper. Ten helpers, twelve sites;
`swept.rs` hosts the ten.

**What was unified.** All eleven, into `crates/sweep/src/swept.rs` — a
private crate-level module, deliberately *not* inside `extrude`, even
though `loft` importing extrude's copies had already made extrude the
de-facto owner. That arrangement is what produced the finding: a core
inside one of its consumers has no boundary, so `loft` could adopt it
while `revolve` kept its own fork and nothing said the two had diverged.
The fork note at `revolve/mod.rs:507` named "a shared lowering layer" as
its own expiry condition; that layer now exists and the note is deleted
rather than rewritten a third time.

Three decisions carry the retractions:

- **`SweptChord` is a trait, not a struct.** Four accessors (`a`, `b`,
  `bulge`, `kind`) let every identical body be shared while both
  `SweptSeg` definitions stand untouched, so no shared body can read
  extrude's `wall_sense`. Note the precise scope: this keeps the bit
  out of **revolve**, which has its own record. It does not keep it out
  of **loft**, which still imports `SweptSeg` and `swept_segments` from
  `extrude` (`loft.rs:69`) and therefore carries the bit. Loft never
  reads it, so there is no defect — but that is loft's habit, not a
  structural guarantee, and the trait bought the guarantee only against
  revolve. `sketch_segment` is a free function over the accessors
  rather than a provided method for the same reason: a provided method
  is one an impl may override, which would put the body back to two.
- **Predicate names are a parameter.** `cosurface` takes a
  `CosurfaceNames { lines, arcs }`. The K-telemetry premise held, and
  the conclusion was checked rather than assumed: the string-literal
  multiset of `crates/sweep/src` is byte-identical before and after —
  60 literals either side, re-verified against `origin/main` at
  **7eaf43b7** after the base moved — so no K row moves.
- **`face_surface_key` returns `EulerOpError`.** A stale key is an
  operator fault and nothing else, so each verb's `?` lifts it through
  the `From<EulerOpError>` it already has. The generic-or-lossy bind
  this finding predicted for shared fallible helpers across three closed
  error enums did not have to be paid, at any of the twenty call sites.

**The duplication the finding missed was real, and had a third copy.**
`partial.rs:73` and `full.rs:73` built the same `mvfs`-anchored chain;
so did `partial.rs`'s hole-ring chain, grown from a `kemr` ring instead
of the seed loop. All three now call `revolve/chain.rs::build_chain`,
parameterised on the loop, its anchor vertex and the closing face's
surface.

**Deliberately NOT unified.** `SweptSeg` — the two records differ (207
vs 178 normalised chars) by exactly the `wall_sense` bit whose three
per-verb rules are the M5 S11 fix. `strut_spec` — 223 vs 350 chars,
different arity, `ExtrudedPoint` on a line against `RevolvedPoint` on a
circle; a name collision. `swept_segments` — 813 vs 643 chars, and only
extrude has a reversal arm. `full::build_lamina` and the `let _ = k;`
inference, both retracted by the steelman and both confirmed here.
`skin.rs` was swept for the same closed forms and shares none of them:
it discretises arcs in `f64` with a signed `θ = 4·atan(bulge)`, a
different derivation that happens to look similar.

**One behavioural delta, and it is a reordering.** Folding
`partial.rs`'s outer chain into `build_chain` hoisted the cap-plane
computation above the chain: `main` ran `mvfs → mev×(n−1) →
cap_points/newell_plane → mef`, the branch runs `mvfs →
cap_points/newell_plane → build_chain`. Both moved statements are pure
and touch no entity, so the solid is identical. Two consequences follow
anyway and are recorded here rather than left to be rediscovered: a
`RevolveError::CapPlane` now pre-empts an operator fault the chain
would have raised first (unreachable — `profile`'s
`vertex_separation` already refuses coincident chord endpoints), and
the **order** in which K samples are emitted changes, because
`newell_plane` emits `newell_plane_residual` and `mev` emits
certification samples. Nothing gates on emission order, and the
byte-identity check above is a multiset comparison and therefore
order-blind, so this costs nothing — which is why it is written down
rather than relied on silently.

**What the change is guarded by.** No guard was added for the ten
shared bodies: every row that covers them was already green, and the
compensating gain is structural — a defect in what was extrude's copy
is now under revolve's suite too. One guard **was** added, at the
weakest of the three `build_chain` callers. `mass_props.rs` carries
exact closed-form volume and area rows for the washer, ball, cone and
three wedges, but had none for a **partial revolve with a hole** —
which is the `kemr`-ring path, the third copy this unit discovered, and
whose only acceptance was `revolve_partial.rs:111-125`'s counts plus
`signed_volume(&body) > 0.0`, the monotone-in-one-direction shape Q3
exists to catch. `mass_props::holed_wedge_matches_theta_scaled_closed_forms`
adds the exact row (V = 3π, A = 12π + 6, both hand-derived by Pappus
and both binding: perturbing the area expectation by 1e-9 reddens it).

**What the sweep could not match** — three blind spots, one of which
was demonstrated on this very unit:

- A copy that changed *representation* rather than spelling. No token
  pattern relates `skin.rs`'s `f64` `mul_add` arc arithmetic to
  `SweptKind::Arc`; nor would a chain built through a different
  operator vocabulary (`mev_line` plus `set_edge_curve` rather than
  `mev` with a spec) match `build_chain`'s shape.
- **Prose that states a sequencing constraint.** S7's verdict carried
  *"W2d (S6) follows the retirement, not beside it: both are in `sweep`
  and will collide"* — a sentence this fix invalidates, in a finding
  whose identifier this unit never touched. An identifier-scoped sweep
  structurally cannot see it (Q4's class). It is corrected at S7.
- **A bare delegation.** A funnel that is nothing but
  `geom_core::k_stats::decide(name, margin, band)` has no distinctive
  token to grep for, which is why the third copy in the same crate
  survived this pass — see residue (d).

**Residue.**

(a) `extrude`'s and `loft`'s lamina chains are the same operator
sequence as `build_chain` and still stand apart from it. `build_chain`
returns a concrete `EulerOpError` — not "error-agnostic", but not tied
to a verb's enum either — so what remains is passing a placement pair
instead of an `AxisFrame` and moving the module to the crate root.

(b) `loft.rs:517`'s `map_err(|_| LoftError::SectionStructure)` — the
live regression the steelman found — is untouched, being a loft-side
error-mapping bug outside D1's scope; it is more visible now, since the
discarded error is spelled `EulerOpError` at the call site.

(c) **A half-fix on a class, stated as such.** `sweep/src` held three
copies of the classification funnel; this unit collapsed two and left
the third, `sweep/src/fillet/mod.rs:76`, identical to `swept.rs`'s down
to the parameter names. The convention is **one funnel per crate** —
re-swept against `origin/main` at 7eaf43b7, the workspace has exactly
one each in `topo` (`validate.rs:266`) and two in `geom-brep`
(`enters.rs:279`, `dihedral.rs:103`), so what makes `sweep` anomalous
is having a *second*, not having one. **`sweep/src/fillet/` is D2's**,
so it is handed over rather than widened into here. Until it goes,
`swept.rs`'s own doc names itself the funnel of the shared lowering and
of `extrude` and `revolve` — and says plainly that it is not the
crate's only one — rather than claiming to be *the* funnel.

(d) **The K-report harness does not run**, so the "byte-reproduce the
CSVs" provenance behind `docs/K-REPORT.md` is currently unreproducible
and the byte-identity check above had to be made statically rather than
by running the instrument. Pre-existing, unrelated to this diff, and
larger than it: raised as Track D row **D15**, diagnosis-first —
**diagnosed and fixed by #718** (a `#101` migration `k_report.rs` was
missed by, plus a rotted command line; K-REPORT's provenance sentence
corrected there, and the committed CSVs left as cut).

(e) **And the funnel bypass is a three-member class, not the one site
first recorded.** Three sites reach past a funnel to
`geom_core::k_stats::decide` directly: `revolve/tube.rs:29`,
`extrude.rs:886` (`tangent_second_order` — in the file that imports the
funnel four lines from the top) and `loft.rs:321` (`loft_stacking`).
Equivalent today, since every funnel is a pure delegation; they are the
places a predicate name could stop being funnel-visible if a funnel
ever stopped being pure.

(f) **The shared home holds one member of S34/S57's class**, and it
holds it once instead of twice. `swept.rs`'s `face_surface_key` is the
`get_face` → dangling-refusal → read-a-field shape that #697 relocated
on the rule *a door lives in the crate whose types it reads* — it reads
`topo`'s. It is the weakest member: one lookup deep, and it returns a
`SurfaceKey` the verbs feed straight back into `FaceSurface::Shared`
and `EdgeGeometry::IsoCurve`, so it is a build-side key lookup rather
than a read-back of geometry, and `topo::readback` offers no door for
it (`face_pose` resolves through to a `Pose`). Deliberately not moved:
`topo/` is outside this unit's scope, and the honest change is one
`topo` door, not a fourth home. What this unit did do is take the site
count from **two to one** — `extrude.rs` and `revolve/upgrade.rs` each
had a copy, plus a hand-inlined third. Recorded against **S57**, whose
"Where" list does not name it.

**Observations, recorded and deliberately not acted on** — each may be
boundary-forced, and none was investigated far enough to say:
`SweptKind` is close to `profile::SegmentKind` but sits on the far side
of a crate boundary and carries swept-traversal, not canonical,
orientation; and the sagitta closed form has two more instances outside
`sweep`'s reach — `profile/src/seg.rs:149`, which is its origin, and
`skin.rs:298-307`, which self-declares as *"the profile crate's ratified
bulge closed forms, verbatim"* at a different scalar.


## S7. FIXED by #688 — two complete fillet assembly implementations, and the older one shadowed the newer

- **Where**: `crates/sweep/src/fillet/build.rs`, `surgery.rs`
- **Confidence**: sure

**FIXED by #688**, executing D3 below. `fillet_surgery` is the fillet
assembly and its front door is the only one: **901 lines of
whole-body-exclusive code plus 63 lines of its in-crate tests** deleted —
landing on the steelman's ~890 correction, not the original ~1200 — with
the `Plan`/`Build`/`Runway` stack, the eleven unreachable `unsupported(…)`
refusal strings, `build.rs:213`'s dead `Err(other) => Err(other)` arm, and
`build.rs:26`'s *"kept (not subsumed) … bit-preserved"* sentence going with
it. The five shared helpers (`face_cycle`, `vertex_faces`, `outward_of`,
`octant_chart`, `corner_convexity`) stay and are called from the surgery.
`FilletNaming::supports` was **assessed rather than assumed**: dead, and
deleted *with* its consumer rather than left as a permanently-empty loop —
S11's shape, refused. The three naming tests were rewritten, not deleted;
the rewrite asserts a **mint/survivor partition** stronger than the claim
it replaced.

**What the goldens actually did, because the first account was wrong.** The
PR's first explanation said chart normals moved and the face `sense` bit
compensated. Review re-derived it from both files: the multiset of face
surfaces — type, location, axis, **ref-direction**, radius, `same_sense` —
is *exactly identical*, and **no `sense` bit differs anywhere in either
file** (`ADVANCED_FACE` 26/26 `.T.`, `EDGE_CURVE` 48/48 `.T.`). What moved
is the **edge carriers**: 15 of 24 corner-arc `CIRCLE` placements
re-oriented and 13 of 24 trimline `LINE` records re-based, same curves as
point sets, and **all 48 `EDGE_CURVE` ordered `(v1,v2)` pairs differ against
an identical unordered multiset**. That vertex order *is* the compensation.
The mechanism: the surgery fixes a carrier's parametric direction from the
face-walk that reaches it first, where the retired door derived every
carrier from a plan in edge-arena order. Volume moved by **ulp distance
exactly 1**.

**The evidence is now mechanical, which is the transferable part.** A
regenerated byte-golden cannot be its own evidence — the change that needs
checking is the change that rewrote it. `filleted_die.probe.py` (on the
existing `check_step.sh` sidecar hook, Part 21 parsed with the stdlib, no
CAD kernel) now checks four kernel-side facts every run: every face's
outward normal points away from the body centroid; every `EDGE_CURVE` is
used exactly twice in opposite directions with `ORIENTED_EDGE`/`FACE_BOUND`/
`same_sense` folded by XOR; every planar support winds CCW about its own
outward normal at `(L−2r)² = 0.5776`; and every arc sweeps exactly 90° in
its own frame. Verified to go red under a flipped `same_sense`, a flipped
`ORIENTED_EDGE`, and a moved vertex. Also recorded against the acceptance
row that read as evidence and was not: the FreeCAD job's orientation
guarantee comes from `isValid()`, **not** from the volume assertion — the
die is `[0,1]³`, so three support planes pass through the origin and
flipping a face whose plane contains the origin is volume-invariant.

**The sweep had to be run outward, and inward was not enough.** The PR's
first pass asked *where does this code's vocabulary appear*; nobody asked
*who cited this code*. Three live registers did: `PERF-SCAN`'s finding 12
(an **open** row whose prescribed fix could no longer be applied — now
struck as RETIRED, subject deleted) and its `mint_pcurves` site list
(10 → 9, re-derived from the tree rather than decremented), plus this
report's own S15 and S19 rows. Stale references in `editor-core/src/eval/`
are **#693** — three sites, of which the one a symbol-scoped sweep can
never reach quotes the deleted predicate *verbatim while naming no door and
no deleted symbol*. **Blind spot, stated:** a register row naming code in
prose with no path and no symbol has no pattern; issue and PR bodies are
registers but are not greppable from the tree.

**Verdict:** ACCEPTED (Evan, 2026-08-18), together with S5 and S6 — with a
pointed note on the *justification*: "yikes, trying to keep outputs
bit-preserved is such a cause of issues; I think I have a note to never
use it as a justification in a memory now, so hopefully this predates
that memory."

Checked: **no such memory exists yet.** The closest is
`memories/demo-purpose.md` (2026-08-09) — "byte-identity soft for
improvements, kept for mechanical migrations" — which is the right
principle but scoped to demos. `git blame` puts `build.rs:26` at
2026-08-11, i.e. two days *after* that ruling, but the blame result is a
`^`-prefixed traversal boundary so the real authoring commit may be
earlier; the steelman pass is dating it precisely.

Two follow-ups arising: (a) is anything actually pinning those M5 bytes,
or is the justification defending nothing? (b) worth generalising the
demo-scoped byte-identity note into a repo-wide rule, distinguishing
load-bearing determinism contracts (D2/D9) from defensive
golden-churn-avoidance.
**Steelman (2026-08-18): SURVIVES IN PART — dated precisely; the
"defending nothing" framing is wrong; the subset claim is provable; and
the repo already caught this exact claim once.**

*The date, settled.* Two commits, both 2026-08-03, both in PR #171:
`701e1b5a` (15:59) shipped the two-door routing with only an inline
parenthetical; `11f3efb0` (17:28) rewrote the module doc and added
today's `build.rs:26`. **The `^4c8a757` blame boundary was a merge
commit, not the author.** So it predates `memories/demo-purpose.md`
(2026-08-09) **by six days** — your hope was right after all, though
that memory is demo-scoped and would not have bound it anyway.

*Happened, then rationalized — with the commit order as the tell.* The
routing shipped first; the *"kept (not subsumed)"* paragraph was written
89 minutes later, when the module doc had to be reconciled with what the
code now did. **PR #171 was opened 06:28 UTC and self-merged at 06:55 —
27 minutes, zero comments.** `DESIGN.md:503` marks the frontier
DISCHARGED and describes only the surgery; **it does not mention a
whole-body door at all.**

*The repo has already formally recorded this claim as unfounded, once.*
The M6-5 PR-2 adversarial review (finding F-D) rejected a test that
claimed to pin the bit-preservation, and commit `252a4126` wrote the
correction into the test: *"It does NOT, and cannot, check
bit-PRESERVATION across the change… That claim was executed out-of-tree
instead."* The fix pass corrected the test and **left `build.rs:26`
untouched.**

*But "defending nothing" is wrong on the facts.* Three artifacts pin
whole-body bytes: `step-export/tests/fixtures/filleted_die.step` is
byte-golden and asserted; `filleted_die.expect` carries
`KERNEL_VOLUME_MM3=965230999.476531`, asserted by **byte equality
against the live kernel**; and `filleted_die` is one of three tight STEP
round-trip fixtures. A different summation order fails CI today.

*The subset claim is provable in three steps, and no counterexample
exists.* Whole-body requires every vertex trivalent and every edge
requested ⇒ every vertex has three requested links ⇒ `walk_chains`'
junction closure only chains through **two**-link vertices ⇒ every
whole-body chain is a single-link open chain, which is exactly surgery's
front door. Ring clearance is vacuous (whole-body refuses faces with
rings). **Surgery's door strictly contains whole-body's.** Corollaries:
all **eleven** `unsupported(...)` sites in `whole_body_links` are
unreachable as user-visible messages, and `build.rs:213` is dead.

*Two corrections:* "~1200 lines" overstates — `face_cycle`,
`vertex_faces`, `octant_chart` and `outward_of` are **shared** with the
surgery, so the whole-body-exclusive part is **~890 lines**.

*What the pins actually cost is a regeneration chore, not a contract.*
Both goldens are explicitly regenerable (*"Regenerate deliberately
with…"*), the FreeCAD volume is rounded and door-independent, and the
rebuild-latency series is *"DELIBERATELY NOT A GATE"*. So the honest
form of `build.rs:26` is *"kept because retiring it would require
regenerating two goldens and re-running the FreeCAD acceptance once"* —
a real cost, but not the one the sentence claims, and not a reason a
reader would accept for a second 890-line implementation.

*The largest unknown, and it is cheap to close:* **the surgery has never
been run on a whole-body input, not once.** `fillet_surgery` is
`pub(super)` with exactly one call site, behind the whole-body door. The
proof above is about a *predicate*; whether the construction succeeds
when every boundary vertex is a corner and every source edge dies is
unknown. The experiment is one line — swap the arms at `build.rs:205` —
plus `cargo test -p sweep --test all`. **This finding's disposition
should probably wait on it:** if the surgery fails on a cube, the two
doors are not redundant and S7 collapses.

*Also real:* three naming tests are whole-body-specific **by design**
(the fresh-arena door retires every source entity; the surgery leaves
supports in place), so they need rewriting, not deleting — and
`FilletNaming::supports` becomes dead.

### D3 EXPERIMENT RUN (2026-08-18): the surgery door handles the whole-body input

The experiment §D's D3 row asks for has been run. Arms swapped at
`build.rs:205` so `fillet_surgery` receives every input — including the
whole-body shape it had never once been given — then reverted. Nothing is
proposed here; this records only the result.

**The construction succeeds.** `fillet_edges` on a cube with all twelve
edges requested returns `Ok` through the surgery door. The largest unknown
the steelman named — *"whether the construction succeeds when every
boundary vertex is a corner and every source edge dies"* — is closed: it
does.

**And it builds the same solid.** Exported to STEP and compared against the
committed `filleted_die.step` golden as an unordered multiset:

| Check | Result |
|---|---|
| Entity-type multiset (625 entities) | **identical** |
| Distinct `CARTESIAN_POINT` coordinates (51) | **identical sets** |
| Total `CARTESIAN_POINT` count (99) | **identical** |
| Circle / sphere radius multiset | **identical** |
| Kernel census (F / E / V) | **identical** — 26 / 48 / 24 |
| Certified volume | `965230999.4765309` vs pinned `965230999.476531` — **adjacent f64s** |

What differs is emission *order*, and how the two doors distribute
duplicate points across use-sites (twelve coordinates shift multiplicity,
3→2 and 1→2, netting zero). **No coordinate, radius, or entity exists in
one file and not the other.**

**The retirement price, measured rather than estimated.** With the arms
swapped:

- `cargo test -p sweep --test all` — 376 pass, **1 fails**:
  `m6_5_fillet_naming::the_whole_body_door_records_every_entity_it_mints`,
  on the `supports` row (0 vs 6). This is precisely the by-design
  difference the steelman predicted — the fresh-arena door retires every
  source face and writes a `supports` row per face; the surgery leaves the
  support in place, so the source key survives and no row is written. It
  needs rewriting, not deleting.
- `cargo test -p step-export` — 50 pass, **2 fail**: the byte-golden
  fixture and the `KERNEL_VOLUME_MM3` sidecar. Both are the regeneration
  chore the steelman priced, not a contract breach: the volume moves one
  ulp, the census does not move at all.

So the subset claim holds **constructively**, not just predicately, and the
cost of retiring the whole-body door is one naming test rewritten, two
goldens regenerated, and one FreeCAD acceptance re-run. **S7 does not
collapse.**

*Still open, and still Evan's:* whether to actually retire the ~890
whole-body-exclusive lines. This entry closes the blocking unknown and
confirms the price already quoted; it does not make the call.

### D3 DECIDED (2026-08-19): retire the whole-body door

**Evan, 2026-08-19: retire.** The price is accepted as measured above — ~890
whole-body-exclusive lines deleted, one naming test rewritten, two goldens
regenerated, one FreeCAD acceptance re-run. Nothing new was learned between
the experiment and the call; the experiment was the whole question.

Three consequences the executing lane inherits:

- **`fillet_surgery` stops being `pub(super)` behind a door.** It becomes the
  fillet assembly, and its front door becomes the only one — which means the
  ~890 lines' eleven `unsupported(...)` refusal strings, all provably
  unreachable today because the caller discards them, go with the code rather
  than being re-homed. `build.rs:213`'s dead `Err(other) => Err(other)` arm
  goes too.
- **`the_whole_body_door_records_every_entity_it_mints` is rewritten, not
  deleted.** Its `supports` row (0 vs 6) is the by-design difference the
  steelman predicted: the fresh-arena door retires every source face and
  writes a `supports` row per face; the surgery leaves the support in place,
  so the source key survives and no row is written. `FilletNaming::supports`
  becomes dead and should be assessed rather than assumed.
- **Sequencing.** #640 (H9 / S50) is editing `fillet/build.rs:692`, which sits
  *inside* the door being retired — the retirement makes that half of #640
  moot, so the two must not run concurrently. **The W2d (S6) half of this
  constraint is spent**: S6 ran as Track D's D1 and landed as **#710**,
  after the retirement and without colliding with it — it touched
  `extrude.rs`, `revolve/` and a new crate-level module, and no file
  under `sweep/src/fillet/`. What remains of the sequencing note is the
  #640 clause.

Also worth recording against `build.rs:26`, the sentence that kept the door
alive: `memories/output-stability-as-justification.md` now names exactly this
shape as its tell — *"a comment saying code is kept, retained, or not
subsumed because its output would otherwise change"*. The goldens were the
regeneration chore the steelman priced, not a contract.

## S8. The fitted (rung-3) pcurve lane has no producer anywhere in `src`

**FIXED by #707 — the lane stays; the sentence the finding rested on was
the false thing, and the frontier now names itself.**

**What the sort found: KEEP.** The steelman (2026-08-18) graded the lane
PLANNED-but-unscheduled with three named consumers, and removing it would
reopen a design ratified three days before the scan (`PCURVE-UNIFY-DESIGN`
U2 defines `General` as certifying *"at the honest Fitted grade"*).
Removal is also all-or-nothing — `certify_fitted` is the variant's sole
constructor — and would falsify ratified exit-walk rows and six test files.

**What was false.** The `Copy` price. `pcurve_cache.rs` blamed `Fitted`'s
`Arc<NurbsCurve2>` for costing `Pcurve`/`PcurveCache` their `Copy`.
Re-verified on today's tree: `Pcurve::IsoArc` carries `breaks: KnotVector`
and `KnotVector` is `Vec<f64>`-backed, so either variant alone denies
`Copy` and deleting `Fitted` restores nothing. The residual cost is three
`Pcurve` clones (`topo/src/pcurves.rs:269`, `:836`, `:1084`) plus one
`PcurveCache` clone (`topo/src/boolean/combine.rs:343`, transient by its
own comment). The finding's cost column was therefore zero, and the
finding was built on it.

**Also false: "machinery with no caller", by ~10×.** Only `certify_fitted`
is callerless. `PcurveCache::recertify` dispatches
`Fitted → run_fitted_checks`, dispatched by the tier-3 validator at
`topo/src/pcurves.rs:1379`, which is why `validate_pcurves` carries the
`PcurveFittedLane` bound at all. **That is static reachability, not
execution**, and the first draft of this fix said "runs in production",
which is the same overclaim in the other direction: since
`certify_fitted` is the variant's sole origin, no body this workspace
builds holds a `Fitted` cache and the arm never executes in-tree. It is
live for an out-of-tree caller attaching one through
`topo::Body::attach_pcurve`, and the code now says exactly that.

**What was true.** The docs read as *shipped* where the truth is that the
certified route exists and no kernel constructor mints one — which #176's
merged body said and the code did not.

**What the frontier now says about itself.** `certify_fitted` states that
it has no `src` caller, that it is the lane's only callerless item, and
names its three consumers in decreasing firmness: (a) mint-side wiring of
the general-circle route for the oblique-trihedron octant, named as open
in `DESIGN.md` and **in no milestone plan and no carried-items register**
(re-verified 2026-08-20); (b) U2's `General` arm; (c) the germ-chord lane.
`Pcurve::Fitted`'s own docs point at that door.

**Coupled with S9 in the removal direction**, which neither finding wrote
down: most of S9's inventory is check-5-only only if this lane goes too.

**Owed elsewhere: #250 needs a row** for (a). Not edited by this unit; the
obligation is recorded at the `mint_pcurves` claim site, at
`certify_fitted`, and in §D below.

**Two sites read as shipped through the first pass, and the sweep is why.**
`pcurve_cache.rs`'s `UnsupportedCarrier` refusal told a **user** that *"The
general fitted/marched rung is live — store the chart image as
`Pcurve::Fitted`"*, instructing a caller toward a rung no in-tree
constructor mints; and `topo::pcurves`' module header still said in the
present tense that the sphere's general circles *"route through the fitted
lane"*, contradicted twenty lines into the function it describes. Both are
fixed. **The sweep missed them because it was scoped to the phrases being
removed** — a literal match over `crates/*/src` and `crates/*/tests` finds
prose that *quotes* the false claim, and neither of these quotes it: one is
a user-facing `write!` string, the other a present-tense paraphrase. The
same blind spot left §A's roll-up saying *"S8 is why `Pcurve` is not
`Copy`"* — the exact falsehood this unit exists to kill, in the document
being edited. Sweep for the CLAIM, not for the sentence.

## S9. The trim-containment limb is vacuous on both production paths

**FIXED by #707 — the mechanism claim was right, the justifying comment was
false, and the limb's documented status is now "precondition on the public
door".**

**What the sort found: KEEP.** The steelman graded this NEITHER planned nor
superseded. Containment is an identity on both production paths, and there
is no planned attach path — content-keyed cache transfer is banked
(`DESIGN.md:1576`) and the one `src` site copying a cache between bodies is
explicitly transient.

**What was false.** `pcurves.rs`'s *"where this limb has teeth"*. It
conceded mint-time vacuity and argued the check bites at re-certification;
`validate_pcurves` builds its window as the hull of exactly the stored
caches it then re-certifies — the same self-referential construction. The
comment also **post-dates the code that falsifies it**: `validate_pcurves`
landed in `9e80547f`, the comment in the same PR's fix pass (`a842090b`),
written in response to a reviewer's mint-time-vacuity note against code
already in the branch.

**What was true, and is now what the code says.** Check 5 is a
**precondition the caller supplies**, vacuous on both of `topo`'s callers.
The design still buys something and the comment says what: check 5 is the
cache's only **branch** constraint — on a periodic chart a τ-shifted pcurve
certifies every other check identically. Those two facts now live in one
home each, split by which crate can know them: the vacuity in
`topo::pcurves`, which owns the callers, and the branch constraint in
`geom_brep::pcurve_cache`, which owns the checks. The first draft of this
fix put both in both, and the geom-brep copy asserted a fact about its
consumer crate across a layering boundary, held by nothing.

**This finding repeated itself inside its own fix.** S9's defect was a
justification written to survive a review note, asserting teeth it did not
have. The first replacement removed that clause and inserted a *new*
empirical one of the same shape — *"the `TrimEscape` rows drive it from the
tests' attach path"* — which is false: there are **zero** `TrimEscape`
assertions under any `tests/`, and the tree's only one
(`geom-brep/src/pcurve_cache.rs:3918`) is a geom-brep unit test handing
`certify` two hand-built windows. One row, not rows; not an attach path.
The clause is gone with nothing in its place. **The lesson, general:** the
replacement for a false justification must not be another justification.

**The inventory claim did not survive.** Only `trim_containment`,
`TrimEscape`, `ChartWindow::hull`, the `window` threading and two hull
loops are check-5-only. `ChartWindow` is `chart_box`'s return type;
`chart_box` feeds check 2's azimuth headroom; `azimuth_lever` feeds
harmonic check 2 and check 4's snap slack; `chart_arms` feeds the fitted
lane's check 2. **They die only if S8 is acted on too** — the two findings
are coupled in the removal direction, recorded here and at S8 because
neither finding wrote it down.

## S10. The schema migration mechanism is dead, and fourteen versions are ceremony around it

**FIXED by #707 — the mechanism is ratified doctrine and stays; the ledger
had drifted worse than the finding recorded, and is repaired.**

**What the sort found: KEEP the mechanism.** The empty step table is doubly
ratified — `docs/archive/M4-LOG.md:1933` (*"The migration MECHANISM stays
(an explicit, currently empty step table)… the next non-breaking format
change adds its step there"*) and **LQ7a** at `LIBRARY-DESIGN.md:332` (*"NO
backwards-compatibility machinery of any kind before release"*). Both
re-verified. The finding does not survive; the ledger does. Note **LQ7b**:
version numbers reset immediately before release, so all fourteen numbers
are planned to be thrown away.

**What was false, twice over.**

*First: v12 did not skip its entry.* A full Version 12 entry was written
and **merged to main in #571** (recoverable at
`git show 3931d68:crates/editor-core/src/persist/mod.rs`). It was then
**deleted by #583's conflict resolution** — the v13 bump kept its own
colliding paragraph and dropped the loser. Not an entry never written; an
entry written, merged, and removed.

*Second: this ledger never had a tripwire to lose.* The finding reads
`memories/schema-claim-discipline.md` as calling **this** prose the
tripwire for the same-number race. It does not. The memory's tripwire is
*claim prose in the shared dispatch ledger*, and it names that ledger —
`docs/MODEL-AB-LOG.md`, where a second claimant collides at dispatch time.
The version ledger in `persist/mod.rs` is the memory's *other* half: the
place the number and its reasoning are recorded, checked **by eye** at the
final re-merge. So when #583's resolution dropped v12's entry, nothing was
pointed at this file and nothing fired. The right account is neither "the
entry was skipped" nor "the tripwire failed" — it is that a document
everything assumed was guarded had no guard at all.

**What was fixed.** v12's **format** paragraphs are restored from
`3931d68` — what the version means and why it is a clean break. Its
eleven-line merge-race post-mortem is **not** restored: this finding named
exactly that prose as the ledger's problem, and reinstating it verbatim
would have been fidelity to what the finding condemned. `migration_step`'s
clean-break list, which stopped at `10 → 11`, now runs through `13 → 14`,
and leads with *empty by RULING, not by omission*, citing LQ7a in place of
*"the mechanism stays because it costs nothing"*. The `# Schema history`
header, which narrates v1–v2 and drifted twelve versions ago, now says it
covers only those two and points at `SCHEMA_VERSION` for the rest — the
file narrates its versions in three places and this makes clear which one
is the ledger.

**And the ledger now has the guard it never had** —
`crates/editor-core/tests/schema_ledger.rs`, beside the other per-version
suites rather than inside the file S38/L2 will trim. Three rows, each
proven red for its own reason:

- *an entry for every `n` in `2..=SCHEMA_VERSION`* — excising the **v13**
  entry (mid-ledger, not the tail) fails naming `[13]`;
- *a golden per version whose header carries that version* — relabelling
  `v7_golden.cad` to `schema: 6` fails naming it. This is the semantic
  anchor: a deleted golden is already a compile error from the existing
  suites, but a **mislabelled** one is caught by nothing else;
- *no golden above the constant* — planting a `v15_golden.cad` fails,
  catching a rolled-back bump or a golden written under a number main
  never took.

Stated at the suite: it cannot tell whether an entry describes the format
the number shipped, and it says nothing about whether `SCHEMA_VERSION`
holds the *right* number — that remains the by-eye read at the final
re-merge, unautomated. Q6: the ledger's completeness is a mechanism now,
not an assumption resting on a deleted pointer.

**Deliberately not done.** The ledger is not trimmed and the five
byte-identical goldens are untouched: comment trimming is **S38 / L2** and
comes last on purpose. The drift was fixed, not the size.

**The cited memory rotted between the scan and its execution.**
`memories/schema-claim-discipline.md` was created at `151afc2b` and
deleted at `dd6d1990` (*"cut unnecessary and harmful prescriptions from the
orchestrator reading path"*) on **2026-08-18** — the day this scan was
written — in a deliberate five-memory pruning pass that also added the
memory-writing criteria to `cad-working-style`. So the citation was live
when made and dead when executed: **S39's own genre, occurring inside the
scan**. The pruning is not this track's to relitigate and no restoration is
proposed; what the memory carried about this file is now held by the guard
test instead of by a pointer.

## S11. Substantial machinery shipped as live, with no producer (roll-up)

- **Confidence**: sure for each row unless noted

S8, S9 and S10 are the largest instances; these are the rest. None are
stubs — all are built, documented, and in several cases certified and
adversarially reviewed.

| What | Anchor | Note |
|---|---|---|
| `bspline_green_integral` + its whole `DerivLadder` substrate | `props/quad.rs:706`, `:629` | Module doc at `:42` claims the patch flux engine consumes it; the patch engine runs a separate near-parallel copy |
| `pcurve.rs`'s ellipse constructors | `geom-brep/src/pcurve.rs:107`, `:219` | Superseded by `pcurve_cache`, which says so in its own docs; the file keeps the name a reader reaches for first |
| `hull.rs` — 8 of 10 public fns test-only | `spline/hull.rs:151`, `:196`, `:211`, `:297` | `span_hull_rational` documented as returning the same hull as `span_hull` — a wrapper whose body is a precondition check |
| `boxes` modules in **both** geom crates | `geom-curves/src/boxes.rs:29`, `geom-surfaces/src/boxes.rs:25` | Zero production consumers, while `topo/boolean/boxes.rs` carries a KNOWN GAP note saying "the sound constructor exists unused" (see S16) |
| `Node::Sweep` | `eval/wire.rs:1524` | Full vocabulary entry — variant, 2 `SlotId`s, content tag, `inputs`/`slots`/`expr` arms — for an op with no success path |
| STEP cylinder recognition | `recognize.rs:257`, `:794` | ~90-line estimator whose own test `p7_exact_cylinder_envelope_is_honest` asserts an exactly cylindrical patch must **not** promote; `PromotedKind::Cylinder` asserted as an outcome nowhere in `src` or `tests` |
| `ProfileError`'s five fillet variants | `profile/src/validate.rs:411`–`:507` | Constructible only from `test_support.rs`, behind the `test-support` feature; `Profile::validate` cannot produce any of them |
| `Mat2` / `Affine2` | `geom-core/src/linalg.rs:135` | Only mentions outside `linalg/` are the re-export and one review test; `Vec2`/`Point2` are heavily used, so it is the 2-D *linear-map* half specifically that is dead |
| `PatchContact` | `boolean/mod.rs:214` | No producer; `ContactRecords.patches` and its face-lineage chase in `remap_contacts` are paths no run reaches. Deliberate per its doc ("the vocabulary is complete") |
| `trace_plane_nurbs_uncertified` | `ssi.rs:970` | Demonstration entry point in `src/`, test-only, re-copying ~40 lines of `plane_nurbs_ssi`'s setup |
| `FlipSet::flips_on_path`, `flips_at` | `resolve/vdiff.rs:181`, `:150` | Public, documented against a spec line, no callers; the one consumer that wants exactly this calls the raw primitive underneath |
| `jet.rs` cone and torus Taylor arms | `ssi/jet.rs:274` | Own comment: "the cone arm refuses at both the enclosure and the certificate today, so nothing depends on it yet" |
| `CornerBall::surface`, `corner_contact_circle`, `trimline_description` | `fillet/blend.rs:56`, `:301`, `battery.rs:1100` | The first builds a chart both production callers discard and replace; the other two have no consumer at all |
| `PairSolve`, `CLASS_DEFERRAL` | `mate/solve.rs:66`, `mate.rs:245` | `pub`, re-exported, never constructed or read |

`DESIGN.md` names building reviewed machinery with no consumer as "the
dead-code pattern M5's reviews repeatedly punished", which makes the
size of this table the finding rather than any single row.

**Verdict:** ACCEPTED, SORT REQUIRED (Evan, 2026-08-18). "A lot of these
are to support planned work, so that'll need to be sorted out from the
ones that are superseded." Steelman pass commissioned to sort every row
into PLANNED (naming the scheduled consumer) / SUPERSEDED /
DELIBERATE-FRONTIER / GENUINELY DEAD, with evidence. **This table should
not be acted on until that sort lands.**
**Steelman (2026-08-18): sorted. Most rows are frontiers with named blockers,
not dead code — but four need a decision and one changed while this scan was
being written.**

| Row | Sort |
|---|---|
| `bspline_green_integral` + `DerivLadder` | **DELIBERATE-FRONTIER**, blocker named at `topo/src/props.rs:581` (it is S8's missing producer; rational half banked at #390/#453). But the `quad.rs:42` liveness claim is **SUPERSEDED** — see S39. |
| `pcurve.rs` ellipse constructors | **SUPERSEDED**, in writing, with a measured deviation (`pcurve_cache.rs:32`). |
| `hull.rs`'s 8 unused fns | **Split**: the rational half is spec-commissioned with a banked lane (#390/#453, register row in #250); `domain_hull` and `derivative_span_hull` have **no named consumer**. |
| `boxes` in both geom crates | **PLANNED**, three named consumers — and one is a *correctness* item: `PERF-SCAN-2026-08.md:208` Tier A finding 1 names `nurbs_surface_aabb` as the fix for S16's unsound `face_box`. |
| `Node::Sweep` | **DELIBERATE-FRONTIER → now PLANNED with a landed first half.** LQ3 ratified (#362); the door landed (`compose_chain`); the `wire_sweep` discharge is explicitly fenced out of U4A as its own later unit. |
| STEP cylinder recognition | **DELIBERATE-FRONTIER** — the refusal is the honest result and the module doc says so in advance; the algebraic tightening is banked but tied to **no issue or plan**. |
| `ProfileError`'s five fillet variants | **SUPERSEDED — and now fully orphaned.** #377 closed 2026-08-18 by PR #608; `test_support.rs` **no longer exists on `origin/main`**. The retirement removed their producer and left the variants. *This row's sort changed while the scan was being written.* |
| `Mat2`/`Affine2` | **GENUINELY DEAD?** — the only row with **no source at all** for a future consumer, against the M0 review's own norm *"add only on consumer demand"*. |
| `PatchContact` | **PLANNED — the strongest row.** Producer is spec-written and queued: `ASM-R2B-SPEC.md` D-2 mints it; `M9-3-SPEC.md:143` is its acceptance row. |
| `trace_plane_nurbs_uncertified` | **DELIBERATE-FRONTIER** — `M5-PR7B-SPEC.md:71` authorised exactly this fate. |
| `flips_on_path`/`flips_at` | **DELIBERATE-FRONTIER** — `M4-PR4-SPEC.md` D2 mandated a general engine: *"must not be specialized to either consumer"*. |
| `jet.rs` cone/torus arms | **DELIBERATE-FRONTIER**, structurally forced by the closed `Surface` enum (D3); the refusal is downstream and reasoned. |
| `CornerBall::surface` | **SUPERSEDED** — both callers replace the chart, with the reason in code. |
| `corner_contact_circle`, `trimline_description` | **SUPERSEDED** — inlined duplicates; the spec's obligation is met by the inline form. |
| `PairSolve` | **GENUINELY DEAD?** — written by ASM-R2a, unmentioned by ASM-R2b, which is the unit that consumes the solve. |
| `CLASS_DEFERRAL` | **This report's claim is WRONG** — it **is** read, at `mate.rs:350`. What is unreachable is the *variant*, kept knowingly (PR #575 deviation 5). |

*On `DESIGN.md`'s "dead-code pattern M5's reviews repeatedly punished":* that
sentence is the grounds for **parking** the canal-surface blend — a rule
against *building* unconsumed machinery, invoked at a decision **not** to
build. None of these rows was flagged under it; the reviews that touched them
each ruled *keep, with the frontier named*.

*Needing a decision:* `Mat2`/`Affine2`; `PairSolve` (ask R2-b's spec whether it
wants the record); `hull.rs`'s non-rational half; the two inlined fillet
helpers; and `ProfileError`'s five now-zero-constructor variants.

### D4 CHECKED (2026-08-18): two of the five rows move

Each remaining row was chased to its spec and its call graph. Two change.

**`hull.rs` — the sort was right, the *scope* was not. Recommend KEEP.**
`domain_hull` is not unconsumed: it is the body of `sup_norm_bound`
(`hull.rs:347`), a documented public certified-bounds function, and
`domain_hull_rational` likewise backs `sup_norm_bound_rational`
(`hull.rs:358`). Those wrappers' own callers are still only tests (4 in
`geom-core/tests/spline_hull.rs`, 3 in `geom-curves/tests/review_m5_pr2_e2e.rs`),
so "no named consumer" holds at the *top* of the chain — but the deletion
on offer is not one dead helper, it is **retiring the `sup_norm_bound*`
API**, and the rational limb of that API sits on the banked #390/#453
lane. Different decision from the one the row describes.

**`PairSolve` — the evidence points the other way from "R2-b will want it".**
R2-b has already merged (**#591**, and `ASM-LOG.md:297` records *"R2 IS
CLOSED"*), and on `origin/main` today `PairSolve` is still exactly two
lines: the `pub struct` at `mate/solve.rs:66` and the re-export at
`mate.rs:57`. No constructor, no reader. R2-a's own commissioning spec is
`ASM-R2A-SPEC.md` D-4 ("the per-pair coset solve"), and what it makes
binding is the fold's *verdicts* (DETERMINED / UNDER / CONTRADICTORY) and
the tree/non-tree *roles* — obligations `SolvedPoses` and `MateRole`
already discharge. It never asks for a public per-pair record type. The
one banked follow-on unit is ASM-XSPLIT (the AQ8 conversion door), which
is the F1/census gap, not the coset record.

So the unit that would have consumed it has closed without doing so. That
is not proof it has no future — but it removes the reason to assume one.

**Unchanged:** `Mat2`/`Affine2` (only the `lib.rs:38` re-export and one
review test); the two fillet helpers (inline duplicates confirmed at
`surgery.rs:1556` / `build.rs:1185` for `TangentIntersection`, and
throughout `surgery.rs` for `Curve3::Circle` — but `trimline_description`'s
doc is the only place D7's prefer-intrinsic obligation is *named*, so that
sentence needs migrating, not dropping); `ProfileError`'s five variants
(`test_support.rs` is gone from the tree, so they have zero constructors).

### D4 DECIDED (2026-08-19): delete — and the execution goes to the back

**Evan, 2026-08-19: delete, with the work deferred to last priority.** The
changes are individually trivial now and landing them mid-programme would add
noise to lanes that are reading the same files for other reasons.

*The decision is recorded now even though the work is deferred*, because the
reason D4 sat in Wave 0 was *"answering stops anyone documenting them"* — and
a recorded verdict does that job on its own. Without it, the next lane that
walks through `linalg.rs` or `mate/solve.rs` writes a paragraph explaining
code that is already condemned. That is C5's growth sink, entered avoidably.

**Scope is three rows, not five.** `ProfileError`'s five variants already went
in **#622**. `hull.rs` is **struck** per the check above: the deletion on offer
is really *retire the `sup_norm_bound*` API*, whose rational limb sits on the
banked #390/#453 lane — a different decision that should be asked as its own
question. What remains: `Mat2`/`Affine2`, `PairSolve`, and the two inlined
fillet helpers.

**Placement.** Back of the queue, but **ahead of W3b** — trimming comments on
code that is about to be deleted is the waste ordering rule 1 exists to
prevent, so the deletions must precede the comment pass over the same files.

**Each deletion owes a provenance note next to the thread of work that
produced it** (Evan, 2026-08-19), so the code can be recovered from history if
the thread turns out to have wanted it after all:

| Row | Where the note goes | Why there |
|---|---|---|
| `PairSolve` | **issue #611** (*ASM program: resume this line of work*) | Its thread is live. R2-a wrote it, R2-b (#591) merged without constructing it, and #611 is where R2's successor will start reading. |
| The two fillet helpers | the fillet thread — **#319** / **#554** | Both are live fillet issues, and `trimline_description`'s doc is the only place D7's prefer-intrinsic obligation is *named*, so that sentence migrates with the note rather than dying. |
| `Mat2`/`Affine2` | the deleting PR body, cross-referenced from **#614** | M0's linalg thread is closed and archived — there is no live thread to annotate, which is itself the strongest evidence for the row's `GENUINELY DEAD?` sort. |

Per §C3, a note in a prose register is not a deferral but a forgetting on a
schedule — so the deleting PR must cite the commit SHA the code is recoverable
from, not merely say "see history".

## S12. FIXED by #706 — the release-profile run the suite instructed and CI never did

- **Where**: `crates/topo/src/review_m1_pr2/release_corruption.rs`,
  `.github/workflows/ci.yml`
- **Confidence**: likely — **headline overturned by the steelman**

**Only part of this finding was implementable, and it is not the part the
finding led with.** #706's body carries the full argument.

*What the steelman overturned.* The headline — "in release builds, with no
postcondition, silent corruption" — was wrong on both clauses.
`assert_euler_postcondition` (`euler.rs:1975`) runs the arena-delta check
**and full tier-1 `validate`** after every successful operator under
`cfg(debug_assertions)`, which the root manifest keeps on in dev, so the test
suite and CI do detect a half-spliced body. And the release disposition is not
convention but **ratified and deliberately tested**: D9's documented-garbage-out
footnote, `euler.rs:47`, and the adversarial suite `release_corruption.rs`,
whose header reads *"typed errors or garbage bodies — never a panic, never a
hang."* The census stands (58 discarding sites; `link_half_edges` exactly as
described), and **S43** carries the larger question this turned out to be part
of.

*What is already ratified, and what is left of it.* The steelman's closing
sentence — that whether to make the write helpers unable to silently do nothing
is a D9 question — **was answered the next day.** The **D2 addendum to D9**
(`DESIGN.md:1138`, ratified 2026-08-19, PR #628) says *"silent discard is never
an answer"* and **explicitly supersedes** the footnote's original *"typed errors
where cheaply detectable, or documented garbage-out in release"*; S43's verdict
`:4517` records it ACCEPTED AND SETTLED and closes *"S12's residue and S14's
disposition follow from this rule and should be re-read against it rather than
re-argued."* So there is no open decision here. What is left is **execution**:
the ~60 silent discards in `euler{,_ring,_kill}.rs` are a known deviation from a
ratified rule, and converting them is **W2c**, which had a verdict and no row anywhere until
#706 placed it as Track D's **D16**.

> **A note on how this row was dispatched.** D6's brief sent its lane to fix the
> CI gap while telling it the rest of S12 was an open D9 question sitting in
> *Open decisions*. Both halves were false — the question was ratified the day
> before, and no such row exists. The brief was written from S12's steelman
> without reading S43's verdict twenty lines further down this same file, which
> is the Q8 failure this document warns about, committed while writing the
> briefs. W2c's absence below (it appeared **once** in this whole file, in
> S43's prose at `:4532`, as *"now unblocked and unstarted"*) is the same miss
> one step earlier: §D's fourth ordering rule is *a verdict is not a
> placement*, and the verdict that added the rule went unplaced four sections
> from where the rule is written.

*What was fixed.* The gap underneath both: `release_corruption.rs` instructed
*"Run this under BOTH profiles"* and **CI ran one** — the only `cargo test
--release` in `ci.yml` was the `oracle-inari` lane, and unlike the file's other
profile choices (`:1165`, `:1210`) this one was an instruction with no
mechanism, the §C14/Q6 shape. #706 adds the job `corrupt input (release
profile)`: `-p topo --lib` filtered to the suites, gated by a new
`RUN_TOPO_RELEASE` root in `ci-filter.py`'s `JOB_ROOTS` (the idiom every other
closure-gated job uses, and the thing that lets `ci-local.sh` mirror the row —
it parses only `TIER`, `CARGO_SCOPE` and `RUN_*`), mirrored there per
`ci.yml:134`'s sync obligation, and the suite headers now name that job instead
of instructing a reader to be the mechanism. It fires on **89 of the last 128
first-parent merges** and costs **93 s cold** on a hosted runner. **Correcting the instruction instead would have
been the wrong outcome** — two of the six rows are not profile-independent.
`large_torn_body_terminates_quickly` attacks 3000 struts in release against 500
under debug-assertions, and the clause it defends — *"the bounded-traversal half
stands: never a hang; every traversal is bounded"* — is the one clause of the
corrupt-input disposition the D2 addendum kept. And
`foreign_parent_loop_garbage_in_garbage_out_release` is the file's only
`#[cfg(not(debug_assertions))]` item, so off a release job it was neither run
nor even *compiled* anywhere (clippy is a debug pass too) — a **profile**-level
rot hazard, independent of what the row asserts, which matters because what it
asserts is scheduled to change: it certifies the very garbage-out W2c converts,
and W2c will change its meaning or retire it. #706 says so at the job and at
the suite rather than leaving the next reader to discover it. The run step also
asserts its own selection is non-empty, because a `cargo test` name filter that
matches nothing exits 0 — the same silence, one level up.

*The stale-contract class, swept.* Three sentences in `euler.rs` (`:22`, `:52`,
`:2010`) still asserted the superseded garbage-out disposition as ratified;
#706 retimes all three to "today's behaviour, pending W2c" — prose only, the
code is W2c's. Pattern: `grep -rn garbage crates/topo/src` plus the footnote's
own wording across `crates/`. **What it could not match**: a site that describes
the superseded disposition without the word, and it deliberately leaves the
`geom-curves`/`geom-core`/`geom-surfaces` "documented garbage-out" hits alone —
those are the addendum's **row 3** (value-domain poison), a different rule.

*Two more sites whose release semantics nothing executed.*
`review_m1_pr4.rs`'s section 9, headed *"Release-mode garbage-in"*, is now in
the job's filter. `geom-core/src/spline/knots.rs:507` (`from_algebra`) has a
`#[cfg(not(debug_assertions))]` arm that **no test runs in any profile and no
CI job type-checks** — same class, different crate, and out of this unit's
scope; it belongs with whoever next opens `spline/`.

## S13. Load-bearing invariants held by CI grep, allowlists, and a magic count

- **Where**: `scripts/gates/bounds-allowlist.sh`,
  `scripts/gates/interval-square-allowlist.sh`,
  `scripts/gates/no-extra-real-bounds.sh` (all three were inline in
  `.github/workflows/ci.yml` at `:322`, `:420`, `:444` when this was
  written; #626 moved them),
  `crates/geom-core/src/real.rs:348`,
  `crates/geom-core/tests/flagged_census.rs:20`
- **Confidence**: sure

The disciplines the design leans on hardest are text-matching CI steps:
a `Bounds` compound-bound gate with thirteen hand-maintained path
regexes; an `EvalScalar` gate ("the Bounds gate, over the name");
`grep -rnE '\bReal\s*\+'` to catch extra bounds on evaluation type
parameters; a `\b([a-z_]\w*)\s*\*\s*\1\b` regex banning `x * x` with its
own file allowlist; an env-read ban; and a test that counts occurrences
of the literal string `k_stats::decide_flagged(` across `crates/*/src`
and asserts it equals **8**.

Correspondingly, `Bounds`'s doc comment has become an append-only
ratification ledger: ~157 lines and six "Ratified amendment / Extension"
entries in front of a two-method trait, each admitting one more file to
the allowlist. Each individual ruling may be right; the aggregate is a
rule whose exceptions now exceed the rule, upheld by string matching
rather than by construction.

**Verdict:** ACCEPTED (Evan, 2026-08-18) — "lots of other great catches."
Steelman pass commissioned on one specific question: `tools/k-lint` is
the repo's own lint crate, so the capability exists — why are these rules
greps rather than lints or types? Some CI greps are the right tool
(`memories/interval-square-poison.md` documents one that caught a real
bug class); the pass is asked to say which are which.
**Steelman (2026-08-18): SURVIVES IN PART — and the premise "greps instead of
types" is largely false.**

*What the types actually do.* The type-level encoding is real and does most of
the work: `Real` is comparison-free **by construction**; `Bounds`/`Decide`/
`SpanLocate` are separate subtraits; **`Bounds` has no `Dual` impl**, so the
whole bracket-reading surface is uninstantiable at duals; and three static lane
splits with refusing arms exist (S3). *(That middle clause was true when this
steelman was written on 2026-08-18 and is false as of the **D1** ruling of
2026-08-19: `Bounds` is implemented for `Dual`, and what is uninstantiable at a
dual is the `CertifiedEnclosure`-bounded surface. The steelman's conclusion is
unaffected — the type-level encoding still does most of the work — so the
sentence is amended rather than rewritten.)* Each ledger amendment litigates a *type*
question, and M5 PR 12 **explicitly costs the type-level fix and rejects it on
the merits** (*"would have had an EMPTY refusing side"*). The greps enforce only
the residue — **which files may *name* a bound** — which Rust cannot express
across crate boundaries, since sealing controls who may *implement*, not who may
*name*. Nobody has written that down.

*Two gates could not be types or lints at all.* The interval-square rule guards
a **whole-program** property (now
`scripts/gates/interval-square-allowlist.sh`, `ci.yml:437` when this was
written: *"whether THIS enclosure can
straddle zero is a global property of upstream callers that refactors change
silently"*), and the env ban has a measured receipt. And `EvalScalar` is
evidence the encoded alternative was **tried and needed more grep, not less**:
now `scripts/gates/evalscalar-allowlist.sh`, `ci.yml:354` then — *"the trait is
`pub`… so without this step any file in any crate
could acquire a compound Bounds bound invisibly to the grep above."*

*The brief's premise about `tools/k-lint` was wrong, and it matters.* `k-lint`
is a **CSV gate** over probe sweeps, not a source lint; it does not parse Rust.
**The repo has zero Rust-AST lint capability today.**

*What does not survive.* (1) **A lint/tool alternative was never evaluated** —
the only mention is a one-line "optional escalation" in `M0-LOG.md:74`, dropped
silently; no `dylint`, `clippy::disallowed*`, `clippy.toml` or proc-macro
appears anywhere in the history. The record is silence, not rejection. (2)
**FIXED by #626** — the dual-maintained allowlist was a **proven drift class**
(a `separation.rs` entry hosted-only, a `test_support.rs` entry stale locally,
a `chart_region.rs` entry before that, and two gates with no local mirror at
all). Nothing about these gates is maintained twice any more, at either level.
The bodies: every mirrored gate lives once under `scripts/gates/`, ci.yml's
`discipline` job and `local-scripts/ci-local.sh` both call the same script, and
the ratified justifications have one home. The rosters: sharing the bodies
would have left each half still naming its own gate list — the same defect one
level up, and the one that let `EvalScalar` and the interval-square gate run
hosted-only — so the local half now runs the gate DIRECTORY in a loop, leaving
it no roster to drift, and `gate-roster.sh` checks the one hand-written roster
that has to exist (ci.yml's named steps, since the Actions UI reads failures by
step name) against that same directory, in both directions. What that roster
gate proves is **wiring, not execution**: it is a grep, so it cannot read YAML
semantics, and a step disabled by an `if:` condition — or a false job-level
`if:` on `discipline` itself — keeps its `run:` line and satisfies the check
while Actions skips it. Closing that would need a workflow evaluator, which was
judged not worth building; the hole is named in the script header instead, and
a silenced step is at least visible as skipped in the Actions UI. Each gate
also carries a `--selftest` (clean fixture must pass, planted violation must
fire) which **both halves invoke before the real pass**, as the sibling python
gate does; that settles the "only one of six has a self-test" residue, which a
self-test nobody ran would not have. A gate that scans an empty or wrong tree
now fails instead of reporting green.
(3) The greps' own remaining defects go unfixed, and #626 left every one of
them alone — it moved the gates without changing a regex, a message's meaning,
or an allowlist's membership. Its one behaviour change is disclosed:
`bit-identity-debug-only` used to exit **0** when `crates/topo/src/source.rs`
was absent (both `grep -c` calls exit 2, the counts are empty, `[ "" -gt 0 ]`
errors and reads as false), i.e. it reported green precisely when its subject
had been renamed out from under it; it now fails loudly. Still standing: the
lint/`dylint`/proc-macro alternative in (1) is unevaluated; the `x*x` lookahead
fix has sat in a log since 2026-08-04; `Real +` strips no comments while its
siblings do; and the regex is leaky both ways — it **cannot see**
`self.x * self.x`, and `linalg/vec.rs:311` contains exactly that shape in
production generic-over-`Real` code.

*On the `decide_flagged` count of 8 — it is a real census, not a tripwire.* It
is tracked as **issue #214** with a per-family retirement plan; `ledger_row` is
`let _ =` at runtime, existing only to force the author to name a row; and the
count **has moved exactly once, downward** (12 → 8, `d92f56b5`, authored by
Evan, in the same diff that converted the ledger rows). Growth is the forbidden
move. Its weakness is that it asserts a **total**: one site added and one retired
nets to 8 and passes. (The scan credited that observation to
`memories/schema-claim-discipline.md`, which says nothing about censuses or
totals — it is about `SCHEMA_VERSION` races merging clean. Same
misattribution as S10's, corrected there; the observation itself stands on
its own.)

## S14. `Span` validity is prose, and the guard's removal turned poison into a documented panic

- **Where**: `crates/geom-core/src/spline/knots.rs:166`,
  `crates/geom-core/src/spline/hull.rs:74`,
  `crates/geom-core/src/spline/basis.rs:9`
- **Confidence**: sure

`Span` carries `index`/`first_control`/`degree` but is deliberately not
tied to the `KnotVector` it was drawn from, and every downstream index
guard was deleted on the strength of that. Three module docs then spend
~60 lines re-explaining the same unenforced obligation, and
`hull.rs:80` states outright that a mismatched `Span` now "can index out
of bounds and **panic**, which is a worse failure than the poison D4 asks
for" — a self-declared violation of the crate's no-panic rule, shipped
and annotated rather than fixed. `Span` carries `degree()`, so it could
at minimum be checked cheaply against `kv.degree()`; nothing does.
`knots.rs:174` costs out two type-level fixes and declines both.

Two related back doors in the same file: `from_algebra` is documented
"Debug builds re-validate", but the debug arm calls `Self::clamped(..)`
and, on `Err`, constructs the unvalidated value anyway — no assert, no
log, the only effect a wasted clone (`knots.rs:507`). And `unit_segment`
clamps `degree.max(1)` rather than refusing degree 0, justified by seven
lines of comment, where every caller in the workspace passes the literal
`1` (`knots.rs:495`).

**Verdict:** DISPUTED — REFRAME PROPOSED (Evan, 2026-08-18). "For `Span`,
the crate doesn't ban all indexing operations, so this strikes me as just
being honest? (/ maybe we need to update that principle to *'no panic on
any reachable state, yes panic on things that can only indicate bugs'*)
— but it'd be even better to just make it constructively valid of
course."

So the finding as written over-reaches: it treats `hull.rs:80` as a
violation when it may be candour about a state only a kernel bug can
reach. The steelman pass is asked to (a) quote the ratified principle
verbatim and say whether Evan's reframe is a clarification or a change,
(b) census how the kernel actually handles "can only be a bug" states
today — the scan found at least four different idioms, which would be a
bigger finding than this one, and (c) determine whether a mismatched
`Span` is reachable by *misuse* through any public entry point, in which
case the honesty defence does not cover it.
**Steelman (2026-08-18): SURVIVES — the honesty defence fails on
reachability, and it fails concretely.**

*The ratified text, quoted* (`DESIGN.md:1100`): *"The kernel never
panics **on any input**: panics are bugs; every failure is a typed
error."* Three things are load-bearing: the rule is scoped to **input**;
the footnote grants its one exemption **only to `debug_assert`s**, on an
explicit unreachability-by-input argument; and where D9 addresses the
can-only-be-a-bug case in release it names two dispositions — *typed
error where cheaply detectable, or documented garbage-out* — and **panic
is not among them.**

*Is the reframe a clarification or a change? Both, in separable
halves.* *"No panic on any reachable state"* is a **clarification** —
D9 already says "on any input", and no existing `debug_assert` would
move. *"Yes panic on things that can only indicate a bug"* is a
**change**: it licenses panics **in release**, which D9 does not, and on
the one such class D9 explicitly disposes of it chose typed error or
garbage-out with *"never a hang"* as the floor. So `hull.rs:80` is not
covered by the reframe as a clarification — only as an amendment.

*But the reframe is not novel — it was already adopted, unnoticed.* PR
#447's merged body argues for panicking indexing on the merits:
*"indexing is the safer shape: it **panics** where a `zip` silently
drops control points. That is the fail-loud direction."* It was simply
never taken back to D9 — and `crates/topo` was meanwhile ratified in the
opposite direction.

*Why the honesty defence fails: the mismatch is reachable by **misuse**,
not only by bug.* All `Span` producers are `pub` on a `pub` type, and
consumers come in two shapes — two independent arguments (`basis_funs`,
`ders_basis_funs`, `span_hull`, `sup_norm_bound_span`) and vector-in-`self`
(`NurbsCurve{2,3}::{eval,ders,deriv,deriv2}_in_span`,
`NurbsSurface::window_of`). **Build two clamped `KnotVector`s of equal
degree and different length, take `long_kv.span(k)`, hand it to a curve
built on the short one: `eval_in_span` indexes out of bounds.** Safe
Rust, public API only, no kernel bug anywhere in the trace.

*The surface case is worse than a panic.* `window_of`'s own docs
concede: *"The **argument order is load-bearing and nothing checks
it**: a `Span` carries no direction, so `window_of(span_v, span_u)`
typechecks and builds a window that is **wrong rather than refused**."*
A swapped-but-valid pair from the correct surface yields a silently
wrong answer through a public method with no bug involved.

*The "no cheap guard" premise is false, and the cost was already
measured.* The deleted guard was one three-way check **per call**, not
per index. `kv.span(span.index()) == Some(span)` reproduces it exactly
in O(1) from existing code — and a degree check alone (this report's
weaker suggestion) misses the equal-degree/longer-vector case for the
same price. #463's own benchmark puts the gains in the upper quartiles.

*The decisive blocker nobody has named:* **Option A does not close the
doors that matter.** Removing the `kv` parameter makes the mismatch
unrepresentable for the *free functions* — but `curve.eval_in_span` and
`surface.window_of` hold their vector in `self`, and a `Span<'a>` that
merely borrows *some* `KnotVector` still typechecks against a different
curve, because lifetimes are covariant. So the type-level options are
**strictly weaker** than `knots.rs:174` and issue #475 present them, and
the O(1) re-validation is the only candidate closing the whole surface.
`knots.rs:174` costs out two options and declines both **without ever
naming the cheap third** — that omission, more than the declining, is
what does not hold up.

*What the finding gets wrong:* it implies negligence. This is documented
three times and **issue #475 is already open** (filed 2026-08-13, five
days before the scan) costing out Options A/B/C. Every in-tree
construction site draws its span from the vector it then uses — all were
checked — so the hazard is not live in-tree. But #475's own test (*"No
live caller does this"*) is the wrong test for an API-first project: the
public signature is the product, and it admits the mismatch. #475's
disposal of Option C is the standard to judge by: *"defensible — but
then the docs should say it is a **decision** rather than a deferral."*
Today the docs promise the brand and call it deferred, which is neither
honest state.

*One stale deferral worth its own look:* `indexing_slicing` was deferred
in `M0-LOG.md:59` with *"revisit at PR 6"*. PR 6 completed the same day
with no mention; nine milestones later the `Cargo.toml` comment still
reads as though evaluation code does not yet exist. Issue #475 notes the
S14 panic is *"one clippy's panic-family gate cannot see because it is
an index expression"* — the one mechanism that would have caught it is
held open by a stale 2026-07 deferral.

## S15. FIXED by #713 — the prose-held invariants, sorted and disposed row by row

- **Confidence**: sure for each row
- **Verdict**: ACCEPTED (Evan, 2026-08-18) — "lots of other great catches."
  Steelman 2026-08-18 (the block under the Tier 2 header below) sorted the
  rows; **#713** (Track D, D5) executed that sort.

The finding tabulated **nine** rows and the steelman's sort accounted for
**eight** — the miscount predates #713 and is why the disposition table below
is the one enumeration of them all. Two rows had already closed under #635,
one under #688, one is moot post-#688, and one leaves as a tracked issue.

| Invariant | Anchor | Disposition |
|---|---|---|
| Fillet birth-record provenance | `fillet/naming.rs:95` | **Left, deliberate.** Re-read on today's main; the written self-assessment is honest and the discipline is the guarantee. |
| `flipped_face_sense_for_tests` | `body.rs:630` | **Left, deliberate.** Re-verified `&self` + clone — it is not even a mutation path, and confers nothing `set_face_sense` lacks. |
| The 16-direction ray schedule | `boolean/solid_contain.rs:76` vs `splitting/containment.rs:102` | **STILL OPEN, scheduled as D10.** Left by this finding as deliberate-and-argued, and **#712 (D9) did not close it** — D9 unified a *different* pair, so this row survived it. Both tables here are 3-D and byte-identical (diffed entry for entry); `chart_region`'s `SCHEDULE_2D` is a different table by dimension, not a third instance. The determinism claim nothing checks is real and the close is mechanical; **D10** carries it. |
| `topo::iso` geometry-blindness | `iso.rs:56` | **Closed by #635**, which replaced the *"`Placeholder` ballast"* justification with the reason that survives. #713 swept the **sibling** it left one bullet down: *"at M1 no geometry hangs off"* `he_plus` is false — a carrier runs forward `start(he_plus) → end(he_plus)` — and the bullet now says why the bit is still ignored (the carrier is ignored too, so a flip and its re-minted curve are invisible together). |
| pcurve cache staleness — *"should say which in its own docs"* | `pcurves.rs:124` | **STILL OPEN, and it is not what #635 closed.** #635 closed the steelman's *fourth claim* (`merge_coplanar_faces` mis-bucketed as neither-clearing — verified fixed, no D4 hand-off owed). The row itself is verbatim where it was: a convention with a survey beside it, `"The lists above are a survey, not an enforced invariant"`. Placed as **D13**. |
| Which door fills which naming field | `fillet/build.rs:48` | **Moot post-#688.** The claim was about the whole-body door's `FaceKind` plan payload; that block, that type and that door were all deleted. What survives of it is the fillet-provenance row above, which the surgery door already carried and which is sorted `left, deliberate`. |
| Fillet `Retired` survivor guard | `names/emit_fillet.rs:216` | **Closed by #688**, incidentally: the door the comment was false for is gone, the surgery's `kef`s kill only faces it minted mid-flight, and the face claim the steelman found **absent from the acceptance suite because it would fail** is now asserted both ways (`m6_5_fillet_naming.rs:263`, `:367`). #713 pointed the comment at that test and retired its neighbour *"from either assembly door"*, which named the dead door. |
| "The only public mutation paths" | `euler.rs:41`, `seqgen.rs:12`, `DESIGN.md:1110` | **Corrected, and now checked** — see below. |
| Tie propagation across emitters | `names/emit_fillet.rs:94` | **Tracked as #708**, with a KNOWN HAZARD block pointing at it. This was the row's whole deliverable. |

**The frozen count of eleven, and what replaced it.** Re-derived rather than
trusted: **37 doors** on today's main — 32 `pub fn (&mut self)` on `Body` plus
5 free functions taking `&mut Body<T>` — not the steelman's ≥ sixteen. None of
the three sites got a new count, because a count is what rotted; each states
the closure property instead, and
`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1` now
**checks that property against the real surface**: each door either declares
`assert_euler_postcondition` (14 do) or sits on an allowlist with a reason
(23 do). It goes red the day a door is added, which is the rot the prose could
only describe.

Writing that test found what the prose had not. **`instance`'s grafts do not
preserve tier 1**, and the closure property as first drafted asserted a
guarantee they do not offer: `graft_disjoint_all_keyed` mints an empty
destination solid per source solid *before* transplanting, and a refusal
raised mid-transplant leaves `dst` partially written — its own docs say the
destination is *spent, never resumable*, and an empty solid is the tier-1
`SolidWithoutShells`. A caller discarding that `Err` can fire a later
operator's postcondition from **API misuse rather than a kernel bug**. Both
`euler.rs` and `DESIGN.md` now name the exception; the consequence is **S14's**
and is recorded there as its second witness. The lesson is the shape: the old
sentence was a false enumeration in front of a property true of everything it
enumerated, which fails safe; the replacement was a true enumeration in front
of a property false of part of it, which does not.

**The `seqgen` half — the one part that could find a defect, and did.**
`split_edge` now enters the randomised lane (`OpChoice::SplitEdge`, plain-apply
and roundtrip arms, and coverage labels split by site shape —
`split_edge` / `split_edge_strut` / `split_edge_self_loop` — so the row proves
the delicate coincidences `split.rs:94` names are reached, not merely that the
op fired). It surfaced two properties of the generator, neither a defect in
`split_edge`:

- **Carrier/endpoint coherence.** The fan-rebasing ops (`mev`'s fan site,
  `kev`'s fan merge) move a run of half-edges onto a different vertex without
  re-describing the survivors' carriers, so an edge's stored curve is routinely
  stale against its own endpoints — invisible to tier 1 and to the oracle, and
  refused by `split_edge`'s child certification. Candidates re-derive the
  parent's certificate first. **Stated blind spot: only carrier-coherent edges
  are split here.** #713 also moved that obligation out of a private
  test-support helper and onto `mev` and `kev` themselves, where a caller
  reads it.
- **Coordinate distinctness.** A split point comes from geometry, not the
  vertex counter, so two edges over one pair of points mint points **one ulp**
  apart — bitwise distinct, and enough to poison the next `mef_chord` chord.
  The gate demands *definite* separation under the same metering the chord
  sugar uses.

The inverse is two ops: `split_edge` is the only catalog member that REPLACES
geometry, so `kev` restores the topology and re-attaching the captured spec
restores the description. **Measured cost: +0.9 s median (1.91 → 2.78 s)** on
`cargo test -p topo --lib seqgen`, five runs each, warm binary, 4-vCPU Xeon
@ 2.80 GHz with four other lanes on the box — an upper bound, and a ratio
rather than an absolute. What that ratio actually buys is charged to the
generator's eager candidate enumeration, placed as **D14**.

---

# Tier 2 — significant

**Steelman (2026-08-18): the three bug-shaped claims verified; sorted row by
row; and one *fourth* false claim this report missed.**

**(a) The `emit_fillet` `Retired` face hole — CONFIRMED, and the repo's own
tests state both halves of the contradiction.** `build.rs:701` fills `Retired`
under the comment *"every source entity is retired by construction"*;
`emit_fillet.rs:239` excuses faces with *"Faces are never retired by either
door"*. The acceptance suite asserts the surgery-door claim at
`m6_5_fillet_naming.rs:275` and the whole-body claim at `:382` **for edges and
vertices only** — the face claim is absent because it would fail. Honest
severity: defence-in-depth, not a live bug (the guard is documented as
unreachable in production) — but the hole is in the **majority** of the guard's
surface, since faces are the bulk of what the whole-body door mints.

**(b) `split_edge` shares `mev`'s Euler vector and is absent from the fuzz
lane — CONFIRMED**, with a mitigation this report omitted: `split_edge` and
`movefac` each carry the **same tier-1 debug postcondition** as the eleven, so
D9's *conclusion* holds; what is stale is the *enumeration* it rests on. The
count is stale in **four** places, one of them ratified (`DESIGN.md:1103`);
the real public `&mut self` surface is **≥ sixteen**. The genuine loss is
randomised site coverage over exactly the cases `split.rs:94` calls delicate.

**(c) `topo::iso` — CONFIRMED stale, OVERSTATED as to consequence.** Vertex
points **are** compared bitwise, so "entirely different geometry" is too
strong. And the same bullet gives a **second, non-stale reason**: surface
anchors are construction-history artifacts, so including them would make
legitimate `kfmrh ∘ mfkrh` roundtrips compare unequal. A stale sentence in
front of a still-correct decision; the fix is the sentence.

**A fourth false claim, not in this report:** `pcurves.rs:91` lists
`merge_coplanar_faces` among the ops that *"neither clear nor re-mint"*. That
list was written 2026-07-30; on 2026-08-05 `merge_coplanar_faces` **started
re-minting** (`merge_faces.rs:193`, typed `MergeCoplanarError::Pcurve`). The
convention worked and the index rotted.

*Row sort:* **three deliberate-and-argued** (fillet provenance —
`build.rs:41` is an unusually honest written self-assessment;
`flipped_face_sense_for_tests`, which confers no capability `set_face_sense`
lacks; the ray schedule, byte-identical today, verified); **two
deliberate-with-stale-prose** (`iso`, pcurve staleness); **three accretion
with no tracking** (the `Retired` face hole, the eleven, tie propagation).

**Zero rows have a tracked issue** — the sharpest structural fact here, since
the repo demonstrably knows how to do all three alternatives: issue #214 for a
census, `attach.rs:119`'s **KNOWN HAZARD** block for a named-and-pinned gap,
and `review_s11_adv` for a reachability pin.

## S16. Three face bounding-box constructions with three different soundness rules

- **Where**: `crates/topo/src/boolean/boxes.rs:179`,
  `crates/topo/src/census.rs:1127`, `crates/topo/src/separation.rs:181`
- **Confidence**: likely
- **Found independently by two scans.**

**FIXED by #620.** `FaceBoxRule` in `boolean/boxes.rs` is now the one
statement of which surface kinds have a cheap sound box and by what
construction — planar faces hull their boundary EDGE boxes, NURBS take the
control-net hull, cone/torus/no-surface poison — and `face_box` is its
`f64`-bracket instantiation. `separation::certified_face_box` is deleted and
calls `face_box`; `census::reach_box` reads its arm from the same rule and
only re-derives the ARITHMETIC, because the `Bounds` allowlist is closed to a
lane that validates `Dual` bodies. The sweep found a fourth instance —
census's instance-containment arm compared two vertex hulls, so a body nested
inside a cylinder cleared silently — fixed in the same PR.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this cluster: "huh, i wonder
how these even happened. they should certainly be unified." Postmortem pass
commissioned — the sharpest question is who wrote `boolean/boxes.rs`'s KNOWN
GAP note about the NURBS half and whether the planar-with-conic-rim half was
considered at the same time.
**Postmortem (2026-08-18). The rule was sound when written; the PR that
invalidated its premise applied the correction to two of three arms — in the
same diff.**

At M5 PR 8 (#135) the boolean operand gate was **planar-only**, so a planar
face's edges really were all lines and the vertex hull really was a superset.
M5 PR 9 (#152, commit `514bab9`) **retired that gate** — *"the F5 planar-only
refusal retired PER C5 TABLE ARM"* — and in the **same diff** added the
cylinder arm (*"the vertex hull is NOT a superset — the wall's belly bulges
past its chords"*) and the conic `edge_box` (*"an arc's belly bulges past its
chord"*), while leaving the planar sentence untouched. The author applied the
bulge argument twice in one sitting; the third arm's premise had just been
invalidated by their own gate change.

**FLAGGED AND PARTLY FIXED — but only the NURBS half was ever named.** The
KNOWN GAP note was written by the perf-scan agent (PR #573, a *doc-correction*
pass, not a fix). That agent read `separation.rs`'s doc and quoted it in the PR
body — but that same doc comment's **Plane bullet, fifteen lines above the text
they quoted**, already stated the planar answer: *"the vertex hull alone would
not, for a circular rim."* They generalised the stale-premise argument to NURBS
and stopped. The planar-with-conic-rim defect in `boolean/boxes.rs` itself:
**NEVER FLAGGED** (checked #135/#152/#564/#571/#573 bodies, the A/B rows,
`M9-LOG.md`, PERF-SCAN finding 1, and every "circular rim" occurrence in
`crates/`).

Sharpest supporting detail: `census.rs`'s `max(r, sagitta)` planar pad exists
because the naive filter *"falsely refused the corpus's cube-beside-cylinder
file"* — **a live wrong answer in a sibling lane, fixed locally, never
propagated.**

*Lesson:* when a PR removes a gate, review must re-check every comment that
cites that gate as its justification — and when a scan finds a stale premise,
the fix is to re-derive **all** arms resting on it, not the one that matched
the search term.

## S17. FIXED by #712 — the two topo ray-parity copies now share one home, and the K convention did not forbid it

- **Where**: `crates/topo/src/chart_region.rs:897`,
  `crates/topo/src/splitting/containment.rs:154`,
  `crates/profile/src/validate.rs:1298`; the home is now
  `crates/topo/src/ray_parity.rs`
- **Confidence**: sure

**What it was.** `chart_region::point_in_polygon` was a line-for-line
port of `splitting::containment::point_in_loop` — same boundary
pre-pass with the same `norm_squared` comment, same clamped-foot
distance, same `'ray:` retry loop, same straddle/advance parity, three
predicate names renamed and the arm gate dropped. Its own doc said so.
`profile::validate::point_in_loop` was a third, with its own
golden-angle schedule and its own `RayCastingExhausted`. Both topo
copies also reused one predicate name for two different questions:
`point_in_loop_boundary` (and `chart_region_boundary`) decided both
the segment-length degeneracy gate and the point-to-segment distance —
the drift `splitting/rules.rs:117` mints a distinct name to avoid.

**The home.** `crates/topo/src/ray_parity.rs` holds the boundary
pre-pass and the per-ray parity walk, generic over a `RaySpace` trait
so neither consumer projects into the other's space, sited as a
**sibling** of both rather than inside either. Both consumers keep
what genuinely differs — their own direction schedules, their own
frame construction, their own typed errors — and pass their K row
names in. Both entry points return the raw `Indeterminate` rather than
a caller-supplied wrapper, so the `.map_err(escalate)` idiom stays
visible at the call site and the shared core cannot drop a diagnostic.

**The arm gate was not "dropped"**: `point_in_loop_arm` gates a *3-D*
schedule member's projection into the loop plane, and a 2-D member is
in-plane by construction, so there is no quantity for it to decide —
an argument `predicate-dimension-audit.md:258` had already made in
M9-2. It belongs to frame construction, which stays with the 3-D
consumer.

**The name collision is closed**: `ParityRows` carries four names, and
the degeneracy gate (margin = segment length) is now
`point_in_loop_segment` / `chart_region_segment`, distinct from
`_boundary` (margin = point-to-segment distance). On the M7 sweep that
splits `_boundary`'s 49 290 samples into 24 645 each.

**`profile::validate::point_in_loop` stays a third copy — a negative
result, not an omission.** `topo` does not depend on `profile` and
never has; closing it would mean adding a crate dependency, a worse
trade than the duplication.

**The S15 ray-schedule row does NOT close with this** — the pair S15
anchors is `boolean/solid_contain.rs:76` vs
`splitting/containment.rs:102`, two byte-identical **3-D** tables;
`chart_region`'s `SCHEDULE_2D` is a different table by dimension, not
a re-declaration. It is scheduled as **D10**.

**This is a half-fix on the class, deliberately.** S17 named the drift
— one predicate name for two questions — as a *class*, and #712 closed
it where the finding pointed and nowhere else. `bool_join_nearest`
(`boolean/join.rs:564,600,804,818`) pools a distance and a difference
of distances under one name across four sites in the same crate, which
is the same drift and worse by site count; it is **D11**, with
`bool_join_facing`, `bool_point_in_solid_plane` and `bool_dir_same`
behind it.

### What the unit is evidence about: the convention charges, but not what the spec inferred

The postmortem below names "new predicate names = new K rows, margins
re-metered" as the mechanism that rewarded copying over parameterizing.
The unification tests that claim, and splits it in two:

- **The convention charged, and this diff paid it.** The four-field
  `ParityRows`, threaded through both call sites and both `const ROWS`
  blocks, exists for exactly one reason: separate populations must stay
  separately metered. That is the rule working as written, and it has a
  real price — a name passed as a parameter is invisible to
  `K-REPORT.md`'s documented `grep -r 'decide("'` inventory, so seven of
  the eight names in these two files stopped being discoverable by the
  project's stated method and nothing went red. The method now names the
  row-name table explicitly (`K-REPORT.md:203`); the `ROWS` blocks say so
  at the definition. **That is the charge, and it is the concrete one.**
- **What was imagined is the stronger inference the spec drew** — that
  *sharing the walk forces pooling the ledger*. It does not. `ParityRows`
  is the counter-example: one body of code, two row sets, zero rows
  removed, two added, no margin moved. The k-lint gate is
  roster-independent (`tools/k-lint/src/lib.rs`: the baseline is re-cut
  "when the DISTRIBUTION moves … not on every merge and not on a
  rename", with #661's six-into-three pooling as precedent in the other
  direction), so no re-baseline was owed and none was taken.

The reading a future spec author should take is **not** "the K
convention costs nothing" — it costs a parameter and a roster entry.
It is: **the K convention does not forbid sharing.** A spec that
reaches for "new names = new rows" as a reason to *port* rather than
*parameterize* is asserting a cost it has not checked.

**Verdict:** ACCEPTED (Evan, 2026-08-18) — see S16.
**Postmortem (2026-08-18). The spec ordered the third copy.**

`profile::validate::point_in_loop` (#28) came first; `topo::splitting::
containment::point_in_loop` (#31) is genuinely separate and reuse was **blocked
by the crate DAG** (`topo` does not depend on `profile`, and never has). The
third — `chart_region::point_in_polygon` (#527) — is the line-for-line port,
and `M9-2-SPEC.md:46` **specified it**: *"port `point_in_loop`'s METHOD to 2-D
— ray parity, a fixed 2-D direction schedule, the four named trileans
re-derived (**new predicate names = new K rows**, margins re-metered)"*. PR
#527's body adds *"reads `containment.rs` for METHOD only (**no refactor**)"*,
with the branch constrained to be "file-disjoint from M9-1's in-flight lane".

**NEVER FLAGGED.** #527 drew a **dual blinded review**; both reviewers
converged on the rung-2 `GeomSource` hole and neither raised the duplication —
correctly, since it was a disclosed, specified deliverable.

*Lesson:* a spec that says "port the METHOD, new predicate names" plus a review
brief scoped to "falsify the PR's claims" together **guarantee** an intentional
third copy passes two independent adversarial reviews without comment. The
lesson this postmortem drew about the K-ledger convention — that "new names =
new rows" actively **rewards copying over parameterizing** — is *half* right,
and #712 corrected it in place above: the convention does charge (a row-name
parameter, and a roster entry the `decide("` grep can no longer find), but it
never required the port. The spec inferred that sharing the walk would force
pooling the ledger, and that inference is what was wrong.

## S18. Certified numeric derivations duplicated across crates (roll-up)

- **Confidence**: sure for each row

| Derivation | Copies | Anchor |
|---|---|---|
| Rational quotient-rule interval assembly | 2 crates (`ssi/enclose` vs `mesh/nurbs_cert`), sharing only the `spline::hull` primitives; a soundness fix in one is invisible to the other | `ssi/enclose.rs:417`, `mesh/nurbs_cert.rs:1039` |
| …and again *within* `mesh`, curve vs surface | The curve version's doc says "the face bound's quotient-rule assembly one dimension down" | `mesh/chords.rs:363` |
| Bulge-arc closed form (ratified convention) | 3 | `edge_geometry.rs:146`, `profile/seg.rs:143`, `sweep/skin.rs:306` |
| Knot insertion | 2 in one module — one against a validated `KnotVector`, one on a raw `&mut Vec<f64>` re-deriving the span with a linear scan where `find_span`'s binary search is one module away | `spline/compose.rs:296`, `algebra.rs:278` |
| "Distinct interior knots with multiplicities" | ≥4, because `KnotVector` only offers `multiplicity_of(u)` — the query every consumer actually needs is the one the data structure makes awkward | `compose.rs:274`, `algebra.rs:563`, `geom-curves/fit.rs:378`, `sweep/skin.rs:370` |
| Prefer-intrinsic upgrade rule | 3, with **3 different sample schedules**: validator uses `CERT_SAMPLES`; `revolve/upgrade.rs` hardcodes `let samples = 9u32`; `extrude.rs` uses a *single* midpoint with no lane gate. The doc claims "the SAME quantity, the same predicate name" — true only by coincidence of the literal 9 | `revolve/upgrade.rs:198`, `extrude.rs:1044`, `validate.rs:1994` |
| Planar divergence-theorem volume | `step-export/volume.rs` re-derives what `props::planar_face` computes, strictly weaker (planes+lines only) and reading its sign with a raw `volume < 0.0` outside the trilean discipline | `step-export/src/volume.rs:88` |
| Negative-zero flush helper | **FIXED by #704** — all four copies call one home, `step-import/src/signed_zero.rs`, and a CI gate now fails a fifth. The two later copies were byte-identical to *each other*; the home was the variant | `step-import/src/signed_zero.rs` |
| Deep-snapshot test helper | ≥4 in `topo` alone, duplication named as intentional in its own doc comment | `fixtures.rs:87`, `review_m1_pr2/mod.rs:35`, `review_m1_pr3.rs:44`, `tests/box_with_hole.rs:368` |

**Verdict:** ACCEPTED (Evan, 2026-08-18) — see S16. "They should certainly
be unified."
**Postmortem (2026-08-18), row by row.**

- **Rational quotient rule ×2 (`ssi/enclose` vs `mesh/nurbs_cert`)** — the
  orchestrator **prescribed a fresh derivation**: M8-5 was *"built on the
  MERGED M8-2 template — **deliberately NOT lifting #309's unmerged
  machinery**"*, #309 being a concurrent slot in the same dispatch block. The
  already-merged `ssi/enclose.rs` home was named by nobody. **NEVER FLAGGED**;
  the reviewer *"hand-re-derived the recurrences"* — verifying the formula
  independently rather than diffing it against the existing one.
  *Lesson: "don't lift the unmerged sibling's machinery" is correct lane
  hygiene and becomes permanent duplication unless the block that creates it
  also books a unify-after-both-merge item.*
- **…and again within `mesh`, curve vs surface** — both landed in the **same
  commit**, because the dispatch defined the unit as *"two gates in two
  files"*. The spec named two homes and never said "share one". **NEVER
  FLAGGED.**
- **Bulge arc ×3** — `geom-brep`'s copy is forced (it sits *below* `profile` in
  the DAG). `sweep/skin.rs` has no such excuse — `sweep` depends on `profile`,
  and the code says so: *"The profile crate's ratified bulge closed forms,
  verbatim (`profile::seg::build_seg`)"*. Copied because `build_seg` is
  `T: Decide`-generic and returns `SegIssue`. **NEVER FLAGGED** — self-disclosed
  in code, which likely read as diligence.
- **Knot insertion ×2** — two PRs the **same day** (#125, #136); the scalar type
  (`RingInterval` vs f64) was the stated reason, and the linear span scan fell
  out of not having a `KnotVector` to call `find_span` on. **NEVER FLAGGED.**
- **"Distinct interior knots" ≥4** — all four appeared within **five days of
  the `KnotVector` type itself**, whose accessor set was frozen in #125 *before
  any consumer existed*. `multiplicity_of(u)` requires you to already know `u`,
  so every consumer needing *the list* hand-writes the same scan. **NEVER
  FLAGGED** — by the fourth copy nobody was tracking.
  *Lesson: a data structure whose API was frozen one PR before its first
  consumer is the tell; "did you have to hand-scan? why isn't that on the
  type?" is a cheap review question nobody is asked to ask.*
- **Prefer-intrinsic ×3** — **FLAGGED AND PARTLY FIXED.** #152's review raised
  the adjacent concern and got *"the demanded set equals the certifiable set
  through the one-home `geom_brep::tangent_certificate_lane`"* — one home **for
  the predicate**. The *sample schedule* was left divergent in the same fix
  pass, and #166 then hardcoded `9` two days later.
  *Lesson: unifying a **name** without unifying the **schedule** it drives is a
  half-fix that reads as a whole one.*
- **`step-export/volume.rs`** — the immediate cause is a **granularity
  mismatch**: it needs *per-shell* volume for void classification and
  `topo::props` exposes only body-scoped `mass_properties`. **NEVER FLAGGED as
  duplication** — the module has been repeatedly revisited and each pass
  *documented* the divergence rather than closing it.
  *Lesson: a module that keeps being edited to explain how it differs from the
  canonical one is a duplication signal the process has no rule for reading.*
- **Negative-zero flush ×3** — **FIXED by #704**: `step-import/src/signed_zero.rs`
  is the one home, all four copies call it, and `scripts/gates/signed-zero-one-home.sh`
  fails a fifth. Same crate, three units, one week. **NEVER FLAGGED** — the concept
  was tracked only as a *fixture* property (`M7-LOG.md:694`, a byte-divergence
  class), never as code ownership. That citation was closer than it looked: the
  recognition flush **is** pinned, by `corpus_fold.rs:130`'s promoted one-cycle
  fixed point, in the file `M7-LOG.md:694` names. Two survival mechanisms: **(a)**
  the two copies were byte-identical to *each other* and the home was the variant
  (`x + 0.0` against a branch on `x == 0.0`), so the available catch — a diff
  between the copies — pointed at collapsing them into each other rather than into
  the home that already existed; **(b)** the helpers were `Vec3`/`Point3` while the
  reader's `as_real` holds one `f64`, so the shared thing sat one type level above
  the site that needed it. `pncad-py/src/py/doc.rs:1101` is the same line under a
  different rule (`__hash__`/`__eq__` consistency) and stays: no shared home exists
  today, and whether one should is a separate question.
  *Lesson: copies that match each other and not the original point away from the
  home that already exists, and a helper typed above its predicate cannot be
  called by the site holding the scalar.*
- **Deep-snapshot helper ≥4** — **policy, not drift**, and **FLAGGED AND
  OVERRULED** in the standing sense: the reviewer-suite independence exemption
  is ratified, and Evan re-affirmed it on this scan (S36). The fourth copy is a
  visibility constraint (`fixtures::deep_snapshot` is `pub(crate)`; an
  integration test cannot see it).
  *Lesson: a deliberate independence exemption needs an expiry — the suites
  were never combed back, so a temporary licence became four permanent copies.*

## S19. Stringly-typed catch-alls in a codebase whose thesis is closed typed errors

- **Confidence**: sure
- **Found independently by five scans.**

| Variant | Sites | Problem |
|---|---|---|
| `AssemblyUnsupported { detail: &'static str }` | 146 | Mixes user refusals with kernel-bug assertions ("a corner left two spur struts (kernel bug)"); `Display` unconditionally appends a recourse telling the user to fillet every edge of a convex trivalent polyhedron |
| `MissingEntity` | 49 | Documented as "corrupt input"; also carries "non-iso trim carrier reached the iso-rectangle walk (**router defect**)" — kernel bugs reported to the caller through the dangling-key variant |
| `UnsupportedCarrier` | ~20 | Doc and `Display` say something specific and *different* from what most sites mean; sits in the same enum as `IsoUnsupported { what }`, which does name the refused class |
| `SplitJoinError::Corrupt` | ~42 | Payload-free — no key, no half-edge, no face — beside `SplitReduceError`, which carries the offending entity and `Indeterminate` diagnostics on nearly every variant. Also used for "the section frame was never established", which is not a corruption |
| `ValidationError` | 59 variants | Spans four validity tiers; tier membership lives in doc-comment prose, so `validate()`'s signature promises nothing and no consumer can exhaustively handle "the structural failures" as a set |
| `pncad-py/tags.rs` | 384 lines | Hand-maintained enum→`&'static str` map that drops every payload the kernel error carries; three sites use `format!("{err:?}")` as the human message despite `value.rs:78` asserting "never a `Debug` dump" |
| `SkippedMerge { reason: String }` | — | `merge_coplanar_faces` runs two incompatible failure regimes depending on a property of the input, and flattens typed errors into `format!` strings for one of them | `merge_faces.rs:489` |
| `ProgramRefusal::Geometry` | — | Degrades a kernel typed refusal to `String` because `PathError` is not `PartialEq` | `program.rs:846` |

**Verdict:** ACCEPTED (Evan, 2026-08-18). "Ha these are funny (and also show
again that we need a bug vs reachable invalid state distinction)" — i.e. this
finding is **downstream of S43**. The postmortem pass is asked to substantiate
or refute that: how many of these catch-alls exist because the author had no
vocabulary for "this can only be a kernel bug"?
**Postmortem (2026-08-18). Evan's hypothesis is substantiated for the three
highest-volume rows, and S43 turns out to be their generator.**

- **`AssemblyUnsupported`** — born legitimate at #166 (9 sites, an explicit
  "front door does not exist yet" marker). #171 then declared the idiom
  `let unsupported = |detail: &'static str| …` at the head of ~12 functions,
  and that closure is what took it to 146: the overwhelming majority are
  `.ok_or_else(|| unsupported("a ring does not walk"))` — arena lookups that
  cannot fail on a valid body, **five of which say "(kernel bug)" outright**.
  #171's deviation 3 declared only the four genuine scope gaps; ~120 assertion
  sites rode in under that sentence. **NEVER FLAGGED** — #166's F6 is the
  closest miss: the reviewer was *inside this enum*, split a tangential case
  into its own variant, and still did not remark on 146 sites sharing one
  string. *Count as scanned:* **#688's retirement took `crates/sweep/src`
  from 172 to 119 `AssemblyUnsupported` sites**, so B3 re-counts before
  scoping rather than inheriting 146.
- **`MissingEntity`** — born a catch-all in one commit (28 sites on day one)
  because the crate needed *some* error for "the arena did not return the key I
  was handed". #157 then used it for a genuinely different thing (`"router
  defect"`). **NEVER FLAGGED.**
- **`SplitJoinError::Corrupt`** — the decisive evidence for Evan's hypothesis.
  **The same commit** (`f814900f`) mints `PointInLoopError::CorruptLoop
  { r#loop }`, `UnclassifiableComponent { shell }` and `CutInvariant { edge }`
  (doc: *"kernel bug, loudly"*) **with** payloads — and `Corrupt` **without**.
  The author was improvising a bug channel per-module because the codebase
  never gave them one. **NEVER FLAGGED**: the reviewer ran a referential-
  integrity falsification, confirmed `Corrupt` never fires, and therefore never
  had to read one.
  *Lesson: a refusal the suite proves unreachable gets reviewed for
  reachability, never for diagnosability — precisely backwards, since the day
  it fires is the day someone needs the payload.*
- **`UnsupportedCarrier`** — born narrow and **true**; broadened by #192's Leg E
  adding ~10 sites for degenerate *configurations* (apex-level rim, Villarceau
  circle), none of which is a carrier-form mismatch. The sibling
  `IsoUnsupported { what }` **existed and names its refused class**; it was not
  extended. **NEVER FLAGGED as an overload** — but *very* narrowly missed: two
  of M6-3's five MINORs were about these exact variants (untested arms, an
  executed-false comment). *Lesson: "is this variant reachable and tested?" and
  "is this variant **true** at this site?" are different questions, and only
  the first is on the checklist.*
- **`ValidationError`** — nobody chose a catch-all; the **tier simply never
  became a type**. **FLAGGED AND OVERRULED on a different axis**:
  `GENERICS-BUILD-COST.md:169` measured it formally (*"57 variants; its
  `Display` impl is 310 source lines producing 7,328 IR lines"*) and ruled
  *"**This is reported, not flagged as a defect.** It is the D9 fail-loud
  charter working as specified."*
- **`tags.rs`** — downstream of a **kernel-side gap**: neither `EditError` nor
  `NodeErrorKind` implemented `Display`. **FLAGGED AND PARTLY FIXED** — #308's
  F6 was *"REOPENED on review and implemented"*, landing `Display` on 34 + 33
  arms. The residue is recorded in the same body: `StepImportError` *"is
  out-of-fence for a Display impl"*, which is exactly why three sites still
  `format!("{err:?}")`.
- **`SkippedMerge { reason: String }`** — **the reverse of the others.** Born
  with a generic label; **the reviewer killed the label**. F3, verbatim:
  *"`Err(_)` catch-all launders tier-2 diagnostics → preserve real reasons."*
  The implementer discharged "carry the actual diagnostics" with `format!`
  rather than an enum — and `DESIGN.md` D4 ¶2 **later cites this outcome as the
  in-repo precedent**.
  *Lesson: a finding phrased as an **information** requirement gets discharged
  by `format!` unless phrased as a **type** requirement.*
- **`ProgramRefusal::Geometry`** — `EditError` requires `PartialEq`;
  `profile::PathError` deliberately derives no equality. **NEVER FLAGGED**, and
  #291 is the strongest possible "we looked": a **dual** review, two MAJORs
  found by both reviewers independently, F1–F8 all reported — and neither
  challenged the declared String degradation.

**On the bug-vs-invalid-state question.** Substantiated for ~239 of ~260 sites.
**54 source sites say "kernel bug" in prose**; `DESIGN.md` uses the phrase
exactly once — in D9's footnote, which is also the ruling that *authorises* the
smear: *"Corrupt in-crate states get typed errors where cheaply detectable, or
documented garbage-out in release."* So these variants **do not violate the
ratified contract; they are its only sanctioned option**, which is why no
reviewer could flag them as violations. **S43 is not a neighbour of S19 — it is
the generator of its three largest rows.**

## S20. FIXED by #689 — the façade layers each invented a vocabulary instead of forwarding

- **Where**: `crates/pncad/src/`
- **Confidence**: sure

**FIXED by #689.** `closure.rs` — 151 lines compiling to nothing — is
deleted, and its audit output went where `LIB-U1-SPEC` §3 said in the first
place: the PR body. `select.rs` went **461 → 55**, its prose becoming
`docs/guide/selecting.md` pulled into rustdoc, with all six code blocks
**byte-identical** after the move and the doctest count unchanged at 34.
`lib.rs`'s "no behavior of its own" claim now says what is true about
`workspace`, and `workspace.rs`'s own header no longer claims a read-only
file that ships three write doors. Narrative went ~1170 → ~700 lines against
194 of code.

**The manual sync obligation is now mechanical**:
`every_document_layer_root_export_is_carried_or_listed` checks all 240
`editor-core` root exports in both directions against an 87-name
reasoned `NOT_CARRIED` list, and is falsified by execution. Its
name-vs-path hole — matching leaf names rather than statements, so a future
`editor-core` export colliding with a name the façade re-exports from
another crate would pass while uncarried — was **closed rather than
disclosed**; the two blind spots that remain are named at the test and
routed to **#696**, which also files the rustdoc-JSON check that three
separate guards had been deferring to an unscheduled nightly.

**One report claim was wrong, and the check that established it is the
useful part.** S20 said `workspace.rs:432` classified kernel faults by
**string-matching** on `PersistError`. It does not, and no revision ever
did: the line is a typed `matches!`, a sweep for classification-by-string
across two statements returns zero, and walking the last 30 commits touching
the file finds `contains(`/`starts_with(` in **none** of them. The real
defect was adjacent — the classification ended in a **wildcard**, so a new
arm would silently read as `Unresolved`. Removing it immediately caught two
`PersistError` arms a hand-written list had missed. *A scan reading a
`matches!` as string-matching is a false positive that pointed at a true
defect one line over.*

**Not closed, and labelled:** the *class* S20 names — per-unit narrative
accretion into a façade file no unit owns — has no mechanism. The ~700-line
figure lives in a PR body with no guard and no scheduled re-measure. The
same PR demonstrated why that matters: its rewritten `lib.rs` contract
shipped a **stale count** (*"six of the seven seam functions"*; there are
six, the seventh removed in `58bcc54f` with its gravestone still in the
file) in the commit whose subject was making `lib.rs` state what is true.
That one is now a shape rather than a count, guarded by
`the_authoring_seam_roster_is_what_the_crate_doc_claims`; three sibling
unguarded counts are recorded as a class and not swept.

Meanwhile `lib.rs:36` states "the façade contains no geometry and no
numeric behavior of its own. Every item below is either a re-export or a
thin wrapper" — and `workspace.rs` is 260 lines of real subsystem:
directory scanning, header parsing, duplicate-id detection, save/resave,
OS entropy, and an `impl PartResolver` that classifies kernel resolve
faults by **string-matching** on `PersistError` variants (`:432`).

**Verdict:** ACCEPTED (Evan, 2026-08-18). "Yikes, nice catches, will need to
think about how to fix these."
**Postmortem (2026-08-18).**

- **`closure.rs` (151 lines, zero code).** `LIB-U1-SPEC` §3 commissioned an
  *audit* and said to **"List the audited enums in the PR body"**; the
  implementer committed the audit as a module instead, and blinded review then
  made it *bigger* — #232's MINOR-2 added nine missed types plus a rationale
  for each miss. **FLAGGED AND PARTLY FIXED**: the module's *claim* was
  falsified hard (the reviewer compiled `use topo as _;` to disprove the
  advertised "physically incapable" proof, `MODEL-AB-LOG.md:571`) and rewritten
  into today's honest `:135` paragraph — but its *existence with no code* was
  never mentioned in the findings, the LIB-LOG row, or the A/B row.
  *Lesson: when a spec commissions an audit, name where its output lives, or
  the audit becomes a source file forever.*
- **`select.rs` (449 comment lines before one `pub use`).** Born at 177 lines
  in LIB-U7; each later unit appended its own titled section to the same header
  (U7 → U5 → SEL1 → SEL2 → SWITCH-E). No single diff was unreasonable; no unit
  ever owned the file's total length. **NEVER FLAGGED** — checked every
  touching unit's LIB-LOG and A/B row; the doc rubric never dropped for module
  size. *Lesson: per-unit review cannot see an accumulation, because the
  accumulation is never in a diff.*
- **`lib.rs:36` vs `workspace.rs`.** The "no behavior of its own" claim was
  written in LIB-U1 (#232) **when it was true**; four days later
  `ASM-1-SPEC.md` **D-5 deliberately placed a real subsystem there**, and PR
  #364 delivered it. Nobody re-read a crate doc written by a different program.
  **NEVER FLAGGED** — ASM-1's dual review returned 0 MAJOR from both reviewers
  and neither brief covered a claim made in another unit's file.
  *Lesson: a cross-program contradiction belongs to no PR, so no per-PR review
  protocol will catch it.*

## S21. FIXED by #689 (hole 1) / instrumented and re-filed as #694 (hole 2) — two concrete holes in the Python surface

- **Where**: `crates/pncad-py/src/`
- **Confidence**: sure

**Hole 1, the constant `DocumentId`: fully closed by #689.** `Doc()` mints a
fresh random id and `Doc(label)` derives one deterministically, with
`IdentityError` refusing on an entropy failure rather than falling back —
there is no remaining path to a constant. Minting lives in a pyo3-free
`pncad_py::identity`, so it is exercised on the default no-Python CI path.
The guard is the invariant itself rather than the constructor: two documents
minted through the seam `Doc()` uses are asserted distinct, `Workspace::create`d
into **one workspace**, and the directory re-opened cold. The residue —
Python still cannot *use* identity — is **G15** in the north-star audit,
whose `test_the_named_gaps_are_still_gaps` fails the day a door lands. That
register has fired four times already, so unlike this finding's own
postmortem it is not a deferral into a closed register.

**Hole 2, three types named `DimensionError`: instrumented, not closed —
and the instrument's premise was false.** #689 renamed the Rust-internal
struct (three names to two, no surface change) and declined the published
Python rename, arguing the collision was latent because the kernel enum
reaches Python through exactly one door, `Expr::literal`, and no bound door
binds the `Expr` operator builders.

**Review falsified that by execution.** `persist/wire.rs`'s
`WireExpr::rebuild()` calls `Expr::add`, `mul`, `sin`, `atan2` and the rest —
those *are* the operator builders — and it sits on the **load** path, which
Python binds as `pncad.load(text)`. Probing found **seven of the ten** arms
reachable today with no new binding, all arriving as `PersistError`/`parse`
with the structured refusal Debug-formatted into a message. The general
shape, worth more than the instance: **reachability argued from the
authoring doors while ignoring the deserialization doors** — every
`Deserialize` impl in `wire.rs` re-runs a smart constructor, so every kernel
refusal reachable from one is reachable from `load`.

The *decision* survives on narrower ground (no bound door raises Python's
`LiteralError` for a genuine dimension mismatch, so the published rename is
still not owed), but the reasoning had been shipped on the **published**
surface in four places — `.pyi`, both `create_exception!` docstrings, and
`ErrorClass::Literal` — all now corrected and citing **#694**, which carries
the misrouting itself. The trigger test could not have detected its own
premise failing, S23's shape exactly; it is now scoped to the door it
actually covers, and a new row drives the seven arms through `load` and
fails when their class changes — with a vacuity guard that fires if the wire
shape moves out from under it.

`Doc::new` calls `ProfileDoc::empty_derived("pncad-py:Doc")`, so **every
document authored from Python carries an identical constant
`DocumentId`**. The workspace store treats id uniqueness as its central
invariant and refuses two files claiming one id, and `DocRef`/`ContentPin`
cross-document references are keyed on it — so two Python-authored
documents cannot coexist in a workspace, and per the assembly model they
are the same part. The docstring calls it "a recorded bindings-parity
pickup"; a constant where an identity is required is a hole, not a gap.

Separately there are **three unrelated types named `DimensionError`**:
`pncad_py::errors::DimensionError` (a 3-field struct for Python operator
mismatches), `pncad::document::DimensionError` (the kernel's 10-variant
expression-dimension enum), and the Python exception class. The
exception is raised only for the first; the actual dimension checker
surfaces in Python as `LiteralError` with a `kind` tag. A user who
catches `DimensionError` for a dimension mistake catches the wrong one.

**Verdict:** ACCEPTED (Evan, 2026-08-18) — see S20.
**Postmortem (2026-08-18).**

- **The constant `DocumentId`.** Not a view about identity — a **lane-contention
  artifact**. `ASM-1-SPEC.md` D-7 ruled "NO new Python doors in this unit…
  **minimizing the collision surface with the concurrent program**", so the
  implementer needed an id with no door and picked a label-derived constant.
  **FLAGGED AND DEFERRED — with no issue number and no milestone.** PR #364
  disclosed it plainly; both blinded reviewers read the disclosure and filed
  nothing. The register it was deferred into (`LIB-LOG.md:439`, the LIB
  residual register) **had closed the day before #364 merged**, and the one
  *machine-enforced* register — `docs/guide/north-star-audit.md`, whose test
  fails as doors land — has no identity/pin/workspace row. No open issue
  mentions it (all 39 checked).
  *Lesson: a deferral recorded only in prose is not deferred, it is forgotten
  on a schedule — the record has to land in the register that fails a test.*
- **Three `DimensionError`s.** Three mints, three units, months apart: the
  kernel enum at M4 PR1; the Python struct and exception class at LIB-U9S
  (#290) minted *because* the kernel type was not curated then; and LIB-DOORS
  F5 (#308) which re-exported the kernel's through `pncad::document` and
  **deliberately routed it to `LiteralError`** to preserve U9S's tag spellings.
  #308's body narrates that swap in detail and never observes the collision.
  **NEVER FLAGGED** — including by #308's dual review, which was *inside this
  exact code* and found the payload drop and the Debug-dump messages but not
  the name.
  *Lesson: nobody owns the name-space; a collision is only visible from the
  whole surface at once.*

## S22. Ambient state contradicting the purity thesis

- **Confidence**: sure for each row

| Mechanism | Anchor | Consequence |
|---|---|---|
| `Band::linear()` reads a `OnceLock` self-initialising from `CAD_TOLERANCE_EPS`/`CAD_AMBIGUITY_K` | `tolerance.rs:214`, `predicate.rs:352` | ε and K are ambient inputs to every predicate, sitting awkwardly beside the commitment that a model is a pure function of a parameter vector. The cost shows in four dedicated integration binaries existing purely to get process isolation |
| The `k_stats` verdict log delivers a **production value** by thread-local side effect; `start_verdict_log` overwrites unconditionally | `k_stats.rs:44`, `:264` | Measured: an `InstantiatePart` node records **0** verdicts where the same geometry records 722. The doc records the bug, warns readers off the mechanism it documents, and notes which of two remedies to take was "left open deliberately at merge" |
| **[FIXED — #664]** `mesh` reads ε twice where the crate docs and `DESIGN.md` D4 say once — and the second read **snaps** a value (as found; both halves are corrected below — one `Tolerance::get()`, and the snapping read is gone) | `walk.rs:730`, `walk.rs:496` | The mesh is a function of (body, δ, **ε**), not (body, δ) as the determinism/memo-key contract claims. F6 also bans "EPS snapping anywhere in the pipeline" |
| `same_chart` decides chart identity by `core::ptr::eq` on two `&Body` | `chart_region.rs:394` | In a module whose premise is "structural identity, never numeric identity". A caller passing a clone silently drops to the weaker rung |

**The `mesh` row is FIXED by #664 — route (ii), and the ε-vs-δ question
it asked is moot.** That row only; every other row of this finding
stands, the `CAD_TOLERANCE_EPS` ambience row emphatically included — it
is an open design question, not a defect, and nothing here touches it.
`walk`'s final meridian now takes its column from the loop's **closing
vertex** (`walk::closing_column`) rather than from the meridian
carrier's midpoint, so the two ends of the closing polygon side are the
same `f64` rather than the same analytic azimuth down two float paths.
The residue is **identically zero** on every governed closure —
`loop_polygon` asserts it bitwise after the walk — the snap is deleted,
and **no ε consumer in `mesh` snaps a value any more**. (That is the
exact claim, and it is weaker than "no ε-derived quantity can move a
value the mesh emits", which is false as stated: pole/apex
identification is a CLASSIFICATION whose outcome substitutes the pole's
exact `v` for `Chart::v_of` and emits two polygon entries instead of
one, both of which reach the bbox, the interior grid and the pole fan.
Its ε-dependence is structural and **unexercised by the in-tree
corpus** — nothing in tree puts a non-pole vertex within a suite ε row
of a pole — which is not the same as absent, and reachability is *not*
established either way (`revolve` would very likely refuse the sliver;
STEP import is the plausible route in). Both surviving copies of the
sentence now say that.) The
comparison survives as a `debug_assert!` inside `closing_column`, where
it gates nothing and therefore measures **data quality** (the
`nist_ftc_09` 21 pm off-axis line endpoint) rather than a tolerance,
which is what it was always really about. The predicate is kept —
nothing snaps, so `closure_is_snappable` became `gap_is_noise`, named
for neither the snap nor the closure — because two of its three
consumers are not closures: `curved`'s domain guard now CALLS it
instead of respelling the same rule inline, and #653 needs the same
shape one rung over.

Three things #664's fix pass added, each of which is the finding's own
lesson applied to #664 itself. **The detector now has a red row.** The
`debug_assert!` was the predicate's only production consumer and the
sole stated reason to keep it, and nothing made it fire — the four unit
rows test the predicate, and the one row that called `closing_column`
picked its eps and radius *so the detector stays quiet*. Removing the
assertion could only have redded a row that observes the panic, and
there was no `#[should_panic]` anywhere in `crates/mesh`. There is one
now. **`loop_polygon`'s retained `debug_assert_eq!` is documented as
what it is** — a revert detector, not a runtime guard: it compares
`out[0].u` with itself and cannot go red for any input, only for a
source edit (which is a real and useful job, and is what reds the
curved row under the revert). **And the closure census has one home**,
this document, rather than three hand-synced copies — the previous
census in `closing_column`'s rustdoc is exactly what went stale.

Measured A/B on current main (both trees release with
`debug-assertions`, wild corpus through `import_step`, δ ∈ {5 mm,
1 mm}, position bits and triangle indices hashed separately): **triangle
indices bitwise identical in all 18 cells; vertex positions bitwise
identical in 17 of 18**. `nist_ftc_09_asme1_rd.stp` — the file that
motivated the ε bar — is byte-identical. The one differing cell is
`stepcode/sg1-c5-214.stp`, positions only, at both δ, worst |Δ| =
1.9e-14 m.

**Two corrections to this finding as written, established by that
work:**

- **"`mesh` reads ε twice" is not literally true.** `mesh` calls
  `Tolerance::get()` **exactly once** (`tessellate.rs:43`) and threads
  `eps: f64` down. What was true is that several structural decisions
  *consumed* it. At HEAD there are three, plus the snap that is now
  gone: pole/apex identification (`walk`); `curved`'s banded
  swept-rectangle domain guard (#648), which decides only whether a
  face is **refused**; and the per-triangle certificate assertion in
  `trimmed`'s review probe, absent from a default build. Only the snap
  could SNAP an emitted value. So today: **one read, three consumers,
  none of which snaps.** Not "none of which can change a value" — the
  pole classification can, structurally; it is unexercised, which is a
  different and weaker fact, and the comments say the weaker one. The
  three stale *"ε is read once, for pole identification"* comments are
  corrected to say that rather than deleted — note that route (ii)
  alone would **not** have made the old wording true again, because
  #648 had already added the third consumer.
- **"All 18 nonzero residues sit in one wild file" was not true at
  HEAD.** The eighteen are all `nist_ftc_09`'s and reproduce value for
  value, but the wild total is **twenty**: `stepcode/sg1-c5-214.stp`
  carries two, at 5.84920e-13 and 5.84865e-13 rad on a 2.0e-2 m radius.
  It is excluded from the montage by **licence**
  (`WILD-CORPUS-LICENSES.md` D2), not by capability, which is how a
  census run off the montage cell set missed it. Wild total at HEAD:
  125 governed closures per δ, 20 nonzero. In-tree: 381 closures, 4
  nonzero, all 1 ulp and all from #648's one obliquely-placed fixture.

**Verdict:** ACCEPTED WITH QUALIFICATIONS, row by row (Evan, 2026-08-18):

- **`CAD_TOLERANCE_EPS`**: *"worth an investigation for good alternatives but I
  think this state is unfortunately basically correct — we need epsilon to be
  the same everywhere and so it's risky to pass it individually, but we also
  want it to be configurable without completely rebuilding. Would be very open
  to better ways to achieve those goals though!"* So the constraint is
  **two-sided** — uniformity *and* configurability-without-rebuild — and the
  finding should be read as an open design question, not a defect.
- **`k_stats` verdict log**: *"as you saw this is already tracked but is
  absolutely a problem that should be fixed."*
- **`mesh` double ε read + snap**: *"mesh should be streamlined and made clear
  but I think snapping is what we want? Not sure… maybe it should be delta
  rather than epsilon?"* — the open question is whether δ is the dimensionally
  correct parameter, not whether to snap.
- **`same_chart` by pointer equality**: *"good catch as well."*
**Postmortem (2026-08-18).**

- **`Band::linear()`'s `OnceLock`** — D4 ¶1 deliberately left "compile-time
  constant vs once-initialized" open; PR #3 closed it for one stated reason
  still in `tolerance.rs:6`: *"which is what lets the test suite run at several
  ε values with **zero test-code cooperation**"*. The L3 CI matrix predated the
  type and fixed the variable's name before the design conversation happened.
  **FLAGGED AND OVERRULED** — see below.
- **The `k_stats` verdict log** — `M4-PR4-SPEC` D2 specified *that* the diff
  engine consumes verdict logs but never *how* they arrive; the implementer
  chose an installable thread-local. Harmless until **ASM-2A (#414)** made
  `run_op` re-enter itself. **FLAGGED AND DEFERRED** (PERF-SCAN §2.4, *"(a) vs
  (b) is UNRESOLVED and was left so deliberately at merge"*) — **no GitHub
  issue**; all 39 checked. Near-miss: #414's own fix pass wrote *"a nested
  run's logs die with its evaluation"* — the interaction was seen **at the PR
  that created the defect**, but read as "the inner log is unobservable" rather
  than "the outer log is destroyed".
- **The mesh double-ε read** — M2 PR 6 shipped the pole read **and, in the same
  commit, the crate-doc claim it contradicts**. The loop-closure snap was
  originally a bare angular constant, which kept "ε read exactly once"
  technically true; **PR #481** correctly found that constant scale-blind and
  rewrote the bar as `residue · radius < eps`, threading ε into a second
  structural decision. **NEVER FLAGGED** — and the invariant it broke had been
  *pinned by an adversarial reviewer*
  (`survives_eps_row_bitwise_independence`), whose comment read
  *"ε is read once, for pole identification"* through the whole life of
  the defect. **The test kept passing**, because only a foreign STEP
  file produces a nonzero residue. (#664 removed that second read and
  rewrote the comment — which by then had to name a *third* consumer,
  #648's domain guard, rather than restore the original wording: a
  stale claim does not become true again by undoing the change that
  falsified it.)
  *Lesson: a regression test that pins an invariant on the corpus that existed
  when it was written keeps passing through the change that breaks it — and its
  stale comment then reads as evidence the invariant still holds.*
- **`same_chart`'s `ptr::eq`** — `M9-2-SPEC:35` defines the rung as *"one body,
  shared `SurfaceKey` ⇒ identical chart"* **without saying how "one body" is
  decided**. These are the only two `ptr::eq` uses in `crates/*/src` anywhere.
  **NEVER FLAGGED** — and #527 drew a dual review in which **both** reviewers
  independently broke *rung 2* (one forged a `PositiveArea` certificate) and
  neither touched rung 1.
  *Lesson: reviewers attack the rung carrying the interesting theorem and walk
  past the premise underneath it.*

### ε configuration: what was actually tried

**Evaluated and rejected:** a compile-time constant (PR #3 — loses the
multi-ε matrix; a const generic would fail the same test and appears nowhere);
hard-fail at init (*"rejected as a D9 panic exception with no compensating
benefit"*); and, in the widening direction, K folded into the same lock by
Evan's direction (#41) on the reasoning that K is "ε-style per-run
configuration".

**A document-carried ε already exists — and is the source of truth.** M4 PR 6
shipped `Doc::epsilon`, `DocEdit::SetTolerance`, the verdict-vector diff, and
`Tolerance::init_document_eps`, whose doc states the ranking: *"the recorded
value wins over an unread `ENV_EPS` — the ambient env mechanism is process
BOOTSTRAP, and a document that states its ε outranks it."* A mismatch refuses
as `ToleranceConflict` at load. **But `init_document_eps` still commits into
the same process-global `OnceLock`, so every predicate downstream still reads
ambient. Better source, identical ambience.**

**Never evaluated in the written record:** threading a `&Tolerance` to the
predicate funnels, or a session/context object. The nearest written analysis of
the threading shape in the whole repo is PERF-SCAN §2.4 — about the *verdict
sink*, not ε — and it names the intermediate design that transfers directly:
*"a sink threaded only as far as each crate's single `sign_within` funnel,
rather than to every call site."* Every deciding crate already has exactly one
such funnel. Nobody has written that argument down for ε.

**The telemetry-gating ruling was applied to the class and ε exempted by name
in the same commit.** #562 wrote the *"no ambient environment in the kernel"*
grep **and** allowlisted `tolerance.rs`, with an error message that
institutionalises the escape hatch: *"…**or ratify this file into the
allowlist**"*. Nothing anywhere argues that the `NURBS_PROBE` rationale —
*"changes shipped behaviour with no rebuild, no flag, and no call site to
review"* — applies verbatim to `CAD_TOLERANCE_EPS`, which it does. The
asymmetry is visible in the justifications: `test-utils/fuzz.rs` got a
**reachability** argument; ε got only a **usefulness** one. Two rules do partly
discharge the concern: `init_document_eps` demotes env to bootstrap, and
`env_init_errors` makes a malformed value fail loudly.

**Costs on the record:** four single-test integration binaries exist purely for
process isolation; issue **#415** (closed, tolerance-init red on pristine main);
**#497** (open, a suite reds under a malformed ambient ε); **#470** (open, 251
statically ε-free tests re-executed three times) — and note #470's framing,
the strongest defence of the status quo in the repo: the zero-cooperation
property is *ratified*, so its proposed fix is to **observe** the `OnceLock`
rather than let any test **declare** itself ε-invariant.

### What the mesh snap actually protects against

**The mechanism.** `out[0].u` and the final column are the same analytic
azimuth reached by two float paths — `atan2` of the closing **vertex** vs
`atan2` of the carrier **midpoint**. The snap makes that polygon side bitwise
vertical before the CDT sees it as a constraint.

**Without it**, the side is a hair non-vertical and the constraint polygon can
self-cross. That is real — M2 PR 6's review blocker was exactly this — but the
guarded outcome is *"this face refuses to tessellate"*, not a silently wrong
mesh. **However**, the code comment's fallback story ("the CDT pre-check
refuses it") **was falsified by #481**: assert and snap read the same constant,
so exceeding it disabled both, and the unsnapped polygon **shipped into the
committed montage**.

**Measured effect** (#481, release STLs byte-compared over the wild corpus):
**7 of 8 cells byte-identical**; on the eighth, same 2148 triangles, **vertex
set bitwise identical (all 1008 points)**, normals bitwise identical, volume
unchanged — **16 quads split the other way**. The snap's only observable effect
is CDT diagonal choice on near-degenerate planar quads.

**Where the residue comes from** (#486 census, 315 governed closures): 266
bitwise exact; worst tour residue 4.0e-15 rad (9 ulps). All 18 nonzero residues
sit in **one wild file**, traced to a STEP file writing ~10 decimal digits and
stating a hole-axis line's endpoints **21.4 pm apart perpendicular to the
axis**. Not accumulation, not unit conversion — **source-file coordinate
rounding.**

**Would δ be right?** Both bars are metres, so both typecheck. **For δ**: by
the crate's own doctrine δ is "how far the mesh may sag" and ε "defines what
the *model* is" — the snap changes the mesh, not the model, and a δ-bar
**restores `mesh = f(body, δ)`** exactly as `lib.rs` claims. **Against δ**: δ
is caller-chosen coarseness (tests sweep 0.2 m and 0.03 m), so a δ-bar would
authorise snapping residues six-plus orders larger than anything measured, turn
the `debug_assert!` into a no-op at coarse δ, and make the constraint polygon's
*topology* depend on the view parameter.

**The third reading, which the history supports: the quantity is not a
tolerance at all.** #486 recorded two routes to actual exactness, both in the
`walk.rs:710` comment, both *"recorded, not decided"* — **(i)** re-mint such a
line onto a single azimuth at import through the existing
`StructureNormalization` healing class (a 21 pm move; caveat: vertices are
shared); **(ii)** have the final meridian take its column from the **closing
vertex** rather than the carrier midpoint — exact whatever the skew, mutates no
geometry (caveat: touches the anchor-branch choice tuned for wedge angles
> 3π/2). **Route (ii) makes the residue identically zero and moots the ε-vs-δ
question entirely.**

**Route (ii) is what shipped (#664), and its caveat was false.**
`unwrap_near(raw, prev)` returns the representative of `raw` nearest
`prev`, so with `prev = anchor` the 2πk branch is *already* the
anchor's; replacing the value with `anchor` changes only the residue
inside that branch and cannot move the branch. Demonstrated by
execution: reverting the substitution reds the two exactness rows and
leaves both wedge-angle rows green. What #486 called a design
conversation was a one-line substitution with no bearing on the tuning
it was thought to touch.

**On F6:** F6 bans ε snapping in the *boolean reduction/classification*
pipeline. This snap is display/export-layer and moves no kernel entity, so it
is not what F6 forbids. What it violates is the narrower mesh-local claim.

### S22 row 1 DECIDED (2026-08-19): keep the `OnceLock`; ε gets a provenance channel, not a thread

**Evan, 2026-08-19: keep the `OnceLock`. Do not thread ε.** This decides
S22's **first row only** — `Band::linear()`'s ambient ε and K. The other
three rows (the `k_stats` verdict log, `mesh`'s double read + snap,
`same_chart`'s `ptr::eq`) keep their 2026-08-18 verdicts and are
untouched by this.

Two things settle it.

**1. No mixed-ε assemblies.** That removes the only *functional*
motivation for a session object. The one thing the current design
genuinely cannot do is resolve an assembly whose member documents record
different ε — `crates/pncad/src/workspace.rs:433`,
`ResolveFault::EpsilonSeam`. With mixed-ε assemblies ruled out, that
limit is **intended behaviour, not a defect**, and the session design has
nothing left to buy.

**2. Threading is already shipped, twice, and one instance
degenerated.** `crates/profile/src/validate.rs:802` is exactly the design
under consideration — `pub fn validate(&self, tol: Tolerance)`, band
built once at the funnel, `Tolerance` is `Copy`. Measured: **256 call
sites in the workspace, and zero pass anything other than
`Tolerance::get()`** — while `profile` still calls `Tolerance::get()`
internally at `path.rs:1292`, `path.rs:1486`, `path/family.rs:242` and
`:310`, so the crate carries *both* mechanisms. It bought a signature
that documents the dependency and no configurability at all. `mesh` is
the second and cleaner instance (one `Tolerance::get()` at
`tessellate.rs:42`, threaded down) and it works because `tessellate` is a
leaf pipeline with one entry point. `topo` is not: ~40 `Band::linear()`
sites across 22 files.

Also decisive: **the `OnceLock` is the only thing structurally enforcing
one ε per process.** Threading deletes that enforcement in exchange for
documentation. The postmortem's own defence of the status quo stands
unrebutted — the zero-test-cooperation property is ratified, and it is
the lock that provides it.

**What the finding got right, and what is being implemented.** The row's
real content was never "ε is ambient"; it was that ε is ambient *and
silent*. A stale `CAD_TOLERANCE_EPS` in a shell changes what
"coincident" means with no output line saying so, which is why **#497**
(open) and **#415** (closed) exist and read as mysteries. Two things
follow, both in this PR:

- **An ε provenance channel** (`crates/geom-core/src/tolerance.rs`).
  `struct Global` gains `EpsilonSource` — *compiled default / env /
  explicit `init` / document* — written by whichever path won the
  `get_or_init`, read back by `Tolerance::eps_source`, and rendered with
  the committed value and any rejected env value by
  `Tolerance::report` / `Tolerance::committed_report`. `pncad::tolerance`
  is the curated door; the demo runs print the line. The distinction the
  channel has to draw is *"an env bootstrap that nothing overrode"* vs
  *"a document stated this"* — `init_document_eps` already outranks env
  by committing first, and the channel now says which one happened.
  `committed_report` is the non-committing door, so reporting cannot
  itself bootstrap ε and turn a later load into a `ToleranceConflict`.
- **The `no-ambient-env` gate's justification**
  (`scripts/gates/no-ambient-env.sh`). Nothing anywhere argued why the
  `NURBS_PROBE` indictment — *"changes shipped behaviour with no
  rebuild, no flag, and no call site to review"* — does not apply
  verbatim to `CAD_TOLERANCE_EPS`, which is what made the allowlist entry
  read as special pleading (and `memories/telemetry-gating.md`, where the
  rule used to live, no longer exists — created by #562, deleted in
  `dd6d1990` / #615). The rule is now stated where it lives: an ambient
  channel escapes the indictment when **(1)** the value is a
  contract-ratified parameter of the model rather than an implementation
  switch, **(2)** it is committed once and immutable, **(3)** the
  committed value and its provenance are *reported*, and **(4)** a more
  authoritative source either wins or refuses. `NURBS_PROBE` had none of
  the four. `CAD_TOLERANCE_EPS` had 1, 2 and 4 and **failed 3** — which
  is exactly what the provenance channel fixes, so the allowlist entry
  becomes an instance of a stated rule rather than an exemption.

**What is explicitly NOT happening.**

- **No threading.** No `&Tolerance` parameter added to any predicate
  funnel, and `profile`'s and `mesh`'s existing threading is left exactly
  as it is — it is the evidence, not a target. (Whether `profile`'s
  double mechanism should collapse is a separate question nobody has
  asked.)
- **No session/context object.** Its only functional payoff was mixed-ε
  assemblies, which are out of scope by decision.
- **No per-model ε.** D4 ¶1 already rejects it and this changes nothing
  there.
- **No decision moves.** The ranking (document ε outranks an unread
  `CAD_TOLERANCE_EPS`), `ToleranceConflict` at load, `env_init_errors`
  and its loud test, and the evaluate-time ε check
  (`eval/mod.rs:971`) all stay bit-for-bit as they were. Provenance is a
  channel: no kernel predicate reads `EpsilonSource`, and no test had to
  be edited to accommodate it — the zero-test-cooperation property is
  untouched.
- **#470 is not decided here.** Evan is re-deciding it separately after
  being shown that the issue defers itself.

**The prose obligation this creates** is a separate, non-self-merging PR:
the purity thesis and D4 ¶1 currently let ε read as an implementation
detail, which is what makes this row look like a contradiction with
[the central commitment](#the-central-commitment). What is actually true
and now ratified — **the model is a pure function of (parameter vector,
ε)**, ε being a declared run parameter with a recorded provenance, one
per process by construction, mixed-ε assemblies out of scope — belongs in
`docs/DESIGN.md`, marked `PROPOSED` pending sign-off exactly as #628 did
for the D2 addendum.


## S23. The exhaustiveness sweep degrades silently to seed-generation

- **Where**: `crates/geom-brep/src/ssi/exhaust.rs:140`, `:267`,
  `crates/geom-brep/src/ssi.rs:711`
- **Confidence**: likely

**FIXED by #617.** The subdivision's duty is now a parameter the caller
states rather than a condition read off `tubes.is_empty()`: seeding
(`seed_r3` / `seed_chart_plane`) takes no tube set and returns no
receipt, accounting (`account_r3` / `account_chart_plane`) returns only
the receipt and refuses `ExhaustivenessInconclusive` at the floor
whatever its tube set holds, empty included. Since only the accounting
duty can produce a receipt, the identity `examined == excluded +
accounted + refined` now holds by construction for every receipt that
escapes the module. The floor row the Postmortem below names by its old
premise-carrying name is now
`the_floor_clamped_planted_fixture_refuses_typed`: the body is kept (it
is the only row exercising the floor with a NON-empty tube set) and the
premise it never checked is gone from the name. The chart lane's own
empty-tube row landed as **H7** (#633), tests only: it needed a fixture
neither existing wall could supply — `hull_slack_wall`, a cubic × linear
patch whose control net reaches 0.05 m past the cutting plane while the true
surface comes no closer than 0.002 m. That 25:1 hull-vs-truth gap is what
drives the sweep into the all-seeds-fail mode, and the floor ordering that
makes the row work is a fact about lengths rather than about ε.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18). Ratified before it was written — and the acceptance
row's premise excluded the failing mode.**

The dual role was **design, not drift**: `CURVED-DESIGN.md:796` argues the cost
of in-op exhaustiveness away with *"the exclusion subdivision doubles as the
marcher's seed generator (C3) … one structure, two duties"*, and PR #146 sells
it as a feature. The implementation expressed "two duties" as one function
branching on `tubes.is_empty()`, and both call-site pairs pass `&[]` vs
`&tubes` **positionally** — nothing in the type or signature records which duty
is being asked for.

*Was the acceptance row written so it could not fail?* Effectively yes, and the
name says it: `the_floor_clamped_variant_refuses_typed_even_though_branches_were_found`
— its **premise is that branches were found**, i.e. `tubes` is non-empty, which
is exactly the mode that cannot exhibit the bug. A later ε-honesty fix then
*widened* that row to accept a second error variant too.

**NEVER FLAGGED.** Checked PR #146's body (2 MAJOR / 6 MINOR, all named),
`M5-LOG.md:1359` (the full review return), `MODEL-AB-LOG.md:249`, and the
GitHub reviews. The closest anyone came is the reviewer's **own adopted probe**
`the_tiny_pair_floor_variant_refuses_typed`, which panics on *any* `Ok` — the
one row in the repo that would catch this mode. Its fixture just never enters
it.

*Lesson:* when a design doc argues a cost away by giving one structure two
duties, the duty must become a parameter the caller states — otherwise duty B's
acceptance row gets written on a fixture that guarantees duty B.

## S24. The assembly gate's success path is documented as unreachable

- **Where**: `crates/editor-core/src/assembly.rs:42`, `:314`,
  `crates/editor-core/src/mate.rs:44`
- **Confidence**: likely

The module doc states plainly that a declared cross-instance contact
"still ends at the chart door's typed refusal, surfacing as
`CensusUnsupported`" — i.e. `assemble` cannot return `Ok` for any
document whose mates actually declare something. On top of that, `mint`
refuses every class but `Rest` with `NoAtRestRecord`, so `Tangent` —
which `mate.rs` advertises as admitted in v1, and which the solver folds
happily — can never assemble.

This may well be the intended frontier posture rather than drift; the
scanning agent could not tell. What makes it a finding either way is
that a shipped door whose success arm is known-unreachable, with the
explanation living in prose rather than in the type or a feature gate,
is hard to tell apart from a bug when it fires.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S25. FIXED by #692 — two ε vocabularies flowed through SSI with nothing reconciling them

- **Where**: `crates/geom-brep/src/ssi.rs`, `ssi/march.rs`, `ssi/certify.rs`
- **Confidence**: sure

**FIXED by #692.** The knob turned out to be **misplaced, not redundant**,
and that is what made a type the answer rather than a deletion. Every
*certifying* call site was already passing `band.zero()` — `edge_nurbs.rs`
silently, `pcurve_cache.rs` with a comment, the rest via `SsiDomain::eps`.
Exactly one site genuinely differed: `review_m5_pr7b_ssi.rs`'s
`trace_deviation`, which marches at a fixed ladder while the run is banded
at ambient ε, deliberately measuring the marcher against itself — and it
goes through `trace_plane_nurbs_uncertified`, which produces **no
certificate**. So the decoupled knob has one legitimate home, and it is the
certificate-free door.

`MarchTol` is the bridge the postmortem asked for, at the signature:
`certify_rung3`/`certify_branch` lose `eps`, `SsiDomain` loses the field,
and `MarchContext`'s `f64` becomes a `MarchTol`, so **no bare `f64`
tolerance can enter the marcher**. Nothing loosened and nothing tightened —
review checked all seven decision sites individually and found every one a
substitution of one operand into an unchanged expression, with no
re-association and no changed evaluation order. The basis is now stated at
the door: `Band::linear()` stores `zero` unmodified through
`from_zero_threshold` → `Band::new`, so `band.zero()` is
`Tolerance::get().eps` **bit-for-bit**, not to within an ulp — with the
contingency named, since `certify_rung3` is `pub` and an angular band would
floor the ladder in radians.

**A name is not a rule, and the first attempt only had a name.** Review
demonstrated the hole rather than suspecting it: swapping
`MarchTol::from_band(band)` for `MarchTol::decoupled(…)` *inside*
`cylinder_sphere_ssi` — one line, no signature change, S25's exact
divergence reintroduced inside a certifying door — compiled clean with all
19 rows green. Decoupling was prevented neither by a type nor by visibility
nor by a test, only by the name being embarrassing: S25's own lesson one
layer in, where the second copy now enters as an innocent
`MarchTol::decoupled(…)` instead of an innocent `eps: f64`.

So the property is **enforced, not asserted**. `decoupled` is `pub(super)`
and the uncertified door takes a bare `f64` and mints its own, so no other
crate can produce one; and because `cylinder_sphere_ssi` sits in the same
module, both finishers additionally call `seam_tol(ctx.tol, band)`, which
refuses `MarchTolMismatch` **before** `fit_branch` runs. `SsiBranch` is
constructed at exactly two sites, both behind that seam; the only
`fit_branch` call that bypasses it is the uncertified door's. Every
certified branch carries the receipt as `SsiBranch::march_tol`, pinned by a
`MARCH-TOL` row.

**The most transferable finding is why the reviewer's mutation looked
green.** A *tighter* decoupled generator blows the sample budget, so the
fixture row **skips on `FitSampleBudget`** — and a skip reads as a pass.
The receipt row alone would not have caught it; the seam is what makes that
case loud. With the seam disabled, a 2× decoupling fires `MARCH-TOL`
directly and a 1000× decoupling escalates on `ssi_on_locus` — the
certificate refusing honestly at the band on a worse carrier, which is the
substantive argument, now demonstrated rather than asserted.

**Residue, scheduled:** the same class is open in `props/` at three
signatures (`quad.rs`'s `cylinder_cut_face`, `rational_patch_face`,
`nurbs_patch_face`) and two call sites (`topo/src/props.rs`'s `cut_face`,
`nurbs_face`) — **#699**, a Track A boundary this lane correctly stopped at.
The count is twice what #692 first disclosed; it was found by a sweep aimed
at that PR's *own declared blind spot*, parsing bodies as well as
signatures.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18).** Both vocabularies were born in the **same commit**
for a real structural reason: the marcher is deliberately the untrusted f64
candidate generator (`SSI_NEWTON_TOL`, `SSI_STEP_DEVIATION` are bare `f64`),
while the certificate's trilean decisions go through the generic `Band`. Nobody
wrote the bridge, so `certify_rung3` took both as independent arguments — and
each later non-SSI caller rediscovered the tie on its own (`pcurve_cache.rs:1012`
with a comment stating it; `edge_nurbs.rs:343` silently).

**NEVER FLAGGED** — searched #146's body, M5-LOG's nine fix items,
`M6-EXIT-WALK.md:30`, `M7-LOG.md:1042`, `MODEL-AB-LOG.md`, and
`predicate-dimension-audit.md`. The nearest thing on record is a *different*
rule: `M4-PR6-SPEC.md:51`, *"a process may not host two ε values simultaneously
— refuse loudly on conflict"*, which was scoped to the ambient/document ε and
never read as applying to a parameter list.

*Lesson:* a ratified "one ε per process" rule needs a signature-level
counterpart, because the second copy enters as an innocent `eps: f64` beside
the `Band`.

## S26. The certified area enclosure is never metered against anything

- **Where**: `crates/geom-brep/src/props/quad.rs:469`, `:803`, `:1800`
- **Confidence**: likely

`mean_boundary_displacement` reads `flux.width()` only; `area` enters
solely as the lever `(lo+hi)/2`. Both patch lanes compute area once at a
fixed `QUAD2_AREA_PIECES = 64` *before* the refinement rounds, and no
round recomputes it — so the flux enclosure tightens while the area
enclosure stays frozen at whatever width the Lipschitz pad produced.
`area.width()` appears nowhere in the file. That width rides out as
`MassProperties::area_pad`, which no gate checks either. A face can
certify with an area bracket orders of magnitude wider than its own
value; the only way it surfaces is the indirect `DegenerateFace` when
the symmetric pad drags `area.lo()` below zero.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18). Area was never meant to be an answer — it landed as
a denominator, and inherited no acceptance obligation of its own.**

PR #192's **numbered deviation 1** says the spec had area refused *"because no
gate consumes it"*; it shipped only because tier-3 check 7 meters
`v_hi / surface_area`. The commit message is blunter still: *"fixed-resolution
hull-rule area (the +V meter's denominator, **deviation report pending**)"*.

*Was the acceptance row written so it could not fail?* Yes — provably, by
contrast with its neighbour in the same file. `m5_pr11_quad_props.rs:71`
asserts `volume_pad < 1e-3 * half_exact` (a real tightness bound); the area row
at `:80` asserts only `area_pad > 0.0` plus containment. **Both conditions get
*easier* as the pad widens**, so no width regression can turn that row red.

**FLAGGED AND PARTLY FIXED, remainder explicitly deferred.** PR #472 hit the
extreme case head-on — it measured an area enclosure of
`[-1.7e20, +1.7e20]`, *"symmetric to the bit"* — fixed the unguarded division,
and ruled the metering question out of scope **in writing**: *"Metering against
`area.lo()` is the certified-conservative gauge and deserves its own proposal
with re-measured floors — not smuggling under a guard."*

*Lesson:* a quantity introduced as a denominator inherits no acceptance
obligation, and containment-plus-positivity is monotone in the wrong direction
— every certified **width** needs a row that goes red when it grows.

## S27. `props/quad.rs` is four independent quadrature engines sharing a file

- **Where**: `crates/geom-brep/src/props/quad.rs:509`, `:2381`, `:2107`,
  `:706`
- **Confidence**: sure

The ~2600 non-test lines hold four unrelated integrators: a
harmonic-channel 1-D Green integral for cylinder cuts, a B-spline 1-D
Green integral, a non-rational tensor patch flux, and a rational patch
flux via the quotient rule. They share almost nothing but `RingInterval`
and the funnel wrapper — each has its own substrate — and the
refinement-round → `mean_boundary_displacement` → `props_quad_converged`
→ `props_quad_face_extent` → `QuadratureBudget` block is copy-pasted
verbatim three times.

Two parallel constant families track this: `QUAD_INIT_PIECES`/
`QUAD_MAX_ROUNDS` and `QUAD2_INIT_PIECES`/`QUAD2_MAX_ROUNDS`/
`QUAD2_RATIONAL_MAX_ROUNDS`/`QUAD2_HULL_BLOCKS`/`QUAD2_AREA_PIECES`/
`QUAD2_REFINE_SPANS` — six hand-tuned magic sizes named after *which
lane arrived second* rather than what they bound.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18).** Pure accretion under the ratified per-arm
retirement doctrine (C12.1, *"retiring per-arm, never wholesale"*): 878 → 1910
→ 2759 → 3559 lines across four units. `QUAD2_*` exists because the first
family's names were taken. Each arrived as its own reviewed unit whose
acceptance was "this arm certifies", **so nobody ever owned the file**.

**FLAGGED IN A NEIGHBOURING PR / PARTLY FIXED — twice, each time only as far as
the immediate bug reached.** #313 shared *one* rule after a duplication-born
defect (the reviewer executed a certificate off by ~1111 widths and proved the
hole latent on main). PR #472 then **measured the duplication precisely** —
*"Unguarded at **four sites** — cylinder, rational, integral-exact,
integral-composite lanes"* — and factored out exactly the divisor, leaving the
convergence block triplicated. No review ever proposed splitting the file.

*Lesson:* when a fix pass has to state "unguarded at four sites", **that count
is the structural finding** and should be raised as one, not absorbed into the
bug's blast radius.

## S28. Three tessellation lanes are parallel pipelines with no shared core

- **Where**: `crates/mesh/src/curved.rs:114`,
  `crates/mesh/src/trimmed.rs:264`, `crates/mesh/src/planar.rs:222`
- **Confidence**: sure

All three independently re-implement the same seven steps — build a
`ConstrainedDelaunayTriangulation`, insert boundary points with a
hand-rolled `if h.index() == meta.len()` dedup, add loop constraints,
decide a `flip` from a shoelace sign, iterate `inner_faces()`, skip
id-degenerate triangles, emit. Each differs: `curved` uses
`can_add_constraint`+`add_constraint` and inserts grid points *after*
constraints (**not** the hazard `planar::triangulate_chart`'s header
warns about — that warning guards planar's crossing bookkeeping, which
`curved` does not build; settled below), while `planar`/`trimmed` use
`try_add_constraint` with crossing counts, and only `trimmed` classifies
intermediate vertices.

What sharing exists is ad hoc: `trimmed` imports `classify_faces`/
`edge_key`/`shoelace2` from `planar`, and both `trimmed` and
`tessellate` import the tolerance bundle `Tol` from `curved`. The common
code lives in whichever lane happened to grow it first.

**ORDERING HAZARD SETTLED by #648, and the premise under it is now a
GUARD — but only that half; the three parallel pipelines stand.**
`curved`'s grid-after-constraints order is **inert**, established by
execution rather than by reading. spade splits a constraint an inserted
point lands on and re-flags **both halves** as constraints, giving the
new vertex the next handle index — so nothing `curved` reads is
corrupted, and the bookkeeping the `planar` warning protects does not
exist in this lane at all. (#648 pins both spade facts as a row;
`spade` is a caret requirement, so a 2.x bump would otherwise move them
with nothing red.)

What was actually carrying `curved` is a different, unstated premise:
its grid runs the OPEN ranges `1..nu` × `1..nv` over the walk polygon's
own bounding box, which is strictly interior **iff the polygon IS that
box** (the swept-UV-rectangle contract). Nothing checked it, and the
router screens carrier KINDS, not loop SHAPE — `Circle` is a distinct
`Curve3` variant from `Ellipse`, so a keyway's own carriers (axial
lines, iso circles) are all invisible to `has_trim_carrier`, and an
un-notched cylindrical face carried entirely by `Line` + `Circle` was
executed all the way into `tessellate_curved`. On a notched domain the
grid splits boundary constraints and `inner_faces()` leaves triangles
outside the face: a silently wrong mesh, neither a typed refusal nor a
panic, and `tessellate` does not run `check_mesh`.

**#648 makes it a refusal**: `curved::require_swept_rectangle` runs the
O(n) off-bounding-box predicate on the walk polygon and returns
`TessellateError::UnsupportedCurvedDomain` — D2 addendum row 2 (valid
input, unbuilt lane), the `curved`-chart twin of `trimmed`'s
`SelfTouchingTrimLoop` and the structural sibling of `RingOnCurvedFace`
sixty lines above it, whose stated reason is this very contract. The
sweep (every chart this build authors, both hardest-walk shapes, a
partial-revolve sphere band, and the boolean-cut die pip, at two δ,
through the public entry point and `check_mesh`) shows nothing in tree
refuses.

**THE COMPARISON IS BANDED, IN METRES — the first form was exact and
that was a FALSE PREMISE (issue #653).** #648's first pass compared the
walk entry's coordinate to the box bitwise, justified in the code by
*"`crate::walk` assigns each side's constant coordinate once per EDGE
(never per point), so a rectangle side is bitwise straight."* That
holds only when the side IS one edge. An iso side carried by two or
more edges has each sub-edge derive its own column from `mid_azimuth`
→ `Chart::u_of` — an `atan2` of a **different point of the same
carrier**. Analytically equal; bitwise equal on axis-aligned dyadic
fixtures, which is why every in-tree fixture and the whole wild corpus
passed; ulps apart under a general rigid placement.

Adversarial review executed the counterexample: a frustum wedge
(`revolve` of a trapezoid through π/2) with its first boundary edge
split twice via the public `topo::Body::split_edge`, then placed by an
oblique rigid map. Pre-#648 it meshed 42 triangles `check_mesh`-clean;
the exact form refused it `UnsupportedCurvedDomain { off_bbox: 1 }` on
an entry **6.245e-17 m** off its own box — 1.6e7 inside ε = 1e-9. The
same shape is reachable with **no kernel mutator at all**: a
hand-authored STEP D-prism stating one cylindrical face's vertical
boundary as two collinear `EDGE_CURVE`s (what every exporter emits when
a vertex lands on that edge), placed obliquely by its assembly
placement, false-refused at **8.88e-18 m**.

The fix measures the gap in metres against the same band the module
already uses at the loop closure — `walk::gap_is_noise`,
`gap · lever < eps` (`closure_is_snappable` until #664 deleted the snap,
briefly `closure_gap_is_noise`, then named for neither once #664 made
this guard CALL it rather than respell it): `Chart::radial` for u — the entry's own
distance from the chart axis, so a cone and a sphere get their varying
lever arm — and the new `Chart::v_lever` for v. Over a 1524-row split ×
oblique-placement sweep the **worst** wobble anywhere was 1.4985e-15 m,
6.7e5 inside ε, while a genuine re-entrant corner is a feature width
off the box (the notch fixture's is 1 m). Fifteen orders of magnitude
separate the two populations; the band is not a calibration.

**Sweep A/B, 1524 `tessellate` calls (split × 3 placements × 15
bodies).** Exact form: 119 guard refusals. Banded form: **0**, and the
per-row status is **identical to a build with the guard removed
entirely** — so the guard as it now ships changes no output anywhere in
the sweep. Of the 119: 47 already refused for another reason (they go
back to those errors), 1 previously meshed watertight (the regression,
now clean again at 42 triangles), and **71 previously produced a
silently non-watertight mesh and still do**. That last group is #653's
other half and is a defect on main in its own right — banding does not
and cannot catch it, because its off-box residual (≤1.5e-15 m) is the
same phenomenon at the same scale as the counterexample's. The fix for
those is #653's option 2, taken below.

**OPTION 2 IS DONE: one constant coordinate per ISO SIDE, not per
edge.** A guard sitting downstream of the walk could only ever see a
symptom 1e-16 wide, and at that width the broken and the correct cases
are indistinguishable — which is why banding removed the false
refusals and could not touch the other half.
`walk::iso_side_starts` groups consecutive traversals into **iso-side
runs** and gives each run ONE coordinate, taken from the run's first
edge; a run's later members take that `f64` verbatim, so the side is
bitwise straight again.

**The sameness test is structural, not a band, and that is the part
worth carrying elsewhere.** A point off the chart axis has exactly one
azimuth, so two meridians meeting there are necessarily co-azimuthal —
there is no meridian-meridian corner away from the axis — and two
coaxial circles at different `v` are disjoint, so two rims that meet
are necessarily co-`v`. The only genuine same-kind corner is a CHART
SINGULARITY at the junction, which is the pole fan the walk already
emits two entries for and the π-apart wire band `unwrap_tie` exists
for. Every singularity lies ON THE AXIS, so `radial(junction) > eps` is
one comparison for all of them. It would also cover the horn/spindle
torus's axis point, which `Chart::poles()` does not list — but
`revolve` refuses both at construction, so that is a property of the
spelling and not a case anything can exercise; it is labelled
unexercised at the function and in its unit row rather than presented
as coverage. ε appears there as a **classification of the body**
(is this junction a chart singularity?) selecting between two exactly
computed coordinates, never as a snap of an emitted value — the
distinction #648's band and the loop-closure snap #664 removes sit on
opposite sides of.

**TWO RESIDUES THE ADVERSARIAL REVIEW FOUND, both recorded rather
than fixed, and both of the same shape: the PROSE claims more than the
code delivers.** The review could not falsify the grouping rule on any
chart kind, seam, torus direction or degenerate construction reachable
through the public API or `import_step` — 594 merges and 280 breaks
over ~1300 `tessellate` calls, worst merged disagreement 1.498e-15 m
against a minimum merged radial of 0.156 m and a maximum broken radial
of 1.68e-16 m, two populations fifteen orders apart with ε six orders
clear of each, and all 258 rim merges bitwise zero (which is the
independent confirmation that `split_edge` keeps one carrier).

1. **The argument's real premise is stronger than the one stated, and
   the sphere is where it leaks.** The argument is about azimuths and
   coaxial circles; `walk::classify` checks neither — a `Line` is a
   meridian unconditionally and a `Circle` is a rim iff
   `|n · axis| > 0.5`. The actual premise is *every boundary edge is
   an iso-curve of this chart*, which the walk never verifies.
   Cylinder and cone are safe by geometry. Torus leaks harmlessly
   (Villarceau circles classify as Rims when `minor/major < 0.866` but
   their centres are equatorial, so `rim_v` is 0.0 for all of them).
   **The sphere is the real one**: every plane section of a sphere is
   a `Curve3::Circle` and `trimmed::has_trim_carrier` diverts only
   `Ellipse` and `Nurbs`, so an obliquely-cut sphere face would arrive
   at this walk carrying two non-iso circles, and two consecutive such
   arcs meeting off-axis get collapsed onto one coordinate. That is a
   severity FLIP: per-edge the polygon would very likely have been
   REFUSED typed; collapsed, it can be its own bounding rectangle and
   be admitted. Closed today by two upstream gates — `topo::boolean`
   refuses the tilted plane × sphere section typed (executed) and
   `import_step`'s tier-3 `props::curved::sphere_boundary` admits only
   coaxial rims and centre-centred great circles (read, not executed:
   the one unverified door). It matters because
   `curved::require_swept_rectangle`'s own rustdoc argues its value as
   *"both can move without a line changing in `mesh`; this cannot"* —
   and `iso_side_starts` IS a line in `mesh` that can defeat it. Both
   rustdocs now carry the qualification.
2. **The `radial > eps` fallback is not refusal-safe, and the wording
   said it was.** Stricter than `poles()` is confirmed — `{poles()
   hits}` is a subset of `{radial <= eps}`, so a pole junction can
   never be swallowed. But when the bar breaks a run at a LEGITIMATE
   boundary vertex, `starts[k]` is `true` there and that junction gets
   exactly main's per-edge assignment: the side stops being bitwise
   straight, `entries_off_bbox` is banded and will not refuse it, and
   `tessellate` does not run `check_mesh`. The fallback **silently
   reinstates #653 for that face rather than refusing it** — a
   possible wrong mesh, not a wrong refusal. It is an unfixed residue
   and not a regression: the walk for that junction is byte-for-byte
   what shipped before. "Conservative" is now qualified in place with
   the direction it is conservative in. Nothing has reached it (cone
   slopes 1 … 1e-5, split distances 1e-9 … 1e-3 from both ends of
   every line edge, ε ∈ {1e-9, 1e-7, 1e-3}, identity and oblique — no
   junction ever landed at `0 < radial <= eps`), because `split_edge`
   is metred against the same band and `revolve` refuses near-horn
   toroids at a certified bar. **The named settling experiment, not
   run:** an `import_step` fixture stating a cone face whose generator
   is two collinear `EDGE_CURVE`s meeting within ε of the apex — that
   route has no `split_edge` gate at all, and it is the same door
   `split_oblique.step` walks through.

A third, smaller: *"one comparison for all of them"* is true of the
run-breaking DECISION and not of pole HANDLING. `Chart::poles()` is
empty for `Torus`, so `pole_v` returns `None` at a toroidal axis point
and the walk emits one ordinary entry rather than the two-entry fan
(`v = atan2(0, −major) = π`). Dormant — `revolve` refuses horn and
spindle at construction — and now said as a precision point at the
function.

**Sweep A/B, the same 1524 `tessellate` calls.**

| | main | with the fix |
|---|---|---|
| CLEAN | 1373 | **1500** |
| DIRTY (`Ok` + `check_mesh` failure) | **71** | **0** |
| REFUSED | 80 | 24 |

Row by row: 1373 CLEAN → CLEAN **at the identical triangle count**, 71
DIRTY → CLEAN, 56 REFUSED → CLEAN (44 `CertificateExceeded`, 12
`Triangulation` — the walk was handing the CDT a self-crossing
polygon), 24 REFUSED → REFUSED (all `CertificateExceeded` on the mirror
nappe, several under the *identity* map: the δ-vs-certificate class,
untouched). The 71 lose 90 triangles between them, one to three each —
the slivers, and nothing else. Zero guard refusals before and after.
The import route goes with them: `split_oblique.step` was 19 triangles
`NonManifoldEdge` and is now 18 triangles clean, exactly what the same
part stated axis-aligned already meshed. **A rigid placement stops
changing how many triangles a body takes.**

**A consequence for this entry's own band.** With the columns exact
again every walk entry in tree sits on its UV box *bitwise*, so
`entries_off_bbox` measures zero everywhere and the banded and exact
forms agree on every in-tree fixture:
`a_split_then_placed_swept_face_is_not_refused` no longer
discriminates between them, and its doc now says so. The band is
kept as the backstop, and what it guarded is asserted directly
(`== 0.0`) by
`a_multiply_carried_iso_side_is_bitwise_straight_and_meshes_watertight`
over every edge of every fixture, split and placed obliquely — **254
meshed plus 4 typed refusals = 258 configurations**, and the row's
floor is per-fixture (every edge must yield a placed body) rather than
a global count, so that total is reported rather than relied on.
Reverting `iso_side_starts` turns the row red with **50 crooked walks
and 33 non-watertight meshes**.

The band's own red-when-reverted row had to become SYNTHETIC, and that
is the honest ledger for this change:
`the_band_admits_a_sub_eps_entry_that_the_exact_form_refuses` feeds one
mid-side entry, nudged four ulps inside its box, through
`entries_off_bbox` twice — banded (admitted) and with a zero band
(refused). Before #653 the band had a live witness; the fix removed the
population that provided it, and a guard whose only evidence is a
sentence is a guard nobody is testing. The import route has its own row and
its own committed AP214 fixtures
(`step-import/tests/fixtures/split-iso/`, generator beside them) —
that is where the *separate-carrier* case lives, which no carrier
identity test could have decided.

**Payload.** `off_bbox: usize` alone could not tell the two apart — it
read `1` for a one-corner keyway and `1` for a 6e-17 m wobble, the same
message for *"re-author your part"* and *"kernel bug, file it."* The
variant now also carries the first offending `(u, v)` and the maximum
distance from the box **in metres**, which is the number that
classifies the refusal. S19's postmortem lesson, applied: *a refusal
the suite proves unreachable gets reviewed for reachability, never for
diagnosability.* Note also that had the original sweep printed margins
instead of pass/fail, the exactness claim would have been visibly
fragile before it shipped.

**Classification re-examined, not merely asserted.** D2 addendum row 2
(`Unsupported*`: valid input, lane not built) is right for the intended
target — a keyway. It was *wrong* for what the exact guard actually
fired on, because the lane **is** built for a wobbling frustum. Banding
is what makes row 2 correct rather than aspirational.

**IMPORT RESIDUAL — CLOSED BY REFUSAL, not pending.** The settling
experiment ran (hand-authored STEP solids, notched and un-notched
cylindrical faces, through `step-import` → `tessellate` → `check_mesh`).
A U-bounded cylindrical face never reaches this lane: `import_step`'s
tier-3 at-rest volume gate refuses it — `PropsError::NotIsoRectangle`
from `du_of_rims` (`geom-brep/src/props/curved.rs`), surfaced as
`ValidationError::VolumeUncomputable` and then
`StepImportError::TierInvalid`. **So the defect was never live.**

That conclusion holds for a *notched* face and only for one. It is not
why the arm stays quiet in general: a face whose walk lands
microscopically off its own box passes the tier-3 gate freely — the
split-boundary D-prism above imports, adopts and reaches this lane
without complaint — so the gate never screened that population at all.
The exact comparison, not the gate, is what fired on them.

The finding is what that reveals, and it *strengthens* the guard rather
than retiring it: the mesher was protected **transitively, by another
module's inability**. `du_of_rims` exists because props' volume closed
form needs the face's iso-parameter rectangle (`cylinder()` computes
`area = radius · du · (hi − lo)`) — the SAME premise `curved` depends
on, checked by a DIFFERENT subsystem for its own reason.
`mesh::tessellate` is public, takes any `Body<f64>`, and asserted
nothing. If the volume quadrature ever grows a general trimmed-face
path — exactly the kind of capability a later milestone adds — the
`NotIsoRectangle` refusal disappears and `tessellate_curved` starts
silently emitting wrong meshes **with no code change in `mesh` at
all**. The guard converts *"protected by another module's limitation"*
into *"checked where the assumption is made."*

Severity if the gate ever moves is not a corner case: a keyway is the
standard torque-transmitting feature on a shaft, and milled flats,
D-shafts, snap-ring groove walls and cross-drilled bosses are the same
class. The damage is honest and bounded — a wrong STL (keyway skinned
over, so a printed or CAM'd part has no keyway), a wrong render, and a
wrong mesh-derived `signed_volume`. The exact B-rep volume is
unaffected, because `mass_properties` is the very thing that refuses.

**Two items belonging to other units, recorded not fixed.**

- **The existing gate is MIS-CLASSED against the D2 addendum.**
  `topo/src/validate.rs:537` documents `VolumeUncomputable` as *"At rest
  every M2-constructible body computes; this is corruption surfaced
  loudly, not an exemption"* — row-1 framing, now falsified by an
  executed counterexample. An imported keyed shaft is not corruption; it
  is valid input the kernel has not built yet (row 2). The user-visible
  consequence today is that the kernel refuses a keyway-bearing shaft
  with an error that reads *"your file is corrupt."*
  `PropsError::NotIsoRectangle`'s own doc is already row-2 language
  (*"outside the M2 iso-rectangle inventory"*); it is topo's wrapper
  that mis-frames it.
- **An `unsure` lead in props, derived from code and not executed.**
  `du_of_rims` compares per-group SPAN SUMS, so a plus/cross-shaped iso
  domain whose arm width is exactly half the u-extent could make all
  four rim groups total the same `du` — passing `props_du_consistent`
  while being flagrantly non-rectangular, and `area = radius · du ·
  (hi − lo)` would then be wrong. That would be a silently wrong
  CERTIFIED volume, not merely a wrong mesh. Dispatched separately.
  #648's guard catches its mesh half regardless.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18). The warning postdates the lane it appears to
indict.**

`planar.rs` and `curved.rs` were born together in PR #39 (M2 PR 6), already
divergent; `trimmed.rs` arrived two milestones later. The warning in
`planar::triangulate_chart`'s header (named by symbol, not by line — the three
citations of it in this document had drifted to three different line numbers)
was written **five days after `curved.rs`**, by PR #116, as a *local
precondition of that PR's new even-odd flood fill* (*"would invalidate the
crossing bookkeeping built below"*). So it was never a claim about `curved`,
which has no crossing bookkeeping — and still inserts its grid after
constraining, now behind an explicit domain check rather than an unstated
premise.

The third author read `planar` and reused it (`trimmed.rs:104` imports
`classify_faces`/`edge_key`/`shoelace2`); neither #116 nor #157 touches or
mentions `curved.rs`.

**FLAGGED IN A NEIGHBOURING PR — the hazard class, never the lane.** The exact
failure mode was raised twice by reviewers and closed inside the lane at hand
each time: #116's root-cause writeup and pre-scan, and #157's fix **F2
(MIN-1)**, which added `TessellateError::SelfTouchingTrimLoop`. `curved`'s
ordering was never re-examined against either.

*Lesson:* an invariant discovered by a bugfix has to be swept across every
existing implementation of the same pipeline **in that same PR**, or it
protects only the code that already knew.

## S29. Sizing and certification vocabulary has fragmented across five modules

- **Where**: `crates/mesh/src/nurbs_cert.rs:356`,
  `crates/mesh/src/curved.rs:226`, `crates/mesh/src/chords.rs:63`,
  `crates/mesh/src/trimmed.rs:112`, `crates/mesh/src/budget.rs:555`
- **Confidence**: sure

"How fine should this be" is answered by `sagitta_angle`, `ellipse_step`,
`torus_grid_step`, `ceil_count`, a per-chart `grid_steps`,
`NurbsFaceBound::grid_steps` + `NurbsCellGrid::band_schedule` +
`row_bound`, `uniform_candidates`/`per_cell_candidates`, and
`divisions`/`best_split_steps` — the last a 321-sample scan over 16
decades of aspect ratio.

The constants are magic and mostly self-admitted: `SAFE_ASPECT = 5.0`
whose own doc says the derived line is √15 ≈ 3.87 and the gap is held by
"measured margin"; a bare `1.25` sphere fudge; `MAX_GRID_RETRIES = 6`
("was 4"); `RATIONAL_CERT_SPLITS = 16`; `DEV_SAMPLES = 6` vs the probe's
`12`; `SPLIT_SCAN_DECADES = 8` / `SPLIT_SCAN_STEPS = 321`; a π/4 cap;
δ_s = δ/2. Each is justified by a paragraph, but nothing says what the
sizing *policy* is — and `trimmed`'s refinement backstop exists precisely
because the schedule cannot be trusted to hit its own target.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18).** The vocabulary accreted one certified-face class
at a time, and the newest constants came from **measured falsification rather
than derivation** — PR #594 records six schedule variants implemented and
killed by the tour before the landed one, with `SAFE_ASPECT` chosen as the
value that survived. `MAX_GRID_RETRIES`'s *"was 4"* comment is literally the
deviation ledger inlined into the source.

**FLAGGED AND DEFERRED — by the author, deliberately, into an open design
conversation.** #594's deviations name both headline constants: *"`SAFE_ASPECT
= 5` is a MEASURED constant, above the derived certifies-under-delta line
(sqrt(15) ~ 3.87); the (3.87, 5] gap is held by measured margin"*. The
**policy** question was routed out rather than answered — *"the aspect-policy
question stays open"*, with PR #568 and `docs/TESS-SPLIT-SPEC.md` as the
designated venue, still unexecuted.

*Lesson:* reporting each magic constant as its own honestly-argued deviation is
a **substitute** for stating the policy, not a step toward it — N well-defended
deviations read as N decisions when they are one undecided question.

**The vocabulary grew again (2026-08-19, PR #684).** `curved::grid_steps` now
carries a sixth rule: `pole_columns`, a floor on the u step count when the
boundary walk carries a chart singularity (issue #678 — `nu == 2` meshes a pole
fan silently non-manifold). Two things to record honestly. It is a
**correctness** floor, not another tuning constant, and the routed-out design
conversation does **not** cover it: #568 and `docs/TESS-SPLIT-SPEC.md` are
scoped entirely to the NURBS **per-cell** schedule (`nurbs_cert`'s
`grid_steps`, certified cells, the first fundamental form), and nothing open
covers `curved::grid_steps`. But it lands squarely on this smell's *class*
complaint anyway: with no stated sizing policy, rule six arrives as one more
locally-argued decision in a module that already has five, and the reader still
has to read all six to know what the schedule promises.

## S30. `budget` and `probe_stats` are ~1,050 lines of instrument in the kernel's hot loop

- **Where**: `crates/mesh/src/budget.rs:1`,
  `crates/mesh/src/probe_stats.rs:1`, `crates/mesh/src/trimmed.rs:406`
- **Confidence**: sure

Two cargo features, two `live`/`inert` module pairs, two thread-local
accumulators, a CSV schema with a column-counting helper, and a
numerical optimizer (`best_split_steps`) live in the kernel's mesh
crate — ~1,050 of ~7,900 lines. The emit loop carries eight
unconditional telemetry calls, one of them with a `?` on it, plus a
`sample` flag and a resampling block containing an `assert!`. The live
half of `note_nurbs_sizing` re-runs `nurbs_cell_bounds`,
`nurbs_cell_grid` **and** `nurbs_face_bound` — a third full pass over
the certified assembly — purely to fill counterfactual columns.
`probe_stats`' module docs are a three-section history of a removed
environment variable, and its only consumer is one test.

Related: certified NURBS bounds are recomputed 2–4× per face per
tessellation, because the chord pass memoizes `nurbs_face_bound` locally
and the trimmed lane computes it again with no memo shared between
passes (`trimmed.rs:192`, `chords.rs:581`).

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18). The meter complies with the rule its own incident
produced; the volume question was never asked.**

`budget.rs` was authored (#547) in the same 24 hours as the `NURBS_PROBE`
incident and **satisfies every clause of the rule that came out of it** — the
live/inert split, zero `#[cfg]` in the hot files, `arm`/`take` in the live half
only, its own CI row. `memories/telemetry-gating.md` cites `mesh::budget` as
**the worked example**. The third full pass is spec-driven too: TESS-SPAN's
**D-4 "the meter stays sighted"** required the whole-patch counterfactual.

**FLAGGED AND PARTLY FIXED — on the gating axis only.** #547 itself caught the
sibling (`probe_stats::armed()` reading an env var once per triangle, 261k
times on the leaf) and hoisted it. Evan then reviewed the meter directly and
cleared it: *"does this affect production behavior? (no: properly gated) and
does anything need fixing? (nothing egregious, three notes)"*. The
volume-and-placement question — 1,050 of ~7,900 mesh lines, eight call sites in
the emit loop, a numerical optimizer in the kernel crate — was **never raised**
in #547, #560, #579, the spec, or the memory.

*Lesson:* a gating rule answers "does it run in shipped builds?" and is easy to
certify green; it says nothing about how much instrument the kernel should
**contain**, so the compliance check quietly became the whole review.

## S31. `geom-curves` / `geom-surfaces`: a crate split that buys nothing and is paid for in duplication

- **Where**: `crates/geom-surfaces/src/lib.rs:12`,
  `crates/geom-curves/src/projection.rs:67`,
  `crates/geom-surfaces/src/projection.rs:115`
- **Confidence**: sure

Identical manifests, identical module lists, and the only stated reason
is a doc sentence ("This crate deliberately does not depend on
`geom-curves`") citing an M2-era file layout, not a dependency need. The
four projection constants are declared twice with identical values under
a comment asserting "the two halves of §6.1 share one policy";
`removal_pass_bound`, `poison_point`, `ring_coords`, `placeholder` and
the weight/count validation are each written twice; the Newton loops and
seeding sweeps are line-for-line analogues.

The halves have already drifted in ways that look accidental rather than
intended: `is_placeholder` exists only on `NurbsSurface` (used in ~15
places downstream) while `NurbsCurve3` has no equivalent — which is why
`step-export/src/writer.rs:329` open-codes the placeholder
representation with `control().iter().all(|p| !p.x.is_finite())` in one
arm of a `match` whose other arm calls the named predicate. And
`SurfaceProjection` was lifted to a generic scalar at M6-2 while
`Projection3` is still `f64`-only.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S32. `Surface`'s one-partial-per-call API created a second surface enum

- **Where**: `crates/geom-surfaces/src/lib.rs:403`, `:460`,
  `crates/geom-surfaces/src/nurbs.rs:794`,
  `crates/geom-brep/src/ssi/system.rs:225`
- **Confidence**: sure

`NurbsSurface` computes point and all `k+l ≤ 2` partials in one pass
(`ders`), but the `Surface` enum exposes six separate accessors, each of
whose NURBS arm calls `n.ders(u, v)` and throws away five of six
results. `Surface::normal` costs two full jets. `SurfaceJet`/
`SurfaceJet3` are publicly exported yet the enum never offers one — so
`geom-brep`'s SSI built a shadow surface enum (`Chart { Plane, Nurbs }`)
with its own `eval`/`jet3`, re-implementing plane evaluation and
hand-filling a `SurfaceJet3` of zeros. The natural query being
unavailable at the enum is what created the second enum.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S33. Neither geometry enum can lift itself to another scalar

- **Where**: `crates/geom-curves/src/lib.rs:870`, `:990`,
  `crates/geom-surfaces/src/lib.rs:679`, `:1113`,
  `crates/sweep/src/skin.rs:770`
- **Confidence**: sure

`DESIGN.md` makes "evaluate the same function with a different scalar
type" the reason the geometry layer is generic over `T`, but `Curve3<T>`
and `Surface<T>` have no `map_scalar`/`lift`. Every place needing
`Curve3<f64> → Curve3<Dual64>` or `→ Curve3<Interval>` writes its own
per-variant ladder: twice inside `geom-curves/src/lib.rs` alone (the
dual and interval versions differing only in the scalar conversion),
twice again in `geom-surfaces`, and roughly ten more across `topo`,
`mesh` and test modules, plus one production copy in `sweep`. Each must
be kept exhaustive by hand as variants are added, and each silently maps
`Nurbs(_)` to the placeholder rather than lifting the payload.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S34. FIXED by #697 — `readback.rs` was a body-wide accessor module housed in `sweep`

- **Where**: `crates/topo/src/readback.rs` (its new home)
- **Confidence**: sure

**FIXED by #697**, on the rule *a door lives in the crate whose types it
reads*. Body-wide reads went to `topo::readback`; `blend_arcs` became
`ValidatedLoop::blend_arcs()` in `profile` — a method rather than a second
readback module, since a second module would have had to **restate** the
three read-back rules, which is this finding's own smell class; the
op-result door stayed in `sweep::revolve`. `sweep::readback` no longer
exists and no reference to it survives anywhere. The dependency claim was
verified narrowly and honestly: `editor-core/src/names/interrogate.rs` now
names **no** `sweep` item, and `editor-core`/`pncad` still depend on `sweep`
only for *evaluating* sweep ops and for façade re-export — neither
removable by moving an accessor.

**Two of the three op doors had zero callers, and their doctests were their
only evidence** — S11's shape, and the doctest was the door's own
advertisement. Deleted, with every assertion preserved as a six-test
integration suite written the way a caller writes it now, which is
simultaneously the proof the deletion is lossless (review mutation-tested
that suite to red). `revolved_caps` survived on a real distinction:
`Extruded` exposes `top`/`bottom` as flat public fields, so its door was
three field reads, while `Revolved`'s caps live inside
`RevolvedKind::Partial`, so its door carries case analysis and a "a full
revolve has no caps" refusal a field read cannot give.

**The finding under-counted: `vertex_point` had FIVE copies, not two** —
and the obstacle to sharing was self-inflicted. `Dangling` named the
failing lookup as a `&'static str`, which no caller can map back to a key,
so each site kept its own walk to keep its own error fidelity. A
`DanglingRef` payload (`Entity(EntityId)` | `Geometry(GeomRef)`) dissolves
that: one body, the two discriminating callers keeping their exact arms
through one `From`, the two corrupt-verdict callers collapsing both
deliberately. *The general shape: a refusal that describes its cause in
prose instead of naming it forces every caller to re-derive the thing it
refused about.*

**Left open, and named rather than lost.** Review ran the
`face_pose`-shaped sweep this PR declined to run and found the class alive
one crate over — see **S57** below. And the "five became one" claim is a
**measurement with no guard**, in the one crate that holds the exact
machine for guarding it (`topo::face_normal`'s anti-re-fork row, whose own
docs record that the fix pass which built it had re-forked the walk in the
same commit). Per Q6 that owes a guard, a scheduled re-measure, or a
written reason it can have neither; it has none of the three, and it is now
the fourth crate-root module in this fix class shipping without the guard
its sibling has (**#695**).

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S35. Remaining Tier-2 items (roll-up)

- **Confidence**: as noted

| Finding | Anchor | Confidence |
|---|---|---|
| `meta/` hand-rolls ~480 lines of `Serializer`/`Deserializer` re-implementing `serde_json::to_value` — in a crate that already depends on `serde_json` for the whole file format. The doc justifies a concrete *value type* (sound); the argument does not reach the hand-written codecs | `meta/ser.rs:40`, `meta/de.rs:24` | likely |
| The edit door and the persistence validator spell the same six invariants twice, in two parallel fault enums. Two checks *are* shared, which shows sharing was possible | `persist/check.rs:320`, `edit.rs:1068` | likely |
| The placement registry is keyed by a value (`gauge`) that is not stored, so every single-row lookup recomputes the whole instance–mate partition; `reconcile` runs a full `solve_document` on every edit that could move the mate graph. Two similarly-named `placement()` accessors return different things | `mate/solve.rs:287`, `doc.rs:210` | likely |
| The coset fold's tolerance lever is order-dependent (`arm = arm.max(..)` inside the loop), while `coset.rs`'s opening argument leans on intersection being order-independent | `mate/solve.rs:489`, `coset.rs:20` | likely |
| The root list stores what the module's own reasoning proves is derivable (the DAG's sink set); the only non-derivable content is the *order*, and around it sit a persisted `Vec`, a 4-arm fault enum, a checker at two doors, two maintenance hooks and a schema bump | `roots.rs:18` | likely |
| `vdiff` builds its type family twice — `Summary*` "owned-name twins" of five types plus two loops — where the shared arithmetic core is already generic over the name type | `resolve/vdiff.rs:375` | sure |
| `apply_with_names` is a second edit door in `resolve/`, publicly exported, called only from tests, with *narrower* validation than `apply`'s own name door | `resolve/mod.rs:911` | likely |
| `resolve` and `appearance` each implement N3 merge offers differently — one scans every `Merged` segment, the other fires only when `Merged` is last | `resolve/mod.rs:824`, `appearance.rs:410` | sure |
| One name-table re-keying algorithm hand-copied five times; three copies carry their own bespoke `e.body != 0` guard. Comments justify tie semantics by appeal to `graft_names`, **a function name that appears nowhere in the repo** | `names/emit.rs:153`, `:216`, `:293`, `product.rs:497`, `emit_topo.rs:178` | sure |
| Multiplicity of same-named siblings is resolved three unrelated ways (rank fragment / N2 tie / positional counter minted into the name), chosen per-site with no stated rule; two put integers in names that shift when a sibling appears | `names/role.rs:198`, `emit_topo.rs:390` | likely |
| A geometric reference that lands in a stable name is picked by arena-walk order via `.or_insert`; the comment defending it describes BTreeMap iteration order, which is not what `or_insert` keys on | `emit_topo.rs:798` | likely |
| `derive_naming` brute-force-searches for a permutation `validate` already computed and discarded, and writes the recovered index algebra twice (search closures vs `LoopAnchor` methods) without the search calling the methods | `eval/anchor.rs:147` | likely |
| `resolve_selection` / `resolve_declarations` are a documented hand-synced duplicate — "**If you change either ladder, change both**" — where the justification is longer than the shared code it declines to factor | `eval/wire.rs:707`, `:1029` | sure |
| The profile resolve→replay→validate ladder exists three times with two error vocabularies, so the same broken profile reports differently at the edit door and at evaluation | `eval/wire.rs:407`, `:1419`, `program.rs:846` | sure |
| `eval`'s finiteness door computes `value * T::zero()` and asks a tolerance band whether the product is `Sign::Zero`, with two magic constants and an `else` the comment calls unreachable — standing in for an `is_finite`/poison predicate `Real` does not expose, and the sole reason `eval` demands `Decide` over `Real` | `expr.rs:854` | likely |
| `Real::is_poison` exists to support a NaN sentinel: "no description yet" is a bilinear patch whose control points are all NaN, recognised by testing `p.x.is_poison()` — in a crate that elsewhere works hard to make illegal states unrepresentable | `real.rs:143`, `geom-surfaces/nurbs.rs:235` | likely |
| `bit_identity.rs` dispatches via `&dyn Any` + `downcast_ref` — the channel `real.rs` names as **banned** — for a mechanism whose production allowlist its own docs record as now empty | `bit_identity.rs:56`, `real.rs:22` | sure |
| Three "the one greppable decide funnel" wrappers, all pure forwards; `geom-brep` has two of them, so the invariant the pattern exists to hold is already broken, and callers bypass both | `dihedral.rs:98`, `enters.rs:163`, `validate.rs:261`, `props/quad.rs:409` | sure |
| `Revolved` is one result type for three topologies, with the mode encoded in whether `Vec<Vec<Option<_>>>` fields are entirely `None`; `None` means several different things per case, all in prose | `revolve/mod.rs:203` | sure |
| Tier 2 re-derives a vertex-incidence map tier 1 already computed and passed via `Tier1Report` for a *different* aggregate — defended by a 9-line comment that concedes it produces a spurious `ScaffoldingStrutVertex` | `validate.rs:1501`, `:2896` | sure |
| Three near-identical tier-3 entry points plus a forwarder that allocates a `SecondaryMap` to throw it away; the "3′ ≡ tier 3 with empty contacts" equivalence is maintained by two separately-written call sequences | `validate.rs:1664`, `:1705`, `:1688` | sure |
| `ContactMark::Unmarked` collapses four distinguishable causes; the code writes two identical branches with `#[allow(clippy::if_same_then_else)]` to keep a distinction only the comment can see | `validate.rs:1985` | sure |
| `transform_rigid` refuses every **described** NURBS surface under an error named `NurbsPlaceholder`, though tier 3 stopped conflating those two states at M6-3 and the same file *was* updated for the description match. Blocks placement of any lofted/swept/imported body with a real NURBS wall | `transform.rs:248`, `:110` | sure |
| Four typed spellings of one 2×2 product in `mekr`, whose precondition blocks re-derive the same seven checks in different orders — so the same misuse yields `SameLoop` in two sites and `LoopNotEmpty` in the others, and the doc declares its check-order listing "`Cycles`-canonical" | `euler_ring.rs:229`, `:1055` | likely |
| The face-sense inheritance rule is implemented three times, one deliberately in a different spelling advertised as "the terser spelling of `mef`'s same truth table" — while the helper both `mef` arms funnel through documents the rule a fourth time | `euler.rs:1600`, `:1703`, `euler_kill.rs:997` | sure |
| The seamed-result output stage is copy-pasted at four exits and has drifted: only two run `volume_backstop`, only two pass a non-empty seam worklist, and one uses `merge_coplanar_faces()` where the others use `_declared` | `boolean/ops.rs:458`, `:1870`, `:1936`, `rest.rs:279` | sure |
| Two functions each documented as "**the one** door" for the same face-pair verdict, where the newer strictly subsumes the older — and the older never retired, so editor-core's flush detector and the boolean's verify-at-use take different paths | `boolean/rest.rs:484`, `:558` | sure |
| Three implementations of "find the nearest mutually-facing unused germ pair"; the module doc claims the REST lane reuses the join's tests — the *predicates* are shared, the *search* is not, and `find_match` has since grown a facing test the others lack | `boolean/join.rs:524`, `:774`, `rest.rs:336` | sure |
| Two shell-classification walkers with the same body and different owners | `boolean/ops.rs:1749`, `finish.rs:117` | sure |
| The "which surface kinds are wired" question is answered by **eight** independent match ladders and five distinct "no arm here" error variants, with no single table anywhere | `boolean/reduce.rs:169`, `ops.rs:390`, `join.rs:337`, +5 | sure |
| The coincidence ladder is implemented twice — a margin-list driver for sphere/cylinder, hand-written rungs for planes re-deriving the same matrix in longhand — and the two already differ observably. The evidence struct is still called `PlaneIdentity` | `plane_eq.rs:154`, `carrier_eq.rs:299` | likely |
| `split`'s mirror-retry lane runs the pipeline up to three times and reports an error from a run it discarded; ~40 lines of doc concede the lane is incomplete and the error attribution wrong on asymmetric failures | `splitting/mod.rs:565` | sure |
| `PlaneSide` is used as a two-valued side wherever `On` is structurally impossible, enforced by `_ =>` wildcards — so a wrong verdict upstream silently becomes Below in four places | `splitting/finish.rs:257` | likely |
| Nested `Result<Option<Result<Vec<T>, Indeterminate>>, ()>` as a four-state control-flow vocabulary, where `Err(())` means "not applicable" and shares the error channel with "failed" | `splitting/classify.rs:147` | sure |
| `adopt_edges` decides a five-rung ladder through mutable flags and a post-hoc `retain` that deletes candidates it just added; ~2/3 of the region is prose | `step-import/adopt.rs:356`, `:536` | sure |
| `normalize` is not the pipeline stage the crate documents (it runs inside `Resolver::solid`), and normalization is split across two modules with two census implementations | `step-import/entities.rs:1901`, `normalize.rs:293` | sure |
| Every step-import identifier is a bare `u64`; `face_id` is a `Vec` index at one line and an entity id at two others in the same file. `mint_id()` allocates into the file's own id space, so refusals on minted topology print `entity #<id>` for entities the file does not contain | `normalize.rs:913`, `entities.rs:1610` | likely |
| `step-import`'s `chart.rs` is a second chart-geometry vocabulary built to a *weaker* standard: `infer_outer` decides regions from a chord-polygon read that `topo::chart_region`'s docs explicitly forbid, then decides ring-in-ring containment from a single probe point per ring | `step-import/chart.rs:115`, `:347` | likely |
| Recognition and detection rest on hand-tuned constants unrelated to ε_in: 16 samples/edge with a `1e-6` rad wrap gate, chart winding read at carrier fractions `0.25`/`0.30` with a `0.1` validity threshold, `TURN_MARGIN = π/6`, fixed u-fraction sample triples. Each carries a paragraph arguing it is not a tolerance | `entities.rs:2021`, `recognize_curve.rs:463` | likely |
| `StepImportError` wraps four kernel error types in `source:` fields and implements `Error` with an **empty body** — no `source()` — while `step-export`, its inverse, does implement it. Display is inconsistent (`{source}` vs `{source:?}`), and `UnsupportedUnit`'s message describes a version of `units.rs` that no longer exists | `step-import/error.rs:433`, `:419` | sure |
| The marcher's control flow mixes named trileans with the raw comparisons its header says it does not have: `back > 3.0 * h_meters`, `d.sqrt() <= 1.0e-12 * h_meters.max(1.0)`, alongside seven hand-tuned `SSI_*` constants | `ssi/march.rs:474`, `:498` | sure |
| Three parallel spellings of one rung-3 certificate, with two hand-written refusal translations, and two `check_residual` calls that re-band limbs `ssi::certify_branch` already banded — dead gates that nonetheless add two more predicate names, for six names over two limbs | `certify.rs:1451`, `edge_nurbs.rs:70`, `ssi/certify.rs:177` | sure |
| Four parallel check-sequence engines in `pcurve_cache` (~850 lines) whose "same fixed order" contract is prose — and step 2 already diverges, two lanes calling `param_rate_gate` and two calling bare `param_rate` | `pcurve_cache.rs:1937`, `:2842`, `:2503`, `:2353` | sure |
| "Derive a pcurve" has two homes split by chart kind, contradicting the stated `geom-brep`/`topo` layering; every caller carries an `if matches!(surface, Surface::Nurbs(_))` fork | `pcurve_cache.rs:3147`, `topo/pcurves.rs:419` | sure |
| Chart lever-arm knowledge is duplicated across the two crates with divergent conventions (`azimuth_lever` gives the sphere `r`; `azimuth_arm` gives `\|r·cos v\|`) and nothing in either name says which is which | `pcurve_cache.rs:1753`, `topo/pcurves.rs:721` | sure |
| Two clearance predicates for one question, where the *exact* one's refuse arm is unreachable through the public API and it is `pub` only so tests can reach it — the weaker, self-admittedly over-refusing screen is authoritative | `fillet/surgery.rs:616`, `:580` | sure |
| `plane_sphere_blend` writes a fail-loud poison and a silent `.max(T::zero())` clamp on sibling quantities three lines apart | `fillet/blend.rs:205` | likely |
| `NullFacePair`'s two variants carry identical payloads under two vocabularies, while the sibling `NullEdge` handles the same duality with one field pair and a documented dual reading | `topo/null.rs:88` | likely |
| Two parallel "where did this come from" side-tables (`*_provenance` ×7, `*_sources` ×3) whose docs claim one pattern but whose access idioms are opposite; three extra accessors re-spell `provenance(EntityId::…)`, one with zero callers | `topo/body.rs:660`, `source.rs:6` | likely |
| `PriorCtx` is a three-method trait with a null-object implementor standing in for an `Option` | `resolve/mod.rs:422` | likely |
| `lift.rs` is 655 lines of migration machinery for a migration `PROFILES-V2-DESIGN` explicitly ruled out ("no migration machinery, no shims"), with its own 6-variant refusal vocabulary — a dev tool promoted to shipped API via the `pncad` façade | `profile/lift.rs:1` | likely |
| The `profile` typestate markers are PhantomData over one untyped `Tip` with two `Option` fields, so `HasPos` does not make `tip.pos` a `Point2` — hence two error variants documented as "expected unreachable" threaded through ~24 sites, and `.to(anchor)` distinguishing lattice states at runtime via `self.core.pending.is_none()` | `profile/path.rs:1091`, `:761` | sure |
| Two different types named `ArcData` in one crate — a resolved carrier circle and the authored-spec enum — both reachable as `super::ArcData`, with the enum exported at the crate root | `profile/path.rs:982`, `path/program.rs:96` | sure |
| The profile elaborator advertises "strictly forward, single pass" and back-patches emitted geometry in three places, so "every authored point lies on the final path" holds in the weaker sense of "lies on some segment" | `profile/path.rs:44`, `:1604`, `:1684`, `:2650` | likely |
| The `nurbs_curve!`/`nurbs_fit!`/`nurbs_project!` macros mint a full 2-D twin whose heavy half (speed meter, removal bounds, `split_at`, `elevate_degree`, a whole `Projection2`) is shipped, monomorphized and unexercised | `geom-curves/nurbs.rs:87` | likely |
| `FitError::ParamCountMismatch` is returned for four unrelated failures — in a module that added `RaggedRows` specifically to avoid exactly that reuse | `geom-curves/fit.rs:471` | sure |
| `frame::path_start_frame` is justified by a deduplication it never performed (no kernel caller; `sweep` still builds its own frames) and duplicates `Vec3::orthonormal_basis`'s role with a different policy | `linalg/frame.rs:322`, `vec.rs:307` | likely |
| `ch_scale_left`/`ch_scale_right` are the same function kept apart "to preserve the rehearsal's association" — but `RingInterval::mul` is bit-for-bit commutative, so production shape is anchored to a test file for nothing | `spline/compose.rs:519` | likely |
| `CurvePlan::apply_points` defends against malformed plans its own three private constructors rule out, and pushes the cost onto callers as an invented poison value plus four near-identical lerp closures | `spline/algebra.rs:161` | likely |
| `lsq::solve_normal` forms `AᵀA` (squaring the condition number) on the fitting path while the sibling `svd.rs` already contains Householder QR; the adversarial review test validates it by implementing QR a *third* time | `linalg/lsq.rs:158`, `svd.rs:183` | unsure |
| ~~`certify_rung3`'s `arm` and `extent` are the same value at three of four call sites, so the `#[allow(too_many_arguments)] // one parameter per named quantity` covers a parameter varied once~~ **FIXED by #692** — `TubeScale<T>` names the two cases (`uniform(arm)` at the three sites where they are one quantity, `split(arm, extent)` at `finish_r3`); `certify_rung3` goes 8 args to 6 and the `allow` is deleted **on merit**. Found by review as a residue of #692's own diff: the PR removed one redundant parameter and left its twin in the same argument list — the exact class it existed to close, one line below the one it closed | ~~`ssi.rs:925`, `:729`~~ | sure |
| Two unrelated `compose` modules with two unrelated `ComposeError`s; defended on the grounds that "the two never meet in one scope", which is a claim about today's imports | `geom-curves/compose.rs:18`, `geom-core/spline/compose.rs:57` | sure |
| `names/flush.rs` puts document-editing sugar and seven kernel contact-type re-exports inside the naming subsystem; `declare` → `declare_all` → `declare_node` is three public doors over one operation, each with a doc block longer than its body | `names/flush.rs:440`, `:113` | sure |
| `mesh`'s `trimmed` retry loop serves two opposite failure modes (one shrinks the candidate set, one grows it) under one budget with two exit conditions, index-coupled mutable state, and an unreachable trailing `Err` — so the stated termination argument no longer covers the loop | `trimmed.rs:264`, `:349` | likely |
| `crates/bvh`: the tree has two live call sites and earns its place, but four crates depend on it **only** for `Aabb`, a plain box type unrelated to hierarchies — the load-bearing export and the crate name disagree, and two of the three duties in its own header are "not yet wired" | `bvh/src/lib.rs:1` | likely |
| The `mekr`/`fillet`/`boolean` files carry seven `#[allow(clippy::too_many_arguments)]` in one file and five near-verbatim copies of one justification comment in another — the shared context these passes need was never given a type | `emit_topo.rs`, `revolve/partial.rs:369` | sure |

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
---

# Tier 3 — real but lower stakes

## S36. Milestone- and PR-named code, and review probes shipped in `src/`

- **Where**: `crates/topo/src/lib.rs:156`,
  `crates/topo/src/review_m1_pr4.rs:6`,
  `crates/step-import/src/cr_r1_probes.rs:1`
- **Confidence**: sure
- **Found independently by six scans.**

Roughly **230 of 345 test files** across the workspace are named after a
PR, milestone or review round rather than a subject (`review_m3_pr55`,
`review_m3_pr3_bob`, `review_s6_probe`, `probe_f34_review`). In `topo`
it is 41 of 53 files — 15,649 of 18,774 lines — plus ~8,200 lines of
`review_*` modules inside `src/` itself.

Three things make this more than a naming preference:

1. **The stated justification for living in `src/` is mostly false.**
   Each module's header claims it needs crate-internal access an
   integration test cannot reach. `review_m1_pr4.rs` (1,576 lines)
   touches a `pub(crate)` arena exactly once; four of `review_m1_pr2`'s
   submodules touch none at all despite `mod.rs` asserting they
   "deliberately raw-build or raw-corrupt states no operator can reach".
   `mod.rs` then concedes the real reason: "the suite moved whole to
   keep its shared helpers in one place."
2. **The convention they cite says the opposite.** The headers point at
   `docs/M1-LOG.md` (since archived — one of 16 stale `docs/M*-LOG`
   references under `crates/`), whose ratified rule reads "*Reviewer
   test suites are promoted into CI as
   `crates/topo/tests/review_m1_prN.rs`*" — `tests/`, not `src/`.
3. **They carry a maintenance exemption enforced by comment**: "do not
   'simplify' them to match shipped fixtures — the independence is the
   regression value." Which is how S18's four deep-snapshot helpers came
   to exist. `step-import`'s probes go further and say "review branch
   only… P1/P2 FAIL as of 35aa0c0" while sitting on main.

`topo` is the outlier, not the house style: `step-import` kept its own
probes as subject-named *inline* modules (`mod review_fuzz`,
`mod review_outerness`).

**Verdict:** ACCEPTED, BUT SEQUENCED — DO NOT RENAME FIRST (Evan,
2026-08-18). *"The milestone naming would be good to clean up eventually but
right now it's implicitly flagging stuff that probably has messy test suites
that need to be streamlined, so they should only be renamed after an actual
review and fixup to become normal test suites."*

This inverts the obvious action. A PR-numbered name is currently **carrying
signal**: it marks a suite that has not been combed into a subject-organised
one. Renaming ahead of that review would destroy the only marker of which
suites still need the work, and would convert a visible backlog into an
invisible one. The unit of work is **review-and-fixup, then rename** — per
suite, not a repo-wide rename pass.
## S37. Milestone naming leaks into shipped artifacts and the public API

- **Where**: `crates/mesh/src/types.rs:164`, `crates/stl/src/ascii.rs:8`,
  `crates/pncad/src/prelude.rs:56`
- **Confidence**: sure

**FIXED by #639 (H4).** The scope held was *everything that leaves this
repository*: bytes written into export files, strings that reach a caller at
runtime, the Python package, and the rustdoc of the façade crate that is the
library.

**The shipped bytes.** The ASCII solid name is `cad-kernel` and the binary
header `binary STL; CAD kernel tessellation export` — the milestone token
only. **Q9 is untouched**; the placeholder is not a name proposal. **No new
public API**: the finding's other half, that the STL header is not
caller-settable while the STEP writer takes `product_name`, `author`,
`organization` and `originating_system` as options, is a **residue for
Evan**, because closing it is an API design call. The STEP side was checked
and has no leak of its own — its defaults are `product_name: "part"` and
empty fields, caller-supplied at every call site — so the asymmetry is
purely that STL has no options struct. No byte-comparison golden moved,
because there are none: the STL oracles compare exports to each other across
ε rows and repeat runs, so they are header-blind. That blindness was itself
the finding — the ASCII `solid` name had **zero test coverage anywhere**
(`review_m2_pr7.rs:62` only checks `starts_with("solid ")`), so the PR adds
a row pinning the exact opener and its matching `endsolid`.

**The estimate was an order of magnitude low.** ~124 public-rustdoc spec
codes were predicted; **~1189** is the measured count, obtained by walking
each crate's `pub mod` tree from `lib.rs` and counting doc lines on publicly
reachable items, with the reviewer's independent parse agreeing at 1188. The
**~1115-line remainder sits entirely in `publish = false` kernel crates**
— `topo` 300, `editor-core` 267, `geom-brep` 192, `geom-core` 107, the rest
below 70 — and is **scheduled as §D H16/H17** together with the
caller-settable-header residue. It is not a leak in S37's sense while nothing
in the workspace publishes; it becomes one the day anything does. `pncad`,
`pncad-py` and `stl` are at zero.

**The class was wider than rustdoc.** The sweep ran over every string literal
outside comments in every crate's `src`, through a tokenizer that follows
`\`-continued multi-line literals: **63 runtime-visible hits across seven
crates**, all `Display`/`Debug` text a library user reads. `UnsupportedCurve`'s
own doc and the value in `trimmed.rs` that carried `M6-3`/`M5 PR 11`/`M7`
into the runtime string are among them.

**Two process facts this fix established**, both recorded in full in §C and
not restated here: the sweep's pattern was blind in exactly the shape it was
hunting, which left S37's own named example `LIB-DOORS F5` shipping in a live
Python `__doc__` (**§C15**, third instance); and ten broken string assertions
shipped under a green `cargo test` claim in the first version of the PR body
(**§C17**).

**Verdict:** ACCEPTED, AND SEPARABLE — CAN BE FIXED EARLIER (Evan,
2026-08-18). *"The shipped artifact comments can be fixed earlier."*

The S36 boundary held: milestone naming **inside** test files is a backlog
marker kept until the suite is combed, and this lane left it alone —
including `topo/src/contact.rs`'s `"the #256 ruling"` and
`topo/src/review_m0_pr7.rs`. Non-public internal comments stay **W3b**; the
plain-`//` residue there measures **473** workspace-wide.

## S38. Comments that argue rather than describe

- **Confidence**: sure
- **Found independently by every scan.**

The pattern, with the extreme cases:

| Site | Ratio |
|---|---|
| `crates/pncad/src/closure.rs` | 151 lines, **zero code** |
| `crates/mesh/src/lib.rs` | 219 comment lines around ~22 lines of module declarations |
| `crates/pncad/src/select.rs` | 449 comment lines in front of one `pub use` |
| `crates/profile/src/fillet_select.rs` | 238 lines hosting 15 lines of code; one function documented as existing "to state that identity in code rather than in prose" |
| `geom-curves`'s `speed_lower_bound` | ~170 doc lines over ~90 code lines, mostly litigating history |
| `mesh/src/walk.rs:653` | ~90 lines of prose — a census of 315 closures, a traced STEP coordinate, two rejected alternatives — on a five-line `if` |
| `crates/mesh` overall | ~40% comment lines |
| `boolean/ops.rs:584` `volume_backstop` | ~100 doc lines over a 60-line function, including an arm annotated "unreachable now… kept as the honest statement of the gate rather than a dead arm removed" |
| `props/quad.rs:418` | ~45 lines of justification on a 20-line function, arguing bit-identity forbids a metering the same text calls arguable |
| `fillet/surgery.rs:689` | A ten-line proof of a perpendicularity claim spanning three modules, attached to `let _ = dir;` |

The prose is high quality. What makes it a finding is that duplicated
*arguments* drift exactly the way duplicated code does, and several
already have — `planar.rs:277`'s 33-line sense-sign argument is repeated
near-verbatim at `curved.rs:90`, and S15's `Retired` comment was a
justification simply false for one of the two doors it covered (#688 retired
that door; the disposition is recorded at S15).

**Verdict:** ACCEPTED (Evan, 2026-08-18). *"The comments should definitely
be trimmed down to what's actually necessary."*
## S39. Stale claims that other code is instructed to rely on

- **Confidence**: sure

**FIXED by #635** (eleven rows: the ten tabulated here plus `pcurves.rs:91`,
added by the S15 steelman). All eleven were still live — none had been closed
incidentally by #617–#627 — and each was **classified before its sentence was
touched**, which is what Evan's verdict below asks for. Ten were benign rot: in
nearly every case the *authoritative* statement (the variant doc, the method
rustdoc, `DESIGN.md`) was already correct and current, and only a summary or
module-level restatement had rotted, so nothing was erased by correcting it.

**One was a lost invariant, and it indicts the instrument.** `props/quad.rs:42`'s
*"the patch flux engine consumes this machinery at rest"* was written on
2026-08-05 **by a previous stale-claims sweep** — `git log -S` puts it in
`e2222617`, whose message names its own "§7 stale-claims sweep tranche" — and it
**replaced two honest sentences** (the parent said "no at-rest construction mints
a stored B-spline pcurve yet … its consumers today are tests") with a false one
naming the wrong engine. It also missed a third: the `weights != 1` refusal
*inside the same function* still contradicted it, ten lines away, for two weeks.
Repointed at the real blocker rather than deleted. Method note for the next
sweep: this checkout is shallow, so `git blame` misattributes by ten days —
`git log -S` is the instrument.

Three process facts worth carrying, all from this lane:

- A line-scoped `rg` cannot see a claim that **wraps across a line break**, which
  is how two survivors escaped the first pass; #635's sweep now joins consecutive
  comment lines into one logical string before matching. #632 failed the same way
  on the same day (its pattern could not see match arms wrapped in
  `Some(…)`/`Ok(…)`), and #639's could not see **bare** clause letters because it
  scanned for prefixed codes.
- Where the truth is "promised and never delivered", a flat present-tense sentence
  is **worse than the stale claim**: it erases the only marker that something was
  owed. #635's first pass did that to three schedules; **#638** now carries
  curved-face containment and is cited from all four sites.
- The `revolve` `MultipleAxisRuns` row nearly became a design escalation before the
  ratified answer was found 150 lines below it in the same file
  (`FullRevolveHoles`' Display: *"sweeps produce genus, never voids; voids are born
  only from booleans"*). Q1 — grep the file you are in.

Unclassified siblings went to **H15**; the `enters.rs` question was **D5**, answered by #665 (the newtype).

**Two rows were added after that fix**, both from #647's style review. The first
needed Evan, because `DESIGN.md` is the ratified contract — he authorised the edit
and it is closed. **The second is still open**: it needs a per-row read rather than
a script, for the reason its own entry gives.

| Claim | Reality | Anchor |
|---|---|---|
| `DESIGN.md`'s crate table, `topo` row: "the boolean engine and its splitting/census machinery (`topo::boolean`)" | **FIXED — see the PR that closed this row.** Evan authorised the `DESIGN.md` edit (the doc is the ratified contract, which is why #647 recorded rather than edited it). The parenthetical was false and getting falser — `splitting` and `boolean` are both `pub mod` at the crate root, `census` and `sector_shape` are `pub(crate) mod` siblings, so none is a member of `boolean`. The prose was a defensible reading of the ratified architecture and is kept; the path is **replaced by the structural claim** ("sibling modules at the crate root rather than underneath `boolean`") rather than by a corrected path list, because a pinned path is the rot mechanism this row is an instance of. | `DESIGN.md:1362` |
| `docs/predicate-dimension-audit.md`'s per-row LINE ANCHORS, in a doc whose own header says "a row and its disposition entry must never disagree" | Verified stale: `validate.rs:1795` points at iso-adjacency prose while `tangent_second_order` is decided at `:2005`; `pcurve_cache.rs:1664` points at an arc construction while `pcurve_chart_radial_moving` is decided at `:3219`. Three more anchors are off by >200 lines and unverified. The audit's convention is that a single-line anchor names the *comparand construction*, a few lines above its `decide`, so a small offset is correct and only a large one is rot — which is why this needs a per-row read, not a script rewrite. #647 fixed the three defects it introduced in its own retarget (a dropped `bool_sector_within`, a one-line-short range, a range-less new row) and declined the sweep; the scan script it used is in its PR body | `predicate-dimension-audit.md` (75 anchored rows) |

**Verdict:** ACCEPTED, WITH A SHARPENED READING (Evan, 2026-08-18). *"The
stale claims should also be fixed carefully rather than just removed since they
may flag cases where the code deviated from something that should've been
projected as invariant."*

This reframes the finding. A stale claim is **two-valued evidence**: either the
doc rotted while the code was right, or **the code drifted away from something
that was meant to hold**. The second case is a latent defect wearing a
documentation costume, and deleting the sentence would erase the only surviving
record of the intended invariant.

Two rows in this finding look like candidates for the second reading rather
than the first:

- **`enters.rs:14`** ✅ **ANSWERED by #665 (D5, Evan's call).** It derived M3's
  entire sign chain from *"every face's stored normal is the outward normal"*,
  a claim `step-export/src/volume.rs:36` says has been false since M5 S10; the
  code worked only because every caller remembered the `sense_sign` multiply.
  The reading was **the second one** — an invariant that was meant to hold, not
  a rotted sentence — and it is now held by `geom_brep::OutwardNormal<T>`,
  whose only constructor takes the sense sign. The derivation opening states
  that; the sentence *"the sense correction is the caller's, and no type
  enforces it"* is gone, being false as of that PR. What #665 could NOT type
  is recorded in its description; the same contract still lives untyped at two
  further doors, scheduled as **D6**.
- **`pcurves.rs:91`** (found by the S15 steelman, not the original scan) listed
  `merge_coplanar_faces` among ops that neither clear nor re-mint the pcurve
  cache. It **started re-minting on 2026-08-05**. Here the code moved in the
  *safe* direction and the index rotted behind it — the benign reading. **This
  PR's own sibling sweep had already corrected it**: `merge_coplanar_faces`
  sits under "Maintains the map" on today's main, which D5 verified rather
  than assumed before deciding it owed no hand-off.

Each row needs the same question asked before its sentence is touched: **which
of the two happened?**
## S40. Residue and editing artifacts

- **Confidence**: sure

- **FIXED by #627.** `run_properties` returns a tally — selections,
  executions, skips, and the selections that could legally skip — and the
  check that carries the weight is PER STEP, not a total: both documented
  irreversible-by-one-op subcases live in `roundtrip`'s `Kev`/`Kef` arms,
  so a selection on any other choice must execute, and that is asserted
  as each step happens. The run-level lines are labelled for what each
  one is — `executed > 0` is the only one that can fail on its own, and
  it is a collapse floor the design does not imply; the other two are
  bookkeeping identities, one from how the tally accumulates and one
  already guaranteed by the per-step assertion. No threshold is asserted
  because there is no number to assert: proptest seeds from entropy, and
  four consecutive runs gave 339/335/4/47, 331/325/6/43, 333/328/5/51 and
  351/345/6/47 for selected/executed/skipped/skippable. The proptest test
  became a plain `#[test]` around `proptest!`'s closure form so the
  totals have somewhere to be recorded; the deterministic issue-#60 case
  asserts its own execution. RED evidence: forcing `roundtrip` to skip
  everything, and separately forcing it to skip one non-`Kev`/`Kef`
  choice (`MevLone`) — the second is the degradation a bare `> 0` floor
  would have passed.
- **FIXED by #627.** `powi_edges`' `|| true` row now asserts what the
  case is for: `x.powi(i32::MIN)` is not poison and is a finite
  underflowed bracket of zero. The overflowed positive power is the
  honest `[MAX, +inf]` (not poison), so its reciprocal has a positive
  subnormal upper bound rather than collapsing — which the old row could
  not have told apart from poison.
- **FIXED by #627.** `every_error_displays` gets a local `variant_index`
  that matches `EulerOpError` with no wildcard, so a new variant fails to
  compile until an arm exists, and a coverage array then names the
  variant whose Display sample is missing. All 27 variants are sampled.
  One residual is stated in the test rather than papered over: the
  variant COUNT is still hand-written, so a new variant given an arm but
  no sample passes — closing that needs the count from the compiler
  (`strum`'s `EnumCount` or the workspace's first proc-macro crate), and
  neither is bought by a hygiene lane.
- **FIXED by #627** (swept in the fix pass, same defect one file away):
  `validate.rs`'s `errors_display_without_panicking` carried the same
  claim — a hand-written `vec![…]` under a comment saying the exhaustive
  Display match "already forces" the list — over 31 of `ValidationError`'s
  59 variants. Same treatment, same stated residual; all 59 are sampled.
- **FIXED by #627.** The seam-vertex parentage match now scrutinises
  the contact-record partners alongside the edge slices
  (`(a_edges, b_edges, partner_a_inner.as_ref(), partner_b_inner.as_ref())`),
  so the two arms that were `is_some()`-guarded and then
  `unwrap_or_else`-defaulted bind their partner by pattern instead. The
  fallbacks are gone with the guards, and every arm draws its A side
  from an A-descended name and its B side from a B-descended one — so
  `Seam{x, x}` has no arm left to come from. `_seam_set` is off the
  signature and the call (the caller still builds `seam_set` for the
  edge and fragment passes, which use it); the second `a_faces`/`b_faces`
  sort+dedup is deleted, the first one already covered them.
- **FIXED by #627.** Confirmed and removed. `select.rs`'s live
  crossings go Python → kernel (`to_kernel`), so its tripwire's
  kernel → Python match is the only exhaustiveness check over the kernel
  enums there; `flush.rs`'s live `plane_relation`/`flush_rung` already
  cross kernel → Python with no wildcard, so the copy checked nothing the
  callable helpers did not. The helpers' own doc lines now state that
  invariant.
- `profile::k_stats` is a self-declared compatibility shim ("new code
  should reach for `geom_core::k_stats` directly") that all eight of the
  crate's own decision-making modules use in preference to the thing it
  forwards to — including modules written long after the unification it
  describes (`profile/k_stats.rs:1`). **STILL OPEN** — retiring the shim
  is structural, not residue.
- **FIXED by #627.** The refusal message's line continuations are
  restored; the runtime text is one clean sentence again (the acceptance
  probe matches on `NON-PARALLEL`, which is unchanged).
- **FIXED by #627.** All three stranded doc comments are reattached to
  the item they describe: `loose_partners`' doc (head and orphaned tail
  rejoined) off `germ_section_frame` and `type LooseMap`,
  `feed_stable_name`'s off `naming_key`, and `run_iso_checks`' off
  `run_iso_arc_checks`.
- **FIXED by #627.** `KeyView` — a one-word enum whose by-value use is
  exactly what forced the recompute — derives `Copy`, so `finish_fallback`
  binds the pair once and no call site changes. The same shadow-and-
  recompute shape was swept out of `pcurve_cache.rs`'s
  `run_harmonic_checks`, which computed `let reach = t0.abs().max(t1.abs())`
  twice in one body.
- `WitnessSlot {}` is an empty struct occupying a field on every
  `NodeValue`, paired with a `NodeErrorKind::WitnessBifurcation`
  documented as never constructed (`eval/mod.rs:230`). **STILL OPEN** —
  deleting either changes the eval value type.
- **FIXED by #627.** `save`'s `drop(replay)` is gone. In `member_of`
  the early `matches!(g, Subgroup::Empty)` return is folded INTO the
  match as its own arm — the arm cannot simply be dropped from
  `Subgroup::Se3 | Subgroup::Empty` without making the match
  non-exhaustive, and moving the structural answer into it keeps one
  place per variant.
- `same_level`'s structurally-impossible arm manufactures its error by
  feeding `f64::NAN` into `classify` and letting the funnel escalate —
  a decision predicate used as a `throw`; `unreachable_zero` returns a
  4-tuple of NaNs into live flux arithmetic (`props/curved.rs:350`,
  `:1090`). **STILL OPEN** — D2 (bug-vs-invalid-state) territory, gated
  on Wave 0.
- `Rim` stores the same traversal direction twice (`d_u: T` and
  `d_u_sign: Sign`), and the exact one is compared through the tolerance
  funnel — subtracting two exactly-±1 values and banding a result that
  is always 0 or ±2 (`props/curved.rs:330`, `:384`). **STILL OPEN** —
  which of the two representations is authoritative is a design call.
- **FIXED by #627.** Removed, with its `eprintln!` block. Nothing in
  CI, `scripts/`, `local-scripts/` or any manifest enabled it; the only
  other mention was a history note in `review_s1_probes.rs`, reworded so
  it no longer names a feature that is gone.
- `crate docs` devote a paragraph to defending the single `HashSet` that
  violates D9's determinism rule, concluding "a `SecondaryMap` would be
  both cheaper and consistent with the rule" — for a set used at three
  sites (`topo/src/lib.rs:60`). **STILL OPEN** — a D9 determinism design
  call, and the comment itself is W3b's pass.

**Verdict:** ACCEPTED (Evan, 2026-08-18). *"The residue stuff should also be
fixed."* Scoped by Evan on 2026-08-19 to the two behavioural rows plus the
unambiguously mechanical residue; the rows marked STILL OPEN above are design
calls or belong to later waves and were deliberately left.
## S41. The `Enclosure` seam launders `Interval` decorations

- **Where**: `crates/geom-core/src/real.rs` (`CertifiedEnclosure`),
  `crates/geom-core/src/spline/hull.rs:98`,
  `crates/geom-brep/src/ssi/enclose.rs` (`ring`, `pad_interval`)
- **Importance**: high
- **Raised by**: the S1 steelman pass, 2026-08-18. Not part of the
  original scan.

**FIXED by #643 and #671.** The lead was filed *unsure*; it is confirmed by
execution. `sqrt([−1, 4])` clamps to `[0, 2]` with decoration `Trv` —
nonempty, finite, tight — and before the fix it crossed into the C9 ring
as a healthy, certifiable bound at **three** entry points, not the two
the finding named: `hull::domain_hull` returned `[0, 2]`,
`implicit_enclosure` returned `[−0.21, 3.005]`, and **`Box3::around`**,
which the finding did not list, returned `[−1, 3]`. All three reach
`RingInterval::from_bounds` through a `T: Bounds` bracket read.

**What the finding could not have anticipated is that whether this is a
defect at all depends on D1.** The first attempt made `impl Bounds for
Interval` refuse below `Decoration::Def`. That is sound only under the
"may enter certified code" reading, and the repo already encodes the
other one: `geom-curves/tests/review_m5_pr3_attack_interval.rs` and
`nurbs_interval.rs` carry three containment rows that assert a component
enclosure contains the pointwise `f64` value, and one of those enclosures
is `DInterval { lo: -inf, hi: inf, dec: Trv }`. It *does* contain it —
interval arithmetic brackets the values the expression was defined on
even when the decoration is `Trv`. Making `Bounds` refuse turned
`cargo test -p geom-curves --features interval` from `116 passed` into
`113 passed, 3 failed`. A `Trv` bracket is simultaneously **a sound
bracket** and **inadmissible in certified code**; those are two
questions, and one accessor cannot answer both.

So the repair is the split, not a refusal bolted onto `Bounds`:

- `Bounds`/`Enclosure` keep meaning *"carries a bracket"* — endpoints as
  stored, decoration never consulted. Unchanged from before the PR.
- `CertifiedEnclosure` means *"may enter certified code"*, one method,
  `certified_bracket(self) -> Option<(f64, f64)>`, refusing below
  `Decoration::Def` — the same threshold `sign_within` uses, now spelled
  once as `Interval::is_certified` and called by both doors. `f64` and
  `RingInterval` always certify (neither has a domain-violation channel).
  `k_stats::Probe` certifies too, for the `f64` reason — it is an `f64`
  with a recorder, and refusing there would make a `--features probe`
  build diverge from the `f64` build. It carries **no supertrait**: a body
  needing both doors writes `T: Bounds + CertifiedEnclosure`, which is an
  honest inventory, and a subtrait would put a third `lo`/`hi` in scope
  wherever a compound bound is written.
- The three crossings require `CertifiedEnclosure`, so a `Trv` enclosure
  cannot reach `RingInterval::from_bounds` through them at all.

The three `geom-curves` containment rows pass **untouched** — that was the
success criterion, and it is what distinguishes the split from the
refusal.

**`Dual` is deliberately not implemented** in the certified lane, as it is
not in `Bounds`. The whole workspace compiles under `--all-targets
--features interval` without it, which re-confirms S44's pricing finding
from the other direction: nothing in `src` puts a dual through certified
code. That keeps D1's `Dual` half open and separable rather than answered
in passing by a Wave-1 lane.

**What it raised.**

1. **The class was wider than three. All eight crossings are now
   converted, and this raised item is CLOSED** (items 2 and 3 below stand
   on their own). #643 fixed three crossings and deferred five
   as *unknown, not benign* — found by grep, with no `Trv` pushed through
   any. **#671 pushed one through all five.** Every one of them
   laundered, measured at the crossing itself:

   | site | what crosses | `Trv` fixture in → out |
   |---|---|---|
   | `geom-brep/src/ssi/certify.rs:648-650` | the plane normal's three components | `[1, 3]`ᵀʳᵛ → transversality margin **0.999999999999997** |
   | `geom-curves/src/nurbs.rs:1169` | `NurbsCurve3` control-hull coefficients | `[0, 2]`ᵀʳᵛ → `RingInterval { lo: 0.0, hi: 2.0 }` |
   | `geom-curves/src/nurbs.rs:1186` | `NurbsCurve2` control-hull coefficients | `[0, 2]`ᵀʳᵛ → `RingInterval { lo: 0.0, hi: 2.0 }` |
   | `geom-surfaces/src/nurbs.rs:912` | `NurbsSurface` control-net coefficients | `[0, 2]`ᵀʳᵛ → `RingInterval { lo: 0.0, hi: 2.0 }` |
   | `topo/src/props.rs:468` (`br`) | the harmonic channels' scalars | `[1, 3]`ᵀʳᵛ → `RingInterval { lo: 1.0, hi: 3.0 }` |

   The `certify.rs` row is the one worth reading twice: the fixture is
   `sqrt([−1, 4]) + 1 = [1, 3]` at `Trv` — finite AND zero-free — so the
   laundered answer is not a suspicious number but a **positive margin**,
   i.e. a uniqueness certificate for a plane equation that was clamped
   out of its own domain. Each row was written red against the shipped
   code before the repair.

   **The deferral's stated reason was wrong.** They were sized as needing
   "each caller's `T` widened, rippling through three more crates". The
   ripple is **zero**: `cargo build --workspace --all-targets --features
   interval` compiles unchanged after all five take
   `RingInterval::from_certified` (new in `geom-core`, the shared
   spelling of the four-line helper #643 wrote three times). Nothing
   outside the five files needed a bound. The reason is that each
   crossing's lane is already statically split at a concrete scalar —
   `PropsQuadLane`'s explicit impls in `topo`, and `CertifiedEnclosure`
   already on `probe_tube_chart`'s own bound in `certify.rs`.

   **Reachability from `src` today: no exercised path delivers one.**
   Measured, not read: all five crossings were temporarily instrumented
   to panic on a non-certifying operand and the whole interval battery
   run (10 crates, ~2,900 rows). Nothing fired. Two guards are pinned as
   rows rather than left as readings — `newell_plane`, the kernel's one
   derived-plane mint at a generic scalar, refuses a `Trv` vertex with
   `Escalated` because the decoration poisons its residual decision
   (`geom-brep/tests/decoration_plane_mint.rs`); and the quadrature
   lane's own arithmetic cannot manufacture a violation, since a norm is
   the square root of a sum of squares and certifies even where it is
   zero, so a `Trv` at `br` must have been *stored* in the body
   (`topo/src/props.rs`, `bracket_seam_tests`).

   That is a guard, not a type, and it does not extend to the public
   API: `NurbsCurve3::<Interval>::new` admits a domain-violated control
   coordinate without complaint — pinned — so `ring_coords` was reachable
   with a `Trv` by any caller of a `pub` method on a `pub` type. The
   crossings now refuse instead.
2. **A pre-existing hazard, independent of this fix**: `f64::max` and
   `f64::min` return the **non**-NaN operand, so the idiom
   `from_bounds(x.lo().max(-1.0), x.hi().min(1.0))` turns a poisoned
   enclosure into the clamping window — a sound-looking `[−1, 1]` with no
   argument behind it. This needs no decorations and no `interval`
   feature: NaI, the empty enclosure and ring poison are all NaN in
   storage, so it is reachable in a plain `f64` build. #643 added
   `RingInterval::clamped_to` (poison-first) and moved five sites onto it;
   four more in `props/quad.rs` are provably poison-free and now say so
   per site.
3. **Three disciplines for that one hazard, none of them linted**:
   `clamped_to`, a hand-rolled `if x.is_poison() { return x }`
   (`props/quad.rs`'s `sqrt_enclosure`), and audited raw `max` with a
   comment (`bvh/tree.rs:209-210`). Nothing prevents a fourth. Still
   open, and **still unlinted** — but the *certified-door* half of the
   same shape now has exactly one body: #671 both added
   `RingInterval::from_certified` and collapsed onto it the two
   byte-identical copies #643 had left behind (`geom-core`'s
   `spline::hull::bracket` and `geom-brep`'s `ssi::enclose::ring`), which
   are now one-line wrappers. The one deliberate non-copy is
   `enclose.rs`'s `pad_interval`, which reads only the bracket's upper
   end and is a different operation. The clamping half still has three
   spellings and no gate; nothing prevents a fourth there, and nothing
   mechanical would notice a fourth certified door either — the gate that
   might have is **S56**.

## S42. Loft's `sense = true` derivation is untested against the shape that broke extrude

- **Where**: `crates/sweep/src/loft.rs:30`,
  `docs/archive/M5-LOG.md:1975`, `docs/archive/M5-S11-SPEC.md:23`
- **Importance**: medium
- **Confidence**: unsure — a lead, not a defect
- **Raised by**: the S6 steelman pass, 2026-08-18.

**VERIFIED by #619 — no defect found.** Loft's derivation holds on both
shapes. A concave-arc section pair and a holed one now form the third
verb's chapter of the S11 constructor audit
(`crates/sweep/tests/m5_s11_concave_sense.rs`), reusing that file's own
fixtures: each row evaluates the shipped wall's `S_u × S_v`, folds in
the stored `sense`, and requires the *same solid built by extrude* to
read material on the inward side and void on the outward one — a probe
a flipped bit inverts, executed as its own row. The prescribed union
check could not be run and was substituted for a direct test of the
property it is a symptom of: a lofted operand is refused typed at
boolean admission, so the swallow is closed **by refusal, not by the
bit**. The rows also pin the residual that makes them load-bearing:
**no tier, prop or boolean in the kernel reads a lofted wall's sense**
— check 6's curved arm skips `Surface::Nurbs` — so an inside-out lofted
wall is tier-3 green. The class sweep turned up no second orientation
instance; fillet's bare `sense: true` on corner patches is guarded by
three typed concave-chain refusals, i.e. an unstated precondition
rather than an S11 defect.

**Verdict:** CLOSED — NO DEFECT (2026-08-19). Verified by #619 on the two
shapes S42 named, and the coverage residual its own reviewer raised (**S51**:
every #619 row lofts two sections at `v_degree = 1`, so no chart can twist) is
itself closed by #636. What this finding leaves behind is not a defect but a
residual worth outliving it: **no tier, prop or boolean in the kernel reads a
lofted wall's sense** — check 6's curved arm skips `Surface::Nurbs` — so an
inside-out lofted wall is tier-3 green, and the rows in
`m5_s11_concave_sense.rs` are what stands between that and a silent wrong
answer.

## S43. The kernel has five different answers to "this state can only be a bug"

- **Where**: `crates/topo/src/euler.rs:1940` (and 57 siblings),
  `crates/geom-curves/src/nurbs.rs:162`,
  `crates/mesh/src/walk.rs:395`, `crates/geom-core/src/spline/hull.rs:80`,
  `docs/DESIGN.md:1100`, `Cargo.toml` (the `indexing_slicing` deferral)
- **Importance**: high
- **Confidence**: sure
- **Raised by**: the S12/S14 steelman pass, 2026-08-18, which argues
  *"this, not S12 or S14, is the finding."*

| # | Idiom | Instances | Prose justification |
|---|---|---|---|
| 1 | Typed error, plan phase | `EulerOpError::{FanOrbitBroken, LoopCycleBroken, OrbitBroken, UnclaimedHalfEdge, …}` | "tier-1-invalid input" |
| 2 | Typed error, pure dispatch defect | `TessellateError::MissingEntity { what: "… (router defect)" }` | "reaching one is a dispatch defect, **surfaced typed**" |
| 3 | `debug_assert`, compiled out | `assert_euler_postcondition` (arena deltas + full tier-1 validate) | D9's ratified exemption |
| 4 | Silent `if let Some` discard | **58 sites** across the three Euler modules | D9's "documented garbage-out in release" |
| 5 | Bare indexing that panics, **chosen deliberately** | `nurbs.rs:162`, `hull.rs` `coeffs[j]`, `mesh/chords.rs:465` | "the fail-loud direction" (PR #447) |

Idioms 4 and 5 are **opposite answers to one question**, each argued in
its own module's prose by appeal to the same principle. `geom-curves`
writes that silently dropping data is the wrong direction and panicking
is the right one; `crates/topo` does the silently-dropping thing 58
times and has it blessed by D9's footnote. Both believe they implement
"fail loud" (`memories/MEMORY.md`). Neither is wrong locally. **D9's
text supports idioms 1, 3 and 4, but not idiom 2's confidence and not
idiom 5 at all** — and idiom 5 was argued into existence in a merged PR
body that was never taken back to D9.

The restatement the steelman proposes is a taxonomy with one row per
state class: reachable by input → typed error (unchanged); reachable
only by kernel bug, cheaply detectable → typed error, not `debug_assert`
and not panic; reachable only by kernel bug, not cheaply detectable →
`debug_assert` + documented garbage-out; **reachable by API misuse →
typed error or poison, never a bare index**. That last row is the one
the codebase has no rule for, and it is where `Span` lives. Settling it
decides S12's residue and S14 at one stroke.

**Verdict:** ACCEPTED AND SETTLED — ratified 2026-08-19 as the **D2 addendum
to D9** (`docs/DESIGN.md:1118`, PR #628; Wave 0 decision D2). Five idioms
become five state classes with one mechanism each: reachable-by-input invalid
→ typed error; reachable-by-input valid-but-unbuilt → typed `Unsupported*`;
value-domain degeneracy → poison; kernel bug observable in a branch →
`unreachable!`; kernel bug detectable only by re-derivation → `debug_assert`.
**Silent discard is never an answer**, so idiom 4 is superseded outright, and
rows 4/5 split on **re-derivation, not cost**. D9's headline bullet is
untouched: `unreachable!` is by construction not input-reachable.

*What is settled is the rule, not the code.* The addendum opens the
`unreachable` lint in both manifests and licenses the conversion; it does not
perform it. Until that lands, the ~60 `if let Some` discards across
`euler.rs`/`euler_ring.rs`/`euler_kill.rs`, idiom 2's `MissingEntity` router
defects, and `AssemblyUnsupported`'s rename are all still the superseded
idiom — which is **W2c**, scheduled as Track D's **D16** (#706; it had a
verdict here and no row anywhere for two days, which is §D's fourth ordering
rule failing on the section that states it). S12's residue and S14's
disposition follow from this rule and should be re-read against it rather than
re-argued.

## S44. The founding ruling for the lane-trait pattern exists only as an agent's paraphrase

- **Where**: `docs/archive/M5-LOG.md:3451`, `crates/geom-core/src/real.rs:348`,
  and the four lane traits of **S3**
- **Importance**: high
- **Confidence**: sure
- **Raised by**: Evan, 2026-08-18, on reading S3's steelman.

The entire lane-trait pattern — four traits, 16 impls, a supertrait bundle, and
six `Bounds` ledger amendments — rests on one sentence in a milestone log:

> **certification_bracket RESTRUCTURED (Evan pushback, 2026-08-02): the static
> split replaces the runtime Option.** Evan: *it is not semantically valid
> type-wise for duals to enter a pipeline that can only refuse them* —
> adopted.

**There is no primary record.** I searched `docs/`, `memories/`, `crates/`, and
PR #157's comment thread: the sentence appears **only** in `M5-LOG.md:3451`, as
an agent's one-line paraphrase of an in-session chat. PR #157's comments are
entirely about a rebuild-latency mystery. No issue, no design-conversation PR,
no `DESIGN.md` entry.

Worse, **the same log page records that this channel was known to be
unreliable**, forty lines earlier:

> **CHANNEL CORRECTION recorded: consultations posted to merged-PR #148 threads
> do NOT reliably reach Evan** (he saw it only via the in-session mention);
> future forks go out as a NEW issue or design-conversation PR per the standing
> memory.

**Evan, on being shown the paraphrase (2026-08-18):** *"ha that the original
source was my comment on things being semantically valid type-wise; it sounds
like the agent must've construed that pretty much backwards / i was confused by
what exactly they were proposing."*

### Why "backwards" is the plausible reading

The remembered sentence most naturally argues that **a dual is not a bracket,
so `Bounds` must not be implemented for it** — a statement about what the trait
*means*. The repo's own text agrees, at `M5-LOG.md:2871`: *"`Bounds` is
implemented for `f64`, the interval scalar and the telemetry probe — never for
`Dual`, **which has no bracket to offer**."*

If that is the content, the correct consequence is a plain `T: Decide + Bounds`
bound on certification code — **which is exactly what the fillet battery seam
does**, and the same log ratifies it: *"A `PropsQuadLane`-style static split
would therefore have had an EMPTY refusing side… So the seam is RATIFIED
rather than split."*

The lane traits do the **opposite**: they let a `Dual` body call
`validate_geometric` and receive a typed refusal from the quadrature arm. That
is, on its face, *duals entering a pipeline that can only refuse them* — the
thing the remembered sentence says is not semantically valid.

Both readings survive the sentence in isolation. What does not survive is the
provenance: a 16-impl pattern, a CI gate with nineteen allowlisted files, and
six ratification amendments all cite a chat nobody can reproduce, and the
person quoted does not recognise the conclusion drawn from it.

### The bundling question underneath it

Evan, same message: *"it does seem like there must be multiple semantic things
bundled together into `Real` if there are things that exist just to refuse
`Dual`."*

`Bounds` is `fn lo(self) -> f64` / `fn hi(self) -> f64`. It is doing two jobs:

1. **A semantic property** — "this scalar carries a bracket." Genuinely true of
   `Interval`, degenerate-but-definable for `f64`/`Probe`, and **definable for
   `Dual` too** (lo = hi = the value channel, discarding the tangent — the
   value channel is already contractually bit-identical to the f64 run).
2. **An access-control marker** — "this scalar may enter certified code." This
   is the job the *absence* of the impl performs, and it is the only reason not
   to write the definable impl in (1).

For every other scalar those two coincide. For `Dual` they coincide **only by
choice**, and the lane traits exist to paper over that choice at the four sites
where a `Dual` body must still be able to make the call. So the pattern is not
"four traits encoding one bit" (S3's framing) so much as **one overloaded trait
whose second meaning has to be re-implemented per-site wherever the first
meaning would let the wrong scalar through**.

**Verdict:** OPEN for the part that matters — see below. The question
supersedes S3's disposition, and S3 should not be acted on until it is
answered, since the answer determines whether the collapse target is "one lane
trait in `geom-core`" (S3's steelman, compiled and working) or "no lane traits
at all, and a `Bounds` split into its two meanings."

> **AMENDED 2026-08-19 — this verdict line is superseded, and the original is
> kept above rather than rewritten because the reasoning below it is the
> record of how the question was reached.** The `Bounds`-meaning half is
> **ANSWERED**: #643 split the trait, and Evan's **D1** ruling then separated
> the two meanings for `Dual` — see the *D1 DECIDED* block below. What is
> still OPEN is the lane-trait half (S3 / W2a), which the ruling explicitly
> does not settle. A reader who stops at this line should carry away
> *"answered on `Bounds`, open on the lanes"*, not *"open"*. (D3/D4 carry no
> verdict line and so set no precedent for how to amend one; a struck-through
> line would lose the reasoning, and a silent rewrite would lose the fact that
> the question was once genuinely open, so it is amended in place.)

**The `Interval` half is settled by code (#643, W1c/S41), recorded here as
fact rather than as a ruling.** The bundling Evan named — *"there must be
multiple semantic things bundled together"* — turned out to be demonstrable
rather than arguable, and the demonstration is in the existing suite:
`geom-curves`' three containment rows assert that a `Trv`-decorated, unbounded
enclosure contains its pointwise `f64` value, which is TRUE under meaning (1)
and inadmissible under meaning (2). An attempt to make `Bounds` serve meaning
(2) broke exactly those three rows. So for `Interval` the two meanings are now
two traits: `Bounds` keeps (1), and `CertifiedEnclosure` carries (2).

**This does not answer the `Dual` question, deliberately.** Meaning (1) is
still "definable for `Dual` too", and meaning (2) is still what the lane
traits mediate; #643 implements `CertifiedEnclosure` for `f64`, `Interval` and
`RingInterval` and **not** for `Dual`, and the workspace compiles that way
under `--all-targets --features interval`. That is a second, independent
confirmation of the pricing entry below: nothing in `src` routes a dual
through certified code. What remains open is precisely what this finding says
is open — whether a dual may certify, whether the four lane traits survive,
and whether the four D9 bit-identity assertions may be re-expressed.

### D1 PRICED (2026-08-18): the lane traits cost nothing in `src`

To price the split, the four refusing `Dual` impls were commented out
(`pcurve_cache.rs:1214`, `chart_region.rs:304`, `props.rs:422`,
`edge_nurbs.rs:561`) and the workspace rechecked. Reverted afterwards;
this records only the measurement.

**`cargo check --workspace` succeeds.** The entire production tree
compiles with no `Dual` lane at all. **Nothing in `src` instantiates any
of the four lanes at `Dual`** — the only `Dual` mentions in `props.rs` and
`chart_region.rs` are the refusing impls themselves.

Everything that breaks is a **test**, and there are five sites:

| Site | What it does |
|---|---|
| `topo/tests/geometric_cube.rs:236` | `validate_geometric` on a `Dual64` body |
| `topo/tests/review_m2_pr3.rs:224` | `validate_geometric` on a `Dual64` body |
| `sweep/tests/extrude_acceptance.rs:565` | `validate_geometric` on a `Dual64` body |
| `sweep/tests/m5_pr11_quad_props.rs:165` | `mass_properties` on a `Dual` body |
| `topo/tests/fixture/mod.rs:302` | `certify_at_dual` — *"the dual lane's refusal, executed"* |

They fall into two kinds. The last one is a test **of the refusing
machinery itself**: if the lane traits go, its subject goes with them and
it deletes rather than being rewritten. The other four assert D9's
bit-identity contract — *"the value channel of a `Dual` build takes the
identical certified path"* — and they express it by literally running the
certified validator at `Dual`.

**So the whole 16-impl pattern's only load-bearing job is making those
four bit-identity assertions compile.** That is worth stating plainly,
because it is also the sharpest form of S44's objection: the tests assert
that a dual **can** enter the certified pipeline, which is the thing the
remembered founding sentence says is not semantically valid. The pattern
and the ruling it cites are asserting opposite things, and the tests are
where they meet.

**Price of the split, then:** zero production edits; one test deleted;
four bit-identity assertions re-expressed without routing a `Dual` through
certified code (validate at `f64`, compare the dual's value channel
through an uncertified path). Whether that re-expression is acceptable is
the actual question D1 has to answer — it is a question about what the
bit-identity contract is allowed to say, not about trait ergonomics.

### D1 DECIDED (2026-08-19): a `Dual` may not certify, but it may have `Bounds`

**Evan, 2026-08-19:** *a `Dual` may not certify — at least for now — but it may
have `Bounds`. And M10 is absolutely still the plan.*

That answers the bundling this finding named by **separating it**, which is a
sentence that could not have been written before #643. Until then `Bounds` was
one accessor carrying both meanings, so granting a dual the bracket would have
granted it the certification right in the same stroke — which is exactly why
the founding ruling's paraphrase was arguable in both directions and why S3
could not move. The two meanings are now two traits, and the split was
**forced rather than chosen**: three `geom-curves` containment rows assert that
a `Trv`, unbounded enclosure still contains its pointwise value, true under
"carries a bracket" and inadmissible under "may enter certified code". The
ruling lands on a seam that evidence cut.

**Meaning (1) — "carries a bracket" — is granted.** `geom_core::Bounds` is
implemented for `Dual<T>` over a bracket-carrying base scalar: `lo` and `hi`
are the **value channel's** bracket, tangent discarded. Neither half of that is
a convenience. The value half is D9's dual contract — the value channel of a
`Dual<T>` build *is* the plain-`T` build, bit-identically, computed with `T`'s
own operations in the written association — so whatever brackets the plain
run brackets the dual run's value with nothing to re-establish per operation;
the impl therefore *delegates* to the base scalar rather than restating a
bracket (`lo = hi = value` at `Dual<f64>`, the value enclosure's endpoints at
`Dual<Interval>`). The tangent half is an **extension of** `ERROR-DESIGN`
**E9**, *tangent poison never refuses* — an extension, not a reading: E9 is
scoped to **leaf refusal** ("refusal is decided solely by value-channel
predicates and W-certificates") and says nothing about `Bounds::lo`/`hi`,
which did not exist for a dual when it was written. The extension is that
hulling the tangent into `[lo, hi]` would let a NaN or unbounded tangent —
which E4 expects at a kink and `copysign`'s straddle rule mints deliberately —
poison a bracket the value channel is entitled to, which is E9's failure mode
reached through a different accessor. `Decide for Dual` settled the identical
question the same way in PR #9/#10 (value-part delegation) and `is_poison` is
value-only, so a tangent-reading `Bounds` would be the one accessor of three
that disagrees.

**What the exclusion costs, recorded so E4 does not rediscover it.** E9 pairs
"never refuses" with a **forfeiture** half — a degraded tangent forfeits its
uses, `per_param`/`rss` entries report `UnavailableBecause` (E5). `Bounds`
carries **no signal** that the tangent is degraded: `lo()`/`hi()` are the value
channel's whether the tangent is `1.0` or NaN. That is intended — a bracket is
not the derivative channel's reporting surface — but it means E4's forfeiture
reporting must read the tangent through the public fields; it cannot be
recovered from a bracket after the fact.

**Meaning (2) — "may enter certified code" — is refused.**
`geom_core::CertifiedEnclosure` still has no `Dual` impl, and it is now the
whole guard rather than a deferral. *At least for now* is Evan's own hedge and
is load-bearing: the door is shut, not nailed shut, and reopening it is a
decision with a name rather than an impl someone adds in passing.

**M10 / E4 remains the plan** — stated because the cheapest reading of "a dual
may not certify" would have been "so stop paying for duals", and that is not
the ruling. S2's cost analysis stands as a cost, not as a case for removal.

### What was implemented under this ruling

- `impl Bounds for Dual` in `geom-core/src/dual.rs`, delegating to the base
  scalar, with the justification above written into the impl.
- The rows that pin what it **returns**, not that it exists: the bracket is
  the value channel bit-for-bit under seven hostile tangents (NaN, ±∞, ±0,
  ±1e308), it is a point at `Dual<f64>`, it is the value *enclosure* at
  `Dual<Interval>` and an unbounded tangent does not widen it, value poison
  surfaces as NaN from both accessors, and after a real `Real`-generic
  computation the dual run's bracket is the `f64` run's bit-identically. A
  tangent-reading definition fails the first, third and last of those.
- The refusal as a **compiler fact**: a `compile_fail` doctest on `Dual64`
  against `CertifiedEnclosure`, beside a passing one against `Bounds`. It goes
  red the day someone writes the certification impl, rather than the day
  something certifies wrongly.
- The stale halves of the discipline record, which asserted the *absence* as a
  fact — each **retargeted** onto the ruling (*may not certify*) rather than
  deleted, so the reason survives where the fact did not. The sweep covers
  `real.rs`'s `Bounds` scope rule (all three ledger entries: M5 PR 11, M5 PR
  12, M9-2 PR-1), `CertifiedEnclosure`'s implementor list,
  `bounds-allowlist.sh`'s header (both the fillet and the chart-region
  paragraphs), the four lane traits' own reason clauses and their consumers'
  comments (`census.rs`, `validate.rs`, `euler.rs`, `certify.rs`), two runtime
  `Display` strings (`edge_nurbs.rs`, `pcurve_cache.rs`) that were telling a
  *user* a dual has no bracket, the test docs, and **`docs/DESIGN.md`** — the
  ratified contract, which at `:531` said *"a dual carries no bracket"* and at
  `:1879` still called `Bounds` "the certification trait" (#643 rot D1 made
  worse).

  **The pattern**, stated so the next sweep can be compared to it: a
  line mentioning `dual` within a ±4-line window of any of
  `no bracket | not a bracket | carries no bracket | no <Bounds> impl |
  uninstantiable | bracket-free | nothing to split | EMPTY refusing |
  never for Dual | no dual lane | no dual path | no dual-scalar |
  not an enclosure | bracket-carrying`, over every tracked `.rs`/`.md`/`.sh`/
  `.py`/`.toml` file, excluding `docs/archive/` (historical record: those
  sentences were true when written and are quoted as such). **What it cannot
  match**: a claim that states the premise without any of those words — e.g.
  *"three of the four sealed scalars have this unconditionally"*, or a
  paraphrase like *"a dual is a value and a derivative"* — and a claim
  separated from the word `dual` by more than four lines. Two sites were found
  by reading rather than by the pattern (`edge_nurbs.rs`'s module header,
  `props.rs`'s `PropsQuadLane` doc), which is the honest measure of its reach.

### What newly admits `Dual`, and the one thing that owes something

Every door that builds a **C9 ring** is bounded by `CertifiedEnclosure` since
#643 — the props quadrature lane, the SSI rung-3 certificate and its limbs,
the fitted-pcurve lane, the edge-NURBS lane — and all of them stay
uninstantiable at a dual. That is the load-bearing verification, and it is
what makes this ruling safe to implement now when it would not have been
before #643.

**Correction (2026-08-19 adversarial review): the stronger form of that
sentence — *"everything that mints a certificate"* — is FALSE, and it was the
sentence certifying the negative.** `topo::separation` mints one:
`Separation::of`, `Separation::certify` and `image` are `T: Decide + Bounds`
with **no** `CertifiedEnclosure`, the module opens *"prove that no two placed
copies can touch"*, `certify`'s doc says *"`Ok(())` is the certificate"*, and
`graft_disjoint_all_keyed` re-checks nothing (#382). It is instantiable at
`Dual64` and `Dual<Interval>`, verified by compilation. The row for it in the
admits-table below also had its reasoning backwards — *"its boxes only ever
refuse"* — when box NON-overlap is precisely the grant.

**No wrong certificate exists, and the correct justification is delegation**:
every endpoint a dual's box carries is its value channel's, which is the
plain-`T` run's bit-identically (D9), so a dual run's separation certificate
is the base scalar's. Whether `separation` should nonetheless take
`CertifiedEnclosure` is a **#643-completeness** question, not D1's, and is
left open.

**A gap in the METHOD, not only in the table** (same review): the enumeration
was over `Bounds`-bounded signatures and so missed a trait.
`geom-core/src/real.rs`'s `impl<T: Bounds> Enclosure for T` means this PR
grants `Dual` the `Enclosure` trait as well, whose doc four lines above
advertised it as what "certification helpers … take". Not a live hole —
`spline::hull` moved to `CertifiedEnclosure` at #643 and no `Enclosure`-bounded
signature remains in `crates/*/src` — but it is **ungated**:
`bounds-allowlist.sh` greps `Bounds`, not `Enclosure`, so a future
`T: Enclosure` bound on something that certifies would be a hole with no CI row
against it. The stale sentence is fixed; the gate gap is issue **#701**.

**One more residue, from the sweep rather than from the review.**
`topo::census`'s duplicated `boolean::boxes` min/max justified itself with
*"cannot join [the `Bounds` allowlist], because the census validates `Dual`
bodies and `Dual` has no bracket"*. That "cannot" has lapsed; what is left is
the discipline rule, which is a process reason and does not by itself justify
two derivations of the same box that can silently diverge with no differential
row between them. Issue **#700**. Both of these are §D ordering rule 3 — a
lane's own residues are rows, not footnotes.

What newly admits a dual is the **bracket half**: `Aabb` constructors and the
curve/surface box builders, the boolean sweep's box lane, the placement
certificate's `Aabb` images, the chart-region predicate, the arc-fillet
carrier seam, and the fillet battery — all of them `T: Bounds` or
`T: Decide + Bounds`. Most read endpoints to *prune*, to *refuse*, or to put
an `f64` margin in a typed error payload; `topo::separation` is the exception
and it **grants** (see the correction above), so "they only prune or refuse"
is not the justification. The justification that covers all of them, grant
included, is **delegation**: at `Dual<f64>` every one of those numbers is the
`f64` run's number and every decision still goes through `Decide`, which
delegates to the value part, so the dual run takes the `f64` run's path — and
at `Dual<Interval>` the `Interval` run's. That is what D9's bit-identity
contract wants of it.

**One seam owes something, and it is worth recording precisely.**
`sweep::fillet::{battery, build, surgery}` is the single allowlisted
`Decide + Bounds` seam with **no lane trait behind it**, and the reason on the
record (M5 PR 12, `real.rs`) is that a lane split *"would have had an EMPTY
refusing side"* because `Bounds` had no `Dual` impl. That guard has now lapsed.

**The mitigation is narrower than first written.** *"Nothing reaches it at a
dual today"* is true of **in-repo** callers only: its one production caller
sits under `evaluate<T>`, which a dual cannot instantiate. This is an
API-first kernel, and `sweep::fillet::build::fillet_edges`,
`battery::run_battery` and `surgery::ring_clearance` are `pub` in `pub mod`s
of a library crate — an external crate instantiates them at `Dual64` today
(compiled, 2026-08-19 review). The `ContentBits` lock guards
`editor_core::evaluate`, not the public API.

What makes it **a standing obligation and not a live hole** is therefore the
AUDIT, not the reachability. All fourteen `Bounds` reads in
`battery.rs`/`build.rs`/`surgery.rs` were enumerated: every predicate's
`Ok`/`Err` comes from a `decide(...)` call (`face_clearance`,
`ring_clearance`, `radius_headroom`, `spine_regularity`, `chain_g1`,
`convexity_at`, `corner_config`), ten reads are typed-error payloads, and four
are selections. **Correction to the first framing of those four:** calling
them all *"representation-level selections among already-classified
constructions"* — `sugar.rs`'s ratified justification — is inaccurate for two.
`battery.rs:836` picks which endpoint's tangent is fed **into** `chain_g1`, an
input to a classification; `surgery.rs:1184` picks the whole-turn `k` for a
`t_split` fed **into** `body.split_edge`, a topological mutation. Their
dual-safety is real but rests on **delegation** — at a dual each takes the
value channel's branch, which is the base scalar's — not on being
post-classification picks, and the `sugar.rs` precedent does not cover them.

So the seam owes either the lane or a written reason it needs none, and it
owes it on the **public** surface rather than from the day E4 seeds a dual.
Recorded in `real.rs` (the home) and pointed at from the gate header.

### E4's door: the `Bounds` lock is open, and there is a second lock

S2's steelman named `Bounds` as E4's unregistered structural prerequisite —
`evaluate<T>` requires it, `Dual` did not have it, so `evaluate::<Dual64>` did
not compile and nothing recorded that. That lock is open. **It was not the
only one.** `evaluate`'s bound is
`Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane`,
and `editor_core::eval::ContentBits` — the trait that feeds a scalar's exact
representation into a content key — has impls for `f64`, `Probe` and
`Interval` and **none for `Dual`**. So `evaluate::<Dual64>` still does not
compile, for a reason no document had registered either, in a different crate
from the one S2 was looking at.

`crates/editor-core/tests/e4_dual_door.rs` pins exactly that: a `Dual64`
satisfies every one of `evaluate`'s bounds *except* `ContentBits`, asserted
for the all-but-one set so that `ContentBits` is the **named** residue rather
than a vague remainder. It is deliberately not fixed here. `ContentBits for
Dual` is a decision, not a formality: a dual's content key has to say whether
the **seed** is part of the key, and if it is not, the memo can serve one
parameter's pass from another parameter's — a soundness question about the
memo, which belongs to whoever builds E4. **Filed as issue #687.** The
suite's negative row — a `compile_fail,E0277` doctest on `ContentBits` in
`editor-core/src/eval/memo.rs` — is what goes red the day #687 lands, so this
record cannot silently outlive it.

### What this does NOT settle

**S3 / W2a — the fate of the four lane traits — is untouched and remains its
own unit.** D1 was its gate and the gate is now open, but the steelman's
compiled collapse (one trait plus a rank-2 job callback in `geom-core`, 16
impls → 2) was derived against the **one-trait** world and its central premise
— that `Bounds` is what distinguishes the lanes' certified side from their
refusing side — is no longer true. The lanes now refuse on a *ruling* that
`CertifiedEnclosure` already expresses as a bound, which is a materially
different collapse target: the question W2a now has to ask is whether four
traits, one trait, or **no** trait is the right shape given that a plain
`T: CertifiedEnclosure` bound can say what all four were saying. That
re-derivation has not been done and is not attempted here.

Two observations W2a will want, from this lane rather than from a fresh scan:

- **The four D9 bit-identity assertions are unaffected.** The pricing entry's
  five sites (`topo/tests/geometric_cube.rs`, `topo/tests/review_m2_pr3.rs`,
  `sweep/tests/extrude_acceptance.rs`, `sweep/tests/m5_pr11_quad_props.rs`,
  and `topo/tests/fixture/mod.rs`'s `certify_at_dual`) go through the lane
  traits' **refusing impls**, which are unchanged, so a `Dual` body still
  validates and still receives the typed refusal from the quadrature arm. This
  ruling adds a capability and removes none; the four-scalar test axes behave
  as they did.
- **One lane's static guarantee no longer rests on a missing impl — one, not
  four.** The generalisation first written here ("deleting a lane trait would
  make the certified machinery instantiable at a dual") is true of exactly
  `topo::chart_region`, whose predicate `chart_region_overlap` is
  `<T: Decide + Bounds>` (`chart_region.rs:345`) with no
  `CertifiedEnclosure` — and which is `pub` and re-exported from
  `topo/src/lib.rs:259`, so an **external** caller can already instantiate it
  at a dual without the lane being consulted at all. The lane guards the
  census path, not the function.

  The other three lane doors — `props::quad_lane::{cut_face, nurbs_face,
  chan, trig_at_start, start_point}`, `pcurve_cache::fitted_lane`,
  `edge_nurbs::lane` — are all `Decide + Bounds + CertifiedEnclosure`, so
  deleting their lane traits leaves them **still uninstantiable at a dual**;
  what would be lost there is the typed refusal a dual body currently
  receives, not the guarantee. W2a should price the four separately: three
  are about ergonomics and error shape, one is about access.

- **The hardening path that exists at `Interval` does not exist at
  `Dual<Interval>`, and nobody has written that down.** At plain `Interval` a
  caller can harden a `Decide + Bounds` seam by adding `CertifiedEnclosure` to
  its bound — the scalar satisfies both. At `Dual<Interval>` that upgrade
  **evicts** rather than hardens: the dual satisfies `Bounds` and not
  `CertifiedEnclosure`, so the seam silently stops admitting duals. That is
  intended under D1, but it means "harden this seam" and "keep duals out of
  this seam" are the same edit, and W2a will hit it first — a collapse that
  reaches for `T: CertifiedEnclosure` as the one bound saying what all four
  lanes said is also, at every seam it touches, a decision to evict duals.

## S45–S48 — reserved

IDs `S45`–`S48` are intentionally unallocated, so that items promoted
out of the S35/S40 roll-ups during review can be given stable IDs
without renumbering anything above.

---

# Findings raised by the Wave-1 fix lanes (2026-08-18)

Seven findings that the Wave-1 fix work turned up and that are **not**
restatements of anything above. They are recorded here rather than in the
tiers because their provenance matters: each was found by an implementer or
reviewer working inside a specific fix, which is a different evidence base
from the twenty structural scans.

`S45`–`S48` stay reserved for their stated purpose (promotions out of the
S35/S40 roll-ups), so these take fresh IDs.

**All carry blank verdicts. Per this document's own rule, none should
be acted on until it has one.** (S49–S51 were raised 2026-08-18; **S55** was
raised 2026-08-19 by the W1c lane, about that lane's own consequence, and
**S56** the same day by the S41 crossing lane, about the gate that could
not see its own diff.)

## S49. The census's planar × planar skip is justified by a claim about solids

- **Where**: `crates/topo/src/census.rs` (arm 1's skip and the justification it rests on)
- **Raised by**: the W1a fix lane (#620) and its reviewer, 2026-08-18
- **Scheduled as §D W2g.**

**FIXED by #637.** Arm 1 skipped a pair when `a.planar && b.planar` — a predicate
on the two **faces** — while the reasoning it cited was about planar-only
**solids**; a cylinder's caps are planar faces on a non-planar solid, so the skip
fired on pairs the justification did not cover.

**The jurisdiction call — arm 1 owns it**, and the other two lanes cannot take it
structurally rather than by preference: `sweep_conformal_patches` iterates
`curved_faces` only, and `snapshot` puts a face there iff its surface is not
`Plane`, so a planar face is *absent from the collection*; the confirm pass is
driven off declared records, and this class is the undeclared one.

**The real premise was about neither solids nor planarity.** `snapshot` keeps line
edges and drops curved ones, so only a **wholly line-bounded** planar face has its
whole boundary in front of the exact sweeps. The skip now tests that, a shared
`edge_is_line` binds it to `snapshot` with the unsound drift direction stated (a
widened `snapshot` alone would leave the skip firing on faces no longer covered),
and the settlement row asserts the planar pair **by key** in the contact plane.
Narrowing it exposed a second instance one line down: the same-`SurfaceKey`
deferral named "the conformal arm's pair" while that arm walks curved faces only —
now guarded, and provably behaviour-neutral because `same_key ⇒ a.planar ==
b.planar`.

**Not a live wrong answer, and the reason is an accident worth recording.** The
*pair* was genuinely unowned; the *body* refused anyway via a neighbouring pair,
because contact at a planar face is contact in its plane and every adjacent curved
face reaches that plane by construction — an arc lies in two distinct planes only
if it is a line, so a non-line-bounded planar face always has a curved neighbour
whose sound box covers both the rim and the plan extent. Measured cap-to-cap: 15
`CensusUndecidable` on wall pairs, **0** naming the cap pair. So A5's letter had a
pair-granular hole whose loudness was supplied by another arm's box fatness —
which is precisely why it was worth closing before #620's contemplated tightening
removes that accident.

The `gate_planar` → `gate_operand_kinds` rename went with it. Its first pass
missed four sites **outside the workspace** — `demos/` is `exclude`d, so
`cargo check` never compiled the files, one of which printed *"gate_planar refuses
curved operands"* **to the user**. Lesson recorded: a rename spanning excluded
members wants its own PR and its own `--manifest-path` check. Two residues are
**H14**.

**Verdict:** ACCEPTED (Evan, 2026-08-18) — *"should be scheduled but i have no
opinion on when"*. The jurisdiction call was part of the unit, not a prerequisite
decision, and #637 made it and argued it in the code.

## S50. Fillet corner patches mint `sense` bare, between siblings that derive it

**FIXED by #640.** Both corner mint sites now derive the bit from
`link.convexity.blend_sense()` through one `corner_convexity` helper, as their
blend and rim-band siblings in the same loops already did. Output-identical
today — three typed front doors refuse concave chains upstream of any corner
minting — and pinned by four in-crate rows at the deepest point the doors leave
reachable.

**Verdict:** ACCEPTED (Evan, 2026-08-18), **and the resolution was chosen**:
*"deriving at mint makes sense"* — a derived bit cannot rot when the front-door
gates change; a stated precondition can.

**Successor.** #640's review established that the sense bit was the only part of
the corner construction that derives anything: the ball centre sign, the corner
feet sign and the octant chart are all convex-hardcoded, so a concave input
would now yield convex-built geometry carrying a concave bit. Unreachable behind
the doors, and **filed as #644** — one change covering ball, feet, chart and bit
together, with the fixture and door work it needs.

## S51. VERIFIED by #636 — S42's verification never varied the loft's `v` direction, and loft's derivation holds anyway

- **Where**: `crates/sweep/tests/m5_s11_concave_sense.rs` (every row),
  `crates/sweep/src/loft.rs:30`
  *(the finding as raised cited `crates/sweep/tests/s42_loft_sense.rs`; that
  file never existed on main — #619 deleted its own draft of it and landed
  the S42 rows as the third verb's chapter of the S11 constructor audit.
  Corrected by #636.)*
- **Importance**: low to medium
- **Confidence**: unsure — a coverage residual, not a defect claim
- **Raised by**: the W1e reviewer, 2026-08-18, against #619
- **Verdict:** ACCEPTED (Evan, 2026-08-18) — worth a lane on its own terms:
  *"those tests are valuable even if they don't find anything today"*. That is
  why "may find nothing" was not a reason to defer, and the rows were the
  deliverable.

**No defect.** Loft's `sense` derivation holds on every chart this lane could
make twist. Both prescribed shapes were built as named and nothing was
substituted: a section pair whose **convexity differs between sections** (bulge
`−b` below, `+b` above on both bowed edges, so the wall is concave below,
convex above, flat at mid-height) and a **three-section `v_degree = 2`** stack.
`sweep_body`'s quarter-turn elbow, where `S_v` rotates 90° across one wall, was
added beyond the ask.

What the lane had to invent was the oracle. #619's probe compares against an
extruded twin, which exists only because identical sections at linear `v` *are*
an extrusion; none of these fixtures has one, because their sections differ.
There was nothing to substitute either — `point_in_solid` has no NURBS arm and
the boolean layer refuses a rung-3 operand at admission, both still pinned by
#619's `a_lofted_operand_refuses_the_union_check_typed`. The replacement reads
the body's **own level sets**: wall iso-curves at one `v`, closed into a planar
ring, containment by crossing parity, level found by bisection. It is
orientation-free by construction and so cannot inherit the bit it is testing —
and that is established by measurement rather than argument: flipping *every*
wall's `sense` leaves the oracle's verdicts bit-for-bit unchanged.

Three things the lane established outlast the result:

- **Degree is not the criterion.** The flipping pair is itself a `v_degree = 1`
  section pair and its chart turns hard, so this finding named the right gap
  for the wrong reason.
- **The guard was not the premise.** Bisection needs `height(t)` monotone; the
  only guard was a proxy cosine `> 0.1`. A 120° elbow satisfies the guard at
  `cos = 0.28` while **already non-monotone**, and the row passed by luck. The
  shipped 90° fixture sat exactly on the boundary, pinned there by an
  anti-vacuity assertion. Bisectability is now scanned directly. A doc claiming
  "preconditions asserted, not assumed" is worth checking against which
  precondition actually carries the result.
- **Two rows were restatements.** `flipped_face_sense_for_tests` moves no
  geometry, so the probe returns the identical point and exactly `−n`, and the
  "flip inverts" assertion is the swapped tuple — algebraically the same
  statement as the row beside it, at double the oracle cost. This lane's copy
  and **#619's original** were both deleted, and the suite got *faster*.

The parity walk declares itself against **§S17** with the three blocked reuse
paths named (`point_in_loop` is `pub` but takes `(&Body, LoopKey, …, Band)`; the
other two are private; and reusing a `Decide`-certified door would reintroduce
#619's ε-fragility). #619's residual still stands: **no tier, prop or boolean in
the kernel reads a lofted wall's sense**, so on these shapes these rows are the
only thing pinning the bit. `sweep_body`'s helix rows are **H13**.

## S52. An in-crate test helper is invisible from `tests/`, so every integration suite mints its own

- **Where**: `crates/topo/src/test_support_impl.rs`,
  `crates/sweep/src/test_support.rs` (both new); the copies collapsed
  were `crates/topo/tests/m3_pr5_boolean_ops.rs`'s `Census` and the
  six private `cube`s in `crates/sweep/tests/`
- **Importance**: medium
- **Confidence**: sure on the mechanism
- **Raised by**: the H8 (#641) and H9 (#640) lanes and their reviewers,
  2026-08-19

Both lanes hit the same wall from opposite directions. A `#[cfg(test)]`
or `pub(crate)` helper cannot be named from a `tests/` binary, which is a
separate crate — so an integration suite that wants the vocabulary
declares its own copy, and the copies drift. #641 collapsed the *in-crate*
duplicate for free and left the `tests/` one standing; #640 put `sweep`'s
shared fixture in a crate-root `#[cfg(test)]` module and left the six
integration copies standing, for the same reason.

**EXECUTED for `topo` and `sweep` by #668, on a mechanism that is
repo-native rather than newly established.** The gate is an
**off-by-default cargo feature reached through a self dev-dependency**
(`[features] test-support = []`, `[dev-dependencies] <crate> = { path =
".", features = ["test-support"] }`), on exactly when the crate's own tests
compile the library and off for every other build. That is the wiring
`topo`'s live **`sweep-testing`** feature already uses one screen away in
the same manifest — and the wiring the retired `profile` shim used — so
#668 applies an existing in-crate pattern to a second purpose.

**Existence and visibility are gated separately**, which is what keeps
"not public API" true in every profile:

- `topo` — `ArenaCounts` and `Body::arena_counts()` moved into a private
  `test_support_impl` whose **existence** gate is `any(debug_assertions,
  test, feature = "test-support")`, the union of the type's three
  consumers (the D1 debug postcondition is a real one), replacing #641's
  `any(debug_assertions, test)`. The **visibility** gate on the public
  `test_support` facade is `any(test, feature = "test-support")` alone, so
  `topo::test_support` resolves in no plain build of either profile.
  `Body::arena_counts()` stays `pub(crate)` — an inherent method's reach
  follows its own visibility, not its module's — and the facade carries a
  free `arena_counts(&body)` for cross-crate readers. The integration
  `Census` copy is gone.
- `sweep` — `fixtures.rs` became `test_support.rs` under `any(test,
  feature = "test-support")`, where existence and visibility coincide
  because nothing in it has a non-test consumer. `cube` took the side
  length its callers were passing, and all six `tests/` copies collapsed to
  it. The seventh `cube` in `crates/sweep/tests/m6_surgery_interval.rs`
  stays, and the first draft of this row justified that wrongly — it
  called it "a different fixture with the same name". It is the **same**
  fixture one scalar up: `RawLoop::polygon` is `new` with every bulge
  zero, so it is the same four-corner loop, same `SketchPlane::xy()`,
  same `Tolerance::get()`, same `Extrusion::Distance`, differing only in
  `Interval` vs `f64`. Collapsing it needs `cube<T: Real>` — a
  generalization, not a deduplication, which is the real reason it is
  out of scope. Filed at #672.

**Two follow-on corrections the execution forced.** (1) `scripts/doc-gate.sh`
runs `cargo doc --workspace --all-features` as a **required** CI row, which
turns `test-support` on — so the repo's own gate would have published, as
public API, a module whose first line says it is not one. Both facades are
`#[doc(hidden)]`, and so is the private `test_support_impl` behind them —
which turned out to be the **only** private module of `topo` a doc build
renders at all, not because `--document-private-items` shows everything
private but because the crate's other private modules (`fixtures`, `seqgen`,
`tier3_tests`) are `#[cfg(test)]` and do not exist in a doc build, while that
one exists whenever `debug_assertions` does. Verified against a wiped
`target/doc`: zero `test_support` references in either crate's `index.html`
or `all.html`, no generated module page, and no `ArenaCounts` page anywhere
in the doc tree. (2) `topo` now has **three** homes for
test vocabulary — `src/fixtures.rs` (`#[cfg(test)]`), `src/test_support_impl.rs`,
and `tests/common/mod.rs` — so `test_support_impl`'s docs carry the routing
rule for all three and the other two point at it: **an item lives at the
narrowest home all of its consumers can reach**, which makes `tests/common`
the default and this module the exception for items the library itself also
names.

**The gate was proved, not asserted.** A throwaway downstream crate that
path-depends on both and names `topo::test_support` / `sweep::test_support`
fails with `error[E0433]: cannot find test_support` for both crates in
**both profiles** — `cargo build --release` and `cargo build` alike. That is
the real downstream case, stronger than a symbol grep, since an
uninstantiated generic leaves no symbol either way. `cargo test --release -p
topo -p sweep` passes, which is the row that rules out
`#[cfg(debug_assertions)]`.

**And the precedent it was modelled on was leaking.** Checking the
mechanism instead of citing it found that `crates/sweep/Cargo.toml:52`
carried `topo = { path = "../topo", features = ["sweep-testing"] }` in
**`[dependencies]`**, three lines under a comment reading *"Dev-dependencies
are the only place it is on"* — and `topo`'s own manifest claimed *"no
production dependent can reach an injector."* Both were false. Cargo unifies
features across a build graph, so that one edge turned `topo`'s
**failure-injection** doors on for every production dependent of `sweep`:
measured, a downstream crate compiled `--release` naming
`topo::PlantedDegradation` and `topo::sweep_traces::<f64>`, and
`crates/pncad/Cargo.toml` depends on both. #668 moved the featured edge to
`[dev-dependencies]` (the plain edge stays), which `sweep`'s non-test code
builds fine without, and the same probe now fails with `E0425` in **both**
profiles.

**The invariant is now a gate, not a sentence.**
`scripts/gates/test-features-dev-only.sh` fails if any non-dev dependency
edge in **any** of the repo's 23 manifests — the kernel workspace, its root,
and the excluded `demos/`, `tools/` and `interval-transcendentals/` roots —
enables a feature named `test-support` or `*-testing`, or if an ordinary
feature forwards to one. It parses with `tomllib` rather than grepping, for
the reason `kernel-serde-free.sh` gives about dependency-entry spellings plus
one of its own: `features = [...]` says nothing about which table it sits in,
and the tables that matter nest.

**What it claims is exactly what a manifest parser can support**, which took
one more correction to get right. The header first said a test-only feature
was reachable "ONLY through a dev-dependency edge" — an absolute this repo's
own CI falsifies, since `scripts/doc-gate.sh` runs `cargo doc --workspace
--all-features` and any command line can say `--features topo/test-support`.
The gate constrains **manifests, not invocations**; that is now the first
thing its header says, and it is the reason the facades need `#[doc(hidden)]`
rather than a competing claim to it.

**Its self-test is derived from ROUTES, not spellings**, and that is the
part worth carrying forward. The first version enumerated the spellings its
author had thought of, passed its own self-test, and was then defeated twice
inside its claimed scope — by `[workspace.dependencies]` + `workspace = true`
(a traversal bug that *read* as covered: the walker recursed only under the
key `target`, so the root manifest was scanned but never descended into),
and by a same-crate forward `interval = ["test-support"]`, which has no
slash and so slipped a check that required one, while the pass message
claimed to cover it. Both are the live leak's own semantics in different
clothes. The gate now enumerates the eight distinct ways a feature can be
switched on for a non-dev edge (inline, workspace inheritance, `target.*`,
`build-dependencies`, cross-crate forward, same-crate forward, a two-deep
forward chain, and the weak `dep?/feat` spelling) and plants one case per
route. Both former defeats were also reproduced against the real manifests
and watched firing there. This is the third gate in this batch whose
self-test could not have found its own hole — the general rule is that **a
self-test derived from the implementation can only confirm what the author
already believed**.

**The negative control does as much work as the positive cases.** Two later
rounds each turned up a shape that is CORRECT code and that the gate
reported: `[package.metadata.deb.dependencies]` (a packaging tool's own
config, caught only because the walker had been made to trust the key name
`dependencies` anywhere in the document — descent is now by the places cargo
actually puts dependency tables, `target` and `workspace`), and `fuzz =
["test-utils"]`, the valid slashless spelling for activating an optional
dependency that happens to share a prefix with the naming convention. The
`test-*` arm of the pattern is gone for that reason: `test-support` and
`*-testing` are the two spellings the repo actually uses, and a wildcard that
matches a real workspace member turns correct code red. Both shapes now sit
in the clean fixture, so a future widening of either has to get past them.

The lesson generalizes past this row: a safety property asserted in a
comment three lines from the code that violates it had survived every other
gate in the directory.

**This row's own "the precedent does not exist" claim was half wrong, and is
corrected here.** Both #641's PR body and the first draft of this row named
`crates/profile/src/test_support.rs` as the shipped remedy; that **file** was
indeed retired by LIB-RETTAIL/ONARC, and `pncad/src/profile.rs:50` says so
outright. The correction then over-reached: the **mechanism** was never
retired. `profile`'s shim used this exact feature + self-dev-dependency
wiring, and `crates/topo/Cargo.toml` still declares it today at the
`sweep-testing` feature and its self dev-dependency, with a comment saying
what it is for. Searching for the pattern *by module name* found the deleted
file and missed the live wiring — the same failure mode S39 catalogues.

**Why not a `tests/common` module instead.** Both crates aggregate their
suites into one `tests/all.rs` binary, so a tests-side shared module *is*
reachable across suites (`topo/tests/common` and `step-export/tests/common`
already work that way). It cannot serve either case here: `topo`'s
`ArenaCounts` is one type the *library* also uses, so a tests-side copy is
still a copy; and `sweep`'s `cube` is named by in-crate pins in
`fillet/surgery.rs` and `fillet/build.rs`, which a `tests/` module cannot
reach. The gate puts one definition where both sides can name it.

**Verdict:** RULED (Evan, 2026-08-19): **kernel crates may carry their own
test support, gated so it does not show up in release builds.** Executed by
#668 for `topo` and `sweep`, whose residue was filed as **#672** rather than
left as the word "unscheduled" (Q6: disclosure is not a schedule).

**#672's `topo`/`stl` half is closed; two rows of it are not.** Closed:
`src/fixtures.rs`'s `ArenaSnapshot` no longer restates the seven — it
**holds** an `ArenaCounts` beside the three geometry-arena lengths, so the
crate has one topology census with one producer, `Body::arena_counts()`,
which the D1 debug postcondition already cross-checks against each
operator's declared `ArenaDelta`. `crates/topo/tests/m3_pr4_boolean.rs`'s
3-field `Census` is gone: its only use was the "operands untouched" check,
which now runs over all seven arenas rather than a three-component sample of
them — a deliberate **strengthening**, not a rewrite. The other three
`tests/` copies are **not** the arena vocabulary and keep distinct types
that now say so: `SideCensus` (`m3_pr3_split.rs`) is a four-arena projection
whose six expectations are hand-derived for exactly those four;
`EulerCensus`/`EulerCensusDelta` (`review_m3_pr1.rs`) carries `rings`, the
`r` of `v − e + f − r`, which is summed over faces and is no arena length;
`GraftCensus` (`graft_disjoint.rs`) carries `shell_refs`, summed over
solids, likewise. Four identically-named types for four different quantities
was itself the drift. `crates/stl/tests/`'s two byte-identical `brick`
fixtures now share `tests/common`, which is the routing rule's default home:
nothing in the `stl` library names `brick`, so no crate-level facility is
warranted.

Still open under #672: `crates/sweep/tests/m6_surgery_interval.rs`'s `cube`,
which needs `cube<T: Real>(l: T) -> Body<T>` — a generalization, not a
deduplication — and the local body builders in `crates/mesh/tests/`,
`crates/step-export/tests/` and `crates/editor-core/tests/`.

## S53. Two `Ledger`s in one crate, with drifted field sets

- **Where**: `crates/topo/src/seqgen.rs` (`Ledger { v, e, f, h, r, s }`),
  `crates/topo/src/review_m1_pr3.rs` (now `GenusInputs { v, e, f, r, s }`)
- **Importance**: low
- **Confidence**: sure it exists; the missing `h` is deliberate, settled below
- **Raised by**: the H8 reviewer (#641), 2026-08-19

Same name, same crate, one component apart — and the two are not the same
kind of thing. `seqgen`'s `Ledger` is the **running** Euler count the
sequence generator carries forward and checks against independently counted
arenas after every op, so `h` is one of the things it carries. The suite's
five-field type is a **census genus is derived FROM**: `genus` solves
`v − e + f − r = 2(s − h)` for `h`, and `check` asserts that derived value
against the expected one.

**The narrower field set is correct and deliberate** (checked 2026-08-19):
`h` is the quantity under test, and tracking it as a field would make the
assertion tautological. So the name was the entire finding, and the fix is a
rename. The suite's type is now `GenusInputs`, whose doc states why genus is
absent and points at `seqgen::Ledger` for the running six-component ledger.
`seqgen`'s keeps the name `Ledger`: with one such type left in the crate it
is unambiguous, and it is the crate's canonical Euler ledger, cited by that
name from the `seqgen` module docs and from `review_m1_pr4.rs`.

**Verdict:** EXECUTED by #673 (2026-08-19) — rename only, behaviour
identical. Nothing further is owed here: adding `h` was considered and ruled
out above, and renaming the `review_m1_pr3.rs` **file** is W3a's job, not a
rename pass's.

## S54. The "kept in step BY HAND" ladder, which the crate around it has twice repudiated by name

- **Where**: `crates/editor-core/src/eval/wire.rs` (`resolve_selection`,
  `resolve_declarations`); the two sites that cited it as the anti-pattern
  they fixed, `crates/editor-core/src/names/flush.rs:37` and
  `crates/editor-core/src/persist/check.rs:9`; same family at
  `crates/profile/src/path/arc_fillet.rs:21` and
  `crates/pncad-py/src/tests.rs:245`
- **Importance**: medium
- **Confidence**: sure on the structure
- **Raised by**: the detector #641 suggested, run 2026-08-19

**Verdict:** ACCEPTED (Evan, 2026-08-19) — "worth doing. Share it." Executed
by **#670**, below.

**FIXED by #670.** The two doors now walk ONE ladder, a private
`mod ladder` sited between them in `wire.rs`, and the "if you change either
ladder, change both" warning is deleted rather than reworded. The shape that
beat the arity objection is the one the finding's own steelman preferred:
share the RUNGS, not the lookup. `Landing` (`Unique(EntityRef)` / `Tied(u32)`
/ `Absent`) is what a table read produces; `live()` is rung 1 (`NodeGone` with
the deleted-vs-foreign split) and hands back a `Live<'_>` token, which BOTH
`landing()` and `resolve()` require; `resolve()` is rungs 2 and 3 (`Ambiguous`
with the tie witness, `Vanished` with the `NodeChanged` fallback and
`last_good: None`).
No closure, no generic over "how to look a name up", one hop from either door.

Each door keeps exactly its own arity, which is what makes the shared version
MORE legible than the duplication rather than less: the fillet door is now
`live` → `landing(&live, target)` → `resolve` → the edge-kind refusal, six
lines with
every rung named against a 45-line inline ladder; the declare door reads its
two tables into two landings, picks a side, and refuses `DeclareBothOperands`
itself — the one refusal in that function that is not N5's, previously buried
in a closure returning `Option<Result<…>>`. The declare door is a wash rather
than an improvement on size, and its four-arm landing table is spelled out
in full — the review caught the first version folding `(Absent, Absent)` into
a fall-through plus a comment, which for a function whose whole job is which
typed refusal comes out is the wrong direction.

The rung ORDER, which was the residual hand-coupling a pieces-only extraction
would have left, is enforced by the type system — and the review is why it
actually is. The first version gated only `resolve()` on the `Live` token,
which enforces rung 1 before rungs 2–3 but NOT before a door's own refusal,
the case the finding used to motivate the order at all; the reviewer moved the
declare door's side-picking ahead of rung 1 and it compiled clean with all 566
tests green. Fixed by threading the token one step further: `landing()` takes
`&Live` too, so no TABLE can be read before rung 1, and a refusal about what
the tables say — `DeclareBothOperands` needs both landings — cannot be reached
first. The same mutation is now a compile error. Threading the token also
closes a second hand-coupling for free: `landing` and `resolve` can no longer
be called for different names, so an `Ambiguous` payload's tie width is
measured on the name the payload is built from, by construction.

Worth recording for whoever revisits this: the inverted order was
**unreachable through the document API**, which is why no pin could have
caught it. A name present in an operand table was minted by an ancestor of
that operand, so it cannot be stranded without the operand failing first —
the reachable `NodeGone` case is a name minted by a node that is NOT an
ancestor (the existing delete fixture). The type-level fix is therefore
defensive, and correctly so: the claim in the module doc is now true rather
than aspirational.

Behaviour-identical, arm by arm, including payloads. The pins were checked for
what they actually assert rather than assumed: `m6_5_selection_refusals.rs`
pinned NodeGone/`NodeDeleted`, `Vanished`'s full payload, `Ambiguous` minus
`tie.node`, plus the kind and empty refusals; `m4_pr5_declare.rs` pinned
`Ambiguous`'s payload but `Vanished` by Debug SUBSTRING only, and neither door
pinned the witness site. Both gaps are now closed (declare's `Vanished`
asserts `last_good: None` and `RecipeEdit{NodeChanged}` typed; both doors
assert `tie.node == name.node`). **`ForeignNode` stays an OPEN GAP**, unpinned at
both doors and not closed here: the edit door refuses never-existed ids before
evaluation, so the arm is reachable only across documents. Sharing the ladder
does not narrow the gap — both doors still lack a pin — it only means the
unpinned arm now has one implementation instead of two. Mutation-checked:
flipping `tie.node` in the shared ladder fails a pin in BOTH suites, which is
the property the duplication did not have.

**RESIDUE, named and not folded: the crate's OTHER ladder.**
`crates/editor-core/src/resolve/mod.rs`'s `resolve_impl` (rung 1 at :549–:560,
the tie witness at :578–:587, the removal-edit helper at :535, the recipe-diff
edit derivation at :1135) carries rung 1 as the SAME code —
`doc.node(name.node).is_none()`, then `name.node.0 < doc.next_id` →
`NodeDeleted` else `ForeignNode` — and builds the same `TieWitness`, with the
"ids are never reused" sentence now appearing verbatim in three places. So the
crate still has two ladders agreeing by hand, at coarser grain, and the
unfolded one lives in the module whose whole job is resolution. They have
already diverged in one respect worth naming, since it is exactly the drift
this class predicts: `resolve_impl` sets `TieWitness.node` to the CARRYING
node returned by its whole-evaluation lookup, while the mid-evaluation ladder
sets it to the MINTING node (no carrying node exists for a single value's
table) — origin/main's behaviour on both sides, preserved. Folding is
cross-module and larger than one evaluation door; deliberately out of scope
here, recorded in `wire.rs`'s `mod ladder` doc as well as here.

**The two family members named above were deliberately NOT folded in**, and
neither is this refactor. `arc_fillet.rs:21` restated the S8 justification and
**miscited it** — it credited `sugar.rs`, which does not contain the paragraph
and never did; `fillet_select.rs:16-18` does, and that module's header argues
in as many words for the rule having one home ("the ladder is stated once …
instead of the same paragraph twice"). The wrong pointer is the Q4 doc-rot
case here, and it is fixed in this PR: the header now cites
`crate::fillet_select` and says plainly that the restatement exists only
because the CI-discipline allowlist line needs a purpose-matched sentence. The
duplication itself is a paragraph, not code, and stays. `pncad-py/src/tests.rs:245` is
the family's already-solved instance: the `[lints]` table CANNOT be shared
(the crate cannot inherit `[workspace.lints]` — `unsafe_code = "forbid"`
versus PyO3's generated `unsafe impl`), and the hand-restatement is already
held by a test that breaks the build on drift. Duplication made incapable of
drifting is the outcome, reached mechanically instead of structurally.

**Method note — RATIFIED (Evan, 2026-08-19) and applied.** #641's parent-sense
row found its fourth copy through a comment whose only job was to explain that
two spellings were one rule, which suggested a detector: *a comment that exists
to reconcile two spellings of one rule is evidence the rule needs one home, and
it is usually the only evidence, because the code compiles either way.*

It is now a bullet under **Q2** in `docs/prompts/reviewer-style-lane.md`. Two
things were corrected before it landed. The first draft was a fixed phrase list,
which is the checklist failure that document's own §1 warns against — so the
**question** is the instrument and the pattern is demoted to a cheap first pass.
And the pattern itself was too narrow (Evan): it is now case-insensitive and
covers the hand-sync, `duplicated from`, `not shared with`, `restated` and
`change both` phrasings, the last of which is the strongest signal in the
`wire.rs` ladder and which the original would have missed.

Its own limits are stated where it is used: it misses phrasings nobody has
written yet, and it over-fires on prose about a *user* authoring something by
hand. Run over `crates/*/src` at ratification it named `arc_fillet.rs` and
`pncad-py/src/tests.rs` — the two family members S54 records — and no longer
names `wire.rs`, because #670 removed the ladder that motivated it.

## S55. `Enclosure` is a live trait with no consumer left

- **Where**: `crates/geom-core/src/real.rs` (the trait, and its blanket
  `impl<T: Bounds> Enclosure for T`), `crates/geom-core/src/lib.rs` (the
  export), `crates/geom-core/src/ring_interval.rs` (the direct impl)
- **Importance**: low
- **Confidence**: sure — this is a fact about the tree, not a judgement
- **Raised by**: the W1c fix lane (#643), 2026-08-19, as a direct
  consequence of its own change.

`Enclosure` existed for exactly one reason, stated in its own docs: the C9
ring is not an evaluation scalar, so it cannot implement `Real` and
therefore cannot implement `Bounds`, and `Enclosure` was the smaller trait
`f64`, `Interval` and `RingInterval` could all meet at. Its only generic
consumer was `geom_core::spline::hull` — every `hull` entry point took
`E: Enclosure`.

**#643 moved all of `hull` to `CertifiedEnclosure`**, because a hull bound
is a certificate and a coefficient that merely carries a bracket is not
enough. `hull` now reads no raw brackets at all. So the trait, its blanket
impl over `T: Bounds`, its direct `RingInterval` impl and its public
export all remain, and **nothing in `crates/*/src` is generic over it any
more**.

This is S11's genre one step sideways: not machinery with no *producer*
but machinery with no *consumer*, still exported as public API. The
question is which of these it is, and they have different answers:

1. **It still earns its keep as the meeting point** — the vocabulary that
   says "these three types all carry a bracket" is worth naming even if no
   generic body currently quantifies over it, and the next certification
   helper will want it.
2. **It is now `Bounds` wearing a second name** — the blanket impl means
   every `Bounds` type is one automatically, `RingInterval` is the only
   type that is one *without* being a `Bounds` type, and a trait whose
   entire remaining content is "`Bounds`, or the ring" could be spelled at
   the one site that needs it.

Deciding this was out of #643's scope — removing or re-scoping a public
trait is design content, and the lane's mandate was the decoration seam.

**Verdict: DEFERRED (Evan, 2026-08-19), pending the `Bounds` narrow-vs-broad
split.** This question is downstream of that one, not independent of it:
`Enclosure` and `CertifiedEnclosure` already *are* a narrow/broad pair in
miniature, so whatever the split settles will either give `Enclosure` a job or
subsume it. Deciding it first would prejudge the larger question from the
smaller one. Whoever takes the split should absorb this row rather than treat it
as separate work — and should weigh #643's evidence in the other direction, that
collapsing the two vocabularies into a supertrait produced an `E0034` ambiguity
storm across `ssi/certify.rs` and was backed out.
Note that (2) is not obviously right even on its own terms: the ring is a
genuine second implementor, and #643's own experience is evidence *for*
keeping the two vocabularies separate rather than merging them, since
collapsing `CertifiedEnclosure` into `Enclosure` as a subtrait is exactly
what produced the `E0034` ambiguity storm it backed out of.

**Verdict:**

## S56. FIXED by #676 — the compound-`Bounds` gate was order-sensitive, so half the spellings it forbids were invisible to it

- **Where**: `scripts/gates/bounds-allowlist.sh` (the matcher, the
  header's `ssi/enclose.rs` paragraph, and `plant`);
  `crates/geom-brep/src/ssi/enclose.rs`, `crates/geom-curves/src/nurbs.rs`,
  `crates/geom-surfaces/src/nurbs.rs`; `crates/geom-core/src/real.rs` (the
  `Bounds` scope rule, and `CertifiedEnclosure`'s doc)
- **Importance**: medium
- **Confidence**: sure — reproduced by planting both spellings
- **Raised by**: the S41 crossing lane (#671), 2026-08-19, from its own
  diff; found by that lane's adversarial reviewer.

The gate grepped `\+\s*(geom_core::)?Bounds\b`. The `\+` was a **required
prefix**, so `Bounds` was only seen when something else preceded it:
`T: Decide + Bounds` fired, `T: Bounds + Decide` did not. The two orders
are the same bound to the compiler; the gate answered a question about
token order. Its `plant()` planted only the spelling its author had in
mind, so the self-test could not have caught it.

**Three files carried the invisible spelling**, none allowlisted:
`ssi/enclose.rs` (ten signatures, since #643), `geom-curves/src/nurbs.rs`
(two) and `geom-surfaces/src/nurbs.rs` (one), the latter two from #671
itself. That was not carelessness: `CertifiedEnclosure`'s own doc comment
**prescribed the exact string** `T: Bounds + CertifiedEnclosure` as *"an
honest inventory of the doors it uses"* — the discipline's documentation
was instructing authors into the one order its gate could not see.

Worse than the miss was what the miss preserved. The gate header claimed
`ssi/enclose.rs` *"takes the sole-bound `T: Bounds` the rule allows
everywhere"* — false since #643, and undetectable precisely because the
order-sensitivity kept the file quiet.

**Verdict — Evan's ruling, 2026-08-19: `Bounds + CertifiedEnclosure` is
not a compound bound in the rule's sense, and the resolution is an alias,
not an exception to the matcher.**

The rule exists to catch **an evaluation or decision parameter that has
also been handed bracket extraction** — one parameter wearing two hats.
That is why every allowlist entry is justified with the same sentence
shape: *it simultaneously DECIDES and reads brackets*. `Bounds` is a
subtrait of `Real`, so `T: Bounds` alone already carries the evaluation
ops; an **extra** bound means the parameter does something beyond reading
brackets, and in every ratified exception that something is `Decide`.
`Bounds + CertifiedEnclosure` has no decide half — `enclose.rs` contains
**zero** occurrences of `Decide` — and both halves are bracket-side
doors: stored endpoints, and the fallible bracket that refuses below
`Decoration::Def`. It was never the rule's class.

A regex carve-out was rejected because it makes the rule un-statable
(*compound bounds are forbidden, except this pair*) and the next pair
needs another carve-out, invisible from the code. So the pair is named:
`geom_core::CertifiedBounds`, with a blanket impl, and the sites write
the **sole** bound `T: CertifiedBounds`. The honest inventory is kept —
it is now spelled as one name rather than as a compound the gate must
special-case. `Decide + CertifiedBounds` is still a compound bound and
still fires, which is correct: that genuinely is an evaluation parameter
with brackets.

**Executed by #676.** All thirteen non-deciding signatures converted
across the three files. `ssi/certify.rs` and `topo/props.rs` were **not**
converted — every one of their sites is `Decide + Bounds +
CertifiedEnclosure`, genuinely decides and brackets, and they stay
allowlisted on their existing justifications. The matcher is now
order-insensitive, its header's `enclose.rs` paragraph says what is now
true (sole-bound `CertifiedBounds`, still deciding nothing), and `plant`
plants **each order as its own case**, since a single fixture carrying
both would still fire if only one spelling matched. Matcher, alias and
conversion landed together: the matcher fix alone turns CI red on
`enclose.rs`, the lesson #668's gate work learned.

Two residues are now stated in the gate header rather than left to be
found: the match is **line-based**, so a bound broken across lines
(`T: Bounds` ending one line, `+ Foo` beginning the next) escapes it, and
closing that needs a parser rather than a grep; and the alias's own two
definition lines are skipped by name, since a name's definition is not a
use of it — two lines, not the file, so any other compound bound in
`real.rs` still fires.

That last claim is **held by a test rather than by the header prose**,
which is the shape this batch spent the day removing. A third planted
case writes `real.rs` carrying both skipped definition lines *and* an
ordinary `T: Decide + Bounds` signature below them, and requires the gate
to fire; widening the skip into a file-wide allowlist entry makes that
case fail (verified by mutation). So a future loosening of those filters
— someone making them fuzzier to survive a reformat — cannot silently
blind the gate to the file that defines the rule.

---

## S57. The `readback` class is alive one crate over, and the "one door" guard cannot see it

- **Where**: `crates/editor-core/src/names/emit_topo.rs:48`,
  `crates/sweep/src/fillet/build.rs:247`,
  `crates/sweep/src/fillet/battery.rs:175`, `:183`, `:189`
- **Confidence**: sure
- **Raised by**: the S34 fix lane's review (#697), 2026-08-20, by running the
  `face_pose`-shaped sweep that PR declined to run and stated as its blind
  spot. **The blind spot was real and it hit.**

**A sixth copy of the readback walk, in the crate the fix was about.**
`emit_topo.rs:48`'s `face_plane` is `get_face` → dangling refusal →
`get_surface` → dangling refusal → destructure `Surface::Plane`. It is not a
literal copy — it folds `sense_sign` and refuses non-planar carriers — but it
refuses with `NamingError::Emission { what: "face_plane: dangling" }`, which
is **verbatim the `&'static str`-names-the-lookup defect #697 eliminated**,
still standing in `editor-core`, the crate whose dependency on `sweep` the
whole finding was about.

**Four `Body`-only accessors housed in an op crate — S34's own shape, one
crate away from where it was looked for.** `fillet/build.rs`'s `outward_of`
and `battery.rs`'s `outward` / `face_of` / `carrier_of` each take a `&Body`
plus an arena key and touch nothing from `sweep`. `outward_of` is a
hand-written copy of `topo::face_normal::face_outward_normal` — the function
whose module doc calls itself **"the one door"** and whose mechanical
anti-re-fork guard walks `topo/src` **only**, and therefore cannot see a
re-fork that lives in `sweep`.

That last point is the finding's sharpest edge and generalises past these
sites: **a guard scoped to one crate cannot enforce a rule stated about a
concept.** The `face_normal` guard was built (#690) precisely because a fix
pass re-forked the planar sense flip while creating the door, and it is
correct and load-bearing within `topo` — but the copy it was built to catch
already existed outside its walk.

Where else to look, unswept: `crates/mesh/src/walk.rs:973`,
`crates/step-export/src/`, and the three sites already named as D6's
hand-multiply class (`boolean::solid_contain::face_plane`,
`chord_join::face_plane_normal`, `merge_faces.rs`).

**Verdict:** _(unreviewed)_

---
## S58. "This face's domain is an iso-rectangle" is re-derived per consumer, in three representations, and no two agree on what it means

- **Where**: `crates/geom-brep/src/props/curved.rs:421` (`du_of_rims` /
  `props_du_consistent`), `crates/mesh/src/curved.rs`
  (`require_swept_rectangle` / `entries_off_bbox`, added by #648),
  `crates/geom-brep/src/props/curved.rs` (`torus()`'s `props_rim_level`)
- **Importance**: high
- **Confidence**: sure
- **Raised by**: the #649 investigation and its boolean-door probe, 2026-08-19

Three consumers need the same property and each derives it independently, from
different data, to different strengths:

| Consumer | Derived from | Strength |
|---|---|---|
| `props`' closed form (cylinder/cone/sphere) | rim structure — per-group **span sums** | **unsound**: sums are a consequence of rectangularity, not equivalent to it (**#649**) |
| `props`' closed form (torus) | rim **levels** — every rim at an end of the anchor meridian's `[v0,v1]` | sound, and the only arm the #649 probe could not break |
| `mesh`'s curved lane | the walked **UV polygon** vs its own bounding box | sound for shape, but sees the wobble of **#653** as well |

The three do not agree, they cannot be compared, and the disagreement is
load-bearing rather than cosmetic: the torus arm is the only one that is right,
and it is right by accident of a periodicity constraint rather than by anyone
deciding this is how the property should be tested.

The cost of the fragmentation is already paid twice. **#649** is a wrong
certified volume with `pad = 0.0` on three of the four kinds. And until #648
the mesher had no check at all — it was protected *transitively*, by `props`'
inability to measure the same faces, which is the kind of protection a later
milestone deletes without noticing.

**Verdict:** ACCEPTED (Evan, 2026-08-19), **and the resolution is chosen: it
should be ONE named predicate.**

Notes for whoever takes it, so the unit does not quietly become four:

- **The predicate belongs on the face**, derived from rim structure, and the
  right rule is the torus's: `w(v)` changes only at a rim level, so *every rim
  at one of the two extreme v-levels* forces `w ≡ du`, which is exactly what
  `area = r·du·(hi−lo)` assumes. Sufficient, and possibly slightly stricter
  than necessary — an interior level with matching `+`/`−` groups would leave
  `w` unchanged — which is the right direction for a precondition.
- **It subsumes #649's fix.** Do not do them separately: the level rule *is*
  the sound predicate, and landing it anywhere other than the one home
  re-creates this finding.
- **The mesher's check is two questions wearing one coat.** `entries_off_bbox`
  answers *"is this domain a rectangle"* (this finding) **and** *"did the walk
  produce a consistent polygon"* (**#653**'s ulp wobble). Once the face-level
  predicate exists, the first question moves to it and only the second stays in
  `mesh` — which is also the cleaner statement of what #653 is about.
- **Ask whether it is a tier property.** Three consumers sharing one
  precondition is an argument for refusing such a body once at `validate`
  rather than at each door. Not decided here.

---
---
# §A. Where I would start

**Superseded 2026-08-19 by §D, and kept as written.** Three of its four items
are done — S16 and S23 were indeed where the wrong answers were (#620, #617),
and S4's two drifts were both real (#618, #632). Item 2 is the one still open,
and it has been sharpened rather than answered: #643 split `Bounds` in two on
forcing evidence, and Evan's D1 ruling settled the `Dual` half, so *what the
scalar abstraction is for* now has an answer and only its consequences are
outstanding. The list is left unedited because its hit rate is the useful
thing about it.

Not a recommendation about what to *do* — the report proposes no fixes —
but about which questions look most worth answering first.

1. **S16 and S23.** These are the two places a real wrong answer is most
   likely already hiding: a bounding box that is not a superset, and an
   exhaustiveness proof that can silently report success. Both are
   checkable against the existing corpora.
2. **S1 + S2 + S3 as one question.** They are not three findings; they
   are one question — *what is the scalar abstraction actually for* —
   asked from three directions. Answering it determines whether S3's 16
   impls are load-bearing or ceremony.
3. **S8, S9, S10, S11.** Deciding per-lane whether each is a *frontier*
   (keep, gate, say so) or a *deletion* would measurably shrink the
   surface, and several of these lanes are the reason other things are
   awkward. (Not S8, though: that lane was accused of costing `Pcurve`
   its `Copy` and does not — see its record.)
4. **S4.** The two observed drifts (mate names not rewritten by
   `Rebind`; `name_args`' fail-quiet wildcard) are worth checking
   regardless of what is decided about the pattern.

---

# §D. The schedule

**Live rows only.** Completed work is **not** listed here — every finished
unit is recorded at its own finding as a bolded `FIXED by #NNN` lead, which is
the one home for it. A row leaves this section when it lands. What follows is
therefore what is *left*, and its length is the honest measure of that.

That claim held only for work someone had already been assigned. An audit of
all 56 finding IDs against this schedule (2026-08-20) found eleven accepted,
disputed or decided findings with no row anywhere. They were tabulated the same
day and given rows the day after: **Track D** below is what they became, and
the audit's table is retired into it.

**Overhauled 2026-08-19.** The original schedule's Waves 0, 1 and 1b are
complete except where a row appears below; W1a–W1e all landed, H1–H10 all
landed, and five of the six Wave-0 decisions were made in one sitting — D1
among them, ruled 2026-08-19, leaving only D6 open. The
wave numbering is retired with them: it encoded a dependency structure that
has largely been discharged, and what remains is better organised by **who can
take it without colliding** than by how it was originally batched.

Two ordering rules survive unchanged, because both still bind:

1. **Decide before you delete; delete before you polish.** Comment trimming
   (S38) and test-suite combing (S36) come last — both operate on files whose
   fate earlier rows have not settled.
2. **A finding whose steelman said SURVIVES IN PART is scoped by the steelman,
   not by the original finding.** Several shrank materially under scrutiny.

A third has been earned since:

3. **A lane's own residues are rows, not footnotes.** Nine of the rows below
   (H11–H17, and both #649 successors) exist because a fix pass or a review
   found something its own PR could not carry. Recording them as prose inside
   a merged PR body is how they get lost.

And a fourth, from the audit that produced Track D:

4. **A verdict is not a placement.** A finding may leave a review with
   ACCEPTED, DISPUTED or DECIDED and no row only if the verdict is *closed*.
   Everything else owes a track row, a decision row, or a `FIXED by`, written
   in the same PR that records the verdict — because accepting findings in
   batches gives the batch's leader a lane and its siblings a verdict and
   nothing else, which is how all ten of Track D's rows were lost.

---

## Open decisions — Evan only

| # | Decision | Gates |
|---|---|---|
| **D6** | **D5's contract is still untyped at two more doors.** #665 typed `enters_material` and `sector_shape`; a differently-shaped sweep (`grep sense_sign`) reaches the rest. This is a schedule, not a sentence — the question is how far the newtype goes, not whether it was right. | nothing hard; colours the sense-carrying surface |
| **S14** | **What the no-panic principle actually says.** Evan's own reframe, 2026-08-18: *"maybe we need to update that principle to 'no panic on any reachable state, yes panic on things that can only indicate bugs'"*. The steelman split it — the first half is a **clarification** (D9 already says "on any input" and no existing `debug_assert` moves); the second is an **amendment**, because it licenses panics in release, which D9 does not, and on the one such class D9 disposes of it chose typed error or garbage-out. The reframe is also already in the tree unnoticed: PR #447 argued for panicking indexing on the merits and never took it back to D9, while `crates/topo` was ratified the other way. And the honesty defence for `hull.rs:80` fails on reachability — two clamped `KnotVector`s of equal degree and different length, `long_kv.span(k)` handed to a curve built on the short one, indexes out of bounds through the public API with no kernel bug in the trace. Issue **#475** costs out Options A/B/C and misses the cheap third (`kv.span(span.index()) == Some(span)`, O(1), the deleted guard exactly). **Second witness (added by #713, D5).** `topo::instance`'s graft is a public door that can leave a body **tier-1-invalid**: `graft_disjoint_all_keyed` mints an empty destination solid per source solid before transplanting, and its own docs state that a refusal raised mid-transplant leaves `dst` partially written and *spent, never resumable* — an empty solid being `SolidWithoutShells`, a tier-1 error. So a caller that discards the `Err` and keeps the body makes the next Euler operator's `debug_assert` fire from **API misuse, not a kernel bug**, which is precisely the class D9's footnote asserts cannot occur and which S43's proposed sixth state class named and the ratified five do not cover. It is the same question as `Span`'s, one crate over and through a door that already concedes the state in writing — where `Span` needed a somewhat contrived pairing to reach, this is a documented failure mode of a shipping API. #713 recorded the exception at both sites (`euler.rs`, `DESIGN.md`) and proposed no fix.  | This is a **decision, not work** — it was the one row of *Accepted, unscheduled* that had no channel at all, which is why it is here. Nothing in Track D touches it. |
| **S22 row 1** | **ε ambience** — *settled 2026-08-19*: keep the `OnceLock`, add provenance (#659), no threading, no session object, no mixed-ε assemblies. Listed here only because the row's *other* halves are now closed and the finding should not read as open. | — |

**D1 was ruled 2026-08-19** (a `Dual` may not certify, but it may have
`Bounds`; M10/E4 remains the plan) and has landed — see S44's **D1 DECIDED**
entry for the ruling, the impl, what newly admits a dual, and the two residues
it left (`ContentBits for Dual`, issue **#687**; and the `sweep::fillet`
seam's standing lane obligation). Note that it does **not** discharge S3: the lane-trait collapse was
derived against the one-trait world and needs re-deriving against #643's.

---

## Track A — in flight, this orchestrator

Do not take these. Each has a running lane.

| # | Work | Scope |
|---|---|---|
| **A2** | **S58 / #649** — one named iso-rectangle predicate, generalising the torus's level rule to cylinder/cone/sphere. Closes the wrong-certified-volume defect (19% low at `pad = 0.0`). | `geom-brep/src/props/curved.rs`, `topo/src/validate.rs`, new STEP fixtures |
| **A3** | **#678** — the slender partial-revolve cone wedge that meshes silently non-watertight, A/B against `main` first. | `crates/mesh/` |
| **A4** | **#667** — the measured-claim sweep continuation, pattern fixed first. | docs + scattered claim sites |

---

## Track B — COMPLETE (2026-08-20)

**All six rows this track ran are merged**, and the two it did not run were
handed to Track D. A row leaves §D when it lands, so this section is kept
only as the record of what the track was and where its residues went; delete
it once those are picked up.

| row | landed as | finding |
|---|---|---|
| **B1** | **#688** | S7 — the whole-body fillet door retired; `filleted_die.probe.py` now checks orientation mechanically |
| **B4** | **#690** | S5 — the join core, the sector-face producer and the planar sense flip get homes, with two standing gates |
| **B5** | **#689** | S20 + S21 — the façade forwards; Python documents get identities |
| **B6** | **#686** | S4's `units` row / #669 — typed unit views become private indices into `UNITS` |
| **B7** | **#692** | S25 — one tolerance per run, enforced at a certifying seam |
| **B8** | **#697** | S34 — body-wide readback moves to the crate whose types it reads |

**B2 and B3 went to Track D** on 2026-08-20 as D1 and D2; `sweep/` is Track
D's for the duration.

**Residues this track opened, all with owners:**

| # | What |
|---|---|
| **#693** | Three stale descriptions of the retired fillet door in `editor-core/src/eval/` — Track A's files, so the lane stopped. One quotes the deleted predicate verbatim while naming no door and no deleted symbol, so no symbol-scoped sweep can reach it |
| **#694** | The load path stringifies structured kernel refusals, contradicting the bindings' "never strings" contract — seven of ten `DimensionError` arms reach Python through `load` today |
| **#695** | Two shared cores still hosted inside one half (`carve`/`single_solid`, and `conic_plane_crossing_roots`, which decides four `split_*` K predicates from the boolean lane), plus the question of whether the crate root is an architecture or an accumulation |
| **#696** | The rustdoc-JSON check three separate façade guards had been deferring to an unscheduled nightly |
| **#699** | S25's class in `props/` — three signatures and two call sites, twice what the fixing PR first disclosed |
| **S57** | The readback class alive one crate over, and a "one door" guard that cannot see outside its own crate |

**What this track learned that outlived its rows**, since the rows are gone
and these are the transferable part:

- **A regenerated golden cannot be its own evidence.** B1's byte-golden was
  rewritten by the change that needed checking; what establishes the result is
  a mechanical probe of properties the format cannot fake.
- **A reproduction is only evidence about the rows its fixtures generate.**
  B4's K-probe reproduced 26541 rows byte-identically and was **blind to the
  change being made** — its fixtures are all-planar and carry zero arc-rung
  rows. "The probe reproduced" is not "the change was neutral".
- **A guard that pins a seal by quoting the pre-seal spelling tests the shape,
  not the seal.** B6's `compile_fail` rows stayed green with the field made
  public.
- **A skip reads as a pass.** B7's decoupling mutation looked green because a
  tighter generator blew the sample budget and the row skipped.
- **Reachability argued from the authoring doors misses the deserialization
  doors.** B5's central justification failed because every `Deserialize` impl
  re-runs a smart constructor.
- **A guard scoped to one crate cannot enforce a rule stated about a concept**
  (S57).

---

## Track C — ready, unclaimed by either orchestrator

It is listed separately because neither track has capacity for it now, and
because several rows want a decision inside them that the taker should expect
to make and record.

**Gating, stated 2026-08-19, because "nothing here is blocked" was too loose.**
Six of these are edge-free and could start today: **C1**, **C2**, **S30**,
**S31**, **S32**, **S24**. Three unblock when **A1** (#682) lands — **C7**
entirely and **C4's S33** — and their input is now better than "wait for the
report": #682's adversarial pass produced a *compile-verified* table of which
lanes sit behind `CertifiedEnclosure`, which is the premise W2a would otherwise
have been designed against wrongly. **S27** waits on **A2** and **S28's
duplication half** on **A3**, both for file overlap rather than for knowledge.

Two will not unblock by waiting, and should not be read as queued: **C6**'s
rows are gated on other programmes entirely, and **S26** wants a written
proposal rather than a lane. **The binding constraint on the rest is capacity
and the width-1 build mutex, not dependency.**

| # | Work | Why it is here rather than in a track |
|---|---|---|
| **C1** | **H12–H15** — four lanes' own residues: the SSI sweeps' other never-silence doors (no acceptance row in either lane), `sweep_body`'s helix rows with no orientation coverage, #637's two jurisdiction residues, #635's unclassified siblings. | Each is small; together they are a lane. They are the clearest instance of ordering rule 3. |
| **C2** | **H11, H16, H17** — #632's two residues; the STL header not being caller-settable while `StepOptions` carries `product_name`; and S37's rustdoc remainder, ~1115 lines across 130 files. | H17 is large and mechanical; H16 is a small asymmetry with a clear right answer. |
| **C3** | **S27, S29, S30** — `props/quad.rs`'s four independent quadrature engines with a triplicated convergence block; the sizing vocabulary fragmented across five modules with self-admitted magic constants; and ~1,050 lines of instrument in the mesh crate's hot loop. **S29 is NOT blocked on a design conversation — corrected 2026-08-19.** This row previously said its policy question was routed to `docs/TESS-SPLIT-SPEC.md` and PR #568. #684's review checked: both are scoped **entirely to the NURBS per-cell schedule** (`nurbs_cert`'s `grid_steps`, certified cells, the first fundamental form — TESS-SPLIT-SPEC's D-1 replaces the AM-GM grouping, with `leaf_a f2` as its poster child). **Nothing in either covers analytic-chart sizing**, so `curved::grid_steps` has no venue at all — and #684 has since added a sixth rule to it. S29's own lesson applies to that: *N well-defended deviations read as N decisions when they are one undecided question.* S27 touches `props/`, so it must follow **A2**; S29 and S30 are edge-free. |
| **C4** | **S31, S32, S33** — the `geom-curves`/`geom-surfaces` split that buys nothing; `Surface`'s one-partial-per-call API, which is what created the shadow surface enum in SSI; and neither geometry enum being able to lift itself to another scalar. | **S33 is coloured by D1**: several of its ~14 hand-written ladders exist only to reach `Dual`, and what `Bounds for Dual` changes there is written in S44's **D1 DECIDED** block. |
| **C5** | **S24, S26, S28's duplication half** — the assembly gate whose success arm is documented unreachable; the certified area enclosure that is never metered against anything (`area.width()` appears nowhere in the file); and the three tessellation lanes that remain three pipelines now that #648/#674 have settled their ordering and column questions. | S26 was explicitly deferred in writing by #472 — *"metering against `area.lo()` … deserves its own proposal with re-measured floors"* — so it is a proposal, not a patch. S28's duplication half must follow **A3**. |
| **C6** | **W2f remainder / S4** — `ProgramStep`/`WireStep`, `SegTag`, and the "no usable value" core. | Each is blocked on something real: the first behind OnArc + RESPELL-TABLE and crossing the same files, the second needs the workspace's first proc-macro crate, the third by a persisted format. |
| **C7** | **W2a / S3 and W2b / S1+S2** — the lane-trait collapse, and `RingInterval` versus an always-on `Interval`. | **The S3 half no longer waits — D1 is ruled, and its report is S44's D1 DECIDED block.** The steelman's compiled collapse for S3 **predates #643's `Bounds`/`CertifiedEnclosure` split** and must be re-derived against the two-trait world; read *"What this does NOT settle"* first, in particular its per-lane correction — deleting a lane trait leaves **three of the four** seams still uninstantiable at a dual, and only `chart_region_overlap` would become instantiable. W2b's blast radius is 535 refs in 15 files with five carrying 60%. **Two rows joined this one on 2026-08-20**, both from the unscheduled audit: **S44's open residue** — whether the four lane traits survive and whether D9's four bit-identity assertions may be re-expressed, which is what S44 means by *"open for the part that matters"* now that its priced half (D1) is ruled — and **S55**, `Enclosure` as a live trait with no consumer, which Evan deferred *pending the `Bounds` narrow-vs-broad split* and which is therefore this row's, not a lane of its own. Whoever takes C7 absorbs both. |

---

## Track D — the unscheduled table, executed; plus B2 and B3

**Constituted 2026-08-20.** It exists because *Accepted, unscheduled* had ten
rows and no owner, and it took **B2 and B3** on the same day when Track B
handed them off.

**This track runs beside the model A/B experiment, not inside it.** No dispatch
here is an A/B row, no result of it is comparable with one, and nothing in it
edits `docs/MODEL-AB-LOG.md`. Sampling resumes when the A/B does; until then
these units are simply implemented.

**D5 landed as #713** (S15's prose-held invariants — its row is retired from
the table below). Two of its rows had already closed under #635 and one under
#688; one is moot post-#688; and three left with placements rather than fixes,
per ordering rule 4 — `emit_fillet`'s tie propagation as issue **#708**, the
pcurve-staleness convention as **D13**, and the fuzz lane's eager candidate
enumeration (which D5's own row is what made expensive) as **D14**. Its
mutation-path correction also produced a **second witness for S14**, filed in
*Open decisions* rather than settled here.

**Reviews are style-only** (`docs/prompts/reviewer-style-lane.md`) except at
the rows marked ADVERSARIAL — D2, D8 and D11 still live, with D1
(**retired, #710**), D9 (**retired, #712**) and D15 (**retired, #718**)
landed — and D5's `seqgen` half, landed with #713.
Those are where a wrong answer is reachable; everywhere else the risk is
that the fix is ugly or incomplete, which is the style lane's question. The style review carries two questions beyond its
standing brief: whether the finding's *original* stylistic problem is now
completely gone, and whether the way it was closed is the best one available —
not merely a way that compiles.

**Each unit records its own completion, in its own PR**: the bolded
`FIXED by #NNN` lead at the finding, with the original problem statement
removed (version control keeps it). Every Track D PR therefore edits this file
and they conflict with each other by construction. Merge one at a time.

### What the audit got wrong about three of its own rows

**U2, U4 and U6 were recorded as needing a frontier/deletion *sort* before they
could be one lane. The sort already exists.** The S8/S9/S10 steelmen and the
row-by-row postmortems under S15 and S18 each did exactly that work on
2026-08-18, and each named a disposition per row. What is left at those three
findings is execution against a sort that has already landed, which is why they
appear below as ordinary rows rather than as decisions.

**And U9's lane has already run.** S51 was raised 2026-08-18 and **verified by
#636** — the row asking for "the lane it says it is worth" was asking for work
that had already been done and recorded at the finding. It is closed here, not
scheduled.

### Landed

**D1 — FIXED by #710.** **Ten** helpers across **twelve** sites: the row's
nine names, of which one (`arc_apex`) had a third copy the name-based list
could not reach (`revolve/axis.rs`'s `apex_of`) and one (`face_surface_key`)
had a hand-inlined copy in `extrude.rs`, plus a helper the row did not list
at all (`SweptSeg::sketch_segment`). All ten now live in
`crates/sweep/src/swept.rs`; the `partial`/`full` chain duplication, which
turned out to have a third copy in `partial`'s own hole ring, lives in
`crates/sweep/src/revolve/chain.rs`. Every retraction on the row's do-not
list was re-checked and held. The K-telemetry premise held too, and the
conclusion was verified rather than assumed — the crate's string-literal
multiset is byte-identical across the change, so no K row moved. One exact
closed-form guard was added, at the `kemr`-ring path whose only acceptance
was a sign test. See S6 for the full record, including the one behavioural
delta (a pure-statement reordering), the funnel copy handed to D2, and the
three-member bypass class this unit half-fixed.

**This unblocks D2**, whose only gate was D1's hold on the same crate. Nothing
else in the table is gated on D1. D1 also **hands D2 one extra item**: the
third copy of the classification funnel, `sweep/src/fillet/mod.rs:76`,
identical to `swept.rs`'s down to the parameter names — it is in D2's file
set, and D1 declined to widen into it.

**D15 — FIXED by #718.** The harness runs again, and the verdict is that the
K-REPORT provenance sentence was two separate untruths, not one.

**The panic: a missed migration, dated.** `#101` (`548c9618`, 2026-07-25,
*profile: declared-tangency discipline*) introduced
`ProfileLoop::tangent_joints`, `judge_joints` and
`ProfileError::UndeclaredTangency`, and from that commit `ProfileLoop::new`
documents itself as producing *no* declared-tangent joints. It touched eight
files, **none under `crates/sweep/`**; its companion `0cda5f08` migrated
editor-core's corpus. `k_report.rs`'s `rounded_prism` was missed, and its
fixture is byte-identical from before `548c9618` to today — so the harness
broke **four days after** the M2 CSVs were cut and stayed broken 26 days.
Of the row's three candidate causes it is the second (*the validator changed
under the fixture*), but with no judgement to escalate: the refusal is
**correct** — every joint of a rounded rectangle is a fillet arc meeting an
edge at first-order contact, all eight confirmed tangent by the kernel — and
the rule is ratified. The fix is #101's own migration path applied late:
`.with_tangent_joints((0..8).collect())`. **No coordinate, bulge, radius or
tolerance moved**, and validation verifies each declaration rather than
trusting it.

**A second break the row did not know about.** The command K-REPORT documents,
`cargo test -p sweep --test k_report`, has named no target since `81bf6f86`
(2026-08-16) folded `sweep`'s 60 test binaries into one `tests/all.rs`. It
fails before compiling. Selection is by `k_report::` module prefix now.

**Why nothing noticed, which is the part worth keeping.** `dump_k_samples` is
`#[ignore]`d *and* **no CI lane compiles `sweep --features probe` at all** —
`ci.yml`'s only probe invocations are `scripts/k_probe_sweep.sh`'s, which build
`editor-core` and `demos/tour`. The file is invisible to CI to the point that a
type error in it would have gone unnoticed the same way. Cost was never the
reason: the harness is **0.05 s** of test time per ε row. Closing that gap needs
a `.github/workflows/` edit, outside this lane's file set, so it is **placed as
D17** rather than left as a recommendation — §D's fourth ordering rule is *a
verdict is not a placement*, and a verdict with a named mechanism and no owner
is the exact thing this track was constituted to stop happening.

**The provenance verdict, now written into K-REPORT.** The 2026-07-21
byte-reproduction was real *against the tree of that day* and was never a
standing property. A fresh cut of the same ten shapes on main's tip records
**16 824 samples over 105 predicate names** against the committed **13 282 over
63** — 42 names added (the `pcurve_*` chart/loop/trim family, the `tangent_*`
family, `props_rim_*`/`props_meridian_*`, `bool_ring_run_winding`), none lost.
**The committed CSVs are left exactly as cut**; re-cutting is the runbook's call
and the orchestrator's. The conclusions survive: the fresh sweep is ε-stable in
exactly the reported sense (shape/predicate/outcome columns byte-identical
across all three ε rows) and lands **0 in (ε, Kε), 0 within a decade of Kε, 0
indeterminate, 0 invalid**, definite-side minimum |m| = **1.0e-2 m**, three
decades clear of the M7 lint floor. The band is still empty at 1.27× the sample
count.

**The gate was never at risk, and this row overstated its own consequence.**
D15 was written from #710's reviewer's finding and carried its implication too
far: that K-REPORT's provenance being unreproducible put the **k-lint gate** at
risk. It did not. `tools/k-lint` gates on the M4/M5/M7 CSVs, which
`scripts/k_probe_sweep.sh` produces over `editor-core` + `demos/tour` and which
run on every building merge. `k_report.rs` is the **M2-era instrument only** —
not, as the row said, *"the instrument that dumps `docs/k-report-data/`'s
CSVs"*. That correction is recorded here in those terms deliberately: the row
is the thing the next reader inherits, and it should not hand them the
overstatement.

**The shape, for §C10's file.** #101 established an invariant and swept the
corpora it knew about; `k_report.rs` was not one of them, and no import
carried the invariant to it. That is C10's rule exactly — *cross-lane
invariants do not propagate; only imports do* — with the aggravation that the
sibling here was not merely unswept but **uncompiled**, so the sweep could not
have failed loudly even if it had been attempted.

| # | Work | Was | Scope | Review | Gated on |
|---|---|---|---|---|---|
| **D2** | **B3 / S19 — the fillet half of the error catch-alls.** D2's addendum is ratified, so these are row 4 (`unreachable!`) and the rename to `Unsupported*` is owed. **The count has moved: 102 construction sites on today's main, not 146** — 97 in `surgery.rs` through one closure, 5 in `build.rs` through two more — because B1's retirement took the rest with the whole-body door. Scope still excludes `MissingEntity` (mesh — Track A) and `SplitJoinError::Corrupt` (splitting — B4/#690). | B3 | `sweep/src/fillet/` | **ADVERSARIAL** — converting a refusal into `unreachable!` in a kernel whose D9 rule is *never a panic* is only sound if "cannot fail on a valid body" is **proven** per site rather than inherited from the closure's name. | **D1** (same crate) |
| **D7** | **U1 / D4 — the three decided deletions.** Decided by Evan 2026-08-19 and unexecuted. Each row owes a provenance note next to the thread that produced it (`PairSolve` → **#611**; the two fillet helpers → **#319**/**#554**; `Mat2`/`Affine2` → the deleting PR body, cross-referenced from **#614**), and the deleting PR must cite the **commit SHA** the code is recoverable from. `trimline_description`'s doc is the only place D7's prefer-intrinsic obligation is *named*: that sentence migrates, it does not die. | U1 | `geom-core/src/linalg/{mat,affine}.rs`, `editor-core/src/mate{.rs,/solve.rs}`, `sweep/src/fillet/{blend,battery}.rs` | style | **split by row.** `Mat2`/`Affine2` is free now. `PairSolve` waits on **#702**, which is editing `mate.rs`, `mate/solve.rs` and the `lib.rs` re-export block it lives in. The fillet helpers wait on **D2**. Evan placed the whole row *"back of the queue, but ahead of W3b"*, and its rationale — noise to lanes reading the same files — is what these two gates discharge. |
| **D8** | **U4's remainder — the knot-vector queries.** `KnotVector` offers `multiplicity_of(u)`, which requires you to already know `u`; every consumer that needs *the list* of distinct interior knots hand-writes the same scan, four times (`compose.rs:274`, `algebra.rs:563`, `geom-curves/fit.rs:378`, `sweep/skin.rs:370`). Beside it, knot insertion exists twice in one module, one of them re-deriving the span with a linear scan where `find_span`'s binary search is one module away. The scan's own lesson: *a data structure whose API was frozen one PR before its first consumer is the tell.* | U4 (rows) | `geom-core/src/spline/{compose,algebra}.rs`, `geom-curves/src/fit.rs`, `sweep/src/skin.rs` | **ADVERSARIAL** — it adds to a certified type's API and replaces a linear scan with a binary search inside knot arithmetic, where an off-by-one is a wrong curve rather than a compile error. | nothing (but it edits `sweep/src/skin.rs`, so sequence it against D1/D2 within this track) |
| **D10** | **S15's ray-schedule row — the close D9 could not reach.** `boolean/solid_contain.rs:76` re-declares `splitting/containment.rs:102`'s 16-entry 3-D table verbatim, justified by *"to keep the module boundaries thin"*, with determinism depending on byte-identity and nothing checking. Byte-diffed entry for entry by D9's reviewer and confirmed identical. Drop the private const, import `containment::SCHEDULE`, raise its `pub(super)` to `pub(crate)` — `boolean` is not a descendant of `splitting`, so `pub(crate)` is the minimum that reaches, and `splitting/order.rs:77` already imports the same const and is unaffected. One table means the row needs no guard. | S15 (row) | `topo/src/boolean/solid_contain.rs`, `topo/src/splitting/containment.rs` | style | **#712** (D9), which is editing `containment.rs` |
| **D11** | **S17's drift class where it bites hardest: `bool_join_nearest`.** `topo/src/boolean/join.rs:564,600,804,818` decides two different questions under one K name — `Margin::of(dist)` (*"is this chord length zero?"*) and `Margin::of(dist - bd)` (*"is this candidate nearer?"*). A distance and a difference of distances, pooled into one row across four sites in one crate: the same drift D9 closed in `point_in_loop`, worse by site count. D9 closed the class where S17 pointed and nowhere else, which is what makes this a row rather than a residue. Next candidates behind it, from the same sweep: `bool_join_facing` (4 sites), `bool_point_in_solid_plane` (3), `bool_dir_same` (3) — cost each before taking them. | S17 (class) | `topo/src/boolean/join.rs` | **ADVERSARIAL** — it splits a shipped K row into two, and unlike D9's split the two questions here are decided at *different* sites rather than three lines apart, so which site gets which name is a judgement the diff must argue rather than inherit. | **#712** (D9) for the convention precedent, not for files |
| **D12** | **`chart_region.rs:1363`'s self-declared verbatim derivation.** *"the `split_section_area` derivation verbatim, chart-space edition"* — a disclosed copy sitting one screen from the code D9 de-duplicated, in a file D9 edited, and D9's sweep pattern could not match it. **A row, not a verdict**: it may be dimension-forced exactly as the two ray schedules are (`split_section_area` is 3-D, this is chart space), in which case the deliverable is the negative result and a doc line, not a shared home. Establish which before writing any code. The standing `rg -n 'verbatim|re-derived|ported from|mirror of'` is the pattern that finds this class; D9's blind-spot list did not name it. | new (D9's sweep residue) | `topo/src/chart_region.rs`, `topo/src/splitting/join.rs` | style | nothing |
| **D13** | **S15's pcurve-staleness row, which is still open.** `pcurves.rs:124`: *"an op that mutates an already-minted body must either clear the map or re-mint before returning, and **should say which in its own docs**"* — a convention, with *"The lists above are a survey, not an enforced invariant"* four lines above it, and nothing that notices when a new op joins the wrong bucket. **What D5 verified before placing this**: #635 corrected the one entry the steelman caught (`merge_coplanar_faces` had started re-minting and the index had not moved), so the survey is *accurate today* — the row is that nothing keeps it accurate. The shape D5 used for its sibling row is available and cheap: a source-walking test over the three buckets, the way `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1` now covers the mutation surface. | S15 (row 1) | `topo/src/pcurves.rs` and the test's home | style | **discharged — #707** (D4) landed the `pcurves.rs` edits this must not conflict with |
| **D14** | **`seqgen`'s candidate enumeration is eager.** `choose_op` builds every candidate `Vec` on every call — including rows whose weight is zero because the body has stopped growing — and then discards all but one. D5's `split_edge` row is what makes that cost visible rather than what causes it: `split_edge_candidates` runs a full `EdgeCurve::recertify` plus an O(V) separation scan **per edge, per step** (~14 re-certifications and ~200 metered decisions at `GROW_CAP`), which is where its measured +46% went. `memories/test-suite-cost.md` is categorical that an ungated fuzzer is a defect in the fuzzer. The fix is not to drop the gates — they are what keep the lane honest — but to skip zero-weight rows and to enumerate lazily. | S15 (`seqgen` half) | `topo/src/seqgen.rs` | style, but **measure before and after**: the row exists because a number was measured, and it closes on a number, not on a shape | nothing |
| **D16** | **W2c — the D2 addendum, executed in `crates/topo`.** S43's verdict (`:4517`) ratified the taxonomy on 2026-08-19 and named W2c as what remains: the ~60 silent `if let Some(...)` discards in `euler.rs`/`euler_ring.rs`/`euler_kill.rs` are the superseded idiom 4, and **silent discard is never an answer**. Each site sorts into row 4 (`unreachable!`, observable in a branch) or row 5 (`debug_assert`, detectable only by re-derivation) — the split is **re-derivation, not cost**. This is the topo half of the same addendum **D2** is applying in `sweep/src/fillet/`; the two must not diverge on how row 4 is spelled, so whoever takes the second one reads the first one's PR. Retiring the discards also changes what `release_corruption.rs`'s garbage-out row means — see S12. **The row existed only in S43's prose (`:4532`, "now unblocked and unstarted") until #706 placed it**; that miss is §D's fourth ordering rule failing on the section that states it. | S43 / Wave 0 D2 | `topo/src/euler{,_ring,_kill}.rs` | **ADVERSARIAL** — it converts ~60 silent no-ops into panics or debug asserts inside the mutation phase of a kernel whose D9 headline is *never a panic on any input*, so every row 4 needs its not-input-reachable argument made per site, not inherited. | nothing (the addendum opened the `unreachable` lint in both manifests) |
| **D17** | **Nothing in CI compiles `sweep --features probe`, so that whole surface is invisible — not merely untested, *unbuilt*.** Raised by D15 (#718) as the reason its break went unnoticed for 26 days: `crates/sweep/tests/k_report.rs` is `#![cfg(feature = "probe")]` and `#[ignore]`d, and `ci.yml`'s only `--features probe` invocations are `scripts/k_probe_sweep.sh`'s, which build `editor-core` and `demos/tour`. The default clippy job skips `--all-features` deliberately and the `--all-features` doc pass does not build test targets, so **a type error in that file would red nothing**. Two candidate mechanisms, both named by D15 and neither chosen: (a) a `cargo check -p sweep --features probe --all-targets` step, or (b) fold the M2 corpus into `k_probe_sweep.sh` so the M2-era instrument runs beside the M4/M5/M7 one. **Cost, measured (this container, NOT a hosted runner — quote it as a ratio, not a hosted number):** the harness itself is **0.05 s** per ε row; (a) cold is **12.0 s** over 36 crates; (b) needs codegen + link, **35.2 s** cold. The honest counterweight is that `probe` is a `Real` instantiation, so it monomorphizes every generic-over-`Real` body — the *build* is the bill here, exactly as it is for `corrupt input (release profile)`, and it is not free even though the test is. **Read #706's job before designing this one** (`ci.yml:731`): same shape — a surface CI never compiled — and it shows how narrow such a job should be (one crate, its own cache key, filter-gated) plus the trap that matters most here, that **a name filter matching nothing exits 0**, which is precisely the silence an `--ignored` module-prefix selection would reintroduce. | raised by D15 (#718) | `.github/workflows/ci.yml`, and `scripts/k_probe_sweep.sh` if (b) | style | nothing |

**No row number is reserved any more.** D15 (D1's `k_report.rs` harness) landed
with #710, D16 (D6's W2c discards) with #706, and **D17** (the unbuilt
`sweep --features probe` surface) with #718 — assigned centrally on the spot
rather than left in that PR's prose, per the rule below. Row
numbers are assigned centrally because several lanes mint rows in parallel and
three collided once already: a lane that needs a row takes the next number the
orchestrator has not assigned, never the next gap it can see.

**D9 is retired — done as #712** (`topo/src/ray_parity.rs`, S17). Its
ADVERSARIAL gate came off cleanly, and the answer is worth carrying,
split: the K convention **did** charge — the four-field `ParityRows`
exists for no other reason, and a parameterised name is invisible to
`K-REPORT.md`'s `decide("` inventory grep, which the report's method
now covers explicitly — but it **did not forbid sharing**. Zero rows
removed, two added, no margin moved, k-lint roster-independent, no
re-baseline owed or taken. The unit also corrected two of its own
premises: S15's ray-schedule row is a different pair (now **D10**), and
the fix is a half-fix on the drift *class* S17 named (now **D11**).
Its sweep residue is **D12**.

**D6 is retired — done as #706** (`ci.yml`, `ci-filter.py`, `ci-local.sh`,
the two corrupt-input suites). The unit's own finding turned out to be two
findings: the release-profile run the suites instructed and CI never made, and
— found only on review — that S12's residue was never an open question at all.
The **D2 addendum** settled it on 2026-08-19 and what it left is execution,
which is **W2c**, appearing exactly once in this file (S43's prose) with no row
anywhere. That is §D's fourth ordering rule failing on the section that states
it, and it is why W2c is **D16** below.

**Left open by D4, needing an owner outside this track: #250 owes a row.**
The mint-side wiring of the fitted general-circle route (the
oblique-trihedron octant, S8 consumer (a)) is named as an open frontier in
ratified `DESIGN.md` and appears in **no** milestone plan and in **no** row
or comment of **#250**, the carried-items register Evan asked to be total
— re-verified 2026-08-20. #707 did not edit the issue (out of its scope);
the obligation's only in-tree home is now the claim site in
`topo::mint_pcurves`, which says so. Adding the register row is a one-line
job for whoever next touches #250.

**Also left by D4, so it is not carried only in a PR body:** the steelman's
*"three `.clone()` sites"* is a count of **explicit `.clone()` calls naming a
`Pcurve` or a `PcurveCache`** — three in `topo/src/pcurves.rs`, and a fourth
the steelman missed at `topo/src/boolean/combine.rs:343`. It is **not** a
count of cache copies. `Body` derives `Clone` (`topo/src/body.rs:131`) and
the boolean clones whole operands twice (`boolean/mod.rs:1265-1266`,
`:1323-1324`), each copying every stored cache in the body. The real price
of `Pcurve`'s non-`Copy`ness is therefore a body-sized quantity that no
grep finds, and anyone pricing it should start from that rather than from
four call sites. All four are correct as written; this is a count
correction, not work, and `boolean/` is another track's file set.

**Not taken, and why.** S18's `step-export/volume.rs` row stays out of Track D: its
immediate cause is that `topo::props` exposes only body-scoped
`mass_properties` and the exporter needs *per-shell* volume, so closing it means
a new door in `props/` — which is **A2**'s file set. It goes to C3 with the
other `props/` work.

---

## Last, deliberately

| # | Item | Why last |
|---|---|---|
| **L1** | **S36** — comb-and-rename, **per suite**, never a rename pass. | A PR-numbered name currently *carries signal*: it marks a suite not yet combed. Renaming first converts a visible backlog into an invisible one. Needs an owner and a slot, not just permission — the 2026-08-13 retirement licence has produced zero deletions. |
| **L2** | **S38** — comment trimming. | Must follow every deletion above; trimming comments on code about to be deleted is pure waste. Note the pressure runs the other way too: three fix passes this week added prose because a finding demanded a claim-site reason that did not exist. |
| **L3** | Remaining **S35** roll-up rows. | Lowest value density; several will be resolved incidentally. |

---

## Accepted, unscheduled — RETIRED 2026-08-20, into Track D

**The table is gone because all ten of its rows now have somewhere to be picked
up from**, which is the only condition under which it was allowed to shrink.
Where each went:

| Was | Now |
|---|---|
| **U1** — S11/D4's three decided deletions | **D7**, split by row: `Mat2`/`Affine2` free, `PairSolve` behind #702, the fillet helpers behind D2 |
| **U2** — S8, S9, S10 | **D4 — DONE, #707.** All three sorted to *keep*; the prose the sort contradicts is truthed at each finding |
| **U3** — S17's ray-parity twins | **D9** — done as **#712**, which spawned three rows: **D10** (the S15 ray-schedule row, a different pair), **D11** (`bool_join_nearest`, the drift class D9 closed only at S17's anchor) and **D12** (its sweep residue) |
| **U4** — S18's duplicated derivations | **D3** (the negative-zero flush) — **landed as #704**, row retired — and **D8** (the knot-vector queries); the `step-export/volume.rs` row goes to **C3**, because closing it needs a per-shell door in `props/` |
| **U5** — S12's Euler atomicity | Executable residue **fixed by #706** (the release-profile run the suite instructed and `ci.yml` never did). The rest is **not** an open question: the **D2 addendum** settled it on 2026-08-19 and the execution is **W2c**, placed by #706 as **D16** |
| **U6** — S15's prose-held invariants | **D5**, landed as **#713**; the three rows it could not close carry placements — **#708** (tie propagation), **D13** (the pcurve convention), **D14** (the fuzz lane's eager enumeration) |
| **U7** — S14's proposed reframe | ***Open decisions — Evan only***. It was the one row here that was a decision rather than work, and the one with no channel at all. |
| **U8** — S44's open residue | **C7**, with the rest of the lane-trait question |
| **U9** — S51's loft-`v` coverage | **Closed.** The lane it was waiting for had already run: #636 verified it the day after it was raised. |
| **U10** — S55's consumerless `Enclosure` | **C7**, which is the `Bounds` split it was deferred pending |

**What the audit was for still holds, and it caught a real thing twice.** Eleven
findings had a verdict and no row because they were accepted in *batches* and
only the batch's leader got a lane. Two of the ten also turned out to be
mis-recorded in the other direction — U9 was already done, and U2/U4/U6 were
recorded as awaiting a sort that had landed the same week. A register that is
never audited against its own findings drifts **both** ways.

**The rule this section leaves behind:** a finding leaves a verdict and no row
only if the verdict is *closed*. Anything else — accepted, disputed, decided —
owes a track row, a decision row, or a `FIXED by`, in the PR that records the
verdict and not later.

---

## The edges that remain

```
A1 (Bounds for Dual) ──► C7  (S3 lane traits, S1/S2 scalars, S44's residue, S55)
                     └─► C4  (S33's dual ladders)
A2 (iso-rectangle)   ──► C3  (S27, and S18's step-export row — same props/ files)
A3 (#678)            ──► C5  (S28's duplication half)
D1 (#710, landed) ─────► D2 (S19 fillet errors) ──► D7's fillet-helper row
#702 (assembly door) ──► D7's PairSolve row
#690 (B4, splitting) ──► D9 (S17's ray-parity twins, #712) ──► D10 (S15's schedule)
                                                          └─► D11 (bool_join_nearest)
all deletions        ──────────────► L2 (S38 comments)
                                 └─► L1 (S36 suites, per suite)
```

**Track B is now edge-free in full** — B1 landed, and B2/B3, the only chain it
had, are Track D's D1/D2.

**Track D's own edges are all inside `sweep/`, plus one on another track's open
PR.** D8, D10, D11, D12, D13, D14, D16 and D17 are edge-free and unstarted (D1
landed as #710, D3 as #704, D4 as #707 — which also discharges D13's gate —
D5 as #713, D6 as #706, D9 as #712, D15 as #718). **D17 is the only row in
the track whose file set is `.github/workflows/`**, so it collides with no
kernel lane and can run at any time. D8 edits `sweep/src/skin.rs`, so it
sequences against **D2** alone within the track: D1 has landed and left
`skin.rs` untouched. The one remaining external edge is D7's `PairSolve` row,
behind **#702**.

---

# §C. Process observations

Evan's standing request (2026-08-18): *"i wonder how they happened; it'd be
cool to get a 1-3 sentence postmortem on each covering the rationale and
whether (per the associated pr description, A/B log, and/or orchestrator's log)
the reviewer flagged it as an issue. i think this could be really useful for
improving process."*

Each finding carries a **Postmortem** line where one has been done. This
section collects only the observations that generalise. **It is being filled
in as the postmortem passes land — treat it as incomplete.**

## C1. The review protocol is claims-driven, not surface-driven

This is the mechanism behind most of the façade findings, and the evidence cuts
against the obvious reading. Reviewers are handed *"explicit claims to
falsify"* (`memories/orchestration-model.md:152`) and they falsify them
**behaviourally and well** — LIB-U1's reviewer compiled `use topo as _;` to
kill a false documentation claim. The effort is *not* aimed at prose over
behaviour.

It is aimed at **asserted** things over **unasserted** ones. A code-free
module, a 449-line accumulated header, and a duplicate type name across a
façade assert nothing, so nothing points a reviewer at them.

## C2. Disclosure functions as immunity, and the scoreboard rewards it

The A/B rubric's headline column is *"silent devs"*. PR #364 scored **"0 silent
(5 deviations reported)"** — and one of those five reported deviations was the
constant `DocumentId` that makes two Python-authored documents un-coexistable
in a workspace. Writing a hole into the PR body converts it into a *positive*
metric. There is no counter-metric asking whether a disclosed deviation was
**acceptable**, only whether it was disclosed.

## C3. Deferrals must land in a register that executes

The repo has exactly one self-enforcing register — `docs/guide/north-star-audit.md`,
whose test fails as doors land — and several prose ones. Every deferral behind
a finding here went into prose: a spec sentence, a constructor comment, or a
residual register that **had closed the day before the PR merged**. Prose
registers have no way to notice. Compare S15's row sort: **zero of its nine
rows had a tracked issue** when this was written, even though the repo
demonstrably knows how to do better (issue #214 for a census,
`attach.rs:119`'s KNOWN HAZARD block for a named-and-pinned gap). D5 closed
that gap the way this section asks — the one row it could not fix left as
**#708**, pointed at from a KNOWN HAZARD block, rather than as prose in a
merged PR body.

## C4. Nothing in the process reads a whole file, a whole namespace, or a whole crate

Specs are per-unit, diffs are per-unit, reviews are per-unit, log rows are
per-unit. The ~1170-lines-of-narrative-over-~180-lines-of-code ratio, the three
`DimensionError`s, and the `lib.rs`/`workspace.rs` contradiction are all
properties that exist **only above the unit** — which is precisely the altitude
at which this repo had no reviewer before this scan.

## C5. Documentation is a growth sink under review pressure

#232's completeness MINOR could only be discharged by making the essay
*longer* (nine more types plus a paragraph on why each was missed). Note the
timing against Evan's own later standing brief line — *"comments state the
INVARIANT, not the history: no retired-type archaeology, no unit tags"*
(2026-08-08) — which is exactly what `closure.rs:135` and `select.rs`'s
unit-tagged sections violate, having been written days before it and never
re-swept.

## C6. Some of these were ratified before they were written

Three of the seven findings in this batch trace to a **design document or spec
clause**, not to a lapse: S23's data-switched dual role
(`CURVED-DESIGN.md:796`, *"one structure, two duties"*), S26's
area-as-denominator (PR #192 deviation 1), and S30's counterfactual third pass
(TESS-SPAN D-4). **The design conversation is where these entered, and it is
the only place they could have been caught.** No amount of implementation
review would have found them, because the implementation was faithful.

## C7. The deviation ledger works as an amnesty, not an alarm

Independently confirmed by two postmortem passes over different scopes. PR
bodies disclose the shape with near-perfect honesty — *"deviation report
pending"*, *"no gate consumes it"*, *"`SAFE_ASPECT = 5` … above the derived
line"*, *"`MAX_GRID_RETRIES` 4 -> 6"* — and **disclosure closes the item**.
Nothing escalates when the same deviation appears in a third PR. Cf. **C2**:
disclosure also scores as a *positive* on the A/B rubric's "silent devs"
column.

## C8. The acceptance rows for degenerate modes are written so they cannot fail

Both correctness-shaped findings in this batch share one failure, and it is
mechanical enough to be a rule.

- S23's floor row is named
  `..._refuses_typed_even_though_branches_were_found` — **the premise excludes
  the failing mode.**
- S26's area row asserts `area_pad > 0.0` plus containment — **both monotone
  in the wrong direction**; the row gets *easier* as the enclosure degrades.
  Its neighbour twelve lines up asserts a real tightness bound, so the file
  contains both the right and the wrong shape.

Neither is a weak assertion by accident; each was written to pin the *feature*,
and the degenerate mode was never a row. The candidate rule: **every "never
silent" or "certified enclosure" claim needs a row that goes red when the
guarantee degrades, not merely when it is violated at a chosen fixture.**

## C9. Reviewers are exceptionally strong at soundness and blind to structure

This is the sharpest characterisation the postmortems produced, and it is
evidenced in both directions. The same reviews ran 8000-matrix SVD
differentials, re-derived a meters conversion by hand, found a certificate
excluding true 2π by ~1111 widths, and wrote the strictest floor-refusal probe
in the repo — and produced **zero** findings on: a mode switch on
`is_empty()`, a two-ε signature, a file holding four engines, or three
parallel CDT pipelines.

Structural findings appear only as **side-effects of bug hunts** (#472's
"unguarded at four sites", #313's shared area rule). Nothing in the protocol
asks a structural question directly.

## C10. Cross-lane invariants do not propagate; only imports do

`planar::triangulate_chart`'s header warning, PR #116's pre-scan, and PR
#157's `SelfTouchingTrimLoop` are **three encounters with one hazard**, each
closed inside its own lane — while the lane that predates all three
(`curved.rs`) still carries the ordering the warning describes. A fix that
establishes an invariant needs an explicit sweep of sibling implementations as
part of its **acceptance**, not just its own regression row.

**The sweep was run (2026-08-19).** For `curved.rs` the *ordering* came back
CLEAR — the ordering is inert there because `curved` builds no crossing
bookkeeping for a split to corrupt — but the clearance is not where it ended.
What the sweep actually found was an *unstated premise* doing the work
(`curved`'s UV domain is its own bounding rectangle), which nothing checked;
#648 turned it into a typed refusal
(`TessellateError::UnsupportedCurvedDomain`). Read that as the general shape:
a sibling that survives the sweep survives *for a reason*, and **the reason
is either enforced or it is the next defect** — writing it down is the
minimum, not the deliverable. S28 carries the detail.

**And the reason has to be right.** #648's first pass wrote the premise
down as an *exact* property and cited the mechanism that supposedly
guaranteed it ("assigned once per edge"). Adversarial review executed
the counterexample in an afternoon: the mechanism guarantees the
property only when a side is one edge, and every in-tree fixture and
the whole wild corpus happened to satisfy that, so a green suite proved
nothing. The refusal became a **false refusal** on valid parts (#653).
**#648 then corrected the premise only where it bit** — in
`curved.rs`'s own docs — leaving `walk.rs`'s module header (the home of
the mechanism) still asserting it flatly, three lines from a paragraph
#664 was rewriting, so for a milestone two files in one crate said
opposite things about one sentence. #664's fix pass qualified it in
`walk.rs`, `mesh/lib.rs` and `Chart::rim_v`. *A correction lands where
the defect bit; the claim lives wherever it was written down, and the
two sets are not the same.*
Two things generalise. A premise stated as *exact* is a claim about
float representation, not about geometry, and needs a fixture that is
adversarial to representation — an oblique placement and a subdivided
edge, not another shape. And a sweep that records pass/fail where it
could record **margins** discards the evidence that would have shown
the claim was fragile; #648's payload now carries the margin for the
same reason.

**The sweep the fix itself owed, run in the same PR (2026-08-19,
#653's option 2).** The invariant established there is *"a curved
face's iso side gets ONE constant coordinate, however many edges carry
it"* — and §C10's rule is that such an invariant is swept across
sibling implementations as part of acceptance. Two siblings existed and
both were taken in the same change rather than left for a follow-up:
**meridian columns** (u) were the reported half, and **rim rows** (v)
are the same shape one axis over, so `iso_side_starts` classifies both
and the `Rim` arm shares its row exactly as the `Meridian` arm shares
its column. The rim half has no live reproducer: a `split_edge` keeps
one carrier circle, so both sub-edges compute the same `rim_v` bitwise,
and every DIRTY row in the 1524-sweep came from a *meridian* split. It
is reachable through import all the same, where two co-`v` arcs are two
independently stated `CIRCLE`s. Fixing only the half with a live
reproducer is exactly the failure mode this section is about.

**The rim half is therefore SHIPPED WITHOUT RED-WHEN-REVERTED
EVIDENCE, and that is scheduled, not shrugged at.** Its `Rim` arm is
*executed* on every rim split in the #653 row — the code runs, it just
cannot come out differently, because the two sub-edges of one carrier
read the same centre and radius. The reproducer that would change that
is a STEP file stating two co-`v` `CIRCLE`s independently, and the
machinery to mint one is already in this PR:
`crates/step-import/tests/fixtures/split-iso/generate.py` writes
hand-authored AP214 with per-edge carriers, and its `arc_edge` helper
already emits one `CIRCLE` per edge. The work is a fifth fixture —
split the D-prism's bottom arc `e1` in two, each half with its own
`CIRCLE` entity, obliquely placed — plus a row beside
`a_split_iso_side_meshes_watertight_under_an_oblique_placement`. It is
named in `curved.rs`'s §C15 blind-spot list at the #653 row so a reader
of the code meets it too, not only a reader of this document.

**And the global-floor class came straight back, in the PR that cited
it.** #653's headline row opened with `assert!(checked > 100)` against
an actual 254, so a 60% collapse would have passed — the same shape as
`walked >= 14`, one section up, written by an author who had just read
that paragraph. Now per-fixture: every edge of every fixture must
produce a placed body, and the helper returns its skips instead of
`continue`-ing on them. The lesson is not "remember the rule"; it is
that a *derived* floor (fixture's own edge count) cannot go stale the
way a transcribed number does, and a transcribed number is what both
failures had in common.

The third sibling was checked and **is not one**: `trimmed`'s
pcurve-driven lane never *derives* a constant coordinate from a
midpoint evaluation — it reads the stored pcurve — so it has no
derived-per-edge premise to break. (An `IsoLine` pcurve does carry a
constant, but as stated data: two sub-edges disagreeing there is a
statement about the file, not about the walk.) `planar` has no iso
coordinates at all.

**A second class the same review named, swept in #648.** *"Sweep a list of
bodies, assert a global count"* has the same hole as a global floor:
`curved.rs`'s row asserted `walked >= 14` against 20 actual walks, so the
boolean-cut die pip — the fixture the row exists for — could have dropped out
through either of `curved_walks`'s two silent `continue`s and left the row
green. Fixed to per-fixture participation. The two siblings the reviewer
named, `crates/mesh/tests/review_m2_pr6_walk_shapes.rs` and
`crates/mesh/tests/revolves.rs`, were checked and are **clear**: both assert
inside the loop (per-θ `check_mesh_acceptance` / `signed_volume > 0`), so a
fixture that stopped contributing cannot hide behind its siblings. Neither
carries an accumulated counter.

## C11. Self-disclosed copies are invisible to everyone, and greppable

Every duplication in S18 is **honestly declared in prose at the copy site**:
*"the profile crate's ratified bulge closed forms, **verbatim**"*; *"**verbatim**
`crate::recognize`'s"*; *"the face bound's quotient-rule assembly **one
dimension down**"*; *"the `boolean::boxes::face_box` construction
**re-derived** in the evaluation lane"*. The codebase is candid about all of
them and **nothing in CI, review, or the log ever reads that prose**.

A grep for `verbatim|re-derived|ported from|mirror of` across `crates/*/src`
would surface most of S18 in seconds. This is the cheapest concrete mechanism
any postmortem in this document produced.

## C12. Specs name the *method* to copy but never the *home* to reuse

*"Port `point_in_loop`'s METHOD to 2-D"*; *"the face Hessian … **AND** the
curve-side gate"*; *"built on the MERGED M8-2 template — deliberately NOT
lifting #309's unmerged machinery"*; *"reads `containment.rs` for METHOD only
(**no refactor**)"*. The convention describes the source as a **reference to
imitate**, not a dependency to call.

Two structural amplifiers: **concurrent file-disjoint lanes** are mandated by
the orchestrator for good reasons and leave behind no unify-after-merge
obligation; and the **K-ledger rule** (new predicate names = new rows) makes
the copy the path of least resistance, since parameterizing would disturb the
census.

## C13. Half-fixes read as whole fixes when the finding is narrower than the drift

Three independent instances:

- #152's reviewer forced **one home for `tangent_certificate_lane`** — and the
  same fix pass shipped **two divergent sample schedules**, with a third
  hardcoding the literal `9` two days later.
- The perf scan corrected `face_box`'s stale **NURBS** premise while the
  identical stale premise for **planar-with-conic-rim** sat fifteen lines from
  the text it quoted.
- M4 PR 5's reviewer correctly identified **information loss** (*"catch-all
  launders tier-2 diagnostics → preserve real reasons"*); the remedy was
  `format!`, and `DESIGN.md` D4 ¶2 then **canonised that outcome as the in-repo
  precedent**.

The generalisation: a review finding gets discharged **at the granularity it
was phrased at**. An information requirement is met by stringification; a
one-home requirement is met by unifying the name and not the schedule it
drives.

## C14. Pins guard the invariant as it was reachable *then*

Three of S22's four rows were introduced by a **later** change to code whose
contract an **earlier** reviewer had checked and pinned — and in each case the
pin still passes:

- `survives_eps_row_bitwise_independence` pins *"ε is read once, for pole
  identification"*; #481 added a second structural ε read; the test passes
  because only a **foreign STEP file** produces a nonzero residue. (#664
  removed that second read and corrected the comment — which by then had
  to name a *third* consumer, #648's domain guard, rather than restore
  the original wording. A stale claim does not become true again by
  undoing the change that falsified it.)
- `parallel_schedule_preserves_verdict_logs` pins **thread** confinement;
  ASM-2A broke **re-entrancy**.
- Four process-isolated binaries pin the ε global's init discipline; none
  observes that a *document*'s ε still commits into the same lock.

Nothing re-derives a pin when a new caller arrives. And a stale comment on a
still-passing test reads as **evidence the invariant holds**.

**The weaker case — no pin at all — is #651**, raised by the style review of
#646. The rule it produced is in `docs/prompts/reviewer-style-lane.md` §Q6 (a
measured claim owes a mechanical guard, a scheduled register that
re-measures it, or a written reason at the claim site that it can have
neither); the classification sweep is a comment on #651, and is not
repeated here. #667 continued it over a corrected population (a
provenance-vocabulary pattern restricted to comment text, deduplicated to
the comment block and filtered by a numeral: **197** blocks, 37 of them
claim-bearing, against #663's 146 `measured` lines) and its rows are the second comment on the
same issue. Its finding for THIS clause is C14's own shape one turn
further on: `ci.yml` runs more registers than #663 found, and two of them
gate — but each re-takes a **subset of the columns** of the document it
produced, so "`docs/TESS-BUDGET.md` is re-measured per merge" is itself a
guard described wider than it is read. The register roster and what each
one actually re-takes live in the sweep comment, not here.

---

## C15. A sweep's result is worth nothing without a statement of what its pattern cannot match

**Observed three times in one day**, across three independent wave-1b fix
lanes, each of which reported its sweep as verified and each of which was
blind in exactly the shape it was hunting:

- **#632** scanned for arms beginning `RoleSeg::` at the wildcard's
  indentation, so **every arm wrapped in `Some(…)`, `Ok(…)` or a tuple was
  invisible** — which is the shape of what it missed.
- **#635** used a line-scoped `rg`, so a claim that **wrapped across a line
  break** could not match. Two survivors of the premise it was sweeping sat
  in the file it had just edited, one of them 25 lines above the list it
  fixed.
- **#639** scanned **prefixed** codes (`LIB-*`, `ASM-*`, `Mn`, `PR n`,
  `#nnn`) and so could not see **bare** clause letters (`F5`, `G1`, `C4`,
  `S13`). It therefore shipped S37's own named example — `LIB-DOORS F5` — in
  a live Python `__doc__`, in one of the three crates its body reported at
  zero.

This is C11/C13's mechanism one level down. Those say a class gets fixed at
the reported instance; this says that even a lane *trying* to sweep the class
will under-report by exactly the margin its pattern cannot express, and will
then state the shortfall as a verified negative. In all three cases the
conclusion happened to survive; in all three the method did not.

**Proposed in #666, awaiting Evan's sign-off** — it amends the review
instrument, which is Protocol v5's territory. The rule text lives in
`docs/prompts/reviewer-style-lane.md`; it is not restated here, because two copies of
one rule is the shape this report exists to hunt.

## C16. A prose-hygiene pass can manufacture the defect it exists to remove

`props/quad.rs:42`'s liveness claim — the one row of eleven that #635
classified as a **lost invariant** rather than benign rot — was itself written
by a **previous stale-claims sweep** on 2026-08-05 (`git log -S` puts it in
`e2222617`, whose message names its own "§7 stale-claims sweep tranche"). It
replaced **two honest sentences** with one naming the wrong engine, and missed
a third, inside the same function, that contradicted it for the next two weeks.

The generalisation is not "sweeps are bad". It is that a pass which rewrites
prose to state the present will, wherever its author guesses at liveness
instead of checking it, **launder a guess into an assertion** — and the
resulting sentence is indistinguishable from a verified one to every later
reader. That is the argument for S39's classify-before-you-touch discipline
being permanent rather than a one-off framing of one finding: the question
"benign rot or lost invariant?" forces the check that the 2026-08-05 pass
skipped.

Method note, cheap and reusable: this repository's checkouts are **shallow**,
so `git blame` misattributed that sentence by ten days. `git log -S` is the
instrument for dating a claim.

## C17. "Green when run alone" is not a verification when lanes share a target directory

#639 reported `cargo test` green for three crates and shipped **ten broken
string assertions**. Two causes, and only the second is the author's.

The orchestrator had put six concurrent lanes on one `CARGO_TARGET_DIR`, which
**clobbers across git worktrees**: at least two lanes were served results from
another lane's binary. Confirmed by counting — the same crate reported 156
tests on the shared directory and 155 on a dedicated one, from identical
sources. So the lane's re-check, run to rule out contention, was green for the
wrong reason.

The author's half is that two failures **of the same shape** — a string
assertion on text just rewritten — were read as load flakes rather than as the
first two members of a class. This is C13 in the verification lane rather than
the fix lane.

Two rules follow, both now in force: **one target directory per lane**, and a
run is trusted only when a `Compiling <crate>` line was observed. And the
deeper one: a lane that rewrites text asserted anywhere must run the affected
crates' **tests**, not their builds — `cargo build` cannot see a broken
`assert!(msg.contains(…))`, and every one of the ten was invisible to it.

# §B. Negative results and coverage

**Coverage.** Twenty scopes covering every file in `crates/*/src`
(~336k lines including tests; ~155k lines of `src`). `demos/`,
`tools/k-lint`, `interval-transcendentals/` and `references/` were out
of scope. Test directories were characterized but not read
exhaustively.

**Reported clean, or clean enough not to flag.** Scans were instructed
to report negative results and to omit dimensions where their scope was
sound:

- `crates/test-utils` — straightforwardly earning its separateness; nine
  manifests name it, all as a dev-dependency.
- `crates/quantity` — earns its existence as a leaf shared by three
  crates. Its problem is external (S4's seven unit vocabularies), not
  internal.
- `crates/stl` — nothing beyond S37's milestone-named header constants.
- `geom-core::spline`'s core algebra — the `algebra.rs` plan/apply split
  and `KnotVector`'s validation were flagged only at the edges (S14,
  S18), not in the substance.
- The `Euler` operator preconditions themselves — the scan found the
  *enforcement mechanism* wanting (S12) but no precondition wrong.
- `crates/bvh`'s tree — confirmed to have live production call sites,
  contra the initial suspicion that it was speculative; only the
  crate-name/export mismatch survived (S35).

**Known limits of this scan.**

- Line numbers are as of `4258584` and were not re-verified after the
  scan, unlike PERF-SCAN's §0a pass. **Claims, not line numbers.**
- Findings marked `unsure` are leads, not conclusions. There are four:
  the `lsq` conditioning question (S35), `update.rs`/`refactor.rs`
  convention mismatch (S35), whether `intersect.rs`'s C5 table should
  become a real dispatcher (S35), and `step-import`'s `chart.rs`
  containment soundness (S35).
- No scan executed the code. Every "unreachable" and "no producer"
  claim is from reading plus `rg`, and could be wrong about a path
  reached through a macro, a trait object, or a feature combination not
  considered.
- The scans were asked to be suspicious of long justifications. That is
  a deliberate bias and it will have produced some false positives on
  designs that are simply well-argued.
