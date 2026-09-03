---
id: S90-impl
kind: unit
title: Tighten the blend seam's three doors to CertifiedBounds — the ruled S90 implementation, with #883 parked here
status: open
opened: 2026-08-21
track: M
pr: 883
refs: [H5, 867, 886, s90-tightening-cost-measured]
---

## What

The largest D1 residue's implementation, and **#883 is parked on this track's ground** (H-g PR 1, folded into `H5`). **TAKEABLE — `S90` is RULED.** #867 merged 2026-08-21 07:14Z: *"tightening to `CertifiedBounds` works at least for now."* That is `H-R3`, #886 implemented it at two of three sites, and it is why #883 exists. **#883 is parked on a RULING, not on `S90`** — folded into `H5` because the fillet seam is one of the two sites where the lane-trait pattern was *deliberately declined* (`S3`), so its work and `H5`'s collapse are one argument. **Read `H-R16` before starting either.** **Re-read against the tree 2026-09-02, as a measurement and not an unparking** — three things moved and one did not. (i) The blocker STANDS: `sweep::blend::fillet_edges` is still reachable from `editor_core::eval::evaluate`, through `eval::wire::run_op`'s `T: Decide + ContentBits + Bounds + Send + Sync + AtRestPolicy + AxisScalar` to `wire_fillet`, and that set admits `Dual`, so #883's twelve red jobs still reduce to that one `E0277`. (ii) The pass is still mixed, and the split still sits where H-g put it: the geometry builder holds ZERO bracket reads at `T: Real`. (iii) **Every #883-era citation needs re-aiming**: the module is `crates/sweep/src/blend/`, not `sweep/src/fillet/`, and the builder is `blend/arms.rs`, not `blend.rs`. (iv) **H-g's bracket-read count of 14 no longer holds, and the unit matters**: 17 LINES carry a bracket read, 19 READS in all — `battery.rs` 11 lines / 12 reads, `build.rs` 3 / 3, `surgery.rs` 3 / 4, `arms.rs` (the geometry builder) 0. The classification also needs correcting rather than repeating: they are NOT all typed-error payloads. In `battery.rs` nine are error-payload fields (`margin`/`radius`/`gap`/`arm`), `:1235` is a BRANCH CONDITION comparing two bracket lows (`d0.min(d1).lo() == d0.lo()`, a junction-end pick) and `:1289` is a value read feeding an f64 quantity; `build.rs` is one `partial_cmp` datum gate plus two payloads; `surgery.rs`'s four are representation-datum selections, each with its argument written at the site. So the population is payloads plus SELECTIONS, and a taker owes a per-read classification rather than either count.

