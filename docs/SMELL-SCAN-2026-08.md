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
- [§A. Where I would start](#a-where-i-would-start)
- [§D. A schedule for fixes](#d-a-schedule-for-fixes)
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

| Concept | Copies | Anchor |
|---|---|---|
| profile `Step` verbs | `profile::Step` / `ProgramStep` / `WireStep` / `StepArg` / content-key tag table — **5**, across 3 crates | `program.rs:64`, `persist/wire.rs:255`, `profile/src/path/program.rs:190` |
| `RoleSeg` → `SegTag` | kernel enum → editor-core fieldless mirror → `pncad` re-export → a **second** 40-variant py mirror → 40-arm `to_kernel` → 40-arm inverse tripwire → 1316-line `.pyi` | `pncad-py/src/py/select.rs:82` |
| node kinds | ~10 parallel match tables; `rg Node::Fillet` → 24 non-test hits in 10 files | `node.rs:423`, `eval/mod.rs:1325` |
| "which `RoleSeg` args are sub-names" | 4 sites | `resolve/mod.rs:969`, `refactor.rs:540`, `names/select.rs:296`, `eval/mod.rs:2040` |
| "which payloads carry a `StableName`" | 4 lists | `edit.rs:1096`, `node.rs:949`, `refactor.rs:801`, `resolve/mod.rs:911` |
| "node has no usable value" | 5 typed + 1 stringly | `resolve/mod.rs:266`, `resolve/hit.rs:23`, `resolve/vdiff.rs:69`, `appearance.rs:172`, `names/geompred.rs:488` |
| units | 7 spellings for 6 units | `quantity/src/units.rs:47`, `expr.rs:181`, `step-import/src/units.rs:76` |
| Euler vector per op | 3 (prose, unnamed positional 7-tuple, `ep_vector`) + a 4th divergent `Ledger` | `euler.rs:857`, `euler.rs:1972`, `seqgen.rs:105` |

Two of these have **already drifted, observably**:

- `Node::named_nodes` lists `Declare`, `Mate`, `Fillet` and its comment
  states `Rebind` is the repair for all three. `DocEdit::Rebind`'s
  rewrite loop in `apply` matches only `Declare` and `Fillet` (`_ =>
  {}`); `refactor::payload_names` returns names only for those two. A
  mate head is validated at insertion, documented as repairable, and
  silently not rewritten.
- Three of the four `RoleSeg` sites carry comments insisting the match
  is exhaustive "so a future variant must be classified here or the
  compile breaks". `select::name_args` ends in `_ => Vec::new()` — the
  exact fail-quiet wildcard the others forbid — and disagrees on
  `Fragment(SideOf)`.

The tell that these are accretion rather than principle: `BooleanOp` is
**mirrored** from the kernel while `ContactClass` is **imported**, each
documented at length as correct, under the identical constraint (kernel
crates take no serde dependency). The `#[serde(with)]` module written
for the second demonstrates the first was never necessary
(`node.rs:43`, `node.rs:1120`).

**Verdict:** ACCEPTED (Evan, 2026-08-18). "Oh boy, good findings — these
look like they'll be a lot of work to fix but definitely worth it."
Steelman pass commissioned for the constraint map: how much of the
duplication the G1 no-serde-in-kernel rule actually forces, given that
the `ContactClass` `#[serde(with)]` remote derive shows the mirror was
avoidable at least once. Includes verifying the two observed drifts.
**Steelman (2026-08-18): SURVIVES IN PART — row by row, and the diagnosis of
the `BooleanOp`/`ContactClass` tell was wrong in a way that makes the finding
*stronger*.**

*The corrected tell.* `BooleanOp` was **never a serde decision**. PR #81 (M4
PR 1) forbade `editor-core` from depending on `topo` **at all** — that is why
the enum was minted locally. `git log -S` shows the dependency was added the
**same day** (`baec1fd9`, M4 PR 2). The mirror has outlived its reason by four
weeks, and `BooleanOp`'s rustdoc **defends nothing**; the only place it is
characterised as a deliberate pattern is inside `ContactClass`'s doc, written
by a different author three weeks later. Meanwhile `ContactClass` **was**
ruled: `M9-1-SPEC.md:22` — *"never a parallel enum… One enum, defined lowest,
re-exported upward"* — and PR #552 states the collision explicitly and resolves
it with `#[serde(with)]`. That technique **did not exist as a repo idiom** when
`BooleanOp` (2026-07-23) or `WireStep` (2026-08-09) were written; it was
invented 2026-08-16. Ordinary accretion — but the project's own most recent
ratification now points at the collapse.

*Also corrected:* `RoleSeg` is **not** a kernel enum — it lives at
`editor-core/src/names/role.rs:226`. Both ends of that chain are editor-core.
And `DESIGN.md:1914`'s *"(layering enforced by CI grep)"* is **stale**: no
serde grep exists in `ci.yml` or `ci-local.sh`. The only mechanical check is
`profile/tests/seal.rs:87`, covering `profile` only.

| Row | Verdict |
|---|---|
| profile `Step` verbs | **SURVIVES** — `WireStep`/`WireTarget`/`WireArcData` are field-for-field mirrors differing in **nothing**. Only `WireSide`/`WireWinding` wrap kernel-foreign types (two two-variant tags), plus `SketchPlane<f64>` needing `WirePlacement`. The scheduled RESPELL-TABLE unit does **not** reach these. |
| `RoleSeg` → `SegTag` | **SURVIVES IN PART** — three of four links are compile-enforced and the python lane runs in CI. Genuine gaps: the `.pyi`'s 40 members are **unpinned** (`test_stubs.py` parses only top-level names, never class bodies), and the py mirror is **forced by the orphan rule** — not collapsible, only generatable. |
| node kinds (~10 tables) | **DOES NOT SURVIVE as stated** — 10 operations over a 12-variant sum type is the design working. Re-scoped to *wildcard* arms it survives: `node.rs` 9, `eval/mod.rs` 5, `resolve/mod.rs` 4, `edit.rs` 3, `refactor.rs` 3, `persist/check.rs` 2. |
| `RoleSeg` arg sites | **SURVIVES IN PART** — the four answer four genuinely different questions and *should* differ. What survives is exact: three carry "exhaustive on purpose or the compile breaks", the fourth ends `_ => Vec::new()` with no note saying why it is exempt. |
| `StableName` payload lists | **SURVIVES** — see the confirmed drift below. |
| "no usable value" | **SURVIVES IN PART** — the four enums have genuinely different membership and closure (`RunStatus` is serde-persisted), but all four embed the identical triple, and the stringly fifth is a real fail-quiet. |
| units | **DOES NOT SURVIVE as counted** — `parse.rs` uses the shared table; `step-import`'s `UnitKind` is a *different vocabulary* (STEP `SI_UNIT` names). Real duplicates: two-and-a-half, one of them **measured and justified** (PR #291 MAJOR-2: inlining the 32-byte row grew every `Expr` by ~40 bytes). |
| Euler vector | **SURVIVES IN PART** — the 6-vector and the 7 arena deltas are **different quantities**; Δh is not an arena count and cannot be derived from them. What survives: the 7-tuple is **unnamed and positional**, passed at 16 sites across 6 files, in a project whose ratified style is "named, never positional". |

*Drift (a) CONFIRMED, and it contradicts ratified design text.* `Rebind`'s
rewrite loop ends `_ => {}` and never reaches `Node::Mate`'s two `StableName`s.
Two bad outcomes: if a mate head is the only reference, the rebind returns
`RebindNoReferences` — a **loud but false** refusal; if a `Declare` also
references it, the mate is **silently left dangling**. `ASSEMBLY-DESIGN.md:566`
states *"A dangling head (N5) contributes no edge until `Rebind`"* — that
repair path does not exist. A third site is also mate-blind
(`resolve/mod.rs:911`). **No issue is filed.** Note the contrast in the same
file: `refactor::remap_node` **is** wildcard-free and **does** handle `Mate` —
the exhaustive sites stayed in sync; the wildcard sites did not.

*Drift (b) CONFIRMED with a qualification:* the `Fragment(SideOf)` disagreement
is **documented and intentional**. The drift is narrower — the fourth site uses
the exact fail-quiet wildcard its three siblings prohibit, without saying it is
exempt. A future variant carrying sub-names compiles in as "no arguments".

*One confirmation this report did not cite: the hand-synced tag table has
already produced a live measured bug.* `MODEL-AB-LOG.md:782` — *"**MAJOR-1 =
`Step::AtToward`'s memo content-key tag 28 COLLIDED with `ArcContinue`'s
existing 28 — latent memo collision, a hit would serve wrong geometry**"*.
Caught by a reviewer, not a type. S4's failure mode, realised.

*Ranked cheapest-to-hardest to act on:* (1) `BooleanOp` → import +
`serde(with)`; (2) `name_args`' wildcard → exhaustive; (3) the `Mate` arms —
small but a **behaviour** fix; (4) the Euler 7-tuple → named struct
(debug-only); (5) units; (6) `ProgramStep`/`WireStep` — cheap in isolation,
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
sees two populations for one question, and any future tolerance tuning
has to be done twice.

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

*The K cost is measured, not hypothetical* (`m7-eps-1e-6.csv.gz`):

| predicate | samples | min margin |
|---|---|---|
| `bool_sector_arm` | 1880 | 4.11e-2 |
| `split_sector_arm` | 64 | 1.95e-2 |
| `bool_sector_reflex` | 1880 | −0.25 |
| `split_sector_reflex` | 64 | **0** |

One geometric question, two populations at **29:1** — and the 64-sample
tail is the one that actually reaches margin 0.

*The project already treats name/margin bijection as a correctness
property, in one direction only.* `M3-LOG.md:264` records PR #55's
review MINOR-1: two margins sharing one K name **had to be split** by
reviewer instruction. One margin under two names has never been
examined. And the counter-precedent exists in-tree:
`bool_planar_chord_spec` and `chord_spec` deliberately **share** the K
name `split_arc_window`, documented as *"same margins, same predicate
names"*.

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

*Hidden costs:* merging the K names is a schema break in an append-only
dataset (`K-REPORT.md:738` tracks a 233-name census); ~267 tests across
36 files; open issue **#561** notes the Python refusal-tag *values* are
pinned nowhere, so an enum reshape can silently change strings the
Python surface exposes; and the sphere asymmetry forces a per-lane kind
gate — so unification **converts** the `JoinLane` cost rather than
eliminating it.

*Could not determine:* whether PR #62's implementer weighed extraction
and rejected it — no spec was committed and the log is silent, so this
reads as *"happened, then documented"*. PRs #62 and #65 have **zero
comments**; the fork was never surfaced to Evan.

## S6. The four sweep verbs share no core; extrude's pipeline is hand-copied four times

- **Where**: `crates/sweep/src/revolve/mod.rs:511`,
  `crates/sweep/src/revolve/surfaces.rs:177`,
  `crates/sweep/src/extrude.rs:565`, `crates/sweep/src/loft.rs:259`,
  `crates/sweep/src/revolve/partial.rs:76`,
  `crates/sweep/src/revolve/full.rs:73`
- **Confidence**: sure

`SweptKind`, `SweptSeg`, `swept_segments`, `arc_span`, `turn_axis`,
`arc_apex`, `cosurface`, `cap_points`, `rim_spec`/`chain_spec`,
`strut_spec`, `face_surface_key` and the `decide` funnel all exist twice
— once in `extrude.rs`, once under `revolve/` — with only predicate-name
strings differing (`side_planes_cosurface` vs `wall_lines_cosurface`;
the two `cosurface` bodies are otherwise identical). The
lamina-plus-holes construction (mvfs → mev chain → Newell cap → bridge
mev/kemr/mef/kfmrh per hole) is then written out a **fourth** time,
near-line-for-line, in `extrude`, `partial::build_partial`,
`full::build_lamina` and `loft::assemble`.

`revolve/mod.rs:507` justifies the fork: "unify when a third sweep or a
shared lowering layer gives the shape an owner." The third and fourth
sweeps have since landed, `loft.rs` imports extrude's copy while
`revolve` keeps its own, and the note was not revisited. The copy in
`loft::assemble` carries a vestigial `let _ = k;` (`loft.rs:280`), which
reads as mechanical transcription rather than derivation.

**Verdict:** ACCEPTED (Evan, 2026-08-18), together with S5 and S7. Steelman
pass commissioned: confirm the unification trigger named at
`revolve/mod.rs:507` ("a third sweep or a shared lowering layer") has in
fact fired, and check whether the four lamina-plus-holes copies are
genuinely the same Euler-operator sequence.
**Steelman (2026-08-18): SURVIVES IN PART — the helper duplication and
the record-keeping failure are confirmed; four specific claims in this
finding are wrong, and one of them inverts a load-bearing correctness
decision.**

*The note's lineage, and a goalpost that moved.* The revolve fork was
written 2026-07-20 (M2 PR 5, #37), two days after extrude. The
**original** note was not an indefinite deferral — it read *"PR
7-adjacent unification"*, a commitment. On the same day, M2 PR 7 (#43)
**rewrote that sentence in the same diff hunk** to the version this
report quotes. So a trigger about to expire was replaced with a later
one in the very commit that let it lapse. PR #43's body — 45k additions,
exhaustively listing every deviation — does not mention it, and neither
does `docs/archive/M2-LOG.md`. The justification never entered the
PR-description record that `CLAUDE.md` designates as canonical.

*The trigger has fired, on both readings — and was half-answered.*
`crates/sweep/src/loft.rs:68` reads `use crate::extrude::{SweptSeg,
cap_points, face_surface_key, rim_spec, swept_segments};`. The third
sweep's author picked an owner — `extrude` — and imported from it. They
did not migrate `revolve` to that owner or revisit the note. The
condition was satisfied and half-executed 14 days before this scan.

*A ratified convention that speaks directly to this landed in between
and was not applied.* `docs/DESIGN.md:1176`, sharpened by Evan
personally at the M4 exit sweep (#119, 2026-07-27 — after the fork note,
before the fourth copy): *"structural sharing beats a sweep — **code
that is literally the same cannot drift**."*

**Four claims in this finding do NOT survive:**

1. **`SweptSeg` is not duplicated, and the divergence is load-bearing
   correctness.** Extrude's carries a `wall_sense: bool` that revolve's
   does not. M5 S11 (`docs/archive/M5-LOG.md:1975`) found concave arc
   walls minting `sense: true`, so a public `union` silently swallowed a
   body — *"volume 3.000 for 3.008, one shell for two, no refusal"*. The
   fix is three genuinely different rules: extrude `(canonical turn ==
   Positive)`; revolve `(canonical Δz > 0)` for cylinder and cone,
   `(canonical Δr < 0)` for plane annuli, turn-sign for sphere/torus;
   loft `sense = true` throughout. **A shared `SweptSeg` carrying one
   `wall_sense` bit would have been wrong.**
2. **`strut_spec` is not a duplicate** — same name and arity, entirely
   different bodies. Extrude mints `MappedCurve::ExtrudedPoint` on a
   `Curve3::Line`; revolve mints `RevolvedPoint { axis_origin, axis_dir,
   angle }` on a `Curve3::Circle`. A name collision read as duplication.
3. **`full::build_lamina` is not the lamina-plus-holes construction** —
   it has no holes (a full revolve of a holed profile is the typed
   `FullRevolveHoles` refusal, ratified in #37) and no Newell cap.
4. **The `let _ = k;` inference is wrong.** `k = sections.len()` is a
   **loft-only** quantity — extrude has no `k` — so it cannot be a
   transcription artifact. It is dead code from loft's own drafting.
   Two lines of lint debt, not evidence of mechanical copying.

**What survives, verified by diff:** the `cosurface` bodies
(`extrude.rs:565` vs `revolve/surfaces.rs:177`) are **token-identical** —
same `t = (prev.b - prev.a).normalize()`, same `t.perp_dot(d)`, same
`c1.distance(*c2) + (*r1 - *r2).abs()` — differing only in two
`&'static str` predicate names. Same for `rim_spec`/`chain_spec`,
`cap_points`, `arc_apex`, `arc_span`, `turn_axis`, `face_surface_key`,
`decide` and `SweptKind`. **~230 lines of genuinely identical code.**

*The K-telemetry objection does not block unification.* The names must
stay distinct — `docs/k-report-data/` shows `die_composed`, `die_pips`
and `kitchen_sink` each emitting **both** `side_planes_cosurface` and
`wall_arcs_cosurface`, so the shape column no longer disambiguates the
verb. But both funnels already delegate to `k_stats::decide(name, …)`
with `name` as a parameter, so a shared body taking a name pair
preserves every existing name bit-for-bit.

*The tightest duplication is one this report missed: it is inside
`revolve/` itself.* `partial.rs:73` and `full.rs:73` build the same
mvfs → mev(Lone) → mev(Fan) → mef chain with the same `chain_spec` calls
and the same `frame.n3`, differing only in the closing `mef`'s surface
and partial's pole tracking. **Same module, same error type, same
imports — none of the axis, error-type or scope barriers that justify
the extrude↔revolve fork apply here at all.**

*The strongest part of the finding is the record-keeping.* The S11
incident is precisely the failure mode the ratified convention names:
the fix had to be spec'd as a manual audit (*"grep for Face literals
with curved surfaces"*, `M5-S11-SPEC.md:23`), and that audit **found the
same defect class already shipped in revolve's bore cylinders, inward
cones and under-side plane annulus**.

*A small live regression found in passing:* `loft.rs:517` writes
`face_surface_key(…).map_err(|_| LoftError::SectionStructure)?`, so a
stale-key operator fault reaches the user as a structural section
refusal — caused by naive sharing across a typed-error boundary.

*Hidden costs of acting:* the predicate names are a gate, not a detail
(`K-REPORT.md` tracks a 233-name census and reviewers byte-reproduce the
CSVs); three closed error enums (D4) make every fallible shared helper
generic or lossy, and PR #192 deviation 7 already refused that bound
ripple once as *"out of scale for this unit"*; the test surface is **425
tests across 77 files, 21,634 lines, 63 of 77 PR-named**, not organized
by code structure, so a refactor cannot be scoped to a subset; and
unifying `swept_segments` hands loft extrude's `reverse` arm, which PR
#192 deliberately refused as untested (*"an untested orientation arm is
worse than a typed refusal"*).

*Could not determine:* whether Evan was ever shown the fork decision
(#37's "Notes for retroactive review" lists three items; this is not
among them), and whether the S11 defect class has a live analogue in
loft — see **S42**.

## S7. Two complete fillet assembly implementations, and the older one shadows the newer

- **Where**: `crates/sweep/src/fillet/build.rs:205`,
  `crates/sweep/src/fillet/build.rs:14`,
  `crates/sweep/src/fillet/surgery.rs:1`
- **Confidence**: sure

`fillet_edges` tries `whole_body_links` first and falls back to
`fillet_surgery` on `AssemblyUnsupported`. But the surgery's own front
door (single-link open chains ending at fully-requested trivalent convex
corners) already covers every input the whole-body door admits —
filleting all 12 edges of a cube satisfies both — so ~1200 lines of
plan/greedy-mint machinery exist to handle a strict subset of what
surgery handles, and win the race by being tried first. Corner balls,
octant charts, trimline/arc carriers, sense bits and naming rows are
derived twice, in two different idioms (plan-then-assemble vs
mutate-in-place).

The stated reason is bit-preservation: "kept (not subsumed) so its M5
outputs stay bit-preserved." A consequence worth noting separately:
every one of `whole_body_links`' ~10 carefully-worded refusal detail
strings is unreachable, because the caller discards the error to fall
through — which also makes the `Err(other) => Err(other)` arm at
`build.rs:213` dead.

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

## S8. The fitted (rung-3) pcurve lane has no producer anywhere in `src`

- **Where**: `crates/geom-brep/src/pcurve_cache.rs:1379`,
  `crates/geom-brep/src/pcurve_cache.rs:907`,
  `crates/topo/src/pcurves.rs:1057`
- **Confidence**: sure

`PcurveCache::certify_fitted` is called from no `src` file in the
workspace — only from test fixtures and three probe tests. `mint_face`,
the one minting pass, goes exclusively through the closed-form
`PcurveCache::certify` door. That makes roughly 500 lines — the
`PcurveFittedLane` trait and its four impls, `fitted_lane`,
`rational_arc_chain`, `carrier_diameter`, `ssi_refusal`,
`run_fitted_checks`, three `PcurveCertifyError` variants and two
`EnvelopeStatement` variants — machinery with no caller.

It is not free: the `Arc<NurbsCurve2>` payload costs `Pcurve` and
`PcurveCache` their `Copy` for every *other* variant. And it is
documented as "the general rung" and "LIVE since M6-2", which reads as
shipped rather than as speculative.

**Verdict:** ACCEPTED, SORT REQUIRED (Evan, 2026-08-18). On this whole
cluster: "a lot of these are to support planned work, so that'll need to
be sorted out from the ones that are superseded." Steelman pass
commissioned to make exactly that determination, and to price the
concrete cost — `Arc<NurbsCurve2>` is what denies `Pcurve`/`PcurveCache`
their `Copy` for every other variant.
**Steelman (2026-08-18): PLANNED (unscheduled) — SURVIVES IN PART, and
the stated price has already evaporated.**

*It discharged a ratified acceptance obligation*, not a speculation:
`M5-EXIT-WALK.md:110` records *"a non-vacuous fitted-cache-at-rest
row… recorded as an **acceptance obligation** of that unit"*. The
`Copy` cost was sized **in advance** (the *"~35-site ripple"*) and was a
named reviewer attack at dispatch.

*"LIVE since M6-2" meant the certified route exists, not that a
constructor mints one* — and PR #176 **said so in the merged body**:
*"no kernel constructor mints a `Fitted` cache into a body until the
banked join lane lands."* It did not ship with a producer that was later
removed; it shipped producerless, knowingly.

*Three named consumers, decreasing firmness:* (a) the **mint-side
general-circle wiring** for the oblique-trihedron octant — a
*currently-reached* coverage gap (`pcurves.rs:940`), named in ratified
`DESIGN.md:557`, but scheduled into **no milestone plan and absent from
#250**, the carried-items register Evan asked to be total; (b) U2's
`General` arm, **ratified 2026-08-15 — three days before this scan** —
with lily wall 8 depending on it (`M9-LOG` tail); (c) the germ-chord
lane, explicitly banked by Evan's M9-5 ruling.

**The stated price is gone.** `Pcurve::IsoArc` carries a `Vec`-backed
`KnotVector` and is minted in production since M8-3. **Deleting `Fitted`
would not restore `Copy` to `Pcurve` or `PcurveCache`.** The doc at
`pcurve_cache.rs:1242` blaming the `Arc` is **stale** — true at M6-2,
false since M8-3. The residual cost is three `.clone()` sites.

*"Machinery with no caller" is overstated by ~10×.* `PcurveCache::recertify`
dispatches `Fitted → run_fitted_checks` and **is called from production
tier-3** (`topo/src/pcurves.rs:1353`, check 8, ungated) — which is why
`validate_pcurves` is `<T: PcurveFittedLane>` and why `PropsQuadLane`
carries it as a supertrait. Only `certify_fitted` itself (~40 lines
including docs) is genuinely callerless.

*Hidden costs of removal:* it is all-or-nothing (`certify_fitted` is the
sole constructor of the variant); ratified docs go false (`M6-EXIT-WALK`
row 3 "MET… DISCHARGED non-vacuously", `DESIGN.md` frontier (b)
"CLOSED (M6-2)"); six test files including adversarially-adopted probes
delete or rewrite; and it **reopens a design ratified three days before
this scan** (U2 defines `General` as certifying *"at the honest Fitted
grade"*), against `CLAUDE.md`'s do-not-re-litigate rule.

## S9. The trim-containment limb is vacuous on both production paths

- **Where**: `crates/topo/src/pcurves.rs:1028`,
  `crates/topo/src/pcurves.rs:1300`,
  `crates/geom-brep/src/pcurve_cache.rs:2300`
- **Confidence**: sure

In `mint_face` the face's `ChartWindow` is computed as the hull of
exactly the chart boxes that are then checked against it, so containment
holds by construction. `validate_pcurves` does the same at
re-certification: window = hull of every stored cache's box, then each
cache is re-certified against that hull. So check 5 cannot fail on
either production path — it can only fail on an attach path that exists
in tests.

The comment at `pcurves.rs:1028` acknowledges the first half and argues
the check bites at re-certification; the re-certification code builds
its window the same self-referential way. Sustained by that check: a
public `ChartWindow` type, a `chart_box` arm per `Pcurve` variant, three
lever-arm helpers (`azimuth_lever`/`chart_arms`/`chart_arms_at`) and a
`TrimEscape` refusal.

**Verdict:** ACCEPTED, SORT REQUIRED (Evan, 2026-08-18) — see S8. Specific
question for the steelman: was the trim-containment check designed
against an *attach path* that was planned and never built? If so this is
a planned-work row, not a superseded one.
**Steelman (2026-08-18): NEITHER planned nor superseded — SURVIVES IN
PART. The mechanism claim held under attack; the inventory claim did
not; and the justifying comment is demonstrably false.**

*The mechanism claim is correct and could not be broken.* Containment is
an identity on **both** production paths (`pcurves.rs:1040` and `:1302`
each hull exactly the set they then check), and there is **no planned
attach path** — content-keyed cache transfer is banked (`DESIGN.md:1576`),
and the one `src` site that copies a cache between bodies is explicitly
transient. So this is a "neither" row.

*The justifying comment is false, and post-dates the code that falsifies
it.* `validate_pcurves` landed in `9e80547f`; the *"where this limb has
teeth"* comment at `pcurves.rs:1028` landed **later, in the same PR's
fix pass** (`a842090b`, *"trim-limb teeth comment"*) — i.e. a reviewer
NOTE flagged the mint-time vacuity, and the response was a comment
asserting teeth at re-certification, written against code already in the
same branch that builds its window the same self-referential way.

*What the design genuinely buys.* `PcurveCache` is a **certify-only
public door**, and this repo's review discipline treats attach-path
inputs as in scope — M5 PR 6's best MINOR was exactly an attach-path
falsification whose fix is now permanent production code. And check 5 is
the cache's **only branch constraint**: on a periodic chart a τ-shifted
pcurve certifies every other check identically, and `TrimEscape` is the
only thing separating them.

*The inventory claim does not survive.* Only `trim_containment`,
`TrimEscape`, `ChartWindow::hull`, the `window` threading and two hull
loops are check-5-only. `ChartWindow` is `chart_box`'s return type;
`chart_box` feeds check 2's azimuth headroom; `azimuth_lever` feeds
harmonic check 2 and check 4's snap slack; `chart_arms` feeds the fitted
lane's check 2. **They die only if S8 is acted on too — the two findings
are coupled in the removal direction, which neither writes down.**

*Cheaper action that captures most of the value:* truth the comment at
`pcurves.rs:1028` and downgrade the limb's documented status from
"bites at re-certification" to "a precondition on the public door,
vacuous on every in-tree caller".

## S10. The schema migration mechanism is dead, and fourteen versions are ceremony around it

- **Where**: `crates/editor-core/src/persist/mod.rs:602`,
  `crates/editor-core/src/persist/mod.rs:683`,
  `crates/editor-core/src/persist/mod.rs:1`,
  `crates/pncad-py/src/tags.rs:328`
- **Confidence**: sure

`migration_step` returns from a permanently empty `TABLE`, so any
`version != SCHEMA_VERSION` errors `SchemaTooOld` before the body is
touched. That makes the entire `else` branch in `load` (JSON-`Value`
parse, step loop, `from_value`), plus `MigrationStep`, `MigrationError`
and `PersistError::Migration`, unreachable code — nonetheless re-exported
through `pncad` and given a tag string in `pncad-py`.

Around it, ~250 lines of doc-comment ledger narrate fourteen versions,
several entries being post-mortems of merge races over a one-line
constant rather than descriptions of formats. v12 has no entry at all,
so the ledger the discipline depends on has already drifted. The goldens
make the point concrete: `tests/golden/v10..v14_golden.cad` are
byte-identical below the header line (as are v1–v3), so five "clean
breaks" are pinned by the same document with a different number on top,
each with its own test file.

**Verdict:** ACCEPTED, SORT REQUIRED (Evan, 2026-08-18) — see S8. Specific
question: is a migration mechanism planned for when the format
stabilises (scaffolding awaiting its first real migration), or was
"clean break, no migration" ratified — in which case the mechanism is
dead by decision and the fourteen-version ledger is the residue.
**Steelman (2026-08-18): DELIBERATE-FRONTIER with a PLANNED post-release
consumer — the mechanism does not survive as a finding; the ledger does.**

The empty table is **doubly ratified doctrine**, not drift.
`docs/archive/M4-LOG.md:1916` already records the emptiness decision verbatim:
*"The migration MECHANISM stays (an explicit, currently empty step table)…
the next non-breaking format change adds its step there."* And **LQ7a**
(`LIBRARY-DESIGN.md:332`): *"NO backwards-compatibility machinery of any kind
before release — no migration chains, no deprecation shims."* Restated as
binding in three more specs. The future consumer is named in-code and in
`DESIGN.md:1701` Band 4.

What survives is the **ledger, not the mechanism** — and it has measurably
drifted: there is **no "Version 12" entry** (v12 was LIB-PLACEDUNION #571,
which took the number and skipped the entry), while
`memories/schema-claim-discipline.md` calls that prose *the tripwire* for the
same-number merge race. Note **LQ7b**: version numbers **reset immediately
before release**, so all fourteen numbers are planned to be thrown away.

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

## S12. Euler atomicity is enforced by convention: every write silently no-ops on a missed precondition

- **Where**: `crates/topo/src/euler.rs:1940`,
  `crates/topo/src/euler.rs:1350`, `crates/topo/src/euler_ring.rs:1300`,
  `crates/topo/src/euler_kill.rs:800`
- **Confidence**: likely

Atomicity is the operators' headline contract — "preconditions fully
resolve before an infallible mutation phase" — but the mutation phase is
~60 `if let Some(x) = self.get_*_mut(k)` blocks across three files, each
of which *discards* the failure it was supposed to have proved
impossible. `link_half_edges` is the clearest case: it writes
`next`/`prev` through two fallible lookups and ignores both, with a
comment asserting "the lookups cannot fail on the operator paths".

Nothing structural distinguishes a validated key from an unvalidated
one, so a missed precondition produces a half-spliced body rather than
an error — and in release builds, with no postcondition, silent
corruption. The plan structs already carry pre-resolved data, so the
type system has the material to make this a proof; it is instead prose.

**Verdict:** ACCEPTED (Evan, 2026-08-18), and identified as one half of a
larger unresolved question: "the Euler atomicity thing also sounds like
it's hitting the tension here" — the tension being S14's question of what
the no-panic principle actually says. Steelman pass commissioned jointly
with S14; it also asks whether the plan structs could carry
proof-carrying resolved handles so the mutation phase cannot fail.
**Steelman (2026-08-18): SURVIVES IN PART — right on the census, wrong
on the enforcement, and the residue is real. See S43 for the larger
question this turned out to be part of.**

*Right on the census:* 58 discarding sites (`euler.rs` 20,
`euler_ring.rs` 20, `euler_kill.rs` 18), and `link_half_edges` is
exactly as described.

*Wrong on "in release builds, with no postcondition".*
`assert_euler_postcondition` (`euler.rs:1975`) runs the arena-delta
check **and full tier-1 `validate`** after every successful operator
under `cfg(debug_assertions)`, which the root `Cargo.toml` keeps on by
default in dev. A half-spliced loop is precisely what tier-1 validate
catches, so the whole test suite and CI do detect it.

*Wrong on "by convention".* The release disposition is **ratified and
deliberately tested**: D9's footnote ("documented garbage-out in
release"), `euler.rs:47`, and a dedicated adversarial suite
`crates/topo/src/review_m1_pr2/release_corruption.rs` whose header
reads *"typed errors or garbage bodies — never a panic, never a hang."*

*What genuinely survives, sharper than written.* The ratified
garbage-out contract is scoped to **corrupt in-crate input** — a body
already tier-1-invalid on arrival. The 58 discards also cover the other
case: a **valid** body plus a defect in the operator's own plan phase.
D9 says nothing about that case, because its footnote asserts it cannot
occur. So what is actually held by convention is the claim that the plan
phase resolves everything the mutation phase touches — and
`link_half_edges`' comment *is* that claim, in prose. Exactly as
unenforced as `Span`'s pairing, which is why the instinct that S12 and
S14 are one question is right.

*Two supporting facts.* The direction contradicts the repo's own most
recent argued position — PR #447's merged body: indexing *"panics where
a `zip` silently drops control points. **That is the fail-loud
direction**"* — and `euler.rs` takes the silently-drops side 58 times.
And `release_corruption.rs` instructs *"Run this under BOTH profiles"*;
**CI does not** — the only `--release` test invocation in `ci.yml` is
the `oracle-inari` lane.

*What blocks constructive validity:* not slotmap
(`get_disjoint_mut` still returns `Option` — it relocates the discard).
The real blocker is that the mutation phase **mutates the same arenas
between its own writes** (`mev_fan_execute` interleaves mints with
splices; `euler_kill` removes from six arenas mid-phase), so any
pre-resolved `&mut` is invalidated by the next mint and any pre-resolved
key is what the plan structs already carry. Key liveness across the
phase's own operations is not a type fact under slotmap's `&mut self`
insertion API. What is *not* blocked: making the write helpers unable to
silently do nothing — e.g. a sticky corruption flag that `validate` and
every public entry refuse on, converting silent release corruption into
a typed refusal at the next public call, for one branch per write.
Whether that is worth paying is a D9 question — which is the point.

## S13. Load-bearing invariants held by CI grep, allowlists, and a magic count

- **Where**: `.github/workflows/ci.yml:322`, `:420`, `:444`,
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
splits with refusing arms exist (S3). Each ledger amendment litigates a *type*
question, and M5 PR 12 **explicitly costs the type-level fix and rejects it on
the merits** (*"would have had an EMPTY refusing side"*). The greps enforce only
the residue — **which files may *name* a bound** — which Rust cannot express
across crate boundaries, since sealing controls who may *implement*, not who may
*name*. Nobody has written that down.

*Two gates could not be types or lints at all.* The interval-square rule guards
a **whole-program** property (`ci.yml:437`: *"whether THIS enclosure can
straddle zero is a global property of upstream callers that refactors change
silently"*), and the env ban has a measured receipt. And `EvalScalar` is
evidence the encoded alternative was **tried and needed more grep, not less**:
`ci.yml:354` — *"the trait is `pub`… so without this step any file in any crate
could acquire a compound Bounds bound invisibly to the grep above."*

*The brief's premise about `tools/k-lint` was wrong, and it matters.* `k-lint`
is a **CSV gate** over probe sweeps, not a source lint; it does not parse Rust.
**The repo has zero Rust-AST lint capability today.**

*What does not survive.* (1) **A lint/tool alternative was never evaluated** —
the only mention is a one-line "optional escalation" in `M0-LOG.md:74`, dropped
silently; no `dylint`, `clippy::disallowed*`, `clippy.toml` or proc-macro
appears anywhere in the history. The record is silence, not rejection. (2) The
dual-maintained allowlist is a **proven drift class**, and (3) the greps' own
known defects go unfixed — the `x*x` lookahead fix has sat in a log since
2026-08-04; `Real +` strips no comments while its four siblings do; only one of
six has a self-test. The regex is also leaky both ways: it **cannot see**
`self.x * self.x`, and `linalg/vec.rs:311` contains exactly that shape in
production generic-over-`Real` code.

*On the `decide_flagged` count of 8 — it is a real census, not a tripwire.* It
is tracked as **issue #214** with a per-family retirement plan; `ledger_row` is
`let _ =` at runtime, existing only to force the author to name a row; and the
count **has moved exactly once, downward** (12 → 8, `d92f56b5`, authored by
Evan, in the same diff that converted the ledger rows). Growth is the forbidden
move. Its inherited weakness is the one `schema-claim-discipline.md` names: it
asserts a **total**, so one site added and one retired nets to 8 and passes.

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

## S15. Other invariants held by prose rather than by types (roll-up)

- **Confidence**: sure for each row

| Invariant | Enforcement as written | Anchor |
|---|---|---|
| pcurve cache staleness | A paragraph ending "an op that starts mutating already-minted bodies must either clear the map or re-mint, and **should say which in its own docs**" | `topo/src/pcurves.rs:84` |
| Fillet birth-record provenance | Same-type key tuples `(FaceKey, FaceKey)`, `(EdgeKey, EdgeKey, FaceKey)` — swapping minted and source compiles and emits wrong names silently. Guard offered: "written three lines from the geometry it describes" | `fillet/naming.rs:95` |
| Which door fills which naming field | Doc claim, no type or test | `fillet/build.rs:48` |
| Fillet `Retired` survivor guard | Has no face channel; the comment justifying the omission ("faces are never retired by either door") is **false for the whole-body door**, where every source face is gone | `names/emit_fillet.rs:238` |
| "The only public mutation paths" | Frozen count of eleven. `split_edge`, `movefac`, `merge_coplanar_faces`, three setters and the splitting pipeline's bulk arena delete are now also public — and `seqgen` inherited the number, so `split_edge` (Euler vector **identical to `mev`'s**) never enters the fuzz lane | `euler.rs:41`, `seqgen.rs:12` |
| Tie propagation across emitters | Per-emitter convention; `emit_fillet`'s `up()` never inspects `Entry`, so a legitimate upstream tie surfaces as `NamingError::Duplicate` — an error whose own docs say it means "the no-silent-aliasing bug" | `names/emit_fillet.rs:94` |
| `topo::iso` geometry-blindness | Justified by "at M1 they are `Placeholder` ballast". Carriers have been real since M2, so the isomorphism oracle now calls two bodies with entirely different geometry isomorphic | `topo/src/iso.rs:56` |
| The 16-direction ray schedule | Re-declared verbatim in a second module, deliberately not shared, with the justification "to keep the module boundaries thin" — determinism depends on byte-identity and nothing checks | `boolean/solid_contain.rs:76` vs `splitting/containment.rs:102` |
| `flipped_face_sense_for_tests` | `pub` on `Body`, `#[doc(hidden)]`, 18-line comment explaining it produces an incoherent body. Only `_for_tests` public fn in the workspace; exists because face orientation has two encodings kept coherent by convention | `topo/src/body.rs:630` |

**Verdict:** ACCEPTED (Evan, 2026-08-18) — "lots of other great catches."
Steelman pass commissioned, prioritising the three rows that are
bug-shaped rather than stylistic: the `emit_fillet` `Retired` comment
being false for the whole-body door, `split_edge` sharing `mev`'s Euler
vector while never entering `seqgen`'s fuzz lane, and `topo::iso`
ignoring carriers that have been real since M2.
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

`boolean::boxes::face_box` special-cases Cylinder and Sphere faces to
whole-extent boxes, then falls through to a **vertex-hull** box for
everything else, justified by "straight edges make the polygon lie in
its vertex hull". A cylinder's planar cap — which this engine's
plane×cylinder lane mints — is a planar face whose rim is a circular arc
that bulges past its two endpoints, so the box is not a superset and the
BVH can prune a pair the exact predicates would have accepted, silently.

`separation::certified_face_box` is a second, corrected implementation
(hulls the planar arm with the boundary *edge* boxes, poisons NURBS) and
it calls `boxes::face_box` to build on — but the sweep itself still uses
the raw one. `census.rs` is a third, with a `max(r, sagitta)` pad and an
outright NURBS refusal; its own comment admits the duplication. The
module already carries a long "KNOWN GAP" note about the NURBS half of
the same defect while the planar-with-conic-rim half sits under a
sentence asserting the opposite.

This is the finding most likely to correspond to a live wrong answer.

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

## S17. Three ray-parity point-in-polygon implementations, one an admitted transcription

- **Where**: `crates/topo/src/chart_region.rs:897`,
  `crates/topo/src/splitting/containment.rs:154`,
  `crates/profile/src/validate.rs:1298`
- **Confidence**: sure

`chart_region::point_in_polygon` is a line-for-line port of
`splitting::containment::point_in_loop` — same boundary pre-pass with
the same `norm_squared` comment, same clamped-foot distance, same
`'ray:` retry loop, same straddle/advance parity, same 16-entry
direction table (`SCHEDULE` vs `SCHEDULE_2D`) — with only three
predicate names renamed and the arm gate dropped. Its own doc says so.
`profile::validate::point_in_loop` is a third with its own golden-angle
schedule and its own `RayCastingExhausted`.

Both topo copies also reuse one predicate name for two different
questions: `point_in_loop_boundary` (and `chart_region_boundary`)
decides both the segment-length degeneracy gate and the point-to-segment
distance — exactly the drift `splitting/rules.rs:117` explicitly mints a
distinct name to avoid.

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
third copy passes two independent adversarial reviews without comment — and the
K-ledger convention (new names = new rows) actively **rewards copying over
parameterizing**.

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
| Negative-zero flush helper | 3 in one crate, three separate doc blocks making the identical argument, plus a 4th inline | `step-import/src/geometry.rs:30`, `recognize.rs:160`, `recognize_curve.rs:222` |
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
- **Negative-zero flush ×3** — same crate, three units, one week; the original
  is already `pub(crate)` and reachable from both later sites. **NEVER
  FLAGGED** — the concept was tracked only as a *fixture* property
  (`M7-LOG.md:694`, a byte-divergence class), never as code ownership. *The
  cheapest one to have caught.*
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
  string.
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

## S20. The façade layers each invented a vocabulary instead of forwarding

- **Where**: `crates/pncad/src/closure.rs:1`,
  `crates/pncad/src/select.rs:450`, `crates/pncad/src/lib.rs:80`,
  `crates/pncad/src/workspace.rs:235`
- **Confidence**: sure

Excluding `workspace.rs` and `export.rs`, `pncad` is ~180 lines of code
under ~1170 lines of narrative. `closure.rs` is 151 lines that compile
to **nothing** — a module existing solely to hold an audit essay, whose
own tail (`:135`) documents that its headline enforcement claim was
previously false and the real guard is weaker. `select.rs` is 449
comment lines in front of one `pub use`. The curated re-export lists are
a permanent manual sync obligation with no mechanism keeping them
complete.

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

## S21. Two concrete holes in the Python surface

- **Where**: `crates/pncad-py/src/py/doc.rs:200`,
  `crates/pncad-py/src/errors.rs:54`,
  `crates/editor-core/src/expr.rs:48`
- **Confidence**: sure

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
| `mesh` reads ε twice where the crate docs and `DESIGN.md` D4 say once — and the second read **snaps** a value | `walk.rs:730`, `walk.rs:496` | The mesh is a function of (body, δ, **ε**), not (body, δ) as the determinism/memo-key contract claims. F6 also bans "EPS snapping anywhere in the pipeline" |
| `same_chart` decides chart identity by `core::ptr::eq` on two `&Body` | `chart_region.rs:394` | In a module whose premise is "structural identity, never numeric identity". A caller passing a clone silently drops to the weaker rung |

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
  (`survives_eps_row_bitwise_independence`), whose stale comment still reads
  *"ε is read once, for pole identification"*. **The test still passes**,
  because only a foreign STEP file produces a nonzero residue.
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

**On F6:** F6 bans ε snapping in the *boolean reduction/classification*
pipeline. This snap is display/export-layer and moves no kernel entity, so it
is not what F6 forbids. What it violates is the narrower mesh-local claim.

## S23. The exhaustiveness sweep degrades silently to seed-generation

- **Where**: `crates/geom-brep/src/ssi/exhaust.rs:140`, `:267`,
  `crates/geom-brep/src/ssi.rs:711`
- **Confidence**: likely

`sweep_r3`/`sweep_chart_plane` do two completely different jobs — seed
generation and the never-silence accounting proof — and the switch
between them is `tubes.is_empty()`, a data condition rather than a mode
the caller states. If every seed fails Newton refinement
(`SeedRefinementFailed => continue`), or the ℝ⁴ arm's branches all lack
`pcurve_b`, the "accounting" call receives an empty slice, takes the
seeding branch, pushes surviving floor cells into a discarded vector,
and returns `Ok` instead of `ExhaustivenessInconclusive`.

The operation then reports zero branches *plus an exhaustiveness
receipt* — exactly the silent incompleteness this module exists to
prevent. The documented receipt identity `examined == excluded +
accounted + refined` also quietly stops holding in that mode, since the
pushed leaves land in no bucket.

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
## S25. Two ε vocabularies flow through SSI with nothing reconciling them

- **Where**: `crates/geom-brep/src/ssi.rs:488`, `:507`,
  `crates/geom-brep/src/pcurve_cache.rs:1012`
- **Confidence**: sure

Every SSI entry point takes both a `Band` (whose `zero()` *is* the run
tolerance) and a separate `eps`. All trilean decisions read `band`; the
tube-radius floor, the exhaustiveness floor, `SSI_NEWTON_TOL` and
`SSI_STEP_DEVIATION` read `eps`. Nothing checks the two agree, so a
caller can march and size tubes at one tolerance while certifying at
another. Two of the four `certify_rung3` call sites have already noticed
and derive `eps = band.zero()` — one says so in a comment — while the
SSI arms keep the independent knob. Same quantity, two sources of truth,
reconciled by convention at some call sites and not others.

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
constraints — **exactly what `planar.rs:229` warns is unsafe** — while
`planar`/`trimmed` use `try_add_constraint` with crossing counts, and
only `trimmed` classifies intermediate vertices.

What sharing exists is ad hoc: `trimmed` imports `classify_faces`/
`edge_key`/`shoelace2` from `planar`, and both `trimmed` and
`tessellate` import the tolerance bundle `Tol` from `curved`. The common
code lives in whichever lane happened to grow it first.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
**Postmortem (2026-08-18). The warning postdates the lane it appears to
indict.**

`planar.rs` and `curved.rs` were born together in PR #39 (M2 PR 6), already
divergent; `trimmed.rs` arrived two milestones later. The warning at
`planar.rs:227` was written **five days after `curved.rs`**, by PR #116, as a
*local precondition of that PR's new even-odd flood fill* (*"would invalidate
the crossing bookkeeping built below"*). So it was never a claim about
`curved`, which has no crossing bookkeeping — and still inserts its grid after
constraining.

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
## S34. `readback.rs` is a body-wide accessor module housed in `sweep`

- **Where**: `crates/sweep/src/readback.rs:163`, `:379`, `:559`
- **Confidence**: sure

`face_pose`, `edge_pose` and `vertex_point` take a `topo::Body` and touch
nothing from `sweep`; `blend_arcs` reads only a `profile::ValidatedLoop`.
Because they live here, `editor-core` and `pncad` depend on the whole
`sweep` crate — NURBS skinning included — to read a face's plane. The
three op-specific doors on top (`extruded_caps`, `lofted_caps`,
`revolved_caps`) are each two `face_pose` calls on public fields the
caller already holds, and two of the three have no callers outside their
own doctests. `vertex_point` is additionally a near-copy of
`revolve/upgrade.rs`'s, differing only in error type. The module's home
was chosen by provenance ("these questions arose from sweep ops") rather
than by what the code depends on.

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
| `certify_rung3`'s `arm` and `extent` are the same value at three of four call sites, so the `#[allow(too_many_arguments)] // one parameter per named quantity` covers a parameter varied once | `ssi.rs:925`, `:729` | sure |
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

`mesh`'s `UnsupportedCurve.note` is documented as carrying "the PR that
lands it (**runtime-visible through Debug**; review m2)". Every ASCII
STL this kernel writes says `solid cad-kernel-m2`, and the binary header
reads `binary STL; CAD kernel M2 tessellation export` — baked into every
exported file and therefore into byte-comparison goldens, with no way
for a caller to set them, while the STEP writer takes product name and
header fields as options.

Most consequentially, ~124 references to internal spec codes, PR
numbers, milestone units and named rulings (`LIB-DOORS F5`,
`ASM-R2a D-4`, `LB7`, `GQ5`, `#286`, "Evan's ruling on #413", "the
ordinal-28 ruling") sit inside `///` and `//!` comments on the **public
API surface** — text that ships to library users in rustdoc and in the
Python stub. These name *when a decision was made*, not what the item
does, and are unresolvable outside this repository. Several also record
history the reader does not need ("an earlier revision of this comment
claimed that, and it was false").

**Verdict:** ACCEPTED, AND SEPARABLE — CAN BE FIXED EARLIER (Evan,
2026-08-18). *"The shipped artifact comments can be fixed earlier."*

Distinguished from S36 deliberately: milestone naming **inside** test files is
a backlog marker worth keeping until the suite is combed (S36), but milestone
naming that **escapes into shipped output** — `solid cad-kernel-m2` in every
STL, the PR number runtime-visible through `UnsupportedCurve.note`'s `Debug`,
and ~124 internal spec codes in public rustdoc and the Python stub — carries no
such signal and can go now.
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
near-verbatim at `curved.rs:90`, and S15's `Retired` comment is a
justification that is simply false for one of the two doors it covers.

**Verdict:** ACCEPTED (Evan, 2026-08-18). *"The comments should definitely
be trimmed down to what's actually necessary."*
## S39. Stale claims that other code is instructed to rely on

- **Confidence**: sure

| Claim | Reality | Anchor |
|---|---|---|
| `enters.rs` derives M3's whole sign chain from "every face's stored normal is the outward normal" and tells future callers to cite that sentence rather than introduce a fresh sign choice | `step-export/src/volume.rs:36` states flatly that since M5 S10 this "is no longer true" — the outward normal is `sense_sign · chart_normal`. Callers do pass sense-corrected normals, so the code is fine; the canonical statement is not | `geom-brep/enters.rs:14` |
| `EulerOpError::CrossShell`'s `Display`: "cross-shell kfmrh merges shells — deferred to M3" | Its only firing site is `ring_move`, so the only user who can see this message is told about a different operator and a shipped milestone | `euler.rs:583` |
| `euler_ring`'s module docs: "cross-shell is a typed error until M3" | Three hundred lines above the `kfmrh` doc describing the cross-shell fusion it now implements | `euler_ring.rs:126` |
| `euler_kill`'s `mfkrh` docs describe a fresh `Placeholder` surface, a deterministic surface anchor and "ballast coordinates" | Machinery the code notes was retired when `mfkrh` grew its `FaceSurface` parameter | `euler_kill.rs:197` |
| `Node::instantiate_part`: "none can in v1: `InterfaceCrossing` is uninhabited" | `InterfaceCrossing`'s own doc four hundred lines above: "**INHABITED as of ASM-R2b D-4**". `refactor.rs:82` repeats the stale claim | `node.rs:970` |
| `pcurve_cache`'s stranded `run_iso_checks` doc still claims the lane admits "exactly the non-rational described-NURBS chart" | M8-3 changed that; the doc block is also attached to the *wrong function* (see S40) | `pcurve_cache.rs:2459` |
| `step-import`'s `UnsupportedUnit`: "the subset covers unprefixed SI metre/radian/steradian only" | `units.rs` resolves all sixteen SI prefixes and `CONVERSION_BASED_UNIT` chains today | `step-import/error.rs:419` |
| `props/quad.rs:42`: "the patch flux engine consumes this machinery at rest" | It runs a separate near-parallel copy; the claim reads as a stale justification for keeping S11's dead lane | `props/quad.rs:42` |
| `assemble::build`'s doc: "the assembly both doors share verbatim" | Only one door remains, and `build_one_solid` is a one-line forward to it | `step-import/assemble.rs:810` |

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

- **`enters.rs:14`** derives M3's entire sign chain from *"every face's stored
  normal is the outward normal"* and instructs future callers to cite that
  sentence. `step-export/src/volume.rs:36` says it has been false since M5 S10.
  Callers currently pass sense-corrected normals, so the code works — but the
  question the claim raises is whether the outward-normal property *should*
  have been preserved rather than devolved onto every caller.
- **`pcurves.rs:91`** (found by the S15 steelman, not the original scan) lists
  `merge_coplanar_faces` among ops that neither clear nor re-mint the pcurve
  cache. It **started re-minting on 2026-08-05**. Here the code moved in the
  *safe* direction and the index rotted behind it — the benign reading.

Each row needs the same question asked before its sentence is touched: **which
of the two happened?**
## S40. Residue and editing artifacts

- **Confidence**: sure

- **FIXED by #NNN.** `run_properties` now RETURNS the number of
  roundtrips that executed, and the counter is asserted at the strength
  the design supports. A single case may legitimately execute none (a
  short decision vector may hold no roundtrip step, and the documented
  irreversible-by-one-op kills are skipped by design), so the honest
  statement is not per-case: the proptest test became a plain `#[test]`
  around `proptest!`'s closure form, summing executions across the whole
  run and asserting the total is non-zero once the run completes. The
  deterministic issue-#60 case, whose final step IS a `kef` roundtrip,
  asserts its own execution. Both go RED against a `roundtrip` forced to
  skip everything.
- **FIXED by #NNN.** `powi_edges`' `|| true` row now asserts what the
  case is for: `x.powi(i32::MIN)` is not poison and is a finite
  underflowed bracket of zero. The overflowed positive power is the
  honest `[MAX, +inf]` (not poison), so its reciprocal has a positive
  subnormal upper bound rather than collapsing — which the old row could
  not have told apart from poison.
- **FIXED by #NNN.** `every_error_displays` is exhaustive BY
  CONSTRUCTION: a local `variant_index` matches `EulerOpError` with no
  wildcard, so a new variant fails to compile until it is listed, and a
  coverage array then stays red until a Display sample for it joins the
  list. All 27 variants are sampled.
- **FIXED by #NNN.** The seam-vertex parentage match now scrutinises
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
- **FIXED by #NNN.** Confirmed and removed. `select.rs`'s live
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
- **FIXED by #NNN.** The refusal message's line continuations are
  restored; the runtime text is one clean sentence again (the acceptance
  probe matches on `NON-PARALLEL`, which is unchanged).
- **FIXED by #NNN.** All three stranded doc comments are reattached to
  the item they describe: `loose_partners`' doc (head and orphaned tail
  rejoined) off `germ_section_frame` and `type LooseMap`,
  `feed_stable_name`'s off `naming_key`, and `run_iso_checks`' off
  `run_iso_arc_checks`.
- **FIXED by #NNN.** `remap_contacts` takes its two `KeyView`s by
  reference like `remap_carried` already did, so `finish_fallback` binds
  the pair once. The duplicate existed only because the first call
  consumed them.
- `WitnessSlot {}` is an empty struct occupying a field on every
  `NodeValue`, paired with a `NodeErrorKind::WitnessBifurcation`
  documented as never constructed (`eval/mod.rs:230`). **STILL OPEN** —
  deleting either changes the eval value type.
- **FIXED by #NNN.** `save`'s `drop(replay)` is gone. In `member_of`
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
- **FIXED by #NNN.** Removed, with its `eprintln!` block. Nothing in
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
## S41. The `Enclosure` seam launders `Interval` decorations, possibly today

- **Where**: `crates/geom-core/src/spline/hull.rs:98`,
  `crates/geom-brep/src/ssi/enclose.rs:45`,
  `crates/geom-core/src/interval.rs:143`,
  `crates/geom-core/src/interval.rs:451`
- **Importance**: high
- **Confidence**: unsure — this is a *lead*, not a confirmed defect
- **Raised by**: the S1 steelman pass, 2026-08-18. Not part of the
  original scan.

`impl Bounds for Interval` forwards `self.0.lo()`/`hi()` without
consulting the decoration. `bracket<E: Enclosure>` at `hull.rs:98` and
the M6-2 lift at `ssi/enclose.rs:45` both cross `T: Bounds` operands
into `RingInterval` **by endpoints**, via `RingInterval::from_bounds` —
dropping the decoration. That is exactly the round trip
`interval.rs:143` warns against.

Empty and NaI do surface as NaN, and `interval.rs:432` argues that at
length. The gap is the **`Trv`-decorated but nonempty** enclosure: a
domain violation with finite endpoints. If one of those ever reaches
these entry points, the violation is silently dropped **now**, before
any change contemplated in S1. No guard and no test pinning it was
found.

Settled by: constructing a `Trv` `Interval` (e.g. `sqrt` of a
zero-straddling enclosure), pushing it through `span_hull` /
`implicit_enclosure`, and asserting the resulting bound refuses.

**Verdict:**

## S42. Loft's `sense = true` derivation is untested against the shape that broke extrude

- **Where**: `crates/sweep/src/loft.rs:30`,
  `docs/archive/M5-LOG.md:1975`, `docs/archive/M5-S11-SPEC.md:23`
- **Importance**: medium
- **Confidence**: unsure — a lead, not a defect
- **Raised by**: the S6 steelman pass, 2026-08-18.

M5 S11 found extrude minting `sense: true` on concave arc walls, so a
public `union` silently swallowed a body (*"volume 3.000 for 3.008, one
shell for two, no refusal"*). The remediating audit then found the same
defect class **already shipped** in revolve's bore cylinders, inward
cones and under-side plane annulus — i.e. the class has recurred once
per verb.

Loft derives `sense = true` on every wall, argued from the skinned
chart's normal following the traversal (`loft.rs:30`). The argument
looks sound. But PR #192 pins *"all six loft faces keep `sense =
true`"* on a single fixture — `loft_prism`, a prism, with **no concave
arcs and no holes**: precisely the shape that did not break extrude
either.

Settled by: lofting a section pair with a concave arc, and one with a
hole, then running the S11 union check (two operands whose union must
produce two shells).

**Verdict:**

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

**Verdict:**

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

**Verdict:** OPEN — this is Evan's own question and supersedes S3's disposition.
S3 should not be acted on until it is answered, since the answer determines
whether the collapse target is "one lane trait in `geom-core`" (S3's steelman,
compiled and working) or "no lane traits at all, and a `Bounds` split into its
two meanings."

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

## S45–S48 — reserved

IDs `S45`–`S48` are intentionally unallocated, so that items promoted
out of the S35/S40 roll-ups during review can be given stable IDs
without renumbering anything above.

---

# §A. Where I would start

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
   awkward — S8 is why `Pcurve` is not `Copy`.
4. **S4.** The two observed drifts (mate names not rewritten by
   `Rebind`; `name_args`' fail-quiet wildcard) are worth checking
   regardless of what is decided about the pattern.

---

# §D. A schedule for fixes

**Status: a proposal, not a plan.** Ordered by value-for-time, with the
dependency edges made explicit so nothing polishes code that is about to be
deleted. Wave 0 is decisions only — most of the rest is gated on it.

Two ordering rules govern the whole thing:

1. **Decide before you delete; delete before you polish.** Comment trimming
   (S38) and test-suite combing (S36) must come *last*, because both operate on
   files whose fate Wave 0 has not yet settled.
2. **A finding whose steelman said SURVIVES IN PART is scoped by the steelman,
   not by the original finding.** Several findings shrank materially under
   scrutiny; the retracted parts are marked in each entry.

---

## Wave 0 — decisions (Evan). Nothing large should start before these.

Cheap in wall-clock, enormous in what they unblock. All four are judgement
calls that no agent should make.

| # | Decision | Gates | Why first |
|---|---|---|---|
| **D1** | **S44 — what is `Bounds`?** Is it "carries a bracket" (a semantic property, definable for `Dual` as lo=hi=value) or "may enter certified code" (an access-control marker)? | **S3** entirely; colours **S1**, **S2** | The lane traits exist only to mediate the second meaning. The answer picks the target: one lane trait in `geom-core` (the steelman compiled one, 16 impls → 2), or none at all and `Bounds` split in two. **PRICED 2026-08-18 (see S44's pricing entry): the split costs *nothing* in `src` — no production code instantiates any lane at `Dual`. It costs one deleted test and four D9 bit-identity assertions re-expressed.** |
| **D2** | **S43 — the bug-vs-invalid-state taxonomy.** D9 currently sanctions only "typed error where cheaply detectable, or documented garbage-out"; the kernel uses five idioms, two of them mutual negations. | **S19** (which it *generates* — ~239 of ~260 sites); resolves **S12**/**S14** residue | Restating D9 decides three findings at one stroke. Touching any error enum first means redoing it. |
| **D3** | **S7 — run the one-line experiment.** Swap the arms at `fillet/build.rs:205`, `cargo test -p sweep --test all`. | **S6**, and all fillet work in **S36**/**S38** | **EXPERIMENT RUN 2026-08-18 — see S7's experiment entry.** The surgery succeeds on the cube and yields the same solid (identical census and coordinate set; volume one ulp apart). The two doors *are* redundant, so the decision is now purely retire-or-keep against a measured price: one naming test rewritten, two goldens regenerated, one FreeCAD acceptance re-run. |
| **D4** | **S11's four undecided rows** — `Mat2`/`Affine2`, `PairSolve`, `hull.rs`'s non-rational unused half, the two inlined fillet helpers. | Cleanup in those files | Each is delete-or-keep. Cheap to answer, and answering stops anyone documenting them. **CHECKED 2026-08-18 (see S11's D4 entry): `hull.rs` should be struck — the deletion is really "retire the `sup_norm_bound*` API", whose rational limb is on the #390/#453 lane. `PairSolve`'s consuming unit (R2-b, #591) has merged without constructing it.** |

---

## Wave 1 — correctness. Fully parallel; no cross-dependencies.

The findings most likely to be a live wrong answer. Every one is disjoint from
every other, so these can run as five concurrent lanes.

| # | Finding | Effort | Note |
|---|---|---|---|
| **W1a** | **S16** — `boolean/boxes.rs`'s planar arm uses a bare vertex hull, but a cylinder's planar cap has a circular rim that bulges past its endpoints, so the box is not a superset and the BVH can prune a pair silently. | S–M | **Highest single-item value in the report.** The fix is already named in `PERF-SCAN-2026-08.md` Tier A finding 1, and `separation.rs` already contains the corrected planar rule. |
| **W1b** | **S23** — the SSI exhaustiveness sweep switches duty on `tubes.is_empty()`, so an all-seeds-fail run returns `Ok` *plus an exhaustiveness receipt* instead of `ExhaustivenessInconclusive`. | M | Make the duty a stated parameter. The acceptance row also needs replacing — its premise (`..._even_though_branches_were_found`) excludes the failing mode. |
| **W1c** | **S41** — `Bounds for Interval` forwards `lo()`/`hi()` without consulting the decoration, and `bracket<E: Enclosure>` crosses operands into `RingInterval` by endpoints. A `Trv`-but-nonempty enclosure may be dropping a domain violation **today**. | S to test, ? to fix | Also the gating question for S1 — until this is settled, "swap `RingInterval` for `Interval`" is unsound. |
| **W1d** | **S4 drift (a)** — `Rebind`'s rewrite loop ends `_ => {}` and never reaches `Node::Mate`'s two `StableName`s, so a mate head is either falsely refused as `RebindNoReferences` or silently left dangling. Contradicts `ASSEMBLY-DESIGN.md:566`. | S | Needs a red-then-green test and an A12 read. No issue is filed. |
| **W1e** | **S42** — loft's `sense = true` is pinned only on `loft_prism`: no concave arcs, no holes, i.e. the shape that did not break extrude either. | S | Loft a concave-arc section pair and a holed one, run the S11 union check. Cheap; may find nothing. |

---

## Wave 1b — hygiene. Parallel with Wave 1, trivially disjoint.

Good work for filling parallel capacity. None blocks anything.

| # | Item | Effort |
|---|---|---|
| **H1** | **ci-local mirror parity** — the local mirror has no `EvalScalar` step and no interval-square `powi(2)` step; hosted has both. Decide add-or-document. (The `separation.rs` and dead-`test_support.rs` halves are fixed in this PR.) | S |
| **H2** | **S39 stale claims** — nine rows, each classified **benign rot** vs **lost invariant** *before* its sentence is touched. `enters.rs:14` is the (ii) candidate: the outward-normal property was devolved onto every caller with no type enforcing it. | M |
| **H3** ✅ #NNN | **S40 residue** — start with the two that are not cosmetic: `emit_topo.rs:1266`'s unreachable fallback would mint `Seam{ae, ae}`, a well-formed name for the wrong thing; `seqgen.rs:853`'s discarded counter means the property suite cannot tell an all-skipped run from a full one. **FIXED by #NNN**: both behavioural rows plus the mechanical residue; S40's design-call rows (the `k_stats` shim, `WitnessSlot`, `props/curved.rs`'s NaN throws and doubled `Rim` direction, the `HashSet` paragraph) stay open there. | S |
| **H4** | **S37** — shipped-artifact naming: the STL header's `cad-kernel-m2`, `UnsupportedCurve.note`'s runtime-visible PR number, ~124 internal spec codes in public rustdoc and the Python stub. Evan: *"can be fixed earlier"* than S36. | S–M |
| **H5** | **S4 drift (b)** — `names/select.rs:319`'s `_ => Vec::new()`, the fail-quiet wildcard its three siblings forbid by comment. One function. | XS |
| **H6** | **Euler postcondition 7-tuple → named struct** — unnamed positional, 16 sites, 6 files, all `cfg(debug_assertions)`. Mechanical. | S |

---

## Wave 2 — structural. Sequenced by dependency.

| # | Finding | Gate | Note |
|---|---|---|---|
| **W2a** | **S3** — lane-trait collapse, or its dissolution | **D1** | The steelman has a working `geom-core` design (compiled, cross-crate, stable): one trait + a rank-2 job callback, 16 impls → 2. Three confirmed defects go with it (`PcurveFittedLane`'s pure-indirection `Option`, `lane_name()` 6/8 dead, `ChartRegionLane`'s collapsing `Option<Result>`). |
| **W2b** | **S1 / S2** — `RingInterval`, and whether `Interval` becomes always-on | **D1**, **W1c** | Blast radius is **535 refs in 15 files**, not the ~600 sites this report first claimed; five files carry 60%. Build cost measured at ~zero. The real obstacle is the decoration seam (W1c), not licensing or build time. |
| **W2c** | **S19** — the three big error catch-alls | **D2** | `AssemblyUnsupported` (146), `MissingEntity` (49), `SplitJoinError::Corrupt` (42). These *are* D9's only sanctioned option today, so D2 must land first or the work is undone. |
| **W2d** | **S6** — sweep helper unification (~230 token-identical lines) | **D3** | Must follow D3: S6 and S7 are in one crate and will collide. K-telemetry does **not** block it — both funnels already take the predicate name as a parameter. Retracted: `SweptSeg`, `strut_spec`, `full::build_lamina` and the `let _ = k;` inference are *not* duplication. |
| **W2e** | **S5** — `splitting/` vs `boolean/` | — | The largest. Start with the narrowest, highest-value piece: the **forked sector predicates**, which are dimensionally identical line-for-line and split one K population 29:1, with the 64-sample tail the one reaching margin 0. The repo already forced the reverse fix once (`M3-LOG.md:264`). |
| **W2f** | **S4** — the vocabulary mirrors, cheapest first | partly **W2c** | `BooleanOp` → `pub use topo::BooleanOp` + `serde(with)` (its constraint provably lapsed the day it was minted, and the technique is shipped); then units; then `ProgramStep`/`WireStep`, which is cheap in isolation but **blocked behind OnArc + RESPELL-TABLE** and crosses the same files. |

---

## Wave 3 — last, deliberately.

| # | Item | Why last |
|---|---|---|
| **W3a** | **S36** — comb-and-rename, **per suite** | Evan: *"they should only be renamed after an actual review and fixup to become normal test suites."* A PR-numbered name currently *carries signal* — it marks a suite not yet combed. Renaming first converts a visible backlog into an invisible one. Explicitly **not** a rename pass. Note the 2026-08-13 retirement licence has produced **zero** deletions in five days against ~10 new review-named suites, so this needs an owner and a slot, not just permission. |
| **W3b** | **S38** — comment trimming | Must follow every deletion above. Trimming comments on code that Wave 0 is about to delete is pure waste. |
| **W3c** | Remaining **S35** roll-up rows | Lowest value density; several will be resolved incidentally by Waves 1–2. |

---

## The dependency edges, in one place

```
D1 (Bounds?) ────────────► W2a (S3 lane traits)
             └───────────► W2b (S1/S2 scalars) ◄─── W1c (S41 decoration seam)
D2 (bug-vs-invalid) ─────► W2c (S19 error catch-alls)
D3 (fillet experiment) ──► W2d (S6 sweep) ──┐
D4 (dead rows) ─────────────────────────────┼──► W3b (S38 comments)
all deletions ──────────────────────────────┘
                                              └──► W3a (S36 suites, per suite)
```

Everything in **Wave 1** and **Wave 1b** is free of these edges and can start
immediately, in parallel, in any order.

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
registers have no way to notice. Compare S15's row sort: **zero of its eight
rows has a tracked issue**, even though the repo demonstrably knows how to do
better (issue #214 for a census, `attach.rs:119`'s KNOWN HAZARD block for a
named-and-pinned gap).

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

`planar.rs:229`'s warning, PR #116's pre-scan, and PR #157's
`SelfTouchingTrimLoop` are **three encounters with one hazard**, each closed
inside its own lane — while the lane that predates all three (`curved.rs`)
still carries the ordering the warning describes. A fix that establishes an
invariant needs an explicit sweep of sibling implementations as part of its
**acceptance**, not just its own regression row.

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
  because only a **foreign STEP file** produces a nonzero residue.
- `parallel_schedule_preserves_verdict_logs` pins **thread** confinement;
  ASM-2A broke **re-entrancy**.
- Four process-isolated binaries pin the ε global's init discipline; none
  observes that a *document*'s ε still commits into the same lock.

Nothing re-derives a pin when a new caller arrives. And a stale comment on a
still-passing test reads as **evidence the invariant holds**.

---

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
