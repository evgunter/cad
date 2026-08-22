# SMELL-SCAN — the open structural findings, and their schedule

**This document is the open half of a structural audit of the kernel, plus
the schedule that carries it.** Two scans (2026-08-18 and 2026-08-20) and the
fix lanes that ran off them produced ~180 findings about parts of the kernel
that play almost-but-not-quite parallel roles, code that is more complex or
more indirect than the job needs, and things that simply do not look like the
way you would do it. What remains here is **only what is still open**, plus
the register that says who may take it.

**Closed findings are deleted, not struck.** A finding whose record read
`FIXED by #NNN` (or `CLOSED`, or `VERIFIED by`) has been removed from this
file: the merged PR is its documentation, per `CLAUDE.md`, and the repo is
merge-only so every deleted record is still reachable in git. A finding that
is only *partly* closed stays here in full, with its closed part marked in its
own lead — `S18`, `S19`, `S29`, `S32`, `S58`, `S66`, `S73`, `S88`, `S110`,
`S112`, `S163`, `S168` and `S211` are the shape. (`S14` is not one of them: it
is an unruled decision for Evan, open in full.)

**So a finding ID cited below that has no heading here is a closed one, and
that is not a broken pointer** (the same rule `memories/docs-ledger.md` states
for deleted documents). Read the PR named beside the citation; where none is
named, `git log -G'^## S61\.' -- docs/SMELL-SCAN-2026-08.md` finds the commit
that removed the record and `git show <sha>^:docs/SMELL-SCAN-2026-08.md` prints
it. (`-G` matches added/removed lines; `-S` counts occurrences and would miss
the deleting commit whenever the ID is still named somewhere in the file — as
`S61` is, one line above.) The surviving sentence is written to stand without the lookup; the lookup
is for the reader who wants the evidence behind it.

**Tracks A–I are closed and their sections are gone the same way.** A, B, D
and I completed; C, E, F, G and H stopped with rows outstanding. Everything
they left is repartitioned into **Tracks J–X** below, which is the live
schedule. Their execution record is `docs/SMELL-C-LOG.md`,
`docs/SMELL-E-LOG.md`, `docs/SMELL-F-LOG.md`, `docs/SMELL-G-LOG.md`,
`docs/SMELL-H-LOG.md` and `docs/SMELL-I-LOG.md` — the rulings those tracks
made (`F-R1`…, `H-R1`…) live in those logs and are cited from here by number.

**Nothing here is ratified and nothing here is a commitment.** A finding is a
*question worth answering*, not a defect. Several findings describe deliberate,
ratified positions that a scanning agent could not distinguish from drift. The
ratified design contract is `docs/DESIGN.md`.

## How the numbering works

- **`S<N>` is a finding** — a problem statement in this document. IDs are
  stable across edits and are never reused, so they can be cited from PRs,
  issues and specs. They are not contiguous: the closed ones are gone, `S45`–`S48`
  were reserved and never allocated, and later blocks were handed out per track
  rather than in order, so a finding's number says when it was *raised*, not
  where it sits.
- **`D<N>` is a schedule row** — a unit of work in Tracks J–X. A row number and
  a finding number are different namespaces (`D40` is a row, `S61` was a
  finding), but they are **not disjoint**: 22 of the live rows are themselves
  numbered `S<N>`, colliding with a surviving finding of that number, and 17
  are numbered `C<N>`. Read a citation by where it points, not by its letter.
  A `D<N>` cited in prose with no row in Tracks J–X is **usually** a landed or
  retired row from a closed track, resolving the same way a closed finding
  does — but **not always, and the exceptions are open work**: `D65`, `D66`
  and `D67` (cited by `S121`, `S122`, `S123`) were placed by Track F and never
  carried into this partition, so they are open and unscheduled. Do not read a
  missing row as a finished one without checking. Beware one
  further collision the tree cannot spell away: `docs/DESIGN.md`'s ratified
  decisions are also `D1`–`D9`, and most prose citations in that range mean
  *those* — `D9` is determinism, not a schedule row.
- **`§C<N>` is a process observation** in §C below. **`C<N>` without the `§` is
  a Track C schedule row**, which is a different thing entirely; the collision
  is a known defect and is listed as a decision for Evan.
- **`#NNN` is a GitHub PR or issue.**

## How to read a row

Each finding carries its evidence at `file:line` or, where a fix pass has
already moved the line, at a **target name** that a reader can grep for. Line
numbers were true when written; **claims, not line numbers, are the content.**
Findings carry the scanning agent's own confidence label (`sure` / `likely` /
`unsure`) where one was recorded, a `**Verdict:**` line where Evan has ruled,
and a `**Steelman:**` line where the original justification was hunted down.
A blank `**Verdict:**` means unruled, not disputed.

A row in a Track J–X table is `| # | What | Was |`: its number, the work, and
the closed track it came from. **A row leaves that table when it lands, and its
finding leaves this document with it** — the merged PR is the record, and a
note here saying the work completed is itself a thing to delete.

## Contents

**The findings**, grouped by the scan or wave that raised them — the grouping
is provenance, not priority. Priority is Tracks J–X.

- [Tier 1 — architectural, load-bearing](#tier-1--architectural-load-bearing)
- [Tier 2 — significant](#tier-2--significant)
- [Tier 3 — real but lower stakes](#tier-3--real-but-lower-stakes)
- [Findings raised by the Wave-1 fix lanes](#findings-raised-by-the-wave-1-fix-lanes-2026-08-18)
- [The second scan (2026-08-20)](#the-second-scan-2026-08-20--the-fixes-and-the-ground-the-first-scan-never-covered), in its own three tiers:
  [1](#second-scan--tier-1--act-on-these) ·
  [2](#second-scan--tier-2--significant) ·
  [3](#second-scan--tier-3--real-but-lower-stakes)
- [Findings raised by the Track F lanes](#findings-raised-by-the-track-f-lanes-2026-08-20)
- [Findings raised by the Track H lanes](#findings-raised-by-the-track-h-lanes-2026-08-21)
- [Findings raised by the Track F, G and I lanes](#findings-raised-by-the-track-f-g-and-i-lanes-2026-08-2021)

**The schedule and the decisions.**

- [§D. The schedule](#d-the-schedule)
  - [Open decisions — Evan only](#open-decisions--evan-only) — not work; no lane may resolve one by implementing something
  - [Tracks J–X — the repartition](#tracks-jx--the-repartition-2026-08-21) — the live schedule: its rules, the twelve territories, and one table per track
  - [What this partition leaves out, said explicitly](#what-this-partition-leaves-out-said-explicitly)
  - [Last, deliberately](#last-deliberately) — the cross-cutting sweeps that go after everything else
- [§C. Process observations](#c-process-observations) — C1–C17 from the first scan, C18–C25 from the second

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
`mesh`, `topo`, `geom` and `geom-core::spline`. `Interval` — Q1's
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
`geom` and `geom-core` is inside `#[cfg(test)]`.
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
| profile `Step` verbs | `profile::Step` / `ProgramStep` / `WireStep` / `StepArg` / content-key tag table / `pncad-py`'s PATHS lattice + its `.pyi` — **6**. **Was 5 across 3 crates; the five named span TWO crates** (`profile` and `editor-core`), so the three-crate count was only ever true of a copy the row did not name. Corrected by **S170**; the `editor-core` half is closed by **S106** | `profile/src/path/program.rs` — the `transition_table!` invocation (`Step`, `Verb`); `editor-core/src/program.rs` — `pub enum ProgramStep`; `editor-core/src/persist/wire.rs` — `enum WireStep`; `editor-core/src/node.rs` — `pub enum StepArg` (**not** `program.rs`, as this row said until #836); `editor-core/src/eval/mod.rs` — `fn verb_tag`; `pncad-py/src/py/path.rs` + `pncad-py/pncad.pyi`. **Cited by name, not line** — the three numbers this row carried all pointed into text #836 rewrote |
| `RoleSeg` → `SegTag` | kernel enum → editor-core fieldless mirror → `pncad` re-export → a **second** 40-variant py mirror → 40-arm `to_kernel` → 40-arm inverse tripwire → 1316-line `.pyi` | `pncad-py/src/py/select.rs:82` |
| node kinds | ~10 parallel match tables; `rg Node::Fillet` → 24 non-test hits in 10 files | `node.rs:423`, `eval/mod.rs:1325` |
| "which `RoleSeg` args are sub-names" | 4 sites | `resolve/mod.rs:969`, `refactor.rs:540`, `names/select.rs:296`, `eval/mod.rs:2040` |
| "which payloads carry a `StableName`" | 4 lists | `edit.rs:1096`, `node.rs:949`, `refactor.rs:801`, `resolve/mod.rs:911` |
| "node has no usable value" | 5 typed + 1 stringly | `resolve/mod.rs:266`, `resolve/hit.rs:23`, `resolve/vdiff.rs:69`, `appearance.rs:172`, `names/geompred.rs:488` |
| units | 7 spellings for 6 units | `quantity/src/units.rs:47`, `expr.rs:181`, `step-import/src/units.rs:76` |
| Euler vector per op | 3 (prose, arena delta, `ep_vector`) + a 4th divergent `Ledger`; the arena delta was an unnamed positional 7-tuple until **#625** made it `ArenaDelta` | `euler.rs:891`, `euler.rs:838`, `seqgen.rs:105` |

Two of these have **already drifted, observably**:

- **CLOSED by #618, and this bullet outlived it — corrected by #731.** As
  written it says `Rebind`'s rewrite loop in `apply` matches only
  `Declare` and `Fillet` (`_ => {}`), that `refactor::payload_names`
  returns names only for those two, and that a mate head is therefore
  silently not rewritten. **All three clauses are false today**:
  `edit.rs`'s loop routes through `Node::rebind_payload_names`, and that
  function and `Node::payload_names` both carry an explicit
  `Node::Mate { a, b, .. }` arm with no wildcard, as does
  `refactor::payload_names`. The drift was real when reported; the
  record of it was left standing as live for two weeks after the fix,
  which is Q4's sub-case rather than a second drift.
- **FIXED by #632; the population was wrong and is corrected by #731.**
  Three of the sites #632 counted carried comments insisting the match is
  exhaustive "so a future variant must be classified here or the compile
  breaks"; `select::name_args` was the fourth and wildcarded, and #632
  closed it. **There were five**: `eval::anchor::remap_seg` classifies
  `RoleSeg` too and ended in a BINDING catch-all (`other => other`) that
  #632's scan could not express, so it was neither counted nor fixed. All
  five are now exhaustive on the `RoleSeg` axis, and the four that see a
  `Qualifier` are exhaustive on that axis too. **Five is #632's population
  plus that one, not the tree's**: a probe `RoleSeg` variant demands a
  decision at **seven** matches on today's head — those five, plus
  `eval::feed_role_seg` and `resolve`'s `visit`, which were already
  exhaustive and so were never anyone's residue. The distinction is the
  one this row is about, so it is spelled rather than left to the count.

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
| node kinds (~10 tables) | **DOES NOT SURVIVE as stated** — 10 operations over a 12-variant sum type is the design working. Re-scoped to *wildcard* arms it survived, and was measured (2026-08-18) as `node.rs` 9, `eval/mod.rs` 5, `resolve/mod.rs` 4, `edit.rs` 3, `refactor.rs` 3, `persist/check.rs` 2 — **a count whose counting rule was never stated, so it cannot be re-derived**. #731 closed seven of the arms it covers (`persist/check.rs` both, `resolve/mod.rs` one, `node.rs` three, `eval/mod.rs` one) and did not re-cut the measurement, because there is no instrument to re-run. Whoever takes this row states the rule first. The live instrument for the class is `--force-warn clippy::wildcard_enum_match_arm`, whose blind spot is recorded at **§C15**. **The count above is stale and is left as a dated record, not as a claim about today's tree.** |
| `RoleSeg` arg sites | **SURVIVED IN PART; FIXED by #632, population corrected by #731.** The sites answer genuinely different questions and *should* differ — what survived was the fourth site's wildcard, closed by #632. The count was wrong: a **fifth** classifier, `eval::anchor::remap_seg`, partitions `RoleSeg` by profile-locator carriage and was fail-quiet until #731. #731 also gave the one partition three of them share — the name-free variants — a single home (`names::role::name_free_seg!`), so it is one decision rather than three that can diverge. |
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

*Drift (b) CONFIRMED, **FIXED by #632**, and its residues **FIXED by #731**
(H11).* The `Fragment(SideOf)` disagreement is **documented and intentional**
and was preserved. #632 disclosed two residues; #731 closed them, and closed
**eight further fail-quiet classifications of the same shape in the same
crate** — two that #632's own body had ruled out in writing, five that
neither #632 nor #731's first pass found, and one that **neither of #731's
own instruments could express**, found by its verification pass. The class as
reported was *"#632's two residues"*; the class as it existed was **ten**.
That gap, not either fix, is the finding — and the count moved three times
(two, four, nine, ten), each time because a differently shaped instrument
was run, which is §C15's clause arriving as a measurement rather than as a
warning.

- `resolve::apply_with_names`' `DocEdit` wildcard is now three named groups —
  checked, name-carrying-and-deliberately-unchecked, name-free — which encodes
  a policy the docs at both ends had already ratified in prose. **The same
  wildcard, on the same enum, was live a second time** at `persist/check.rs`'s
  `edit_non_finite`, the load door, which #632 set aside as *"different enum"*
  and is not; a future float-carrying `DocEdit` variant would have been
  classified "no floats". Both are closed, with `param_site`'s `DocParam`
  wildcard beside them. A probe variant carrying a name and an `f64` broke one
  site before and three after.
- The triplicated 17-variant name-free list is one `name_free_seg!()`
  or-pattern macro in `names/role.rs`. A pattern rather than a predicate,
  because a predicate is a thing a caller may forget to call (**C-R5**); every
  match stays exhaustive, and the delta is that "name-free" is one decision at
  one site instead of three that can diverge. #632's stated obstacle — that
  `role.rs` has no `impl RoleSeg` block to hang a classifier on — is not paid,
  because a macro is the list rather than a classifier.
- **`eval/anchor.rs`'s `remap_seg` was a live fail-quiet #632's scan could not
  see**, contradicting its *"no fail-quiet wildcard in any `RoleSeg` or
  `Qualifier` match in the workspace"*. Its catch-all was `other => other` — a
  BINDING wildcard, not `_` — in a match written entirely through
  `use RoleSeg as R`, so the literal `RoleSeg::` never appeared in the window
  the scan required. Its twelve explicit arms are exactly the twelve variants
  embedding a profile locator, so behaviour was right; a thirteenth carrier
  would have crossed a profile re-anchor with a **stale locator**, silently.
  See **§C15**, which this corrects.
- **The five #731's own first pass missed, found by its style review**, all
  in `editor-core` and all closed under **C-R17** (orchestrator, 2026-08-20: fix in-crate residues rather than reporting them). The one that carries the
  argument is `eval::content_key`'s structural-payload match: its **tag**
  half was exhaustive and its **payload** half wildcarded, so the compiler
  forced a decision at one half of one key and silently defaulted at the
  other. A `Node` variant whose recipe payload lives outside its slots would
  hash identically to one that differs in it, and the memo would serve
  another node's geometry — which is not hypothetical, and is recorded two
  paragraphs below as `Step::AtToward`'s tag-28 collision, *caught by a
  reviewer, not by a type*. The others: `Node::expr`/`expr_mut` and
  `placement_rule_fault` on the `Node` axis, and `Expr::child` and
  `with_replaced` on the `ExprKind` axis. `node.rs`'s two nameless-payload
  lists — #618's own growth of the same duplication — and `expr.rs`'s four
  arity lists took the same treatment as the triplication above
  (`name_free_node!`, `binary_kind!`/`unary_kind!`/`leaf_kind!`).
- **The tenth, found by this PR's verification pass, and it is the one
  neither instrument could see.** `doc.rs`'s `DocParam::bit_eq` matched
  `(self, other)` and closed with `_ => false`. Invisible to the type-aware
  instrument because the scrutinee is a **tuple** — its declared blind spot,
  exactly — and invisible to the arm-content instrument because the arms are
  spelled `Self::Continuous` / `Self::Count` and name no enum, which is the
  **same alias tell** as the `use RoleSeg as R` miss that started this. Its
  consequence is a wrong value, not a missed check: a future `DocParam`
  variant would have compared unequal to ITSELF, and `Doc::bit_eq` and
  `diff.rs` would have read that answer — D7's replay identity and the
  document diff both reporting a change that is not there. Closed
  exhaustively on both sides of the pair; a probe `DocParam` variant carrying
  an `f64` breaks **4** sites on the head against **2** on `origin/main`.
- **What the second sweep changed about the method.** #731's first
  instrument was type-aware (`--force-warn
  clippy::wildcard_enum_match_arm`) and it *declared* a blind spot — an
  enum nested in `Option`/`Result`/a tuple is attributed to the outer type —
  and then did not compensate for it, which is the exact failure **§C15**
  names and which #731 was simultaneously correcting C15 about. The
  compensating instrument is **arm-content-directed**: find every `match`
  whose ARMS mention a target vocabulary's variants and which has a
  catch-all, `_` or a bare binding, never consulting the scrutinee's type.
  It is self-tested — run against `origin/main` it re-derives
  `eval::anchor::remap_seg` unaided, through both the alias and the binding
  catch-all. **That property was re-verified, but not that script**: no
  sweep script ships with the fix, so the verification pass wrote its own
  instrument of the same family (whole-file brace tracking rather than a
  window, `if let`/`while let`/`matches!` as well as `match`, keyed on every
  enum-variant name in `crates/` rather than a target list) and it re-derives
  `remap_seg` on `origin/main` too. A claim that cannot be re-executed is
  weaker than one that can, and this one is the weaker kind: what stands is
  the property, held by a second instrument, not the run. **Its own blind spot**: a brace-balance window, so it
  false-positives across a long `impl` (it flagged `pncad-py`'s
  `select_refusal_tag`, whose wildcard is forced by `#[non_exhaustive]` and
  compensated by a pin test), and it cannot see a classification written as
  `if let` / `matches!` chains rather than a `match`.

- **`evaluate`'s bound, restated in five places, three of them uncounted.**
  Raised by **H-g** (2026-08-21) while measuring #883's `CertifiedBounds`
  cascade. `editor_core::eval::evaluate`'s where-clause and the named
  `EvalScalar` alias restate the same term list ten lines apart in
  `eval/mod.rs` — and that pair is **already known and already pinned**, in
  both directions, by `editor-core/tests/e4_dual_door.rs`. What nothing
  watches is the other three: **`editor-core/tests/corpus/mod.rs:196`,
  `topo/tests/fixture/pr4.rs:36`, `editor-core/tests/m5_pr8_bvh_diff.rs:49`**,
  each a hand-copy of the same list in test support. They surfaced only
  because a bound change stopped them compiling — S4's usual tell, arriving
  as a build break rather than as a drift report. **The asymmetry is the
  row**: the file that exists to detect exactly this drift covers two of the
  five sites, so its own coverage claim is a member of the finding it
  detects. `corpus/mod.rs` is `mod`-included by **22** test files, so the
  single cheapest copy to miss carries the widest blast radius — 78
  diagnostics from 5 sites, when the cascade was measured.

*One confirmation this report did not cite: the hand-synced tag table has
already produced a live measured bug.* `MODEL-AB-LOG.md:782` — *"**MAJOR-1 =
`Step::AtToward`'s memo content-key tag 28 COLLIDED with `ArcContinue`'s
existing 28 — latent memo collision, a hit would serve wrong geometry**"*.
Caught by a reviewer, not a type. S4's failure mode, realised.

*Ranked cheapest-to-hardest to act on:* (1) `BooleanOp` → import +
`serde(with)` — **DONE, #642** (cheaper than listed: the `From` pair was
already the identity, so nothing had to be preserved; dearer in one place
the ranking could not see — the read direction needs a run-time write-door
check, because safe Rust cannot make it exhaustive); (2) `name_args`' wildcard → exhaustive — **DONE, #632**, with the fifth
site it could not see **DONE, #731**; (3) the `Mate` arms —
small but a **behaviour** fix — **DONE, #618**, and the drift bullet above
went on describing it as live until #731; (4) the Euler 7-tuple → named struct
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
`crates/topo/tests/probe_s5_sectors.rs`, is **committed and type-checked
but not run by CI**: it is `#![cfg(feature = "probe")]`, so since D17
(#739) the `k-lint` job's *"compile and list every probe-gated test target"*
step compiles it — `cargo check -p topo --features probe --all-targets`
over a census derived from the tree — but nothing
in `.github/workflows/` *runs* `cargo test -p topo --features probe`.
So it is a *reproducible hand-run* artifact, not a standing gate: it can
no longer rot into a build error, but its recorded stream can still
drift green. `tests/probe_census.rs` is in the same position. (This
paragraph originally named `tests/probe_f34_review.rs` alongside it;
that file carries **no** `cfg(feature = "probe")` gate and has always
been compiled and run by the default rows — the claim was wrong when
written, and D17's census is what surfaced it.) The standing gate over
the same stream is CI's `k-lint`, which runs the full
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
| `boxes` modules on **both** halves of `geom` | `geom/src/curves/boxes.rs:29`, `geom/src/surfaces/boxes.rs:25` | Zero production consumers, while `topo/boolean/boxes.rs` carries a KNOWN GAP note saying "the sound constructor exists unused" (see S16) |
| `Node::Sweep` | `eval/wire.rs:1524` | Full vocabulary entry — variant, 2 `SlotId`s, content tag, `inputs`/`slots`/`expr` arms — for an op with no success path |
| STEP cylinder recognition | `recognize.rs:257`, `:794` | ~90-line estimator whose own test `p7_exact_cylinder_envelope_is_honest` asserts an exactly cylindrical patch must **not** promote; `PromotedKind::Cylinder` asserted as an outcome nowhere in `src` or `tests` |
| `ProfileError`'s five fillet variants | `profile/src/validate.rs:411`–`:507` | Constructible only from `test_support.rs`, behind the `test-support` feature; `Profile::validate` cannot produce any of them |
| `Mat2` / `Affine2` — **DELETED by #721**, see the D4 DECIDED block | `geom-core/src/linalg/mat.rs:21`, `affine.rs:17` (pre-deletion) | Only mentions outside `linalg/` are the re-export and one review test; `Vec2`/`Point2` are heavily used, so it is the 2-D *linear-map* half specifically that is dead |
| `Vec2::unit_x`, `Vec2::unit_y` | `geom-core/src/linalg/vec.rs:44`, `:49` | **Minted as a row by #721, the PR that closed the one above.** Their only `src` consumer in the workspace was `Mat2::identity`; the sole remaining caller anywhere is one line of their own module test (`vec.rs:469`). No verdict — delete-or-keep has not been asked |
| `PatchContact` | `boolean/mod.rs:214` | No producer; `ContactRecords.patches` and its face-lineage chase in `remap_contacts` are paths no run reaches. Deliberate per its doc ("the vocabulary is complete") |
| `trace_plane_nurbs_uncertified` | `ssi.rs:970` | Demonstration entry point in `src/`, test-only, re-copying ~40 lines of `plane_nurbs_ssi`'s setup |
| `FlipSet::flips_on_path`, `flips_at` | `resolve/vdiff.rs:181`, `:150` | Public, documented against a spec line, no callers; the one consumer that wants exactly this calls the raw primitive underneath |
| `jet.rs` cone and torus Taylor arms | `ssi/jet.rs:274` | Own comment: "the cone arm refuses at both the enclosure and the certificate today, so nothing depends on it yet" |
| `corner_contact_circle`, `trimline_description` — **DELETED by #748**, see the D4 DECIDED block — and `CornerBall`'s `surface` field, which stays | `fillet/blend.rs:301`, `battery.rs:1100` (both pre-deletion), `blend.rs:56` | The first builds a chart both production callers discard and replace; the other two have no consumer at all |
| `PairSolve` — **DELETED by #735**, see the D4 DECIDED block — and `CLASS_DEFERRAL` | `mate/solve.rs:66` (pre-deletion), `mate.rs:254` | `pub`, re-exported, never constructed or read |

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
| `Mat2`/`Affine2` — **deleted by #721** | **GENUINELY DEAD?** — the only row with **no source at all** for a future consumer, against the M0 review's own norm *"add only on consumer demand"*. |
| `PatchContact` | **PLANNED — the strongest row.** Producer is spec-written and queued: `ASM-R2B-SPEC.md` D-2 mints it; `M9-3-SPEC.md:143` is its acceptance row. |
| `trace_plane_nurbs_uncertified` | **DELIBERATE-FRONTIER** — `M5-PR7B-SPEC.md:71` authorised exactly this fate. |
| `flips_on_path`/`flips_at` | **DELIBERATE-FRONTIER** — `M4-PR4-SPEC.md` D2 mandated a general engine: *"must not be specialized to either consumer"*. |
| `jet.rs` cone/torus arms | **DELIBERATE-FRONTIER**, structurally forced by the closed `Surface` enum (D3); the refusal is downstream and reasoned. |
| `CornerBall::surface` — **not deleted; census moved**, see the D4 DECIDED block's row-3 paragraph | **SUPERSEDED** — both callers replace the chart, with the reason in code. |
| `corner_contact_circle`, `trimline_description` — **deleted by #748** | **SUPERSEDED** — inlined duplicates; the spec's obligation is met by the inline form. |
| `PairSolve` — **deleted by #735** | **GENUINELY DEAD?** — written by ASM-R2a, unmentioned by ASM-R2b, which is the unit that consumes the solve. |
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
`geom-core/tests/spline_hull.rs`, 3 in `geom/tests/curves/review_m5_pr2_e2e.rs`),
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
| The two fillet helpers | the fillet thread — **#319** / **#554** | Both are live fillet issues, and `trimline_description`'s doc is the only place D7's prefer-intrinsic obligation is *named*, so that sentence migrates with the note rather than dying. **The last clause was wrong** — `CURVED-DESIGN.md:735` and `DESIGN.md:691` both name it; what was unique to the doc comment, and what #748 migrated, is the invariant stated *at the code seam that builds trimlines*. The obligation to migrate stood; its justification did not. |
| `Mat2`/`Affine2` | the deleting PR body, cross-referenced from **#614** | M0's linalg thread is closed and archived — there is no live thread to annotate, which is itself the strongest evidence for the row's `GENUINELY DEAD?` sort. |

Per §C3, a note in a prose register is not a deferral but a forgetting on a
schedule — so the deleting PR must cite the commit SHA the code is recoverable
from, not merely say "see history".

**EXECUTED — row 1 of 3, `Mat2`/`Affine2`, by #721 (2026-08-20).** The census
was re-run on that day's tree before anything was deleted, and had not moved
since 2026-08-18: outside `linalg/` the only mentions were still `lib.rs:38`'s
re-export and `geom-core/tests/review_m0_pr5.rs`. **Recoverable from
`9f559f6a4179a77a87d569bc0b8f363fa1cf2c1a`**, whose tree carries both types in
full; the provenance note is #721's body and a comment on **#614**, per the
table above. Three things that *referred* to the 2-D types while being about
something else migrated rather than died — `Affine3`'s `Mul` doc and its
worked-example test, both re-expressed in 3-D, and `Mat3`'s Rodrigues
cross-check, which now builds its reference block from `sin_cos` directly.

**The deletion minted a row of the same class it closed.** `Vec2::unit_x` and
`Vec2::unit_y` (`vec.rs:44`, `:49`) had exactly one `src` consumer in the
workspace — `Mat2::identity` — so after #721 the only caller left anywhere is
one line of their own module test. They are tabled above without a verdict.
The class to watch is **anything in `Vec2`/`Point2`'s surface whose last live
caller was 2-D-map code**, and the general lesson is that a deletion's census
pattern is scoped to the deleted names, so it structurally cannot see what the
deletion orphans. #721's pattern was `mat2|affine2`; that is its blind spot,
per §C15. The follow-up sweep that found this pair matched *qualified*
associated-function calls (`Vec2::foo(`) across the workspace and found no
other orphan — `new`, `zero` and `origin` all keep outside callers. It cannot
see method calls (`v.dot(w)`, `p.distance(q)`), whose `Vec2`-vs-`Vec3` receiver
needs type inference rather than grep; the method half of the surface is
therefore unswept.

**EXECUTED — row 2 of 3, `PairSolve`, by #735 (2026-08-20).** The census was
re-run on that day's tree before anything was deleted, and had not moved since
2026-08-18: workspace-wide, `PairSolve` was still exactly two lines — the `pub
struct` at `mate/solve.rs:66` and the re-export at `mate.rs:66` — with no
constructor and no reader. The re-run widened the census past `crates/*/src`
to the whole tree bar `.git` and `target`, so `crates/pncad-py/`, every
crate's `tests/`, `demos/`, `tools/`, `scripts/`, `review/` and
`review-probes/` are covered; outside `docs/` there were no other hits, in
either spelling (`PairSolve` or `pair_solve`, matched case-insensitively).
**Recoverable from `adbeff0976dd6d2fc1a0a4f663aef965f01f8dd7`** —
*ASM-R2a: the Mate node, the coset algebra, clusters and the solve*, the
commit that wrote it — whose tree carries the type in full, checked by
reading the struct out of that tree rather than trusting `git log -S`.
The authoring commit is the citation #611's successor wants: the shape and
the intent that produced it sit together there. The provenance note
is on **issue #611**, per the table above. It is two comments: the note itself
(`#issuecomment-5353012588`) and the **authoritative** correction carrying this
SHA (`#issuecomment-5353068751`). Cite the second — no tool in the lane's
surface can edit a posted comment, so the first still names a weaker SHA.
Nothing migrated: the type had no `impl`, so it read nothing, and its three
field types (`RecipeNodeId`, `Vec`, `Coset`) all keep consumers.

**This deletion could orphan nothing, and that is provable by inspection
rather than by sweeping.** `PairSolve` was a field-only struct — three fields,
`#[derive(Debug, Clone)]`, no `impl`, not one method call inside it. Its
*entire* reference footprint was therefore three type names: `RecipeNodeId`,
`Vec`, `Coset`. That closes the set of things its removal can strand, and all
three keep consumers (`Coset` alone has seven further uses in `solve.rs`, plus
`editor-core` and `pncad` tests). Row 1's trap — a deletion orphaning a
helper whose last caller was the deleted code — **has no purchase on a type
that called nothing**. Row 1's lesson still holds in general; it simply does
not bind here, and a grep-shaped sweep was never what established that.

**The instrument that did establish it, and what it found.** Grep answers
liveness only approximately: it matches bare identifiers, so a method name
that collides with a common one elsewhere reads as consumed whether or not its
own callers survive. The exact instrument is **rename-and-rebuild** — rename a
definition in place, rebuild, and let the compiler enumerate the consumers.
Run over all sixteen `pub` items of `mate/coset.rs` and `mate/solve.rs`
(`cargo check -p editor-core -p pncad --all-targets`, with `SolvedPoses::role`
planted first to prove the instrument can go red), it found **one dead item:
`MateRole::name`**, ten lines above the deleted struct, documented *"The
role's name, for messages"*, with nothing anywhere formatting a role into a
message. It builds clean without it under default features, under
`--features probe`, and for `pncad-py`. R2-a wrote it too, so
`adbeff09` above is its recoverable commit as well. **Deleted here**: dead in `mate/` is
the class D4 ruled on, so removing it applies the ruling rather than making a
fourth decision. Every other item answered LIVE, including all three
`MateRole` arms — `Determining` in production at `solve.rs:666`, `Declaring`
at `:535`/`:639`, `Refused` at `:554`/`:584`. **The instrument's own limit:**
renaming a *type* always answers LIVE, because its own `impl` block and fields
count as consumers, so it measures functions and methods exactly and says
nothing about type-level deadness.

**Two items tabled without a verdict** — found by the instrument, but a sweep
of newly-discovered rows is a new decision, not an application of D4's:

- `Subgroup::name` and `Subgroup::dimension` (`coset.rs:159`, `:147`) have
  **no production caller**: rename either and the lib-only build is clean;
  every consumer is in `editor-core/tests/asm_r2a_mate_solve.rs`. (`mate.rs:428`
  and `:479` call `MateSide::name`, a different type's method — not these.)
- `fold_pair` (`solve.rs:436`) is `pub` in `pub mod solve` but named by no
  `pub use` in `mate.rs`. This is **facade completeness**, not a visibility
  bug: `mate` presents a curated re-export surface, and four of its
  submodules' public items escape it under no stated rule — while the deep
  path is a *used* idiom here, `asm_r2a_mate_solve.rs:725` calling
  `mate::coset::intersect_subgroups` by full path. Whether the facade is meant
  to be complete has not been asked.

The class behind `MateRole::name` is placed as **D24**: a `pub` item that is
dead workspace-wide is exempt from `dead_code`, and the workspace lints carry
only `unsafe_code`/`missing_docs`, so nothing mechanical catches one.

**EXECUTED — row 3 of 3, the two inlined fillet helpers, by #748
(2026-08-20). D4 is discharged in full.** The census had **moved**, so it was
re-derived rather than trusted: this row's three-name cell was written on
2026-08-18, before #688, #705 and #740 rewrote the neighbourhood. What
`corner_contact_circle` (`blend.rs:301`) and `trimline_description`
(`battery.rs:1100`) still had in common was zero callers, confirmed by
rename-and-rebuild rather than by grep. **Recoverable from
`60941420a979a99346893f024b052fa430d4a3db`** — *"fillet: the validity battery
(six fillet3_* predicates), analytic blend arms, OQ6 refusal vocabulary"*,
2026-08-02, which authored both in one commit and whose tree carries them in
full, read out rather than inferred. The provenance note is split across the
fillet thread per the table above — `blend.rs`'s two items on **#319**, the
C8 arm-table tracker, and `trimline_description` on **#554**, which is live in
`battery.rs`.

**The obligation migrated, and it is not the one this document said it was.**
`trimline_description`'s doc named *"prefer-intrinsic, D7"* — that is
**`CURVED-DESIGN.md` §D7**, whose fifth leave-room obligation
(`CURVED-DESIGN.md:735`) already states it in full, and the rule itself is
`DESIGN.md`'s prefer-intrinsic paragraph under **D2** (`DESIGN.md:691`), which
names the fillet case by name. So the claim above — that the doc comment is
the *only* place the obligation is named — was **false**: it is named twice in
the design corpus, and neither statement was at risk. What the doc comment
uniquely carried was the invariant **in the code, at the seam that builds
trimlines**, and that is what moved: `attach_contact` (`surgery.rs:1799`, the
live inline site) now carries the born-with rule, the never-a-`MappedCurve`
half, and pointers to both design statements. A reader at the construction
site can now reach the obligation; before this PR only a reader of a function
nothing called could.

**One orphan, and the compiler found it for free.** Deleting
`trimline_description` left `use geom_brep::EdgeGeometry` unused in
`battery.rs` — an instance of exactly the class row 1 minted and row 2 argued
itself out of. It is worth recording *how* it surfaced: not by any sweep, but
by `-D warnings` on the next build. For an item a deletion orphans **inside
the same crate**, the compiler is the complete instrument, and no census
pattern is needed. Row 1's orphan escaped only because `Vec2::unit_x` kept a
test caller and so never went unused.

**A second `pub`-and-dead `name()` — D24's class, verified again.**
`BlendArm::name` (`blend.rs:87`), documented *"The arm's name, for refusal
text and report rows"*, is dead workspace-wide by the same instrument. It is
**deleted here** on the same reasoning that took `MateRole::name` in #735:
dead, in the files this row's scope names, in the class D4 ruled on. Two
independent instances now — one in `editor-core/src/mate/`, one in
`sweep/src/fillet/` — each a `name()` for messages that no message uses, each
found only because a lane happened to open the file. That is D24's argument,
and it no longer rests on one row.

**Tabled without a verdict, because the census moved under it:**
`CornerBall::surface` was cited in this table beside the two helpers, sorted
**SUPERSEDED** on the evidence that *"both callers replace the chart"*. Today
it is a struct **field**, not a method, and it has **one** production caller
(`surgery.rs:461`), which still discards it and rebuilds `Surface::Sphere`
from `octant_chart`'s axis and `u_ref`. Evan's ruling named the two helpers,
not this, so it is not deleted. Note also that this is precisely where the
instrument is blind: **rename-and-rebuild is silent on struct fields for the
same reason it is silent on types** — the struct literal in `corner_ball` is
itself a consumer. It was checked by reading the one caller.

## S13. Load-bearing invariants held by CI grep, allowlists, and a magic count

- **Where**: `scripts/gates/bounds-allowlist.sh`,
  `scripts/gates/interval-square-allowlist.sh`,
  `scripts/gates/no-extra-real-bounds.sh` (all three were inline in
  `.github/workflows/ci.yml` at `:322`, `:420`, `:444` when this was
  written; #626 moved them),
  `crates/geom-core/src/real.rs:348`,
  `crates/geom-core/tests/flagged_census.rs` (the count is
  `LEDGER_FLAGGED_SITES`; it was `:20` when this was written and moved
  when #801 rewrote the scan around it)
- **Confidence**: sure

The disciplines the design leans on hardest are text-matching CI steps:
a `Bounds` compound-bound gate with thirteen hand-maintained path
regexes; an `EvalScalar` gate ("the Bounds gate, over the name");
`grep -rnE '\bReal\s*\+'` to catch extra bounds on evaluation type
parameters; a `\b([a-z_]\w*)\s*\*\s*\1\b` regex banning `x * x` with its
own file allowlist; an env-read ban; and a test that counts
`decide_flagged` call sites across `crates/*/src` and asserts the total
equals a hand-written **8**. **#801 narrowed this last one without
retiring it**: the scan no longer keys on the literal string
`k_stats::decide_flagged(` — which a site spelled bare after a `use`
evaded — and a second assertion now requires each site's `ledger_row`
argument to name a row that exists in
`docs/predicate-dimension-audit.md`, so the *citations* are computed
rather than trusted. **The count itself is still the magic one**: `8` is
a literal in the test, hand-synced with the audit's own prose, derived
from nothing. That half of this row stands unfixed, and #801 says so at
the constant.

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

**S14(a) IS CLOSED — all three halves are in (#845, #846, #848) — and
S14 STAYS OPEN, with its problem statement, because S14(b) does not.**
#823 split this row into **S14(a)** (the `Span`/`KnotVector` pairing) and
**S14(b)** (the graft, of which §S70 is the documentation residue);
Evan's ruling on #823 then split S14(a) itself. State:

- **S14(a), half one — #845, done:** `NurbsSurface::window_of` is
  private, so the argument-order hazard is no longer spellable outside
  the module that mints windows, and the doc comment conceding it is
  gone with the door.
- **S14(a), half two — #846, done:** `KnotVector::admits`, asked at
  `basis_funs`, `ders_basis_funs`, `hull::span_indices` and the four
  `NurbsCurve{2,3}::*_in_span` families. The ruled two compares
  (`degree` agreement, `index <= last_span`) close every out-of-bounds
  read; a **third**, `span_is_nonempty(index)`, was added on review and
  closes the exit that is worse than the panic — an empty foreign span
  reads no knots on the hull path, so `sup_norm_bound_span` returned a
  *finite wrong* bound and the C2.2 honesty limb **certified a span it
  never bounded**. It also ended a state in which two `pub` predicates
  on one `KnotVector` disagreed about one index (`span(i)` refusing
  what `admits` accepted).
- **S14(a), half three — #848, done:** `NurbsSurface::admits` — both
  directions' `KnotVector::admits` plus
  `win.stride() == knots_v.control_count()` — asked at the three `pub`
  `NurbsSurface::{eval,ders,ders3}_in_span` doors, which took an
  unbranded `SurfaceWindow` and indexed `self.control`/`self.weights`
  off its foreign `base`. **Neither of the other two halves reached
  it**, and the asymmetry is the part worth carrying: `window_at` is a
  *total* public mint, so #845 does not touch it, and #846 poisons the
  basis ROW while the loop bound comes from `self.knots_*` and the flat
  index from the window — **poison in a row does not stop an index**.
  Demonstrated as a panic first (`index out of bounds: the len is 9 but
  the index is 9`, `self.weights[idx]`), then inverted;
  `stride` shown necessary as well as sufficient. **The stride compare
  is the term with no one-dimensional analogue**, and it is what makes
  an admitted window bit-identical to one this surface would have
  minted itself.
- **S14(b) — open and unscheduled.** Row 0 reframed its first question
  and did not answer it.

**The residue #846 and #848 accept, deliberately, in both dimensions:**
two vectors — or two surfaces — of equal degree and equal control count,
whose index `i` is a nonempty span in both, admit each other's span `i`
and window, so evaluation is a wrong answer rather than a refusal. That
is the same species as `hull::span_indices`'s length-only `coeffs` check
one line away, and it is stated at both types rather than implied away.
Closing it wants the brand, not a compare, and nothing here pays for
one.

**The `Where` citations below are the ORIGINAL ones and none of them
resolves to what it quoted** — #846 rewrote all three module docs,
including `hull.rs:80`'s self-declared panic, which was this row's own
evidence, and #848 rewrote `SurfaceWindow`'s not-branded concession,
whose *"panics (loudly, correctly)"* was the surviving copy of the same
sentence, together with the two relocated copies of it that #846's own
sweep and #848's first pass both missed (`knots.rs`'s `Span` doc, which
pointed AT the surface hole, and `geom/tests/surfaces/m5_pr7_ders3.rs`'s
suite header). **S14(a) has no live claim site left.** What stands is
S14(b): §S70's graft footnote, unscheduled.

**CHANNEL and decision record: #823**, the design conversation this row
was ruled from. S14 had no channel from 2026-08-18 to 2026-08-20 and its
context was scattered across §S14, §S70, issue #475, `DESIGN.md`'s D9 and
row 0, and a deleted `M0-LOG.md`; #823 assembles it, re-derives every
cost at its own base, and argues the strongest case against its own
recommendation — which is the section Evan ruled from, against that
recommendation.

**#823's contribution to this row is the split, and it is why the row is
now tractable.** **S14(a)** is the `Span`/`KnotVector` pairing in
`geom-core`/`geom` — the finding below, `hull.rs`'s self-declared panic,
and issue #475. **S14(b)** is `instance`'s graft leaving a
partially-written destination — the second witness below, of which §S70
is the documentation residue, and the half that bounds #740's fillet
lookups. `DESIGN.md:1358-1375`'s ratified row-0 paragraph about "S14"
describes **S14(b) only**; it names neither `Span` nor #475. The two
halves share a shape and nothing else, and neither blocks the other. The
ruling then split S14(a) again — a redundant door and a missing
two-integer check are separate defects, and bundling them is what made a
single lifetime-carrying redesign look like the only complete answer.

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

> **Dated-correct, now superseded (noted 2026-08-21).** The third clause
> quotes D9's *"typed errors where cheaply detectable, or documented
> garbage-out in release"*, which the **D2 addendum SUPERSEDED on
> 2026-08-19** — a day after this steelman was written — replacing it with
> the six-row taxonomy whose rows 4 and 5 make a panic the ratified answer
> for a bug state. The steelman's conclusion is untouched; only the
> disposition list it cites has moved. Kept rather than rewritten: the
> record is what it was, and the pointer is what it needs.

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
wrong answer through a public method with no bug involved. **That method
is private as of #845 and the quoted concession no longer exists**; the
two public mints take indices (`window`) or parameters (`window_at`),
neither of which is a value advertising that it has been validated. The
paragraph is otherwise unchanged — it is the statement of the finding,
not a claim about today's tree.

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

## S18. Certified numeric derivations duplicated across crates (roll-up) — the knot-vector rows PARTLY CLOSED by #744; four rows live, plus two the closing spawned

- **Confidence**: sure for each row
- **Partly closed, and precisely how.** #744 gave *"Distinct interior knots"* one home and closed *"Knot insertion"*'s **span** half. It did **not** close the coefficient fork (deliberate, permanent, now documented at both sites), the second span search for non-representable derived vectors (**D30**), or the consumer routine duplicated on top of the query (**D31**). The two quotient-rule rows, the bulge-arc row, the prefer-intrinsic row and `step-export/volume.rs` are untouched and **live** — the last of those is Track C's **C3**, not this row's to close.

| Derivation | Copies | Anchor |
|---|---|---|
| Rational quotient-rule interval assembly | 2 crates (`ssi/enclose` vs `mesh/nurbs_cert`), sharing only the `spline::hull` primitives; a soundness fix in one is invisible to the other | `ssi/enclose.rs:417`, `mesh/nurbs_cert.rs:1039` |
| …and again *within* `mesh`, curve vs surface | The curve version's doc says "the face bound's quotient-rule assembly one dimension down" | `mesh/chords.rs:363` |
| Bulge-arc closed form (ratified convention) | 3 | `edge_geometry.rs:146`, `profile/seg.rs:143`, `sweep/skin.rs:306` |
| Knot insertion | **SPAN HALF CLOSED by #744; the fork STAYS.** The raw `&mut Vec<f64>` path calls the same search `find_span` does, under a stated precondition. The **coefficient arithmetic is a permanent deliberate fork** — a `CurvePlan` of `Step`s over f64 weights vs an in-place `RingInterval` fold whose α rounds outward — and both sites now say so in their own docs. Not one search in the tree: one for **clamped** vectors, plus `geom-brep`'s `props::quad::raw_span` for the derived vectors `KnotVector` cannot represent (**D30**) | `spline/knots.rs` (`span_offset_in`, `find_span_in`) |
| "Distinct interior knots with multiplicities" | **PARTLY FIXED by #744** — the census was **eight**, not "≥4". Two copies reduce the scan to a predicate (`mesh/chords.rs`, `mesh/nurbs_cert.rs`), which no search for an "interior" *helper* reaches; one runs over the **whole** vector (`step-export/writer.rs::run_length_knots`, self-declaring the duplication in its own doc), which no search keyed on the interior bounds reaches; and one is `KnotVector::clamped`'s own validation, 180 lines from the home. All eight now call `runs_in`, through `interior_knots()` / `knot_runs()` or directly. **What did NOT close: the consumer routine those queries feed** — `sweep::make_compatible` and `geom::fit::deviation_from` are one union-and-refine routine in two crates (**D31**) | `geom-core/src/spline/knots.rs` (`runs_in`) |
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
- **Knot insertion ×2 — FIXED by #744 (the span half).** Two PRs the **same
  day** (#125, #136); the scalar type (`RingInterval` vs f64) was the stated
  reason, and the linear span scan fell out of not having a `KnotVector` to call
  `find_span` on. **NEVER FLAGGED.** #744 found the two searches were **not the
  same function**: the linear scan agrees with `find_span` on `u ∈ [lo, hi)` and
  nowhere else — at or above the domain end `find_span` returns the last span
  while the scan runs into the trailing clamp, and below the domain (or at NaN)
  the scan returns its *initialiser*, `0`, which is not a located span at all.
  The substitution is therefore stated with its precondition at the frame that
  makes it, as an `unreachable!` (D2 addendum row 4 — observed, not re-derived),
  and `span_offset`'s body moved to a free function both paths call so there is
  one search rather than two that agree. The **coefficient arithmetic was
  deliberately left forked**: one emits a replayable `CurvePlan` of `Step`s over
  f64 weights, the other folds `RingInterval`s with an outward-rounding ring
  quotient and has no weights at all.
  *Lesson: "the same derivation, twice" and "the same function, twice" are
  different claims. Only the first was true here, and unifying on the strength
  of the second would have been a wrong curve at the domain end.*
- **"Distinct interior knots" — FIXED by #744, and there were SIX.** The dating
  was re-checked against the full history (`git log -S`, the clone was shallow
  when this row was written) and it is **sharper** than the row claimed: the
  `KnotVector` type landed `a0c58ee5`, **2026-07-27 01:48**, and the first
  hand-written scan landed `24442408` the **same day at 23:58** — twenty-two
  hours, the very next PR. Four more followed within five days (`67aa508d`
  07-28, `2bd911d5` and `92250b85` both 08-01); the sixth, `5484ecbf`, came at
  nine days. The accessor set was frozen *before any consumer existed*.
  `multiplicity_of(u)` requires you to already know `u`, so every consumer
  needing *the list* hand-wrote the same scan. **NEVER FLAGGED** — by the fourth
  copy nobody was tracking, and the row's own count of "≥4" was two short: this
  scan's fifth and sixth copies (`mesh/chords.rs`, `mesh/nurbs_cert.rs`) reduce
  it to a max-multiplicity predicate instead of collecting it, so no search for
  an "interior" *helper* could reach them. **Two review passes then found two
  more**, and the count is **eight**: `step-export/writer.rs`'s
  `run_length_knots` (a fifth crate, three call sites, all handing it a
  validated `KnotVector`'s knots — and self-declaring the duplication in its own
  doc, the shape this postmortem already says nothing reads), and
  `KnotVector::clamped`'s **own** interior-multiplicity validation, 180 lines
  from where the home was put. The primitive is now `runs_in(&[f64])`, private
  to `knots.rs`, with `interior_knots()` and `knot_runs()` as its two one-line
  public faces; the awkwardness was measurable throughout — `fit.rs`'s
  `deviation_from` called the scan and then `multiplicity_of` **per value**,
  paying an O(n) re-scan to rebuild a number the scan it had just run already
  held.
  *Lesson: a data structure whose API was frozen one PR before its first
  consumer is the tell; "did you have to hand-scan? why isn't that on the
  type?" is a cheap review question nobody is asked to ask.*
  *Second lesson, from the misses — there were three shapes, and one census
  pattern is blind to each.* A pattern keyed on the RESULT misses copies that
  **reduce** it (the two `mesh` gates were the same scan with an `any()` on the
  end). A pattern keyed on the interior BOUNDS (`p + 1 .. len - p - 1`) misses
  the copy that scans the **whole** vector, because that one contains no degree
  at all. And a pattern keyed on either misses the copy inside the **validating
  constructor**, which runs before the type exists and so cannot call the type's
  own method — the fix there is the one this PR had already made for the binary
  search (lift the body to a free function over the slice) and did not
  generalise until a reviewer pointed at the asymmetry inside its own diff.
  *Third lesson: giving a query a home does not close the routine BUILT on the
  query. `make_compatible` and `deviation_from` became legible as one routine
  only once both opened with the same expression — see D31.*
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

## S19. PARTLY FIXED by #740 — the fillet catch-all was three state classes wearing one name, and only 18 of its 103 sites can be proved to be the assertions the finding read them as

**The `AssemblyUnsupported` row is closed (D2, #740). Every other row of this
finding is LIVE.** Row by row: `MissingEntity` has a Track A placement;
`SplitJoinError::Corrupt` is FIXED by #690 (and the table below described it as
payload-free long after #690 gave it `entity: EntityId` — corrected);
`ValidationError` is discharged as FLAGGED AND OVERRULED by
`GENERICS-BUILD-COST.md:169`, which measured it and ruled it the D9 charter
working as specified.

**The placement audit is FIXED by #752 (D26): the four rows that had no row
anywhere are now placed as D36–D39, one of them is partly refuted, and that
PR's review placed two more — D47 and D48.** Each
was re-derived against the tree rather than transcribed, and each disposition
is recorded on its own row in the table below. What #752 closed is the
*placement* gap — §D's fourth ordering rule, *a verdict is not a placement* —
not the defects themselves, which are what D36–D39 carry. Three corrections
the re-derivation forced, on this finding's own text:

- **`DESIGN.md` D4 ¶2 does not cite `SkippedMerge`.** The string
  `SkippedMerge` does not appear in `docs/DESIGN.md` at all. What D4
  commitment 2(ii) cites is *"`merge_coplanar_faces`' already-collapsed error
  as the in-repo precedent"* — for a **message-level** rework (a shared
  `Indeterminate` `Display` string), which is message policy, not the
  `format!`-instead-of-a-type outcome. The in-tree citation that *does* make
  the stronger claim is `boolean/mod.rs:828`, and it reads the precedent the
  other way round: *"refused typed, never a laundered catch-all (the
  `SkippedMerge` precedent)"*.
- **`StepImportError` is no longer *"out-of-fence for a Display impl"***
  (`step-import/src/error.rs:289`), nor is `MassPropsError`
  (`topo/src/props.rs:102`) — so the *"three sites use `format!("{err:?}")`"*
  clause was two live sites by the time it was audited, and **#752 fixed both**
  as one-liners (`pncad-py/src/py/value.rs`, `err.to_string()` — the idiom the
  same file already used fifteen lines away). The third site was retired
  earlier; only a comment in `pncad-py/src/tests.rs` still names it.
- **`tags.rs`'s founding premise had expired.** Its module doc justified the
  tag-only shape with *"Neither `EditError` nor `NodeErrorKind` implements
  `Display`"*; both do (`edit.rs:592`, `eval/mod.rs:649`, landed by #308),
  and `py/doc.rs:22` says so in the same crate. **#752 corrected it.**

- **Confidence**: sure
- **Found independently by five scans.**

### `AssemblyUnsupported` — FIXED by #740

**Re-derived census: 103 construction sites** (not the row's 102, which counted
closure *calls* — `build.rs:127` constructs the variant directly; not the
finding's 146, which predates #688). The idiom was **eleven** closure
declarations, not the nine the row names: nine in `surgery.rs`, two in
`build.rs`. All eleven are gone.

**108 sites out**, because five refusals conflated two addendum rows behind one
test — an empty chain with an unbuilt one (twice), a corner's CONFIGURATION
with the REQUEST's coverage, and two new local checks (below). One of those
splits is a user-visible correctness fix: an *empty* open chain used to be
reported as *"an open chain with **more than one** link"*.

| Row | Sites | What they became |
|---|---|---|
| 2 — valid input, unbuilt | **41** | `UnsupportedChain{edge,detail}` 16, `UnsupportedGeometry{at,detail}` 14, `UnsupportedRunOut{at,detail}` 7, `FilletCornerUnsupported{vertex,corner,policy}` 3, `UnsupportedBody{solids,shells}` 1 |
| 1 — invalid input | **49** | `BodyNotIntact{at,detail}` 46, `EmptyChain` 2 (**both retired by #768**, D27 — the variant no longer exists), `RepeatedEdge{edge}` 1 |
| 4 — kernel bug, observable | **18** | `unreachable!`, each carrying its own per-site proof |
| 5 — re-derivation | **0** | rows 4/5 split on re-derivation; a failed lookup is observed |

`at` is **`topo::EntityId`** — the sum this document's own line 792 records
#690 giving `SplitJoinError::Corrupt`, *"so the compiler proved that sweep
complete rather than a grep."* A first revision of #740 minted a near-copy of it
and review caught the duplication inside the PR closing a duplication finding.

**Which frontier variant a site takes is now a stated rule** on the enum: *what
the branch READS*, never which noun the frame holds. Without it one fact refused
two ways — a run-out was a chain variant at one site and a corner variant at
another **in the same function**, shipping two different recourse sentences —
and "a corner support is not a plane" was worded four ways across two modules.
Two refusals turned out to be existing `CornerConfig` variants written as prose
(`NEdgeVertex`, `MixedConvexity`) and now go through the type, giving
`RunOutPolicy` its first constructor surface.

**The per-site standard** (both clauses required): **(i)** every arena key the
site dereferences was minted by an operator in THAT call, returned by a walk
that succeeded in THAT call, or proven present by a check in THAT call — never
inherited from "the body is valid"; and **(ii)** the condition it observes is a
property of state that call itself produced. Three sites failed clause (i) as
first written — their proofs cited checks one frame up — and were **made
provable rather than downgraded**, by adding the two local checks the fusion
argument needs. `corner_convexity`'s arms stayed typed because their fact was about
a caller's behaviour, not about data in hand, and that is the stated line
between the two dispositions.

**That line stands; its worked example does not, and the reversal is the
point — REVISED by #768 (D27).** `corner_convexity` is deleted. A fact about a
caller's behaviour is not condemned to stay a typed refusal forever: it can be
**put into the data in hand**, at which point clause (i) is satisfied honestly
and there is no arm left to dispose of either way. #768's tokens do exactly
that, and none of the three helpers this document cited as row-4 candidates
became an `unreachable!` — each lost its branch instead. So the line to cite
here is *"is the fact in the data this call holds?"*, and the third disposition
it admits, beside typed-refusal and `unreachable!`, is **change what the call
holds**.

**On the finding's number, and on the retracted refutation.** #740's first
revision claimed the row-4 reading was *refuted*, on the argument that
`graft_disjoint_all` demonstrably hands `fillet_edges` a tier-1-invalid body.
**That argument was false and is retracted.** The public `instance` doors bridge
with `combine::Bridge::RemapKeys`, whose arm never reaches the only
`GraftRecertify` raise — so `instance.rs`'s `# Errors` named an error its own
door cannot return (**fixed by #740**, found only because the argument
collapsed) — and, independently, a spent destination comes back with ≥2 solids
and is refused by `fillet_surgery`'s entry gate above all 46 sites. What
survives is weaker and true: **no input is demonstrated to reach any of the
46** — an adversarial search reached none of them and no panic, at 1,842
requests per run on the shipped effort dial and 12,210 at `CAD_FUZZ_EFFORT=10` — and the standard cannot discharge them locally, because their keys
arrive from outside the call. Converting on an unproved negative is the
direction D9's headline forbids, so **row 1 is the safe disposition on an open
question**, not a proven classification. **S14** is the open question
underneath.

Against the finding's *number*: the comparable family — `ok_or_else(||
unsupported(` plus one `None =>` arm — is **63 of 103**, of which **14** are
among the 18 `unreachable!`s. So the comparison is 14 against ~120, on a family
of 63; the other four row-4 sites are not lookups but shape observations on
entities the call minted.

**`EmptyChain` was filed under row 1 and did not meet its definition — FIXED by
#768 (D27), by deleting the state rather than the row.** Row 1 is *"reachable by
input, invalid"*, and `walk_chains` seeds every chain with one link, so no input
reached it; it was not row 4 either, by clause (ii). The addendum had no home
for *"not locally provable, not input-reachable"*, and **it did not need one**:
`Chain` now holds its first link in its own private field, so the variant has no
representation and both of its sites are gone. Rows 1–5 stand unamended, and the
generalisation became **row 0 of the D2 addendum** — *can this error state be
made unrepresentable?*, asked before the classification and **preferred over
every row below wherever it is available** (Evan, 2026-08-20, PR #777 — which reached `main` only via **#817**, having
been merged into a lane branch). Row 0 *reframed* **S14**'s first question
rather than answering it; S14 itself is still open. The pin behind the
old sentence also **re-derived to 305 verdicts per run, not 343**; #768 retired
it, because `Chain`'s type now carries what it asserted and an assertion that
cannot fail is §C8.

**The `Display` recourse — the finding's concrete user-facing wrong.**
`FILLET3_ASSEMBLY_RECOURSE` (*fillet a set of edges whose open chains are single
convex plane–plane links…*) rode on all 103 sites, including every "does not
resolve". It now attaches only to `UnsupportedChain`; run-outs and corner
configurations take `FILLET3_CORNER_RECOURSE`; two new constants serve body and
geometry; the row-1 variants carry **none** and say so. Verified through the
front door, not just against the table: a one-edge cube request returns
`UnsupportedRunOut` carrying only the corner sentence and none of the other
eleven, and a repeated edge carries none at all
(`tests/review_d2_recourse_at_the_site.rs`).

The in-file guard is an exhaustive match carrying **both** the recourse decision
and a witness of its own variant, after review found it had a false row
(`Escalated` appends one of six constants; the table said none) and a
hand-kept witness list that let a decision ship unchecked. Its residual is
stated on it: Rust cannot enumerate variants, so the seeds are hand-written.
`m5_pr12_refusals.rs`'s constant list was short by two before #740 and by four
after; all twelve are there now.

**`pncad-py`.** `FilletError` does not appear in `tags.rs` — the map is one
level above, at `NodeErrorKind::Fillet { .. } => "fillet"`, which drops the
whole payload (this finding's own live row). pyo3's `#[pyfunction]` trampolines
*do* `catch_unwind` and raise `PanicException`, so a reachable `unreachable!`
would be worse than #740's first revision stated; the boundary holds only
because none of the 18 is reachable at all.

| Variant | Sites | Problem |
|---|---|---|
| `MissingEntity` | 49 | Documented as "corrupt input"; also carries "non-iso trim carrier reached the iso-rectangle walk (**router defect**)" — kernel bugs reported to the caller through the dangling-key variant |
| `UnsupportedCarrier` — **placed as D36** | 22 | Re-derived: 22 construction sites, all in `geom-brep/src/pcurve_cache.rs`, carrying **three** unrelated meanings under one payload-free name, beside a sibling `IsoUnsupported { what }` at 16 sites in the same file that names its refused class every time |
| `SplitJoinError::Corrupt` — **FIXED by #690** | ~42 | *Was* payload-free — no key, no half-edge, no face — beside `SplitReduceError`, which carries the offending entity on nearly every variant. #690 gave it a required `entity: EntityId`; the entry survives as the row's history, not as a live problem |
| `ValidationError` | 59 variants | Spans four validity tiers; tier membership lives in doc-comment prose, so `validate()`'s signature promises nothing and no consumer can exhaustively handle "the structural failures" as a set |
| `pncad-py/tags.rs` — **REFUTED as stated; residue placed as D37** | 383 lines | A discriminant tag map is the right FFI shape and *does not* "drop the payload": the payload path is the exception's `Display` message plus its per-variant fields, and the map's exhaustiveness is a drift alarm that fires in CI. The `{err:?}` clause is fixed (above). What survives is a **duplicated** discriminant (`path_error_tag` re-derives in `pncad-py` what belongs on `PathError`) and an unowned deferral (*"full per-variant field projection … deferred to the unit that binds the complete surface"* — no such unit exists). #752's own review found two more in the same crate, placed as **D47** (the *"never a `Debug` dump"* rule's two remaining sites, and nothing guarding it) and **D48** (the one non-exhaustive tag map's compensating alarm, which cannot fire) |
| `SkippedMerge { reason: String }` — **placed as D38** | 2 | Confirmed live at `merge_faces.rs:496` and `:508` (the row's `:489` is the `match`, seven lines up). `merge_coplanar_faces` runs two incompatible failure regimes on one door — unlicensed planar groups propagate a typed `MergeCoplanarError` and refuse the whole call, licensed-or-curved groups `format!` the same typed errors into a skip record — and one of the two `format!`s `{:?}`-dumps a `ValidationError` that has a `Display` |
| `ProgramRefusal::Geometry` — **placed as D39** | 1 | The constraint still holds exactly as stated: `EditError` derives `PartialEq` (`edit.rs:246`), `PathError<T: Real>` derives only `Clone, Debug` (`path.rs:516`). The degradation is at `program.rs:862` (`:846` is the enclosing `check`). Its cost is now visible in the tree: `editor-core/tests/switch_slots.rs:191` can only identify *which* geometry refusal fired by `rendered.contains("radius")` |

**Verdict:** ACCEPTED (Evan, 2026-08-18). "Ha these are funny (and also show
again that we need a bug vs reachable invalid state distinction)" — i.e. this
finding is **downstream of S43**. The postmortem pass is asked to substantiate
or refute that: how many of these catch-alls exist because the author had no
vocabulary for "this can only be a kernel bug"?
**Postmortem (2026-08-18). Evan's hypothesis is substantiated for the three
highest-volume rows, and S43 turns out to be their generator.**

- **`AssemblyUnsupported`** (**fixed by #740** — the disposition and the
  refutation of this bullet's last sentence are above; the archaeology below
  stands and is why the row existed) — born legitimate at #166 (9 sites, an explicit
  "front door does not exist yet" marker). #171 then declared the idiom
  `let unsupported = |detail: &'static str| …` at the head of ~12 functions,
  and that closure is what took it to 146: the overwhelming majority are
  `.ok_or_else(|| unsupported("a ring does not walk"))` — arena lookups that
  cannot fail on a valid body, **five of which say "(kernel bug)" outright**.
  #171's deviation 3 declared only the four genuine scope gaps; ~120 assertion
  sites rode in under that sentence — but see above: on a per-site standard
  **18** are assertions, and the rest report invalid input. **NEVER
  FLAGGED** — #166's F6 is the
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
  arms. The residue was recorded in the same body: `StepImportError` *"is
  out-of-fence for a Display impl"*, which is exactly why three sites still
  `format!("{err:?}")`. **Corrected by D26:** `StepImportError` has had a
  `Display` since `step-import/src/error.rs:289`, the residue was two sites by
  the time it was audited, and both are fixed.
- **`SkippedMerge { reason: String }`** — **the reverse of the others.** Born
  with a generic label; **the reviewer killed the label**. F3, verbatim:
  *"`Err(_)` catch-all launders tier-2 diagnostics → preserve real reasons."*
  The implementer discharged "carry the actual diagnostics" with `format!`
  rather than an enum — and `DESIGN.md` D4 ¶2 was read as **later citing this
  outcome as the in-repo precedent**. **Corrected by D26:** it does not.
  `SkippedMerge` appears nowhere in `DESIGN.md`; D4 commitment 2(ii) cites
  `merge_coplanar_faces`' *already-collapsed error* as precedent for a
  **message-level** sweep. The citation that reads the other way is in the
  tree — `boolean/mod.rs:828` calls `SkippedMerge` the precedent for
  *"refused typed, never a laundered catch-all"*.
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
[the central commitment](DESIGN.md#the-central-commitment). What is actually true
and now ratified — **the model is a pure function of (parameter vector,
ε)**, ε being a declared run parameter with a recorded provenance, one
per process by construction, mixed-ε assemblies out of scope — belongs in
`docs/DESIGN.md`, marked `PROPOSED` pending sign-off exactly as #628 did
for the D2 addendum.


### S22 row 1 REVISED (2026-08-21): threaded after all — as a witness, not a value

**Evan, 2026-08-21: thread ε, at every call site.** This reverses the
*"do not thread ε"* half of the 2026-08-19 ruling above and nothing else.
Everything that ruling settled stands untouched: the `OnceLock` keeps its
place and its enforcement job, no session object, no per-model ε, no
mixed-ε assemblies, and the provenance channel it commissioned (#659) is
unaffected — this change gives `EpsilonSource` no new readers and moves no
decision.

**The reversal turns on a design the ruling did not consider.** Both sides
of the 2026-08-19 argument assumed the threaded parameter would be a
`Tolerance` — the *value*. Both of the ruling's decisive objections are
objections to exactly that, and neither reaches a witness:

- *"The `OnceLock` is the only thing structurally enforcing one ε per
  process; threading deletes that enforcement in exchange for
  documentation."* True of a value parameter. A zero-sized `Tol` witness
  carries evidence instead — the value never leaves the `OnceLock`, which
  stays where it is. Nothing is deleted; enforcement is added to.
- *"It bought a signature that documents the dependency and no
  configurability at all"* — `profile`'s 256 call sites, every one passing
  `Tolerance::get()`. That reads as a false promise because `tol:
  Tolerance` *looks* like it could carry something else. `Tol` has one
  inhabitant and cannot, so the signature promises precisely what it
  delivers, and "every call site passes the same thing" stops being
  evidence of a bad trade and becomes the type's stated content.

**The objection that survives is churn**, which was real then and is being
paid now: ~80 `Band::linear()` and 17 `Band::angular_at()` sites in `src`,
their callers up to each operation entry, and ~400 test sites. What makes
it affordable is that it is compiler-driven and mechanical — the 355
functions that already take a `Band` are where threading stops, since the
band is the derived value — and that no conflicting work is in flight.

**What it buys that neither 2026-08-19 option could.** The `no-ambient-env`
rule gains an enforceable sibling rather than a documented convention;
the central commitment's ε exception is *deleted* rather than reworded,
which is the prose obligation above discharged at its root instead of
patched; `mesh`'s ε inventory — pinned as a test by #872 and the subject
of #884's open D9 question — becomes structural, since an ε read that is
not in a signature stops compiling; and `profile`'s double mechanism, the
open question this row explicitly left behind, collapses into one.


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

**H12 — the rest of the doors — landed as #734**, tests only. The
never-silence claims in this area were enumerated from the refusal sites
in `exhaust.rs` and the one guard in `ssi.rs` that makes the sweep's
floor meaningful. **Sites are not the grid**: the budget check and both
poison arms sit in the ONE shared recursion, which runs under both
values of `SweepDuty` on separate calls with separate floors, so duty is
an axis — and it is the axis this finding's bug lived on. Crossed with
the lane, with duty where duty applies, and with the tube set for the
floor refusal (the one site that exists under the accounting duty only),
the grid is **thirteen cells, of which three had rows. Five more do**:
the floor refusal's fourth {lane}×{tube set} cell, and the cell budget
and the poison arm in each lane under the **seeding** duty — verified by
instrumenting both `exhaust` entry points on each new row. The eight
cells that had a row and the five that still do not are enumerated at
the block itself in `geom-brep/tests/m5_pr7_ssi.rs`. **The four
Account-duty cells are §D row C18**, together with the two other
residues of this enumeration — the macro blind spot on the containment
measurement below, and the file header's claim that the battery runs it
at the interval scalar when the interval leg runs only the set
difference of the two test lists. A sixth row landed that covers no
cell: it is the executable record of a live defect, below.

The one that matters beyond the count is the **chart lane's containment
test**. Each lane's accounting predicate reaches the sweep through one
`SweepCell::contained_in` forwarder, and those two are the like-for-like
pair: making the chart one return `true` for every cell — the accounting
pass claiming to have proved cells it never touched — reddened **one** of
`geom-brep`'s 260 rows, and making the ℝ³ one return `true` reddened
**three** (measured on the merged head, 92 lib + 168 integration). One
against three is the asymmetry. Mutating the ℝ³ predicate at its
*definition* reddens six, but three of those are not accounting coverage:
one is the predicate's own unit test in `enclose.rs`, and two are
adversarial loop rows that go red only when the ℝ³ **seed dedup** at
`ssi.rs:781` — a second, unrelated consumer — breaks at the same time.
#633 had examined the chart cell and argued it should stay unwritten; the
argument priced the shared floor arm and not the containment test
underneath it.

**One door is recorded rather than closed, and one is LIVE.** The
chart-speed guard's own two arms are unreachable from the public doors as
they stand. The hole *beside* them is not: `plane_nurbs_ssi` guards on
`speed.is_nan() || speed <= 0.0`, so a certified chart speed of `+∞`
passes, both floors translate to exactly `0`, the certified tube padding
would be zero-width, and the **cell budget answers with the wrong
diagnosis** — the caller is told its search was too big when the wall has
no usable chart speed. Reachable from the public door on a NURBS surface
any caller can build, and measured. That is a source-side logic fix
inside a certifying door, so under C-R19 it is **issue #762** and no §D
row; the same issue carries the latent `f64::max`-drops-a-lone-`NaN` in
the same expression, and the ℝ³ poison arm's message naming surface kinds
its own door excludes.

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
beside it asserts only `area_pad > 0.0` plus containment. **Both conditions get
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

**Half of this was closed by #873, and the half that matters was not.** The
acceptance row this finding names now pins `area_pad` to the `volume_pad` the
kernel does meter — an identity of the lane's arithmetic, so it needs no
metering rule to state — and both it and the loft site carry outer ceilings;
#873's PR body carries the derivation and the numbers. **The metering half is
still open, as issue #870** — this finding's first paragraph remains an
accurate description of the kernel, and #870 measures what it costs. Three
further unmetered certified widths, in crates no live track owns, are **S230**.

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

## S29. Mechanical half FIXED by #803 — the sizing vocabulary has one home and one word per quantity; the sizing POLICY is still unstated and is a separate design PR

- **Where**: `crates/mesh/src/sizing.rs` (new), and
  `crates/mesh/src/{chords,curved,nurbs_cert,trimmed,tessellate}.rs`,
  `tools/tess-meter/src/lib.rs`
- **C-R2 split this lane.** #803 is the mechanical half only. *Stating*
  the sizing policy is a design act; it goes out as its own PR and waits
  for Evan's sign-off. Until then the class complaint this finding
  raised — *N well-defended deviations read as N decisions when they are
  one undecided question* — **still stands** for `SAFE_ASPECT`,
  `MAX_GRID_RETRIES`'s value, `RATIONAL_CERT_SPLITS`, the sphere
  margin's value, δ_s = δ/2 and the 12-vs-6 sample-density split. See
  also **S116(h)**, which is that half and not this one.

**The recorded population was a snapshot, and wrong in both
directions.** Measured on `origin/main@d4e38059` by data flow (seed on
the sizing target in a parameter list, close over the call graph) plus a
call-site census of every step/count producer across `crates`, `tools`
and `demos` — an instrument shaped differently from the finding's list.

* **Three of the eight named constants and two of the nine named entry
  points had already left the crate.** `DEV_SAMPLES`,
  `SPLIT_SCAN_DECADES`, `SPLIT_SCAN_STEPS`, `divisions` and
  `best_split_steps` moved to `tools/tess-meter` when #709 landed S30.
  **Three of this finding's five scope citations did not resolve**, and
  C-R11 makes each of them a claim site, so all three are recorded here
  rather than silently rewritten — checked line by line against
  `d6b5ae01^1`, the tree immediately before #803:
  `budget.rs:555` (the file is **307** lines), `curved.rs:226` (lands on
  `cdt.add_constraint(a, b);`) and `nurbs_cert.rs:356` (lands on
  `memo: &mut FaceBounds,`). Only `chords.rs:63` (`sagitta_angle`) and
  `trimmed.rs:112` (`MAX_GRID_RETRIES`) pointed at anything this finding
  is about. The `curved`/`nurbs_cert` pair was reported by the lane and
  written down by the style review.
* **The remaining vocabulary is wider than the list.** Genuinely
  unnamed by it, and there are **four**:
  `chords::nurbs_chord_count`, `chords::nurbs_tighten`, the inline
  cylinder sizing at `trimmed.rs`, and an **unnamed inline step
  formula** inside `nurbs_chord_count` that was `ellipse_step`'s
  formula spelled a second time. `curved::pole_columns` is a fifth in
  spirit — the finding recorded it arriving after the fact, in #684.
  **Corrected by the style review, 2026-08-20**: the version of this
  bullet #803 wrote also listed `NurbsCellGrid::row_bound` and
  `trimmed::uniform_candidates` as unnamed, and **both are named by the
  original text, verbatim** (`git show
  d6b5ae01^1:docs/SMELL-SCAN-2026-08.md`, §S29 ¶1: *"…
  `NurbsCellGrid::band_schedule` + `row_bound`,
  `uniform_candidates`/`per_cell_candidates`…"*), along with a third,
  `per_cell_candidates`, which the replacement dropped entirely. The
  correction's DIRECTION stands and its size was overstated. Recording
  it here rather than in a log because the recording convention
  **replaces** the original inline, so this section is the only in-tree
  account of a document it was getting wrong.

  **This is a standing hazard of the convention, not one PR's slip, and
  is filed here for whoever writes the next replacement.** A record that
  replaces its subject in place makes version control the only surviving
  copy of what was replaced — so a replacement that *characterises* the
  original ("the list was too narrow", "the citation was wrong", "the
  finding missed X") is asserting something no future reader can check
  without `git show`, and reviewers do not reach for `git show` by
  default. The failure mode is specific and it is not carelessness: a
  correction is written while looking at the NEW tree, and the sentence
  about the OLD text gets composed from memory of it. The cheap
  discipline is to **quote the original inline whenever the replacement
  makes a claim about it**, as this bullet now does — a quotation
  survives the replacement, a characterisation does not.

**What #803 did.** One module, `crate::sizing`, owns the vocabulary and
its module doc fixes one word per quantity: a **target** (δ_s — one of
them, `sizing_target(δ)`, carried by `Tol`, which moved out of `curved`);
a **step** (an `f64` parameter increment); a **count** (a `usize`
division count, always `ceil_count(span, step)` or a rule applied to
one). The doc states the enforced rule as one sentence — *"step" names
an `f64` increment and nothing else; a `usize` count is never called a
step, and neither is a sample count* — rather than as a suffix
convention, because a suffix convention would have been false at
`pole_columns` and `cap_angular` the moment it was written. Under it:
`sagitta_angle` → `sagitta_step` (it was a step named "angle");
`curved::grid_steps` → `grid_counts` (it returned counts, and shared its
name with `NurbsFaceBound::grid_steps`, which returns steps — a collision
that had already made one `nurbs_cert` doc sentence ambiguous);
`MAX_STEPS` → `MAX_COUNT` (it capped a count); `BandDivisions` →
`BandCounts` (a third word for the same thing);
`tess_meter::SPLIT_SCAN_STEPS` → `SPLIT_SCAN_SAMPLES` (they are samples).
Three literal `FRAC_PI_4` spellings and two ways of comparing against
them became `MAX_ANGULAR_STEP` + `cap_angular`; two spellings of the
second-derivative step formula became `curvature_step`; the bare `1.25`
became `SPHERE_SIZING_MARGIN` with its argument moved to a doc.
`tess_meter::divisions` stays a second spelling **deliberately** — a
different cargo root, `f64`, refuses nothing — and its doc now says the
different *name* is the tell. `MAX_GRID_RETRIES`'s *"was 4"* comment,
which this finding named as the deviation ledger inlined into the
source, was rewritten as the invariant.

**Proof, and its exact limits.** No meshing decision moved: every
arithmetic expression is preserved operation-for-operation. Two oracles,
base vs head as worktrees, separate target dirs. **Naming the trees:**
oracle 1 was run at `1a94204d` vs `2e9206a1` (whose difference is exactly
this PR's code) and **re-derived** at `90b5e178` (`main` after an
unrelated `crates/sweep` landing) vs this PR's tip; both identical, and
the re-derivation also shows `main`'s own output over these rows did not
move between those two `main` commits. Oracle 2 ran once, at `1a94204d`
vs `2e9206a1`; the only `crates/mesh`/`tools` delta from there to the tip
is comments, checked mechanically. (1)
f64-bitwise `dump()` hashes — positions' `to_bits`, triangle indices,
boundary polylines — over 17 bodies × 8 deltas = **136 rows, all
identical**, including the trimmed lane's split-cylinder halves and two
NURBS bodies. (2) The whole `demos/tour` output (52 STL + 52 STEP + 1049
UV SVGs + `scenes.json` + `uv.json`, 59 MB) and the whole `demos/wild`
output (8 imported third-party STEP bodies): `diff -r` clean. **What
neither reaches**: the tour STLs are f32, so oracle 2 proves structure
and f32 geometry, not f64 identity (oracle 1 owns that claim, over its
136 rows); refusal paths no in-tree body reaches have no byte-identity
oracle, only the typed-refusal tests; and the doc-only edits have none at
all.

**C-R4, preserved because the row that carried it is gone.** §D's C3 row
once routed this finding's policy question to `docs/TESS-SPLIT-SPEC.md`
and PR #568. That was wrong and was corrected in the row on 2026-08-19,
before this lane opened: both #568 and TESS-SPLIT-SPEC are scoped
**entirely** to the NURBS per-cell step derivation in `nurbs_cert`, so
`curved::grid_steps` (now `grid_counts`) never had a venue. **Stated
exactly, because "nothing is ratified" would be too wide**: exactly one
sizing question has a ratified answer — `docs/TESS-BUDGET.md`'s *split
schedule's aspect policy*, A = 16, ratified 2026-08-16 on #568 and still
unexecuted. It rules where the NURBS split schedule takes its point on
the certified ellipse and **nothing else**: not δ_s = δ/2, not the
sphere margin, not `SAFE_ASPECT`, not the retry or refinement budgets,
and not the analytic charts at all. Those are what the C-R2 design PR
has to speak to, and they have no venue.

**Residue filed as §D row C23**: `nurbs_cert::RATIONAL_CERT_SPLITS` and
`geom::curves`' `RATIONAL_METER_SPLITS` are one refinement schedule
hand-synced across a crate boundary.

*Lesson kept from the original text:* reporting each magic constant as
its own honestly-argued deviation is a **substitute** for stating the
policy, not a step toward it. Unifying the vocabulary does not change
that; it only makes the six locally-argued rules legible as six.


## S32. FIXED IN PART by #804 — the API half; the "second surface enum" half was FALSE

- **Where**: `crates/geom/src/surfaces.rs` (`Surface::jet`, and the five
  partial accessors plus `normal` as its projections), `crates/step-import/src/chart.rs` (`metric_floor`),
  `crates/geom/tests/surfaces/s32_jet_projection.rs` (the pin)
- **Confidence**: sure

**The API half is FIXED.** `Surface::jet(u, v) -> SurfaceJet<T>` is the
enum's derivative primitive: one match, the point and every partial with
`k + l ≤ 2`, the analytic arms building the azimuthal frame and
`sin_cos(v)` once, the NURBS arm making a single `ders` pass. The five
partial accessors and `normal` are now its projections — **one
per-variant implementation where five stood, plus a `normal` that ran
two of them**, and that is the change's justification. No performance
claim was made for it, because #804 measured none; the review then
measured the two arithmetic claims anyway, with a `#[global_allocator]`
counter on a rational patch: `normal` 40 allocations against 80 with
the two-evaluation body restored, and `eval` 2 against the jet's 40. So
`normal` does make one `ders` pass rather than two, and
`NurbsSurface::eval` is the cheaper pass — both now measured rather
than read.

That counter also **bounds a claim #804 made too strongly**. The PR
said the evaluation-count reduction was *"guarded by reading the code,
not by a test"*, on the ground that no bitwise row can witness a count.
The first half is right and the second is not: `ders` allocates, so the
count is observable from outside the crate without reading a line of
source, and both of the mutations #804 called unguardable trip that
counter. The true statement is **unguarded by any bitwise row, and
guardable only at the price of a second test binary** — `geom`
aggregates every suite into one (`autotests = false`, `[[test]] name =
"all"`) and a global allocator is per-binary. The practical decision
stands; the absolute claim did not.
`step-import`'s `metric_floor` was the only production caller of any of
them and asks once per sample instead of twice. Preservation is
**bitwise, not tolerated**: a 23904-row dump of all seven doors over 9
charts (five analytic, a rational NURBS patch, the placeholder, two
degenerate charts) × 2656 parameter pairs is byte-identical between
`1a94204d` (the pre-change tree) and the shipped head `5524b1f5`, md5
`d80a47f4cb63eda6ad2b0670f5a4f0ab`, re-taken to the same md5 after each
of the three `origin/main` merges in between. That includes the 8042
rows whose fields are NaN — NaN **payloads** match, which is where an
order change would have shown. **8042 is derivable, not just
observed**: three all-poison charts × 2656 = 7968, plus the cone arm's
`v = 0` rows, where `du = tangential·(v·sin α)` vanishes — 41 + 31 + 2 =
74 across the two grids and the named points. The sphere poles
contribute nothing, because `cos(π/2) = 6.1e-17 ≠ 0`.

**Two limits on that md5, both load-bearing.** First, the dumper lives
**outside this repo**, so the hash is not reproducible from the tree;
what is reproducible is
`crates/geom/tests/surfaces/s32_jet_projection.rs`, which pins the same
property in-tree. Second, byte-identity holds **for finite
parameters**. An adversarial dump over ±∞/NaN chart parameters found
192 differing rows, every one a quiet-NaN **sign** flip and zero of
them with both parameters finite — and one flipped field belongs to
`Surface::eval`, whose source is identical across both trees. So those
flips are LLVM sinking an `fneg`, not this change, and the general
lesson is recorded rather than the local one: **quiet-NaN sign bits are
not a stable function of this source**, and *"the expressions were
copied verbatim"* is a source-level statement that does not by itself
reach the bits. The finite-parameter identity is what was measured and
is all that is claimed.

**The claim now covers the other scalars too, bitwise.** The suspicion
worth testing was that sharing one `azimuth::frame` result could widen
an enclosure. It cannot, and the reason is structural — `Interval` is a
stateless value type, so evaluating a deterministic sub-expression once
or twice yields the same enclosure, there being no dependency tracking
to de-correlate. Measured as well as argued, on a head-only instrument
that sets base's accessor bodies beside the shipped ones (removing the
cross-tree confound): **0 differences at `Interval` and 0 at
`DualInterval`** over 262,440 comparisons per scalar, on lo bits, hi
bits *and* decoration, with wide enclosures rather than points. This is
the **first bitwise interval evidence in the repo** — the interval CI
lane exercises only `deriv_u` and `deriv_vv`, and only for containment.

**Two clauses of the original finding were false and a third was
overstated, and correcting them is the more valuable half.** The count
is exact on purpose: an overstatement restated at the notch it can hold
is not a falsehood, and a record that over-claims its own corrections
is the one thing this track can least afford.

1. *"The natural query being unavailable at the enum is what created the
   second enum."* No. `Chart` was born at `ffc0b0fa` carrying the doc it
   still carries, and **three independent reasons survive a jet door,
   any one sufficient**: it is **third**-order where `SurfaceJet` is
   second, and `Surface` exposes no `∂³` for any variant (writing four
   analytic third-order ladders is a new feature, not a cleanup); its
   `Plane` variant carries `u_range`/`v_range` because *"a plane is
   unbounded, so the domain is the caller's named extent, never a
   guess"*, and `Surface::Plane` has nowhere to put a domain; and a
   `Surface`-wide constructor is **deliberately refused** under a named
   rule — *"Adding a chart is adding an arm, per C12.1's per-arm
   retirement rule."* `Chart` stays, and nothing in `ssi/system.rs`
   changed.
2. *"hand-filling a `SurfaceJet3` of zeros"* reads a right answer as a
   workaround. A plane's second and third partials **are** exactly zero;
   those fields are the correct analytic values. `Chart`'s own doc says
   so: *"whose second and third partials vanish identically, so the
   cubic term costs nothing."*
3. *"re-implementing plane evaluation"* is one notch too strong. What
   `Chart::Plane` holds is `Surface::Plane`'s own arithmetic **hoisted**
   — `dv = normal.cross(u_ref)` resolved once in `plane_of` — plus the
   window. A hoisted cross product, not an independent implementation.

**The population, measured rather than read.** Five accessors discard
five of six, not six: `eval`'s NURBS arm is not a member, because
`NurbsSurface::eval` is its own cheaper pass. `Surface::normal` had
**zero** production callers — every call site sits in a `#[cfg(test)]`
module or a `tests/` file — and two crates' module docs already said so
in prose (`mesh/src/lib.rs`: *"`Surface::normal` is never sampled
anywhere"*; `geom-brep/src/implicit.rs`: *"nothing here ever calls
`Surface::normal`"*). `deriv_uu`/`deriv_uv`/`deriv_vv` have no caller
outside `crates/geom` at all. **Before #804, four of `Surface`'s seven
public evaluation doors were exercised only by `geom`'s own tests**
(`deriv_uu`, `deriv_uv`, `deriv_vv`, `normal`); **after it, six of eight
are** — `metric_floor` was `deriv_u`/`deriv_v`'s only production caller
and now asks for the jet, so `eval` and `jet` are the only two doors any
production path walks. Recorded because it will decide a future question
and nobody will re-measure it cheaply; it is not a mandate to delete
them.

**The class's curve-side member is filed, not fixed** — §D row **C24**.
`Curve3::deriv`/`deriv2` are the identical shape one file over, and
`topo/src/splitting/neighborhood.rs` calls both at the same `t` under a
comment naming the result *"the base-endpoint jet"*. That call site is
the **analytic** member of the class, not the NURBS one — it sits in the
`Circle | Ellipse` arm and repeats an azimuthal frame, while the arm
above routes `Curve3::Nurbs` elsewhere entirely; C24 carries both
members separately, and says which is which. **C-R6 does not reach it**
even though it is the same crate: the surface side had a ratified
`SurfaceJet` to project onto, and the curve side has no `CurveJet` at
all, so that work is minting a public type — a design element, C-R19
tier two.

**What #804 cost, recorded because the record volunteers gains.** The
collapse **minted a duplication of the shape it closed**: each analytic
arm's point expression is now written twice, in `eval` and in `jet`,
where before each existed once. That is S4's shape one level over,
created by the unit whose job was to remove it. It is a considered
trade — the alternative makes `eval` a projection and charges the
workspace's hottest evaluation door an order-2 basis pass to spare five
short expressions — and it is guarded, by
`eval_agrees_bitwise_with_the_jets_point`, which a re-association
mutation reddens. Both facts are stated at the site in `Surface::eval`'s
own docs. Separately, **the five projections got slower on the analytic
arms**: `deriv_vv` on a `Plane` was `Vec3::zero()` and now builds a
whole jet. Affordable only because no production path calls those
doors — which is a measured fact of today, not a property of the
design. Two further shapes are **noted and deliberately not fixed
here**: `SurfaceJet` still lives in the NURBS arm's module though it is
now the enum-wide primitive's type, and this PR's test file is the
eleventh copy of the workspace's stock `axis`/`u_ref`/`origin` fixture
frame.

**Verdict:** ACCEPTED (Evan, 2026-08-18). On this batch: "huh these ones also
baffle me with how they ever happened." Postmortem pass commissioned.
## S33. Neither geometry enum can lift itself to another scalar

- **Where**: `crates/geom/src/curves.rs:818`, `:908`,
  `crates/geom/src/surfaces.rs:653`, `:1057`,
  `crates/sweep/src/skin.rs:774` — paths re-anchored by #705's crate
  merge; the finding is unchanged
- **Confidence**: sure

`DESIGN.md` makes "evaluate the same function with a different scalar
type" the reason the geometry layer is generic over `T`, but `Curve3<T>`
and `Surface<T>` have no `map_scalar`/`lift`. Every place needing
`Curve3<f64> → Curve3<Dual64>` or `→ Curve3<Interval>` writes its own
per-variant ladder: twice inside `geom/src/curves.rs` alone (the
dual and interval versions differing only in the scalar conversion),
twice again in `geom/src/surfaces.rs`, and roughly ten more across `topo`,
`mesh` and test modules, plus one production copy in `sweep`. Each must
be kept exhaustive by hand as variants are added, and each silently maps
`Nurbs(_)` to the placeholder rather than lifting the payload.

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
| `Real::is_poison` exists to support a NaN sentinel: "no description yet" is a bilinear patch whose control points are all NaN, recognised by testing `p.x.is_poison()` — in a crate that elsewhere works hard to make illegal states unrepresentable | `real.rs:143`, `geom/net.rs:137` | likely |
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
| The `nurbs_curve!`/`nurbs_fit!`/`nurbs_project!` macros mint a full 2-D twin whose heavy half (speed meter, removal bounds, `split_at`, `elevate_degree`, a whole `Projection2`) is shipped, monomorphized and unexercised | `geom/curves/nurbs.rs:61` | likely |
| `FitError::ParamCountMismatch` is returned for four unrelated failures — in a module that added `RaggedRows` specifically to avoid exactly that reuse | `geom/curves/fit.rs:115` | sure |
| `frame::path_start_frame` is justified by a deduplication it never performed (no kernel caller; `sweep` still builds its own frames) and duplicates `Vec3::orthonormal_basis`'s role with a different policy | `linalg/frame.rs:322`, `vec.rs:307` | likely |
| `ch_scale_left`/`ch_scale_right` are the same function kept apart "to preserve the rehearsal's association" — but `RingInterval::mul` is bit-for-bit commutative, so production shape is anchored to a test file for nothing | `spline/compose.rs:519` | likely |
| `CurvePlan::apply_points` defends against malformed plans its own three private constructors rule out, and pushes the cost onto callers as an invented poison value plus four near-identical lerp closures | `spline/algebra.rs:161` | likely |
| `lsq::solve_normal` forms `AᵀA` (squaring the condition number) on the fitting path while the sibling `svd.rs` already contains Householder QR; the adversarial review test validates it by implementing QR a *third* time | `linalg/lsq.rs:158`, `svd.rs:183` | unsure |
| ~~`certify_rung3`'s `arm` and `extent` are the same value at three of four call sites, so the `#[allow(too_many_arguments)] // one parameter per named quantity` covers a parameter varied once~~ **FIXED by #692** — `TubeScale<T>` names the two cases (`uniform(arm)` at the three sites where they are one quantity, `split(arm, extent)` at `finish_r3`); `certify_rung3` goes 8 args to 6 and the `allow` is deleted **on merit**. Found by review as a residue of #692's own diff: the PR removed one redundant parameter and left its twin in the same argument list — the exact class it existed to close, one line below the one it closed | ~~`ssi.rs:925`, `:729`~~ | sure |
| Two unrelated `compose` modules with two unrelated `ComposeError`s; defended on the grounds that "the two never meet in one scope", which is a claim about today's imports | `geom/curves/compose.rs:106`, `geom-core/spline/compose.rs:57` | sure |
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

*(D23 re-derived both denominators at this document's own scan base
`4258584`. **`topo`'s 53 is exact** — that many `crates/topo/tests/*.rs`.
**The workspace's 345 is not**, under any counting rule tried:
`crates/*/tests/*.rs` gives 398, 384 without the `all.rs` aggregators;
recursively through the group directories, 427 and 413; repo-wide,
436 and 422. The numerator is not re-derivable at all — "named after a PR,
milestone or review round" has no mechanical form — so **L1 should be read
as a proportion, not a pair of counts**. Stated rather than corrected: this
is a dated survey record, and the reusable half is that naming a scan base
makes a number checkable without making it right.)*

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

**The shipped bytes as #639 left them.** The ASCII solid name was
`cad-kernel` and the binary header `binary STL; CAD kernel tessellation
export` — the milestone token only. **Q9 was untouched**; the placeholder
was not a name proposal. The
finding's other half — that the STL header is not caller-settable while the
STEP writer takes `product_name`, `author`, `organization` and
`originating_system` as options — was scheduled as **H16** and left as a
residue for Evan, because closing it is an API design call.

**H16 is FIXED by #732** (rulings C-R1, C-R13–C-R15, C-R19). Both writers now
take options where `step_string` takes `&StepOptions` — **one options type per
writer**: `stl::AsciiOptions` carries the `solid <name>` name,
`stl::BinaryOptions` carries the 80-byte header, and neither can be handed a
field its format does not read, so the inert-field class the one-struct draft
had does not exist. Each field is a **validated newtype** — `SolidName` and
`BinaryHeader`, each refusing at construction with its own error type — so the
three refusals (a solid name outside the single-line grammar's printable
ASCII, a header over 80 bytes, and a header that would read as the ASCII-STL
`solid` keyword, whitespace-skipping and case-folding because sniffers do
both) are properties of the values rather than of the write, and `StlError` is
about the mesh and the sink only. **The ASCII default moved**, by ruling:
`solid <name>` names the solid the file describes, so a producer identity was
the wrong kind of thing in it — the default is now `part`, and the producer
stays in the binary header, which is the field the format leaves free. That
also takes the exported bytes off Q9, which the old default rode on.

*What was measured, and what nothing re-runs.* A cross-tree probe exported
thirteen fixtures in both formats from `origin/main`'s writer and from the
PR's: **all thirteen binary files byte-identical**, and **all thirteen ASCII
files differing in exactly the two lines that carry the name** — the `solid`
opener and the `endsolid` closer — and nowhere else. That was a one-off local
run and **no committed byte-golden of an STL exists to reproduce it**, so
nothing in the hosted matrix re-derives it; what does have a standing guard is
the option path, via literal pins in `export.rs` and `review_m2_pr7.rs`. It was
re-run on the shipped shape rather than carried forward from the first draft —
a measurement is a measurement of a tree, and this one had two of them.
No byte-comparison golden moved,
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
below 70 — and is **scheduled as §D H17**. The
caller-settable-header residue it was scheduled beside, H16, is FIXED by
#732 (above). It is not a leak in S37's sense while nothing
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
| `geom`'s `speed_lower_bound` | ~170 doc lines over ~90 code lines, mostly litigating history |
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

Unclassified siblings went to **H15**, **FIXED by #775**; the `enters.rs`
question was **D5**, answered by #665 (the newtype).

**What H15 found, because it corrects this entry's own arithmetic.** H15 named
three sites. #775's disposition table has **twenty-three rows**, four of which
cover more than one site each — a row count, not a site count, and the
distinction is stated because the first version of this sentence lost it. Two
of the rows falsify records written here. (i) *"Unclassified siblings"* omitted the
**"Still not swept, and why"** list above — three further residues once its
overlap with H15's own third site is removed, two of them still live. (ii) The
sibling table's `topo/src/entity.rs` row is marked **Swept**, and the identical
sentence (*"multi-shell solids arrive at M3"*, false since M3) was live in two
other files of the same crate. (iii) `splitting/mod.rs`'s *"unimplemented until
SSI"* was not one site: the literal phrase stood at **eight** places in `src/`
across `topo` and `geom-brep`, plus two test pins, and **three further members
of the class used none of those words** — `edge_geometry.rs`'s public variant
doc, a test header's future tense about a shipped variant, and `bvh`'s
seam — while the corrected form was already in the file the phrase most often
appeared in. (iv) The not-yet-checked list's *"M6 interval clearance"*
contradicted `DESIGN.md`, which schedules it at **M10** in two places. None of
the three named sites was a lost invariant: `planar.rs`'s outward-normal claim
is TRUE and survives S10 because it reads the walk and never a chart; the other
two were benign rot whose substance is now stated against a name
(`ContactMark::Unmarked`) or against the per-arm retirement rule, rather than
against a date.

**And the method fact this leaves for the next sweep, which is the entry's own
subject.** #775 ran four instruments and the class still had three survivors,
because each instrument's **selection rule** — not its regex — excluded them:
one required the live-scope marker *before* the milestone token (*"SSI arrives
(M3+)"* puts it after), and one *discarded* every clause carrying a milestone
token. A fixed-string re-grep for the retired wording then returned zero and was
read as the class being closed. **A fixed-string grep cannot close a class; it
can only prove one phrasing is gone.** The mechanical guard that can is now in
`geom-brep/tests/intersect_table.rs`: every route-table note is asserted not to
carry the retired vocabulary, on every row rather than only the rung-3 ones.

**Two rows were added after #635's fix**, both from #647's style review. The first
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
- **FIXED by #801.** `profile::k_stats` is deleted. Its seven in-crate
  users (`seg`, `path`, `sugar`, `validate`, `path/verbs`,
  `path/arc_fillet`, `path/family`) and its four `probe` test consumers
  now name `geom_core::k_stats` directly, which is what the shim's own
  docs told them to do. **The population was seven, not "all eight of
  the crate's own decision-making modules"** — the eighth mention,
  `profile/src/lift.rs:636`, is a doc line saying there is *no* funnel
  call site there, so the count was raised by a sentence stating its own
  negation. **Everything the workspace EXCLUDES was swept and then
  BUILT.** `demos/`, `tools/` and `interval-transcendentals/` are
  `exclude`d, so a green root `cargo check` is no evidence about them —
  and *three excluded directories* hold **six crates**, which is not the
  same number: `demos/tour` (with `--features probe`), `demos/wild`,
  `tools/k-lint`, `tools/tess-lint`, `tools/tess-meter` (which does
  depend on `profile`) and `interval-transcendentals`. All six build.
  (#801's first record named three of the six — the stated population
  smaller than the real one, which is the defect this whole unit is
  about, committed inside its own evidence.) **Which of them reached
  what, exactly**: `tools/k-lint` names `geom_core::k_stats` directly
  (`tests/litmus.rs:33`); `demos/tour` reaches it as
  `pncad::geom_core::k_stats` (`src/probe.rs:29`, `src/booleans.rs:68`)
  — **through a re-export**, the same shape as the consumer this PR's
  sweep missed; the other four contain no `k_stats` token at all. None
  named the retired shim, which is what mattered. **The sweep pattern
  still missed one consumer, and that is the transferable part**:
  `profile::k_stats` matched nothing outside `crates/profile`, but
  `crates/pncad/src/profile.rs:53` re-exported the module as `pub use
  ::profile::{k_stats, lift, path};` — a **brace-list re-export names a
  symbol without ever writing its path**, so no path-shaped pattern can
  see it. `cargo check --workspace` caught it (E0432), the sweep did
  not. The re-export is removed and `pncad::profile::k_stats` had no
  consumers, verified afterwards by scanning every `k_stats` token in
  the tree rather than a path. This section's **four** other STILL-OPEN
  bullets are untouched, which is why the heading does not become FIXED:
  `WitnessSlot`, `same_level`/`unreachable_zero`, `Rim`'s twice-stored
  traversal direction, and the D9 `HashSet` determinism paragraph.
  (**Counted as two in the first draft of this record** — the same
  defect this bullet corrects three paragraphs above, committed one
  bullet away from the correction. Conclusion survives, population did
  not.)
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
- The rim-level rule's structurally-impossible arm manufactures its
  error by feeding `f64::NAN` into `classify` and letting the funnel
  escalate — a decision predicate used as a `throw`; `unreachable_zero`
  returns a 4-tuple of NaNs into live flux arithmetic
  (`props/curved.rs`, `mixed_levels` and `unreachable_zero` — cited by
  target name per **S176(a)**; **`same_level` no longer exists**, the
  two rim-level spellings having been unified into `level_coincides`
  by **#877 / S81**, and this bullet named it). **STILL OPEN** — the
  idiom survived the unification unchanged, it is now at one site
  instead of two, and it is D2 (bug-vs-invalid-state) territory gated
  on Wave 0.
- `Rim` stores the same traversal direction twice (`d_u: T` and
  `d_u_sign: Sign`), and the exact one is compared through the tolerance
  funnel — subtracting two exactly-±1 values and banding a result that
  is always 0 or ±2 (`props/curved.rs`, `Rim`'s two fields and
  `du_of_rims`' `props_rim_dir_group` decide — cited by target name per
  **S176(a)**; the line numbers were written against a tree #877 moved).
  **STILL OPEN** — which of the two representations is authoritative is
  a design call. #877 did not touch it: the margin is now levered at
  `RimArms::azimuth` rather than a bare `arm`, which is the same
  comparison at a named lever.
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
other one: `geom/tests/curves/review_m5_pr3_attack_interval.rs` and
`nurbs_interval.rs` carry three containment rows that assert a component
enclosure contains the pointwise `f64` value, and one of those enclosures
is `DInterval { lo: -inf, hi: inf, dec: Trv }`. It *does* contain it —
interval arithmetic brackets the values the expression was defined on
even when the decoration is `Trv`. Making `Bounds` refuse turned
`cargo test -p geom-curves --features interval` (the crate is `geom`
since #705; the command is left as it was RUN) from `116 passed` into
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
  build diverge from the `f64` build. **That last claim is superseded by
  S86**: all three of them do have a poison state readable off the value,
  and all three now refuse on it. It carries **no supertrait**: a body
  needing both doors writes `T: Bounds + CertifiedEnclosure`, which is an
  honest inventory, and a subtrait would put a third `lo`/`hi` in scope
  wherever a compound bound is written.
- The three crossings require `CertifiedEnclosure`, so a `Trv` enclosure
  cannot reach `RingInterval::from_bounds` through them at all.

The three `geom` containment rows pass **untouched** — that was the
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
   mechanical would notice a fourth certified door either: the gate that
   might have is `scripts/gates/bounds-allowlist.sh`, whose matcher does not
   reach a certified door at all.

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
  `crates/geom/src/curves/nurbs.rs:126`,
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
| 4 | Silent `if let Some` discard | **58 sites** across the three Euler modules, **all retired** — 56 by #720 (the two `kfmrh` sites gained typed preconditions and are *among* the 56) and the last 2, `link_half_edges`', by **#736** once its two unproven callers gained the plan-phase link check that makes them provable | D9's "documented garbage-out in release" |
| 5 | Bare indexing that panics, **chosen deliberately** | `nurbs.rs:162`, `hull.rs` `coeffs[j]`, `mesh/chords.rs:465` | "the fail-loud direction" (PR #447) |

Idioms 4 and 5 are **opposite answers to one question**, each argued in
its own module's prose by appeal to the same principle. `geom`
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

*The rule was settled first, and the code has now followed it — in `topo`.*
The addendum opened the `unreachable` lint in both manifests and licensed the
conversion without performing it; **W2c**, scheduled as Track D's **D16**
(#706; it had a verdict here and no row anywhere for two days, which is §D's
fourth ordering rule failing on the section that states it), performed it as
**#720**. Idiom 4 is gone from `euler.rs`/`euler_ring.rs`/`euler_kill.rs`: of
the **58** discards there (the "~60" was an occurrence count; two of them were
ordinary `Option` matches), **56 are now `unreachable!` with a per-site proof
in the message** (#720) **and the last 2 followed in #736**, which is the whole
58. `kfmrh`'s two sites
are *inside* the 56 — they converted only because its plan phase gained the
`StaleKey` checks that make them provable, and two new rows go red if either
check is deleted. The last 2 were `link_half_edges`', held back because **two**
of its callers passed a `prev` read out of the arena that nothing proved:
`split_edge`, and `kef` — whose cycle walk steps `next`, so it proves
`next(he)` and not `prev(he)`. Placed as **D18** and closed by **#736**, which
gave each caller the plan-phase link check symmetric with the one it already
had and then converted the helper. **W2c is done.** **No site was row 5.** Rows 4/5 split on
re-derivation, and a failed key lookup is observed rather than re-derived, so
`debug_assert` never applied to any of them; `assert_euler_postcondition`
remains the only row-5 member in these modules and is untouched.

*What the taxonomy still owes.* Idiom 2's `MissingEntity` router defects and
`AssemblyUnsupported`'s rename were the addendum's remaining unexecuted rows,
in `mesh` and `sweep` — outside `topo` and outside D16. The `sweep` half landed
as **#740** (D2); `MissingEntity` in `mesh` is what is left. S14's disposition
follows from this rule and should be re-read against it rather than re-argued;
S12 is closed.

One correction #720 owes upward, and it is a **class, not an instance**. Three
places name `SolidWithoutShells` as *the* state a failed graft leaves —
`DESIGN.md:1130-1135`, the 37-door allowlist entry at
`review_m1_pr5_internal.rs:312`, and `euler.rs:76`, five lines above a
paragraph #720 rewrote. All three understate it: that is the state of the
**late** `GraftRecertify` refusal, raised after pass 2 with every key patched.
A refusal raised *between* `graft_solids_with`'s two passes leaves entities
holding source-internal keys, and because these are slotmap keys such a key
may either dangle in `dst` **or resolve to an unrelated live entity** — the
live-but-wrong class, which no plan phase can refuse. #720 corrected the
`euler.rs` copy in passing and named the class there; whoever takes S14 fixes
one of three. The `DESIGN.md` and allowlist copies are S14's, not this row's.

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
`geom`'s three containment rows assert that a `Trv`-decorated, unbounded
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
| `sweep/tests/m5_pr11_quad_props.rs`, `dual_lane_keeps_the_closed_form_refusal` | `mass_properties` on a `Dual` body |
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
**forced rather than chosen**: three `geom` containment rows assert that
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

**The hedge now has a place to be collected (Evan, 2026-08-21).** `DESIGN.md`'s
**M10** roadmap entry carries it as an open question — *what does a `Dual`
actually have to do*, and clean up the `Bounds` / `CertifiedEnclosure` split on
that answer. Recorded there rather than answered anywhere: *at least for now*
was never given an expiry, and a hedge with no owner is the state this scan's
own closing rule refuses. **Anything that deletes a guard on the strength of
this ruling states that exposure rather than resolving it.**

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

**Not a live wrong answer — of the skip S49 itself names.** (One of its residues
was, and the two verdicts are about different skips: see the H14 subsection
below, which is the arm-2 `bridged` skip, not this arm-1 one.) The
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
members wants its own PR and its own `--manifest-path` check.

### H14 — FIXED by #737. The two residues, and the one that was a live wrong answer

**This verdict does not contradict the *"Not a live wrong answer"* paragraph
above.** That one is about the arm-1 planar × planar skip S49 was raised on;
this one is about arm 2's `bridged` skip, a residue #637 recorded and did not
fix. S49's escape — the *pair* was unowned but the *body* refused anyway via a
neighbouring pair — does not transfer, because arm 2 reports at solid
granularity and no other arm reports on a solid pair at all.

#637 left two residues, both recorded in its own body under *"Found, NOT fixed"*.
Its scope cell in the Track C roster cited `splitting/rules.rs:268` for both,
which is **C-R11**'s worked example: on the merge base that line is
`let face_data = body.get_face(face)…` inside `face_extent`, six lines above the
`continue; // an empty loop contributes no extent` that IS residue 2 — so the
citation resolved to the right function for one residue and to nothing for the
other. Checked rather than complied with, per C-R11.

**Residue 1 — arm 2's `bridged` skip — was S49's defect with a live wrong answer
attached.** The skip suppressed the containment test for any solid pair linked
by a contact record, on the ground that such a pair is *"under the confirm
pass's examination"*. The confirm pass asks of each record whether that record's
own coincidence is real; no variant of `ContactRecords` states a placement
relation, so it never asks where one instance sits relative to another.
Measured on a 1 m cube sitting wholly inside a 4 m cube, flush at `z = 0`, with
its four bottom corners declared v-on-f and every record confirming:
`validate_pseudomanifold` answered **`Ok(())` declared** and
**`CensusUndecidable` on the solid pair undeclared**. Declaring a true contact
was what switched the examination off. The skip is deleted — a solid pair
carrying records is examined exactly like one carrying none — and the arm's
docs state the vocabulary that *would* license a deferral (C6's recorded
gate-skips, a statement about placement) and that keying one on contact again is
the unsound direction. The only row the skip ever had bridged its nested pair
with a **bogus** record and measured the confirm pass's staleness refusal, so it
stays green under the reverted fix; the new row forbids `StaleContactDeclaration`
and `ContactContradicted` outright.

**Residue 2 — `splitting/rules.rs`'s `face_extent`** walked past a
`LoopBoundary::Empty` loop with *"an empty loop contributes no extent"*. The arm
must OVER-estimate the displacement it divides out of, so an isolated **ring**
vertex now contributes its distance and an empty **outer** loop — an unbounded
face, which has no finite arm — refuses. The zero answer was loud at one caller
by accident (`apply_rule_a` gates on the extent being definitely positive) and
silent at `chord_join`'s two.

**The sweep found a third instance of residue 1's shape in the arm above it**,
and this one was S49's own sentence with the sides swapped: arm 1's v-on-f
deferral tested `planar_face_bridged(a.face, b.solid)` — a record naming the
planar face and any vertex of the other **solid** — while the finding it
suppresses is about the **face pair**. One declared interface silenced that
planar face against every other face of the same solid. It now requires the
record's vertex to be a boundary vertex of the other face of the pair.

The reviewer's third bullet — arm 1's placeholder-NURBS exclusion — is **not**
protection by accident, and it is stronger than "only caller": `census_and_certify`
is `pub(crate)` in a `pub(crate)` module, so nothing outside `topo` can reach it
by any re-export or alias, and its one production caller
`validate_pseudomanifold` runs it solely when `tier3_local_checks` came back
empty, whose check 1 refuses every placeholder face. So a placeholder there is
**a state no production path can mint** — which is not "an unreachable state":
the in-src scaffold rows are in it, deliberately, by calling the census directly.
Both facts are now at the skip, with the drift direction named.

**Both reviews then found that the arm 2 half of that had been left undone, and
it mattered.** `boxes::face_box_rule` routes a NURBS placeholder to
`ControlNet`, whose net is poison points; census's fold is `min`/`max`, which
propagate NaN, so `reach_box` answered `Some((NaN, NaN))` — a value that is
neither a claim nor a refusal. Every containment margin taken against it decides
neither sign, so arm 2 fell out at its **in-band** refusal having compared no
geometry at all, and the typed *"unclaimable extent"* refusal was dead code for
the case it was written for. Measured: moving one scaffold sheet a kilometre away
produced the byte-identical refusal. The PR had reported those refusals as the
arm measuring two overlapping sheets, which was false as to mechanism.

`reach_box` now answers `None` for a placeholder net, which is what its own
contract says an unboxable kind answers; the whole solid becomes unclaimable and
the typed refusal fires. **It is `None`, not an exclusion**: dropping the face
from a solid's reach would shrink a box required to be a superset and could clear
a body nested inside it — the unsound direction. That asymmetry between the two
arms (arm 1 excludes the FACE, arm 2 refuses the SOLID) is written at both sites,
and `a_placeholder_seed_leaves_the_containing_extent_unclaimable` pins the typed
verdict and the position-independence together.

**Two more skips in the same function were brought to the bar this record
sets.** Arm 1 dropped a face with no boundary vertices — an unbounded face — out
of the sweep with a bare `continue`, in the function whose header forbids exactly
that; it now refuses `CensusUnsupported`. And the same-`SurfaceKey` deferral
tested only that the conformal arm *walks* the pair: that arm `continue`s on a
same-sense pair without deciding it, so a cross-solid same-key **same-sense**
pair was deferred by one arm and dropped by the other. The deferral now requires
opposed senses, which is the configuration the conformal arm returns a verdict
on; the rest fall through to the box test.

**The capability cost is real, and it is kernel work rather than style: it is
filed as issue #750**, *"Containment examination is extent-box coarse: no
non-convex-container assembly can certify after #737"*. Deleting the `bridged`
skip means no assembly with a non-convex container — L-bracket, blind bore,
pocket, cavity — can pass `validate_pseudomanifold` by any declaration, where
before a declaration was the only route it had. Measured on an L-bracket with a
part resting flat on its inner wall, four v-on-f records all confirming, plain
`Plane` surfaces, no placeholder anywhere: `Ok(())` at base, one
`CensusUndecidable` at head. That is not a false refusal — the arm genuinely
cannot separate it from the embedded cube — but the cause is the coarseness of an
axis-aligned extent-box test, not interference, so C6's recorded gate-skips
(scoped by `ASSEMBLY-DESIGN.md:199` to deliberate interference fits — overlapping
shells, which the L-bracket has none of) are **not** the remedy: a gate-skip would
suppress the refusal rather than fix it, and suppressing it re-creates the class
this unit removed. The pointer that named them is withdrawn from the code and
from this record. **This gets no §D row** (Evan, 2026-08-20: *"the cost sounds
like it's real kernel work and not style; if my read is accurate, file a github
issue about it instead"*); #750 carries the reproduction, the base-vs-head table,
the extent-box diagnosis, and the style lane's `declared.faces` same-solid
residual. It also records the one **negative result** this unit produced, so it
is not retried: the obvious record-free narrowing that reuses data already in
`Geo` — deriving a separating plane from the container's own face planes — is
unsound **exactly** on the class that needs it, since extending the L-bracket's
inner wall `x = 1` puts points at `x > 1, y < 1` outward of that plane and inside
the material. A separating face plane certifies "outside" only for a convex
container. That converts this unit's *"minimality was never proved"* disclosure
into a stated negative result about one named candidate — not a proof that the
skip's deletion is minimal.

**No instrument was added, and the recurrence is not addressed.** This defect has
now been found four times — S49, #637's two residues, and this PR's third
instance in arm 1 — each time by someone reading the code. What this PR leaves is
a paragraph stating the bar (*a deferral names an arm that asks the SAME question
about the SAME pair*), not a token, a convention, a test or a gate; a `continue`
with a prose justification remains indistinguishable to grep and to CI from one
without. Residue 2's class is measurably larger than its instance: a
differently-shaped sweep for `LoopBoundary::Cycle { first } = … else { continue }`
returned **29 sites in 18 files** when the style lane ran it; re-run at this
unit's merge head (`eef045ac`) with the pattern
`LoopBoundary::(Cycle|Empty).*else \{ … continue \}` over `crates/`, it returns
**26 sites in 17 files**, of which this unit audits the three inside
`census.rs`/`rules.rs` and none of the rest. **What that pattern cannot match**,
and what therefore is not in either number: the same handling written as a `match`
arm rather than a let-else — 15 further sites, two of which
(`topo/src/seqgen.rs:1080`, `editor-core/src/names/emit.rs:422`) are
`LoopBoundary::Empty { .. } => {}`, residue 2's exact reading in match form,
uninspected here; `continue`s that drop a loop for a different reason (an arena
miss); and the same class over any other enum. `scripts/gates/` holds three
allowlist-shaped gates, and since this unit opened, one **derived-census** gate
(`probe-suite-census.sh`, landed on main 2026-08-20) — a gate that derives its
population rather than listing it, and refuses an empty answer or a drop below a
floor. That is closer to the shape a deferral register would take than an
allowlist is, and it is the nearest existing precedent for building one.

**Verdict:** ACCEPTED (Evan, 2026-08-18) — *"should be scheduled but i have no
opinion on when"*. The jurisdiction call was part of the unit, not a prerequisite
decision, and #637 made it and argued it in the code.

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

## S57. The `readback` class is alive one crate over, and the "one door" guard cannot see it

- **Where**: `crates/editor-core/src/names/emit_topo.rs:48`,
  `crates/sweep/src/fillet/build.rs:247`,
  `crates/sweep/src/fillet/battery.rs:175`, `:183`, `:189`
- **Confidence**: sure
- **Raised by**: the review of #697, 2026-08-20 — the PR that rehoused
  `topo`'s body-wide accessor module out of `sweep` on the rule **a door lives
  in the crate whose types it reads** — by running the `face_pose`-shaped sweep
  that PR declined to run and stated as its blind spot. **The blind spot was
  real and it hit.**

**A sixth copy of the readback walk, in the crate the fix was about.**
`emit_topo.rs:48`'s `face_plane` is `get_face` → dangling refusal →
`get_surface` → dangling refusal → destructure `Surface::Plane`. It is not a
literal copy — it folds `sense_sign` and refuses non-planar carriers — but it
refuses with `NamingError::Emission { what: "face_plane: dangling" }`, which
is **verbatim the `&'static str`-names-the-lookup defect #697 eliminated**,
still standing in `editor-core`, the crate whose dependency on `sweep` the
whole finding was about.

**Four `Body`-only accessors housed in an op crate — the same misplacement
#697 fixed, one crate away from where it was looked for.** `fillet/build.rs`'s `outward_of`
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
## S58. FIXED IN PART by #714 — "This face's domain is an iso-rectangle" was re-derived per consumer, in three representations, and no two agreed on what it meant

- **Where**: `crates/geom-brep/src/props/curved.rs` (`du_of_rims` /
  `props_du_consistent` — by target name per **S176(a)**, the line
  moved under #877), `crates/mesh/src/curved.rs`
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

**Executed by #714 (2026-08-19). It closed #649 as prescribed, and it
did NOT close #649's CLASS — read the scope statement at the end of
this block before citing it.**

The predicate is `geom_brep::props::curved::require_rims_at_extremes`,
decided under the name the torus already used — **`props_rim_level`**:
*every rim sits at one of the face's two extreme `v`-levels*. `w(v)`
changes only where a rim is (between rim levels the boundary is
meridians, which move no `u`-endpoint), so the rule establishes
`w ≡ Δu` — **one** of the two premises `area = r·Δu·(hi − lo)` and its
per-kind siblings integrate; the other is the `v`-extent, and it is NOT
established here (see SCOPE below). Cylinder, cone and sphere adopt it, each passing its own two
extremes in its own `RimLevel` representation (`Length` bare;
`Unit` direction pairs levered — sphere `× R`, torus `× minor`); the
torus's private loop **became** the shared call rather than a second
copy. `du_of_rims` keeps its M5-PR-9 span-sum check, which now pins the
`Δu` VALUE of a domain already known to be a rectangle instead of
standing in for the shape test — its doc says so, and a multi-arc-rim
row proves PR 9's reason survives.

Measured: the plus domain now refuses on **all four** kinds
(cylinder/cone/sphere were accepting it at −28.57 %/−29.91 %/−29.30 %),
the tall-arm, double-plus and Z-staircase rows with it, and every
control iso-rectangle still measures **exactly**. Both proven doors are
closed with #649's own fixtures, committed under
`crates/step-import/tests/fixtures/iso-rect/` with their generator and
pinned in `tier_gate`'s corpus: `cross.step` refuses at import, and
`merge_coplanar_faces()` on `xsplit.step` — which used to turn an exact
`8.96e-7 m³` into `7.2533e-7` with `pad = 0.0` — now refuses instead of
mis-measuring.

Over-refusal: nothing legitimate newly refuses. The wild and FreeCAD
corpora, the whole `tier_gate` matrix (every committed STEP file × 3
ambient ε × 3 ε_in), the volume oracle at its `1e-11` budget, and the
`geom-brep` / `topo` / `mesh` / `sweep` / `step-import` suites are all
green. Exactly **one** disposition moved — `cross.step`, from Pass to
Refused. `tee.step` was already refused (on `props_du_consistent`, its
one-sided arm making the span sums disagree); only the REASON in its
refusal string moved, which is why those rows pin the reason.

The "zero refusals" evidence is *ran and passed*, not *never reached*,
and structurally rather than by instrument: every curved flux/area
closed form calls the predicate before it integrates, so a curved face
that produced an area is a face where it ran and returned `Ok`. Its
blind spot, stated (§C15): rimless sphere bands and
`boundary_material_sign` are exempt, planar and quadrature-lane faces
never reach it, and neither does a face on a body refused before check
7 runs. The COUNT is not instrumented.

**The third consumer's prose was also wrong, and that is fixed —
twice.** `ValidationError::VolumeUncomputable`'s doc read *"at rest
every M2-constructible body computes; this is corruption surfaced
loudly"* — D2-addendum **row 1** framing, falsified by an executed
counterexample: an imported keyed shaft is not corruption. #714's first
pass replaced it with a flat **row 2** claim, which is wrong the other
way, because `source: MassPropsError` has five arms. It is now stated
PER SOURCE: `Face` is row 2 and the reachable one; `Corrupt`,
`NullScaffoldEdge` and `RingOnCurvedFace` document themselves as
corruption or mid-surgery and are row 1; `Band` is neither. Its closing
sentence used to assert an invariant the doc site does not enforce, and
now names what actually makes the row-1 arms near-unreachable — the
`if errors.is_empty()` gate on check 7's call — and says that is a
sequencing fact, not an invariant. `mass_properties`' own `# Errors`
doc carried the same falsified claim (*"bodies that pass the structural
tiers and were built by M2's public operations always compute"*) one
crate-module over, and is fixed with it: `merge_coplanar_faces` is the
public operation that falsifies it.

**Two residues, both PLACED as rows** (§D ordering rule 3 — a lane's
own residues are rows, not footnotes; they were footnotes here until
#714's style review said so):

- **`mesh`'s half of the fragmentation still stands** — issue **#726**,
  and §D row **C11**. `entries_off_bbox` answers a second question the
  face predicate cannot (#653's walk consistency; the bar is spatial
  for that reason), so only the first question folds in. Its prose now
  names the shared predicate rather than calling the agreement a
  coincidence. **Nothing was loosened** — #648 is what makes tightening
  `props` safe at all.
- **The tier-property question is answered "no, and here is the real
  gap"** — issue **#727**, and §D row **C11**. The predicate is a
  **capability boundary** (D2 row 2), not a validity property, so a body
  that trips it is *valid* and a tier property asserting otherwise would
  have to be deleted the day the certified-quadrature lane can measure
  such a domain. What the question is really pointing at is that `mesh`
  and (once its curved-pierce door lands) the boolean are still
  protected **transitively**, through tier 3's check 7 calling
  `mass_properties` — the same shape as the pre-#648 mesher. The thing
  worth writing down is which door owns the refusal, not a new
  invariant.

**SCOPE OF THE FIX — why this reads FIXED IN PART.** #714's
adversarial review broke the sufficiency argument on the sphere, with
an executed at-rest counterexample, and the residue is **issue #723**.

What the predicate establishes is `w ≡ Δu`: the domain's `u`-measure is
constant across the open `v`-interval. `area = r·Δu·(hi − lo)` needs a
**second** premise that nothing states — that the `(lo, hi)` handed to
it is the face's true `v`-extent. The **torus is immune precisely
because it does not use `min_max`**: its extent comes from the anchor
meridian's stored span. The other three take theirs from `min_max`,
which folds edge ENDPOINT levels only. On the sphere that is
falsifiable: `sphere_boundary` certifies a meridian's CARRIER is a
great circle, not that the traversed ARC stays on one meridian half, so
an arc can cross a pole — `u` jumps by π mid-edge and the latitude
reaches ±1 in the arc's INTERIOR, unseen. Through `import_step`: tier 3
**green**, `volume = 1.858e-7` against exact `3.518e-7`, **`pad = 0.0`,
−47.19%** — more than twice #649's 19%. The identical solid without the
extra vertex is refused (`DegenerateFace`), so **one added vertex turns
a refusal into a wrong certified volume**, which is #649's second door
in the other direction.

`require_rims_at_extremes` passes that face **correctly**, at margin
exactly 0: the domain genuinely IS an iso-rectangle. So:

- **Fixed**: the fragmentation this finding names, on the `props` side —
  three derivations to three strengths became one named predicate with
  one margin, one refusal name and one justification. #649's own
  fixtures and both of its proven doors are closed.
- **Not fixed, and not this unit's**: the wrong-certified-volume CLASS.
  It is open on the sphere at −47% (#723), pre-existing and carried
  across untouched — #714 carried the rim rule to cylinder/cone/sphere
  and did **not** carry the extent derivation with it. Nothing in this
  entry, in `props/mod.rs` or at the predicate may be read as closing
  that class; all three now say so and cite #723.

---
---
# The second scan (2026-08-20) — the fixes, and the ground the first scan never covered

**Merged into this document on 2026-08-20**, from `docs/SMELL-SCAN-2-2026-08.md`,
which is deleted and recorded in `docs/DOC-LEDGER.md`. It was written as a
separate file to avoid colliding with work in flight; the collision was real,
and separation was the wrong fix for it — **one register, one ID space, one
schedule**, which is the whole argument of this document's §C3. Its open
findings follow, unedited except where a claim about *this* document was wrong;
its §C is folded into §C, and its starting-order and coverage sections are
deleted with the first scan's.

**The merge corrected one thing neither author could see from inside their own
file.** The second scan's §C said it continued this document's process
numbering *"at C15"*, and this document's forward pointer said the same. §C
already ran to **C17**. The eight new observations are therefore **C18–C25**
here, and the renumbering is noted where they start. Two documents agreed on a
number that was wrong in both — which is §C13's shape at the level of the
register itself, and worth more than the two minutes it cost to fix.

**Nothing here has a verdict yet.** The second scan shipped every
`**Verdict:**` line blank, and verdicts are Evan's. Until they are ruled, a
finding below is a *question worth answering* and no more — the same contract
the first scan opened with.



**Status: REPORT ONLY.** Same contract as
`docs/SMELL-SCAN-2026-08.md`: nothing here is ratified, nothing is a
commitment, no code was changed, and no finding proposes a specific fix.
A finding is a *question worth answering*, not a defect.

**Why a second document.** The first scan is now owned by live fix
tracks (`docs/SMELL-C-LOG.md`, `docs/SMELL-E-LOG.md`) and by §D's
schedule; appending to it would collide with in-flight work. IDs
continue the same series — **`S59` onwards** — so there is one global ID
space and a citation never means two things. `S45`–`S48` remain reserved
in the first document.

**Scan base: `0714d540` (main, 2026-08-20), 916 commits after the first
scan's base `4258584`.** Line numbers are as of that commit. Claims, not
line numbers, are the content.

**Method.** Thirteen parallel scopes, in two kinds:

- **Fix audits (9 scopes).** Code rewritten *in response to* the first
  scan, read as a diff against `4258584`, looking specifically for what
  the fixes introduced. This is the higher-yield half: a fix pass
  touching a file is a fix pass with the file open, and the recurring
  outcome below is that it swept the reported instance and left the
  sibling.
- **New ground (4 scopes).** Code the first scan explicitly excluded:
  `interval-transcendentals/`, `demos/`, `tools/`, `scripts/gates/`,
  and the test files added since the base.

Every scope ran the style brief's stance and eight questions as its
method — the brief that came out of the first scan's process
observations. It has since been split into
`docs/prompts/reviewer-style-lane.md` (the reviewer's document: §1 the
stance, §2 the questions, §3 what a finding looks like) and
`docs/REVIEW-STYLE-DISPATCH.md` (dispatcher notes); the agents ran
against the pre-split `docs/REVIEW-STYLE-BRIEF.md`, whose §2/§3 are the
current §1/§2. Citations below use the live paths. **This is the brief's
first use at scale**, and §C reports on how it did.

**What I verified by hand.** Ten of the highest-stakes claims, marked
**[verified]** where they appear. Everything else carries the reporting
agent's own confidence label (`sure` / `likely` / `unsure`), unmodified.
Two claims in my own dispatch briefs were wrong and the agents caught
them; both are recorded in §C, because "the coordinator's premise was
wrong and the agent checked it anyway" is a process result.

**Independent corroboration is called out where it happened.** Five
findings were hit by two agents working from different files with no
knowledge of each other; that independence is the strongest signal in
this report.

---

# Second scan · Tier 1 — act on these

## S65. The #678 watertightness backstop is compiled out of every build that ships a mesh

**[verified]** **OPEN — Evan's decision, and #872 equipped it rather
than taking it (I-R1).** Cited by NAME rather than by line, because
the line has already moved once under this finding: the original text
cited `curved.rs:306`, which was `:273` at `68921183` and `:278` after
#872's header edit.

> **Premise update, 2026-08-21 (#895) — the heading's *"every build"* is
> now false in this tree.** The root `Cargo.toml` gained
> `[profile.release] debug-assertions = true` (Evan's directive). `cfg`
> `debug_assertions` follows that setting, so `curved.rs`'s
> `#[cfg(debug_assertions)]` block **is compiled in and does run** under
> `cargo build --release` from this workspace — verified against that
> block specifically, by planting a failing `debug_assert` as its first
> statement and watching a release binary panic inside it, and by the
> same build running clean under
> `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false`. Census corroborates:
> 71 rustc invocations under `--release -v`, 48 with
> `-C debug-assertions=on`, the other 23 host/build units that do not
> build under `release`.
>
> **This note records the premise, not the verdict.** The stanza is a
> pre-publish posture and sits on `DESIGN.md`'s *Before publishing* list
> to come back out, at which point the debug/release asymmetry returns.
> **The ruling on S65 itself is #884's** — including which D2 row the
> non-manifold state is, which is the question the option-B correction
> below says actually decides it. Nothing below is edited here.

`crates/mesh/src/curved.rs` — the re-derivation in `tessellate_curved`'s
emit pass that catches the #678 class is `#[cfg(debug_assertions)]`. The class #678 named is a
*silently* non-watertight mesh returned as `Ok`. `tessellate` does not
run `check_mesh`, and `rg check_mesh` finds no consumer outside
`crates/mesh`, `stl`, `topo` and `sweep` tests — no demo or tour row
runs it either. So in a release build the entire guard for the class is
`pole_columns`' three-line `if has_pole && nu == 2`.

Two narrowings compound it. The filter is
`poles.contains(&a) || poles.contains(&b)`, but `crates/mesh/src/trimmed.rs`
(the degenerate-triangle note in the trim CDT harvest) names **two** sources of "one
repeated mesh id at two distinct UV locations" — chart singularities
and the full-2π seam double-traversal — and the seam case, held off by
an arithmetic argument (`nu >= 8` from the π/4 sagitta cap) rather than
a floor, is the half with no mechanical check, in the lane that
actually has seams. And the assert is per-patch, so cross-face
identification is out of scope too.

**What #872 did:** the module header no longer presents the floor and
the assert as a pair without saying one is absent from release — it
says which build each holds in, and points here. `lib.rs`'s copy of the
paragraph says it too, as do `lib.rs`'s two claims that
`validate::check_mesh` backstops the mesh: it does, and **`tessellate`
does not call it**, which is why the class is silent. **The header
states the asymmetry; it does not resolve it.**

### The question, and it is three-way rather than two

**Option A — stay debug-only.** Cost 0. A release build carries the
`nu` floor and nothing else for a class whose failure mode is a
corrupt STL that no error reports.

**Option B — re-derive in release, and REFUSE typed.** That is a
**behaviour change** — bodies that today return `Ok` with a silently
non-manifold mesh would start refusing.

> **This option's D9 argument was WRONG and is corrected here (2026-08-21).**
> It read *"D9 says the kernel never panics on any input"* as forbidding a
> release panic outright, and concluded a typed error was *"the only form
> consistent with D9."* **It is not.** D9's rule is scoped to states an
> **input** can reach; the **D2 addendum**'s rows 4 and 5 make a panic the
> *ratified* mechanism for a state that can only be a kernel bug. So
> `assert!` is not excluded by D9, and B is not the only D9-consistent
> option. **The phrasing that produced this error is fixed at
> `DESIGN.md`'s D9 bullet in the same PR as this correction.**
>
> **What actually decides it is a question this row never asked**: which
> D2 row is the non-manifold state? **Row 1** (reachable by input and
> invalid) → a typed error, and B is right for the right reason. **Row 5**
> (kernel bug, detectable only by re-derivation) → the assert is already
> the correct mechanism and only its *release* reach is in question.
> **Left open deliberately — S65 is Evan's decision in #884** and this
> note corrects a false constraint on the choice rather than making it.

**Option C — keep the floor, widen it.** The two narrowings above are
independent of the debug/release question: the seam case has no floor
at all, and cross-face identification has no check in any build.

### Option B priced, by measurement

Tree `68921183` (= `main` at `5d4b88ab` plus Track I's docs-only
constitution; `crates/mesh/src/curved.rs` byte-identical to
`5d4b88ab`). `cargo test --release`, one container, 40 reps per row
after a warm-up, `mesh::tessellate` end to end. Priced by **making the
guard real in release** — dropping the `#[cfg]`, turning the
`debug_assert!` into an `assert!` — not by modelling it; the patch and
the bench were reverted and are not committed.

| body | δ | triangles | baseline (ms) | guard live (ms) | Δ |
|---|---|---|---|---|---|
| ball | 0.05 | 224 | 0.174 | 0.203 | +17% |
| ball | 0.01 | 1 216 | 0.61 | 0.71 – 0.81 | +16 – 31% |
| ball | 0.002 | 6 224 | 3.32 – 3.37 | 3.96 – 3.98 | +18 – 20% |
| ball | 0.0005 | 24 616 | 14.8 – 15.5 | 16.6 – 16.9 | +9 – 13% |
| cone | 0.05 | 116 | 0.061 | 0.067 – 0.071 | +10 – 16% |
| cone | 0.01 | 484 | 0.211 | 0.255 – 0.260 | +21 – 23% |
| cone | 0.002 | 2 244 | 0.98 – 1.02 | 1.25 – 1.28 | +25 – 28% |
| cone | 0.0005 | 8 964 | 4.46 – 4.88 | 6.03 – 6.19 | +26 – 35% |
| washer (no pole) | 0.01, 0.002 | 308, 684 | 0.52, 1.32 – 1.45 | unchanged | **0** |

Ranges are min–max over two or three runs of the same binary. These are
wall-clock figures on one box; `mesh/lib.rs`'s standing caveat about
such numbers applies verbatim.

**Read them as:** ~10–30% of tessellation time on a body whose curved
faces **all** carry a pole, and **exactly zero** on a body with none —
the block is inside `if has_pole`. The ball and the cone are the worst
case that exists, not a representative part. And the price is of *this
implementation*: it allocates a `HashSet` and a `HashMap` per pole
patch, which is most of what the table measures. A non-allocating form
(pole-incident edges into a small `Vec`, sorted) would be materially
cheaper, so the table is an **upper bound on B**, not its floor.

**Verdict:**

## S66. FIXED IN PART by #876 — the style halves close; the over-width itself remains as issue #862

**Split, and the split is the point.** The **logic** half — the cylinder slab
arm's over-width by a full radius along its OWN axis, and the slab's
single-endpoint bracket reads (`along(origin.x.lo(), axis.x.lo())`, and
`hi().abs()` at the conic arm, which is not an upper bound on `|x|` when the
lower endpoint is larger in magnitude) — **stays open as issue #862**. #876
changed no arithmetic. What closed is the two style halves.

**The suite could not go red for a box that is too big, and it was worse than
the finding said.** Re-derived at 882 lines (one of the two cited ranges had
drifted; the rows are `:596-616` and `:623-635`): with `face_box` returning
`[-1e300, 1e300]` on every arm that has a box, **`boxes.rs`'s own six rows pass,
and so does the whole `topo` lib suite — 448 rows**. Five rows in `topo/tests/`
red, all at that catastrophic magnitude, none inside the module whose contract
it is. **Four rows, one per arm** —
`the_*_arms_box_is_exactly_the_construction_its_rule_states` — each state their
arm's box as a **formula in the fixture's own parameters**, the construction the
rule's own docs state written out, and pin `face_box` to it on all six faces in
**both** directions. One row per arm so a red names its arm before the message
does. They red on `[-1e300, 1e300]`; the cylinder row also reds on one extra
radius of axial widening — **#862's own magnitude** — while every locus row
stays green.

**The tests module's own header said the opposite**, and that was the half that
would have made this finding read as closed: it certified the locus family as
*"the assertion that degrades correctly"*, which is precisely the reassurance
S66 refutes, forty lines above a suite that #876 had just proved inadequate. It
now states **two** contracts needing opposite assertions, says in as many words
that a locus suite is not an adequate guard for this module, and says which
family checks the rule against the geometry and which checks the code against
the rule.

**Three terms are deviations rather than construction, and each is named as one**
— `axial_overwidth`, `redundant_axial_pad` (the arm pads the axial range and
`padded` pads every coordinate again, so the axial one carries the pad twice)
and `overwide_half_extent`. All three are **#862**'s, all three are stated at
their rule so the rows transcribe the RULE and not an implementation detail, and
each reds at a named line the day it is fixed. A term called `amplitude` would
have read as the quantity's proper name; the idiom only works when the term's
name carries the accusation.

**A second over-width was found in the same module and reported to #862, and
it is the sharper of the two.** `EdgeBoxRule::ConicAmplitude`'s per-coordinate
`|û_i|·a + |v̂_i|·b` is the triangle-inequality bound over the conic's extent
`√((û_i·a)² + (v̂_i·b)²)` — so **the box is not a function of the LOCUS**.
Measured: a **unit circle** named from a `u_ref` at 45° gets `x ∈ [−1.414,
1.414]` where the same circle — same centre, axis, radius — named from
`u_ref = x̂` gets `[−1.000, 1.000]`. A box rule that reads its own
parameterization is not a rule about the entity, and the module's one-sided
contract (*"contains the entity's whole locus"*) is silent in exactly the
direction that hides it.

**In-tree bodies already take the wide branch — not reachable-but-unexercised.**
Measured on `s16_box_soundness.rs`'s `cylinder()`, built through the public API
(`Profile` → `extrude`): **four of its six circle carriers** carry
`u_ref = (∓0.5, ±0.866, 0)`, a widening factor of **1.366** in both x and y, so
its cap faces' `BoundaryHull` claims `x, y ∈ [−0.683, 0.683]` against a true
`[−0.5, 0.5]`. The mint sites make it structural rather than incidental: the
plane×cylinder rim inherits the cylinder surface's own `u_ref`
(`geom-brep/src/intersect.rs:602`), and the plane×sphere circle derives one
from the seam or polar candidate (`:716`) — neither axis-aligned in general.

**The exact box already EXISTS and SHIPS, and has no production caller.**
`geom::curves::boxes::circle_arc_aabb` computes `Aᵢ = √((û_i·a)² + (v̂_i·b)²)`
outward-bracketed **and** restricts to the certified span — tighter on both
counts — is `pub`, and takes the two params `EdgeCurve::params()` already hands
`edge_box`. `git grep` finds its callers in `crates/geom/tests/` and nowhere
else. So this is not *"someone should write the tight version"*: the tight
version ships while production calls the hand-derived one. **Recorded as its own
row, `S235`**, because it outlives #862 — after both tightenings land, *why were
there two constructions and why was the correct one the unused one* is still
unanswered, and that is S16's subject at the curve level, a fourth instance
never counted.

**"Looseness is free" is a claim about a DOOR, and the finding's count was low.**
Not two of three: **four** doors read a box from this module and **three** read
over-width as a refusal — `separation.rs` (non-overlap IS the grant),
`census.rs` arm 2 (a false `CensusUndecidable`) and **`boolean/ops.rs:1486`**,
the sphere-extent fallback's cylinder arm, which the finding did not count. Only
`boolean/reduce.rs`'s C10 tree prunes. The header now states the property rather
than the roster, and the roster is **computed**:
`every_door_that_reads_a_box_is_inventoried` walks `topo/src` and pins, per
file, the call sites of **both** rules — face **and** edge. That matters: the
header's claim is about *a box*, the sharper over-width lives on the EDGE rule,
and `edge_box` has doors of its own including a refusing one
(`boolean/ops.rs:1421`, the extent scan's near-boundary test). A face-only walk
would have attributed the conic arm's cost to a list computed for a different
function and let an edge-box-only door land green.

**Raised on the way: `S232`, `S234`, `S235`.** `S232` is the same
*"never prunes"* sentence six times over in
`crates/geom/src/{surfaces,curves}/boxes.rs`, one crate below the same doors —
**routed to Track H**, whose scope those files are. `S235` is the unused exact
box above. **`S234` is this unit's own**: the door inventory computes the
roster's KEYS and recites its direction column, which is the whole content of
the header's argument — S66's shape one level up, inside the fix for S66 — and
it is a row rather than a disclosure because per Q4/Q6 a deviation owes an
owner.

**The module contract itself carried no caveat, and a consumer recited it as
live.** `boxes.rs`'s one-sentence contract (*"every box this module returns
contains the entity's whole locus"*) was unqualified while #862 holds cases
where it is false under `Interval`, and `separation.rs:20-22` restated it as the
premise of its certificate — **at the one door where non-overlap is a GRANT**.
That is S97's second sub-case at a citation site the prose sweep had not
covered: the over-width half had been given the treatment the unsoundness half
had not. Both ends now carry #862, and `separation`'s door states what
over-width costs it as well.

**Nothing in this file separated in z.** `s16_box_soundness.rs`'s counter-row
separates at `cx = 3.0` in **x**, the axis the widening does not touch;
`a_body_above_the_cylinder_is_still_cleared_by_containment` puts the probe over
the top cap and radially inside the wall, so z is the only axis that can clear
the pair. **The near case is red on this tree and was handed to #862 rather than
tuned green**: a probe at `z ∈ [1.05, 1.45]` — entirely above a solid that ends
at `z = 1` — is reported as `CensusUndecidable { a: Solid, b: Solid, what: "one
instance's extent box inside another's" }`, #862's predicted wrong answer
observed. The landed row sits at `z ∈ [2.0, 2.4]`, clear of the over-width, and
its doc says the near case belongs here once #862 lands.

## S68. The W2c discard sweep stopped inside the function it was editing

**[verified]** W2c's claim, restated in `crates/topo/src/euler.rs:24`
(*"A mutation phase announces a failed lookup rather than discarding it,
at every write"*), in `review_m1_pr2/release_corruption.rs:33` and in
`ci.yml:740`, is scoped to `euler{,_ring,_kill}.rs` plus
`link_half_edges`.

`split_edge`'s mutation phase was edited in this same diff — two
`unreachable!` conversions landed at `crates/topo/src/split.rs:279` and
`:286` — and ten lines below them sit three silent `if let Some(..)`
discards at `:287`, `:294`, `:299`. `movefac` — named in
`docs/DESIGN.md:1117` as one of the four structural mutators the closure
property rests on — carries three more (`movefac.rs:150,162,166`), and
`attach.rs:81,252` two.

The sweep's own scope sentence is what made the sibling instances
invisible. And `split_edge` is not a sibling — it is the same call.

**Verdict:**

## S69. `kfmrh`'s shell-fusion form is outside the fuzz catalog, and the `Ledger` counts solids, so it cannot notice

S15's finding was that `split_edge` shared `mev`'s Euler vector while
sitting outside the fuzz catalog; the fix added `SplitEdge`. Meanwhile
`kfmrh` gained a cross-shell **shell fusion** form
(`crates/topo/src/euler_ring.rs:809` — re-homes faces, kills a shell,
`ArenaDelta { shells: -1 }`), and both `kfmrh_candidates`
(`seqgen.rs:502`) and teardown's re-make search (`:1103`) filter on
`face1.shell == face2.shell`, so the fusion branch is **never
generated**.

The `Ledger` cannot notice either: its `s` is the *solids* count
(`seqgen.rs:283`), not shells, so a shell-count error passes property
(b) silently.

Related, same operator: `euler_ring.rs:846-862` hands
`assert_euler_postcondition` an `ArenaDelta` computed as
`if killed_shell.is_some() { … shells: -1 … } else { … }`, where
`killed_shell` is produced by the same `cross_shell` branch that did the
mutation. The postcondition therefore follows the code down whichever
path it took and can no longer detect the fusion branch running when it
should not have. Every other operator hands it a constant, and
`euler.rs:869` describes `ArenaDelta` as *"One operator's signed
shift"*, named at the site *"so a site reads as the op's actual shift"*.

**Verdict:**

## S70. `DESIGN.md`'s ratified graft footnote is documented-as-false in a source comment, and "whoever takes S14" is the schedule

`crates/topo/src/euler.rs:84-88` says of the graft's failure state:
*"**All three understate it.** A refusal raised between the transplant's
two passes leaves entities holding source-internal keys, which in `dst`
either dangle or resolve to an unrelated live entity. Whoever takes S14
fixes one of three copies of the same sentence."*

So a ratified design document (`docs/DESIGN.md:1131-1140`) is documented
as false in a source comment, and the other two copies
(`review_m1_pr5_internal.rs:288-296` and DESIGN.md itself) were left
carrying the weaker `SolidWithoutShells` claim on purpose. That is Q4's
second sub-case (code known to be worse than the sentence) plus Q6.

**"Whoever takes S14" now points somewhere: #823**, S14's channel, which
splits that row into **S14(a)** (the `Span`/`KnotVector` pairing in
`geom-core`/`geom`) and **S14(b)** (this door). **S70 is S14(b)'s
documentation residue** and is decided with it: the three copies of the
sentence stand or fall on whether `graft_disjoint_all_keyed` gets real
atomicity, which is row 0's "if possible" judgement at
`DESIGN.md:1358-1375` and is Evan's. #823 does not answer S14(b) — it
scopes it, and it took S14(a) out from behind it. **S14(a) has since been
ruled** and closes as two changes (#845, and the two-integer structural
refusal), so this row no longer waits on a decision about a different
crate's type: **S70 and S14(b) are now the whole of what S14 leaves
open.**

The door itself is still only documented, not prevented:
`graft_disjoint_all_keyed` mints destination solids before transplanting
and remaps as it goes, with no atomicity added anywhere in the diff.

**Verdict:**

## S73. FIXED IN PART by #783 — parts one and three; part two remains as Track C's C15 (issue #746)

Three findings on `tools/`, which is where the project's
measure-don't-guess rule (`memories/tessellation-budget.md`) is
implemented. **Parts one and three are closed by #783. Part two — the
positional face-ordinal join — is Track C's row C15 and issue #746, is
unstaffed, and stands below as it was found.**

**Part one is closed at the parse boundary rather than at the
division.** `ratio` answered `1.0` on every broken input, which for a
gate that fires only on growth is the smallest expressible slack and a
guaranteed pass — the instrument's failure mode and its pass condition
were the same value. The replacement draws the distinction the finding
asked for **per column, and in the type**: a cell count is never absent
(`tess_meter`'s `divisions` floors every count at one), so a
non-positive or non-finite one is sweep drift and leaves through
`main.rs`'s harness voice, the exit a renamed column already gets;
`worst_dev` IS absent on every `--sizing-only` sweep — the CI gate's
own — and now parses to `None` rather than to a float, so no arithmetic
can read the absence as a small number. `ratio` is gone: with the
inputs admitted, `span_held`, `recoverable` and the scene totals are
plain divisions, and a scene with no Hessian-sized face reports `None`
instead of 1.0. One policy table (`SIZING_COLUMNS`) covers the six
measured columns and is checked against `EXPECTED_HEADER` itself.
**Five rows red when the guarantee degrades**, not only when it is
violated at a chosen fixture: the per-column admission table (whose
width is a compile error if the block grows), the header-bracketing
row, the harness-voice row, the CLI exit-code row (an unreadable
denominator must reach exit 1 and never the clean 0), and the positive
control — a denominator that *genuinely* collapses still fires.

**The slack rule joins on a face ordinal that any geometry change
re-keys.** `tools/tess-lint/src/lib.rs:451-460` keys per-face slack on
`(scene, face_ordinal)`, the positional index into `mesh.patches`. Any
change adding or removing a face renumbers every face after it, so
surviving faces are compared against a *different* face's baseline row —
a mis-join, not a measurement, in either direction. The `else { continue }`
branch's comment (*"the scene's absence is already a Vanished
finding"*) is false whenever the scene is present and only the ordinal
moved; `Vanished` is scene-granular. The same path silently swallows a
NURBS face that reroutes to a non-NURBS lane, which is the *"silent
coverage loss reads as an improvement"* case `lib.rs:79-81` calls a
finding rather than a footnote. No test covers scene-present-face-missing.

*Line numbers as found, and #783 moved them further: the join is
`compare`'s `key` closure and the `else { continue }` arm below it,
which that PR left untouched and marked at the site.*

**Part three is closed by boxing the constant from both sides, on both
rules it governs.** Re-derived on `4f959cb4` by bisecting
`GROWTH_TOLERANCE` against the suite rather than transcribing the
finding: the tests admitted `[1.0384615, 1.9615385]` — an 18×
loosening of the 5% margin, and `1.9` did pass. Four rows now pin it: a
scene exactly 4% larger stays clean, one exactly 6% larger fires, and
the same pair on the slack rule. Measured box `[1.04, 1.06)`, verified
by bisection at 1.0399 / 1.04 / 1.0599 / 1.06. Pinning both rules
rather than one keeps the box intact if the constant is ever split in
two. The `k-lint` shape the finding held up is the model, and the
argument now lives at the constant, which is the claim site.

**The class check reached a bigger result than the box it was sent
for: these constants are not boxable, and the reason is a property of
the quantity rather than of any attempt at it.**
`SPLIT_SCAN_DECADES` / `SPLIT_SCAN_SAMPLES` (`tools/tess-meter/src/lib.rs`;
the second was `SPLIT_SCAN_STEPS` until a naming pass renamed it) were
**not** *"pinned by nothing"* as this finding said —
`a_ruled_wall_pays_for_its_flat_direction` constrained them
**non-monotonically**, green at 5 samples, red at 9/17/33/65, green at
161, which is an accident of whether the scan's lattice held a point
near the argmax.

**Two attempts to replace that accident with a box both failed, the
second under independent verification.** The first pinned the answer
and its argmax and reproduced the original defect with the sign
flipped — it reds on refinements that IMPROVE the number. The second
pinned the answer's distance to a denser reference, and an independent
lane scanned **every** sample count in 322..2000 and found the
counterexample two steps from the shipped value: **323 samples, a
strict refinement, 5.24% against a 5% pin.** The same verification
found the row's oracle guard vacuous (the shipped sample set is a
strict subset of the reference's, so *reference ≤ shipped* held by
construction, and replacing the reference with the subject itself left
the row green at 0.00%) and the tolerance fitted to its five-member
family — a sixth ordinary member puts the shipped pair at **5.88%**.

**The first-principles fact, measured.** The worst relative excess
moves ~4 percentage points between ADJACENT sample counts (321: 5.88%,
322: 3.64%, 323: 5.24%, 324: 1.79%, 325: 3.94%) and does not converge
(2,000 samples is still 0.79%). **The quantity is discontinuous in the
parameter it would be pinned against**, so no tolerance on it can
admit every refinement and exclude every degradation: both attempts
failed for this reason, wearing different clothes. What landed is
therefore **Q6's third option — a written reason, at the claim site,
carrying the measurement and `323` as its witness** — and the row that
survives is the divisions pin on the ruled wall, which the verification
judged a real property rather than a second accidental pin.

**The result worth more than the box, and the answer to "what
resolution guarantee do these constants provide":** a resolution in
**aspect ratio**, and no bound on the answer. The discontinuity is the
two `ceil`s, not the scan — the same worst excess computed without
them moves by hundredths of a point across the same neighbours (0.017%,
0.083%, 0.011%, 0.030%) and falls smoothly with resolution (65
samples: 1.82%; 200: 0.096%; 1,000: 0.0034%). The `ceil` quantisation
sits on top and is not these constants' to control. A guard on the
continuous quantity is possible and is not written, because it measures
something the columns do not report; a guard on the cell count is not
possible at all — the guard that IS possible, with the measurements
behind it, is **S160 / D105**. **Nobody should re-attempt the third box
on the cell count**, and the
claim site now says why.

**Verdict:**

## S75. The recourse-contract guard's central assertion is a tautology, and two suites defer their completeness obligation to it

`crates/sweep/src/fillet/mod.rs:696` — `contract()` returns
`(err.clone(), Recourse::…)` on **every** arm, so the witness it hands
back is definitionally the same variant as the seed, and

```rust
assert_eq!(discriminant(&witness), discriminant(&seed),
           "the table's arm must witness its OWN variant")
```

cannot fail. The doc at `:684` claims *"Each arm carries BOTH halves —
the decision and a witness value of its own variant — so the two cannot
drift apart by omission, which is exactly how a hand-kept witness list
lets a wrong decision ship green."* That mechanism is described and not
implemented; what actually constrains the test is `seeds()`, which **is**
a hand-kept list, seeding 8 of ~21 variants. **Both numbers moved under
#768 (D27/D29), which deleted `FilletError::EmptyChain`: the list seeds 7
of ~20 today. Re-derive rather than transcribe** — the count is a
property of an enum this track is still editing.

Two other rows defer completeness to it:
`crates/sweep/tests/m5_pr12_refusals.rs:509` (*"the standing check on
completeness is `fillet::recourse_tests`' exhaustive match"*) and
`crates/sweep/tests/review_d2_recourse_at_the_site.rs:6`. So the whole
recourse-correctness story rests on a guard whose central assertion is
vacuous.

And the row those defer *to* is itself weaker than its name:
`m5_pr12_refusals.rs:514`'s
`every_recourse_sentence_is_reachable_from_both_arms` asserts only
`!s.is_empty()` and `!s.ends_with('.')` over a list of constants —
nothing renders a `FilletError`, nothing exercises an arm, nothing
checks reachability. Its doc says *"this row is what would catch a
future edit that inlines one of them"*; inlining a constant's text at a
`Display` site would leave both the constant and this row green. **This
fix pass touched the row**, adding four new constants, without noticing
it cannot go red for the reason its name gives.

**Verdict:**

---

# Second scan · Tier 2 — significant

The four rows below were flagged **high** by their reporting agents and
are placed here only because Tier 1 was already full at the point they
landed; read them as the tail of Tier 1.

## S79. The three demo-surfaced API gaps — FILED as #757, #758, #759

The `demos/` scan is the best available evidence of what the public API
is like to use, per `memories/demo-purpose.md` (demos demonstrate real
natural usage; awkwardness is a library finding). Its three highest rows
are already issues:

- **#757** — `BooleanDeclarations` has no geometric producer. To call
  `union_with`/`intersect_with` a caller iterates faces, matches
  `Surface::Plane`, crosses and dots normals, computes plane offsets and
  runs three `k_stats::decide_flagged`/`decide` calls against a raw
  `Band::linear()` — ~55 lines, duplicated character-for-character at
  `demos/tour/src/booleans.rs:67-122` and
  `crates/topo/tests/common/mod.rs:446-498`, with the twinning declared
  in prose at both ends. (`editor_core::eval::wire::resolve_declarations`
  produces declarations from *authored names*, not from two bodies'
  geometry.)
- **#758** — no public census/genus query, so the Euler–Poincaré
  identity is hand-written ~13 times in four different return-tuple
  shapes, including byte-identically in both demo crates
  (`demos/tour/src/main.rs:186-194`, `demos/wild/src/main.rs:175-183`).
  `genus` exists only in `crates/topo/src/review_m1_pr3.rs` and
  `review_m1_pr4.rs`, both `#[cfg(test)]`.
- **#759** — the `pncad` façade's polygon door was demoted
  (`crates/pncad/src/authoring.rs:115-127`, *"a reasonable future door,
  fenced out of this unit"*, no issue number) with no replacement
  scheduled; 11 demo call sites route around it through a demo-hosted
  fold, whose own doc still says it *"Mirrors
  `pncad::authoring::polygon`'s `(f64, f64)` slice signature"* — a
  function that no longer exists. Polygon doors *do* exist at
  `crates/profile/src/lib.rs:266`, `crates/editor-core/src/program.rs:1213`
  and `crates/pncad-py/src/py/doc.rs:594`; the façade one is what is
  missing.

**Verdict:**

---

## S82. N7 now governs a refusal in the accepting direction, unscheduled and unrowed

Carrying the rim rule to the sphere carried `RimLevel::Unit(sin v, 0)`
with it (`props/curved.rs`, `sphere_boundary`'s rim arm — the
`level: RimLevel::Unit(sin_v, T::zero())` push — and `RimLevel::Unit`'s
own doc; **cited by target name per S176(a)**, because #877 moved 200+
lines of this file and the two line ranges these were written against
now land in unrelated code), so the predicate's
margin is `R·|Δ sin v|` — the axial separation, which collapses as
`cos v̄ → 0`. Two genuinely distinct near-polar rims therefore decide
`Zero` and the predicate **passes** a non-rectangular domain.

`docs/predicate-dimension-audit.md:171,550` and
`crates/geom-brep/tests/rim_dim_scale_twins.rs:369` both say this in
prose (*"here it is a REFUSAL that is affected"*, *"the lever
UNDERSTATES toward the poles, in the ACCEPTING direction"*) and both
file it as "typed-margin conversation input"; the audit table still
marks the row `OK`. No issue number, no named plan unit, and no row
exercising a near-polar interior rim. It is also the answer to "does the
new predicate have the same premise gap anywhere else" — yes, on the
same kind #723 is open on, by a second mechanism.

**Verdict: FILED AS ISSUE #893 (2026-08-21) — a kernel-logic defect, so
it leaves this document.** Offered to Evan as a third decision in #884
and never answered; Track I, whose ground this is, closed in #890, so it
was unowned rather than deferred. **The routing was not a judgement
call**: `S82` is a defect in what the predicate *decides*, not in how the
code reads, and this scan sends those to a register that executes —
`#723` and `#862` are the precedents, both struck out of a style row
rather than absorbed into one. **`S82` is `#723`'s sibling by a second
mechanism**, which its own text above already says.

**The one thing worth carrying that the issue's title does not**: the
audit table at `docs/predicate-dimension-audit.md` still marks this row
**`OK`** while its own prose at `:171`/`:550` describes the defect. A
document that records a fault in prose and passes it in its verdict
column is a second finding wearing the first one's clothes, and #893
asks for that row to be corrected as part of the fix.

## S83. `seam_tol` / `MarchTolMismatch` cannot be reached and has no row; `MarchTol` is public with no possible caller

Both finishers call `seam_tol(ctx.tol, band)` where `ctx.tol` was built
as `MarchTol::from_band(band)` from the same `band` a few dozen lines
above (`crates/geom-brep/src/ssi.rs:818-826,837,1028`), and
`MarchTol::decoupled` is `pub(super)`, reachable only from the
certificate-free door. So the refusal is unreachable by construction;
nothing in the workspace names `MarchTolMismatch`; and the MARCH-TOL
acceptance row (`tests/m5_pr7_ssi.rs:309`) asserts an identity
`seam_tol` has already forced.

Note the contrast **inside the same batch**: `du_of_rims`' doc
(`props/curved.rs`, the *"`props_du_consistent`'s reachability is
`unsure`"* paragraph — cited by target name per **S176(a)**; the line
range moved under #877, which left the paragraph itself verbatim)
worries at length that an unreachable
`require_zero` is *"a value computation wearing a typed-refusal
costume"*. The SSI seam is that shape and got no such treatment.

Separately, `MarchTol` (`ssi/march.rs:133,145,176`) is `pub` and
re-exported, its doc saying *"Outside this crate the only constructor is
`MarchTol::from_band`"* — but no public item takes or returns one.
`MarchContext` became `pub(crate)` in the same diff; `march` and
`newton_refine` are `pub(crate)`; `SsiBranch::march_tol` is a bare
`f64`. A `pub` type for an external caller who cannot exist.

**Verdict:**

## S85. The `Bounds` trait's headline still calls it the certification door, and its ledger grew 50% under the fix meant to retarget it

`crates/geom-core/src/real.rs:350`'s first line still reads *"Bound
extraction for **certification and driver code**"*, unchanged since the
base. The same PR corrected precisely this wording on `Enclosure`
(`:625`: *"Not 'certification helpers', which is what this said before
#643 — and the word matters more since D1"*) and on
`CertifiedEnclosure`'s implementor list. The trait D1 explicitly demoted
out of the certification role kept the sentence — the sweep fixed the
siblings and left the anchor. Also worth reading: `interval.rs`'s
`Bounds` impl doc and `Bounds::lo`/`hi`'s own method docs.

Meanwhile the doc block went from 156 lines to **234**. Three entries
were edited in place *and* had a paragraph appended explaining what the
edited sentence used to say (`:385`, `:414`, `:441`), so the file now
carries both the corrected text and prose about the text it replaced. A
234-line doc block on a two-method trait is past the point where a
reader finds the rule. (C5, still running.)

**Verdict:**

## S87. A fifth lane trait exists, blanket-implemented, and D1 never looked at it

`crates/profile/src/path/arc_fillet.rs:593-595`:

```rust
pub trait ArcCarrierScalar: Decide + Bounds {}
impl<T: Decide + Bounds> ArcCarrierScalar for T {}
```

Structurally the fifth member of the lane-trait family S3 counted at
four — same supertrait bundle, same `Decide + Bounds` content — with the
polarity **inverted**: it admits by blanket impl instead of refusing per
scalar. Before D1 the missing `Bounds for Dual` kept `Dual` out; now
`Dual64: ArcCarrierScalar` holds automatically, and with it the whole
`path::family` arc surface (`open_arc`, every `ArrivalSpec` /
`TangentIncoming` / `PointIncoming` impl), re-exported from
`profile/src/lib.rs:133` and `pncad::profile:57`. Neither
`arc_fillet.rs` nor `family.rs` contains the string `Dual`, so nothing
at the code site records the change — and any re-derivation of S3 that
enumerates "the four lane traits" will miss it.

**Verdict:**

## S88. FIXED IN PART by #875 — the `geom` half; the `profile` half is OPEN and UNSCHEDULED

**Scope, first, because the `FIXED` lead would otherwise read as the
whole finding.** #875 closes the `geom` half only. The `profile` half —
`crates/profile/src/fillet_select.rs` — was routed to **Track G's `G4`**,
and that route no longer resolves: Track G is closed, and the `G4` this
partition carries (Track V) is scoped to `ArcCarrierScalar` alone and does
not cover this. **So the `profile` half has no row.** Its territory is
Track V's; the receipt #875 hands it is in the handoff section.
`crates/geom-brep/` is likewise enumerated and not taken. **The
enumeration is keyed on symbols, not on line numbers**, in the census
tables — this document's citations are the thing G-R13 keeps finding
falsified by a merge that touched nothing the citation was about, and
#875 moved every one of those lines itself. Line numbers survive only in
the handoff section, where the files are untouched, as of #875's merge
base.

**The row was not "fix a line".** The finding's point is that the
admits-table's six seams were a *sample*, so the deliverable was the
*census* — every sole-`T: Bounds` door the D1 ruling opened.

### The pattern, and what it could not match

Grep the **shape**, not the symbol: the word `Dual` is exactly what a
door of this class never says. The census greps the identifiers
`\bBounds\b` and `\bEnclosure\b` over `crates/geom-core/`, `crates/geom/`
(and, for the handoffs, `crates/bvh/`, `crates/geom-brep/`,
`crates/profile/`), strips leading-`//` comment lines, and reads every
surviving hit by hand — 21 in `geom/src`, 10 in `geom-core/src`.

**The alias gap is the one that would sink this, and it is CLOSED for
these two crates by construction, not by the grep.** `bounds-allowlist.sh`'s
KNOWN GAP 3/4 is that a compound bound given a name (`ArcCarrierScalar`,
`Bracket`) is invisible at its ~49 use sites. So the census also
enumerates every `trait` **declared** in the two crates and reads its
supertrait list: `ControlPoint<T: Real>`, `Real`, `Bounds`, `Enclosure`,
`CertifiedEnclosure`, `CertifiedBounds`, `Decide: SpanLocate`,
`SpanLocate: Sealed + Real`, `KinkJacobian: Real`, `sealed::Sealed`. Only
the trait definitions themselves name a bracket door, so **no alias route
into these crates exists** and the identifier grep is complete for them.
**Independently re-derived by #875's style review**, including inside all
three `macro_rules!` bodies, which is where a hand walk is likeliest to
miss one.

**What it still cannot match**, stated rather than left to be
discovered: (1) an alias declared in a **third** crate and used here —
none exists today, and closing it needs the whole-tree version of the
trait walk above, not a bigger regex; (2) a bound reached by a
supertrait obligation, `bounds-allowlist.sh`'s KNOWN GAP 2, which the
trait walk covers only because these crates' traits are few enough to
read; (3) `crates/*/tests/`, deliberately — a test is not a door. What
would close (1) and (2) for the tree is a walk of every generic
parameter's *resolved* bound set — a `rustc` driver or a
`rust-analyzer` query, not a grep — and it is the only thing that does.
**The census is accurate as of #875's merge base.**

### The census — `crates/geom-core/`

**No generic door — but not "none", and the difference is the point.**
No function and no inherent impl in `geom-core/src` takes a sole
`T: Bounds`; its one generic bracket consumer, `spline::hull` (ten `pub`
doors), is `CertifiedEnclosure`-bounded, which D1 explicitly refused
`Dual`.

**What D1 did open in the crate it changed is a blanket impl**:
`impl<T: Bounds> Enclosure for T` (`real.rs`), sole-bounded bracket
extraction, so **a `Dual` is an `Enclosure` now**. That impl's own doc
says so, and says the rest too — *"Nothing in `crates/*/src` is
`Enclosure`-bounded today … but it is not gated either:
`bounds-allowlist.sh` greps for `Bounds`, not for `Enclosure`. **A new
`T: Enclosure` bound on anything that certifies would be a hole, and no
CI row would say so.**"* That is **issue #701**, it is the same class as
**S210**, and the two were unlinked until now. The first draft of this
record said *"D1 opened no door in the crate it changed"*, which is
contradicted 150 lines above the very insertion point it was written at.

### The census — `crates/geom/`, five modules, 13 public functions

One line per door: what it is, whether a `Dual64` through it is
meaningful, and the disposition.

| Door | A `Dual64` through it | Disposition |
|---|---|---|
| `curves::boxes::circle_arc_aabb` | meaningful — the value channel's box, which by D9 *is* the plain-`f64` run's box | **fine**, by delegation; nothing written here |
| `curves::boxes::ellipse_arc_aabb` | same | **fine**, same reason |
| `curves::boxes::nurbs_curve_aabb` | same, and it is bracket reads only — no `Brk` arithmetic on this path | **fine**, same reason |
| `surfaces::boxes::nurbs_surface_aabb` | same | **fine**, same reason |
| `curves::boxes::Brk::of` (`pub(crate)`) | the mechanism of the two arc constructors | **fine**; not a door |
| `NurbsCurve{2,3}::project` | meaningful and **partly wrong** | → **issue #874** + docs |
| `NurbsCurve{2,3}::project_seed` | meaningful — returns `f64`, so the type already says it carries no tangent | **fine**, and it is *why* the two above are wrong |
| `NurbsCurve{2,3}::project_from_seed` | as `project` | → **issue #874** + docs |
| `NurbsSurface::project` | as the curve half, in two parameters | → **issue #874** + docs |
| `NurbsSurface::project_seed` | as the curve half | **fine** |
| `NurbsSurface::project_from_seed` | as `project` | → **issue #874** + docs |
| `projection::mid` (`pub(crate)`) | where the derivative channel leaves | **documented** — this is the freeze site |

**"Fine, because —" is the disposition for seven of the twelve rows, and
the reason is one reason.** The box constructors' bracket read is a
**payload**, in `real.rs`'s own vocabulary: it goes into an `f64` box and
stops. Every endpoint a dual produces is its value channel's, which is
the plain-`T` run's bit-identically, so a dual run's box *is* the base
scalar's box — `topo::separation`'s delegation argument verbatim. Nothing
is owed and **nothing was written at those four modules**: the general
statement has one home, at `impl Bounds for Dual`, which already named
boxes as what the impl opens.

### What the census found that the finding did not: the projection lane is a wrong answer

The projection doors read a bracket and **select** with it — `mid()`
picks the foot parameter and an `f64` struct field freezes it — so
delegation makes the value right and says **nothing about the tangent**.
Measured on a sliding degree-1 line and its bilinear surface twin,
against central differences:

```
true dfoot.x/ds = 0      Dual64 says 1
true dortho/ds  = 0      Dual64 says 4
true ddist/ds   = 0      Dual64 says 0
```

Per this track's routing that is a **GitHub issue, not a smell row**:
**#874**, carrying the reproduction, the per-field table and three
dispositions. `geom/tests/dual_foot_tangent.rs` pins both halves, so the
claim is checked rather than asserted and goes red the day #874 moves it.

**The record's first draft got the SAVING half wrong, and it is worth
keeping the correction rather than the claim.** It said `distance`'s
tangent is safe on "both exits" by the envelope theorem. There are
**three** acceptance conditions, not two — `surfaces/projection.rs`'s
module docs name them as the Book's three — and only one of them saves
anything:

- **cosine**: `|g| ≤ ε₂·|C′|·|C − P|`, so the dropped term is at most
  `ε₂·|C′|·|dt*/dp|` — small, **not zero**;
- **coincidence** (`|C − P| ≤ ε₁`): returns `Ok` with **no** orthogonality
  condition held, so nothing bounds the coefficient;
- **stagnation**: fires at any foot whose parameter step dies. Domain-end
  feet *land* there — which is what the code comment says — but they are
  **not the only ones**, and at an interior stagnation foot `dt*/dp ≠ 0`.

**And the missing term is not one term.** For `foot` the coefficient is
`C′(t*)`; for `orthogonality = |C′·(C − P)|` it is
`C″·(C − P) + |C′|²`; for `distance` it is `C′·(C − P)/|C − P|`. Quoting
`C′·dt*/dp` for all three, as the first draft did, sizes the error
wrongly for two of them.

### The distinction, which is the durable part — and which already existed

**#875's first draft coined `terminal` / `fed-back` for it. That was
wrong twice**, and it is worth recording because it is the defect this
scan's Q1 calls the highest-yield one, committed inside a fix for a
neighbouring instance. (i) The distinction is **already written** in the
same doc block, ~85 lines above the insertion point: *"ten reads are
typed-error **payloads**, and four are **selections**. Two of those four
**feed a classification or a mutation rather than sitting after one**"*,
with `sugar.rs`'s "choice among already-classified constructions" as the
precedent that does not reach those two. (ii) `terminal` **already means
something else in this crate** — an unrefinable decision outcome, in bold,
at `interval.rs` and `predicate.rs`.

**And the coined rule named the wrong invariant, which matters more than
the vocabulary.** *"The `f64` re-enters the computation as a frozen
constant"* over-fires on four **ratified** conventions in `dual.rs`
itself — `impl SpanLocate for Dual`, `floor`'s plateau factor, `min`/`max`'s
branch pick, `copysign`'s σ — all correct under branch consistency. The
axis that actually separates them from `project` is **locally-constant
selection versus smoothly-varying implicit function**: a span index is
piecewise constant, so freezing it loses nothing; `t*(p)` is a smooth
implicit function of the input, so freezing it drops `dt*/dp`. That is
the framing #875 ships, at `projection::mid` and in one clause at
`impl Bounds for Dual`.

### What enforces it afterwards: nothing, and that is by design

`scripts/gates/bounds-allowlist.sh` **cannot** see this class — a sole
bracket bound is its planted **must-not-fire** self-test case
(`plant_sole_bracket_bounds`), for the sound reason that firing would red
every certification file in `geom` and `geom-brep`. The `Enclosure`
blanket impl above says the same thing about its own trait and points at
**#701**. So the census is a snapshot nothing re-derives, and the next
such door is invisible the day it is written. **A disclosed blind spot is
a work order**: that is **S210**, unstaffed, with the cost of closing it.

### What #875 cost the two doc blocks, said out loud

The first draft grew `impl Bounds for Dual`'s doc **75 → 107 lines
(+43%)** guarding a six-line impl body, and grew `real.rs`'s `Bounds`
block **236 → 246** — the block **S85** measures at 234 and calls past
the point where a reader finds the rule. **The mitigation for S85 failed
in both directions at once**, which is Track F's standing lesson (*the
fix minting a fresh instance of the defect it closed*) landing again.
What shipped instead: `real.rs` is **byte-identical to `main`** — its
paragraph was a second home for what the `Enclosure` impl already says —
and `dual.rs` is **79 lines, +4 on main**, an in-sentence correction to
the three-item list rather than a section. The mathematics moved to
`geom/src/projection.rs`, which is in neither contested block and is
where the freeze physically happens.

### The structural half, added by H-g (PR 2 of `S90`'s implementation)

**What this PR cost the `real.rs` `Bounds` block, said out loud, in the
place the section above says it out loud.** #875 shipped `real.rs`
byte-identical to `main` after finding its first draft grew the block
`236 → 246`. **This PR grows it `236 → 249`, +13 lines (+5.5%)** —
measured the same way, as the contiguous `///` block above `pub trait
Bounds`. That is a real cost against **`S85`**, which `H-c` owns in wave
2, and it is disclosed rather than absorbed. Reduced from **+29** during
review by cutting every restatement the block did not need: the "which
door tightens" rule points at `geom::projection`'s `mid` instead of
repeating it, and the allowlist's chart_region entry went back to being
a pointer. **What is left is not compressible without dropping content
that belongs here** — an open question of this rule's own (#643's
`separation` half) answered in the rule, and two corrections to entries
that had become false. A doc block that may not grow to answer a
question posed inside it is not a doc block, and `S85`'s subject is the
block's *undirected* growth, not its content.

#875 wrote the census and the corrected mathematics; it left the doors
open. **`{NurbsCurve2, NurbsCurve3, NurbsSurface}::{project,
project_from_seed}` now bound `T: CertifiedBounds`**, so the wrong
tangent is UNREACHABLE rather than fixed — **#874 stays open** for the
fix that would make it right (carry `dt*/dp` through the
implicit-function theorem), and the tightening is one of the three
dispositions that issue lists, taken structurally.

**`project_seed` and the box constructors are deliberately NOT
tightened** — the census table above already gives each its verdict and
its reason, and `geom/src/projection.rs`'s `mid` carries the rule they
share. **The bound follows the defect, not the class of the bound**, so
a door whose freeze costs no derivative keeps the sole bracket bound
even though its signature looks identical to one that tightened.

`geom/tests/dual_foot_tangent.rs` survives the eviction with its subject
changed: it now measures the TRUE derivatives by central difference at
`f64` — the numbers a #874 fix must reproduce — and records what
`Dual64` reported as prose, because that half can no longer be executed
in-tree. The eviction itself is pinned by `compile_fail,E0277` rows in
`geom/src/projection.rs`'s module docs, which is where a doctest is
actually collected.

### Handed off, not taken

- **Open, no row** (`crates/profile/`, Track V's territory):
  `fillet_select.rs::nearest_joint`
  (`:169`) is the sole-bound door. **The finding's `:98` is a doc line,
  not a door** — re-derive against the tree. `path/arc_fillet.rs:361`'s
  `map_refusal<T: Bounds>` is a second sole-bound door the finding does
  not name; the rest of that file is the ratified `Decide + Bounds` seam.
- **`crates/geom-brep/`** (Track C's ground, no live orchestrator):
  `ssi.rs:218`'s `impl<T: Bounds> TubeScale<T>` — **the finding's `:1187`
  is `certify_rung3`'s doc, and `certify_rung3` is shut at a dual by its
  `CertifiedEnclosure` term**, not by being compound; compound bounds as
  such are exactly what *is* reachable, which is why
  `rational_arc_chain` below is. Also `ssi/certify.rs:{271,277,289}`'s
  `exact`, `exact3`, `composite_form`, three private sole-bound helpers
  the finding does not name.
- **`crates/geom-brep/`, a different class, listed here so it is not
  lost**: `pcurve_cache.rs:1055`'s `rational_arc_chain` is
  `Decide + Bounds` with **no** `CertifiedEnclosure` — instantiable at a
  dual for want of that term, the same shape as `topo::separation` and
  `chart_region_overlap`. Not a sole-bound site, and not S210's subject.
- **`crates/bvh/`** (no track): `aabb.rs:87`'s
  `Aabb::from_points<T: Bounds>` — a payload read, sound by delegation,
  and the one door every `geom` box constructor funnels into.

## S89. The one-home fix for the ring crossing minted three local aliases and a hand-counted tally

`RingInterval::from_certified` is the declared one home, and three
private one-line wrappers now sit on top of it — `bracket`
(`ring_interval.rs:160`), `ring` (`geom-brep/src/ssi/enclose.rs:195`),
`br` (`topo/src/props.rs:494`) — each carrying its own multi-paragraph
restatement of the same rule, two of them sharing a verbatim sentence
and the third restating it differently. The door's doc also carries a
**prose census of its callers** (*"five call it directly, and the rest
go through…"*) which nothing enforces and which already needed a
correcting commit (`88616177`, "S41: correct `from_certified`'s own
count of its call sites").

A unit that unifies duplicates minting three named copies plus a
hand-maintained count is the fix reproducing what it closed.

Related: the suite named after the split,
`crates/geom-core/tests/decoration_seam.rs:21`, claims its rows pin
*"that the three C9-ring crossings follow the second door"*; every
executable row reaches the ring through one crossing,
`hull::domain_hull` via `hull_bound` (`:139`). **Two independent agents
hit this** (the geom-core auditor and the new-tests auditor). The
new-tests agent adds that `trv()`/`healthy()` and the whole
`the_fixture_is_a_finite_bracket_that_cannot_certify` row are restated
verbatim in three test files, two of which share a filename in sibling
directories and run in the same `geom` binary.

**Verdict:**

## S90. The largest D1 residue is the only one without a schedule

`crates/geom-core/src/real.rs:470-477`: *"What is owed is a lane, or a
written reason it needs none, and it is owed on the **public**
surface"* — recorded as prose, pointed at from
`scripts/gates/bounds-allowlist.sh:27-31`, with no issue number and no
plan unit. The smaller residues from the same ruling all got numbers
(`ContentBits for Dual` → #687, the census box duplication → #700, the
`Enclosure` gate gap → #701); the one seam the ruling actually left
unguarded got neither. `real.rs:394`'s *"#643-completeness question …
deliberately left open here"* is in the same position.

§D's fourth ordering rule says a finding *"leaves a verdict and no row only if
the verdict is closed"*. This one is decided-and-open.

**Verdict: ANSWERED — Evan, 2026-08-21: *"tightening to `CertifiedBounds` works at least for now."* Answer (4).** The fillet seam's three public entry points take `<T: Decide + CertifiedBounds>`, which makes an external `Dual64` instantiation a **compile error** rather than a thing an audit has to keep being true about. *"At least for now"* is part of the ruling and is recorded as such: this closes the seam, it does not settle whether a fillet battery should ever be differentiable.

**What the ruling does NOT do — and the distinction is Evan's, drawn on the evidence:** it does **not** delete the four lane traits. `CertifiedBounds` refuses at the **function**; a lane trait refuses at a **sub-operation inside a function that has non-certifying work to do**, and no bound on a whole function can say *"this arm needs certification, the rest does not"*. All four lane traits gate mixed passes, and `topo/tests/geometric_cube.rs:236` calls `validate_geometric` at `Dual64` and asserts it **succeeds** — the quadrature arm declining internally while the rest genuinely validates, after which every certificate's value channel is compared bitwise to the `f64` build. Bounding that pass on `CertifiedBounds` would not harden it; it would delete `Body<Dual64>`'s ability to go through a validation pass at all. **The doors tighten; the passes keep their lanes.** Full ruling and its scope: `docs/SMELL-H-LOG.md`, **H-R3**.

**Implemented in two PRs, split so a decision flagged *"at least for now"* is independently revertible.** **#886** took `topo::chart_region_overlap` and `geom`'s two projection doors: no capability lost, and a wrong answer (#874) made unreachable. **#883** carries the `sweep/fillet` third, which prices one — `Filleted<T>` carries a `Body<T>`, so a fillet stops being differentiable — and it is **open with Evan**, because implementing it turned up `fillet_edges` reachable from `editor_core::eval::evaluate`, a mixed pass. **This sentence is the one home for the split; #883 amends it in place rather than adding its own.**

*(Asked by Track H on claiming, 2026-08-21; the record of the question is below and in PR #867.)*

**Original question, kept because the answer is only legible against it — ASKED 2026-08-21.** Track H
owns `geom-core/` and this is the one row in its ground that is a
decision rather than work, so it goes out at constitution rather than
when a neighbour stalls on it: **`H-c` edits the same `real.rs` doc
block** (S85 is that block's 234-line growth) and **`H-f` inherits the
seam** (C7 is the lane-trait collapse this refusal would live in).

**The state of the seam, checked against the tree rather than
transcribed.** `real.rs:470-477` says *"What is owed is a lane, or a
written reason it needs none, and it is owed on the **public**
surface"*, and `scripts/gates/bounds-allowlist.sh:24-31` points at that
paragraph as the ONE home rather than restating it — calling it a
**STANDING OBLIGATION** and naming the lapse plainly: *"the fillet seam
is the one allowlisted seam with NO refusing lane behind it, and the
guard that made that acceptable — `Bounds` having no `Dual` impl — has
lapsed."* The enumeration behind it is real and was done twice
independently (PR #682's body): every predicate's `Ok`/`Err` comes from
a `decide(...)`, ten reads are typed-error payloads, four are
selections, and **nothing mints a certificate object**.

**So the question is not whether the seam is understood — it is what
that understanding discharges.** Three answers, and the row wants one:

1. **The lane is owed now.** The sibling residues from the same D1
   ruling each got a number — `ContentBits for Dual` → #687, the census
   box duplication → #700, the `Enclosure` gate gap → #701. This one is
   the *largest* of them and got neither an issue nor a plan unit, which
   looks like an oversight rather than a decision. If so it wants an
   issue and a Track H row, and `H-f` is where it would land.
2. **The written reason is already sufficient**, in which case the
   paragraph IS the discharge and the finding closes — but then the
   sentence *"what is owed is a lane, or a written reason it needs
   none"* should stop describing the obligation as outstanding, because
   two documents currently read it as live and a third gate points at
   it.
3. **The verdict is closed and the prose is the residue** — i.e. the
   answer is (2) plus an edit, and the only work is making the three
   sites agree.

**`real.rs:394`'s *"#643-completeness question … deliberately left open
here"* is in the same position** and takes the same answer; whichever
way this goes, it should not need asking twice.

**CORRECTION, same day, on Evan's question — there is a fourth answer
and it is much cheaper than (1).** The question above was written as if
"the lane is owed" meant building a `PropsQuadLane`-style trait. It
probably does not. **#643 already shipped the type-level mechanism**:
`CertifiedEnclosure` is implemented for exactly `f64`, `Interval`,
`RingInterval` and `Probe` — **never for `Dual`** — and `real.rs:800`
gives the pair a sole-bound spelling, `pub trait CertifiedBounds: Bounds
+ CertifiedEnclosure {}` with a blanket impl. So a seam that wants duals
out does not need a lane to refuse them at runtime; it needs a **bound
that does not type-check**.

**The fillet seam is one word from that.** `fillet_edges`
(`build.rs:127`), `ring_clearance` (`surgery.rs:775`) and `run_battery`
(`battery.rs:828`) are each `<T: Decide + Bounds>`. Tightening all three
to `<T: Decide + CertifiedBounds>` makes an external `Dual64`
instantiation a **compile error**, leaves both real scalars (`f64`,
`Interval`) untouched, and does not change a line of the bodies. It stays
a compound bound, so it still fires `scripts/gates/bounds-allowlist.sh`
and still needs ratification — which is correct, and is the thing being
ratified here.

4. **Tighten the bound to `Decide + CertifiedBounds`** — three words, not
   a lane. Costs: it **evicts** duals rather than hardening the seam (see
   below), and it is only right if nothing should ever differentiate
   through the fillet battery.

**The wrinkle that decides between (2) and (4), and it is written down in
exactly one place** — S44's *"What this does NOT settle"*: at plain
`Interval` a caller hardens a `Decide + Bounds` seam by adding
`CertifiedEnclosure`, because the scalar satisfies both. At
`Dual<Interval>` that same upgrade **evicts** rather than hardens. So
*"harden this seam"* and *"keep duals out of this seam"* are the **same
edit**, and there is no spelling that does one without the other. (4) is
therefore not a free tightening: it is a decision that the fillet battery
is not a differentiable surface, taken at the API.

**Track H is not proposing an answer.** What it will say is that the
current state — decided-and-open, pointed at from a gate, with no
register that executes — is the state the first scan's closing rule
exists to forbid, and that the real choice is now (2) versus (4): *is the
audit the discharge, or should a dual stop type-checking here?* Option
(1) as originally written — build a refusing lane — is very likely the
wrong shape, because three of the four existing lane traits are already
redundant for the guarantee and only their typed refusals are load-bearing
(see `C7`/`H5`).

## S93. #713's prose-held-invariant sweep minted two new prose-held caller obligations

`crates/topo/src/euler.rs:1048-1059` (`mev`'s fan site) and
`crates/topo/src/euler_kill.rs:522-535` (`kev`'s fan merge) each now
carry a paragraph stating that re-based edges keep carriers certified
against their *old* endpoint, instructing the caller: *"**Re-describe
the moved run** (via `set_edge_curve`) whenever the two points differ."*
Nothing enforces it — tier 1 does not constrain it, no operator
re-checks it, and the paragraph says so.

That is the exact "the caller must / kept in step by hand" shape S15 was
raised to retire, minted by S15's own fix pass. `seqgen.rs:562-575`
already works around the resulting state (*"an edge's stored curve is
routinely stale against its own endpoints"*) with a re-certification
filter that admits it costs coverage.

**Verdict:**

## S94. Two hand-maintained `VARIANTS` ladders, with the disclosure copied verbatim

`crates/topo/src/euler.rs:3220-3251` and
`crates/topo/src/validate.rs:4079-4140` both carry a
`const VARIANTS: usize` and a wildcard-free `variant_index` match
restating the enum's declaration order by hand, and both carry the same
four-sentence *"what it does NOT enforce … when you add an arm, its
index is the new `VARIANTS - 1`"* paragraph word for word — a hand-kept
mirror of a compiler-known fact, self-declared as such at both copy
sites. The disclosure names `strum`'s `EnumCount` as the way out and
then declines it, which owes a schedule and has none. Worth checking
whether `MassPropsError`, `PcurveMintError` and `BooleanError` grew the
same ladder or lack it.

**Verdict:**

## S95. Two operand gates with different admitted kind sets, and a doc that now describes only one

`boolean_op_with`'s doc was rewritten to *"Surface kinds are gated per
arm, not wholesale — see `reduce::gate_operand_kinds`"*
(`crates/topo/src/boolean/ops.rs:251-252`), but fifteen lines down the
function still runs its own wholesale scan for Subtract and Intersect
(`:388-402`) that admits only `Plane | Cylinder | Sphere` and refuses
`Nurbs` with a **different** error (`CurvedOpUnsupported` vs
`CurvedBooleanUnsupported`). So for two of the three ops the doc's claim
is false, and the crate carries two spellings of "which surface kinds
may be an operand" that have already drifted by one variant.
`gate_planar` was renamed to `gate_operand_kinds` *"for what it admits"*
in the same churn window, which makes the surviving inline copy easier
to miss.

**Verdict:**

## S96. The shared `chord_join` core still imports from one of its two consumers

`crates/topo/src/chord_join.rs`'s placement argument is that it is *"a
**top-level sibling** of `boolean/` and `splitting/`, like
`crate::sector_shape` and `crate::sector_face`, so neither half hosts
the other's core."* It then imports `crate::splitting::SplitPlane`,
`crate::splitting::containment::{…}` and
`crate::splitting::rules::face_extent` (`:62-67`, `:90-92`), and its
error docs point back at `crate::splitting::split` and
`crate::splitting::plane_section`. The two modules it cites as precedent
import nothing from either lane, so the analogy is doing work the
dependency graph does not support. S5's shape, one indirection later.

**Verdict:**

## S98. `K-REPORT.md`'s dated M3 crop was back-filled against its own twice-stated rule, and its arithmetic no longer closes

`bool_join_chord` (minted by #719) and `point_in_loop_segment` (minted
by #712) were inserted into the *"M3 addendum (snapshot, 2026-07-23)"*
crop (`docs/K-REPORT.md:307,312-322`), with its bullet counts bumped
24→25 and 4→5. The same document rules twice that this must not happen —
*"back-filling it would make it describe something it never
described"*, and *"The M3 addendum's inventory … is likewise left as
written: it is a dated 2026-07-23 record."* The header still reads
*"added 59 predicate names"* while the bullets now sum to 61, and the
per-family table at `:848` still reads `point_in_loop_* | 4`.

For this section, the K data no longer means what it did after the name
churn, and the internal inconsistency is the evidence.

**Verdict:**

## S99. `net::is_placeholder` tests one channel while the crate doc promises all of them

The hoisted predicate (`crates/geom/src/net.rs:142`) is
`control.iter().all(|p| p.channel(0).is_poison())` — every point, but
only channel 0. `crates/geom/src/lib.rs:71-78`, directly above it, says
the discriminator is that *"a placeholder's every control point is
all-poison"*, and that a described net carrying poison *"must fail
loudly as such … never masquerade as the benign placeholder"*. A
described net whose every control point has a poisoned `x` and finite
`y`/`z` is precisely that masquerade, and it now reads as the benign
placeholder at ~25 consumer sites (`step-export/src/writer.rs:44`,
`topo/src/props.rs:660`, `mesh/src/trimmed.rs:186`,
`geom-brep/src/certify.rs:999`, …).

The single-channel form was inherited from the surface half; the
crate-merge dedup moved it into a helper that has `CHANNELS` and
`channel(d)` in scope and did not widen it. This is the one place the
merge picked a half's behaviour and landed it under a doc describing the
other half's.

**Verdict:**

## S100. `scalar_lift` is named for the job it explicitly declines to do

S33's ~14 hand-written per-variant ladders are the thing called "scalar
lift" everywhere else in this programme;
`crates/geom/src/scalar_lift.rs:1` deduplicates only the four leaf
point/vec converters and says so in its header (*"the per-variant
*ladders* … stay where they are"*). The result is a module whose name is
the concept and whose contents are not, in the same crate as four
surviving ladders (`curves.rs:818`, `:908`, `surfaces.rs:653`, `:1057`),
each still spelling `Nurbs(_) => nurbs_placeholder()` — the exact silent
substitution S33 named.

The merge also put two spellings of one operation side by side that were
previously in different crates: `lift_to_dual` (curves) vs `lift_dual`
(surfaces), plus two unrelated `lift` functions. A reader looking for
"where does this crate lift a `Surface<f64>` to `Interval`" will open
`scalar_lift.rs` and not find it.

**Verdict:**

## S101. The merge's prose sweep deleted a cross-reference instead of re-aiming it

The pre-merge text at `crates/geom/src/curves/nurbs.rs:684-687` read
*"(The opposite choice — dividing by the min-weight floor, as
`geom_surfaces::recognize`'s conic-derivative work does — is the
direction for an UPPER bound…)"*. The sweep removed the clause, leaving
the claim without its precedent. The pointer was **already** mis-aimed —
there is no `recognize` module in `geom-surfaces`; the actual site is
`crates/step-import/src/recognize.rs:422`, which builds exactly the
`/ w_min` upper bound the sentence contrasts against. A sweep whose
stated job was "every stale crate name" resolved a stale *name* by
deleting the *fact*, and the only record that these two bounds are
deliberate opposites is gone.

Look for the same shape at every other site that sweep touched: the
pattern it matched was the identifier, and the identifier is what the
sentence was hanging on. The same sweep also left *"the geometry
**crates**"* standing at `crates/geom/src/curves/boxes.rs:8` and
`surfaces/boxes.rs:4` while correcting the manifest comment
(`Cargo.toml:26`) to the singular — it saw the argument and fixed only
the instance carrying a literal crate name.

**Verdict:**

## S102. Two more copy-sites in `geom` that the merge's whole justification was about

- `crates/geom/src/surfaces/nurbs.rs:4-11`: *"Data model, evaluation
  contract, and fixed-association rules are the curve module's … the
  conventions are **stated once there and once here**."* A copy-site
  declaring itself, in the crate whose justification was that the two
  halves stated one thing twice. The crate-doc hoist took the
  *enum-level* conventions and left the *payload-level* ones duplicated.
- `crates/geom/src/surfaces.rs:26-30`: a bullet titled *"The shared
  helper"* spelling the `radial`/`tangential` formula, kept verbatim,
  never naming the thing it calls the shared helper. The helper moved to
  `crate::azimuth` and is now shared with the *curve* half too — the
  merge's whole point. `azimuth.rs:1-20` claims to be the single home;
  `curves.rs` carries no matching paragraph, so the two halves' headers
  now disagree about who documents the frame.

**Verdict:**

## S103. The iso-curve placement rule now lives only in the code that already obeys it

The merge deleted the acyclicity sentence that had been enforcing
"iso-curve extraction belongs to the EdgeGeometry layer" and restated
the rule as 23 lines of prose at
`crates/geom-brep/src/nurbs_iso.rs:19-41` — honestly, including the
admission that *"nothing stops this module moving down except the rule
itself"*. But the restatement is in `geom-brep`, and the code that would
violate it is in `geom`. Nothing in `geom/src/lib.rs`,
`geom/src/surfaces.rs` or `geom/src/surfaces/nurbs.rs` mentions it, so
the person who adds the next extractor next to the payload — the exact
move the rule forbids, and the one the merge made structurally possible
— will not encounter it.

**Verdict:**

## S190. `attribute`'s decline lookup consults ONE of the pair's two faces, and arena order picks which

**Found by lane G-e** closing S104; **filed as issue #855** rather than
fixed, because the fix is `topo`'s. Sharpened by the adversarial
review, which added the half that matters.

`crates/editor-core/src/assembly.rs`'s `CensusUnsupported` arm resolves
the declaration from the ONE face the error carries, but the census's
subject is a face **pair**:

- a face that **two** mates declare answers to whichever declaration
  comes first in document order — realistic, since `SelfMate` refuses
  only same-*instance* pairs, so `(f,g)` and `(f,h)` can both be minted;
- `topo::census`'s conformal-patch sweep reaches this refusal on an
  **undeclared** pair, before declaredness is consulted, and one of
  that pair's faces may still be declared against a third;
- **and `sweep_conformal_patches` always carries `EntityId::Face(fa)`,
  the first face of the pair in arena order, never `fb`** — so when
  `fb` is the declared one and `fa` is not, the lookup misses
  entirely, the finding is `Unattributed`, and the assembly falls to
  `AtRest` instead of `Uncertified`. The defect is not *"picks the
  first of two declarations"*; it is *"only ever consults one of the
  two faces"*, and which one gets a chance is decided by arena order
  rather than by declaredness.

**The `AtRest`/`Uncertified` split stays sound, structurally.** Both
reviewers checked it independently: every production `CensusUnsupported`
push is a *cannot-decide* path, and the **refuting** direction at the
same door goes to a **different variant** — `census.rs:1955`'s
`ChartOverlap::Empty` raises `StaleContactDeclaration`, which attributes
`Refuted`. The split reads only the relation, never the identity. What
can be wrong is the mate a message names, and the day a GUI highlights
"the offending mate" that becomes user-visible.

`AssemblyError::Uncertified`'s doc claimed *"every finding is the census
DECLINING a declared pair"*, which the code never established. It, the
`Display` string a caller actually reads, and `Attribution::Declined`'s
own doc are corrected to what `attribute` does establish — *declining to
certify a face that a declaration names* — and `Declined` now carries
the width-1 caveat where a caller meets it.

Narrowing the lookup needs the PAIR in
`ValidationError::CensusUnsupported`, which is `topo`'s to carry.

## S192. A wildcard over a closed enum, one layer upstream of `attribute()`: `topo::census` decides `Escalated` vs `Unsupported` behind one

**Raised by #833's style review** over `crates/topo/src/census.rs`, which
is outside that lane's scope cell. **Not fixed there** — §D row **D120**.

`census.rs:1018` and `census.rs:1962` are both `Some(Err(_)) =>` over
`ChartRegionError`, whose own doc says *"closed enum, D3 style: every arm
names its recourse"*, and both decide between
`ValidationError::CensusEscalated` and
`ValidationError::CensusUnsupported`. That is **precisely the
discrimination `editor_core::attribute` then turns into `Unattributed`
against `Declined`, i.e. `AssemblyError::AtRest` against
`AssemblyError::Uncertified`** — so an arm added to `ChartRegionError`
becomes `CensusUnsupported`, becomes `Declined`, and is reported to the
caller as an unrefuted frontier over geometry nothing decided. **The
enum has ten variants today**, so the arm at risk is the eleventh (the
review said tenth). `:1962` is the *declared* record's confirm pass, so
the pair reaching it is one a mate minted — the promotion is not
hypothetical for that site.

By the taxonomy #833 wrote into `ValidationError`'s doc these are
**classification** sites, not rendering or extraction: the wildcard's
answer is a different, smaller vocabulary, chosen here.

**Where else to look**, and the sweep this row owes: every
`Err(_)`/`Some(Err(_))` over a `topo` refusal enum in `census.rs` and
`boolean/`.

## S193. A classification spelled as a let-else — the hit a disclosed blind spot produced

**Found by #833's adversarial reviewer**, running a differently-shaped
sweep: it extracted all 59 `ValidationError` variant NAMES and grepped
for the names rather than the type, which catches aliased imports,
`matches!`, `if let` and let-else at once. That sweep also returned
**zero aliases**, closing #833's disclosed blind spot #2 more strongly
than the lane could claim it, and confirmed `attribute` is the only
production classification site on that enum.

`crates/editor-core/src/eval/wire.rs:993`'s `refusal_menu` classifies
`topo::BooleanError` with a **let-else**: one variant gets the refusal
menu, everything else falls through to `NodeErrorKind::Boolean(err)`.

**Disposition, on inspection: benign, for the reason
`pncad/src/workspace.rs:432` is** — the fallback preserves the error
verbatim, so the `else` answer is the value itself and a variant added
later answers for itself. It is a **rendering** site by #833's own
taxonomy, not a classifying one.

**The record is worth keeping for the sweep, not for the site.** #833's
sweep pattern could not have found this whatever its disposition,
because it greps the type name and this site never spells it in a match.
A lane sweeping `BooleanError` — or any `topo` refusal enum — should
sweep by VARIANT NAME, and should expect let-else and `matches!` shapes.
A disclosed blind spot that produces a real hit is a work order.

## S105. The shared refusal ladder retired one duplication and minted a documented hand-synced one

`crates/editor-core/src/eval/wire.rs:717-723`'s new `ladder` module doc:
*"**Not shared with `crate::resolve`, yet** … The two agree by hand
across a module boundary, at coarser grain than the duplication this
module retired; folding them is a larger change … recorded as such and
not attempted here."* The duplication is real:
`crates/editor-core/src/resolve/mod.rs:552-557` re-derives the
`next_id`/`ForeignNode`/`NodeDeleted` rung and `:583`/`:606` rebuild the
same `TieWitness`. "Recorded as such" is not a schedule.

Same wave, same shape:
`crates/editor-core/src/persist/kernel_wire.rs:17-20` says the module
exists *"so the technique has one home and one doc instead of one per
type"*, and `boolean_op.rs:13-15` makes a point of the read direction
not restating the write direction — *"it calls it"*.
`contact_class::untag` (`contact_class.rs:87-113`) restates the table
anyway, for the enum that is `#[non_exhaustive]`, and has no equivalent
of `boolean_op::serialize`'s round-trip guard.

**Verdict:**

## S170. The PATHS verb vocabulary's sixth copy is the Python surface, and it is the only silent one

`crates/pncad-py/src/py/path.rs` binds the PATHS lattice state for
state, one `#[pymethods]` block per state, and
`crates/pncad-py/pncad.pyi` declares the same methods again. The module
header's *"The Python layer re-implements NOTHING"* is true of the
BODIES — every verb clones its `PartialPath` and calls the same generic
Rust method — and false of the vocabulary: which verbs exist at which
states is written out by hand, twice.

Nothing anchors either copy. Measured: with a probe verb added to
`transition_table!` and `editor-core`'s two exhaustive matches on
`profile::Step` discharged, `cargo check --workspace --all-targets` is
clean, `pncad-py` included. A verb the table gains simply does not exist
in Python, and no test says so.

S4's `Step`-verb row counted **5, across 3 crates**; with this one it
is **6**, and its anchor list named no `pncad-py` file. Both are
corrected at that row, which now cites every copy by name.

The `editor-core` half is closed by S106's census, which cannot reach
here: `Verb::ALL` is Rust and this surface is a PyO3 binding plus a
`.pyi`. The shape that would work is the one S4's `RoleSeg` row already
describes for the same crate — enumerate the Rust vocabulary and assert
one Python attribute per member — so the first question is whether the
`Step` mirror and the `RoleSeg` mirror want one census or two.

**Verdict:**

## S195. The arc-mode vocabulary is the profile `Step` vocabulary one level down, and it has no census at all

**Raised by #836 (G7/S106) out of its own claim site.** The verb
vocabulary now has a table, a `Verb::ALL` and a cross-crate census. The
**arc-mode** vocabulary — which travels inside the verbs — has none of
the three, and is spelled four times:

- six standalone spec structs in `profile/src/path/verbs.rs`
  (`Radius`, `Bulge`, `Via`, `Center`, `Sweep`, `ArcLen`), consumed by
  the state-keyed trait matrix;
- restated field-for-field as the six variants of `pub enum ArcData` in
  `profile/src/path/program.rs`;
- restated as `pub enum ProgramArcData` in `editor-core/src/program.rs`
  (the `Expr`-valued document form);
- restated as `enum WireArcData` in
  `editor-core/src/persist/wire.rs` (the persisted form).

**There is no `ArcData::ALL`**, so nothing anchors a census the way
`Verb::ALL` anchors S106's. And the hop that matters is the same one:
`res_spec` in `editor-core/src/program.rs` **matches `ProgramArcData`
and constructs `profile::ArcData`** — the constructs-so-the-compiler-
cannot-see-it shape S106 names as its whole finding.

**What a seventh mode would do.** It would be forced into
`ProgramArcData`'s and `WireArcData`'s conversions by exhaustiveness,
and into `spec_slots` — but `spec_arg_access!`'s table ends `_ => None`,
so the new role would enumerate and **address nothing, silently**.
S106's bijection test cannot catch it, because that test walks a corpus
and nothing forces the corpus to grow when a mode is added. The verb
census escapes this only because `Verb::ALL` forces growth; the mode
vocabulary has no equivalent.

**The same shape, in three smaller pairs**, all in the same two files
and all hand-mirrored: `ProgramTarget`/`WireTarget`,
`ArcSide`/`WireSide`, `ArcSweep`/`WireWinding`.

**Why it is raised rather than fixed.** #836's scope is the verb
vocabulary, and the fix here is not a comment: it is an `ALL` on the
mode enum plus a corpus anchored on it, and possibly one shared census
for all four vocabularies rather than four. That is a unit, not a
residue. The claim site is fixed in #836 — the `transition_table!`
header and `PATHS-DESIGN.md` both used to assert that the round-9
exhaustiveness pressure over `ArcData` rides the verb table for free,
which would have meant this finding was already solved.

**Verdict:**

## S107. The `DimensionError` untangling renamed the Rust type and left the Python-visible confusion in place

`DimensionError` → `QuantityOpMismatch` is a Rust-side rename only
(`crates/pncad-py/src/errors.rs:52-61,115-131,172-176`). From Python,
`DimensionError` still means the quantity-operator check, and the
document layer's real `DimensionError` still surfaces as `LiteralError`
on one door and as `PersistError` with `variant == "parse"` on the
other (`pncad.pyi:83-108`). The situation the first scan named — the
real dimension checker not being the thing called `DimensionError` — is
preserved and now defended by cross-referencing docstrings on both
classes plus a paragraph in `errors.rs`. Tag strings are correctly
unmoved and #694 schedules the `load`-door half; the naming half was
closed by argument rather than by change, and the length of the argument
is what draws attention to it.

**Verdict:**

# Second scan · Tier 3 — real but lower stakes

Tier 3 is grouped into class roll-ups rather than one ID per instance,
the way the first scan handled S35 and S40. Each bullet is a distinct
site; cite them as e.g. `S110(d)`.

## S110. FIXED IN PART — the sort ran; #790 closed four members, #786 and #787 three more, #844 closed (a), two are routed

**The sort is the deliverable, and re-deriving it moved one member.** §D
proposed three dispositions — *(a)* vacuous assertions in shipped test
files, *(b)* hand-run diff artefacts whose comparison no longer exists,
*(c)* equal-by-construction asserts in `demos/`. Re-derived against the
tree, **(c)(d)(e)(f)** landed in lane F-c and are closed by **#790**;
**(g)(h)(j)** are disposition (c) and were closed by **#787** and
**#786**; **(a)(b)(i)** are disposition (b). **(d) is not disposition
(a)** and is recorded below as what it turned out to be.

**§D's proposed destination for (a)(b)(i) — "C23's class … the
test-suite-cost sweep" — was checked by #790's review and does not
exist**, and #790 had already written it into this record. Bare `C23` is
§D's *rational refinement schedule* row, `§C23` is an unrelated process
observation, and the *"test-suite-cost sweep"* appeared only in the second
scan's starting-order note and was never a lane, a row or an owner. The three members went to **F8** (whose rows
are **D84**/**D85**, not the D44/D45 this sentence was written against) and to
**D104**, per the ruling above F8. A lane that complies with a
citation is not at fault; the citation is the claim site (C-R11), and
this one was tested only after it had been copied twice. **(a) is closed:**
it was D84's finding stated from the other end — `probe_s5_sectors.rs`'s six
coverage assertions cannot go red because nothing runs
`cargo test -p topo --features probe` — and **#844 closed it** by giving that
file a declared disposition, naming the six assertions as the sharper half of
what does not execute, and putting the whole censused probe set behind a floor
keyed on tests that ran. **(b) and (i) remain routed to D104.**

**Two of this finding's own claims did not survive re-derivation**, both
in member (c): the second test is *not* a byte-for-byte copy (it inlines
`boolean(op)` where the first binds it, and its message differs), and it
sits 29 lines later, not eleven. A literal-duplicate detector does not
find it; that is a blind spot #790's sweep reports.

**Every member #790 closed carries a mutation receipt** — the guarded
thing was changed and the assertion was watched go red — because an
argument that an assertion *can* fail is the same kind of evidence this
finding exists to reject. All of them were re-executed by the review.

- (a) `crates/topo/tests/probe_s5_sectors.rs:164-172` — six per-lane
  coverage assertions added so that *"delete the splitting fixtures and
  six assertions go red"*. They cannot: `ci.yml` runs
  `cargo check -p "$c" --features probe --all-targets` and **nothing
  anywhere runs `cargo test -p topo --features probe`.** The file
  discloses the type-check at `:24-36` and then adds assertions whose
  stated purpose is gating. **Confirmed, and TAKEN BY Track F's F8
  (D84)** — it is not a homeless member of this roll-up, it is that
  row's probe-suite finding stated from the other end.
- (b) `crates/sweep/tests/review_m6_5_pr2_sweep_probes.rs:70` — `x3b`
  computes a `Debug` hash and `println!`s it; no assertion. It existed
  to be diffed between the merge-base and HEAD of #220 — a comparison
  that no longer exists — and this diff kept it by re-scoping the doc to
  "two revisions" and renaming the label. **Two independent agents hit
  this.** `memories/test-suite-cost.md` names this the class to drop
  first; the change that touched it made it permanent. The file header
  also now claims the probes *"measure rather than assert"*, false of
  `x4` two functions above. **Confirmed, and ROUTED to D104.** #790
  added the clause the memory was missing: the cross-build differential
  licence **expires with the comparison**.
- (c) `crates/editor-core/tests/boolean_op_wire.rs` — **CLOSED by #790.**
  `the_write_door_admits_every_operation_it_can_read_back` was a
  semantic duplicate of `every_operation_round_trips` and is deleted;
  the file is not. The write door's refusal fires when an operation is
  present in `tag` but missing from `ALL`, and **nothing — inside the
  module or outside it — can construct that state** in a build whose
  `ALL` is complete, so the row was documentation calling itself a
  test. The argument now lives once, beside the check. **Receipt:**
  dropping `Subtract` from `ALL` reddens `every_operation_round_trips`
  *and* `the_operation_rides_the_wire_as_its_variant_name` with the
  door's own refusal message — the deleted row guarded nothing they do
  not. **The other direction is not covered and is not coverable
  here**: an operation added to the kernel that reaches `tag` but not
  `ALL` reddens nothing, because every row iterates a hand-written
  vocabulary literal. #790 folded four such literals into one; the tie
  to the enum needs a proc macro the workspace does not have, which the
  `with` module's own doc already states.
- (d) `crates/sweep/tests/s16_box_soundness.rs` — **CLOSED by #790, and
  re-derived out of disposition (a); this finding over-stated it.** The
  row is a working tripwire, not a vacuous assertion: **mutating
  `gate_operand_kinds` to admit NURBS edges makes the lofted operand
  refuse with `NurbsExtentUnsupported { operand: A, face: FaceKey(3v1) }`**
  — the end-to-end path the re-gate exists for, reachable the moment the
  neighbouring gate lifts, exactly as the row's doc says. What was
  missing was the **premise**: nothing said the operand is NURBS-*faced*,
  so the row would have stayed green over a loft that stopped producing
  NURBS walls. That premise is now asserted, with its own receipt — the
  fixture swapped for an extruded box reddens it. A second candidate
  assertion, that the operand carries NURBS *edges*, was written and
  removed: it is implied by the `CurvedEdgeUnsupported` refusal below
  it, and minting an implied assertion is this finding's own defect.
- (e) `crates/profile/tests/onarc_probe.rs` — **CLOSED by #790.**
  `assert!(!lp.tangent_joints().contains(&anchor_idx))` now runs against
  a **populated** set asserted on the same chain (the opening fillet's
  two joints), and the row states its subject positively: the leg
  leaving the anchor departs on the authored heading of 2.6 rad.
  **Receipt:** with `ProfileLoop::tangent_joints` returning `&[]`, the
  pre-#790 row is green and the post-#790 row is red. **The delta is
  locality, not new coverage** — the same mutation already reddened the
  sibling test 45 lines up, which is what this finding said when it
  named the positive half as living elsewhere.
- (f) `crates/geom-core/tests/d8_knot_queries_adversarial.rs` —
  **CLOSED by #790.** The row asserted `Ok` and finiteness, so a break
  parameter that mislocated a span passed. It now asserts **where the
  spans were placed**: `SurfaceResidual::breaks()` against the
  decomposition the contract admits — the domain ends, **both** curves'
  interior knots, and every extra strictly inside the domain,
  deduplicated on exact `f64`. **Receipt:** with
  `surface_curve_residual` silently dropping the caller's refinement,
  the pre-#790 row is green and the post-#790 row is red on
  `single span / one ULP inside each end`; with it dropping the
  pcurve's interior knots instead, the same row is red only once the
  fixture gives the two curves different knot sets, which #790's review
  found and #790 fixed. The rest of the file, whose case count and
  per-regime floors S78 holds up as the discipline other suites lack, is
  untouched.
- (g) **FIXED by #787** — `demos/wild/src/main.rs`. The vacuous
  `assert_eq!(scenes.len(), WILD_CELLS.len())` is replaced by the two
  properties it stood in for, neither of which was checked: cell NAMES
  are distinct (a name is an STL stem, a scene name and a PNG stem at
  once, so a collision silently overwrote one export and put one body
  on the sheet twice — checked before any cell runs, so the failure
  arrives before the clobber). **One assertion, not two**: the fix pass
  dropped a companion "every named STL exists on disk" check, itself
  near-equal-by-construction — with distinct names, `run_cell` writes
  `{name}.stl` from the same field the manifest then names and panics
  on a failed write, so it could only fire on an external deletion
  mid-run. The site says so, rather than letting S110(g) read as closed
  twice.
- (h) **FIXED by #786** — the subsumed
  `assert!(mine.is_empty() || !mine.is_nai())` in
  `interval-transcendentals/tests/common/mod.rs` is gone; the
  `assert!(mine.is_empty())` below it carries the whole claim, with the
  offending value in its message.
- (i) `crates/sweep/tests/review_d8_consumer_differential.rs:217,298,398`
  — pinned literal seeds (`fuzz::pinned(…, 0x00d8_c0de_0000_000N)`) are
  licensed by the memory for the *digest* half, which is printed and
  never asserted. The same pinned draws also feed real counterexample
  searches (`form.num.breaks() == want` at `:239`, the union-vector
  checks), which will explore the same 24 points for the rest of the
  project's life. Two shapes in one test, of which only one is safe to
  pin. The merge-base comparison that motivated it has happened;
  nothing schedules another. **Confirmed, and ROUTED to D104**: the fix
  is a seed policy, not an assertion rewording.
- (j) **FIXED by #787** — `demos/tour/tests/eps_regression.rs`. The
  finding's "appears to" is confirmed: the tour's only
  `std::process::exit` sites are the two feature-gated modes reporting
  a usage error, and every typed refusal on the scene path is a panic
  by construction. The DOC was describing a contract the demo does not
  implement; the header now states the one it has (green means exit 0,
  that is the whole assertion, and nothing else is available to assert).
  **Whether that is the right posture is now issue #795**, not a
  sentence in a test-file comment: review was right that the first
  replacement deferred without a schedule. The old arm dropped the body
  and still exited 0, so the deleted sentence was never true at any
  commit — doc rotten, not code drift. **Two things came out of establishing that**: no gate had
  ever run this pin — or any assertion under `demos/` — which is S129
  below and is partly closed by the same PR, and `demos/tour`'s unit
  tests turned out to be RED on main (issue #782).

## S111. Frontier vocabulary and API surface with no reachable caller (roll-up)

- (a) **PARTLY CLOSED by #768 (D27), which was aimed at D27's three sites
  and reached four of this member's five.** S7's charge against the
  retired door included *"eleven `unsupported(…)` refusal strings, all
  provably unreachable"*, and the surviving door carried the same shape
  while self-declaring it. Of the five self-declarations this member
  listed: `build.rs`'s *"Neither refusal below is reachable through the
  front door"* is **gone with the function that carried it**;
  `links_here.is_empty()` is **gone** (a corner's incidence list is
  non-empty by shape); `corner_plan`'s `faces.len() != 3` is **no longer
  a self-declared-dead branch** — it is `CornerFaces::admit`, a door
  whose whole job is to decide it, and the derivation downstream reads a
  type instead of re-testing; `EmptyChain` is **gone**, variant and all.
  **What survives is `resolve_rim`'s one-link guard** (*"Likely dead in
  practice"*, `surgery.rs`), which stays typed for the reason written on
  it: nothing in that call proves the battery's screen ran, and no token
  in this unit attests it. **This member is not fully closed and has no
  row** — the orchestrator owns whether the residue earns one.
- (b) `crates/sweep/src/fillet/surgery.rs:778` — `pub fn ring_clearance`
  is production API whose only caller outside the module is
  `crates/sweep/tests/m6_surgery.rs:434`, and whose doc says so. S52
  landed a `feature = "test-support"` gate in this very crate for this
  class; #672 records the residue and does not name this. Also:
  `sweep::fillet::surgery` is `pub` while every other item in it is
  `pub(super)`.
- (c) **FIXED by #786 — the diagnostic was right and the REMEDY was
  wrong.** `git grep '\.intersection('` over the tree returns only
  `src/ops.rs`'s own `#[cfg(test)]` unit test, so the charge stands
  exactly as written: a `pub` function with **no production caller**.
  (#786's first write-up said the charge "was wrong about the code",
  counting one test function's five assertions as call sites. It was
  not; that sentence is withdrawn, and ruling **G-R8** is amended to
  match.) What the finding got wrong is *delete it*:
  `docs/semantics-diffs.md` §D7 defines `hull`'s single deliberate 1788
  divergence BY CONTRAST with `intersection`, so deleting it deletes
  what a live argument points at — S74's mechanism, one document over.
  It is kept, and the zero-caller fact is now recorded rather than
  papered over: `docs/inventory.md`'s new set-operations table says
  `none today` in the callers column, and `lib.rs`'s scope paragraph
  says outright that *"everything here has a caller"* is false and why
  this one is the exception. The same diagnostic also convicted
  `docs/inventory.md`, which omitted `hull` by the identical test while
  `hull` is unquestionably used; both set operations are inventoried
  now. The mirror case, `consts.rs`'s `neg_frac_pi_2`, is `pub(crate)`
  and IS `-frac_pi_2()` rather than a second hand-written endpoint pair
  — which puts it under `certify_constants`' cross-check transitively,
  the step between them being exact (verified bit-identical on both
  endpoints by #786's adversarial review); two unit tests pin that and
  the constants' one-ulp brackets.
- (d) `crates/sweep/src/fillet/naming.rs:56` — `Retired` still has no
  face channel, so the one thing it exists to catch (a source entity
  destroyed without a record) is structurally uncatchable for faces. The
  whole-body door's retirement made the emitter's *"faces are never
  retired"* comment true of today's surgery; the hole S15 named is
  unchanged, asserted only over two fixtures.

## S112. Prose that describes a world the code has left (roll-up) — **six of eight closed, and the ledger is the deliverable**

**The class ledger, per G-R2**, which re-scoped **G10** to *"(g) plus the
class ledger"* and made the row's retirement turn on the ledger rather than
on the closing lane's own member. Every line below was **re-derived from the
tree** at this branch's final merge of `origin/main` (`cfdc1c6f`) and from the
merge commit named — not taken from a dispatch list; two of the eight came back
different (below). **Every `file:line` in this whole section — the
table AND the member bullets under it — was re-derived against the tree after
this PR's final merge of `origin/main`**, which is S176(a)'s discipline applied
to the entry that records it. Six had drifted and are cited by target name or
by expression instead; what line numbers remain are the ones that re-derived
at `cfdc1c6f`, and they are anchored to it rather than to a different commit
from the one the census names.

| member | site | closed by | how it was verified |
|---|---|---|---|
| **(a)** | `sweep/src/fillet/naming.rs`, the *"What consumes these rows"* header | **NOBODY — open, and its owner has retired** | the sentence stands; `editor-core/src/names/emit_fillet.rs:220-221` still builds `retired_e`/`retired_v` out of `rec.dead` and consults them at `:236-246`, so the defect is intact. Routed to Track E's **E-g** after that lane was dispatched; E-g landed as **#768** and its own §D row says S112(a) is **not in it**, and that row is now struck from §D. See **S177** |
| **(b)** | `interval-transcendentals/tests/certify.rs` | **#786** (G-a / G1), commit `520f21f1` | the header names `ci.yml`'s `oracle-certify` job, its `ORACLE_PATHS` trigger and `CAD_FUZZ_EFFORT=8` (`:21-24`); `ci.yml` no longer contains *"stays a by-hand gate"* |
| **(c)** | `interval-transcendentals/src/ops.rs`, `docs/inventory.md` | **#786** (G-a / G1), same commit | `ops.rs:4` now opens *"`copysign` is deliberately NOT here"*; `docs/inventory.md:40` says the same and points at the trait impl. The code did not move and S1 is untouched |
| **(d)** | `geom-brep/src/props/curved.rs`, `cone_arm`'s doc | **#877** (I-a), and re-homed off C-m by ruling **I-R3** | the sentence named an ordering that is true at ONE of `cone_arm`'s two call sites and false at the other — `boundary_material_sign` never calls `du_of_rims` at all. Replaced by the invariant both routes establish; **N3 in `docs/predicate-dimension-audit.md` carried the same false claim and is corrected in the same PR**; `s58_iso_rectangle::a_rim_free_cone_refuses_at_both_doors` is the row |
| **(e)** | `geom-brep/src/ssi/exhaust.rs:92` | **open — Track C by mechanism, but named in NO live §D row** | `exhaust.rs:92` still says *"The floor used, in meters"* while `ssi.rs`'s `account_chart_plane` call still passes `domain.floor(band) / speed` (cited by expression, per **S176(a)**). The frozen table's C-m/C3 entry lists **S112(d) only**; no row in §D names S112(e) or `geom-brep/src/ssi/`. Its only live mention is G10's own row |
| **(f)** | `profile/src/sugar.rs` | **#831** (G-d / G10) | below |
| **(g)** | `crates/pncad/src/lib.rs` | **#831** (G-d / G10) | below |
| **(h)** | `demos/render.py` | **#787** (G-b / G2), commits `85510376` + fix pass `040fb699` | no `stl is None` branch, no *"#111 pin"*, no `drawn` counter and no boolean return in `render.py`; `render_freecad.py:164` reads `body["step"]` with the guard gone |

**Two corrections the walk returned, both from the tree.**

1. **G10's sentence mis-splits rather than over-counts.** It says *"three of
   them (`geom-brep/props/curved.rs`, `geom-brep/src/ssi/`) are Track C's …
   the rest are free"*. **The free five are exactly right** — (b), (c), (f),
   (g), (h), every one of them Track G's. What is wrong is *"three"*: Track C
   holds **two** members, (d) and (e); the third body in that parenthetical is
   **(e)'s second file**, `ssi.rs`, counted as a member. The eighth is **(a)**,
   which is Track E's and is not in the parenthetical at all, so the sentence
   reaches the right total by two errors that cancel. **G-R2 located the
   over-count in *"the rest are free"*; the tree puts it in *"three of them are
   Track C's"*.** The ruling's conclusion — the sentence is wrong by one, the
   rides-along paragraph is right, (a) is not this row's — is unaffected.
2. **This row cannot retire on this ledger.** As the walk found it, three
   members were open and only one of the three ((d)) was named by a live §D
   row. **(d) closed at #877** (Track I, lane I-a), which leaves **(a) and (e)
   open and NEITHER named by a live §D row** — the half-fix shape G-R2 invoked
   to require the ledger in the first place, now with the one tracked member
   gone. **G10 stays in §D**, re-scoped to the residue; the general form is
   **S177**, and **D124** is the row that re-homes (a). (e) — `ssi/exhaust.rs`'s
   *"The floor used, in meters"* against a chart-unit store — still has no row
   anywhere, and Track I's scope does not reach `geom-brep/src/ssi/`.

- (a) `crates/sweep/src/fillet/naming.rs`, under *"What consumes these
  rows"* — *"`editor-core`'s
  `names::emit_fillet` … reads every field EXCEPT [`Retired`]"*.
  `emit_fillet.rs:220-221` builds `retired_e`/`retired_v` straight out
  of `rec.dead` and consults them. The diff rewrote the paragraphs
  immediately above and below and left this one.
- (b) **FIXED by #786** — `certify.rs`'s header now names
  `oracle-certify`, its `ORACLE_PATHS` trigger and its effort setting,
  and gives the by-hand invocation as an addition rather than as the
  gate; `ci.yml`'s *"stays a by-hand gate"* sentence is gone from the
  `interval-backend` job's header.
- (c) **FIXED by #786 — the promise, not the placement.** `ops.rs`'s
  header and `docs/inventory.md` now say that `copysign` is on the
  `Real` surface but NOT in `src/`, that it lives with the trait impl
  at `crates/geom-core/src/interval.rs:356`, and why: a `sign` operand
  containing zero has no sign to transfer, so the result is a hull of
  `±|self|` with the decoration capped at `Def` — not endpoint
  selection. **The code did not move, and S1 is untouched**: where it
  should ultimately live is part of the `RingInterval`-vs-`Interval`
  question, and this member's fix is only to stop two documents
  promising code that was never there.
- (d) **FIXED by #877 (I-a) — and the sentence was not fine.**
  `cone_arm`'s doc said the `T::one()` fallback *"covers the no-rim
  case, where `du_of_rims` refuses before any margin is metered"*.
  Checked from the inside: `cone_arm` has **two** call sites, and
  `boundary_material_sign`'s cone arm never calls `du_of_rims` — it
  computes the arm, then refuses on `rims.first()`. So the sentence
  named a route that exists at the flux lane and does not exist at the
  gate. It is now the invariant both routes establish — the fallback IS
  reached (both callers compute the arm before they know whether there
  is a rim) and is **never metered against**, because every route from
  it to a margin refuses on the empty rim list first. Audit note **N3**
  carried the stronger and also-wrong form (*"the `T::one()` fallback is
  unreachable"*) and is corrected with it.
- (e) `crates/geom-brep/src/ssi/exhaust.rs:92` / `ssi.rs`'s
  `account_chart_plane` call (**cited by expression, per S176(a)** — the
  line moved by 13 under a merge) — `Exhaustiveness::floor`'s public
  doc says *"The floor used, in meters"*; the chart lane stores chart
  units (`domain.floor(band) / speed`). The sibling field on
  `ExhaustivenessInconclusive` gets it right (*"in meters (or chart
  units)"*), and so does `SweepCell::width`. The one place a caller
  reads the number back out is the one place the unit is wrong. S23's
  refactor made it visible by collapsing two lanes onto one parameter.
- (f) **FIXED by #831 (G-d / G10).** `arc_fillet_trims`' header claimed a
  *current* two-consumer property — *"extracted verbatim from the raw
  builder's corner door so the twin and the PATHS algebra lowering share
  one code path"* — with no twin and no raw builder in the tree. It now
  states the one consumer it has (`path::arc_fillet`'s lowering, the only
  call outside the module, checked workspace-wide) and puts the
  extraction in the past tense, keeping what the extraction still buys:
  the surviving lowering calls the ratified construction instead of
  carrying a second copy. The three other *"raw builder"* sentences —
  `sugar.rs`'s `arm`-mapper comment and its gated-methods comment, and
  `path.rs`'s `set_leaving_bulge` note — are untouched: they read as
  historical provenance and the finding said so. (**Cited by target, not by
  line**, per **S176(a)**: this PR's own edit six lines above moved one of
  them.) **This member is why G5's re-read
  fence on `sugar.rs` was lifted for it, and only for it** (G-R2); the
  enclosing-candidate machinery §D fences off is unread and unedited.
- (g) **FIXED by #831 (G-d / G10).** *"What the façade itself contains"*
  enumerated `authoring`/`validated` plus the `workspace` exception and
  did not mention `tolerance`, added in the same wave as the S20 rewrite.
  The section now names it as the third thing, and states the fact that
  made the omission worth fixing rather than merely completing the list:
  `tolerance::report` and `tolerance::eps_source` **commit the ambient ε
  bootstrap as a side effect of being asked** (exactly `Tolerance::get`'s
  behaviour), so a program that later loads a document turns that load
  into a `ToleranceConflict` by having asked — the one place in the
  façade where calling a wrapper changes the run, against a section whose
  claim is *"no geometry and no numeric behavior"*.
  `tolerance::committed_report` is the door that does not, and is named
  as such. Verified by `scripts/doc-gate.sh` (exit 0) and `cargo test -p
  pncad --doc` (34 passing).
- (h) **FIXED by #787** — `demos/render.py`. The `stl is None` branch,
  its `#111 pin` warning, the `drawn` counter, `draw()`'s boolean
  return and `main`'s skip-the-scene arm are all gone: neither producer
  can emit that state, and `run_stop` never emits a bodiless scene, so
  the empty-scene path below it was unreachable for the same reason.
  `render.py` and `render_freecad.py`'s `Mesh.insert` call now read
  `body["stl"]` the same way. The docstring states what the two PRODUCERS do and defers
  what the FORMAT promises to (c) — a guard against a state nothing
  emits is wrong under every schema, so this half did not wait for it.
  Verified by rendering all 35 tour scenes before and after: identical
  PNGs, chunk for chunk.
  **Both twins, after the fix pass.** The first pass closed the
  `render.py` half and left two siblings standing, both found by
  review: `render_freecad.py`'s `if not body.get("step")` guard (since
  deleted, so there is no line to cite) guarded a
  null `step` that this reader's producer (the tour) cannot emit — the
  wild generator writes one for every cell but is rendered by
  `render.py` and never reaches that file — and `render.py`'s own new
  docstring claimed the two readers read "the same field the same way",
  true of `stl` and false of `step`. Guard deleted with its producer
  named at the site; docstring narrowed to the claim it can support.

## S113. Counts and enumerations stated in prose, already drifted (roll-up)

Beyond S64, S67, S74 and S98:

- (a) **FIXED by #787** — every count in this member is COMPUTED
  rather than restated, because a fresh number would drift on the next
  scene exactly as these did. The tour's closing lines now print
  `35 scenes (20 montage cells, 15 standalone)`, `1049 face charts`,
  `946 checkable / 103 branch-jumped / 0 disagree`, and the two worst
  junction gaps; `uvdump.rs` and `compose_uv_montage.py` point at that
  line and state no number. **Measured on `de103835`, and every prose
  figure was wrong**: 982 → 1049 faces, 879 → 946 checkable, and
  `9.4e-16` → `9.93e-16` for the worst 3-D loop gap (`uvdump.rs:294`,
  which the finding did not name). `Cargo.toml`'s *"eighteen scenes"*
  is deleted at both sites — it carried no load in either sentence and
  a TOML comment cannot compute one. The sweep also reached
  `demos/README.md:346` (*"the tour emits 34 scenes … 34 − 15 = 19"*,
  drifted to 35/20; the paragraph now states the RULE for staying off
  the sheet and leaves the arithmetic to the run) and
  `demos/render-wild.sh:14` (*"8 cells"*, a second copy of the count
  (b) had just moved into `WILD_CELLS`'s array length).
- (b) **FIXED by #787** — `demos/wild/src/main.rs`. The heading's
  *"6 cells"* is deleted rather than corrected to eight: the derivation
  it headed already ends at 8, and `WILD_CELLS: [Cell; 8]` is the
  enforcement — a cell added or dropped without re-deriving is a type
  error. The doc now says so, and says why the count is not written in
  prose a second time. `render-wild.sh`'s duplicate *"8 cells"* went
  with it.
- (c) `crates/geom-core/src/ring_interval.rs`'s `from_certified` doc
  carries a prose census of its own callers, already corrected once
  (`88616177`). See S89.
- (d) `crates/topo/src/chart_region.rs:875-891` — `SCHEDULE_2D`'s
  members 9 (`[0.75, -1.0]`) and 15 (`[-0.75, 1.0]`) are exact
  negatives, and both graze conditions in `ray_verdict` are invariant
  under negating `d`, so the sixteen-member retry ladder is fifteen. The
  S17 unification added the claim *"axes plus oblique spread members"*
  directly above it. Related: `splitting/containment.rs:126` still calls
  its dyadic-rational table *"golden-angle-spread"*.

## S114. Duplications the fix passes did not reach (roll-up)

- (a) **FIXED by #786** — one `TWO_PROD_VALID_MIN` and one
  `two_prod_witness(x, y, z, mag)` in `round.rs`, called by `mul`, `div`
  and `sqrt`. The FMA witness went from four spellings to one and
  `SQRT_EXACT_WITNESS_MIN` is gone with its prose reconciliation;
  `derivations.md` §3 already derived ONE floor for all three, so the
  code now matches the derivation's shape. `round.rs`'s header no
  longer claims the lemmas are restated at each helper — it says the
  shared one has a single home.
- (b) **FIXED by #787** — `demos/tour/src/skinned.rs`. The
  *"VERBATIM"* claim was false twice over: nothing computed byte
  equality, and `fn quad` is not even the same code — the corpus builds
  the section with `ProfileLoop::polygon`, the demo through the PATHS
  lattice (`paths::path_polygon`), which is the spelling the tour
  exists to show. The duplication STAYS, stated as deliberate with its
  reason at the copy site: the fixture's other home is another crate's
  test-support module, unreachable from outside that crate's test build
  and not something a user could import, so reaching into it would make
  the demo worse evidence — and no public door hands out corpus
  fixtures. What the claim was FOR is checked instead: `stops()` asks
  `mass_properties` for the prism's volume and pins it inside its own
  certified enclosure against the 9 m³ that
  `sweep/tests/m6_loft_body.rs` DERIVES for the fixture. **Corrected in
  the fix pass, on both halves review named.** The number is no longer
  a `9.0` literal — it is computed from the sections themselves
  (`V = 8 + 8d/3`, `d` read off `PRISM_TRAPEZOID`), so a section
  drifting *here* makes the note wrong before the kernel is asked; and
  `volume_pad` is now **bounded from above** before it is used as a
  tolerance, because `|V − v| ≤ pad` alone gets easier as the enclosure
  degrades (G-a's `assert_contains` shape, reproduced in `demos/` by a
  fix pass for a different finding). What the pin does **not** check —
  agreement with the corpus fixture, which this demo cannot read — is
  stated at the site instead of asserted in the panic message. `ELBOW_H`'s
  *"shared"* becomes *"the same value, copied; nothing links them"*.
  **Not this unit**: the byte-identical three-way `fn quad` across
  `sweep/`, `mesh/` and `step-export/` test-commons lives in three
  crates' test trees, not in `demos/`.
- (c) **FIXED by #837** (the design question was closed by Evan,
  2026-08-20; the schema framing was refused and the emitter half
  ruled *no shared type*). **The duplication was in the READERS**, and
  the fix is one: `demos/manifest.py`, which owns the walk, the field
  names and the `view.up` convention. `render.py`,
  `render_freecad.py`, `compose_montage.py` and `render.sh`'s scene
  lister all read through it. **No schema, no serde type, no
  validation layer and no new crate edge.**

  **The `view.up` convention is now written once, in the world →
  display direction only**, as a signed permutation; the display →
  world direction a camera needs is *derived* by inverting it. The two
  hand-written maps that were exact inverses by coincidence
  (`render.py:51-57`, `render_freecad.py:105-133`) cannot disagree
  because there is no longer a second one to disagree with. Its
  selftest pins the convention against both spellings it replaced and
  pins that the derivation really is an inverse, in both compositions.

  **Both defaults are gone, and gone by being unnecessary rather than
  by being unified.** `transparency` and `montage` are now read, not
  `.get`-defaulted, because both producers write both keys for every
  body and every scene — so a producer that stopped would fail the
  first render instead of rendering something plausible. `step` KEEPS
  its defensive posture and says why: it is nullable in fact (the wild
  generator writes `null` for every cell, because its STEP is an input
  fixture rather than something the pipeline exported), and the one
  reader that opens it names its own producer at the site.

  **The eight `uv.json` fields nothing read are deleted** — `scene`,
  `half_edges`, `cached`, `area`, `sense`, `winding_ok`, `gap`,
  `chart_jump` — after re-deriving the claim rather than trusting it:
  seven are consumed Rust-side by `draw_face` *before* serialisation
  and stay in `FaceStats`, drawn into each cell's own SVG; only
  `FaceDump::scene` was dead end to end and it is gone with its
  plumbing. The corpus-wide aggregates of the same numbers are printed
  by the run that measures them, from the in-memory dumps, so nothing
  that was computed became uncomputable. Every field `uv.json` still
  carries is read by `compose_uv_montage.py`.

  **The wild emitter now writes the tour's field set** (`transparency`
  added; `step` stays `null`, which is the fact and not a placeholder)
  — **independently, and both sites say the agreement is deliberate
  and unenforced**: no shared type, no dependency edge, nothing
  comparing the two emitters. That is the ruling, verbatim: *"seems
  fine to make the two renders line up 'coincidentally' on step and
  transparency. sharing for just those two fields is overengineered
  even ignoring the realism concern."*

  **Measured, on this tree.** No committed artifact moved. All 35 tour
  frames and all 8 wild cells render byte-identical old-vs-new
  (including the four `up: "y"` scenes and the transparent `klein`),
  the tour montage sheet is byte-identical, the scene lister returns
  the same 35/20 names, and `renders-uv/montage-uv.svg` regenerates
  clean. `freecadcmd` is not installed on the lane box, so
  `camera_rotation` was checked locally by extracting both revisions
  and comparing the camera basis under an `App.Vector` shim over 151
  views — 43 from the two committed manifests, 108 synthetic, 50
  through the straight-down fallback (2 of them committed scenes) —
  bit-identical throughout. **Hosted CI is the verification of record
  for that lane and does run it**: `ci.yml`'s `render lanes` job calls
  `render.yml` through its `workflow_call` entry, all four lanes, on
  every PR the change filter marks as building (`run_k_lint`) — so not
  on a docs-only one — and reports render drift.

  **What is not enforced, stated.** Nothing compares the two emitters'
  field sets; that is the ruling's choice, not an oversight. What
  exists instead is that the shared reader *reads* rather than
  defaults, so a drift surfaces at the first render.
- (d) **FIXED by #786** — `sin`/`cos` share one `sinusoid` body
  parameterized by the libm entry point and the two extremum phase
  offsets; `asin`/`acos` share `clamp_to_unit` (propagate, full-miss
  Empty, partial-miss clamp-and-poison) and each keeps visible the half
  that genuinely differs, which endpoint feeds which bound.
  `Decoration::continuous_on` and `DInterval::is_bounded` replace the
  hand-spelled idiom everywhere. **Count corrected:** the decoration
  idiom was FIVE sites, not six — the sixth spelling of `bounded` is
  `tan`'s, where it is a refusal condition rather than a decoration.
- (e) `crates/geom/src/curves.rs:942` and
  `crates/geom/src/surfaces.rs:1118` — `fn contains` duplicated with two
  parameter names, in the same crate, beside the four converters the
  dedup did collapse.
- (f) **FIXED by #887** — `crates/mesh/src/planar.rs` and
  `crates/mesh/src/trimmed.rs`. Both #678-sibling arguments are now
  **checks at the site whose fact they are**, and both shrank.
  `trimmed`'s twenty-eight lines became nine plus two `debug_assert`s:
  *no chart singularity reaches this lane* is asserted where the lane
  is chosen (`Chart::of(surface).is_none_or(|c| c.poles().is_empty())`
  — the `walk` fact the comment was quoting, now unable to move
  silently), and *the seam double-traversal cannot meet a single
  interior column* is asserted where the count is
  (`nu != 2 || !id_repeats_apart(&polygon)`). `planar`'s nine became
  seven plus `cdt.num_vertices() == meta.len()`, which is *"there is no
  interior grid at all"* stated so that adding one fails the line
  rather than leaving the comment false. The new helper carries its own
  row (`a_repeat_at_one_uv_is_not_a_repeat_apart`), including the ulp
  case that decides whether two seam copies are one CDT vertex or two.
  Both asserts demonstrated red by hand. **Disclosed**: the mesh suite
  runs the trimmed cylinder lane on 19 faces with `nu ∈ {5, 16, 50}`
  and `id_repeats_apart` false on every one, so that assert is a guard
  rather than a live check and both of its operands had to be forced to
  see it fire.

## S115. Disclosed and unscheduled (roll-up)

Every item here is honestly written down, and none has an issue number
or a named plan unit. This is C2/C3 measured after the A1 rule landed;
see §C.

- (a) `tools/tess-lint/src/lib.rs:26-34,141-145` and
  `tools/tess-meter/src/lib.rs:239-248` — the `agree` column is
  `grid_cells / span_cells` where `span_cells` is assigned `grid_cells`
  verbatim, so it is `1.00` by arithmetic. **Both crates** say so at
  length, both say it *"cannot detect the drift it was described as
  detecting"*, and both give the same reason for keeping it: *"a schema
  change and a re-cut baseline, and is unscheduled."* Meanwhile the CLI
  prints it to an operator in the ranked table and the totals line,
  labelled `agree`. Two crates independently documenting why a column is
  inert is more effort than deleting it.
- (b) `scripts/doc-gate.sh:45-58` — *"a row is owed that does the same
  for every excluded root, and it is unscheduled."* ~1,050 lines of
  `tools/tess-meter` prose went from covered to uncovered by moving, in
  a gate whose existence argument is that prose which stops rendering is
  a real loss. #709 is cited as the cause, not the schedule. (S71's
  dangling intra-doc link is in a `tests/` file and outside this gate
  too — and **S135** measures that hole: rustdoc builds no test targets
  at all, so it is not an excluded root but an excluded *target kind*,
  a few hundred links wide — the census is at S135 and is not restated
  here, per S176(b).)
- (c) `crates/pncad/src/prelude.rs:11-21` — the corpus-frequency
  measurement that chose the prelude cut *"was taken once, by hand, and
  nothing re-takes it"*, classified as *"unguarded rather than
  unguardable — a re-run of the import census would guard it"*. By its
  own account a guard is available and not taken; no import-census row
  exists in `ci.yml`.
- (d) **FIXED by #872** — `crates/mesh/src/walk.rs`'s D2-addendum
  `debug_assert` deviation, disclosed with *"A typed warning channel
  would dominate all three; there is none."* It has a schedule now:
  **issue #868**, which names the four things a fix has to decide
  (where the channel lives, what a warning carries, how a caller
  receives it against D9 byte-identity, and whether the three
  detectors then become D2 rows 1/3 proper). Both claim sites cite it
  — the finding named one, and the lane's sweep found a **second copy
  of the same disclosure** at `closing_column` (*"would dominate
  both; there is none"*), which had never been recorded.
- (e) `crates/editor-core/src/eval/wire.rs:717-723` — see S105.
- (f) `crates/topo/src/euler.rs:3220-3251` — `strum::EnumCount` named as
  the way out and declined; see S94.

## S116. Naming, shape and residue — no mechanism at stake (roll-up)

- (a) `crates/geom/src/net.rs:20-90` — the control-net trait replaced
  `vec![lift(|p| p.x), lift(|p| p.y), lift(|p| p.z)]` (exhaustive by
  construction, no panic reachable) with a `0..P::CHANNELS` loop through
  `channel(d)`, so correctness now rests on each impl's `CHANNELS`
  agreeing with its own match arms — announced with two `unreachable!`
  arms that did not exist before, and a nine-line comment explaining
  that they are *"unguardable by construction"*. The `unreachable`
  family is deliberately outside the panic ban, so this is taste, not
  policy — but it is a hazard the abstraction introduced.
- (b) `crates/geom/src/` now has three modules named `projection`, two
  named `boxes` and two named `nurbs`; inside `surfaces/projection.rs`
  the line `use crate::projection::{…}` and the module's own name refer
  to different things. And `azimuth::frame` returns
  `(radial, tangential)` — both `Vec3<T>`, so a transposed destructure
  compiles silently, which `azimuth.rs:64-80` says and points at
  indirect coverage — while the overwhelming majority of call sites take
  `(radial, _)` or `(_, tangential)`. The header's two-door rationale
  does not match what the arms need, which is three shapes.
- (c) `crates/topo/src/sector_face.rs:118-123` — three things in one
  crate are named `sector_face`, and the shared module's doc concedes
  the collision in advance (*"so `sector_face` in prose means one thing
  per scope instead of three"*) rather than resolving it. Inside the
  boolean wrapper, `match resolved.carrier { Plane | Cylinder | Sphere
  => {} }` is a no-op whose only purpose is to make a future variant a
  compile error — an invariant held by a statement where a type would
  hold it structurally.
- (d) **FIXED by #787** — `demos/tour/src/main.rs`. Both forwarders
  are deleted and their four call sites say `plain`/`seamed`; what
  `seamed_curved`'s doc was actually for (a curved boolean result
  validates the same way — the contacts decide, not the surface kind)
  moved onto `seamed`, which had no doc at all. `step_expected` is gone
  with both of its checks: the subset-frontier arm now panics outright
  and says what reaching it would MEAN (a scene grew past the writer,
  not a writer that broke), which is what the conditional was
  pretending to decide. `ManifestBody.step` follows it from
  `Option<String>` to `String`, since the arm that produced `None` no
  longer exists. **And so does everything downstream, after the fix
  pass**: with no `None` path left, `run_body`'s `Option` return,
  `run_stop`'s `filter_map`, its `if bodies.is_empty()`, its `-> bool`
  and `main`'s `if run_stop(…)` were dead, and `run_stop`'s doc
  described a staged-stop world the code had left — **S112's own class,
  created by the PR closing S112(h)**, and caught by review. Tour output
  byte-identical before and after, verified against `origin/main`'s
  `demos/` on the same kernel.
- (e) `crates/topo/src/euler.rs:1-208` — the module header grew ~55
  lines in this diff, and the new material is a titled essay
  (*"**The exception, and it is a real one.**"*) about `crate::instance`'s
  graft — which is `instance`'s contract, and a reader of `instance.rs`
  will not find it. Two screens of taxonomy prose before the ten Euler
  operators. C5, measured.
- (f) `crates/topo/src/review_m1_pr2/release_corruption.rs:187-202` — a
  hard **10 ms wall-clock bound** in the one release-profile job that
  gates, defended by twelve lines giving the measurement (1.5 µs
  release) and an explicit statement that CI's box is different and
  slower. The clause it defends (*"every traversal is bounded"*) was
  already detected by the previous `< 5 s`. No evidence it has flaked;
  the thoroughness of the defence is what draws attention.
- (g) **FIXED by #887, closing the residue #872 narrowed** —
  `crates/mesh/src/curved.rs`. S28's shared core still does not exist
  and this lane's answer is still an argument; what changed is that the
  argument no longer outweighs the code it defends.
  `docs/prompts/implementer-discipline.md` §4 was the lever — *comments
  state the invariant, not the history* — and the three guard docs were
  roughly 60% history.

  **Re-measured by script at both ends** (production half = everything
  above `#[cfg(test)]`), merge base `ecc1d492` → #887: the file's
  production half **681 lines / 404 comment (59%) / 259 code → 631 /
  348 (55%) / 265**; `entries_off_bbox` **52/20 → 45/26**;
  `require_swept_rectangle` **94/24 → 57/24**; `pole_columns`
  **82/3 → 56/3**. The three guard functions together: **228 doc / 47
  code → 158 / 53**. **Comment down 56 lines, code UP 6** — the ratio
  moved because argument became check, not because prose was thinned.

  **What went, and where each claim lives now**, is a per-deletion table
  in #887's body. Three are worth naming here: the **six-site,
  four-file sweep roster** is deleted rather than restated — its own
  text called it *"an unguarded list of the kind S64 was about"* — the
  `iso_side_starts` qualification is cut to its pointer because it
  already said *"stated in full at `walk::iso_side_starts`"*, and the
  A/B sweep behind `pole_columns` is kept but **labelled Historical**,
  since nothing in the tree re-derives it. Every live invariant, every
  reader warning and both of #678's answers survive at a named site.

  **What this does NOT do** is build S28's shared core, which was
  answered with a ruling and is not this row's.
- (h) `crates/mesh/src/nurbs_cert.rs:374-419` — S29's constant count did
  not go down. `SAFE_ASPECT = 5.0` is unchanged and still sits above its
  own derived √15 ≈ 3.87; `MAX_GRID_RETRIES` is still a bare `6`; the
  12-vs-6 sample-density split survives. What changed is that
  `SAFE_ASPECT`'s doc grew from ~20 lines to ~50, adding a register
  whose first bullet concedes the guard is one-sided and whose last
  paragraph concedes the 0.60·δ margin has no guard at all — a more
  honest version of one undecided question, not fewer decisions.
- (i) `tools/tess-lint/src/lib.rs:36-39` — `split` is printed under a
  name that denies two of the three things it measures: `grid_cells /
  span_opt_cells` mixes the cheaper split point, the banding's
  max-across-u cost, and the aspect snap. `tess-meter`'s doc says so;
  `tess-lint`'s doc and the CLI legend the operator reads say only
  *"what a cheaper split point per cell would still recover"*. Commit
  `1aba0704` is this in the record: a safety fix moved the number the
  gate calls a sizing regression.
- (j) `tools/k-lint/src/lib.rs:231,368-371` — adding a name to
  `EPS_COUPLED_PREDICATES` moves it from the metre floor (4e-5) to
  `1.5e2·ε` — 2.4 decades looser at ε=1e-9 — and exempts it from rules
  (2) and (3). Nothing tests membership. The CLI's discipline message
  enumerates two recourses and does not mention this one, so the
  cheapest available move is the one the tool never warns about.
- (k) `tools/tess-lint/src/main.rs` uses bare `println!` throughout
  while claiming to follow *"`k-lint`'s three-voice split, and for its
  reason"*; `k-lint` has a deliberate `say()` wrapper so that
  `k-lint … | head` ends quietly. `tess-lint … | head` panics and exits
  101 — a fourth, unnamed voice.
- (l) `tools/k-lint/src/lib.rs:1-209` and `tools/tess-lint/src/lib.rs:1-92`
  are outside every doc gate; the CI job adds a `cargo doc` step for
  `tess-meter` only, with a comment stating *"the same hole covers every
  excluded root"*. `k-lint`'s 209-line header is the densest
  intra-doc-linked prose in `tools/`. Cf. S115(b).
- (m) `scripts/gates/bounds-allowlist.sh:24-30` says the argument
  *"live[s] in ONE home: geom-core/src/real.rs … Not restated here; keep
  this a pointer"* — and is immediately followed by ~100 lines restating
  the `chart_region`, `edge_nurbs`, `arc_fillet`, fillet-battery and
  `CertifiedBounds` rulings that `real.rs:365-556` also carries. S13's
  complaint was a ~157-line ledger in front of a two-method trait; the
  fix gave that ledger a second home in front of a 20-line grep. The
  copies already drifted: `:69-73` is a self-correction (*"This
  paragraph used to say … Both clauses are false"*).
- (n) `scripts/gates/signed-zero-one-home.sh:108` —
  `PAT_ADD_REVERSED='0\.0 \+ [A-Za-z_*(]'` fires on `0.0 + eps`, which
  the header implies it exempts (*"`0.0 + 1e-12`, a real offset"*). In a
  numerics codebase the real offset is more often a named epsilon than a
  literal. The header's blind-spot section lists five things the gate
  cannot see and no false positives; the green-case fixture plants only
  literals. Otherwise the best-tested gate in the directory, which is
  why the omission stands out.
- (o) `scripts/k_probe_sweep.sh:96,99` — the demo-scene half has none of
  `run_dump`'s guards (nonzero passed-count, ≥2 CSV lines) and merges
  with `tail -n +2`. A tour that emits a header and no scenes merges
  silently into the linted CSV. The header at `:50-56` claims *"this is
  the only place that can tell the difference between 'clean' and 'ran
  nothing'"* — true of the corpus and M2 halves, not of the demo half
  sitting between them. `rundump-guard-selftest.sh` proves all three of
  `run_dump`'s refusals fire; the self-test's coverage is defined by
  which code was extractable as a function.
- (p) `crates/sweep/src/revolve/mod.rs:391,544` — `MultipleAxisRuns`
  went from *"the boundary would split into multiple shells (M3)"* to
  *"a **permanent** refusal under the ratified sweeps-vs-voids invariant
  … not a deferral"*, and its `Display` from *"deferred to M3"* to a
  four-clause recourse. That is a change in what the kernel promises,
  resting on an unstated geometric claim (that every profile with ≥2
  disjoint on-axis runs encloses a void when fully revolved). Probably
  true for a validated simply-connected profile; nothing in the diff
  proves it and no test pins it. The sibling `FullRevolveHoles` has its
  own argument written out. (Confidence: unsure.)
- (q) `crates/topo/src/seqgen.rs:839-849` — the `SplitEdge` roundtrip
  asserts only `canonical_form(before) == canonical_form(after)`, and
  `iso`'s module doc — edited in the same diff — says curve/surface
  payloads are ignored. So the `set_edge_curve(e, spec)` that
  `split_site`'s doc justifies at length is not checked by the property
  it was written for; delete it and the roundtrip still passes.
- (r) **FIXED by #786, narrowed to what it really was — a pointer
  problem.** The analysis was never missing: `docs/semantics-diffs.md`
  §D7 has it. What was missing is a pointer where the reader hits it, so
  `lib.rs`'s decoration paragraph now enumerates the three things that
  lower a decoration to `Trv` — a domain clamp, `atan2` at the origin,
  and `intersection` on EVERY input — and says why the third is named
  there rather than only in the divergences file. §D7's argument is not
  restated; a second home for it would be S13's defect. The consumer-side
  half (`crates/geom-core/src/interval.rs:135-143`) is outside this
  lane's workspace and is **not closed** — scheduled as **S134** / §D row
  **D78** rather than left as a sentence.
- (s) `crates/geom/tests/surfaces/m5_pr7_surface_projection.rs:224-228`
  and `crates/geom/tests/curves/projection.rs:219` — both new overflow
  rows are built around *"Finite inputs throughout"*, which is the
  load-bearing half of the claim; the surface row checks
  `p.x.is_finite() && p.z.is_finite()` and skips `y`, the coordinate
  that actually varies across the fixture, and the curve row checks `x`
  alone.
- (t) **FIXED by #786** — `bench.rs`'s header gives the invocation that
  works (`--features oracle-inari` plus inari's AVX+FMA floor) and states
  what cargo does otherwise, both verified: `cargo run --example bench`
  refuses outright, `cargo build --examples` matches no target.
  `cmp_f64_vs_rat`'s deviation is discharged by USE rather than by a
  schedule — it is what the new `×` lanes of `review_fuzz_exact.rs`
  (formerly `review_fuzz_div.rs`) compare against, and the
  `#[allow(dead_code)]` is gone.
- (u) Residue: `crates/sweep/tests/m5_pr12_refusals.rs:518` has a
  leftover `let p = Point3::new(0.0,0.0,0.0); let _ = p;`.
  `crates/pncad-py/src/py/doc.rs`'s raise-site literal was S104's
  sibling and is **FIXED with it** by #833 — the tag now comes from
  `tags::workspace_error_tag`.
  `interval-transcendentals`' `2^-960` vs the literature's `~2^-969` is
  a nine-binade round-up justified as absorbing *"every boundary
  quibble"* — empirical rather than derived, harmless because
  over-gating only costs tightness, and the one number in that crate
  that is chosen rather than proven.

## S117. Twelve source-text guards, five hand-rolled Rust readers, and no two of them lex the same language

Raised by **#788** (S92) while closing that finding's two members. Every
guard below reads `.rs` source and asserts a count or a presence over
raw text, with no comment awareness or with line-leading-`//` awareness
only. **The direction that matters is the silent one**: a real site
commented out leaves its text in the file, so the count does not move
and the guard stays green over exactly the change it exists to catch.
For several the loud direction is live too — a doc comment mentioning
the string reds the guard, which is F3's already-realised
cry-wolf-then-allowlist outcome.

**Twelve is a floor, and the count's own history is the evidence for
that — read it before you read the number.** It went **7 → 9 → 11 → 12 in a
single session**: 7 from the lane's own six patterns, 9 when
`include_str!` was added as a seventh spelling, 11 when that seventh
sweep was re-run. **Twice under the lane, once under its review**, and
every step was a differently-*shaped* sweep rather than a deeper one —
not one of the three found a member by looking harder in the places
already searched. Then it moved a **fourth** time, and not by
searching: while #788 was in review, lane **G-g landed a fifth reader**
— a second `code_only` in `topo/src/fixtures.rs`, serving
`face_normal`'s re-fork guard, its own docs conceding it does not model
raw strings (the `br"x\"` hole #788 had just fixed in the other one).
The two met in #788's merge. **A member arrived while the row was being
written, in the same crate, by a lane that could not reach the reader
that already existed** — which is the row's thesis, not a
counterexample to it.

**A number that has moved four times, once by growing rather than by
being looked for, is better evidence of the population's shape than any
single value of it** — which is why this row closes on a rule rather
than on a list: a taker who works the twelve and stops has done the
smaller half.

Neither sweep covered `scripts/`, `tools/`, `demos/`, guards over
non-`.rs` artefacts, or a source read through a runtime-built path not
rooted at `CARGO_MANIFEST_DIR`. **An eighth spelling is the expected
case, not the surprising one.**

**The sharper framing, which the first two counts missed.** This is not
only *"guards blind to comments"*. **Every guard that reads Rust source
in this workspace rolls its own reader, at whatever competence its
author needed that day** — there were four: `topo`'s two (a delimiter
matcher and a blanker, now one), `pncad/tests/all.rs`'s
`code_without_comments`, and a family of ad-hoc `find(…)`/`\n}\n`
slicers. `code_without_comments` is the tell: it is line-based, knows
`//` and naive strings only, and its own header explains that a
character literal *in its own source* would corrupt its string-state
tracking, so the author wrote the delimiters as code points rather than
fix the lexer. That is the same defect #788's review found one layer
under #788's own fix, worked around in a comment instead.

**The class needs two views, not one helper.** #788's
`source_walk::CodeOnly` blanks comments **and** literals, which is right
for a guard whose needle is a call and wrong for one whose needle is a
string literal or a doc heading. Sorted by what each member actually
greps for:

*Served by `CodeOnly` as it shipped — the needle is a code fragment (7):*

- `topo/src/review_d18.rs` — `body.matches("unreachable!").count() == 2`
  over `link_half_edges`, and `!body.contains("if let Some")`;
  commenting both `unreachable!`s out keeps the count at 2.
- `topo/src/face_normal.rs` — `Surface::Plane {` and `from_chart`; the
  *"the guard earns its line only if the string is reachable at all"*
  half is satisfied by a commented-out home spelling.
- `geom-core/tests/flagged_census.rs` — the ledger count of
  `k_stats::decide_flagged(` sites, no strip at all, both directions
  live.
- `step-import/tests/tier_gate.rs` — the *"exactly two validator call
  sites"* pin, line-leading `//` only, so a block-commented call still
  counts.
- `profile/tests/seal.rs` — `pub struct ProfileVertex<T: Real>` plus
  `serde` / `Serialize` / `Deserialize` over `include_str!`'d source.
- `geom-brep/tests/pcurve_conic.rs:399` — carves `pub fn route(` to the
  next `\n}\n` and forbids `_ =>`, `(_,`, `| _`. The carve is the same
  fragile slice #788's review found in `topo`, and a comment inside
  `route` naming a wildcard shape is a false red.
- `pncad/tests/all.rs:427,460` — `code_without_comments`, the fourth
  reader, feeding the facade guard. **Its file is Track E's open #763**
  — flagged here, not to be taken from under it.

*Need a comments-only variant — the needle contains a string literal,
which `CodeOnly` blanks (3):*

- `topo/src/sector_shape.rs` — searches for `"bool_arm"` **with its
  quotes**. Its own comment records that spelling the retired names
  inline *"briefly"* made it its own first counter-example, so the loud
  direction has already fired once here.
- `topo/src/chord_join.rs` — `decide("split_arc_window"` on a
  whitespace-stripped copy; commenting the real site out keeps the
  count at one.
- `topo/src/review_d18_probes.rs` — reads the message text inside
  `unreachable!(…)`, and skips line-leading `//` only.

*Arrived during #788's own review, and is the row's thesis (1):*

- `topo/src/fixtures.rs` — a **second** `code_only`, landed by lane G-g
  for `face_normal`'s re-fork guard while #788 was closing the first.
  Its docs concede raw strings are unmodelled — the `br"x\"`
  over-strip #788 had just fixed in `source_walk::CodeOnly`, re-forked
  because that one is `pub(crate)` and G-g could not reach it. **Two
  readers of different competence, one crate, one merge.**

*Needs the inverse view — its needle is prose (1):*

- `editor-core/tests/schema_ledger.rs` — asserts the raw text of
  `persist/mod.rs` contains `Version {n} is`, whose entries are **doc
  comments** (`persist/mod.rs:124`, `:133`). `CodeOnly` would blank
  exactly what it looks for. A commented-out ledger entry satisfies it,
  which is the silent direction; the guard's own docs already concede
  it *"reads for a heading, not for meaning"*.

**Why #788 did not sweep them.** Seven were reachable with the helper as
shipped and were left because they are outside F5's two files, and five
of the twelve are outside `topo` entirely, where `source_walk` cannot be
named (it is `pub(crate)` and `#[cfg(test)]`, so sharing it means a
test-support crate — which is this row's real question). The other four
need a helper that does not exist.
**This row closes on the helper shapes plus the twelve conversions**, and
`topo/src/{face_normal,chord_join}.rs` are Track G's **G8/G9** — a taker
must sequence with them.

**Update, #872 (Track I / I-c): the test-support crate this row names as
its real question now EXISTS.** Ruling **I-R8** put S64's ε inventory on
a computed pin, and the justification for *not* computing it had cited
`topo`'s `code_only` being out of reach — true, and **the removable half
of the obstacle**: the row's own answer, a test-support crate, was
already in the tree as `crates/test-utils`, a zero-dependency leaf that
`topo` and `mesh` both already dev-depend on. #872 added
`test_utils::source` with `code_only`, `mentions_raw_string`, and
`rust_sources` — the recursive traversal, added because sharing the
*predicate* and re-forking the *walk* reproduced exactly the defect the
sharing was for (a flat `read_dir` left a subdirectory invisible and the
pin green).

**Three corrections to what an earlier draft of this paragraph claimed,
because they change what the collapse means:**

1. **The full-blanker population is FOUR, not three.** `topo` has
   **two** live blankers — `source_walk.rs`'s `CodeOnly` and
   `fixtures.rs`'s `code_only` — plus `pncad`'s `code_without_comments`
   (which this row's own body classes as a different shape), plus the
   shared one. #872 added the fourth full blanker; it did not reduce the
   count.
2. **The port is of the WEAKER of `topo`'s two.**
   `source_walk::CodeOnly` models raw strings via `raw_string_open`;
   `fixtures::code_only` does not, and that is what was ported. So
   *"the collapse is a deletion"* holds **only for
   `fixtures::code_only`**. Collapsing `CodeOnly` onto the shared one
   as it stands would be a **downgrade**, and this row names `CodeOnly`
   as the shape serving seven of the twelve members. **Porting
   `raw_string_open` into `test_utils::source` is a prerequisite of the
   collapse, not an optional extra.**
3. **`topo::fixtures::code_only`'s collapse is UNOWNED**, and saying it
   was *"G8/G9's to sequence"* was a §C3 — a deferral pointed at a
   register that does not execute. **G8 landed as #834**, and **G9's
   scope is `topo/src/boolean/{ops,reduce}.rs` plus `chord_join.rs`**,
   which contains neither `fixtures.rs` nor `face_normal.rs`. Recorded
   here as unowned in the same words S230 and S231 use: **it needs a
   lane and does not have one.**

The port's faithfulness is established rather than asserted: 500,000
adversarial inputs across `/ * " ' \ # r`, `//!`, `///`, `'a`, `'é'`,
`b"x"`, `r"x"`, `\"`, newlines and multibyte, **zero mismatches**
against `fixtures::code_only`; the only body difference is
`i + 1; k += 1` → `i + 2`.

**One member of the class was fixed on the way, and it is D61's third
occurrence.** `mentions_raw_string` originally missed `br"…"` and
`cr"…"` — the prefix byte satisfied its non-identifier-predecessor
guard — which is the exact `br"x\"` spelling **D61 records #788 fixing
in `CodeOnly` and G-g re-introducing in `fixtures::code_only`**. Third
time, in the function whose whole job is to tell a caller its tree is
free of that construct. Fixed in #872 with a row per prefix; it could
not have bitten `mesh/src`, which is the argument for fixing it before
there is a caller who trusts it, not against.

**What is still open**: the **second** helper shape (the needle that IS
a comment or a literal — the four members `CodeOnly` cannot serve),
the twelve conversions, the raw-string port above, and the deletion of
the private copies. A taker starts by moving, not by writing.

**Verdict:**
## S126. The silent whole-row stand-down has a population, and it is 13 in three files

Found by lane F-d while sweeping F4's class. The shape is
`let … else { return; }` in the first statements of a `#[test]`: the
fixture the row needs is unavailable, the row returns, and the battery
log records nothing at all. The whole test is the skip, so there is no
partial coverage to floor.

The population, from `crates/*/tests/**/*.rs` (excluding `all.rs`):
**13 sites in 3 files** —
`sweep/tests/m8_4_intersection_iso.rs` (11),
`sweep/tests/review_probes_m8_4.rs` (1),
`topo/tests/m5_pr7_split_meter.rs` (1). None is announced.

**Why this is smaller than it looks, and the sentence is the finding.**
All thirteen stand down on an ε-conditional fixture, and the helper that
returns `None` *asserts its own classification* first —
`m8_4_intersection_iso.rs`'s `seam_at_eps` asserts `eps >= 1e-9` on the
`Ok` arm and `eps < 1e-9` plus the refusal's own measured sup on the
`Err` arm, so a mis-classified ε reds inside the helper rather than
skipping. What is missing is only that the *caller* announces nothing,
so the battery log cannot distinguish "this ε ran the row" from "this ε
did not". That is the D45 shape — coverage nobody can read off the run
— not the S84 shape.

**Not proposed here: floors.** C21 rules this class un-rolled-up because
a floor concedes the skip and *whether the row should be ε-conditional
at all* comes first. This finding contributes the hit list that question
needs, and nothing else. Scheduled as **D70**.

**What the sweep could not match.** It keys on `let`-else with a literal
`return;` in the following four lines, so it misses: a stand-down
written as `if x.is_none() { return; }` or `match … { None => return, …
}`; one spelled as an early `return` inside a helper the test calls; any
whole-binary skip behind `#[cfg(feature = …)]` (a different, already
loud idiom); and the 116 `let`-else sites whose body is `continue`, which
this sweep deliberately excludes. Those are a **selector** over a
heterogeneous collection — *"only the certified circle edges"* — rather
than a stand-down, in every one of the ~20 I read; the distinction is
not one a regex can draw, and I read a sample, not all 116. If any of
them is in fact a stand-down, it is inside this blind spot.

**So 13 is a FLOOR, not an enumeration** — the same qualifier S117 owed.
Every miss above adds to it and none subtracts, and a reader taking 13 as
*the* population would be taking a regex's reach for a fact about the
tree.

**A differently-shaped sweep finds a different class, and it is not this
one.** F4's style review swept for *a counter that is incremented,
printed, and never floored* and found **23 sites in 13 files**, including
`geom-core/tests/spline_hull.rs` — in the crate S78 cited as where *"the
discipline exists elsewhere in the tree"*. That is the S78 shape rather
than the S84 one, its members are live rows rather than skipped ones, and
it is the reviewer's to place; recorded here only so the next reader does
not mistake this row's 13 for the whole of *"guards that count and never
floor"*.

## S119. `k-lint` scores an unreadable margin CLEAN, and the argument against that is already written at the site

Found by lane F-b while closing S73's part one, which is this shape in
the sibling instrument.

`tools/k-lint/src/lib.rs`'s `lint_csv` parses `margin`, `band_zero` and
`band_escalate` with `f64::parse` and checks nothing further, so
`"NaN"`, `"inf"` and a negative all parse. Every rule in `lint_sample`
is an `m < threshold` or `m > threshold` comparison, so a `NaN` margin
makes all of them false: the row scores **clean** and still counts in
`Scan::scanned`. A `NaN` `band_zero` is worse — `band_zero >=
AMBIENT_BAND_MIN` is false, so the row falls through the `_ => {}` arm
and answers to no rule at all, while the CLI still reports it as
scanned. **A sweep the lint cannot read is not a corpus that got
safer.**

The argument is already at that site, for the column next door: the
`outcome` string is checked against a five-name allow-list because
*"scoring it silently CLEAN would let a sweep-format drift disarm the
whole lint (review MIN-2)"*. The three floats beside it get no such
check.

**Not currently wrong, and that is the qualifier the finding owes**:
the three committed sweeps under `docs/k-report-data/` carry no `NaN`
or `inf`, and the margins are recorded kernel decisions rather than
arithmetic that obviously produces one. The finding is that the
instrument's answer to an unreadable measurement is its own pass value,
and nothing upstream of the parser promises otherwise — which is
exactly what S73's part one was, one tool over.

**Fix shape, if taken:** the per-column admissibility S73's fix landed
in `tess-lint` (`margin` finite; the two bands finite and
non-negative), refused in the harness voice `lint_csv` already owns,
with the same two guards — a policy table checked against the header,
and a row that reds when a policy is loosened rather than only when one
fixture breaks. **Not taken by F-b**: `tools/k-lint/` is outside F6's
scope.

## S120. What the tessellation and K instruments still cannot see, after F6 (roll-up)

Recorded by lane F-b out of #783's style review. Four members, one
subject: **readings and constants in `tools/` that nothing re-derives**.
None is in F6's scope; each is a live gap rather than a residue, which
is why they are scheduled (**D64**) rather than left in a PR body.

- **(a) A fallback inside a COMPARISON has two sides, and a direction
  argument taken on one of them is not an argument.** This is the
  mechanism, and it is what makes the next instance findable: S73's
  whole method is *"which way does a broken reading push the
  verdict"*, and every instance of it so far has been checked on the
  reading being taken. A differential gate reads TWO — `now` and
  `was` — and a fallback that is safe in one is the opposite in the
  other. **A disposition that names one side has answered half the
  question, and reads as if it answered all of it.**

  The two sites: `tess_meter::divisions` answers `1` on a non-finite
  step, and `best_split_steps` answers `INFINITY` on a non-positive
  `q`. #783's sweep dispositioned both as *"the opposite direction —
  a broken reading shrinks the denominator, so the gate is MORE
  likely to fire"*, which is true of the **fresh** row only: the rule
  is `now > was · GROWTH_TOLERANCE`, so the same fallback in the
  **committed baseline** inflates `was` and hides a real regression.
  The lane found this in its own stated negative result, on being
  asked to re-derive it. And `divisions`' fallback value is `1.0` —
  the exact floor F6's parse guard now refuses in `tess-lint`,
  tolerated one crate upstream where the number is produced.

  **Not a one-line edit.** The doc's argument for answering one on a
  genuinely unconstrained direction is sound *for a counterfactual
  column*, so what is owed is a decision about which columns may carry
  a fallback at all — and, with it, a re-check of every other
  disposition in this document that reasoned about a gate's direction
  from a single side.
- **(b) The CSV already distinguishes the two `NaN`s and the parser
  discards the column that does it.** `worst_dev` is `NaN` both when
  the sweep did not resample and when it resampled and a sample came
  back `NaN` — `crates/mesh/src/trimmed.rs`'s
  `if dev_samples == 0 || d.is_nan() || …` deliberately lets the second
  win. `dev_samples` is the discriminator, it is column 21, and
  `tess_lint::parse` never reads it, so genuine deviation drift parses
  as *"not resampled"*: a skip. F6 put that absence in the type; it did
  not make the two states distinguishable, and the CSV already can.
- **(c) `SPLIT_SCAN_DECADES` / `SPLIT_SCAN_SAMPLES` have neither a
  guard nor a register, and the guard is impossible while the register
  is merely absent.** The cell-count excess these constants produce is
  **discontinuous in the constants** (~4 points between adjacent
  sample counts — S73's record, with `323` as the witness), so a
  tolerance on it cannot be written; #783 records that as Q6's written
  reason at the claim site. The register is the part still open, and
  the part CI cannot supply as it stands: a real `--sizing-only` sweep
  is linted on every merge, and that gate **structurally cannot** see a
  degraded scan, because a worse scan RAISES `span_opt_cells`, which
  LOWERS the recoverable slack, and the gate fires only on growth. So
  the constants are watched by one divisions pin and nothing else.
  **What a register could measure is not the cell count**: the same
  excess without the two `ceil`s is continuous, falls smoothly with
  resolution, and depends only on the sampling step and the range —
  i.e. on exactly what these constants set. That quantity, and the
  guard on it, are **S160 / D105**, placed with their evidence; this
  bullet is the register half only. Whoever takes it owns `ci.yml` and
  should decide between a scheduled re-measure of S160's quantity and
  an explicit "unwatched, and here is why" — **and should take D105
  first or together**, since a register over a quantity nothing
  computes yet is not a register.
- **(d) `k-lint`'s other three constants are unpinned, and one sweep
  shape is unswept.** `PROXIMITY_FACTOR`, `EPS_COUPLED_FLOOR_RATIO` and
  `AMBIENT_BAND_MIN` were disclosed by #783 as outside its sweep, and
  **S119 covers `lint_csv`'s float admission, not these**. Also
  unswept, and disclosed the same way: fallbacks written as an
  early-exit (`if !x.is_finite() { return … }`) rather than as an
  `else` arm, and the whole of `scripts/gates/*.sh`, which #783's
  `--include=*.rs` excluded. A disclosed blind spot is a work order,
  which is what this bullet is.

## S161. Two of `review_d18`'s seven hammered operators never reach the arms under attack

Found by lane F-d while replacing S76's floor, and by F4's adversarial
review one step further.

`crates/topo/src/review_d18.rs` says its sweep *"drives every operator
that reaches `link_half_edges` over every key"*, at the module doc and
again on `hammer`. The per-operator exposure the S76 fix added measures
that sentence for the first time, and it is **true of the calls and false
of the arms**:

- **`kemr` enters a mutation phase in NEITHER row** — 0 of 96 calls on
  the spent-graft destination, 0 of ~1900 across the torn sweep. Its
  plan phase refuses on every input either row produces, so every
  `link_half_edges` site below it is attacked by nothing here.
- **`mfkrh_plug` cannot reach them at all.** `mfkrh`
  (`euler_kill.rs:1037-1118`) mints a face surface, adds a face and
  touches `face`, `loop` and `shell` records; it calls
  `link_half_edges` nowhere. It is in the sweep because it is an euler
  operator, not because it can reach a row-4 arm.

**Two different defects wearing one sentence.** `mfkrh_plug` is a
*classification* error — the file counts an operator that is not in the
class its prose names, and the S76 fix therefore excludes it from the
floor (`LINK_OPS`) while still driving and printing it. `kemr` is a
*coverage* gap: it is in the class, and nothing this file does gets it
past its plan phase. Only the first is closed by #825.

**What D107 owes.** Either a fixture on which `kemr`'s plan phase
succeeds — its preconditions are a ring-merge shape neither `ops_cube()`
nor a torn cube presents — or the written finding that `kemr`'s row-4
arms have no input witness from this door, which is a claim of the same
kind the D2 addendum's row 1 already makes elsewhere and would be worth
as much. What is not admissible is the current state, where the prose
asserts the coverage and the exposure line disproves it on every run.

The exposure prints `kemr: 0` before the floors, so this is derivable
from any `--nocapture` run of the row rather than only by reading
`euler_ring.rs`. It is **not** on the CI board: libtest captures a
passing test's stdout and the release-profile job passes no
`--nocapture`. That is a `ci.yml` question and `ci.yml` is Track F's F8
and Track G's G-a, so it is named here rather than changed.

---

## S127. The local gate has no mirror of `oracle-certify`, and nothing enforces ci.yml ↔ ci-local.sh job parity

**Found by #786's style review, in the sentence #786 was rewriting.**
`local-scripts/ci-local.sh:432-438` carried both of the claims S72
convicted in `ci.yml` (*"the row that catches a dropped outward round"*,
*"stays a by-hand gate"*) — doc rot, and fixed there by #786. The
**code** drift underneath it is not fixed: `ci-local.sh` mirrors
`ci.yml`'s cheap `interval-backend` row and has **no `oracle-certify`
row at all**.

That matters because `local-scripts/gate.sh` documents itself as the
merge gate when hosted Actions is unavailable. On the hosted pipeline a
dropped transcendental pad is caught (by `oracle-certify`, keyed on
`ci-filter.py`'s `ORACLE_PATHS`); under the local gate it is not caught
by anything, and the same is true of a dropped `+`/`−` pad.
`scripts/gates/gate-roster.sh` enforces gate-SCRIPT parity, which is a
different set — **nothing enforces JOB parity between the two files**,
so this is a class, not an instance: any hosted job with no local mirror
is invisible the same way.

Two decisions, and neither is #786's to take: whether the local gate
should carry a ~250s GMP+MPFR build, and whether job parity should be
mechanically enforced or explicitly declared per job. #786 records the
gap at both sites (`ci-local.sh`'s comment and the crate README) rather
than closing it.

**Verdict:** ACCEPTED, unstaffed. §D row **D71**.

## S129. `demos/` has assertions and no runner (raised by #787)

**Raised by Track G's G2 lane while establishing what CI actually does
with `demos/`, which S110(j) required knowing.** Searched
`.github/workflows/{ci,render}.yml`, `local-scripts/ci-local.sh` and
`scripts/`: **nothing ran `cargo test` anywhere under `demos/`.**

- `ci.yml:1693,1695` run `cargo fmt --check && cargo clippy
  --all-targets -- -D warnings` in `demos/tour` and `demos/wild`.
  `--all-targets` **type-checks** the test targets and executes none of
  them — S110(a)'s shape (`probe_s5_sectors.rs`), one tree over.
- `render.yml:334` and `ci-local.sh:258` run `cargo run --release`, the
  binary's scene path only.
- `scripts/k_probe_sweep.sh` and `scripts/tess_budget_sweep.sh` run the
  two feature-gated modes.

Ten assertions were therefore unguarded: `tests/eps_regression.rs`'s
three (the #99 ε pin, whose whole purpose is gating) and
`lily.rs::review_probes`' seven. **Two rows of the seventh have been
red for an unknown period** — `finding_13_tessellation_table_reproduces`
pins seven `(body, δ, triangle_count)` rows and the two SWEPT blades
now tessellate to 1016/854 against a pinned 976/826 while all five
analytic rows are exact. That is a kernel-behaviour question, so it is
**issue #782**, not a row here.

**PARTLY FIXED by #787**: the ε pin is armed in `k-lint` (`cargo test
--release --test eps_regression`), mirrored in `ci-local.sh`. **It is
not free** — the row builds `demo-tour` in release at DEFAULT features
while the tess-budget sweep below builds the same crate at `--features
budget`, a different unification, so the job now pays two release
builds of the kernel where it paid one; the runs themselves are ~7 s.
The comment at the row carries that, and rejects the obvious saving
(folding the pin into `--features budget` would arm the per-face meter
inside the run the pin is about). **The `--bin demo-tour` unit tests
are deliberately NOT armed** — arming them today lands a red gate — and
what stays unrun is load-bearing: those seven include
`the_spine_curl_wall_re_measured`, which pins a typed
`LoftError::ReversedStacking` frontier from both sides, so `lily.rs`
keeps its frontier pins in two spellings of which only the
tour-runtime one (`wall_probes`) executes. Widening the row is owed the
moment **#782** is decided; until then this row stays open.

## S130. `demos/tour/src/lily.rs` — the first end-to-end read (roll-up, raised by #787)

**Raised by #787's style review**, which took the file as free ground: the
second scan's coverage record named `lily.rs` as its highest-yield uncovered
file — sampled, never read end to end — and nothing had read it since. 2,446 lines, 975 comment / 1,393 code
(41%), with a 137-line module header before the first item — the
tour's most geometrically involved scene and the one written most like
a user modelling a real object, which is what makes it worth reading.

**Nothing here is fixed.** #787's scope was `demos/`'s nine G2 members,
and one item — `bud`/`sepals` returning `Vec<Body<S>>` where a naming
`.zip` truncates silently — was pulled forward into it as a sweep of
that PR's own governing move (`WILD_CELLS: [Cell; 8]`); both now return
`[Body<S>; 3]`. The rest are recorded:

- (a) **`:1104-1113` is an orphaned comment block whose live number is
  wrong.** Ten lines about the globe centre and the sepal polar angle
  sit between `};` and `// The BUD:` with no code between them and no
  blank line, saying *"38 degrees puts them on the shoulder of the
  globe"* — while the live `sepals(…)` call passes
  `(FLOWER_TOP / FLOWER_GLOBE).acos() + deg(4.0)` = **28.6°**, with its
  own restatement of the same argument. S112's class, in the file the
  scan flagged as least read.
- (b) **A shadow vector algebra beside the kernel's own.** `nrm` is
  byte-identical at `:395-397` and `:972-974`; `rot` at `:162`; the
  radial-frame builders at `:387` and `:932` — all in bare
  `(f64, f64, f64)` tuples, converting to `Vec3`/`Point3` only at the
  API boundary through `v3`/`pt3`, while `mod review_probes` in the
  same file uses `Vec3::norm`/`cross` freely. The duplication is this
  bullet; **the reason is a library finding and is issue #796** — if
  authoring a plant naturally means not using `Vec3`, that is evidence
  about `Vec3`'s ergonomics (`memories/demo-purpose.md`).
- (c) **Two near-parallel "extract the single stored carrier" helpers
  with different rigor**, inside one `mod review_probes`: `:1687`
  `torus()` scans all faces and cross-checks that multiple torus faces
  share one carrier; `:2300` `sphere_of()` documents itself as *"The
  single stored sphere of a body"* and returns the **first** one with
  no such check. And `torus()`'s own agreement check is partly
  vacuous — it compares `center.x`/`center.z` but never `center.y`,
  compares `axis.y.abs()` only (so two tori whose axes differ in x/z
  pass), and ignores `u_ref` — while the surrounding prose claims the
  stored parameters come back bit-for-bit.
- (d) **`assert_cap` is an existential over two, and gets easier as the
  caps converge.** `:1755-1770` is
  `[caps.start, caps.end].into_iter().any(…)`, so a body whose two
  joint frames collapsed onto one plane satisfies every call. The
  comment defends the disjunction (*"which of the two ends answers is
  the revolve's business"*), which is reasonable; the check as written
  cannot see the degeneracy.
- (e) **`cap_frames` asserts `on.len() == 8` for every planar face it
  meets** (`:2325-2372`), then returns a `Vec<Cap>` with no arity check
  at all. For a body with a third planar face, or two nearly-coplanar
  caps on a shallow blade, the vertex set is wrong before the assert
  fires.
- (f) **41% comment, 137-line header, accumulated one titled essay at a
  time.** S116(e)'s class, one crate over.

**Reading (b)–(e) together**: this is the file `mod review_probes`
lives in, and those probes are exactly the seven `#[test]`s no gate
runs (**S129**, **#782**). A rigor gradient inside an unrun suite is
worth less attention than the same gradient inside a running one — but
it is also why nobody found it.

## S134. What is still one-directional in the interval backend, after #786 made most of it two-sided

#786 gave the pads an upper constraint in both tiers. Three things it
did not close, gathered so that they are a register entry rather than
four sentences in a PR body:

- **`powi`'s tightness ceiling is a deferral, not an unguardable.**
  `certify.rs` passes `None` because the steps `powi` is entitled to are
  a function of its exponent rather than a constant — which argues
  against a *constant* ceiling and concedes that an exponent-dependent
  one is derivable. Something downstream computes with that width:
  `crates/geom-core/tests/review_m0_pr4.rs`'s
  `powi_f64_lane_is_contained_by_the_padded_enclosure` pins the kernel's
  f64 lane inside this enclosure. Measured worst ratio moves with the
  seed (117 at effort 1, 122 at effort 2, |n| ≤ 31), which is exactly
  why fitting a constant would be the wrong answer.
- **The oracle tier's upper constraint is a RATIO, and a ratio is
  scale-free.** `pad_contract.rs` now covers wide boxes for the monotone
  operations, but a fixed absolute over-widening on a non-monotone shape
  with a large oracle width still moves no ratio and matches no fixture.
  A per-endpoint oracle-relative bound is the obvious instrument and
  #786 declined it for a stated reason — extremum capture, huge-argument
  degradation, pole and branch-cut refusals all make it fire on sound
  output — so what is owed is a bound that excludes those paths from the
  INPUT rather than from the output.
- **S116(r)'s consumer-side half.** The caveat that `intersection`
  returns `Trv` on every input is now at `lib.rs`, in the backend.
  `crates/geom-core/src/interval.rs:135-143` is the other place a
  consumer meets it and does not carry it; that file is outside the
  backend's workspace and was outside #786's fence.

Not on this list, deliberately: `copysign`'s placement inside a consumer
(`crates/geom-core/src/interval.rs:356`). That is **S1**'s, it is Tier 1,
and S112(c) exists to hand it the fact rather than to decide it.

**Verdict:** ACCEPTED, unstaffed. §D row **D78**.

---

## S131. A unification declares "one home" while self-declared hand-copies of the same rule stand inside its own scope

**[verified, #781's fix pass]** S74 unified the two swept-traversal
builders and `swept.rs` then claimed `swept_segments` was *"the one home
of the profile crate's reversal involution."* It was not.
`revolve/tube.rs:216` builds two `SweptSeg` values by hand under a
comment that **says so**: *"the same transform `swept_segments` applies
to a canonical CCW loop: endpoints swapped, bulge −1, turn Negative."*
A self-declared copy of the exact rule being unified, in a file §D
listed inside the unit's own scope.

**This is the class, and it has three members in these files alone.**
Run the marker vocabulary over the **working tree** rather than over
history and `crates/sweep/src` returns, outside `fillet/`:

- **`revolve/tube.rs:216`** — the reversal involution, hand-applied.
- **`revolve/tube.rs:169`** — *"mirrors revolve's angle
  classification"*: `revolve/mod.rs:640-648` decides span and headroom
  under `revolve_angle` / `revolve_angle_headroom`, `tube.rs:174-179`
  decides the same pair under `tube_window_span` /
  `tube_window_headroom`. Live and true.
- **`loft.rs:400`** — *"extrude's phase verbatim, loft rim specs"*:
  `extrude.rs:537` is the same `mev_line` → `kemr` → `mev` hole phase,
  differing only in the rim specs. Live and true.

**All three are honest markers and all three stay.** The tube ones
cannot share code: that door exists to store the caller's centre and
radii bit-exactly rather than reconstruct them from bulges, so it cannot
hand the shared builder a `ValidatedLoop` at all. `loft.rs:400` is a
real candidate for sharing and nobody has costed it.

**The defect is not the copies; it is that a unification asserted "one
home" without looking for them**, and that both of the unit's sweeps
were structurally blind to them — Sweep 1 grepped the *not-sharing*
vocabulary, Sweep 2 grepped the *marker* vocabulary but over `git log`,
never over the tree. **The cheap generalisation: any claim of the form
"X now has one home" is owed a marker-vocabulary grep of the tree in
the scope it claims**, and that is one command.

**Two measurements on the vocabulary itself**, from running it over
`crates/sweep/src` (minus `fillet/`) at `6a479c94` — 13 hits:

- **It is mostly false positives here: 9 or 10 of the 13.** *"verbatim"*
  overwhelmingly modifies a **value** carried through unchanged — a
  carrier, a parameter interval, the caller's centre and radii
  (`extrude.rs:47,983`, `upgrade.rs:8,199`, `tube.rs:190`) — not a body
  being a copy. *"re-derived"* twice cites Mäntylä, a book
  (`lib.rs:8`, `extrude.rs:4`). Only 3 clearly declare duplicated code
  (`tube.rs:169`, `loft.rs:400`, `skin.rs:302`, the last cross-crate and
  already recorded at S6), plus `skin.rs:1121` arguably. **Every hit has
  to be read**; the grep is a candidate list, never a count.
- **It cannot see a marker written in fresh words.** `tube.rs:216`'s
  marker existed and was honest, and the vocabulary missed it because it
  said *"the same transform `swept_segments` applies"* — none of the
  eleven phrases. #781's first fix pass then rewrote it as *"a
  HAND-APPLICATION of"*, which was **still invisible to the grep**. It
  now reads *"a hand-written COPY OF"* and says at the site why the
  wording is not free. **A duplication declared in words the tree's own
  greps do not carry is a duplication nothing will find** — which is
  C11's observation with the mechanism named, and it is the argument for
  a fixed vocabulary being written down somewhere a marker's author will
  read.

`swept.rs`'s sentence is corrected to state the qualifier that makes it
true (*from a validated loop*) and to name `tube.rs` as the exception.
Recorded because the *shape* — a consolidation's headline claim
falsified by a marker already in the tree — is not specific to `sweep`.

**Verdict:**

## S132. `strut_spec` — one name, two different rules, in one crate

**[verified]** `extrude.rs:384` and `revolve/surfaces.rs:75` are both
`strut_spec`. They share nothing else: different arity (five parameters
against six), different description (`MappedCurve::ExtrudedPoint`
against `RevolvedPoint`), different carrier (`Curve3::Line` against
`Curve3::Circle`) and different parameter intervals (`0..w_norm` against
`0..|θ|`).

S6 saw it, called it *"a name collision"*, and correctly left it
unified-nothing. S74's class re-check saw it again, confirmed the
non-unification claim was sound, and **put it in a lane report**. That
is the inverse of S4's shape — one vocabulary, N implementations — and
it is the project's standing failure in miniature: **an instance
recorded outside the register is not recorded.** The register is here.

Not obviously a defect: two verbs each naming their own strut spec after
the thing it specifies is defensible. What is not defensible is that a
reader who greps `strut_spec` gets two functions with incompatible
contracts and nothing at either site saying the other exists. Minimum
close: a marker at each. Fuller close: name them for what differs.

**Verdict:**

## S133. The consolidation-deletes-its-own-evidence sweep was run over one crate, and the mechanism is not crate-specific

**[disclosed, #781]** S74's mechanism is *a consolidation pass removed
the prose that was its own evidence and left a positive claim behind
that is now false.* The class re-check that closed it swept
`crates/sweep/` — honestly scoped and said so — but #710 was one of
several consolidation passes this milestone, and the mechanism has
nothing to do with `sweep`.

**Why the disclosure could not be discharged in place:** run
workspace-wide, `git log -S` cannot distinguish a marker being *added*
from one being *deleted*, so the phrase families return dozens of
commits that are mostly additions. Separating them needs a per-commit
read, which is a lane, not a paragraph.

**Where to look, and who owns it.** `topo/src/chord_join.rs` and
`profile/src/path/` are the other trees that took consolidation passes
this milestone. Both are already staffed — `chord_join.rs` is G-f's
(G8) and G-g's (G9), `profile/` was G-d's (G5) — so this is **not a new
lane**: it is an obligation those lanes can discharge cheaply while they
are in the file, by running the marker vocabulary over the tree in their
scope and dispositioning what it returns. The one-command version is in
**S131**.

**The `profile/` half is discharged, by #831 (G5).** Vocabulary run: the
eleven phrases of the reviewer brief's Q2 pattern minus the D9 `bit-identical`
family, plus `verbatim`, `re-derived`, `ported from`, `mirror of`, `one
dimension down`, `the twin of`, `hand-written copy`, `hand-applic`,
`duplicated from` — **but not `restated`**, which the standing prompt carries
and this run dropped (it matches one line in `review_s2.rs` — the MAJOR-1
regression pin's *"restated at the lattice door"* — checked, not a
duplication). A sweep's vocabulary is part of its result. Run over
`profile/tests/review_s2.rs` and `profile/src/sugar.rs` (that lane's scope):
**9 hits, 0 duplications** — six *"re-derived"*s are the oracle declaring it
is written from the geometry rather than from `src`, which is the file's
purpose; one *"verbatim"* modifies mined coordinates; and `sugar.rs`'s two *"extracted verbatim"* headers
both say *"extracted verbatim so … share one code path"*, i.e. they declare a
unification, not a copy. Run again over `profile/src/path/` and `path.rs`,
which is the tree this row actually names and is **outside** that lane's scope
cell: **14 hits, three candidates**, all left for whoever owns those files —
`path.rs:1668` (*"this is `fillet_corner`'s emission verbatim"*, a
self-declared duplicate emission sequence, though its own sentence is D9
bit-identity vocabulary and so partly fenced), `family.rs:826` (*"the sharp
`Via` leg mode's own derivation, verbatim"*), and `program.rs:1875` (*"the
dynamic mirror of the typestate lattice"*, a duplication by design and the
biggest of the three). **The vocabulary's blind spot from S131 holds here
too**, and in this instance it did real damage in the file the sweep was run
over: `review_s2.rs`'s two enclosing pins carried the SAME `Ok`-branch body —
`fillet_segment` → `circle_from_bulge` → per-leg `d + radius > r - 1e-9` →
the same panic with the same literal — plus a third copy of that arithmetic,
inverted, in `check_corner`'s enclosing arm. Three copies, **none of them
declaring the other**, so no marker vocabulary could have found them; #831's
style review found them by reading, and its fix pass gave them two homes —
`assert_swallows_nothing` for the arithmetic (which `check_corner` shares) and
`report_moved_refuse_pin` for the whole `Ok` arm (which only the two pins
share) — with each pin's doc now naming the other. That is the argument for
the reading, not for a longer word list.

**Verdict:**

## S128. A pin's building bands round the twin crossing, so the conditioning it was mined for is never exercised there

**[verified, #831]** `crates/profile/tests/review_s2.rs`'s
`an_uncertifiable_tangent_point_refuses_instead_of_being_returned` is ε-keyed:
at ε = 1e-12 it must refuse typed, at 1e-9 and 1e-6 it must build with its
tangent point on the outgoing carrier at the ulp floor — *"because the
construction no longer has any 1/ρ amplification left to spend"*.

**On the building bands there is no amplification, because it is not this
corner's fillet.** Found by making `check_corner`'s doc claim true instead of
narrowing it — the doc said every hand-built row runs the battery, one did not,
and adding the call turned the row red with a ρ-predictor residual of
`1.134 = 2r`. The pair's carriers sit within **8.61e-5** of external tangency
(`|O₁O₂| = 2.2664641` against `R₁+R₂ = 2.2665502`) — **not the 3e-4 the row
asserts, which bounds ρ and is a different quantity** — so the crossings are 1.7115e-2
apart, `mirror_excluded` is **false**, and the ladder rounds the twin: the
returned fillet has `|P−O| = R−r` on the incoming leg and `R+r` on the
outgoing, the opposite offset sign on both from the ρ the row re-derives. The
8-ulp assertion holds for any fillet tangent to that circle, so it was never
evidence about the collapsed lever. The refusing band is sound — there the
typed error carries `offset_radius` and the row checks it against ρ.

**Not a kernel defect**: the file's own fuzz *skips* draws whose mirror
survives the gates, for exactly this reason. It is a fixture that bypassed that
skip. #831 makes the twin fact an assertion rather than a description, so the
row goes red if the ladder ever returns the drawn corner's candidate; **`D72`
schedules the re-mine** of a corner whose twin is excluded, which is what the
building band needs to mean what its prose says.

**The general shape, which is why this is recorded and not just fixed:** *a
fixture can pass on geometry that is not the geometry it was mined for, and a
one-sided residual check cannot tell.* The distinguishing instrument existed in
the same file the whole time and one row did not call it.

**Verdict:**

## S135. Every intra-doc link under `tests/` is inert — nothing renders them, nothing checks them, and nine are already broken

**[verified, #831]** S71's dangling link was invisible to
`scripts/doc-gate.sh` *"because it is in a `tests/` file"*.
`cargo doc` documents lib and bin targets, and the gate's own header says the
consequence in its own words — *"rustdoc builds no test targets"*
(`scripts/doc-gate.sh:71`), written there to justify `--all-features`. So an
intra-doc link in a `tests/` file is decoration with a promise attached, on
every tier, at every eps, forever.

### The census — **stated once, here, and reproducible**

Every other document that wants this number **points at this block** rather
than restating it, which is **S176(b)**'s cure applied to the finding that
raised it.

**At `cfdc1c6f`** (this branch's final merge of `origin/main`, at
`c0d90bc3`; unchanged from the previous merge `4cc0dbf3` — that merge brought
no test file carrying a link):
**465 test `.rs` files in the workspace, 106 of them carrying 294
bracket-link candidates on doc lines.** None of the 294 is ever resolved.

**The branch head carries 293, and saying so is the point.** This PR's last
edit de-linked one of its own — `review_s2.rs`'s
`an_enclosing_leg_forces_an_equally_enclosing_partner`, in a paragraph the diff
re-authored, where leaving a link while converting three siblings twenty lines
up would be the finding arguing with itself. So the census moves **with the PR
that states it**, not only with merges: that is why the reproducible
**definition** and a named **anchor** are the deliverable here and the integer
is not (**S176(b)**).

**What the sweep counts, exactly** — so that a re-derivation is a
re-derivation and not a second definition: files tracked by git whose path
ends in `.rs` and contains a `tests/` component; lines whose trimmed form
starts with `///` or `//!`; every `[target]` on such a line that is not
followed by `(`, whose target is identifier-shaped
(`Ident(::Ident)*`, optionally wrapped in backticks). **57 of them are
`#[attribute]` names written in prose** (`#[ignore]`, `#[test]`,
`#[non_exhaustive]`, `#[path]`, `#[cfg]`) — they are link candidates by the
same test, and they **resolve**, to the built-in attributes in the prelude;
21 of the same shape sit in `src/` doc lines with `doc-gate.sh` green over
them, which is how that was settled rather than assumed. They are inert here
like everything else in `tests/`, and none of the nine broken ones is one.

**Drift, in the record of the finding about drift.** The previous spelling of
this census — *"285 in 106 of 465"* — does not re-derive: the same sweep at
this branch's pre-merge head returns **285 in 105**, so the pair that replaced
*"277 in 105"* was itself half-drifted, mixing a re-swept candidate count with
a file count from a different run. The final merge then added **9 candidates
across 4 files** (`editor-core/tests/boolean_op_wire.rs`,
`editor-core/tests/m4_pr8_k_probe.rs`, `geom-brep/tests/m5_pr7_ssi.rs`,
`sweep/tests/review_d2_adv_probes.rs`); the five distinct new targets
(`EVERY_OPERATION`, `test_utils::vacuity`, `test_utils::vacuity::stood_down`,
`SsiDomain::floor_scale_for`, `census_corpus`) all resolve, member path
included, so **no new breakage**. This is the third time these numbers moved
under a merge, which is the argument for writing the definition down rather
than the count.

**Nine are broken today, and nine is a FLOOR rather than a count** — see the
blind spot below, which is the same shape as the defect being fixed: the check
resolves a link's ROOT and never its member path, so every `Type::method` link
whose `Type` is in scope was counted as resolving without `method` being looked
for at all. Resolved root-by-root against each file's own scope, three point at
names that exist nowhere and six name items that exist but are out of scope
where they are cited. **Cited by target rather than by line**, because a fix
renumbers its own file and the line then names something else (see **S176**).
The line numbers that were carried here against merge base `3ddd6011` are gone
with that second anchor: **this document states one anchor for #831's
measurements, `cfdc1c6f`**, and a citation that cannot be pinned to it is
written as a name.

**#831 fixed three of the nine** — `review_s2.rs`'s module-header link to
`enclosing_tangency_is_constructed_not_stumbled_upon` (S71's own, a name that
exists nowhere), its `Leg::tangent_point` link (a real `pub(crate)` item an
integration test can never resolve), and `geom-core/tests/review_m0_pr4.rs`'s
link to `powi_diverges_from_the_tight_enclosure` (renamed AND inverted by the
M5 backend swap). **The other six are dispositioned in its PR and untouched**,
each named by its target because none of them is this PR's to renumber and all
six re-derive by grep: `editor-core/tests/corpus/mod.rs`'s `Vocab` (never an
item — the fix turns on what the sentence means, in a crate that lane did not
read), `editor-core/tests/m9_d1_r1_probes.rs`'s `m4_pr3_names`,
`geom/tests/surfaces/span_window_pairing.rs`'s `SurfaceWindow` and `Span`,
`step-import/tests/freecad.rs`'s `PlacedInstance` and
`topo/tests/common/mod.rs`'s `BooleanDeclarations`.

**#831 also added four such links before removing them again**, which is the
finding arguing with itself inside one diff: the replacement prose and
`check_corner`'s doc were first written in the `[`item`]` form this finding
calls a promise that cannot be kept. They are plain backticks now, and the
two in-tree precedents for that spelling are `geom-core/src/interval.rs:62` and
`interval-transcendentals/src/lib.rs:114`. **The links that were already in
those files are left alone** — converting them is the policy call **D113**
holds, not a lane's to take mid-review.

**The disposition is a policy question, not a patch — and it takes a row,
`D113`.** Either the form stops being used in `tests/` (it promises a link that
cannot exist), or something is built that resolves it, which rustdoc cannot be
asked to do for a test target. Not taking a row was wrong: **D71** sits in the
same table as `ACCEPTED, unstaffed` and is exactly a decision with no patch, so
the channel exists and declining it was an amnesty rather than a scoping call.
The decision-holder is **Evan**, and the shape is already ruled once — S61's
*"a gate must be sited where it can fire on its own inputs"*; if the answer is
build something, the work belongs with Track F's instruments. **The sweep's own
blind spots are in #831's body**; the one that bounds the count is above, and
the others (doc lines only, identifier-shaped targets only, no `benches/` or
`examples/`) each only push the floor further down.

**Verdict:**

## S136. A band-keyed row's NAME asserts one arm, and it is the arm the shipped default does not take

**[verified, #831]** `crates/profile/tests/review_s2.rs`'s
`an_uncertifiable_tangent_point_refuses_instead_of_being_returned` (cited by
name, per **S176**) refuses on
exactly one of the three shipped bands: the ε-crossover it carries is
≈ 2.53e-10, so at 1e-12 it refuses and at 1e-9 (the default) and 1e-6 it
BUILDS. Its own doc says so in a section header — *"# This corner is now
BUILT, not refused, at ε = 1e-9"* — which is the file telling the reader that
the name is wrong rather than fixing it. **The sibling one screen down is the
shape it wants**: `a_collapsed_offset_lever_refuses_typed_at_every_band` names
its band scope, so a reader knows what the row claims without opening it.

**Not renamed by #831, deliberately.** `profile/src/sugar.rs`'s `LEVER_ULPS`
doc cites the row
BY NAME as `LEVER_ULPS`'s *"what goes red if this stops being true"* — the
issue-#667 Q6 pointer — so the rename is a two-file change into a file §D
fences to a re-read for that lane. Recorded rather than done, and the
orchestrator concurred.

**It is a class, with at least two more members**, found by grepping
`crates/*/tests` for rows that branch on the ambient band
(`Tolerance::get().eps`, `tol().eps`, `eps <`/`eps >=`) and reading each
enclosing name: `step-import/tests/recognize_pins.rs:281`'s
`the_integral_mixed_body_imports_first_class_with_a_charted_seam` and `:396`'s
`the_mixed_arc_prism_imports_first_class_over_the_intersection_pcurve_arm`
both assert a first-class import in the name and then assert `eps >= 1e-9`
inside the `Ok` arm — true on two bands of three. Checked and NOT members:
`r1_dm1_probe.rs:86`'s `dm1_no_longer_refuses_at_the_instancing_gate` (the
refusal panic is unconditional; only the frontier cell is band-keyed) and
`review_s2.rs`'s own `a_collapsed_offset_lever_refuses_typed_at_every_band`.

**What that grep could not match:** a row whose outcome is band-keyed but which
does not read ε explicitly (the branch hidden in a helper, or the row simply
not run at the other bands), and the judgement itself — *does this name assert
an arm* is a reading, not a match, so the negative results above are one
reader's.

**Verdict:**

## S176. A landing PR's own record drifts from the change it documents, in two mechanical ways (roll-up)

**[verified, #831 — found in that PR's own entries, by its style review]** The
*Recording convention* has each unit write its record in the landing PR. Two
things go wrong there by construction, and both did:

- **(a) `file:line` citations are correct when written and stale in the commit
  that writes them.** #831's S135 and S136 entries cited
  `review_s2.rs:45`, `:1347` and `:1266` — and the same diff renumbered that
  file, so `:1347` landed on a coordinate literal and `:1266` on a `let`. The
  fix in #831 is to **cite by target name** (names are stable and greppable;
  lines are not) and to say which commit any surviving line number is relative
  to. **Every `FIXED by #NNN` entry in this document has the same exposure**,
  because every one of them was written inside the diff it describes —
  recorded, not swept; S72's and S127's are the samples to check first if
  someone takes it. **It recurred inside the fix, twice**: this PR's own G10
  half moved `sugar.rs`'s lines by six and its merge of `origin/main` moved
  `geom-brep/src/ssi.rs`'s by fourteen, so three citations written earlier in
  the same PR — `sugar.rs:911`, `sugar.rs:190,390` and `ssi.rs:974` — were
  stale before it opened — and a **third** slipped past the re-check that
  recorded the first two: the ledger's member bullets still carried
  `sugar.rs:821`, `ssi.rs:975` and `render_freecad.py:159`, all three wrong,
  directly under a sentence claiming every line below had been re-derived.
  **S176(a) recurring inside the entry that documents S176(a)** is the sharpest
  form of the finding, and it was the PR's re-check that found it, not its
  author. Every one of them is cited by target name or by expression now.
  **That is the argument for the remedy being *cite by name*, not *re-check
  more carefully*:** the re-check catches it only if the citation outlives the
  edit that breaks it by long enough to be re-read.
- **(b) One census, written into three documents, drifts inside the PR that
  took it.** #831 put its link census in `SMELL-G-LOG.md`, in this document, in
  §D's D113 row and in its PR body; the re-sweep at its second merge of
  `origin/main` updated one of the four, so the log said *"277 links in 105
  files"* while this document said *"285 in 106 of 465"*. **It then happened
  again inside the fix that recorded it**: the replacement pair does not
  re-derive either — the sweep at the pre-merge head returns 285 in **105**
  files, so the corrected figure had a re-swept candidate count beside a file
  count from an older run, and the branch's final merge moved the candidates
  again. **The general form:** a number restated in N places is guarded in
  none, and the moment that bites is a re-sweep, which is exactly when the
  register is supposed to be right. **The cure, applied here rather than
  described:** the census has **one home** (S135), it states the sweep's
  definition rather than only its result so a re-derivation is checkable, and
  the other three sites now point at it instead of carrying a number.

**Why this is a roll-up and not two rows:** one mechanism — the record is
written before the change is final and is never re-derived against it.

**Verdict:**

## S177. A lane's §D row is struck while it is the only live tracker for the findings routed to it after dispatch

**[verified, #831 — found by G10's ledger walk, which G-R2 required be
re-derived from the tree]** Findings routed to an already-dispatched lane
ride along on that lane's §D row rather than taking rows of their own. When
the lane lands, **the row leaves the table under §D's own *live rows only*
rule — and the rides-along leave with it**, whether or not the lane touched
them. There is no other index: a finding is tracked by the row that names
it, and prose in §D pointing at a retired row is not a schedule.

**The instance, re-derived from the tree.** Track E's **E-g** row records,
in its own words, *"S111(a)(b)(d), S112(a) and S75 were routed to this lane
after it was dispatched and are **NOT in #768** — #768 partly closes S111(a)
as a side effect and says so at that finding; the rest are unstarted."* The
row is struck (`~~E-g~~`, *"both closed, #768"*), E-g is **DONE** in
`SMELL-E-LOG.md`'s roster, and §D's rides-along paragraph still reads *"…
belong to Track E's **E-g**, which is already ADVERSARIAL on those files"* —
present tense, about a lane that has retired. Three of the five checked
against the tree, all standing:

- **S112(a)** — `sweep/src/fillet/naming.rs`'s *"What consumes these rows"* header still says
  `names::emit_fillet` *"reads every field EXCEPT `Retired`"*, while
  `editor-core/src/names/emit_fillet.rs:220-221,236-246` builds and consults
  `retired_e`/`retired_v` from `rec.dead`.
- **S111(b)** — `surgery.rs:775` is still `pub fn ring_clearance`, and its
  only caller outside the module is still `sweep/tests/m6_surgery.rs:434`.
- **S111(d)** — `naming.rs`'s `Retired` still carries `edges` and
  `vertices` and no face channel.

S75 and S111(a)'s residue are named by the same sentence and are not
re-derived here. **S111(a) says outright *"this member is not fully closed
and has no row — the orchestrator owns whether the residue earns one"***,
which is the same hole seen from the other side: a member that knows it is
open and knows nothing is holding it.

**Why it is a class and not a clerical slip.** The mechanism is structural,
not anyone's oversight: rides-along exist so that a late finding reaches a
lane already reading those files, and the same *live rows only* discipline
that keeps §D honest is what deletes them. It will recur on every track that
routes work to an open lane. **The cheap fix is at retirement**, not at
routing: a lane's row may only be struck once each rides-along it did not
close has been re-homed or given its own row, and the lane's landing report
is the natural place to check, since E-g's own report is where the
five-member sentence was written.

**Not swept.** This finding names one instance because G10's ledger walked
into it. Every other struck row on every track has the same exposure and
none of them were checked; that is a sweep with an owner, which is **D124**.

**Verdict:**

**chord_join.rs discharged by #834 (G8).** The vocabulary
(`verbatim|re-derived|ported from|mirror|copy|copies|duplicat|identical|twin|hand-appl|hand-writ|parallel`,
case-insensitive) returns **37 hits** in that file, re-derived on `origin/main` at the
merge (`3ddd6011`), and S131's warning that the grep is a candidate list
rather than a count holds hard: **36 are false positives**, and four
shapes account for most of them. Mäntylä citations (`:18`, `:29`,
`:1895` — the book's *mirror site*). `derive(Clone, Copy)` (`:101`,
`:424`). **Values** copied or compared, never bodies: an arena copy of
the mate's wall surface (`:1318`), two spellings of one azimuth
(`:1482-88`), and the seven *"bit-identical"* claims about OUTPUT, which
are the single largest family. And records of duplication **already
removed**, including the guard row that forbids re-forking it (`:808`,
`:1347`, `:2462-2510`). The remainder are the word *copy* used for a
split's above-side vertex duplicates (`:39`, `:176-202`, `:1554`), which
is the one shape the vocabulary cannot be narrowed against without
losing real markers.

**One was stale, and is fixed by #834:** `:1232` said the boolean
planar-side lane's *"selection logic mirrors `chord_spec`'s S9 block
deliberately — same margins, same predicate names, same refusal
cases"*, but since S5 both lanes call the one `select_arc` body
(`:1180` and `:1317`) and `:2472-2479`'s guard exists to keep it that
way. A marker asserting a copy that is now shared code sends the next
reader to unify what is already unified — **S133's mechanism with the
sign reversed**, and it costs the same. The vocabulary found it
because the marker was written in the vocabulary's own words; S131's
blind spot (a marker in fresh words) is untested here and stays open.

---

## S171. The S11 sense-inheritance hazard was discharged by the Euler-atomicity work, and three comments still describe it as open

**[verified]** `Body::set_face_sense` (`topo/src/attach.rs:123-137`)
carries a **KNOWN HAZARD** block (`:123-138`): *"splitting does not
inherit the bit yet … Every `mef` mints its new face `sense: true`, including the
boolean splitting/reassembly re-mints (`chord_join.rs`,
`splitting/reassembly.rs`), so splitting a `sense: false` face today
would silently stamp `true` on the pieces."*

**That is false today.** `Body::mef_chords` reads the parent's
`face_data.sense` as `inherit_sense` (`euler.rs:1725-1726`) and hands it
to `mint_face_surface_and_sense` (`euler.rs:1981-1994`), which returns
the parent's bit whenever the fragment lands on the parent's surface and
`true` only when it mints a new one; `mef_lone` and `euler_kill`'s
`mfkrh` take the same decision from the same helper. `chord_join`'s two
re-mints both pass `FaceSurface::Inherit`, so both inherit. The guard
exists and is green: `sweep/tests/m5_s12_curved_ops.rs`'s
`a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit`.

**Three copies, and the hit list.**
`chord_join.rs:1788-1794` (*"SENSE HAZARD (M5 S11, banked)"*) —
**FIXED by #834**, replaced by the invariant and its guard.
`attach.rs:104-105` and `:123-138` — **not this unit**: outside G8's
scope cell, and §C cites `attach.rs:119`'s KNOWN HAZARD block as the exemplar
of *a named-and-pinned gap*, so correcting it edits prose belonging to another
section (and the citation is itself four lines off the block it names). `entity.rs:258`
(*"The Euler operators mint `sense: true`"*) — **not this unit**, same
reason, and it needs the qualifier rather than deletion: `mvfs` still
mints `true`, and so does `mef` on a NEW surface.

**Why it matters more than a stale sentence usually does.** The block
is what a reader consults before touching orientation, it names two
files by path, and it says the hazard is *unreachable in the current
battery* — an argument that was already weakening independently, since
`revolve` mints a **planar** `sense: false` face (the under-side
annulus, `sweep/src/revolve/axis.rs:373-374`) and needs no curved
boolean to do it. A reader who trusts the block will add the
inheritance that is already there, or will avoid a split that is
already safe.

**The replacement sentence must stay CONDITIONAL, and here is why**
(re-derived on this branch after #834's style review raised it). A
corrected block saying *"splitting inherits"* unqualified would be the
next false sentence, on two counts:

- `splitting/finish.rs:280` promotes a section ring through
  `mfkrh(ring, FaceSurface::New(plane_for(ring_side)))` on a **live**
  path — a new surface, so `mint_face_surface_and_sense` returns
  `true` and the promoted face is stamped, correctly, rather than
  inherited. Inheritance is conditioned on the fragment keeping the
  parent's surface, and this path deliberately does not.
- of the block's two named sites, `splitting/reassembly.rs` is a
  **test-only** oracle (`reassembly.rs:1`: *"The reassembly oracle
  (test-only)"*), so naming it beside `chord_join.rs` reads as two live
  re-mints when it is one.

So the fix is *"splitting inherits the parent bit whenever the fragment
keeps the parent surface, and mints `true` when it mints a new one —
`finish.rs`'s section promotion is the live case of the latter"*, not a
flat retraction.

**Scope:** `topo/src/attach.rs` and `topo/src/entity.rs`, plus the surviving
`§C` citation of `attach.rs:119` in this document. **Row: D77.**

## S172. Five spellings of "is this line code", beside seven guards that already share the walk

**[verified]** Raised by #834's style review over its own new guard.
The textual guards that walk a crate's own sources each carry their own
answer to *is this text code or comment* — `trim_start().starts_with("//")`
— and each inherits the same blind spots: a `/* … */` block, a
`#[doc = "…"]` attribute, the needle inside a string literal or a
`macro_rules!` body, and any code that FOLLOWS a comment on the same
line. Four instances:

- `topo/src/face_normal.rs` — **FIXED by #834**, the instance this row
  was raised from. Its predicate now comes from
  `topo::fixtures::code_only`, which blanks comments and literal bodies
  while preserving byte offsets and line structure, and is pinned on
  all four shapes above plus the one a naive quote scanner breaks on (a
  lifetime, `&'a str`, which must not open a char literal).
- `topo/src/review_d18_probes.rs:263`
- `geom-core/tests/flagged_census.rs:182`
- `step-import/tests/tier_gate.rs:787`

**The class is the finding, and the walk was already consolidated.**
Seven guards in `topo/src` share `fixtures::crate_sources()`
(`chord_join.rs:2490`, `pcurves.rs:1621`,
`review_m1_pr5_internal.rs:323`, `sector_shape.rs:508`,
`review_d18_probes.rs:252`, and two rows in `face_normal.rs`) — the
style review counted eight; it is seven, and its `chord_join` line
number is six lines off. `fixtures.rs:74` says outright that *"a guard
against duplication should not be the next copy of its own walk"*, which
is the argument for the predicate as much as for the walk. **A home
existed and the comment test is the part that kept being copied
instead.**

**Why it is not cosmetic.** A guard whose comment test is wrong is
green for the wrong reason. `face_normal.rs`'s inventory pins that file
at **zero** reads of `Face::sense_sign` while the file names the method
five times in `//!`/`///` prose — the whole zero rested on one prefix
test, so a single block comment in that file would have reddened its
own guard.

**Scope:** the three remaining instances, in three crates. `topo`'s is
a lift into `fixtures::code_only`; the other two are in crates with no
such shared home, and whether one is minted or the predicate is copied
twice more is the scheduling question, not the lane's. **Row: D80.**

## S173. The curved generalization of the one door lives inside `boolean/`, which is what the door's own header argues against

**[verified]** Raised by #834's style review; recorded rather than
acted on, because the fix is a move, not a sentence.

`face_normal`'s module header spends nine lines (`face_normal.rs:14-24`)
on why the planar sense flip moved to the crate root: when
`sector_face` became a crate-root module shared with the splitting
lane, a door inside `boolean/` could no longer be the one door, because
*"a crate-root module importing from `boolean/` would be the same
wrong-way edge, pointed the other way."*

`boolean::rest::face_carrier` (`rest.rs:510`) is documented at its own
site (`rest.rs:502-504`) as *"the curved generalization of
[`face_plane`], folding the face's sense into the material side exactly
as that door does (S10)"*. It is `pub`, re-exported from the crate root
(`topo/src/lib.rs:258`), and folds the `±1` itself (`rest.rs:512`). So
the curved half of the one door sits inside the consumer the planar
half was deliberately moved out of, and reaches the crate root by
re-export rather than by living there.

**Not a defect today, which is why it is a row and not a fix.**
`face_carrier` has no consumer outside `boolean/`, so the wrong-way
edge the header warns about does not exist yet; what exists is a
placement contradicting a stated argument next door, and that argument
is load-bearing — it is why `face_normal.rs` exists at all. The
question is whether the two halves want one home, which is issue
**#695**'s territory (where `face_outward_normal`'s own placement
question is already banked) rather than a prose fix.

**Scope:** `topo/src/boolean/rest.rs`, `topo/src/face_normal.rs`. **No
§D row** — scheduling is the orchestrator's call and it may want this
to ride #695 instead.

---

# Findings raised by the Track F lanes (2026-08-20)

## S121. Bound-domination rows with no ceiling and no floor — five sites, four crates

A recurring test shape: a certified bound is compared against a sampled
true value and asserted to **dominate** it, and nothing else is
asserted. `sup >= max` is monotone in the safe direction — an
implementation that returned `f64::MAX`, or that lost a cancellation,
satisfies it forever. Several of the rows are *named* for domination,
so the geometry each was written to cover is the one thing it never
constrains.

The discipline exists in the tree and is written down where it is
applied: `geom-core/tests/m5_pr7b_tensor_compose.rs:195-207` carries
both halves — `assert!(max > 1e-3, "the fixture's residual must be
genuine")` as an anti-vacuity floor, and `sup <= 10.0 * max` as the
ceiling, with the reason stated: *"the bound is TIGHT — the whole point
of the composition … the composite must track the residual's own
scale."* **A row missing the ceiling can be arbitrarily loose; a row
missing the floor is satisfied for free by a fixture whose sampled
residual collapsed.** Most of the sites below are missing both.

- `geom-core/tests/m5_pr7b_tensor_compose.rs:222` —
  `the_bound_dominates_when_the_two_curves_disagree_on_knots`.
- `geom-core/tests/m5_pr7b_tensor_compose.rs:244` —
  `the_bound_dominates_when_the_pcurve_straddles_surface_cells`.
- `geom-core/tests/m5_pr7b_tensor_compose.rs:425-426` —
  `a_bicubic_bicubic_composition_completes_within_the_budget`, whose
  doc says the composition *"must complete with a finite, sound
  bound"*; `is_finite` plus `sup >= max` is exactly that and no more.
- `mesh/src/nurbs_cert.rs:1538` —
  `hessian_hull_dominates_sampled_second_partials`, doc *"the convexity
  claim, measured (never the other way round)"*; the measurement has no
  upper side.
- `geom-brep/tests/r1_pxn_probes.rs:176` — `hull_sup >= truth * 0.99`,
  labelled `UNSOUND:` on failure, with no companion ceiling.
- `geom/tests/curves/m5_pr7_speed_meter.rs:36` —
  `the_meter_lower_bounds_the_real_speed`, a one-sided meter claim with
  only a positivity check above it.

**The floors and ceilings have to be measured, not copied.** `10.0` is
one row's number for one row's geometry; a constant transplanted from
the row above, or a threshold that re-pins today's output, is
`memories/output-stability-as-justification.md`'s shape rather than a
fix. Whoever takes this owes a ratio per site at more than one ε, and
is entitled to conclude for some site that no honest ceiling exists —
which is a written verdict, not a silent omission. Placed as **D65**. **That row was placed by Track F and never carried into Tracks J–X, so it is OPEN AND UNSCHEDULED — the missing row is not evidence it landed.**

Found by lane F-c's sweep for S110's shapes: the second of the style
brief's Q3 shapes, *an assertion monotone in the safe direction*. F-c's
first pass filed only the two `tensor_compose` rows and called the
class a single-file instance; #790's review found the other three.

## S122. The NURBS re-gate's comment sends a reader to the wrong crate for half of its evidence

`topo/src/boolean/ops.rs`'s door-2 comment, on the `NurbsExtentUnsupported`
re-gate, reads: *"NO end-to-end path reaches it today (a lofted operand
is refused at its NURBS EDGES first, a placeholder's poison box is never
pruned) — `sweep`'s `s16_box_soundness` pins both blockers, so the day
one lifts is loud."*

`sweep/tests/s16_box_soundness.rs` pins **one** — the lofted operand,
refused at its NURBS edges. The placeholder blocker is pinned in
`ops.rs` itself, as **door 1** at `crates/topo/src/boolean/ops.rs:2126-2132`
— a `let … else { panic! }`, thirteen lines above the comment, not an
`assert!` (the function's three `assert!`s are at `:2150-2152`, after
door 2). So the sentence is wrong in both directions at once: it credits
a sibling crate with a row it does not have, and it hides a row the
reader is standing in.

Nothing checks it — a comment naming a test file in another crate is
the one cross-reference shape no compiler and no gate in this tree can
see (C-R11's class, and S39's).

Two answers, and the choice is the deliverable: correct the sentence, or
write the missing row. The second is available — `sweep` can build the
placeholder operand, and a row there would make both blockers loud in
the file the sentence already points at. Placed as **D66**. **That row was placed by Track F and never carried into Tracks J–X, so it is OPEN AND UNSCHEDULED — the missing row is not evidence it landed.**

## S123. Assertions whose condition is the value's own codomain

The narrowest member of S110's class and the one a reader is most
likely to walk past, because it looks like a soundness check:

- `geom-core/tests/review_m5_pr7b_tensor.rs:520` —
  `assert!(sup_far >= 0.0 || sup_far.is_nan(), "bound must stay sound")`.
  `SurfaceResidual::sup_bound`'s own doc (`tensor.rs:252-255`) says the
  value is a fold of **nonnegative magnitudes**, `NaN` on every poison
  path. The disjunction is the function's entire range, so the row can
  only ever change a panic message — **S110(h)'s exact shape**, and it
  sits one line above a genuine ceiling (`sup_far <= 1e-8`) that does
  the work. This is in the file S121's text cites as its cancellation
  witness.
- `geom-core/src/real.rs:1393` — `prop_assert!(r >= 0.0)` on
  `Real::sqrt(x)` for `x in 1.0e-12..1.0e12`. IEEE `sqrt` of a positive
  finite is positive finite; the line below it (`(r*r - x).abs() <=
  1e-15 * x`) is the property.

Both are cheap deletions rather than repairs: the real assertion is
already adjacent in each case, and removing the redundant one makes the
surviving message unambiguous, which is what
`memories/test-suite-cost.md` asks of a merged row. Neither is a
correctness risk. Placed as **D67**. **That row was placed by Track F and never carried into Tracks J–X, so it is OPEN AND UNSCHEDULED — the missing row is not evidence it landed.**

**This shape is invisible to the obvious detector**, which is why it is
recorded separately from S121: a rule of the form *"a test whose EVERY
assertion is weak"* cannot see a vacuous assertion standing beside a
real one, and both members here do exactly that — as does S110(h), and
as did the `nurbs_edges > 0` assertion #790 wrote and removed in
S110(d). Anyone sweeping for this must key on the assertion, not the
test.

## S124. A compound bound given a NAME is invisible at its use sites, and the gate header says the opposite

**[verified]** `crates/profile/src/path/arc_fillet.rs:593` declares
`pub trait ArcCarrierScalar: Decide + Bounds {}`. The **declaration**
fires `bounds-allowlist.sh` (it writes the pair literally) and
`arc_fillet.rs` is allowlisted for it under ruling LB3. Every **use** of
the name is a `Decide + Bounds` bound the gate cannot see: **49
occurrences** across `profile/src/path/family.rs` (27) and
`profile/src/path/program.rs` (22), on `pub` and `pub(super)` fn
signatures, on four public trait declarations (`ArrivalSpec`,
`TangentIncoming`, `PointIncoming`, `LegEndIncoming`) and on
`PartialPath` impl blocks. The name is re-exported from
`profile/src/path.rs:364`, `profile/src/lib.rs:133` and
`pncad/src/profile.rs:57`, so the pair is `pub` from the top-level crate.

The gate header's arc_fillet entry said *"Confined to this one file so
path.rs itself stays bracket-free"*. The **literal spelling** is confined;
the **bound** is not. #791 corrected that sentence and recorded the hole
as the gate's KNOWN GAP 3, rather than closing it by grep — closing it by
grep means redding two files and allowlisting them, which is the
cry-wolf-then-allowlist outcome S63 already records at `linalg/mat.rs`.

**This is the same class as S59 one level up.** S59 was a matcher blind to
an alias *name*; this is a matcher blind to an alias *use*. Whether these
49 sites are legal depends on what `ArcCarrierScalar` is bound to, which
is **S87**'s question (§D row **G4**).

**G4 does not discharge this, and saying otherwise would be a disclosed
blind spot read as a discharge.** G4 changes what the alias is *bound to*;
the 49 uses stay exactly as invisible to any grep, and `bounds-allowlist.sh`'s
KNOWN GAP 3 is as open after G4 as before. Two different jobs: G4 decides
whether the sites are legal, D68 is whether they can be *checked*. The
same correction bears on G4's own evidence — the widened matcher fires on
**none of G4's sites** (`arc_fillet.rs` is allowlisted by file, the uses
are alias-shaped, `geom`'s doors are sole bounds), so a green gate there
is not ratification evidence.

**And the matcher's own version of the same hole is KNOWN GAP 4**, added
by #791: an alias whose name does *not* end in `Bounds` is invisible at
its uses too. Its mitigation — the declaration writes the pair literally
and fires, so the declaring file must be ratified — is planted in the
gate's self-test rather than asserted, which is the shape D68 should
close in.

**Verdict:**

## S158. The compound-`Bounds` gate anchors on `+`, and `+` is not how Rust expresses a compound bound

**[verified, with compiled counterexamples]** Found by the F-e verification
pass, and it **subsumes S59 rather than sitting beside it**. S59 was *"the
matcher is blind to an alias"*; this is *"the matcher is blind to a
spelling"*, and the alias case is one member.

`scripts/gates/bounds-allowlist.sh` matches a `…Bounds` identifier adjacent
to a `+`. Rust has several ways to write the same obligation, and the others
are silent. **Plain compound bounds, no alias involved:**

```rust
pub fn f<T>(_t: T) where T: Decide, T: Bounds {}   // gate exits 0
pub fn g<T: Decide>(_t: T) where T: Bounds {}      // gate exits 0
```

**And the alias form, which is `bounds-allowlist.sh`'s KNOWN GAP 4:**

```rust
pub trait Bracket: CertifiedEnclosure           // gate exits 0
where
    Self: Bounds,
{
}
pub fn use_it<T: Decide + Bracket>(t: T) { t.lo(); t.sgn(); }
```

**The fact that makes this a hole rather than a curiosity: that last form is
what `rustfmt --edition 2021` converges on** from the single-line spelling
the gate *does* catch. A gap a formatter produces out of the caught form is
the resting state, not a corner case. Two further members, independent of
each other: the same declaration in an **allowlisted** file is silent (so a
file ratified for one bound silently ratifies this one — that half is
**S159**), and a chain through a non-`…Bounds`-named intermediate is silent.

**Not a violation today.** Every `where`-clause `…Bounds` line in
`crates/*/src` was checked: no live instance in an unratified file. This is a
hole in the instrument, not a breach of the rule.

**Deliberately not closed by #791**, on the orchestrator's ruling: it is a
redesign of what the gate matches, no line-based matcher can reach the
formatter-stable spelling, and widening the declaration matcher far enough to
try false-positives on a trait generic over a **sole** bracket bound
(`trait ArrivalSpec<T: CertifiedBounds>`), which is outside the class.
Whoever takes it will red a population nobody has counted, so **F-R6's
grandfathering caveat becomes live on a real residue** rather than the empty
one #791 met.

**Verdict:**

## S159. The compound-`Bounds` allowlist is file-granular, so ratifying one bound in a file ratifies every bound later added to it

**[verified]** Found by the F-e verification pass, alongside S158 and
**recorded separately because it is a different mechanism and will outlive
whatever the matcher does.** Every entry in `bounds-allowlist.sh` is a path:
`^crates/geom-brep/src/(pcurve_cache|ssi|ssi/certify|edge_nurbs)\.rs$` and so
on. The ratification argument in the header is per-*seam* and often
per-*function* — `probe_tube_chart` is justified by what that one body does —
but the enforcement is per-*file*. A second, unrelated compound bound added to
any allowlisted file inherits the first one's ratification silently.

This is the same shape as S63's `linalg/mat.rs` (allowlisted wholesale to
resolve a false positive, now unguarded for the genuine case), except that
here it is the *design* of every entry rather than one bad entry.

The cheap half is that the header already writes the per-seam argument, so the
information exists; what does not exist is any check that a file's current
bounds are the ones argued for. Options a taker should weigh rather than
assume: line-scoped or symbol-scoped entries; a count pinned per file; or
accepting file granularity and saying so at each entry.

**Verdict:**

## S163. What the F3 sweep left open in `scripts/gates/` — the member that was live in production `Interval` code is FIXED by #885; four remain open

Raised by lane **F-g** while closing S63 and S157, and **grown by that
PR's style and adversarial reviews from three members to five** — the
two added are the ones with numbers attached. One row (**D109**),
because each is a disclosed blind spot of the same sweep and a reader
who takes one should see the others.

**(a) FIXED by #885 — both sites reassociated, then the matcher
widened.** `orthonormal_basis`'s `b1` now writes `s * self.x.powi(2)`
and `rotation_about`'s three diagonal entries write `t * nᵢ.powi(2)`;
the off-diagonals are genuine mixed products and were left alone. The
gate's matcher grew a second branch for the parenthesized scaled square
**after** the conversion, so it landed on a tree it passes, and
`linalg/mat.rs` came off the allowlist — its entry was a scheduled
residue held open for exactly this conversion and nothing else in the
file is a square. The record of what moved, what did not, and what the
widened matcher still cannot see is #885's; the ruling that authorised
it is kept below because it is Evan's and outlives the work.

**RULED YES — Evan, 2026-08-21.** The question was *may a D9-fixed
evaluation order be reassociated when the reassociation is strictly
tighter?*, and the answer rests on two grounds, both of which are worth
carrying because both were nearly got wrong in the asking:

1. **Moving output is not on its own a reason not to act.**
   `memories/output-stability-as-justification.md` names *arithmetic
   association* as exactly the kind of decision output stability may
   not settle, and says committed bytes are *"usually a golden, and
   regenerating a golden is a chore, not a contract."* The document
   that asked this question offered the `f64` byte-move at `mat.rs` as
   a **downside**, two paragraphs after citing that memory — the error
   the memory exists to prevent, committed in the act of consulting it.
2. **The D9 carve-out does not reach this.** D9 is **determinism at one
   kernel, not a pin on last year's output**: the same document
   evaluated twice must agree, and it need not agree with a build from
   a year ago. `u_ref` is stored as data under D2, so documents that
   already exist keep the frames they were built with.

**What the exact-`f64` argument predicted, and what happened.** It
predicted **cost, not permission**: `b1`'s scale is `s = ±1`, so that
reassociation is byte-free, and `mat.rs`'s is `t = 1 − cos θ`, so it
moves `f64` bytes and re-cuts whatever goldens sit downstream. The
first half held and #885 asserts it rather than assuming it. **The
second half moved bytes and re-cut nothing**: the reassociation is
*exactly* byte-free whenever every axis component is `0` or `±1`
(`(t·0)·0 = t·(0²)` and `(t·1)·1 = t·(1²)`), which is what every
rotation feeding a committed artifact in this tree turns out to be, so
the full hosted matrix — three ε values, the interval feature, k-lint,
all four render lanes, the byte-golden STEP corpus — came back green
with no re-cut. The change is nonetheless real: 34.6% of random
(θ, axis) pairs give a different diagonal entry. **At `Interval` both
sites narrow, but conditionally, and #885 states the condition rather
than the slogan**: the old order treats the two factors as independent,
so an enclosure straddling zero acquires a spurious sign range that the
scaled square does not — *provided the scale is sign-definite*. Where
`n.z` straddles zero `s` is `[−1, 1]` and the two spellings are
algebraically identical; and `powi(2)` is 1 ulp wider on each side
below `|x| < 2^-480`, per the third rider on the ruling. `mat.rs`'s
`t = 1 − cos θ` is nonneg by construction, so its narrowing is
unconditional.

**(b) `bounds-allowlist.sh` is the one gate still on the leading-`//`
comment filter, the one still keeping a self-test helper that runs
in-process, and the one whose `gate_main` call still passes arguments
to a default self-test that no longer exists.** All three because
F-g's brief fenced that file (F1 had just landed it). Its own KNOWN
GAP 5 names this lane as the taker, and its `bounds_selftest_passes`
carries a comment saying it is named gate-specifically so that
promoting one into `lib.sh` cannot shadow it — `lib.sh` now has
`gate_selftest_passes`, so the promotion is a deletion and two
call-site edits. The in-process helper is a must-PASS case, so it is
not blind the way S157's were: a gate that died would fail it, with a
misleading message.

**A fourth gate is off the shared reader on purpose, and it is the one
that decides the reader's interface.** `probe-suite-census.sh`'s
probe-gate matcher looks for `#[cfg(feature = "probe")]` — **its needle
contains a string literal**, so the code-only view would blank exactly
what it wants. It needs a fourth view (comments stripped, literals
kept) that `gate_rust_code` does not build. Its matcher is anchored at
column zero instead, and it carries a prose fixture because of that.
**#849's first record claimed no gate here greps for a string literal**,
which was the whole argument for building one view; the argument still
holds for the six gates converted, but as a statement about them, not
about the directory.

**(c) The shared reader is a lexer, and says so.** Nested block
comments (Rust allows them; the first `*/` closes here),
`macro_rules!` bodies, `include!`d text, and anything behind a
`#[cfg]` other than the `test` skip are all read as ordinary code.
Only a **test-only** attribute may exclude — `any(test, …)` and
`not(test)` are both scanned, since `any(debug_assertions, test, …)`
is every debug build (`topo`'s `test_support_impl` is exactly that,
and an earlier draft skipped it). An ALL-CAPS operand
(`SOME_CONST * SOME_CONST`) and an indexed square (`v[i] * v[i]`) are
both invisible to the square matcher, the first deliberately (the
ALL-CAPS population here is `usize` sizing) and the second not.

**(d) 14 of 71 `gate_error` call sites in `scripts/gates/` are reached
by no self-test case.** Measured on the landing tree by instrumenting
`gate_error` with `BASH_SOURCE`/`BASH_LINENO` and tracing all fourteen
`--selftest` runs; 57 are reached. The two shared cases #849 added — an
unreadable `--root` and an empty tree — took several of these off the
list, including one of `lib.sh`'s own *"a gate that scanned nothing is
not a pass"* pair, and found `kernel-serde-free.sh` dying before its
own diagnosis. What is left, and each is a fixture somebody has to
write: `lib.sh:92` (needs `crates/*/src` present and empty, which the
empty-tree case does not produce) and `lib.sh`'s *"defines no
`gate_selftest`"* guard, unreachable while every gate defines one;
`gate-roster.sh` × 3; `probe-suite-census.sh` × 3 (including the
nested-suite diagnosis inside the command substitution that motivated
the stdout→stderr move); `interval-square-allowlist.sh`'s *"every source … is test-only"* guard (**named, not numbered, per G-R13** — the member originally cited a line number, which was already stale when written and moved twice more inside #885 alone; the guard is the one reached only when the production-source set is empty),
`kernel-serde-free.sh:79`, `signed-zero-one-home.sh:96`,
`test-aggregation.sh:67`, `test-features-dev-only.sh:271,277`.
**`lib.sh` says a guard never shown to fire is not a guard**; this is
the size of that sentence unapplied inside its own directory, and it is
a number rather than a caveat so that it can be worked down.

**(e) The real-scan cost went from ~4.5 s to ~48 s across the six
converted gates**, measured on the F-g lane container (a shared box at
load average ~12, not a runner): 5.9 / 6.2 / 6.0 / 9.6 / 11.3 / 9.5 s
against 0.5–1.0 s each before, and both halves of CI run every gate
twice (`--selftest` then the gate). **On the hosted runner the whole
`discipline` job is 30 s against a 23 s pre-change baseline and every
gate step is 0–1 s** (runs `32439375293` and `32388258102`), so nothing
is at risk today — the number is here because a tenfold aggregate
change should be findable by the next person rather than discovered.

**Verdict:**

## S164. `scripts/ci-filter.py` decides whether any gate runs at all, and it is the one script here with no test

**Relocated out of S63 by lane F-g rather than closed with it** (F-R11:
a claim the closing record cannot carry moves, it does not vanish).
S63's other three halves are fixed; this one is not, and recording the
whole finding as FIXED would have made a live claim read as closed.

**Re-derived, because S63's line numbers had moved.** The script is
**385 lines** (S63 said 367). Every gate under `scripts/gates/` carries
a `--selftest` that both halves of CI invoke; `check-interval-cfg-additive.py`,
`check-ci-mirror-parity.py` and `demos/check_render_provenance.py` do
too. `ci-filter.py` has neither a self-test nor a test file anywhere in
the tree, and both halves depend on it: `ci.yml`'s `change filter` job
runs it and every downstream job reads its output, and
`local-scripts/ci-local.sh` walks the same output. It fails **closed**
on uncertainty — `Bail` (`:141`) is caught at top level and turned into
`TIER=all`, which covers the direction that matters most — but the
`docs` branch (`:202`, `all(_is_docs(f) for f in files)`) is a
**fail-open** path taken before any of that, and it is the branch S61
depends on. `lib.sh` says a guard that has never been shown to fire is
not a guard; the sentence was never applied one level up.

**Why F-g did not take it.** Wiring a `--selftest` into both halves is
an edit to `.github/workflows/ci.yml`, which lane F-2 had just landed
and F-g's brief fenced — and `check-ci-mirror-parity.py` now Bails on
workflow shapes it does not recognise, so the edit may red with an
instruction to extend the recogniser. That is a sequencing fact, not an
argument against the row.

**Verdict:**

---

## S168. FIXED IN PART by #844 — three sites say the probe lane's decisions are bit-identical to f64's, and nothing anywhere checks it

**Found while re-deciding where the probe preconditions should run, which is
why it matters rather than being a wording quibble:** the placement argument
for those tests was built on what their docstrings say, and the docstrings say
something the assertions do not.

Three sites, all in `crates/editor-core/tests/`:

- `m5_pr5_corpus_probe.rs` — *"the document replays at the recording scalar
  with **bit-identical decisions**"*. The body is
  `assert_eq!(corpus::failures(&ev), Vec::<String>::new())`.
- `m4_pr8_k_probe.rs`'s `corpus_evaluates_green_at_probe` — *"The Probe lane's
  decisions must be **bit-identical to f64's** — the recording scalar is a
  wrapper, never a different arithmetic."* The body asserts
  `failures(&ev).is_empty()` per document.
- `m4_pr8_k_probe.rs`'s `run_doc` — the same claim inside an **assertion
  message**: *"decisions are bit-identical to f64, so this can only mean the
  f64 rows are broken too"*, printed when the assertion fires, telling a
  debugger a thing the test never established.

**All three assert one-sided GREENNESS at `Probe`. None compares a `Probe`
result against an f64 one**, and no test in this tree does. Greenness is
evidence for bit-identity and is not a check of it — a `Probe` lane that
decided differently but still produced a valid body would pass every one of
them. Greenness is also **tolerance-dependent**, which the *"bit-identical"*
framing hides: it is why the dump is swept at three ε, and it is why the
argument *"there is nothing per-ε about this claim"* was wrong.

**S39/S112's class**, in the two tests F8 exists to make run, and it cost
something concrete: an orchestrator ruling and an amendment relayed to Evan
were both written from the doc comments and were wrong about what the code
does.

**FIXED IN PART by #844**: all three sites now say what they assert, name the
stronger claim as unasserted, and say that greenness is ε-dependent. **What is
NOT fixed is the gap itself** — the wrapper property is still checked by
nothing. That is **D114**.


# Findings raised by the Track H lanes (2026-08-21)

## S214. Eleven `compile_fail` doctests in `geom-core/tests/` have never been collected, and each one asserts the compiler rejects something

**Raised by the Track H orchestrator, 2026-08-21**, from a fact lane H-g
established while implementing `S90`'s ruling: **a doctest in a `tests/`
target is never collected.** H-g verified it empirically — it planted a
deliberately-failing `compile_fail` block in
`crates/geom/tests/dual_foot_tangent.rs` and `cargo test --doc -p geom`
reported **zero** rows from the file. That is why #886 sites its
compile-fail rows in `geom/src/projection.rs`, where rustdoc runs them.

**This is `D113`'s root cause with a second and worse consequence, which
`D113` does not draw.** `D113` already records that *"`cargo doc` builds
no test targets, so every [intra-doc link in a `tests/` file] is inert —
never rendered, never resolved, never checked, on any tier."* The same
fact applies to **fenced code blocks**, and there the failure is not a
dead link but a **dead proof**.

**Census, over `crates/*/tests/**.rs`.** 28 opening fences inside `///`
or `//!` comments across 17 files. **15 are `text` and 2 are `sh` — those
are correctly annotated, never meant to execute, and are not this
finding.** The remaining **11 are `compile_fail,EXXXX`**, all in Track
H's own ground:

| file | count | codes |
|---|---|---|
| `geom-core/tests/review_m0_pr2.rs` | **8** | E0369 ×2, E0277 ×2, E0599 ×3, E0605 |
| `geom-core/tests/review_m0_pr3.rs` | **3** | E0369, E0599, E0277 |

**Why this is the sharp end of `S110`'s class rather than another member
of it.** A vacuous assertion passes for a bad reason; a `compile_fail`
block in a `tests/` target **is not run at all**, and what it claims is
that *the compiler rejects a specific program*. So each of the eleven is
a negative proof about the type system that **no tier has ever
evaluated** — and negative proofs rot in the one direction nothing else
catches: the day a bound is loosened or a method added, the program
starts compiling and the block that would have gone red is not there to.
These are M0-era probes (`Real` has no `PartialOrd`, no equality, no
bad cast) pinning exactly the kind of property later work erodes.

**What is owed.** Move them where rustdoc runs them — `geom-core/src`,
the pattern #886 uses — or convert them to a form a tier executes, or
delete them and say the claims are unguarded. **What they must not do is
stay somewhere that reads as executable and is not.** Whoever takes this
runs each block *first*: a block that has never been collected may not
even fail correctly any more, and **an eleven-of-eleven pass would be
the surprising outcome, not the reassuring one.**

**Not swept beyond `crates/*/tests/`.** Doc-comment fences under
`demos/`, `tools/`, `review/`, `review-probes/` and `local-scripts/` are
not censused here; the same fact applies wherever a fence sits in a
non-`lib` target, and `crates/bvh`-style unowned ground is where it will
be missed.


## S210. The sole-`T: Bounds` class has a rule, no instrument, and no census outside `geom`

**Raised by H-d while producing S88's `geom` census.** `real.rs`'s
`Bounds` scope rule says bracket extraction may appear only in
certification and driver code, and that code writes `T: Bounds` as the
parameter's **sole** bound. Every ratified amendment to that rule, and
the whole of `scripts/gates/bounds-allowlist.sh`, is about the
**compound** form — and the gate plants a sole bracket bound as an
explicit **must-not-fire** self-test case (`plant_sole_bracket_bounds`),
because firing on it would red every certification file in `geom` and
`geom-brep`.

So the form the rule actually prescribes is the form nothing watches.
That was free while `Dual` had no `Bounds` impl: a sole-bound door was
reachable only by scalars that certify. **The D1 ruling (2026-08-19)
ended that in one stroke**, and S88 is what one crate's worth of the
consequence looks like — five modules, twelve doors, one of them a
reachable wrong answer (**#874**) that no grep for `Dual` could have
found.

**This has a twin already on the register, and they were unlinked.**
`impl<T: Bounds> Enclosure for T` is sole-bounded bracket extraction, so
D1 made a `Dual` an `Enclosure` too; its doc says *"it is not gated
either: `bounds-allowlist.sh` greps for `Bounds`, not for `Enclosure`. A
new `T: Enclosure` bound on anything that certifies would be a hole, and
no CI row would say so"* — **issue #701**, open, undecided. Same rule,
same gate, same silence, one trait over. **Whoever takes either takes
both**, and #701's *"it may well not need to be gated, but nobody has
decided"* is the honest state of this row as well.

**The class boundary, since S88's handoff draws it and this row's first
draft lost it.** This is about **sole** `T: Bounds` (and `T: Enclosure`).
A **compound** `Decide + Bounds` door with no `CertifiedEnclosure` — e.g.
`geom-brep/src/pcurve_cache.rs:1055`, `topo::separation`,
`chart_region_overlap` — is dual-reachable too, but it is a *ratified
seam whose third term is missing*, which the scope rule already tracks by
name and which the gate does see. Two different holes; conflating them
makes the census unbuildable.

**What is not censused.** S88 covers `geom-core` (no generic door; one
blanket impl, above) and `geom` (five modules). Enumerated but handed off
rather than taken: `geom-brep/src/ssi.rs:218`,
`ssi/certify.rs:{271,277,289}`; `profile/src/{fillet_select.rs:169,
path/arc_fillet.rs:361}`; `bvh/src/aabb.rs:87`. Nobody has walked
`topo/`, `sweep/`, `editor-core/`, `mesh/` or `step-*` for the shape at
all, and the doors that matter are the ones nobody would think to look
for, since they never mention a dual.

**What would close it, and what it costs.** Not a bigger regex — the
gate's own KNOWN GAPs 1–4 are the proof, and a sole-bound matcher has a
worse problem than the compound one: the population is large and mostly
*legitimate*, so an allowlist of it is a roster nobody maintains. Two
honest options. **(a)** A whole-tree walk of every `trait` declaration's
supertrait list plus the identifier grep — what S88 did for two crates —
producing a **census with a disposition per door**, re-derived per
milestone rather than per merge. Cost: one lane per crate group, and it
expires. **(b)** A walk of every generic parameter's *resolved* bound
set, which is the only thing that sees an alias declared in another crate
or a bound reached by a supertrait obligation. Cost: a `rustc` driver or
a `rust-analyzer` query — a real tool, and the only version that stays
true.

**Not H-d's to place.** The class spans Track G's ground (`profile/`),
Track C's (`geom-brep/`), Track I's (`mesh/`) and `bvh/`, which no track
owns — so a schedule row for it is an orchestrator's act, not a lane's,
and one is deliberately not minted here.

## S211. FIXED IN PART by #875 — the two `geom` modules; the `bvh` member is unowned and open

**Scope in the lead, per S88's own convention one finding above.** #875
fixes the two `crates/geom/` members. **The third member, in `crates/bvh/`,
is not fixed and has no owner** — `bvh` is outside Track H's scope
(`geom-core/`, `geom/`) and outside every other track's.

`geom/src/curves/boxes.rs` said *"certified-box driver code, an
allowlisted [`Bounds`] seam (ratified 2026-07-29 …)"*, and
`geom/src/surfaces/boxes.rs` said the same in one clause. Neither file is
on `scripts/gates/bounds-allowlist.sh`'s list, and **neither can be**:
both write a **sole** `T: Bounds`, which is the gate's planted
must-not-fire case. The 2026-07-29 amendment they cite ratifies the box
constructors to write the **compound** `Decide + Bounds` form — a
permission neither module uses.

So the sentence was wrong twice over, and in the direction that costs
most: it told a reader that a gate is watching this file. That is the
same class as `bounds-allowlist.sh`'s own retracted GAP-4 mitigation —
*"a false mitigation is worse than a disclosed hole because it tells the
next author the door is shut"* — committed in the files the gate was
written to leave alone. Both now say sole-bound, and say that the rule
covers them while the gate does not.

**The open member:** `bvh/src/lib.rs:56-61` says its `Bounds` reads are
ratified *"(the CI discipline grep allowlists exactly these seams)"*, and
`crates/bvh` appears nowhere in the gate's filters — `aabb.rs:87` is a
sole bound too. One clause, and whoever next opens that crate should take
it. **This row does not retire until they do.**

## S232. `geom`'s box constructors say a loose box "never prunes" — one crate below four doors, three of which do not prune

**Raised by I-d (#876) while fixing the same sentence in
`topo/src/boolean/boxes.rs`; ROUTED TO TRACK H**, whose scope is
`crates/geom-core/` and `crates/geom/` (§D, in those words) and which is
live. Not a wandering row: it has an owner and this line is it.

**The claim, six times over.** `crates/geom/src/surfaces/boxes.rs:22-24` —
*"looser is conservative"* and *"the poison box, **which never prunes**"*, on
`nurbs_surface_aabb`. `crates/geom/src/curves/boxes.rs` says it three more
times (`:16` for the whole module's poison rule, `:175` on `circle_arc_aabb`,
`:373-375` on `nurbs_curve_aabb` with its own *"looser is conservative"*), and
`:98` adds *"Slack only ever includes more extrema, so it errs outward (a
looser box)"* as though outward were free.

**Why it is false, with the count I-d had to correct to see it.** These boxes
are not consumed by a pruner. `nurbs_surface_aabb` is what
`topo::boolean::boxes::face_box`'s `ControlNet` arm returns, and `face_box`
feeds **four** doors — `boolean/reduce.rs`'s C10 tree, `separation.rs`,
`boolean/ops.rs:1486`'s sphere-extent fallback, and (through the shared
`FaceBoxRule`) `census.rs`'s arm 2. **Only the first prunes.** At the other
three, box non-overlap is the answer being sought, so a looser box is a
REFUSAL: a placement pair that is genuinely separated stops being certifiable,
a separated cyl×sphere pair becomes `FallbackExtentUnsupported`, and an
instance genuinely outside another becomes `CensusUndecidable` — the
interference class.

**The count is the load-bearing part.** S66 itself said *"two of its three
consumers"*; I-d established the doors are four and that `ops.rs:1486` is a
third refusing one nobody had counted. `geom`'s copies of the sentence inherit
that error rather than merely repeating a phrase — they are false for the same
reason and by the same arithmetic, which is why this is one finding and not a
grep hit. `geom` also cannot see its consumers (`topo` depends on it, not the
reverse), so the honest fix is almost certainly to state the CONTRACT (a
superset, erring outward) and stop characterising what looseness costs, rather
than to recite a door list one crate below the doors.

**Not fixed by #876**, which is Track I's and does not edit `crates/geom/`.


## S234. The door inventory computes the roster's KEYS and none of its content — the direction column, which is the whole argument

**Raised by I-d (#876) as its own guard's disclosed blind spot, and
lifted out of that list deliberately.** `boolean/boxes.rs`'s
`every_door_that_reads_a_box_is_inventoried` walks `topo/src` and pins,
per file, every call of `face_box` / `face_box_rule` / `edge_box` /
`edge_box_rule`. It computes **where** the doors are. The module header
it guards makes a claim about **what each does with looseness** —
`reduce.rs` prunes, `separation.rs` grants on non-overlap, `ops.rs`
refuses unless the box clears, `census.rs` arm 2 refuses on a containing
box — and **nothing computes that column.**

**Why it is a finding and not a disclosure.** The direction column is the
entire content of the header's argument; a door roster without it is a
grep result. So the row computes the half that was never in doubt and
recites the half that is. A door that changes its reading **without
moving** — `reduce.rs` growing a second use that grants rather than
prunes, `ops.rs`'s scan being rewired — leaves the header's dispositions
false and the guard green. That is **S66's own shape one level up**, in
the fix for S66.

Worse in one specific way: the guard's assert message **instructs the
next author to hand-write a direction into the module docs**. It mints a
kept-in-step-by-hand invariant in the same diff that removed one, and
per Q6 a disclosed deviation owes an issue number or a named unit. This
row is that owner. **On Track I this is the third declared blind spot to
come back as a finding.**

**What would close it, and it is writable today.** One row per door that
*executes* the direction: widen the box the door reads and assert the
door's verdict moves the way the header says. #876 demonstrated the
mechanism by hand for two of the four while proving its own rows could
red — `face_box → [-1e300, 1e300]` reds five rows in `topo/tests/`
(prune-side), and census's `reach_box` widened axially reds
`a_body_above_the_cylinder_is_still_cleared_by_containment`
(refuse-side). Four such rows would make the header's column a computed
claim rather than a recited one; the guard above would then pin **where**
and the rows would pin **which way**.

**Not fixed by #876**, whose scope was the sentence and the ceiling rows.
Three blind spots remain in that guard's disclosure list and belong
there: an out-of-crate door (impossible today — all four functions are
`pub(crate)`), a door reading through a wrapper defined in `boxes.rs`,
and a call spelled through an alias, re-export or macro.

## S235. The exact conic box exists, is public, and has no production caller; `topo` re-derives a looser one by hand

**Raised by I-d (#876) while measuring S66's second over-width.** S16's
class, a fourth instance nobody counted, and it **outlives #862's fix**.

`geom::curves::boxes::circle_arc_aabb` (and `ellipse_arc_aabb`) computes
the conic's true per-coordinate amplitude `Aᵢ = √((û_i·a)² + (v̂_i·b)²)`
through the outward `Brk` bracket, **and** restricts it to the certified
span via an extremal-angle interval — so it is tighter than
`topo::boolean::boxes::EdgeBoxRule::ConicAmplitude` on **two** counts,
orientation and span. It is `pub`. It takes exactly the two parameters
`geom_brep::EdgeCurve::params()` already hands `edge_box`. `git grep` for
its callers returns `crates/geom/tests/curves/boxes.rs` **and nothing
else.**

Meanwhile `edge_box` hand-derives `|û_i|·a + |v̂_i|·b` — the
triangle-inequality bound over the same quantity, span-blind, and **not a
function of the locus**: two `Curve3::Circle` values describing one
circle with `u_ref` rotated in the plane get boxes `r√2` against `r`
apart. It is not latent. Measured through the public API on
`s16_box_soundness.rs`'s extruded three-arc `cylinder()`, **four of six
carriers** take the wide branch at factor 1.366, so that body's cap faces
claim `x, y ∈ [−0.683, 0.683]` against a true `[−0.5, 0.5]`.

**This is not "someone should write the tight version".** The tight
version ships. The finding is that there are two constructions of one box
and **the correct one is the unused one** — which is exactly S16's
subject (*"Three face bounding-box constructions with three different
soundness rules"*, FIXED by #620, which unified two of three at the
SURFACE level). This is the curve level, and it was not in that count.

**The two halves separate, and only one of them is #862's.**

- **Correctness / tightness → #862.** The axial over-width is a
  *deletion*. The conic amplitude is a **tightening**, which carries the
  obligation `EdgeBoxRule`'s NURBS bullet already states in writing: it
  would start pruning pairs that are examined today, so a rung-3 operand
  gate has to admit the kind first. Adopting `circle_arc_aabb` also
  changes the box's SPAN behaviour, not only its width — a larger
  behavioural step, worth landing separately.
- **Structure → this row.** Even after both land, *why are there two
  constructions and why was the correct one the unused one* is
  unanswered, and answering it is what stops a fifth from appearing. A
  row that lives only on #862 retires when the defect does; the
  duplication would not.

**Not fixed by #876**, which documented the arm, pinned the current
formula so tightening it is loud, and changed no arithmetic.


## S212. The certified door's postcondition now has one home, and at least three doors re-derive it without reference to it

**Raised by H-a (#880) on closing S86**, out of Track H's `S210`–`S229`
block. **Re-derived against `main` at merge time and renumbered**: this was
drafted as `S210`, which #875 (H-d) took while #880 was open — the block
protects against other tracks, not against a sibling lane, which is
G-R13's point exactly.

#880 put a postcondition on `CertifiedEnclosure`: **a `Some((lo, hi))`
never carries a NaN end** — a bracket that is neither a claim nor a
refusal may not leave a certification door. That rule was already being
enforced, correctly, in at least three other places, each of which
open-codes it and none of which references the trait:

- `topo/src/census.rs:1338` — `reach_box`, whose placeholder arm carries
  **twenty lines** of comment deriving the rule from scratch: *"Folding it
  would return `Some((NaN, NaN))` — a box that is neither a claim nor a
  refusal"*, then walks through what a NaN margin does to the caller's
  sign decision. That is the trait's sentence, re-argued, in another
  crate.
- `geom/src/curves/boxes.rs:129` — `extremal_angle_interval`, a four-way
  `is_nan` guard returning `None`, over the private `Brk` newtype, which
  carries brackets and has no certified door of its own.
- `topo/src/chart_region.rs:839` — `exact`, which admits a coordinate only
  when it is a point bracket **and** finite, so it refuses NaN as a side
  effect of a stronger test.

**This is a missing single home, not a bug.** Each site already answers
correctly; the sweep for S86's *defect* found no siblings, and the sweep
for its *invariant* found these three. The cost of the present state is
that the rule is stated in four voices, one of them normative, and nothing
connects them — so a fifth door gets no help, and a change to the
normative one propagates to nobody.

**Not taken in #880**, and the reason is per site rather than one reason:
`census.rs` is **Track I's ground** and `boxes.rs` was **live under H-d**
(rostered on that exact file) while #880 was open; `chart_region.rs` is in
**neither track's scope** and has no owner at all — which is `S230`'s and
`S231`'s shape, one file over. Editing another lane's open file to
centralise a rule is how a fix mints the collision it was meant to avoid.

**Related to `S210`, and deliberately not folded into it.** `S210` is
about a *bound* nothing watches — sole `T: Bounds` bracket extraction, with
a gate that must not fire on it. This row is about a *postcondition* that
has one normative home and three unlinked restatements; the doors here all
answer correctly, so there is nothing for a gate to fire on. They are
siblings in the *a rule with no instrument* family and a taker of either
should read the other — which is `S210`'s own *"this has a twin on the
register, and they were unlinked"* observation, applied to itself.

**What the work is**, so it is not re-derived a fourth time: give the
postcondition one referenceable home on the trait (it is written there
now), then point the three sites at it and delete the local derivations
that duplicate it — keeping each site's *own* argument for anything the
postcondition does not say. `Brk` is the one that may need more than a
doc link: it is a bracket carrier with no certified door, so the question
of whether it should have one is a design question, not a doc edit.

## S213. `real.rs` credits the M7-8 lane with a technique it does not use, and the false half is the generalisable one

**Raised by H-g on implementing `S90`'s ruling.** Drawn from Track H's
`S210`–`S229` block after re-deriving against `main`, which is what the
track's constitution asked for and which turned out not to be a defence:
H-e minted `S213` for a different subject on an unmerged branch at the
same time, and neither lane could see the other. The collision was
resolved by the orchestrator, who now **allocates** Track H's numbers in
`docs/SMELL-H-LOG.md` rather than lanes drawing them — this row keeps
`S213`.

`crates/geom-core/src/real.rs`, the M7-8 entry of the `Bounds` scope
rule, says of `geom_brep::EdgeNurbsLane`:

> *"…it is precisely what keeps `Bounds` out of `topo`'s signatures: the
> attach and validate doors take the lane as an injected function, so no
> `topo` API grows a bracket bound."*

**The second clause is false as a description of the technique.** What
`topo/src/euler.rs:2173-2199` actually does is open a **separate door**
carrying the lane bound on its own impl block —

```rust
impl<T: geom_brep::EdgeNurbsLane> Body<T> {
    pub fn set_edge_curve_nurbs_lane(...) {
        self.set_edge_curve_via(edge, curve, Self::certify_edge_spec_nurbs_lane)
    }
}
```

— and the `_via(..., f)` injection is how the shared machinery is
parameterised *behind* that door, not how the bound is avoided. The
euler.rs comment beside it is accurate and says the opposite of the
real.rs sentence: *"Raising the whole Euler surface to that bound would
push it through hundreds of `T: Decide` signatures … so the lane is a
SEPARATE DOOR onto the same shared machinery."* **Injection moved the
bound onto a narrower signature; it never removed one.**

**Why this is a finding and not a typo.** The true half — no *default*
`topo` door grows a bracket bound — is a claim about this repo's API.
The false half reads as a claim about the *technique*, and that is what
a reader carries off: *injection avoids a bound*. It did exactly that
here. Implementing `S90`'s ruling turned up a door reachable from
`editor_core::eval::evaluate`, and the lane went to its orchestrator
offering "function injection" as an option that keeps the bound off
`evaluate` — sourced from this sentence — which cost a round trip
before reading `euler.rs` showed there is no such technique in the tree.
A ratified doc that misdescribes a mechanism is worse than one that
omits it, because the omission does not get quoted.

**The fix** is one sentence, and it is in H-g's PR: say that the lane is
a separate door whose own impl block carries the bound, and that
injection parameterises the machinery behind it.

**The sentence says "the attach AND VALIDATE doors", and the validate
half is a second instance by a different mechanism** — stated here
because a fence that covers only the half being fixed is the S8 failure
mode. `topo::validate_geometric<T: crate::props::PropsQuadLane>` carries
the obligation on its own signature too, and it reaches there by
SUPERTRAIT: `PropsQuadLane: … + geom_brep::EdgeNurbsLane`. Nothing is
injected. That spelling is the one `bounds-allowlist.sh`'s **KNOWN GAP
2** names as invisible to the gate's grep, so this half is both a
mis-description and a bound the instrument cannot see — a worse pairing
than the attach half, and NOT fixed by the one-sentence correction,
which only stops the doc claiming a technique. **Left open deliberately;
a taker should read it beside KNOWN GAP 2 rather than as a doc edit.**

**What is NOT claimed:** that the other three lanes' entries
misdescribe themselves — they were read and they do not. This is one
sentence about one lane, with two doors under it.

## S215. The tree's only oblique-axis bit-exact rotation test cannot fail for a change inside the rotation

**Raised by H-e (#885) on closing D109(a)**, out of Track H's `S210`–`S229`
block. **Renumbered `S213` → `S215` by the Track H orchestrator before merge,
and the reason is worth more than the number.** Not scheduled — recorded, not
claimed.

**Why the collision happened, since re-deriving did not prevent it.** This was
minted as `S213` after re-deriving against `main`, where `S210`–`S212` were
taken and `S213` was free. **`smellh/h-g-doors` (#886) had already minted its
own `S213` about two hours earlier**, and `S214` was open on the orchestrator's
own branch. Both lanes checked correctly and both were wrong, because **neither
branch was merged**: `main` cannot show a number that only exists on a sibling's
unmerged branch. G-R13's fourth rule — *re-derive against `main` after every
merge* — caught `H-a`'s collision this morning only because the colliding PR
(**#875**) had already **landed**. Here nothing had, so the rule that saved
`H-a` could not fire at all.

**The published block is the wrong instrument for this.** `S210`–`S229` protects
Track H from other tracks; between two open branches *inside* Track H it does
nothing, because a reservation the reserver alone can see is not one — which is
G-R13(b) stated about a block instead of a lane. **The disposition: the Track H
orchestrator now allocates numbers on request and publishes each reservation in
`docs/SMELL-H-LOG.md`, which lands on `main` far more often than any lane branch
does.** That moves the reservation to where the other party actually looks,
which is the only thing that could have prevented this.

`editor-core/tests/asm2a_instantiate.rs:1051` —
`r1_the_placement_frame_matches_the_transform_node_bit_for_bit` — sweeps four
axes including `[1.0, 2.0, 3.0]`, *"non-unit, oblique"*, and asserts
`to_bits()` equality. It reads exactly like a pin on `Mat3::rotation_about`'s
output at an oblique axis, and **it is the only such row in the tree.** It is
not one. Its expected side is `eval::wire::wire_transform`'s expression
re-spelled — *"verbatim"*, in its own comment — so it calls
`Mat3::rotation_about(unit, angle)` itself and both sides move together. Any
change **inside** the rotation is invisible to it.

**How this was found, which is the part worth keeping.** #885 reassociated
`rotation_about`'s diagonal from `((t·nᵢ)·nᵢ) + c` to `(t·(nᵢ²)) + c` — an
`f64`-visible change on 34.6% of random (θ, axis) pairs — and the entire tree
stayed green. Part of that is genuine (every rotation on a committed
artifact's path has axis components in `{0, ±1}`, where the two associations
are identical). But **the one test positioned to catch the rest could not**,
and its name, its axis list and its `to_bits()` assertions all say otherwise.

**The test is not wrong and should not be deleted.** It correctly pins what it
says in its heading — that `Frame::rotate_then_translate` and the `Transform`
node agree bit for bit, including for a non-unit axis, *"the case the claim
used to get wrong"*. That is an **agreement** property between two callers, and
for that job re-spelling the callee is the right construction. The finding is
that the row reads as a **value** pin and is shelved among value pins.

**This is `S110`'s class** — a suite that cannot go red for the defect it
appears to cover — **in the one place where it mattered for this change.**
`S110`'s existing members are about assertions too weak to discriminate; this
one is about an oracle that is a copy of the subject, which is the sharper
form: no strengthening of the tolerance would help.

**What the work is.** Either say at the site that the oracle is a re-spelling
and therefore blind to the callee (cheap, and enough to stop the next reader
mistaking it), or add a genuine value pin for `rotation_about` at an oblique
axis. **#885 added the second for the diagonal specifically**
(`mat.rs::tests::rotation_diagonal_takes_the_square_before_the_scale`, which
reds on the reverted association), so what remains here is the off-diagonals
and the doc note — not the whole gap.

**Ownership.** `editor-core/` is in **neither Track H's nor Track I's scope**,
so this row has no home track today — the same shape as `S230`/`S231`. Recorded
rather than taken.

## S216. The repo has ~39 `compile_fail` rows and not one verifies what it claims

**Number allocated by the Track H orchestrator** (`docs/SMELL-H-LOG.md`),
not drawn from the block by this lane: `main` cannot see an unmerged
sibling branch, so re-deriving against it is not a defence between two
open lanes inside one track. `S213` is this lane's; `S214` and `S215`
were held elsewhere when this was written.

**One finding, two halves, found six hours apart by two lanes, and
neither half is visible from the other.**

| where | rows | what is wrong |
|---|---|---|
| `crates/*/src/` (library targets) | **36**, of which **28 carry an error code** | collected and run, but **the error code is never compared to anything** |
| `crates/*/tests/` (integration targets) | **11** | **never collected at all** — `S214`, which is where that half is recorded |

**Read the two together or neither is sized right.** `S214`'s rows are
inert evidence that reads as executable; this half's rows execute but
verify less than they say. The union is that **no `compile_fail` row in
this repo currently checks its own stated reason.**

**Raised by H-g on review of #886**, and found in this lane's own new
rows, which is where it should be read from: the lane wrote *"a mistyped
path reds rather than passes"* in a PR body, and that sentence was false
about every `compile_fail` row in the repo.

**The measurement.** Two rows planted in `crates/geom/src/projection.rs`
on toolchain 1.97.0:

| annotation | what it actually emits |
|---|---|
| `compile_fail,E0277` | `E0308` (type mismatch) |
| `compile_fail,E0308` | `E0425` (undefined symbol) |

`cargo test --doc -p geom`: **`4 passed; 0 failed`**, no warning, no
diagnostic. **The code after the comma is not compared to anything.**
Reproduced independently by the reviewer and by the lane.

**The population of this half.** 36 `compile_fail` rows across the
library targets, of which **28 carry an error code** — 10 `E0599`, 9
`E0277`, 7 `E0308`, one `E0451`, one `E0382` — spread over 9 files.
Every one of the 28 asserts *"this fails, and here is why"* and delivers
only *"this fails"*. The 8 uncoded rows claim less and therefore
misstate nothing, but they verify no more.

**Why that gap is not academic.** `compile_fail` is satisfied by a
**typo**. A row that stops compiling because an item was renamed, a path
went stale, or a feature gate moved is indistinguishable from a row that
stops compiling because the guarantee it pins is still holding — and the
green is the same either way. These rows are load-bearing: they are the
only in-tree evidence for the sealed-trait matrix
(`profile/src/path.rs`), the key-type discipline (`topo/src/entity.rs`,
`body.rs`), and now the `CertifiedBounds` evictions (#886). Each is a
guarantee whose test cannot fail for the right reason.

**The other half, `S214`**, is eleven rows in
`geom-core/tests/{review_m0_pr2,review_m0_pr3}.rs` that rustdoc never
gathers, because it collects doctests only from targets with
`doctest = true` and an integration-test target is not one. Recorded
there, not restated here.

**THE FLOOR, and it is the part that changes what a fixer should do.**
Correcting every annotation does not close this, because for an
**inherent method no annotation exists that would distinguish the proof
from a typo.** `c.project(p)` at an evicted scalar emits `E0599`
(*"the method exists but its trait bounds were not satisfied"*), and
`c.projekt(p)` — a misspelling — emits `E0599` too. Only the **free
function** case can be pinned by code: `E0277` carrying *"required by a
bound in …"*, which is what `topo`'s `chart_region_overlap` row emits
and is why that one row is genuinely sound. So the 28 rows split again,
into *"could be pinned if the annotation were checked"* and *"could not
be pinned by any annotation."*

**Whoever takes this decides what the rows are FOR before deciding how
to spell them.** Annotating the inherent-method rows correctly still
documents the obligation and is worth doing — but **it buys no
verification**, and a sweep that stops there and reports the class
closed leaves a worse state than today, because the rows would then look
checked.

**What a real closure needs** is machinery, not a sweep: `trybuild`
(which compares stderr, not a code) or a gate that extracts each row,
compiles it, and matches the emitted code against the annotation.
Neither is in the tree — the repo has no `trybuild` dependency. **Not
takeable as a doc edit**, and sized here so the next lane does not start
one.

**What is NOT claimed:** that any of the 28 rows is currently passing
for the wrong reason. They were not audited one by one, and #886's three
were each confirmed to fail for their stated obligation and to go green
under `Dual64`→`f64`. The finding is that **nothing would notice if one
started passing for the wrong reason**, which is a statement about the
instrument.


---

# Findings raised by the Track F, G and I lanes (2026-08-20/21)

Raised by lanes of tracks that have since closed. Every finding below is open;
the lane that raised it is named in its own lead.

---

## S169. The loud stand-down has ten hand-rolled spellings, and three of them announce no coverage

**Placed by lane F-d (#825) at its stopping point, not left to evaporate.**
F4 gave the tree a home for the idiom —
`test_utils::vacuity::stood_down`, one line, `SKIPPED (label): what
this run did not assert` — and converted the sites under its own hand:
three in `geom-brep/tests/m5_pr7_ssi.rs`, including the local helper
`wall_stand_down`, which is now one argument's worth of vocabulary over
`stood_down` rather than a second implementation of it.

**The residue, re-derived on the merged tree rather than transcribed**
(`SKIPPED (` / `SKIPPED:` over `crates/*/{src,tests}`): **ten in-row
sites, four files.**

| file | sites | what they say |
|---|---|---|
| `crates/topo/tests/review_m6_2_probes.rs` | `:42`, `:78`, `:158` | three **byte-identical** copies of `println!("SKIPPED: FitSampleBudget stand-down at this ε")` |
| `crates/topo/tests/m6_2_fitted_at_rest.rs` | `:49`, `:138`, `:210` | one of them the same one-liner; the other two name the fixture |
| `crates/geom-brep/tests/review_m5_pr7b_ssi.rs` | `:168`, `:216`, `:229` | full sentences, naming the ε and the reason |
| `crates/mesh/tests/fitted_refusals.rs` | `:249` | names the fixture and the door |

**The three one-liners are the finding, and the other seven are
tidying.** *"FitSampleBudget stand-down at this ε"* announces that a
stand-down happened and nothing about **what the run therefore did not
assert** — which is the whole content of a loud skip. A reader of the
battery log learns that a row stood down and cannot learn what coverage
went missing, so the announcement is only marginally better than the
silence it replaced. Three byte-identical copies of it is also S13's
shape.

**Not a floors proposal.** C21's ruling stands: whether these rows
should be ε-conditional at all comes first, and D70 already carries that
question for the *silent* whole-row form. This row is narrower — these
sites already announce; the ask is that they announce **through one
door** and **say what was lost**.

**No edge today.** #790 (F-c) is merged and touched none of these four
files; no live Track F lane names any of them. `review_m5_pr7b_ssi.rs`
is inside F4's own `geom-brep/tests/` scope and F-d deliberately did not
convert it: its stand-downs are sound in content, and F4's finding was
about the tolerant *arm* at `:349`, which F-d examined and judged sound
for its own disjunctive claim. Converting prose that is already correct
was outside what that row asked for.

**Four further sites are a different idiom and are NOT in this
population**: the whole-binary
`interval_lane_skipped_no_certified_coverage_here` test
(`sweep/tests/{m5_s13_pips_interval,m5_s12_curved_ops_interval,m6_surgery_interval}.rs`,
`topo/tests/m6_2_fitted_at_rest.rs:171`), which `memories/test-suite-cost.md`
names by name and whose entire body is the announcement.

**What the grep could not match**: a stand-down announced without the
word `SKIPPED` — `eprintln!("standing down …")`, a `dbg!`, or a comment
where a print should be; one that returns green through a helper that
prints elsewhere; and `demos/`, `tools/` and `interval-transcendentals/`
were scanned and clean but are not this row's scope. **Ten is a floor,
not an enumeration.**

---

## S160. The split scan's constants can be guarded — on the continuous objective, which the cell count is not

**Placed by the orchestrator out of #783's F-R14 ruling, with lane F-b's
evidence, so the taker inherits the argument rather than re-deriving it.**

`tools/tess-meter`'s `SPLIT_SCAN_DECADES` / `SPLIT_SCAN_SAMPLES` ship with
**no mechanical guard and a written reason why** (S73's record, and the
constants' own docstring): the cell-count excess they produce is
**discontinuous in them**, moving ~4 percentage points between adjacent
sample counts — 321: 5.88%, 322: 3.64%, **323: 5.24%**, 324: 1.79%, 325:
3.94% — with no convergence (2,000 samples is still 0.79%). Two
instruments were built against that quantity and both failed; `323` is
the witness that killed the second, two samples above shipped and a
strict refinement.

**The guard that is possible is on a different quantity, and the
discontinuity is the reason it works.** The jumps are *entirely* the two
`ceil`s in `divisions`. Compute the same worst relative excess over the
same family **without** them — the cost as a continuous function of the
aspect ratio `t` — and it moves by **hundredths** of a point across the
same neighbours (321: 0.017%, 322: 0.083%, 323: 0.011%, 324: 0.030%)
and falls smoothly with resolution:

| samples (8 decades) | 65 | 161 | 200 | 321 | 400 | 1,000 | 2,000 |
|---|---|---|---|---|---|---|---|
| continuous excess | 1.82% | 0.449% | 0.096% | 0.017% | 0.0069% | 0.0034% | 0.0022% |

It is continuous because it is a sampled minimum of a smooth function of
`log t` with no rounding in it, so it depends on the sampling step
`2·DECADES/(SAMPLES−1)` and the range — **and on nothing else these
constants do not set**. It reds on both failure modes with one number:
too narrow (`DECADES = 2` → 10.44%, the optimum outside the scanned
range) and too coarse (`DECADES = 40` → 1.82% at the shipped sample
count).

**The validation, and it is the part that should convince a taker rather
than the argument above it:** `DECADES = 40` at 321 samples scores
**1.82%**, *identical* to 8 decades at 65 samples — the two configurations
that share a sampling step. A quantity that is equal wherever the step is
equal is measuring resolution, not which lattice the count happened to
land on, which is exactly what the cell-count excess could not do.

**Why it is not in #783.** F-R14 forbade a third instrument in a row that
had already shipped two; it needs `best_split_steps` parameterized by
`(decades, samples)`, which is code #783's brief marked read-only; and it
measures a quantity the budget CSV does not report, so it deserves its
own review rather than riding another unit's clearance.

**Scope:** `tools/tess-meter/src/lib.rs` (a parameterization, and the
constants' docstring, which currently says a guard on this quantity is
possible and unwritten) and `tools/tess-meter/tests/derivations.rs`.
**Sequencing:** the register half of **S120(c)** asks whether CI should
re-measure this and belongs with it — that gate structurally cannot see a
degraded scan, so if the answer is a scheduled re-measure, this is the
quantity it would measure. **Row: D105.**

## S174. S113(a)'s sweep stopped one paragraph short, in the file it edited

**Raised by G11 / #837 while regenerating the uv lane to check for
drift.** S113(a) is recorded FIXED by #787 and its record names the
exact corrections — *"982 → 1049 faces, 879 → 946 checkable, and
`9.4e-16` → `9.93e-16` for the worst 3-D loop gap"* — and says the
sweep reached `demos/README.md:346`. **Five of those same numbers are
still standing in `demos/README.md`, above and below the line the
sweep fixed**, measured on a tour run from this tree (35 scenes, 1049
face charts, 946 checkable / 103 branch-jumped / 0 disagree, worst 3-D
loop gap 9.93e-16 m, 285 curved faces, 48 sheet cells):

- `:176-177` *"879 of the 982 M7 faces are checkable … all 879
  agree"* → **946 of 1049**, and all 946 agree (`0 disagree`).
- `:186-187` *"103 of the 982 M7 faces show such a jump"* — **103 is
  right, 982 is not** (1049).
- `:188-189` *"the true closure gap never exceeds 9e-16 m anywhere in
  the corpus"* — **false as stated**: the measured worst is
  9.93e-16 m, and this is the same number S113(a) corrected at
  `uvdump.rs:294`.
- `:218` *"982 at M7"* and `:224` *"all 982 SVGs stay in `out/uv/`"* →
  **1049**, a number the composer PRINTS on every run.
- `:232` *"Of the 238 curved faces"* → **285**.

**Not claimed:** `:196`'s *"3 of 36 on the sheet"* and `:232`'s
*"234 have boundaries built entirely from iso-curves … only 4 do not"*
are almost certainly stale too — the sheet carries 48 cells now — but
neither is derivable from anything a run prints or `uv.json` carries,
so this finding does not put a number on them. Deciding what they
should say is part of the work, not part of the evidence.

**The disposition is #787's own rule, not a recount:** *compute or
delete, never restate.* The tour's closing lines already print every
figure in the first four bullets and the composer prints the fifth, so
the prose should point at the run and state the RULE, exactly as
`uvdump.rs` and `compose_uv_montage.py` now do. A corrected number
here is a number that drifts again on the next scene.

**Why it is not fixed in #837:** `demos/README.md` is outside G11's
scope cell, and the honest fix is an editorial pass over that whole
section — the half of it that can only be restated has to say at the
site why. A partial recount is §C13's half-fix. **Row: D123.**

## S196. Twelve `# noqa` markers for three linters, none of which the repo runs

**Raised by G11 / #837**, whose own fix pass added two of them before
checking whether anything reads them.

`git grep noqa -- '*.py'` returns **twelve markers in six files**, and
`git grep -il 'ruff|flake8|pycodestyle|black|pylint'` over `.github/`,
`local-scripts/`, `scripts/` and every `*.toml`/`*.cfg` returns
**nothing**. No Python linter is installed, configured or invoked
anywhere in this repo.

The markers are not one tool's, either — they name rules from three
separate namespaces, so no single install would even make them all
meaningful:

- **`E402`** (module import not at top) ×6 and **`E731`** (lambda
  assignment) ×1 — pycodestyle, all in `demos/render_freecad.py`,
  where the imports genuinely cannot move (the notification-area wedge
  must run before `FreeCADGui` loads) and the lambda genuinely reads
  better than a `def`.
- **`F401`** (imported but unused) ×3 — pyflakes, in the three FreeCAD
  probe scripts, where `import Part` is imported for its side effect
  of registering the workbench.
- **`S102`** (`exec` used) ×1 and **`BLE001`** (blind `except`) ×1 —
  flake8-bandit and flake8-blind-except, i.e. **ruff's** extended set.

**Why it is a finding rather than dead decoration.** Each marker is a
claim that a specific check exists and was consciously overridden, and
in every one of these cases the *reason* is the load-bearing part —
`render_freecad.py:48` sits above a fifty-line comment explaining why
the import order is what it is, and the `# noqa: E402` is doing none of
that work. A reader who greps for the linter to see what else it says
finds nothing at all.

**Two dispositions, and it is a small decision rather than a patch.**
Either the repo runs one Python linter (ruff would cover all three
namespaces, and `scripts/`+`demos/`+`crates/**/tests/*.py` is a small
surface) and the markers start meaning something; or they are deleted
and the reasons they gesture at stay in the prose that already carries
them. **Do not do half** — deleting some and keeping others is how
this got here. **Row: D122.**

## S230. Certified widths with no ceiling, in crates no live track owns

**Raised by lane I-b while closing S60 (#873).** The parent is **S26**, and
the class is S26's own lesson stated as a rule: *every certified width needs a
row that goes red when it grows.* This is not an S110 member — S110's class is
vacuous assertions in shipped test files, and a containment row is not
vacuous. It is a **live** assertion that happens to be monotone in the
degrading direction, which is the sharper and narrower thing.

**This row has no home, and that is part of the finding.**
`crates/editor-core/` and `crates/pncad-py/` are outside Track I's scope
(`props/`, `mesh/`, `census.rs`) and outside the scope of every other live
track. **Nobody is scheduled to fix this.** Recorded unrouted rather than
implied-owned; a reader should not infer a lane from its presence here.

**The class is five live members, and this row is the two that had no owner.**
The other three were in `crates/sweep/tests/`, which no lane owns either but
which #873 was already editing, so it fixed them there: `m6_loft_body.rs`'s
`Interval` row (folded both pads into its bracket and read neither),
`m6_tube.rs`'s `Interval` row (containment only on a bracket whose width is the
scalar's own, both quadrature pads being exactly zero on the closed-form torus
lane), and `mass_props_interval.rs`, where the admitted band was widened *by
the enclosure's own width* so every degradation satisfied the row twice over.
**#873 found the last of those by executing a blind spot it had itself
declared** — a width computed inline as `hi() - lo()` and never named — which
is the argument for declaring them.

Both members below carry the numbers I-b measured, because the measurement is
what makes them actionable. **Measured at `5d4b88ab`**, dev profile, x86_64
Linux, across CI's own ε matrix (`CAD_TOLERANCE_EPS` ∈ {default, 1e-6,
1e-12}). Note the unit: `area_pad` and `volume_pad` are **half-widths** — the
bracket is `value ± pad` — so each figure below is half the bracket it names.

- **`crates/editor-core/tests/m5_pr11_corpus_curved.rs:75`** — containment
  only on `volume_pad`, and it never reads `area_pad` at all. It runs the
  **same** tilted-cut fixture as `sweep/tests/m5_pr11_quad_props.rs`, whose
  volume row has carried a tightness ceiling since PR 11 and whose area row
  gained one in #873. On that fixture's below half the certified widths are
  `volume_pad` = 3.5356e-7 m³ and `area_pad` = 2.1214e-6 m² at default ε, and
  3.0780e-4 m² at ε = 1e-6 — a five-order spread across the matrix that this
  row cannot see in either quantity.
- **`crates/pncad-py/tests/test_north_star.py:555-556`, `:1238-1239`** — the
  Python door's rows on the shape (iii) loft. `:556` bounds `volume_pad` at
  `1e-6` and asserts **nothing** about `area_pad`; `:1238-1239` is containment
  only. On that same loft the measured widths are `volume_pad` =
  **1.0725e-13 m³** and `area_pad` = **0.1986 m²** — 7.8e-3 of the body's
  25.31 m² surface, eleven orders of magnitude apart, and **identical bits at
  all three ε legs** because that width is resolution-driven, not
  tolerance-driven. The Python door therefore reports a certified area of
  25.31 ± 0.20 m² with no row that would notice the pad growing.

- **`crates/editor-core/tests/review_m5_pr9_doc_probe.rs:151`** — the curved
  boolean's `Interval` union row, containment only on the volume enclosure,
  with nothing bounding its width. Same crate as the first member and the same
  disposition: out of every live track's ground.

The kernel-side reason the loft's area half-width is what it is — the area
enclosure is never metered — is **issue #870**, not this row. This row is only
about the missing ceilings.

**What would close it**: a ceiling on each, derived from a measurement, at the
sites named. The two `editor-core` rows and the Python rows are three separate
crates' test suites; nothing here is a kernel change.

---

## S231. `chords.rs`'s "the only places adjacent surfaces enter chord counts" is an unfalsifiable absence claim, and no lane owns the file

**[verified]** **Found by I-c (#872) while sweeping the class it was closing
— the `mesh` crate's ε ledger, recited in prose rather than computed. I-c is
NOT fixing this one, and as of this row no lane owns it.**
`crates/mesh/src/chords.rs`'s module header lists four chord-count
rules — line carriers, circle carriers, the adjacent-torus tightening,
the adjacent-NURBS tightening — and then closes the list with:

> These tightenings are the only places adjacent surfaces enter chord
> counts — chord points remain a pure function of (carrier + interval,
> endpoint points, adjacent surface parameters, δ).

**The claim is load-bearing.** The clause after the dash is the
memo-key contract's chord half, restated in `mesh/lib.rs`'s crate
header as *"the chord points themselves are a pure function of (edge
carrier + interval, endpoint vertex points, the adjacent faces' surface
parameters, δ) — adjacent surfaces enter only through the torus and
trimmed-NURBS boundary-step requirements, documented on [`chords`]"*.
So the claim exists at **two** sites, one of which delegates to the
other, and neither is checked.

**Why it cannot be falsified as written.** It is an absence claim over
a whole module: not *"these four rules are the four"* — which a reader
settles against the list directly above it — but *"nothing else in this
file reads an adjacent surface when it sizes a chord"*. A fifth
tightening added anywhere in the module's ~550 production lines makes
the sentence false and nothing in the tree goes red. It is **exactly the
shape #872 closed one file over** — the crate's other absence claim about what
a value may reach — and **that remedy transfers only halfway.** #872 replaced
the ε ledger's hand-written roster with a computed pin over `eps` identifiers,
which works because ε has a NAME a textual walk can count. *"Adjacent surfaces
enter chord counts"* has no such token: the two real sites read `get_face` /
`get_surface`, spellings that appear in four files for other reasons.
A count pin here would pin the wrong thing, so the remedy is an open
question rather than a transcription of the ε ledger's.

**What a reader should NOT conclude from this row.** Not that the
sentence is false — it was checked at `acfbfb9c` (`chords.rs`
unchanged since `5d4b88ab`) and it is **TRUE**: the file reads an
adjacent face's surface at exactly two sites, the `Circle` arm's
`torus_step` call in `compute_chords` and `nurbs_tighten`, which are
the two the sentence names.
Not that it is a duplicate of the ε-ledger finding #872 closed: that one is
closed, this is a different file, a different value, and a different remedy.
And not that anyone is working on it. The row exists so that *"the ε ledger is
fixed"* cannot be read as *"the `mesh` crate's unmechanized absence claims are
dealt with"*, which is precisely what a scan closing one instance of a class
invites.

**Ownership, stated because a row with no owner reads as one with an
implicit one.** `crates/mesh/` is inside **Track I**'s scope, but
`chords.rs` is in **none of Track I's five lanes' file sets** — I-c is
`{lib,sizing,walk}.rs` plus `curved.rs`'s header, I-e is `curved.rs`'s
guard bodies plus `{trimmed,planar,budget}.rs`. It is deliberately
**not** routed to I-e: widening a running lane's brief by writing a row
at it is how a lane discovers its scope grew after dispatch. This row
needs a lane, and does not have one.

**Verdict:**

---

## S236. `cert_cylinder` is falsified by nothing, in any build — and closing that is a contract question, not a coverage chore

**Raised by lane I-e while fixing S109 (#887); I-e is NOT fixing it, and no
lane owns it.** `crates/mesh/src/trimmed.rs`'s deviation pass reads

```rust
let dev_samples_per_edge = if matches!(lane, Lane::Nurbs { .. }) {
    crate::budget::deviation_samples()
} else {
    None
};
```

so the per-triangle resampling that produces `worst_ratio` runs on the NURBS
lane **only**. `cert::cert_cylinder` certifies every cylinder triangle in
**both** tessellation lanes (`curved.rs`'s `ChartKind::Cylinder` arm and
`trimmed.rs`'s `Lane::Cylinder` arm), and **nothing anywhere samples a cylinder
triangle against it** — not the meter, not `probe_review`'s Z1 row, not a unit
row in `cert.rs`, which has no test module at all. The cylinder certificate is
the one certificate in this crate with no empirical falsifier in any build.

**What #887 did and deliberately did not do.** It corrected the assertion
messages, which read as universals about triangles (*"a triangle's samples
exceeded its own certificate"*), to say NURBS, and stated the gap in
`the_deviation_pass_samples_and_stays_under_its_certificates`' doc. **That
makes the limit honest and leaves it in place.**

**Why this is a decision and not a five-minute fix.** The obvious remedy —
sample the cylinder lane too — means handing `budget::note_face` a
`FaceMeasure` whose NURBS-only columns have no meaning: `grid_cells`,
`cells`, `patch_steps`, `muu`/`muv`/`mvv` are all `NurbsCellGrid` and
`NurbsFaceBound` quantities, and a cylinder face has none of them. That is a
change to **#320's consumer contract** — `tools/tess-meter` and
`tools/tess-lint` read those columns per row — so whoever takes it is choosing
between at least three shapes: a nullable/enum lane discriminant on
`FaceMeasure` (every consumer grows an arm), a second hand-off channel for
lanes with no grid (two shapes to keep in step), or a falsifier that is not the
budget meter at all — a unit row in `cert.rs` sampling synthetic cylinder
triangles, which needs no contract change and does not measure the tessellation
lane's real triangles. **The third is the cheap one and the weakest; the row
exists so that is a decision someone makes rather than a default someone
falls into.**

**Ownership.** `crates/mesh/` is inside **Track I**'s scope, and this is inside
I-e's file set — but I-e is a lane that has landed, and a contract change to
`budget::FaceMeasure` reaches `tools/` which is in no Track I lane. **This row
needs a lane and does not have one.**

**Verdict:**

---

## S237. The `worst_ratio` ceiling CI actually runs is the one still monotone the easy way — three live instances, not one

**Raised by lane I-e while fixing S109 (#887); I-e is NOT fixing it — one of
the three is outside the lane's file set and two are in a file it never
opened.** The class is **four instances, one fixed**: I-e's own sweep saw two
and #887's adversarial review found the other two, which its declared blind
spot (*"a bound spelled another way"*) would not have caught **because they
spell it the same way** — the sweep simply undercounted.

| site | state |
|---|---|
| `crates/mesh/tests/budget_meter.rs` | **FIXED by #887** — floor added, gated on `worst_cert > eps` |
| `crates/mesh/tests/probe_review.rs::z1_per_triangle_certificate_falsification` | open — **the row hosted CI runs** |
| `crates/mesh/src/nurbs_cert.rs:2167` | open — same `d/cert` accumulation, `assert!(worst_ratio <= 1.0, ...)`, inside `#[cfg(test)]` in `src` |
| `crates/mesh/src/nurbs_cert.rs:2553` | open — the R1 extreme-weight row, and its assertion carries **no message at all** |

The rest of this row is about the `probe_review.rs` instance, which is the
load-bearing one; the two `nurbs_cert.rs` rows are the same shape at unit
scale, over synthetic patches rather than tessellated bodies, and whoever takes
this takes all three.
`crates/mesh/tests/probe_review.rs`'s `z1_per_triangle_certificate_falsification`
asserts, over four fixtures at two deltas:

```rust
assert!(f.worst_ratio <= 1.0, ...);
```

`worst_ratio = d / (bound + eps)`, so **the assertion gets EASIER as `bound`
grows** — a certificate loose enough to be worth #320's attention passes it by a
wider margin than a tight one, and the row reports green most confidently
exactly where the budget question is sharpest. #887 fixed this shape in
`crates/mesh/tests/budget_meter.rs` by adding a measured **floor** beside the
ceiling, applied on the faces where the certificate rather than ε is the
denominator (`worst_cert > eps`).

**State the asymmetry plainly, because it inverts the intuition.** The sibling
that was FIXED (`budget_meter.rs`) is the one CI runs at **default ε only**.
The one still monotone (`probe_review.rs`) is **the row hosted CI actually
runs as its certificate falsifier** — ci.yml's *"mesh budget meter +
certificate falsifier (feature = budget)"*, mirrored by
`local-scripts/ci-local.sh`. The weaker check is on the more load-bearing row.

**A floor here is feasible and has a measured starting point.** #887 measured
this corpus' per-face **maxima** at all three ε legs, on `loft_prism`,
`nonuniform_loft`, `swept_elbow` and `rational_pie` at δ ∈ {3e-2, 6e-3}:
**0.363–0.500**, stable to three decimals across the ε legs. What a floor needs
is the per-face **minima**, which #887 did not take here (it took them on
`loft_prism` alone, through `budget_meter.rs`: **0.1667–0.4966 over
δ ∈ [1e-4, 1e3] at four ε legs**, bottoming at 1/6 once the sizing reaches its
coarsest grid — note that #887's first pass reported 0.454 as the minimum from
a 66× δ band, and both its reviewers were right that the band, not the
population, produced that figure). Whoever takes this row measures the minima over the four
fixtures and picks one floor, with `worst_cert > eps` as the applicability
test — `loft_prism`'s planar wall certifies at ~5e-17 and its ratio is decided
by ε outright (5e-7, 5e-10, 5e-4 at the three legs), which is the case a bare
floor would red on.

**What a reader should NOT conclude.** Not that the Z1 row is weak — it is the
row that killed a planted `0.25 -> 0.05` certificate bug empirically. Only that
it is one-sided, in the direction #320 exists to measure.

**Ownership.** `crates/mesh/tests/probe_review.rs` is in **none of Track I's
five lanes' file sets** — the same shape as **S231**, and routed the same way:
not to I-e, because writing a row at a dispatched lane is how a lane's scope
grows after dispatch. **This row needs a lane and does not have one.**

**Verdict:**

---


---

# §D. The schedule

> **READ THIS FIRST. Every track A–I is closed, and what they left is
> repartitioned into [Tracks J–X](#tracks-jx--the-repartition-2026-08-21),
> which is the only live schedule.** A, B, D and I completed; C, E, F, G and H
> stopped with rows outstanding. Their tables and lane records are deleted —
> `docs/SMELL-{C,E,F,G,H,I}-LOG.md` are the execution record, and the rulings
> those tracks made are cited from here by number (`F-R11`, `H-R2`, `I-R8`, …)
> and are read there. **109 open items** are carried below, partitioned by file
> territory so that no two tracks edit one file and no branch waits on, fences
> against, or re-derives another's scope.

**Live rows only.** Completed work is **not** listed here, and neither is a
note saying it completed: a landed unit is documented by its merged PR, and its
row and its finding both leave this file. What follows is therefore what is
*left*, and its length is the honest measure of that.

The wave numbering the schedule started with (Waves 0, 1, 1b) is retired: it
encoded a dependency structure that has been discharged, and what remains is
organised by **who can take it without colliding** rather than by how it was
originally batched.

Four ordering rules survive every reorganisation, because all four still bind:

1. **Decide before you delete; delete before you polish.** Comment trimming
   (S38) and test-suite combing (S36) come last — both operate on files whose
   fate earlier rows have not settled.
2. **A finding whose steelman said SURVIVES IN PART is scoped by the steelman,
   not by the original finding.** Several shrank materially under scrutiny.
3. **A lane's own residues are rows, not footnotes.** Many rows below exist
   because a fix pass or a review found something its own PR could not carry.
   Recording them as prose inside a merged PR body is how they get lost.

4. **A verdict is not a placement.** A finding may leave a review with
   ACCEPTED, DISPUTED or DECIDED and no row only if the verdict is *closed*.
   Everything else owes a track row, a decision row, or a `FIXED by`, written
   in the same PR that records the verdict — because accepting findings in
   batches gives the batch's leader a lane and its siblings a verdict and
   nothing else. An audit of all 56 finding IDs against this schedule
   (2026-08-20) found eleven findings lost exactly that way, with a verdict and
   no row anywhere; that is where the rule comes from.

---

## Open decisions — Evan only

| # | Decision | Gates |
|---|---|---|
| **D6** | **D5's contract is still untyped at two more doors.** #665 typed `enters_material` and `sector_shape`; a differently-shaped sweep (`grep sense_sign`) reaches the rest. This is a schedule, not a sentence — the question is how far the newtype goes, not whether it was right. | nothing hard; colours the sense-carrying surface |
| **S14** | **What the no-panic principle actually says.** Evan's own reframe, 2026-08-18: *"maybe we need to update that principle to 'no panic on any reachable state, yes panic on things that can only indicate bugs'"*. The steelman split it — the first half is a **clarification** (D9 already says "on any input" and no existing `debug_assert` moves); the second is an **amendment**, because it licenses panics in release, which D9 does not, and on the one such class D9 disposes of it chose typed error or garbage-out. The reframe is also already in the tree unnoticed: PR #447 argued for panicking indexing on the merits and never took it back to D9, while `crates/topo` was ratified the other way. And the honesty defence for `hull.rs:80` fails on reachability — two clamped `KnotVector`s of equal degree and different length, `long_kv.span(k)` handed to a curve built on the short one, indexes out of bounds through the public API with no kernel bug in the trace. Issue **#475** costs out Options A/B/C and misses the cheap third (`kv.span(span.index()) == Some(span)`, O(1), the deleted guard exactly). **Second witness (added by #713, D5).** `topo::instance`'s graft is a public door that can leave a body **tier-1-invalid**: `graft_disjoint_all_keyed` mints an empty destination solid per source solid before transplanting, and its own docs state that a refusal raised mid-transplant leaves `dst` partially written and *spent, never resumable* — an empty solid being `SolidWithoutShells`, a tier-1 error. So a caller that discards the `Err` and keeps the body makes the next Euler operator's `debug_assert` fire from **API misuse, not a kernel bug**, which is precisely the class D9's footnote asserts cannot occur and which S43's proposed sixth state class named and the ratified five do not cover. It is the same question as `Span`'s, one crate over and through a door that already concedes the state in writing — where `Span` needed a somewhat contrived pairing to reach, this is a documented failure mode of a shipping API. #713 recorded the exception at both sites (`euler.rs`, `DESIGN.md`) and proposed no fix. **Correction, #740 (D2):** the door's `# Errors` section named `GraftRecertify` as the mid-transplant refusal, and **that variant cannot be raised at this door at all** — both public `instance` doors bridge with `combine::Bridge::RemapKeys`, whose arm never reaches the only site that raises it. The witness is unharmed: the mid-transplant refusal these doors *can* raise is `JoinDesync`, from the reference remap, and it writes as it goes just the same. #740 corrected the doc. **Third witness, from the other side — RETRACTED by #768 (D27):** executing the addendum over `sweep/src/fillet` produced a state none of the five classes fits — `FilletError::EmptyChain`, neither reachable by input nor locally provable. It is not a witness for this row after all: the state was representable only because `Chain` held its links in a `Vec`, and #768 removed the representation rather than adding a class. **The distinction that leaves is the one this row still turns on, and it is now a question in front of Evan rather than an answer** — `EmptyChain` was a state a type could stop spelling; the graft class above is a state a public door genuinely produces, and no type change removes it. Whether *"can the type stop representing it?"* is therefore the FIRST question at a site of this shape is now **ratified as row 0** of the D2 addendum (Evan, 2026-08-20, #777). **Row 0 reframes this row's first question and does not answer it.** What it asks of the graft class before any classification is: *can `graft_disjoint_all_keyed` be restructured so a partially-written destination is not representable* — staging into a fresh body and committing on success, the shape `merge_coplanar_faces` already uses in the same crate (`merge_faces.rs:468`, `let mut work = self.clone()`, under its own *"Never a partial commit: each sub-stage is tier-2-gated before adoption"*). Whether that restructuring is affordable is exactly row 0's *"if possible"* judgement, and it is **Evan's**, unchanged by #777 — which is why no row was minted for it. **If the answer is "yes, restructure it", that is a row worth minting at that moment**, and it moves the 46 lookup sites #740 left typed because this question is open. And the practical bite of this row is now measurable: #740 left 46 lookup sites as typed errors rather than `unreachable!` **because this question is open**, so S14 is no longer only a taxonomy gap, it is a bound on how much of the kernel can be converted. | This is a **decision, not work** — it was the one row of *Accepted, unscheduled* that had no channel at all. **It has one now: #823**, which splits the row into **S14(a)** (the `Span` pairing) and **S14(b)** (the graft), executes the reachability claim as a running test, re-derives every option's cost at its own base, and leaves the decision to Evan. Nothing in Track D touches it. |
| **S22 row 1** | **ε ambience** — *settled 2026-08-19, half of it reversed 2026-08-21*: keep the `OnceLock`, add provenance (#659), no session object, no mixed-ε assemblies — all standing. The *no threading* half is **reversed**: ε is threaded at every call site as a zero-sized `Tol` witness, which is not the value-parameter design the ruling rejected. See **S22 row 1 REVISED** above. Listed here only because the row's *other* halves are closed and the finding should not read as open. | — |

**D1 was ruled 2026-08-19** (a `Dual` may not certify, but it may have
`Bounds`; M10/E4 remains the plan) and has landed — see S44's **D1 DECIDED**
entry for the ruling, the impl, what newly admits a dual, and the two residues
it left (`ContentBits for Dual`, issue **#687**; and the `sweep::fillet`
seam's standing lane obligation). Note that it does **not** discharge S3: the lane-trait collapse was
derived against the one-trait world and needs re-deriving against #643's.

---

### Decisions only Evan can make, from the second scan

These are **not** work and no lane may resolve them; they belong to the table
above rather than to any track. Two rows near the end are **already ruled** and
are kept because the ruling carries conditions a taker of a live row still has
to honour.

| # | The question |
|---|---|
| **S65** | **The #678 watertightness backstop is `#[cfg(debug_assertions)]`, so it is absent from every build that ships a mesh**, and the module header presents floor and assert as a pair without saying one is absent from release. Either it pays an O(triangles) per-patch re-derivation in release — against D9's *never a panic* and against tessellation cost — or it stays debug-only and the header says so. Both are defensible; the finding must not resolve it silently. |
| **S70** | **`DESIGN.md`'s ratified graft footnote is documented-as-false in a source comment**: `euler.rs:84-91` says *"All three understate it"* while `DESIGN.md:1131-1140` carries the weaker `SolidWithoutShells` claim. Either the graft gets real atomicity and the footnote stands, or the footnote is corrected to state the stronger failure and the door is documented unsafe-on-`Err`. **This is S14, one crate over, and S14 is now a bound on other lanes rather than a taxonomy question** — #740 left 46 lookup sites as typed errors because it is open (re-derived at #823's base as **45** announcement sites under a stated criterion, and the figure belongs to **S14(b)**, not to the `Span` half). **Channel: #823**, which splits S14 into (a) `Span` and (b) this door, and names S70 as (b)'s documentation residue. |
| **S82** | **The sphere rim predicate's lever understates in the *accepting* direction near the poles** — `RimLevel::Unit(sin v, 0)` makes the margin axial, so near-polar distinct rims decide `Zero` and the predicate accepts. Two documents and a test file already say so and both file it as *conversation input*; the audit table still marks the row `OK`. Is this a #723 sibling that needs an issue and a row, or genuinely typed-margin-conversation input that can wait? |
| **S90** | **The largest D1 residue is the only one without a schedule.** It says a lane is owed *"on the **public** surface"*, and every smaller residue got a number. Is that lane owed now, is the written reason sufficient, or is the verdict closed — and if closed, does §D's *"a verdict and no row only if the verdict is closed"* rule apply to it? |
| **S107** | **The `DimensionError` untangling renamed the Rust type and left the Python-visible confusion in place**, now defended in prose. Is that a defect or a deliberate compatibility choice — and if deliberate, may the tree stop re-documenting it? |
| **S116(p)** | **`MultipleAxisRuns` changed what the kernel promises**, from *"deferred to M3"* to *"a **permanent** refusal under the ratified sweeps-vs-voids invariant"*. That rests on an unstated geometric claim — that every profile with ≥2 disjoint on-axis runs encloses a void when fully revolved — with no test, no proof, and the reporting agent's own confidence at `unsure`. It is a promise already shipped to callers. |
| **D111 — RULED (Evan, 2026-08-21): run all fourteen; the two collection runs stay** | **Placed by Track F's F8 (#844), which found the premise of the ruling it was executing to be false.** Evan's F-R5 split *"thirteen `--ignored` dump harnesses (posture) versus one accidental plain test (run it)"*. Re-derived: of 17 censused probe-gated suites **exactly two carry any `#[ignore]`d test, and both are already executed**. The unexecuted **fourteen are plain `#[test]`s**, unrun because nothing runs `cargo test -p <crate> --features probe` at all — and several are not dump harnesses but the **Probe-lane halves of ordinary suites** (`profile/tests/validate_ok_probe.rs`, `geom-core/tests/k_stats_doors.rs`, `geom-brep/tests/span_meter_dim_twins.rs`), split out purely because `probe` monomorphizes every generic-over-`Real` body — **a compile-cost decision that silently became an execution decision.** The ruled disposition was applied where it fit (both accidental tests now run); **the population it was ruled over does not exist**, so the fourteen are undecided rather than decided-as-posture. **Cheap on one side:** CI's `compile and list every probe-gated test target` step already builds each crate `--features probe --all-targets --no-run`, so the marginal cost is execution only. **A decision anyway:** it is `k-lint`'s wall clock, and F-R5's ~2.1 min slack was one sample offered as an argument for placing *one* test, not a licence for fourteen. **And not free in the other direction:** none has ever executed, so a red is as likely to be a kernel finding as a harness one — so expect reds, and expect some to be kernel questions rather than harness ones. The live instance is **S110(a)**. **RULED: run all fourteen. The two `#[ignore]`d collection runs stay.** Evan, 2026-08-21.

**And the term that produced the confusion is retired with it.** *"Dump harnesses"* came from the finding, was quoted into F-R5, and described a population that does not exist — then went on being used after F8 refuted it. There are exactly **two** collection runs, `editor-core::m4_pr8_k_probe::dump_corpus_k_samples` and `sweep::k_report`: they **produce data rather than assert anything**, are `#[ignore]`d because they run one process per ε, and **are already executing**. They are disjoint from the fourteen, so cutting them would not have touched D111's population.

**They stay because they are `k-lint`'s only input.** `k-lint`'s CLI is `k-lint <k-probe-csv>…` and those CSVs come from `k_probe_sweep.sh`'s `run_dump`. Retiring them retires the K-telemetry gate — `K = 10` is the ratified default and `docs/k-report-data/`'s committed baseline is honest only while fresh rows are linted against it. That is a decision about the gate, not about test hygiene, and nothing in D111 argues for it.

**The work, for a taker.** Compilation is already paid by the `compile and list every probe-gated test target` step, so the marginal cost is execution only. **Turn all fourteen on at once.** An earlier draft of this row advised landing them in batches *"so a failure is attributable"* — **that reason does not survive inspection and is retracted**: CI names the failing test, so attribution is free either way, and batching buys nothing while costing extra round trips. What is true and worth knowing is only that **none has ever executed, so expect reds, and a red is as likely to be a kernel finding as a harness one** — a kernel fix is a different lane from a harness fix, and one run over the whole set tells you which you have. F8's floor (`probe-suite-census.sh --check-executed`, keyed on tests that **ran** and on the `plain.ignored == ignored.passed` complement) is the mechanism to extend as each batch lands. |
| **The scaled square — RULED YES (Evan, 2026-08-21)** | **Raised by Track F's F-g (#849), which stopped at the boundary rather than deciding it.** The interval-square gate forbids `x * x` because the general multiply must consider four endpoint products and cannot exploit `x·x ≥ 0`; the tight square is never wider and is **strictly tighter when the enclosure straddles zero**. F-g converted `linalg/vec.rs`'s `orthonormal_basis` `b2` (`self.y.powi(2)`) on exactly that ground — bit-identical at `f64`, tighter at `Interval`, still containing the truth. **One line above sits `b1`'s `((s * self.x) * self.x)` — a *scaled* square, invisible to any matcher of this shape, and deliberately left alone**, because tightening it means rewriting `(s·n.x)·n.x` as `s·(n.x²)` and the doc says *"each component exactly as parenthesized"*. **The question: may a D9-fixed evaluation order be reassociated when the reassociation is strictly tighter at `Interval` and bit-identical at `f64`?** It is not a matcher question and must not be answered by widening one — **it decides two ratified sites**: `orthonormal_basis`'s `b1`, and `linalg/mat.rs`, whose interval-square allowlist entry is justified by *"`rotation_about`'s evaluation order"*. A taker who treats it as a sweep will red both. Note `memories/output-stability-as-justification.md` does **not** settle it: it says byte-preservation may choose among equivalent implementations but never justify keeping code, and the live claim here is that the *order itself* is the ratified thing. **RULED: YES — reassociate.** Evan, 2026-08-21, on two grounds. **(1) Moving output is not on its own a reason not to act.** `memories/output-stability-as-justification.md` names *arithmetic association* as exactly the kind of thing output stability may decide, and is explicit that committed bytes are *"usually a golden, and regenerating a golden is a chore, not a contract"*. The orchestrator had offered the `f64` byte-move at `mat.rs` as a downside **while citing that same memory two paragraphs earlier**, which is the error the memory exists to prevent. **(2) The memory's carve-out does not reach this.** It preserves *"the D2/D9 determinism contract itself (bit-identical replay, byte-identical export)"* — and Evan: **D9 is determinism at one kernel, not pinning the same output forever.** So the same document evaluated twice must agree; it need not agree with last year. `u_ref` is stored as data per D2, so existing documents keep their frames.

**D109(a) is CLOSED by #885** and what follows is the record of the prediction it tested. **The exact-scalar test predicted COST, not permission**: `orthonormal_basis`'s `b1` has `s = ±1`, so `(s·x)·x` and `s·(x²)` are bit-identical at `f64` (round-to-nearest-even is sign-symmetric) — free, and #885 pins that with a proptest. **The *"strictly tighter at `Interval`"* half of the original prediction is the one thing here that does not survive checking**: it narrows only where the scale is a definite `±1` and `n.x` straddles zero, it is exactly a wash where `n.z` straddles zero (`s = [−1, 1]`), and the ruling's own third rider already forbade stating the width absolutely. `mat.rs::rotation_about`'s `t * x * x` has `t = 1 − cos θ`, arbitrary, so it **moves `f64` bytes** — 34.6% of random θ/axis pairs — **and re-cut no golden at all**, because the reassociation is exact for axis components in `{0, ±1}` and that is every rotation on a committed artifact's path here. The prediction's first half held; its second half was right about the arithmetic and wrong about the chore.

**Three conditions ride with it, none of them a reason to decline.** The **`Dual<f64>` tangent changes and nothing tests it** — `Dual::mul` is `x'·x + x·x'`, `Dual::powi` is `(2·x)·x'`; 6,388 of 3,000,000 inputs differ at the last ulp of a subnormal tangent and `x = (1e308, 1e-308)` gives old `1.9999999999999998`, new `inf`, while the in-tree guard asserts only the **value** channel. Extend the guard **with** the change, not after. *"Strictly tighter"* has an exception: `powi(2)` is **1 ulp wider** below `|x| < 2^-480` (the *"never wider"* claim cited inari, which has not been the backend since M5 PR 1) — unreachable in the live regime, 0 widenings in 3M samples over `|x| ∈ [1e-60, 1e60]`, but do not state it absolutely. And **the gate cannot see scaled squares at all**, so this authorises a manual sweep rather than producing one; a taker who reaches for a matcher widening will red two ratified sites.

Full statement in §D row **D109**(a). |
| **The C-namespace** | After the merge renumbered the second scan's observations to **C18–C25**, §D's Track C rows still occupy **C15** and **C17** for different things. One sentence giving Track C's rows a distinct prefix closes it permanently. |


---

## Tracks J–X — the repartition (2026-08-21)

**Every track A–I is now closed.** A, B, D and I completed; C closed its
session with most of its table unstarted; F closed on landing its eight rows;
**E, G and H are dead too** — E left four unstarted lanes and ~26 unstaffed
rows, G left two rows and thirteen placements, and H landed three of its ten
rows and stopped. What each of them left behind did not stop being work when
its orchestrator stopped, and §C3 says a deferral that lands nowhere that
executes is the failure this document keeps re-finding. **This section is the
one register for all of it.**

**109 open items, repartitioned into twelve tracks by FILE TERRITORY.** The
partition rule is the only one that matters here: **no two tracks may edit the
same file**, so no branch waits on, fences against, or re-derives another's
scope. Dependencies *inside* a track are its own orchestrator's to sequence —
that is what an orchestrator is for — and there are no dependencies *between*
tracks that any lane must honour. A track can be claimed the day it is read.

**What this repartition is not.** It is not a re-verdict. Every item keeps the
row number, finding number and disposition it already had; `D105`, announced in
prose and never given a table row, is given one here. Nothing below is closed,
re-scoped or re-argued by being moved.

## The rules this partition runs on

1. **The fence is the file, not the subject.** A track owns paths. If a row's
   work reaches a path another track owns, the reaching half is **filed as a
   row on the owning track** and the first track lands without it. No lane
   ever edits across the fence, and no lane ever waits for the other side —
   filing the row *is* the handoff.
2. **Number blocks are published here, before any lane is given a number**
   (Track G's rule, learned the expensive way). Blocks are clear of every
   existing reservation and of the tree's maxima (`D179`, `S249`).
   **Re-derive after every merge anyway**: a block cannot stop a number
   arriving from another track, only re-checking can.
3. **A row leaves this section when it lands, and its finding leaves the
   document with it.** Both are *deleted*, not annotated: the merged PR is the
   record and git is the archive, so a `FIXED by #NNN` lead is a thing to
   remove rather than a thing to write. Where the closing PR establishes a
   standing rule or a correction later work depends on, **relocate that
   sentence into text that survives** — a track log, `memories/`, the code
   itself — before deleting the record that carries it. Every landing PR
   therefore edits this file, and they conflict by construction — **merge one
   at a time, within a track**. Across tracks the conflicts are this section's
   tables only.
4. **Review policy, inherited unchanged from Tracks F, H and I:** style review
   on every unit against `docs/prompts/reviewer-style-lane.md`, carrying the
   two questions the standing brief does not ask — *is the original problem
   completely gone*, and *was it closed in the best way available*.
   **Adversarial only where a wrong answer is reachable**; the items that
   carry it are marked **ADV** below.
5. **The one thing every closed track agreed on**, and it held eight units out
   of eight on Track F and every unit on Track G: **the fix mints a fresh
   instance of the defect it closes**, and naming that trap in your own PR body
   does not prevent it. Only a reader who did not write the fix has ever caught
   it. Standing rules: `docs/SMELL-F-LOG.md`, `memories/agent-lane-operations.md`.
6. **Not in any track, and deliberately:** `L1` (S36, comb-and-rename per
   suite), `L2` (S38, comment trimming), `C2`/`H17` (S37's rustdoc remainder,
   ~1115 lines across 130 files) and `C21` (two workspace-wide comment
   populations, read per item). All four are cross-cutting comment or naming
   sweeps that would collide with **every** track on this list, which is the
   same reason they were last before. They go after this section empties.
7. **Not work at all:** the *Open decisions — Evan only* table above, plus the
   second scan's `S65`, `S70`, `S82`, `S90`, `S107` and `S116(p)`. No lane may
   resolve one by implementing something. Where a track holds the work that
   *follows* a decision, its row says so and the row is not takeable until the
   decision lands.

## The twelve territories

| Track | Territory (the fence) | Block | Items |
|---|---|---|---|
| **J** | `.github/workflows/`, `local-scripts/`, `scripts/doc-gate.sh`, `scripts/gates/{gate-roster,probe-suite-census}.sh`, **every `*.py` in the repo**, root `Cargo.toml`'s `[workspace.lints]` | `D180`–`D199` / `S250`–`S269` | 7 |
| **K** | `scripts/gates/` (everything J does not name), `tools/`, `docs/K-REPORT.md` | `D200`–`D219` / `S270`–`S289` | 11 |
| **M** | `crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`, `interval-transcendentals/`, `crates/bvh/` | `D220`–`D239` / `S290`–`S309` | 6 |
| **N** | `crates/geom/src/`, `crates/geom-core/src/{spline/,linalg/}` | `D240`–`D259` / `S310`–`S329` | 8 |
| **P** | `crates/topo/src/{euler*.rs,split,attach,movefac,revert,live,merge_faces,seqgen,validate}.rs` | `D260`–`D279` / `S330`–`S349` | 10 |
| **Q** | `crates/topo/src/{boolean/,splitting/,census.rs,chord_join.rs,chart_region.rs,face_normal.rs}`, `crates/geom-brep/src/{ssi*,pcurve_cache.rs,nurbs_iso.rs,edge_nurbs.rs}`, `docs/predicate-dimension-audit.md` | `D280`–`D299` / `S350`–`S369` | 10 |
| **R** | `crates/geom-brep/src/props/`, `crates/mesh/` | `D300`–`D319` / `S370`–`S389` | 10 |
| **T** | `crates/sweep/` | `D320`–`D339` / `S390`–`S409` | 10 |
| **U** | `crates/step-import/`, `crates/step-export/`, `crates/stl/`, `crates/pncad-py/`, `crates/pncad/` | `D340`–`D359` / `S410`–`S429` | 10 |
| **V** | `crates/editor-core/`, `crates/profile/` | `D360`–`D379` / `S430`–`S449` | 11 |
| **W** | `crates/*/tests/` (all crates), `crates/test-utils/` | `D380`–`D399` / `S450`–`S469` | 12 |
| **X** | `demos/` (Rust and Markdown; its Python is J's), `docs/DESIGN.md`'s companion table | `D400`–`D419` / `S470`–`S489` | 4 |

**Two seams are stated rather than left to be discovered**, because both are
places where a reasonable reader would think the fence ambiguous:

- **`crates/*/tests/` is W's, in every crate, without exception.** A track that
  owns a crate's `src/` does **not** own its `tests/`. Where a src change needs
  a test, W is not in the way: the test belongs to the PR that makes the change,
  and W's rows are about the *test-side mechanisms* named in them — the guards,
  the doctests, the stand-downs, the fixtures, the probe-gated suites. W files a
  row on the owning track when a mechanism reaches into `src/`, and vice versa.
- **Every `*.py` in the repo is J's**, including the fixtures under
  `crates/*/tests/` and the renderer under `demos/`. There are 12 Python files
  in the tree outside `.venv`, one class of question is open across them
  (`D122`), and splitting them four ways to match the Rust fences would put four
  tracks in one 12-file population.

## Track J — what CI actually runs

**Fence:** `.github/workflows/`, `local-scripts/`, `scripts/doc-gate.sh`,
`scripts/gates/{gate-roster,probe-suite-census}.sh`, every `*.py`, and root
`Cargo.toml`'s `[workspace.lints]`. **Block:** `D180`–`D199` / `S250`–`S269`.

| # | What | Was |
|---|---|---|
| **D40** | `scripts/doc-gate.sh` is the repo's only rustdoc gate and has never been shown to fire — the sole guard for lints `cargo check` and `clippy` are both silent about | Track E |
| **D41** | The rustdoc gate covers workspace MEMBERS; `tools/k-lint`, `demos/tour`, `demos/wild` and `interval-transcendentals` run no `cargo doc` anywhere | Track E |
| **C17** | Two excluded roots carry a hand-copied `cargo doc` step and nothing gates the copies against drift — `D41`'s other half, and they must land together | Track C |
| **D71** | The local gate has no `oracle-certify` mirror and nothing enforces `ci.yml` ↔ `ci-local.sh` job parity (S127). Two decisions inside it, both this track's | Track G |
| **D110** | `scripts/ci-filter.py` decides whether any gate runs at all and is the one script here with no test (S164) | Track F |
| **D24** | A `pub` item dead workspace-wide is invisible to every mechanical check the repo runs; `[workspace.lints]` has no `unreachable_pub`. **Closes on a chosen mechanism that runs in CI** | Track E |
| **D122** | Twelve `# noqa` markers for three linters the repo does not run (S196). A cost decision — run one Python linter over all 12 files, or delete the markers. **Not half.** If the answer is "run one", the CI wiring is this track's too | Track G |

## Track K — the instruments, and what they cannot see

**Fence:** `scripts/gates/` less the two scripts J names, `tools/`,
`docs/K-REPORT.md`. **Block:** `D200`–`D219` / `S270`–`S289`.

| # | What | Was |
|---|---|---|
| **D102** | The compound-`Bounds` gate anchors on `+`, which is not how Rust expresses a compound bound — `where T: A, T: B` is silent, and so is the multi-line form `rustfmt` converges on (S158) | Track F |
| **D103** | The allowlist is file-granular while its justifications are per-seam, so a second unrelated bound in an allowlisted file inherits the first's ratification silently (S159) | Track F |
| **D106** | `bounds-allowlist.sh` is a 204-line header in front of a 20-line function, grown three times for three honest reasons — split the ratification ledger out of the script | Track F |
| **D68** | `ArcCarrierScalar`'s 49 use sites are a compound bound no grep gate can see (S124). **A VISIBILITY row: Track V's `G4` changes what the alias is bound to and leaves the 49 sites exactly as invisible** | Track F |
| **D109** | What the F3 sweep left open in `scripts/gates/` (S163) — four live members; (a) is CLOSED by #885 | Track F |
| **D63** | `k-lint` admits `NaN`, `inf` and negative floats and scores the row clean (S119) | Track F |
| **D64** | What the tessellation and K instruments still cannot see after F6 (S120, four members) — including a fallback inside a comparison having two sides | Track F |
| **D105** | The split scan's constants can be guarded, on the continuous objective, which the cell count is not (S160) — `tools/tess-meter`. **Row announced in prose and never tabled; tabled here** | Track F |
| **C15** | `tess-lint`'s budget gate joins baseline to fresh rows on the face ORDINAL, so a reorder compares two unrelated faces or drops one with no finding (#746) | Track C |
| **D114** | The recording scalar is asserted to be a wrapper by no test and three sites say otherwise (S168). **The differential test is this track's; a `geom-core/src` change it turns up is Track M's row** | Track F |
| **S98** | `K-REPORT.md`'s dated M3 crop was back-filled against its own twice-stated rule and its arithmetic no longer closes | unrowed |

## Track M — the scalar and certification traits

**Fence:** `crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`,
`interval-transcendentals/`, `crates/bvh/`. **Block:** `D220`–`D239` /
`S290`–`S309`. **Six items, and it is the smallest track by count and the
largest by blast radius** — `H5` alone is 535 refs across 15 files and is
expected to split into two or three sub-lanes inside the track.

| # | What | Was |
|---|---|---|
| **H5** | The lane-trait collapse, `RingInterval` vs an always-on `Interval`, and the scalar ladders — Track C's `C-l`, never started; carries `S1`, `S2`, `S3`, `S44`'s residue and `S55`. **The sub-lane that REWRITES `Dual` arithmetic rather than re-spelling it is ADV** (C-R12) | Track H |
| **H3+H4** | The `Bounds` trait's headline still calls it the certification door and its ledger grew 50% under the fix meant to retarget it (S85); the one-home fix for the ring crossing minted three local aliases and a hand-counted tally (S89). **One lane — both sit on `real.rs` and `from_certified`** | Track H |
| **H10** | A rule with no instrument (S210): `real.rs`'s `Bounds` scope rule governs the sole-`T: Bounds` class and the allowlist gate cannot see it. **Carries `S211`'s unowned `bvh` member.** The gate-side half is Track K's `D68`/`D103` and is not this row | Track H |
| **S213** | `real.rs` credits the M7-8 lane with a technique it does not use, and the false half is the generalisable one | unrowed |
| **D78** | What is still one-directional in the interval backend after G1 — `powi`'s tightness ceiling, the oracle tier's scale-free ratio, and `interval.rs:135-143`'s consumer-side caveat (S134) | Track G |
| **S90-impl** | The largest D1 residue's implementation, and **#883 is parked on this track's ground** (H-g PR 1, folded into `H5`). **TAKEABLE — `S90` is RULED.** #867 merged 2026-08-21 07:14Z: *"tightening to `CertifiedBounds` works at least for now."* That is `H-R3`, #886 implemented it at two of three sites, and it is why #883 exists. **#883 is parked on a RULING, not on `S90`** — folded into `H5` because the fillet seam is one of the two sites where the lane-trait pattern was *deliberately declined* (`S3`), so its work and `H5`'s collapse are one argument. **Read `H-R16` before starting either.** | Track H |

## Track N — `geom`, and the spline and linalg substrate

**Fence:** `crates/geom/src/`, `crates/geom-core/src/{spline/,linalg/}`.
**Block:** `D240`–`D259` / `S310`–`S329`.

| # | What | Was |
|---|---|---|
| **H2** | One merge's residue, six findings, and it wants ONE lane — `S99`–`S103` plus `S116(b)`. **ADV**: `S99`'s widening changes what `net::is_placeholder` answers at ~25 consumer sites | Track H |
| **H9** | `geom`'s box constructors claim a loose box *"never prunes"* — false for three of the four doors those boxes feed, and the claim appears six times (S232) | Track H |
| **S235** | The exact conic box exists, is public and has no production caller, while `topo` re-derives a looser one by hand. **The one `topo` call site is this row's**; everything else in `topo` is P's or Q's | unrowed |
| **D97** | `KnotVector::from_algebra`'s debug arm carries row 5's classification and none of row 5's mechanism — no assert, no log, nothing observed | Track E |
| **D98** | `unit_segment` clamps a degree it could refuse, and the claim licensing the clamp is the wrong claim | Track E |
| **D31** | `sweep::skin::make_compatible` and `geom::curves::fit`'s `deviation_from` are ONE routine in two crates, and the proposed home is `geom-core/src/spline/algebra.rs`. **The `sweep/src/skin.rs` call site is this row's** | Track E |
| **C24** | S32's class on the curve side, which S32 does not name — `NurbsCurve::deriv_in_span`/`deriv2_in_span` each run a full order-2 basis and discard | Track C |
| **S215** | The tree's only oblique-axis bit-exact rotation test cannot fail for a change inside the rotation. **The named test file is this row's, by exception to W's fence** | Track H |

## Track P — `topo`'s Euler surgery, liveness and the generator

**Fence:** `crates/topo/src/{euler.rs,euler_ring.rs,euler_kill.rs,split.rs,attach.rs,movefac.rs,revert.rs,live.rs,merge_faces.rs,seqgen.rs,validate.rs}`.
**Block:** `D260`–`D279` / `S330`–`S349`.

| # | What | Was |
|---|---|---|
| **D49** | The literal liveness-check block `D25` named survives ~9 times, for the five arenas #755 did not touch | Track E |
| **D50** | `Live`'s unforgeability is guarded by nothing the repo runs — a `compile_fail` doctest cannot name a `pub(crate)` type, so the test that would try the forge cannot be written where the claim is | Track E |
| **D88** | A fourth spelling of the discard idiom, and the one site `D21` found that cannot meet #720's standard: `absorb` drops every ring of an absorbed face and returns `Ok`. **ADV** | Track E |
| **D38** | `merge_coplanar_faces` runs two incompatible failure regimes on one door, and the one that `format!`s is cited in the tree as the precedent for the other | Track E |
| **D90** | `octant_chart` scores a chart off two faces it never checks belong to the corner, and a wrong chart is the failure mode nothing downstream would catch. **ADV** | Track E |
| **D77** | The S11 sense-inheritance hazard is discharged and three comments still say it is open, including a **KNOWN HAZARD** block a reader consults before touching orientation | Track G |
| **D20** | D5's +46% on the `seqgen` lane is real and, after #722 excluded the candidate it was charged to, unattributed. **Closes on an attribution off hosted CI — a number, or a written finding that it is inherent** | Track E |
| **S68** | The W2c discard sweep stopped inside the function it was editing | unrowed |
| **S69** | `kfmrh`'s shell-fusion form is outside the fuzz catalog, and the `Ledger` counts solids, so it cannot notice | unrowed |
| **S93** | #713's prose-held-invariant sweep minted two new prose-held caller obligations, at `mev`'s fan site and `kev`'s fan merge | unrowed |

*(`S94`'s two hand-maintained `VARIANTS` ladders sit in `euler.rs` and
`validate.rs` — both this track's files. It is folded into whichever lane opens
`validate.rs` first and is not a separate row.)*

## Track Q — `topo`'s boolean, census and charts, and the predicate ledger

**Fence:** `crates/topo/src/{boolean/,splitting/,census.rs,chord_join.rs,chart_region.rs,face_normal.rs}`,
`crates/geom-brep/src/{ssi*,pcurve_cache.rs,nurbs_iso.rs,edge_nurbs.rs}`,
`docs/predicate-dimension-audit.md`. **Block:** `D280`–`D299` / `S350`–`S369`.

| # | What | Was |
|---|---|---|
| **G9** | Two operand gates with different admitted kind sets and a doc that describes only one (S95), plus `chord_join`'s placement argument contradicted by its own imports from `splitting/` (S96). **Both sides of the old Track C fence are inside this track now** | Track G |
| **S173** | The curved generalization of the one door lives inside `boolean/`, which is exactly what the door's own header argues against. The fix is a move, not a sentence | unrowed |
| **D120** | A wildcard over a closed enum, one layer upstream of `attribute()` (S192): `census.rs` decides `CensusEscalated` against `CensusUnsupported` behind a wildcard over a CLOSED ten-variant enum, so an eleventh arm becomes an unrefuted frontier | Track G |
| **H11** | A postcondition stated in four voices, one normative (S212) — `CertifiedEnclosure`'s *"a `Some` never carries a NaN end"* has a home and at least three doors re-derive it, two of them here | Track H |
| **S234** | The door inventory computes the roster's KEYS and none of its content — the direction column, which is the whole argument the guarded header makes | unrowed |
| **D95** | `boolean/combine.rs` now answers one proof two ways: two sites converted to `unreachable!` and six structurally identical siblings in the same function left as they were | Track E |
| **D57** | Nine names carry the K predicate vocabulary in refusal diagnostics while never reaching the funnel | Track E |
| **D46** | Twenty-three funnel-reaching predicate names in `geom-brep` and `topo` have no dimensional verdict anywhere, and three of their eight homes are files the audit has never named | Track E |
| **S83** | `seam_tol` / `MarchTolMismatch` cannot be reached and has no row; `MarchTol` is public with no possible caller | unrowed |
| **D36** | `PcurveCertifyError::UnsupportedCarrier` is payload-free and means three different things across 22 construction sites, beside a sibling that names its class at every site | Track E |

## Track R — the measuring consumers: `props/` and `mesh/`

**Fence:** `crates/geom-brep/src/props/`, `crates/mesh/`. **Block:**
`D300`–`D319` / `S370`–`S389`. Track I completed the rest of this ground; what
is here is what it left plus what Track C never started.

| # | What | Was |
|---|---|---|
| **C3** | `props/quad.rs`'s four independent quadrature engines with a triplicated convergence block (S27). **NOT TAKEABLE until #723 is fixed** — a wrong certified volume lives in the file this row consolidates, and consolidating first bakes it in or moves it away from its reproduction. The lane is described in a comment on #723 | Track C |
| **D30** | `quad.rs` holds a second span search and a second index clamp, because `KnotVector` cannot represent what the module needs. **Same file as `C3` and gated with it** | Track C |
| **C11** | #726 and #727 — fold the iso-rectangle SHAPE question onto the named predicate, and decide which door owns the refusal now that `mesh` and the boolean are protected only transitively | Track C |
| **S26** | The certified area enclosure is never metered against anything, now measured: 7.8e-3 relative on an ordinary loft where the same body's volume bracket is 1.2e-14 (#870). **Wants a written proposal, not a patch** | Track C |
| **S28** | Three tessellation lanes are parallel pipelines with no shared core — the duplication half, now that #648/#674 have settled the ordering and column questions | Track C |
| **S231** | `chords.rs`'s *"the only places adjacent surfaces enter chord counts"* is an absence claim over a whole module, true today and unfalsifiable by anything in the tree | Track I |
| **S236** | `cert_cylinder` is falsified by nothing, in any build — and closing it changes `budget::FaceMeasure`, whose consumers are in `tools/`. **The `tools/` half is Track K's row** | Track I |
| **S237** | The `worst_ratio` ceiling CI actually runs is the one still monotone the easy way — three live instances, not one | Track I |
| **C22** | A crate's sizing vocabulary is `pub` in three places and `pub(crate)` in seven, and nothing consumed the public three when #803 wrote them | Track C |
| **C23** | One rational refinement schedule hand-synced across a crate boundary — `mesh`'s `RATIONAL_CERT_SPLITS` and `geom`'s `RATIONAL_METER_SPLITS`. **The `geom` constant is one line and is this row's, by exception to N's fence** | Track C |

*(`S65` — the watertightness backstop absent from every shipping build — is
**Evan's decision**, not this track's row. Its equipped statement, the three
options and the measured price are at `S65`; issues #896 and #897 carry what
#872 could route. When it is ruled, the implementation is this track's.)*

## Track T — `sweep/`

**Fence:** `crates/sweep/` (both `src/` and, by exception to W's fence, the four
`sweep/tests/` files its own rows name). **Block:** `D320`–`D339` /
`S390`–`S409`.

| # | What | Was |
|---|---|---|
| **D124** | Re-home the findings E-g's retirement left untracked — `S111(a)(b)(d)`, `S112(a)` and `S75` were routed to a lane that landed without them and whose row is struck (S177). Three re-derived from the tree and standing | Track G |
| **S75** | The recourse-contract guard's central assertion is a tautology, and two suites defer their completeness obligation to it | Track F routing |
| **D96** | Thirteen `unreachable!` arms are row-0 candidates — states a type change could stop spelling, `EmptyChain`'s exact shape. **Ten are this track's**; the remainder are filed as rows on the tracks that own their files | Track E |
| **D91** | `map_err(\|_\| …)` is D29's own disclosed blind spot and D29 never ran it inside its own crate — two hits, both in `loft.rs` | Track E |
| **S131** | A unification declares *"one home"* while self-declared hand-copies of the same rule stand inside its own scope — 13 hits in `sweep/src` outside `fillet/` | unrowed |
| **S132** | `strut_spec` — one name, two different rules, in one crate, sharing nothing but the name | unrowed |
| **C20** | Every turning-path swept or lofted chart shape that is NOT a quarter-turn arc or a constant-pitch helix is unpinned for orientation — including the one the tree itself authors a ROLL on | Track C |
| **C25** | One swept body built from scratch six times across three crates, enumerated by #779's class sweep and reported there as COVERAGE rather than as the duplication it is | Track C |
| **D104** | The two hand-run diff artefacts `S110` could not place — a printed `Debug` hash with no assertion, and pinned seeds licensed for a digest half that is printed and never asserted | Track F |
| **C-e/H13** | `sweep/tests/`'s helix orientation coverage — the row §D twice records as having no home. **Verify against #779 before staffing**: Track C recorded H13 FIXED by that PR and the H/I handover recorded it open, and both statements are in this document | neither |

## Track U — the exchange surface and the bindings

**Fence:** `crates/step-import/`, `crates/step-export/`, `crates/stl/`,
`crates/pncad-py/`, `crates/pncad/`. **Block:** `D340`–`D359` / `S410`–`S429`.

| # | What | Was |
|---|---|---|
| **E-m / #711** | The recognizer's *"unreachable"* arm is reachable and the spec sentence has no document — **PR #784 is open and red on this ground**; the track inherits it | Track E |
| **C13** | ε has no type of its own, so `StepOptions::uncertainty_m` and two bare `f64`s restate `Tolerance::init`'s finite-and-strictly-positive rule by hand (#741). **Wants a plan signed off before implementation** — cross-crate public API | Track C |
| **C14** | The STEP writer hardcodes two Part 21 header fields the standard assigns to the user (#742). **Same signed-off-plan caveat** | Track C |
| **C16** | The Python STEP door exposes one of `StepOptions`' six fields, silently (#730) | Track C |
| **C19** | `assemble.rs`'s `build`/`build_one_solid` pair — dead code rather than a stale claim | Track C |
| **C26** | The sizing-vocabulary rule is violated verbatim one crate outside the fence it was written for, by the same question — `const STEPS: usize = 16` | Track C |
| **D94** | The discard idiom does not stop at `crates/topo`, and `D21`'s crate clause was a scope of work rather than a finding about the class. **The sharpest instance answers the same question two incompatible ways five lines apart** | Track E |
| **D37** | `pncad-py`'s tag map re-derives a discriminant that belongs on the kernel error, and its field-projection deferral has no owner | Track E |
| **D47** | The *"never a `Debug` dump"* rule has two remaining violations, both blocked on kernel types with no `Display` — **the `Display` impls themselves are Track V's rows** | Track E |
| **D48** | The compensating alarm for the one non-exhaustive tag map cannot fire, and two source comments say it can. **Closes on a verdict plus two comments matching it, not on a guard** | Track E |

*(`D37`, `D47` and `D48` are one crate and one class, and `D37(a)` shares a
mechanism with Track V's `D121`. Taking them as one lane is cheaper than any of
them alone.)*

## Track V — `editor-core` and `profile`

**Fence:** `crates/editor-core/`, `crates/profile/`. **Block:** `D360`–`D379` /
`S430`–`S449`.

| # | What | Was |
|---|---|---|
| **G4** | `profile`'s fifth lane trait, blanket-implemented, which D1 never looked at — `ArcCarrierScalar` over `T: Decide + Bounds`, so `Dual64` carries the whole arc surface. **Per Evan's ruling this is mechanical**; both of its old gates have fallen (#791, #801). The gate-visibility half is Track K's `D68` and is not discharged by this row | Track G |
| **D121** | The arc-mode vocabulary is the profile `Step` vocabulary one level down, with no `ALL` and no census (S195) — six spec structs restated three times, and `res_spec` CONSTRUCTS the kernel form so the compiler cannot see a mode that fails to arrive | Track G |
| **D75** | The PATHS verb vocabulary's sixth copy is the Python surface and it is the only silent one (S170). **The `pncad-py` stub edit is this row's, by exception to U's fence** | Track G |
| **D54** | Six refusal arms render prose over a payload they hold, and the blocker on every one is the same — four of the crate's OWN types have no `Display`. **This row unblocks Track U's `D47`** | Track E |
| **D81** | A typed payload with a `Display`, rendered by `Debug` at a composing layer — eight live sites, six of them one-liners, including the very `Display` #767 corrected bypassed one layer up in the same crate | Track E |
| **D89** | `DocEdit::DeleteNode` proves the node live, then eleven lines later defaults its input list to empty on a `None` that cannot happen — and the value is handed straight to `roots::on_delete` | Track E |
| **D39** | A typed refusal degrades to `String` at the edit door because two derive sets do not meet, and a test in the tree is already substring-matching prose to get the class back. **Both sides of it — `EditError` and `profile::PathError` — are inside this track** | Track E |
| **C12** | `editor-core`'s remaining classification residues from #731's style review — three things, including a test oracle that under-reports silently through a `_ => false` arm | Track C |
| **S105** | The shared refusal ladder retired one duplication and minted a documented hand-synced one | unrowed |
| **S190 / #855** | `attribute`'s decline lookup consults ONE of the pair's two faces, and arena order picks which | unrowed |
| **C6** | W2f remainder / S4 — `ProgramStep`/`WireStep`, `SegTag` and the "no usable value" core. **Genuinely blocked**, each member on something real (OnArc + RESPELL-TABLE, a first proc-macro crate, a persisted format); kept as a row so the block is visible rather than forgotten | Track C |

## Track W — the test targets: guards, doctests, fixtures and stand-downs

**Fence:** `crates/*/tests/` in every crate, and `crates/test-utils/`.
**Block:** `D380`–`D399` / `S450`–`S469`. **This track owns test-side
mechanisms, not test coverage for other tracks' changes** — a src change carries
its own tests in its own PR, as always.

| # | What | Was |
|---|---|---|
| **D61** | Twelve source-text guards, five hand-rolled Rust readers, and no two of them lex the same language (S117). The count went 7 → 9 → 11 → 12 in one session, each step a differently-*shaped* sweep. **Build the shared home here; a reader living in another track's `src/` is converted by that track, on a row this one files** | Track E |
| **D80** | Five spellings of *"is this line code"*, beside seven guards that already share the walk (S172) — blind to block comments, to `#[doc = "…"]`, to a needle in a string literal. **Same class as `D61` and the same home; one lane** | Track G |
| **D113** | Decide what an intra-doc link in a `tests/` file is (S135): `cargo doc` builds no test targets, so every one is inert on every tier and nine are already broken. **Closes on a decision plus its mechanism** | Track G |
| **H12** | Eleven `compile_fail` doctests in `geom-core/tests/` have never been collected (S214) — each asserts the compiler rejects a specific program, so each is a negative proof no tier has ever run | Track H |
| **S216** | The repo has ~39 `compile_fail` rows and not one verifies what it claims — 28 of the 36 collected ones carry an error code that is never compared to anything. **The generalisation of `H12`, and the two want one lane** | unrowed |
| **D111** | Fourteen probe-gated suites are compiled and never run. **RULED by Evan (2026-08-21): run all fourteen; the two collection runs stay.** Compilation is already paid, so the marginal cost is execution — turn them all on at once. Whatever reds is filed as a row on the track that owns the file | Track F |
| **D70** | The silent whole-row stand-down: a population of 13 — a FLOOR, not an enumeration — in three files (S126) | Track F |
| **D115** | The loud stand-down's ten hand-rolled spellings (S169), against a home that already exists in `test_utils::vacuity`. Three of the ten are the actual finding | Track F |
| **D107** | `review_d18`'s `kemr` reaches no mutation phase in either hammer row (S161), so the arms below it are attacked by nothing. Scoped work: a fixture whose ring-merge form gets it past its plan phase | Track F |
| **D72** | Re-mine the ε-keyed conditioning pin so its building bands exercise the collapse (S128) — #831 turned the defect into an assertion, which is a tripwire and not a fix | Track G |
| **C18** | Three residues of H12's own enumeration, left open by #734 — a tests-only unit, so all three are coverage or prose | Track C |
| **S230** | Certified widths with no ceiling, in crates no live track owned — `editor-core/tests/`, `pncad-py`'s and three in `sweep/tests/`. **All four sites are test targets and therefore this track's** | Track I |

## Track X — `demos/`, and the design doc's companion table

**Fence:** `demos/` (Rust and Markdown; its Python is Track J's),
`docs/DESIGN.md`'s companion table. **Block:** `D400`–`D419` / `S470`–`S489`.
**Four items — the smallest track, and it is small because its ground is
small.** It shares no file with anything above, so it can be taken alongside
another track by one orchestrator without breaking the partition.

| # | What | Was |
|---|---|---|
| **D79** | `lily.rs`, read end to end for the first time — six members, no owner: an orphaned comment block whose live number is wrong, a shadow tuple vector algebra beside `Vec3` (whose *reason* is #796), two carrier extractors with different rigor, and a partly-vacuous agreement check | Track G |
| **S129** | `demos/` has assertions and no runner — nothing runs `cargo test` under `demos/`, and **two rows of the tessellation pin are red on `main`** (#782) | Track G |
| **D123** | `demos/README.md`'s uv-lane numbers, which `S113(a)` is recorded as having swept: five figures the FIXED record itself names are still standing in the file that fix pass edited, plus two no run prints. **#787's own rule applied to the prose — compute or delete, never restate** | Track G |
| **D51** | `DESIGN.md`'s companion table describes the dimensional audit's open findings as they stood two retirements ago, naming as open a residue the audit's own disposition list records as closed | Track E |

## What this partition leaves out, said explicitly

- **The `Open decisions — Evan only` table** above, plus `S65`, `S70`, `S82`,
  `S90`, `S107`, `S116(p)` and the C-namespace collision. Seven of the twelve
  tracks hold work that one of these gates; each such row says so.
- **`L1`, `L2`, `C2`/`H17` and `C21`** — the four cross-cutting comment and
  naming sweeps, which collide with every track here and go after it.
- **`L3`** — the remaining `S35` roll-up rows, lowest value density, several of
  which will be resolved incidentally by the tracks above.
- **The unscanned crates**, which are a scanning input rather than a work item:
  the second scan never scoped `crates/step-import/`, `step-export/`, `stl/`, `bvh/`,
  `quantity/`, `pncad-py/` or `profile/`. Tracks **U**, **V** and **M** now own
  that ground, so a scan of it is theirs to commission and no longer collides
  with a live lane.

---

## Last, deliberately

| # | Item | Why last |
|---|---|---|
| **L1** | **S36** — comb-and-rename, **per suite**, never a rename pass. | A PR-numbered name currently *carries signal*: it marks a suite not yet combed. Renaming first converts a visible backlog into an invisible one. Needs an owner and a slot, not just permission — the 2026-08-13 retirement licence has produced zero deletions. |
| **L2** | **S38** — comment trimming. | Must follow every deletion above; trimming comments on code about to be deleted is pure waste. Note the pressure runs the other way too: three fix passes this week added prose because a finding demanded a claim-site reason that did not exist. |
| **L3** | Remaining **S35** roll-up rows. | Lowest value density; several will be resolved incidentally. |

---


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
  invisible** — which is the shape of what it missed. **Its conclusion did not
  survive** (established by #731, 2026-08-20): the same pattern was blind in a
  second way it did not name — a **binding** catch-all (`other => other`) is
  not `_`, and a match written through `use RoleSeg as R` never spells
  `RoleSeg::` in the window the scan required — and `eval/anchor.rs`'s
  `remap_seg` was both at once, a live fail-quiet of exactly the shape #632 was
  hunting, under a body that reported *"no fail-quiet wildcard in any `RoleSeg`
  or `Qualifier` match in the workspace."* The instrument that found it asks
  **rustc** rather than the text — `--force-warn
  clippy::wildcard_enum_match_arm`, reading each diagnostic's missing-variant
  list — which no alias, wrapper or indentation can fool. Its own blind spot,
  stated because that is this clause: an enum nested inside `Option`/`Result`
  is attributed to the outer type, so the lint cannot see the very arms #632's
  correction was about, and it says nothing about `if let` / `matches!`.
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
then state the shortfall as a verified negative.

*The original entry read "in all three cases the conclusion happened to
survive; in all three the method did not." **Corrected by #731**, and the
correction turns on which conclusion is meant, so the reading is stated
rather than assumed.* Take **the conclusion** to be **the negative result
the lane reported at the time** — *"nothing else matches", "the class is
closed"*. On that reading **none of the three survived**, and the bullets
above are the evidence for all three: #635's two survivors sat in the file
it had just edited, #639 shipped an instance in a crate its body reported
at zero, and #632's *"no fail-quiet wildcard in any `RoleSeg` or
`Qualifier` match in the workspace"* was falsified by #731's
`eval::anchor::remap_seg`. The original sentence was true of a different
quantity — whether the class turned out to be closed once each lane's own
correction had landed — and that is a claim about the corrected state, not
about the sweep. It is the sweep this clause is about.

What differed is only how the shortfall surfaced: #635's and #639's inside
their own programme, #632's not until a lane a day later ran an instrument
of a different shape. Where a sweep's blind spot is unstated, *"the
conclusion happened to survive anyway"* is itself an unverified claim.

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
instrument for dating a claim. The same shallowness makes a **cited SHA look
like a bad citation**: this document's own scan base `4258584` is not an object
in a fresh agent container and resolves only after `git fetch --unshallow`
(D23). A pointer that does not resolve is therefore not evidence of a wrong
pointer until you have unshallowed — check before reporting one.

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

## From the second scan (C18–C25)

Continuing this document's process numbering. **Renumbered by the merge:** these were written as C15–C22 on the premise that §C ran to C14. It ran to **C17** — and the first document's own forward pointer repeated the same wrong number, so the collision was invisible from inside either file. They are **C18–C25** here. The
first scan's §C was written from PR descriptions, A/B logs and
orchestrator logs. This one is written from something better: **a
controlled second look at code that the first round's findings had
already been applied to.** That is the closest this project has come to
measuring its own fix quality.

## C18. Two of my dispatch briefs were wrong, and both agents checked anyway

I wrote thirteen briefs. Two contained a false premise:

1. I told the `topo` agent that CI never runs the corrupt-input contract
   in release. It does — `ci.yml:769-816`. The agent opened its report
   with *"One correction to the dispatch premise"* and then reported on
   what the job **covers** rather than on whether it runs.
2. I told the `geom-brep` agent that S26 (area enclosure metering) was
   fixed, and asked which direction the metered faces moved. It is not
   fixed. The agent: *"Whatever briefed this as fixed was reading #472's
   deferral, not the tree."* My brief was internally inconsistent — it
   said #472 deferred it and then asked which direction moved — and the
   agent resolved the inconsistency against the tree rather than against
   me.

**This is the finding, not the errors.** A dispatcher's brief is the
highest-authority text a scanning agent sees, and the failure mode it
invites is confirmation: an agent told "X was fixed, check for
regressions" can produce a plausible regression report about a fix that
never landed. Both agents refused the frame. What made that possible is
plainly in the stance — *"a textual justification is not a defence"* and
*"your taste is evidence"* generalise to the brief itself, and neither
agent needed to be told that a dispatcher can be wrong.

**Worth making explicit in the reviewer's own document anyway.**
`docs/REVIEW-STYLE-DISPATCH.md` §3 already tells the dispatcher that
*"reviewers correcting the dispatcher is a working lane, not a
malfunction — say so in the brief"*, and
`docs/prompts/reviewer-style-lane.md` §1 does not yet say it. The
missing sentence is that the dispatch is a **hypothesis**, and that
contradicting it is a first-class result. It cost nothing here; it will
not always. (I have not edited either document — that is a ratified
process artefact and the change is Evan's call.)

## C19. The dominant defect shape is now "the fix pass had the file open"

This is the single strongest signal in the scan, and it is a *new* shape
— the first scan could not have seen it because there were no fixes yet.
Count: S59 (the ruling swept to two gates, not the third, in the same
directory), S60 (`volume_pad` fixed, `area_pad` twelve lines away not),
S63 (same), S68 (`split_edge`'s discards ten lines below the same diff's
`unreachable!` conversions), S74 (markers deleted at the copy sites), S80,
S84, S85 (`Enclosure` and `CertifiedEnclosure` corrected, `Bounds`'
headline not), S101 (the sweep deleted the fact rather than re-aiming
the pointer), S102, S110(b), S114(f), S116(m).

The mechanism is consistent enough to state as a rule: **a fix pass
scoped by the finding's citation list sweeps the citations and stops.**
The sibling instance is one screen away, in a file the author had open,
and the scope sentence in the fix's own prose is what makes it
invisible — several of these fixes *state* their scope
(`euler{,_ring,_kill}.rs` plus `link_half_edges`; "the per-variant
ladders stay where they are"; "the reported instances"), and the
statement reads as completeness.

**The rule already exists, on both sides, and it did not fire.**
`docs/REVIEW-STYLE-DISPATCH.md` §2 names *"the fix reproducing the
defect it closes"*; `docs/prompts/reviewer-style-lane.md` §3's
class-not-instance rule says that sweeping only the reported instance
*"is a **half-fix** and should be labelled one"*; and
`docs/prompts/implementer-discipline.md` §5 puts the obligation on the
fix pass directly: *"If your unit fixes an instance of a class, say what
pattern you swept with and **what that pattern could not match**."*
Thirteen instances landed anyway. The interesting question is not what
rule is missing but why the one we have does not bite.

**Two mechanisms, both visible in the artefacts.**

- **§5's trigger is the author's own classification.** *"If your unit
  fixes an instance of a class"* — and the recurring failure is a fix
  that was never classified as a class fix. `volume_pad` was fixed as a
  row, not as an instance of the monotone-enclosure class (S60). The
  both-operand-orders ruling was applied to the gate that reported it
  (S59, S63). The condition is exactly the judgement that fails.
- **§5's deliverable is a pattern, not a hit list.** An author who greps,
  sees three hits, fixes one, and writes *"swept `euler*.rs`; the pattern
  could not match delegating callers"* has complied in full. And a scope
  sentence reads as completeness even when the claim above it does not
  share its scope: `euler.rs:24` asserts the universal — *"at every
  write"* — while its evidence is *"these modules"*, which is how
  `split_edge` ended up three discards deep in the same diff (S68).

**So the amendment is small and specific: make the trigger unconditional
and make the artefact the hits.** Grep for the *shape* — not the symbol
(Q4's distinction) — before writing the scope sentence, and put the hit
list and its disposition in the PR description, one line per hit: fixed,
or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt. S60 is the cleanest demonstration:
`rg area_pad crates/*/tests` returns two tightness-relevant sites,
neither bounds it, and the fix pass was editing the file that contains
both.

## C20. C11's mechanism is real and has now been observed running backwards

C11 (first scan): every duplication in this codebase is self-declared in
prose (`verbatim`, `re-derived`, `ported from`, `mirror of`), and
nothing ever reads that prose. It was proposed as the cheapest
actionable mechanism available.

S74 is the counter-case that proves its value: the `revolve`/`extrude`
twins carried exactly those markers, and a consolidation commit
**deleted both markers while leaving both copies**, replacing them with
a sentence asserting the two are not twins — a sentence that is
factually wrong about `reverse: bool`. The greppable evidence was the
only evidence, and a well-intentioned cleanup removed it.

If the marker vocabulary is ever mechanised, the guard has to include
"marker removed without the code converging", not just "marker
present".

## C21. Q3 ("can this test fail?") is carrying the scan

Of ~110 findings, the largest single class is assertions that cannot go
red: S60, S75, S76, S78, S84, S91, and the ten sites in S110, plus
S66's acceptance suite, S72's pad probes and S73's `ratio`. Several were
found by *executing* a mutation — the `interval-transcendentals` agent
set `PAD_ULPS = 64` and `PAD_ULPS = 0` and reduced the rounding helpers
to round-to-nearest; the `scripts/gates` agent planted fixtures against
every gate. **Every claim so produced held.**

One sub-shape dominates:

- **Monotone in the wrong direction.** `area_pad > 0.0` plus
  containment; `assert_contains` on a widening enclosure;
  `worst_ratio ≤ 1` as `bound` grows; `holds(&box, sample)` on a box
  that only widens; `!contains(&anchor_idx)` on a list that may empty.
  The pattern is: *the assertion is satisfied more easily by exactly the
  degradation it exists to catch.* `reviewer-style-lane.md` Q3 already
  names this one; these are measurements of it, not a gap in the brief.

**A second shape — a skip reading as a pass — is deliberately left
un-rolled-up.** The instances stand on their own (S84 and the
`else { continue }` / `if let Ok(...)` / tolerant-arm /
`println!("SKIPPED")` sites cited with them). A class-level rule was
drafted and dropped: it was written around giving skips *floors*, which
concedes the skip, and the prior question — whether a test should be
skipping at all — is the one to answer first. Recorded here so the
next scan re-opens the question rather than re-proposing the floors
(Evan, 2026-08-20).

**Cheapest mechanisation available:** for every enclosure-style
acceptance row, require a *ceiling* alongside the containment. The
volume rows already do it; the pattern is three lines and it is the
difference between S60 and a row that works.

## C22. Executing the mutation beats reading the code, and it was rare

Three of thirteen agents ran experiments rather than only reading. Those
three produced the scan's most certain findings — every "green with the
guard removed" claim is a fact, not a judgement, and none needed a
steelman pass. The other ten produced findings that are mostly still
*questions*.

This is a cheap upgrade to the brief: **when a finding is "this guard
does not guard", try to break it.** A scratch copy of the crate and a
one-line mutation is minutes, and it converts a `likely` into a `sure`.

## C23. The A1 rule (non-improvement deviations owe a scheduled followup) has not taken yet

S115 is six disclosures written *after* the rule, none with an issue
number or a named plan unit, several stating "unscheduled" as though it
were the schedule (`tools`' `agree` column says it in two crates
independently; `doc-gate.sh` says *"a row is owed … and it is
unscheduled"*). S90 is the sharpest version: the D1 ruling's three
*smaller* residues all got issue numbers (#687, #700, #701) and the one
seam it actually left unguarded got prose.

The disclosures are honest and well written, which is exactly the C2
diagnosis. `docs/REVIEW-STYLE-DISPATCH.md` §4 already warns the
dispatcher not to let the `## Style` section *"become the place where
known problems go to be recorded and forgotten"*, and Q6 exists to close
it — so this is not an unnamed problem. It is a named problem with no
mechanism.

What the rule lacks is a place that *executes*; C3 said this. The
register has to be mechanical: a grep for the disclosure vocabulary that
fails without an adjacent issue number would be a gate in the style of
the fourteen that already exist — and S63 is the warning about how
carefully that regex would need to be written, since every one of the
six existing grep gates has a hole of exactly that kind.

## C24. The style brief worked, and here is what it cost

First use at scale: thirteen agents, ~110 findings, of which I judge
roughly a dozen to be over-reaches and ten hand-verified to hold. The
question-numbered self-reports at the end of each agent's output (*"Q1
— findings 2, 4, 6, 7; Q4 not exercised, no diff to invalidate
against"*) were unexpectedly useful as a coverage receipt, and I would
keep them.

Two observations for the next revision (recommendations only — I did not
edit `docs/prompts/reviewer-style-lane.md`):

- **Q8 (read a whole file end to end) produced the findings nothing else
  would have.** S116(e) (the euler header is now two screens of another
  module's contract), S116(g) (60% comments), and the demos agent's
  honest note that `lily.rs` was sampled rather than read. C4 said
  nothing in the process reads a whole file; Q8 is the fix and it is
  working.
- **The stance's "report more rather than fewer" produced a long tail
  that needs a coordinator.** Roughly a third of the raw findings became
  roll-up bullets here rather than standing rows. That is the right
  outcome, and saying so in §3 ("what your findings must look like")
  would stop agents calibrating toward fewer, better-defended findings —
  the defended ones are not the valuable ones.

## C25. Documentation growth is still the default response to a finding

C5 measured this in the first scan; it has not turned. Measured this
round: `real.rs`'s `Bounds` block 156 → 234 lines (S85);
`crates/mesh/src/curved.rs` 243 → 712 production lines, 60% comments,
with ~180 doc lines over ~55 lines of guard code (S116g);
`SAFE_ASPECT`'s doc ~20 → ~50 lines while the constant did not move
(S116h); `crates/topo/src/euler.rs`'s header +55 lines (S116e);
`scripts/gates/bounds-allowlist.sh` — 130 lines of header defending a
20-line function, restating a ledger it declares it is not restating
(S116m). **Re-measured: 131 lines when #791 opened, and #791 takes it to
204** — it cut five lines of comment archaeology and compressed its own
additions twice, and the remainder is three newly disclosed blind spots
(GAPs 3, 4, 5), the reason the definition skip is exact text, and the
correction of a mitigation that was published false. The finding is *not*
discharged by that accounting and the number is recorded rather than
restored, and **placed as D106** — split the ratification ledger out of the
script — because an argument recorded is not a row that executes.

**And the growth is itself the finding's answer, which is worth more than
the row.** A gate whose gaps are honest is longer than one whose gaps are
silent: every line #791 added past its own fix is a blind spot named, a
false claim retracted, or a repair the next reader is told *not* to make.
The conclusion is that **this directory wants the ratification ledger
split out of the script** — the per-seam justifications are a document
that happens to live in a comment block, and they are what makes a 20-line
function carry a 204-line header. That is a real observation about the
shape of `scripts/gates/`, not an argument for un-disclosing anything, and
it is the disposition S116(m) should close on. **The full progression —
131 → 157 → 195 → 204, and what each step bought — is in D106's record**, where
its taker will read it.

In several of these the prose is the *only* change: S116(g) answers
"three parallel pipelines with no shared core" with a long argument that
this lane does not need one; S107 closes a naming confusion by argument
rather than by change; S116(h) converts one undecided constant into a
more honest account of the same undecided constant.

None of this is dishonest — the opposite; it is unusually candid. But
the brief's own rule (*"unusual justification length is mild evidence
for a smell"*) now has a large, measured corpus behind it, and the
question it raises is a policy one for Evan rather than a finding:
**when a finding's honest answer is "we are not going to change this",
what is the maximum acceptable length of that answer, and where does it
live?** A 234-line trait doc and a 130-line gate header are both past
the point where the rule is findable, which is the failure mode that
matters.
