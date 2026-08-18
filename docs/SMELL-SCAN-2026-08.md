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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

## S22. Ambient state contradicting the purity thesis

- **Confidence**: sure for each row

| Mechanism | Anchor | Consequence |
|---|---|---|
| `Band::linear()` reads a `OnceLock` self-initialising from `CAD_TOLERANCE_EPS`/`CAD_AMBIGUITY_K` | `tolerance.rs:214`, `predicate.rs:352` | ε and K are ambient inputs to every predicate, sitting awkwardly beside the commitment that a model is a pure function of a parameter vector. The cost shows in four dedicated integration binaries existing purely to get process isolation |
| The `k_stats` verdict log delivers a **production value** by thread-local side effect; `start_verdict_log` overwrites unconditionally | `k_stats.rs:44`, `:264` | Measured: an `InstantiatePart` node records **0** verdicts where the same geometry records 722. The doc records the bug, warns readers off the mechanism it documents, and notes which of two remedies to take was "left open deliberately at merge" |
| `mesh` reads ε twice where the crate docs and `DESIGN.md` D4 say once — and the second read **snaps** a value | `walk.rs:730`, `walk.rs:496` | The mesh is a function of (body, δ, **ε**), not (body, δ) as the determinism/memo-key contract claims. F6 also bans "EPS snapping anywhere in the pipeline" |
| `same_chart` decides chart identity by `core::ptr::eq` on two `&Body` | `chart_region.rs:394` | In a module whose premise is "structural identity, never numeric identity". A caller passing a clone silently drops to the weaker rung |

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

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

**Verdict:**

## S40. Residue and editing artifacts

- **Confidence**: sure

- `seqgen` maintains a `roundtrips` counter and ends with
  `let _ = roundtrips;` under a comment describing an assertion that is
  not there — so the property suite cannot distinguish a run where every
  step was skipped from one where all roundtrips executed
  (`seqgen.rs:853`).
- `ring_interval.rs:551` asserts `!x.powi(i32::MIN).is_poison() || true`
  — unconditionally true, testing nothing.
- `every_error_displays` claims exhaustiveness "by compiler guidance"
  that a hand-written array literal cannot have, and covers 16 of 27
  variants (`euler.rs:3060`).
- `name_boolean_vertices` still takes `_seam_set` "unused since M4 PR 5"
  and the caller still builds and passes it; `a_faces`/`b_faces` are
  sorted and deduped twice; guarded arms carry `unwrap_or_else`
  fallbacks that are unreachable *and* would mint `Seam{ae, ae}` — a
  well-formed name denoting the wrong thing — if they fired
  (`emit_topo.rs:1133`, `:1266`, `:1283`).
- `pncad-py/src/py/flush.rs:182`'s `growth_tripwire` is a byte-for-byte
  copy of two functions ~100 lines above that were *already* exhaustive
  matches over the kernel enum — the pattern was copied from
  `py/select.rs`, where the inverse direction genuinely adds a check,
  without noticing the direction was already right.
- `profile::k_stats` is a self-declared compatibility shim ("new code
  should reach for `geom_core::k_stats` directly") that all eight of the
  crate's own decision-making modules use in preference to the thing it
  forwards to — including modules written long after the unification it
  describes (`profile/k_stats.rs:1`).
- A refusal message in `boolean/ops.rs:1600` lost its line-continuation
  backslashes and contains three ~40-space runs mid-sentence.
- In `boolean/join.rs`, `loose_partners`' doc comment was merged into
  `germ_section_frame`'s (so it documents the wrong function) and its
  orphaned tail sits on `type LooseMap` 150 lines away. The same
  stranding happened to `feed_stable_name`'s doc in `eval/mod.rs:1543`
  and `run_iso_checks`' in `pcurve_cache.rs:2459`.
- `finish_fallback` computes an identical `let (a_view, b_view) = match
  kind {…}` twice in a row (`boolean/ops.rs:1944`).
- `WitnessSlot {}` is an empty struct occupying a field on every
  `NodeValue`, paired with a `NodeErrorKind::WitnessBifurcation`
  documented as never constructed (`eval/mod.rs:230`).
- `save` writes `drop(replay)` on a value about to go out of scope;
  `member_of` lists a `Subgroup::Empty` arm an early return already made
  unreachable (`persist/mod.rs:637`, `coset.rs:583`).
- `same_level`'s structurally-impossible arm manufactures its error by
  feeding `f64::NAN` into `classify` and letting the funnel escalate —
  a decision predicate used as a `throw`; `unreachable_zero` returns a
  4-tuple of NaNs into live flux arithmetic (`props/curved.rs:350`,
  `:1090`).
- `Rim` stores the same traversal direction twice (`d_u: T` and
  `d_u_sign: Sign`), and the exact one is compared through the tolerance
  funnel — subtracting two exactly-±1 values and banding a result that
  is always 0 or ±2 (`props/curved.rs:330`, `:384`).
- The `dbg-join` cargo feature exists for exactly one `eprintln!` block
  (`topo/Cargo.toml:12`).
- `crate docs` devote a paragraph to defending the single `HashSet` that
  violates D9's determinism rule, concluding "a `SecondaryMap` would be
  both cheaper and consistent with the rule" — for a set used at three
  sites (`topo/src/lib.rs:60`).

**Verdict:**

## S41–S48 — reserved

IDs `S41`–`S48` are intentionally unallocated, so that items promoted
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
