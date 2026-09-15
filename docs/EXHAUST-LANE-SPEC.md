# EXHAUST-LANE — the exhaustiveness receipt carries its lane, with the chart lane's `SupSpeed`; metres by one method

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-15).** Binds
the implementer of unit `exhaustiveness-receipt-carries-its-lane`;
deleted at merge per `docs/DOC-LEDGER.md`. Read
`docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/exhaustiveness-receipt-carries-its-lane.md`; the ruling is
`work/scalar/D283.md` §RATIFIED, route A (PR 2457) — its unit (ii),
`D283.md:115-132`; the first unit landed as PR 2657 (`SupSpeed`/`InfSpeed`
in `crates/geom-core/src/predicate.rs`).

## 0. The ruling this executes

The parameter ↔ metres crossing is a typed rate whose bound direction
is the content. The SSI exhaustiveness receipt reports a floor "in the
lane's own units — meters for the ℝ³ lane, chart units for the chart
lane" (`exhaust.rs:92-93`), the error's `cell_width` doc hedges the
same way (`ssi.rs:260`), and neither `Display` names a lane or a unit;
the one test that reads a chart-lane floor back in metres pins the
fixture's speed by hand (`WALL_CHART_SPEED`, `m5_pr7_ssi.rs:1750`)
because the public API cannot recompute it. The receipt says which lane
it is and carries the rate that lane used, and metres are derived by
one method.

## 1. What this unit delivers

**The tag.** In `crates/geom-brep/src/ssi/exhaust.rs`, beside
`Exhaustiveness`:

```rust
pub enum ExhaustLane { R3, Chart { speed: SupSpeed<f64> } }
```

`Clone, Copy, Debug` (no `PartialEq` — `SupSpeed` has none, by the
`Real` surface's rule). `Exhaustiveness` gains `lane: ExhaustLane`;
`floor` stays in the lane's own units (the units `cell.width() <= floor`
was decided in, `exhaust.rs:209`) and its doc says so without the
hedge. `SsiError::ExhaustivenessInconclusive` gains the same `lane`;
`cell_width` and `floor` stay in lane units. The derived `Default` on
`Exhaustiveness` (`exhaust.rs:71`, used at `:184`) goes: the one writer
(`sweep`, `:183-186`) builds the receipt from the lane it was handed —
`account_r3` passes `ExhaustLane::R3`; `account_chart_plane` takes the
chart lane's `SupSpeed<f64>` and the METRES floor, converts once
(`speed.to_param(floor_m)`) and passes `Chart { speed }` — so
`plane_nurbs_ssi`'s accounting division (`ssi.rs:1058`) moves into the
door and the seed-floor and tube-pad divisions (`:1011`, `:1046`) stay
where they are, all three through the same local `speed`.

**Metres by one method.** `Exhaustiveness::floor_meters(&self) -> f64`
and, on the error, `cell_width_meters()`/`floor_meters()`, all through
ONE private function `fn meters(lane, x) -> f64 { match lane { R3 => x,
Chart { speed } => speed.to_meters(x) } }`. No dimensional newtype (the
ruling's rejected alternative, `D283.md:100-108`). `Display` for the
error names the lane and prints the lane-unit value with its unit word
and the metres reading beside it, e.g. `… a cell of width {cell_width}
chart units ({m} m) at the refinement floor {floor} chart units ({m} m)
on the chart lane …`; the ℝ³ arm prints metres once. Give
`Exhaustiveness` a `Display` of the same shape (it has none today;
every reader prints `Debug`).

**The helper retires.** `assert_floor_is_the_meters_floor_over_the_chart_speed`
and `WALL_CHART_SPEED` (`crates/geom-brep/tests/m5_pr7_ssi.rs:1735-1768`)
are deleted; their two callers (`:1883`, `:1897`) assert
`e.floor_meters()` against the metres floor the row asked for, within
the tolerance the helper used (`4 · ε · meters`) — the tolerance is the
`to_param`-then-`to_meters` round trip and its doc says so. No pinned
speed remains in the test.

**Siblings in the fence, same class, same PR:** the bare divisions
`radius / su`, `radius / sv` through a local speed closure in
`crates/geom-brep/src/ssi/certify.rs:856-870` are metres over a
derivative-box sup — the twin of `ssi.rs:1046`; mint the `SupSpeed` where
the closure reads the box and go through `to_param` (one operation,
bit-identical). `certify.rs:450-459`'s `levered_inv(res, speed)` and
`:692-709`'s transverse stretch are levers, not this crossing — list
them with the reason. `march.rs`'s speed guards are pointwise (out by
the ruling).

**What must not change:** every verdict and every `floor`/`cell_width`
value, bit for bit; the `cell.width() <= floor` decision is untouched;
the k-lint gate's predicate counts do not move (no decide is added or
removed). The `Debug` text of both types gains the `lane` field — say
so; no golden, render or Python surface prints it (`Exhaustiveness`,
`ExhaustivenessInconclusive` and `SsiError` have no Python binding and
`pcurve_cache.rs:1583`'s `CacheFault` arm reads no field).

## 2. Docs

`exhaust.rs`'s module doc says the receipt carries its lane and that
metres come from `floor_meters` — one sentence, pointing at the rate
pair's home for the direction rule; no restatement. `crates/geom-brep/README.md:113,117`
re-worded where it describes the receipt. No history.

## 3. The pin

- D9 differential: `geom-brep` suites green unchanged at default and
  `--features interval`.
- A row per lane that `floor_meters()` is the lane-unit floor on ℝ³ and
  `speed.to_meters(floor)` on the chart lane, bit for bit against the
  bare arithmetic; the two rewritten `m5_pr7_ssi.rs` assertions.
- The `Display` text of both types on both lanes, asserted (the lane
  word and the unit word are the claim).
- A compile-time fact: `ExhaustLane::Chart` cannot be built from an
  `InfSpeed` (a compile-fail doctest with a compiling twin, as PR 2657
  did).

## 4. Sweep

The class: a receipt or error that carries a length whose unit depends
on which lane produced it, without saying which. Over `crates/geom-brep/src/ssi*`
and `crates/geom-brep/src/*.rs`: every `struct`/`enum` field named
`floor|width|pad|radius|extent|step` whose doc says "meters (or …)",
"in the lane's units" or names two units; disposition each (this unit /
metre-only, say so / filed). `SsiError::StepCollapsed{step_meters}` and
`TubeLadderEmpty{extent, floor}` are metre-only — their names or docs
say so; check and say. State the blind spot (a field whose unit is
stated nowhere at all).

## 5. Fence

This program claims no paths. This unit reaches TRIM's ground behind
PCURVE P-2 (`crates/geom-brep/src/ssi.rs`, `ssi/{exhaust.rs,certify.rs}`
— in no program's literal `paths:`, claimed by TRIM's, PROPS' and
BOOL's keep-out prose), `crates/geom-brep/README.md`, and
TCOST/TINT's `crates/geom-brep/tests/m5_pr7_ssi.rs`. Announced by the
orchestrator; merge `origin/main` before opening the PR; territory
output in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p geom-brep` at default features and
`--features interval`; `cargo clippy -p geom-brep --all-targets -- -D
warnings` at default AND `--all-features` (feature-gated probes read
these types); `scripts/doc-gate.sh` and `scripts/doc-gate.sh
--skip-viewer-toolkit`. Hosted CI is the verification of record; poll
to conclusion in the foreground. Report ≤100 lines: the tag and the one
method, the door signature that moved, the retired helper, the sibling
divisions, the differential's receipt, the sweep table, deviations,
rows filed and where, PR number, head SHA, CI run id and conclusion.