**Re-derived by compiling, 2026-09-03, and the row's TAKEABLE now has a
price attached — see `s90-tightening-cost-measured`.** The tightening
applies to all 35 signatures in `blend/{battery,build,surgery}.rs` with
`sweep` compiling CLEAN (lib, tests, examples, doctests); the cascade
downstream is three single-site hops (`verbs::Verb::run` → `wire_blend`
→ `run_op`) and then 39 `E0277` sites at `EvalScalar`, **none in `src`
and all in M10's dual suites, `e4_dual_door.rs`'s `evaluate::<Dual64>`
among them**. Fillets are not hypothetically differentiable but actually
so: three fillet/chamfer corpus documents evaluate at `Dual64` with a
bit-identical value channel, asserted and passing. **And the obligation
this row tracks was discharged on 2026-08-29 by `DL5`** (`DUAL-DESIGN`,
ratified #1146, landed at `real.rs:688-711`) — by ratifying the
delegation rule, i.e. the *"written reason it needs none"* half of the
same either/or, which is the half this row's re-read of 2026-09-02
cited without noticing it retires the line below. `H-R3` and `DL5` are
in tension at this one seam; that is Ev's to settle.

Status note: the row says TAKEABLE, so the item is `open`; `pr: 883` records the parked measurement branch, not a queued landing. The id `S90-impl` is carried as written (all characters are within the tracker's id alphabet).

## Was

Track H.

## Finding

### S90. The blend seam's three doors still admit a dual

- **Where**: `crates/sweep/src/blend/{build,battery,surgery}.rs` — `fillet_edges`, `run_battery` and `ring_clearance`, each still `T: Decide + Bounds` (`fillet_edges` with `+ PcurveFittedLane`). **The module was `sweep/src/fillet/` when this finding was raised**, and every citation below, #883's included, spells it that way.
- **Confidence**: sure

`Bounds` has a `Dual` impl since D1 and these are `pub` doors on an API-first kernel, so the seam is instantiable at a dual. What made that a finding was that the D1 ruling's three *smaller* residues each got a number (`ContentBits for Dual` → #687, the census box duplication → #700, the `Enclosure` gate gap → #701) and the one seam it left unguarded got prose. **Both halves of that premise have since moved**: the seam's written reason for needing no lane exists in one home — `real.rs`'s delegation rule (DUAL-DESIGN DL5), which `scripts/gates/bounds-allowlist.sh` points at rather than restating — and the ruled tightening is rowed as **`S90-impl` on Track M**, which carries #883. What stays open is the tightening itself, at these three doors.

**Verdict: ANSWERED — Ev, 2026-08-21: *"tightening to `CertifiedBounds` works at least for now."* Answer (4).** The fillet seam's three public entry points take `<T: Decide + CertifiedBounds>`, which makes an external `Dual64` instantiation a **compile error** rather than a thing an audit has to keep being true about. *"At least for now"* is part of the ruling and is recorded as such: this closes the seam, it does not settle whether a fillet battery should ever be differentiable.

**What the ruling does NOT do — and the distinction is Ev's, drawn on the evidence:** it does **not** delete the four lane traits. `CertifiedBounds` refuses at the **function**; a lane trait refuses at a **sub-operation inside a function that has non-certifying work to do**, and no bound on a whole function can say *"this arm needs certification, the rest does not"*. All four lane traits gate mixed passes, and `topo/tests/geometric_cube.rs:236` calls `validate_geometric` at `Dual64` and asserts it **succeeds**. Bounding that pass on `CertifiedBounds` would delete `Body<Dual64>`'s ability to go through a validation pass at all. **The doors tighten; the passes keep their lanes.** Full ruling and its scope: `docs/SMELL-H-LOG.md`, **H-R3**.

**Implemented in two PRs, split so a decision flagged *"at least for now"* is independently revertible.** **#886** took `topo::chart_region_overlap` and `geom`'s two projection doors: no capability lost, and a wrong answer (#874) made unreachable. **#883** carries the `sweep/fillet` third, which prices one — `Filleted<T>` carries a `Body<T>`, so a fillet stops being differentiable — and it is **parked, not landed**: implementing it turned up `fillet_edges` reachable from `editor_core::eval::evaluate`, a mixed pass, and its 12 red jobs reduce to that one `E0277`. It was then folded into `H5` and kept as `H-f`'s prototype, because `S3` records the blend battery as one of the two sites where the lane-trait pattern was *deliberately declined*. **Track M's `S90-impl` is where that now sits; the branch is a measurement, not a queued landing.**

**The choice this row turns on, kept because the ruling reads against it.** *"Harden this seam"* and *"keep duals out of this seam"* are the **same edit**: at plain `Interval` a caller hardens a `Decide + Bounds` seam by adding `CertifiedEnclosure`, but at `Dual<Interval>` that same upgrade **evicts**. So tightening is a decision, taken at the API, that the blend battery is not a differentiable surface. That is what *"at least for now"* hedges, and it is written out at S44's *"What this does NOT settle"* (carried at `H5`).

**Building a `PropsQuadLane`-style refusing lane is very likely the wrong shape here**: #643 already ships the type-level mechanism (`CertifiedEnclosure` is implemented for exactly `f64`, `Interval`, `RingInterval` and `Probe`, never for `Dual`, with `CertifiedBounds` as the sole-bound spelling), so a seam that wants duals out needs **a bound that does not type-check**, not a runtime refusal. Three of the four existing lane traits are already redundant for the guarantee and only their typed refusals are load-bearing — see `C7`/`H5`.

## Fence

Track M — the scalar and certification traits. **Fence:** `crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`, `interval-transcendentals/`, `crates/bvh/`, `crates/topo/src/props.rs`. **Block:** `D220`–`D239` / `S290`–`S309`. (`crates/topo/src/props.rs` was drawn onto this track 2026-09-02; `crates/topo/src/validate.rs` stays Track P's.)
