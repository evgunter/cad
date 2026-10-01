---
id: S90-impl
kind: unit
title: Tighten the blend seam's three doors to CertifiedBounds — the ruled S90 implementation, with #883 parked here
status: open
opened: 2026-08-21
track: M
pr: 883
refs: [867, 886]
priority: P1
cost: H
needs_ev: true
---

## What

The largest D1 residue's implementation, and **#883 is parked on this track's ground** (H-g PR 1, folded into `H5`). **TAKEABLE — `S90` is RULED.** #867 merged 2026-08-21 07:14Z: *"tightening to `CertifiedBounds` works at least for now."* That is `H-R3`, #886 implemented it at two of three sites, and it is why #883 exists. **#883 is parked on a RULING, not on `S90`** — folded into `H5` because the fillet seam is one of the two sites where the lane-trait pattern was *deliberately declined* (`S3`), so its work and `H5`'s collapse are one argument. **Read `H-R16` before starting either.** **Re-read against the tree 2026-09-02, as a measurement and not an unparking** — three things moved and one did not. (i) The blocker STANDS: `sweep::blend::fillet_edges` is still reachable from `editor_core::eval::evaluate`, through `eval::wire::run_op`'s `T: Decide + ContentBits + Bounds + Send + Sync + AtRestPolicy + AxisScalar` to `wire_fillet`, and that set admits `Dual`, so #883's twelve red jobs still reduce to that one `E0277`. (ii) The pass is still mixed, and the split still sits where H-g put it: the geometry builder holds ZERO bracket reads at `T: Real`. (iii) **Every #883-era citation needs re-aiming**: the module is `crates/sweep/src/blend/`, not `sweep/src/fillet/`, and the builder is `blend/arms.rs`, not `blend.rs`. (iv) **H-g's bracket-read count of 14 no longer holds, and the unit matters**: 17 LINES carry a bracket read, 19 READS in all — `battery.rs` 11 lines / 12 reads, `build.rs` 3 / 3, `surgery.rs` 3 / 4, `arms.rs` (the geometry builder) 0. The classification also needs correcting rather than repeating: they are NOT all typed-error payloads. In `battery.rs` nine are error-payload fields (`margin`/`radius`/`gap`/`arm`), `:1235` is a BRANCH CONDITION comparing two bracket lows (`d0.min(d1).lo() == d0.lo()`, a junction-end pick) and `:1289` is a value read feeding an f64 quantity; `build.rs` is one `partial_cmp` datum gate plus two payloads; `surgery.rs`'s four are representation-datum selections, each with its argument written at the site. So the population is payloads plus SELECTIONS, and a taker owes a per-read classification rather than either count. **Re-read again 2026-09-02 by CERT-M3, and STILL NOT UNPARKED**: that unit deleted `EdgeNurbsLane` and touched nothing on this seam, and the reason is the seam's own citation — `fillet_edges` is `T: Decide + Bounds + PcurveFittedLane`, and `PcurveFittedLane` is the one lane trait the census says does not split (its certified half CONSTITUTES the pass's product). So the mixed pass this branch reduces to is still mixed, through a trait whose split is blocked on a representation question and not on a bound, and #883's twelve red jobs still reduce to the same one `E0277`. Nothing here moved; the reading is recorded so the next taker does not re-derive it.

Status note: the row says TAKEABLE, so the item is `open`; `pr: 883` records the parked measurement branch, not a queued landing. The id `S90-impl` is carried as written (all characters are within the tracker's id alphabet).

## Was

Track H.

## Finding

### S90. The blend seam's three doors still admit a dual

- **Where**: `crates/sweep/src/blend/{build,battery,surgery}.rs` — `fillet_edges`, `run_battery` and `ring_clearance`, each still `T: Decide + Bounds` (`fillet_edges` with `+ topo::AtRestPolicy`, where the fitted door is read). **The module was `sweep/src/fillet/` when this finding was raised**, and every citation below, #883's included, spells it that way.
- **Confidence**: sure

`Bounds` has a `Dual` impl since D1 and these are `pub` doors on an API-first kernel, so the seam is instantiable at a dual. What made that a finding was that the D1 ruling's three *smaller* residues each got a number (`ContentBits for Dual` → #687, the census box duplication → #700, the `Enclosure` gate gap → #701) and the one seam it left unguarded got prose. **Both halves of that premise have since moved**: the seam's written reason for needing no lane exists in one home — `real.rs`'s delegation rule (DUAL-DESIGN DL5), which `scripts/gates/bounds-allowlist.sh` points at rather than restating — and the ruled tightening is rowed as **`S90-impl` on Track M**, which carries #883. What stays open is the tightening itself, at these three doors.

**Verdict: the blend doors stay generic — `fillet_edges`/`chamfer_edges` at `T: Decide + Bounds + topo::AtRestPolicy`, `run_battery` and the battery predicates at `T: Decide + Bounds` — under DUAL-DESIGN DL5's delegation rule (the `real.rs` ledger's edge-blend entry), and the fillet third of the 2026-08-21 tightening (H-R3, *"tightening to `CertifiedBounds` works at least for now"*) is retired.** The blend certifies nothing: its battery decides on the value channel, its 18 bracket reads are payloads, discrete selections and value-channel refusal gates (DL5 (a)/(b)), and it mints no certificate. Its doors sit beneath `editor_core::eval::evaluate<T>`, which the E4 sensitivity tier runs at `Dual64` (DL3), so a `CertifiedBounds` bound would remove a working capability — measured: a filleted cube's `Dual64` volume tangent matches central differences of the `f64` build to ~1e-11, for the fillet radius and for parameters upstream of it. H-R3's other two doors (`chart_region_overlap`, the projection doors, #886) and its "the passes keep their lanes" half stand. What a dual blend owes is a pin: a tangent row against finite differences on `die_fillet`/`die_chamfer`, and a stack-up row that differentiates through a 3D blend.

**What the ruling does NOT do — and the distinction is Ev's, drawn on the evidence:** it does **not** delete the four lane traits. `CertifiedBounds` refuses at the **function**; a lane trait refuses at a **sub-operation inside a function that has non-certifying work to do**, and no bound on a whole function can say *"this arm needs certification, the rest does not"*. All four lane traits gate mixed passes, and `topo/tests/geometric_cube.rs:236` calls `validate_geometric` at `Dual64` and asserts it **succeeds**. Bounding that pass on `CertifiedBounds` would delete `Body<Dual64>`'s ability to go through a validation pass at all. **The doors tighten; the passes keep their lanes.** Full ruling and its scope: `docs/SMELL-H-LOG.md`, **H-R3**.

**Implemented in two PRs, split so a decision flagged *"at least for now"* is independently revertible.** **#886** took `topo::chart_region_overlap` and `geom`'s two projection doors: no capability lost, and a wrong answer (#874) made unreachable. **#883** carries the `sweep/fillet` third, which prices one — `Filleted<T>` carries a `Body<T>`, so a fillet stops being differentiable — and it is **parked, not landed**: implementing it turned up `fillet_edges` reachable from `editor_core::eval::evaluate`, a mixed pass, and its 12 red jobs reduce to that one `E0277`. It was then folded into `H5` and kept as `H-f`'s prototype, because `S3` records the blend battery as one of the two sites where the lane-trait pattern was *deliberately declined*. **Track M's `S90-impl` is where that now sits; the branch is a measurement, not a queued landing.**

**The choice this row turns on, kept because the ruling reads against it.** *"Harden this seam"* and *"keep duals out of this seam"* are the **same edit**: at plain `Interval` a caller hardens a `Decide + Bounds` seam by adding `CertifiedEnclosure`, but at `Dual<Interval>` that same upgrade **evicts**. So tightening is a decision, taken at the API, that the blend battery is not a differentiable surface. That is what *"at least for now"* hedges, and it is written out at S44's *"What this does NOT settle"* (carried at `H5`).

**Building a `PropsQuadLane`-style refusing lane is very likely the wrong shape here**: #643 already ships the type-level mechanism (`CertifiedEnclosure` is implemented for exactly `f64`, `Interval` and `Probe`, never for `Dual`, with `CertifiedBounds` as the sole-bound spelling), so a seam that wants duals out needs **a bound that does not type-check**, not a runtime refusal. Two of the three remaining lane traits are already redundant for the guarantee and only their typed refusals are load-bearing — see `C7`/`H5`.

## Fence

Track M — the scalar and certification traits. **Fence:** `crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`, `interval-transcendentals/`, `crates/bvh/`, `crates/topo/src/props.rs`. **Block:** `D220`–`D239` / `S290`–`S309`. (`crates/topo/src/props.rs` was drawn onto this track 2026-09-02; `crates/topo/src/validate.rs` stays Track P's.)

## Claimed by BLEND (2026-09-06)

Moved from `work/code-quality/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Track M's row on BLEND's files (`blend/{build,battery,surgery}.rs`); `cert` disclaimed it at its exit and FILLET said coordinate. It stays blocked in fact on the lane-trait split `H5` names; BLEND carries it so the per-read classification it owes has an owner.

## Re-homed at BLEND's exit (2026-09-17)

BLEND's unit 13, walked as blocked on PROPS' `H5` (the lane-trait
split) at BLEND's exit walk. Carried here because the blend seam's
three doors are `crates/sweep/src/blend/*`, CARVE's ground. What BLEND
owed and did not deliver, now this row's first step: the per-read
classification of the nineteen bracket reads, so that the day `H5`
lands the tightening to `CertifiedBounds` is one PR.

## Note from SCALAR (2026-09-29)

The 2026-09-02 CERT-M3 re-read's reason (`fillet_edges` is `T: Decide + Bounds + PcurveFittedLane`, the lane trait that does not split) is moot: LANE-4 (PR #3194) folded that trait into the door value `FittedLane<T>`, which `AtRestPolicy::fitted_lane()` answers, so the bound is now `T: Decide + Bounds + topo::AtRestPolicy`; `Dual` implements all three, so the `E0277` blocker it records stands.

The Finding's `CertifiedEnclosure` roster lost `RingInterval` when
RING-3 (#3153) dissolved it into `Interval`. The roster's *"exactly"*
also misses `Sym<T>` (`crates/geom-core/src/sym.rs`), which implements it
only over a `T` that does, so `Sym<Dual>` is excluded too and the
conclusion stands.

## Measured 2026-10-01 (BAND, step 1): the tightening needs a design fork decided first

**The doors, as of the tree today.** `fillet_edges` and `chamfer_edges`
(`blend/build.rs`) take `T: Decide + Bounds + topo::AtRestPolicy`.
`run_battery` and `run_battery_for` (`blend/battery.rs`) take
`T: Decide + Bounds`. **`ring_clearance` is no longer a public door.** It
is `pub(crate)`, and its only public spelling is
`ring_clearance_for_tests`, which is behind `cfg(any(test, feature =
"test-support"))` and re-exported as `test_support::ring_clearance`.
Outside `blend/`, nothing calls `run_battery`, `run_battery_for` or
`ring_clearance` generically. All three reach the build doors through
`fillet_edges_inner` and `chamfer_edges_inner`, so tightening any one of
the three means tightening `fillet_edges` too.

**The generic chain, found by compiling it.** I tightened both build
doors to `Decide + CertifiedBounds + topo::AtRestPolicy`, ran `cargo
check -p verbs -p editor-core` and reverted. The first `E0277` is no
longer in editor-core. It is `verbs::Verb::run` (`crates/verbs/src/run.rs`,
`impl<T: Decide + Bounds + topo::AtRestPolicy> Verb<T>`, at its `Fillet`
and `Chamfer` arms). Above that call the chain is
`editor_core::eval::evaluate<T: EvalScalar>`, then `wire::run_op`
(`EvalScalar`'s set), then `wire_blend<T: Decide + Bounds + AtRestPolicy>`,
then `Verb::run`. `Dual<T>` implements every member of `EvalScalar`
(`ContentBits`, `AxisScalar`, `SeedScalar`, `MinClearanceLane`,
`SectionScalar`, `AtRestPolicy`), each with a deliberate impl. All other
callers outside the blend module are concrete (`f64` or `Interval`) or
are re-exports (`sweep::{fillet,chamfer}`, `pncad::prelude`).

**Why editor-core is generic over `Dual`, and where it runs a fillet at
`Dual`.** It is the E4 sensitivity tier: `stackup::driver` (under
`stackup::sensitivities` and `stackup::stackup`) evaluates the whole
document at `Dual64`, one unseeded base and one seeded pass per
parameter, and pairs each pass with the `f64` anchor (DUAL-DESIGN DL2
and DL3). That is the only `Dual` instantiation of `evaluate` in shipped
code. Today a fillet at `Dual64` runs, and it runs green:
`m10_di_dual_corpus::every_document_evaluates_at_dual64_with_the_f64_value_channel`
iterates the whole registry, `die_fillet` and `die_chamfer` included,
and asserts that every node is `Ok` and the value channel is
bit-identical to `f64`. No sensitivity test or demo differentiates
through a 3D blend: the "filleted bracket" rows are profile arcs.

**So tightening forces a choice the ruling did not make.** Once
`Verb::run` cannot call the door at `Dual`, a `Dual64` evaluation of a
document with a fillet must do something at that node:

1. **Refuse typed at the node (the shell precedent).** The pattern is a
   door value on `AtRestPolicy` (`blend_door() -> Option<BlendDoor<Self>>`,
   `None` at `Dual`, its constructor bounded on `CertifiedBounds`), which
   `Verb::run` takes as `run_shell` takes `ShellDoor`, with
   `wire_blend` refusing `BlendLaneUnsupported { scalar }`. What it
   costs:
   - `die_fillet` and `die_chamfer` leave the green-at-`Dual64` corpus
     row, as `cup` and `vessel` sit outside the registry today.
   - `stackup::sensitivities` over any filleted or chamfered part stops
     answering. Whether it fails whole (`PairingViolation`, which is what
     `pair_pass` does today with the shell's refusal; filed as
     `work/stack/stackup-pairing-reads-the-shells-dual-refusal-as-a-violation`)
     or answers `Unliftable` per entry is a second choice inside this
     one.
2. **Run the blend on the value channel and lift it back with zero
   tangents.** The value channel would match, but every tangent
   downstream would be wrong, so DL3's "sensitivity of the as-built
   body" would be false. Rejected.
3. **Keep a `Dual`-admitting internal door that the verb or document
   path reaches.** That reopens the external `Dual` route the ruling
   closes. Rejected.
4. **Put option 1's cost to Ev before acting.** The `#883` pricing ("a
   fillet stops being differentiable") was recorded against the kernel
   door. That the E4 tier does differentiate through fillets today, and
   that a corpus row pins it, was found after the 2026-08-21 ruling.

Recommendation: option 1 with an `Unliftable` valve (a new `LiftRefusal`
arm), so a stackup over a filleted part reports the fillet node as the
place its sensitivity stops rather than failing whole. That reads
straight off DL3's door-value list and the shell precedent. It still
needs Ev's sign-off because it removes a capability that works today,
and "at least for now" was hedged without that capability in view.

**The bracket reads, re-counted and classified.** I counted `.lo()` and
`.hi()` outside comments in `blend/`, including `arms.rs`, `admit.rs`,
`naming.rs` and `open/`: **13 lines, 18 reads.** That is fewer than the
17 lines and 19 reads recorded on 2026-09-02, because the nine payload
fields in `battery.rs` now go through one `measured` constructor. The
helpers carry these reads to their callers (`classified`, `short_arm`,
`piece_distance` and `piece_along`, the last two used from
`open/ruled.rs`), so a pattern counting `.lo()` calls cannot see those
callers. The second pass, for `to_f64`, `Bounds::` and `width`, found no
other bracket spelling.

| site | reads | class |
|---|---|---|
| `battery::holds_enclosures` (`third.lo() < third.hi()`) | 2 | payload: picks the `MarginDiag` variant by measuring the type, decides nothing about geometry |
| `battery::measured` (`value.lo(), value.hi()`) | 2 | payload: every margin field in a refusal, via `classified` and `short_arm` |
| `battery::radius_headroom`, `spine_regularity` (`radius: radius.lo()`) | 2 | payload |
| `battery` junction-end pick (`d0.min(d1).lo() == d0.lo()`) | 2 | selection: picks which tangent feeds the chain-G1 decision |
| `build::nonpositive_size_gate` (`size.lo().partial_cmp(&0.0)`) | 1 | decision: refuses a size that is not positive |
| `build::nonpositive_size_gate` (`size: size.lo()`) | 1 | payload |
| `build::octant_chart` candidate score (`.norm().lo().abs()`) | 1 | selection: an `f64` score that picks the corner chart |
| `surgery::CircleFrame::misses` | 2 | selection: chooses between the ends and the whole circle (DL5(b), argued at the site); feeds `ring_clearance`'s margin |
| `surgery::CircleFrame::distance` (`rho.lo() <= 0.0`) | 1 | selection: on-axis guard |
| `surgery::CircleFrame::along` (`nu.lo() <= 0.0`) | 1 | selection: on-axis guard |
| `surgery::seam_split_param` period guard | 1 | decision on a representation datum: refuses typed |
| `surgery::seam_split_param` in-window test | 2 | selection or refusal on a representation datum |

Totals: 9 payload reads, 7 selection reads, 2 decision reads.

**The class is wider than three doors.** Eight more `pub` functions in
`battery.rs` take `Decide + Bounds`: `radius_headroom`,
`spine_regularity`, `convexity_at`, `chain_g1`, `corner_config`,
`face_clearance`, `run_battery_for` and `cap_transverse`. Whatever
tightens `run_battery` should tighten them in the same edit.
