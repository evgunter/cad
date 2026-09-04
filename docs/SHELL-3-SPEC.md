# SHELL-3 — the clearance engine's body-level half moves into `topo`

**Status: DRAFT (SHELL orchestrator, 2026-09-04)** — ratified at
dispatch, which waits for M10-7 (#1725) to merge and for M10's
co-review to be asked. Executes Ev's ruling B on `[ev]` #1737
(`work/shell/shell-curved-clearance-consumer.md`). Binds the
implementer of unit `SHELL-3`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first.

## 0. What this unit is, and what it is not

`crates/editor-core/src/clearance.rs` (2.9k lines) holds two engines
in one file. The OUTER one is the document's: a `Selection` resolved
through the leaf's `Evaluation<Interval>`, the E6 leaf fold
(`clearance_over`, `LeafFold`, `ClearanceMass`), `facet_restrict`
over a `ParamBox`, and the `f64` witness rebuild that re-evaluates
the document at the leaf's midpoint (`verify_witness`). The INNER one
reads nothing but a `Body<Interval>`: `window_of` (a face's carrier
window from its surface and boundary box), `Cell` / `CellPair` /
`split` / `enclosure` / `cell_box` / `separation` / `refines` /
`boundary_of` / `in_plane_axis` / `chart_frame`, the
level-synchronous `Sweep` frontier, and `min_separation` with
`MinSepSelection` / `MinSeparationConfig` / `MinSeparation` /
`CellReceipt` / `DischargeWidths` / `CellBudget`.

This unit moves the inner engine DOWN into `topo` (behind `topo`'s
existing `interval` feature) as `topo::clearance`, and leaves the
outer engine in editor-core calling it. **No behaviour change**: the
M10-5 and M10-6 suites, their goldens (`tests/golden/m10_6_*`), the
K rows and the serialized receipts are the differential, byte for
byte. It is a MOVE with one seam made explicit (§2); it is not a
redesign, not the shell gate (SHELL-4), and not a change to any
verdict.

## 1. The cut

Moves to `crates/topo/src/clearance/` (a module directory; the file
is too large for one page): everything in the list above plus the
body-level refusal arms. Stays in editor-core: `Selection`,
`FaceScope`, `SelectionRefusal`, `windows_of` (it reads an
`Evaluation`), `clearance` / `self_intersection` / `clearance_with` /
`clearance_over`, `LeafFold`, `LeafAnswer`, `ClearanceMass`,
`ClearanceReport` (its `render` uses `report::percent` /
`mass_bits`, its `content_key` uses `eval::key_of`), `ParamWitness`,
`Violation`, `facet_restrict`, `MonotoneOracle` / `NoTangents` /
`Pruning` (the accelerator restricts a PARAMETER box), and
`verify_witness`. `GeometryWitness` moves (it is points and keys).

Two types split along the seam, and each split is a finding to
disclose if it turns out differently in the tree:

- **`ClearanceRefusal`.** The body-level arms — `Sliver`, `Budget`,
  `Unsupported`, `PoisonEnclosure`, `NotADistance`, `EmptyScope`,
  `ToleranceHasNoBand`, `NoAdmittedPair`, `WitnessUnverified` — move
  as `topo::clearance::ClearanceRefusal`; the document-level arms
  `Selection(SelectionRefusal)` and `NothingCertified` stay in an
  editor-core enum that wraps the moved one (`Engine(topo::…)`).
  Every `Display` text is unchanged. `MinClearanceRefusal`
  (`measure.rs`, the stringly twin M10 filed) is NOT touched — its
  issue is M10's.
- **`Window`.** Its `at: RecipeNodeId` / `body: u32` fields exist so
  the `f64` witness rebuild resolves the same face at its own node
  (the docs say why: two nodes can carry the same arena key). The
  moved `Window` carries an opaque attribution instead — a caller-
  supplied `Attribution` (`Copy + Ord + Debug`, a generic parameter
  or a small `u64`-pair newtype; choose the one that keeps the
  `same_body` rule and the wedge rule readable) — and editor-core
  packs `(RecipeNodeId, u32)` into it. `MinSepSelection` keeps its
  public field names; `at`/`index` become the attribution.

## 2. The witness seam

`Sweep::run` takes `doc` and `leaf` for exactly one reason:
`verify_witness` rebuilds the geometry at `f64` through the document.
The moved sweep takes a **witness verifier** instead:

```rust
pub trait WitnessVerifier {
    /// Re-verify a candidate violation at `f64` by an independent
    /// rebuild. `Err(what)` is the engine's `WitnessUnverified`.
    fn verify(
        &self,
        at: (&Window, Cell, &Window, Cell),
        bound: ClearanceBound,
        band: Band,
        tol: Tol,
    ) -> Result<GeometryWitness, String>;
}
```

editor-core's impl is today's `verify_witness` with `doc` and `leaf`
captured. A verifier that cannot rebuild answers `Err`, and the
engine then refuses `WitnessUnverified` exactly as it does today for
an unconfirmed witness — which is the arm SHELL-4's verb-side
consumer will land on (no `f64` body exists inside `shell::<Interval>`),
and why the seam is a trait rather than an `Option<&Doc>`: the
absence of a rebuild is a typed refusal at the site, not a degraded
verdict. Nothing else about the sweep changes: same candidate order,
same level-synchronous frontier, same stop at the first verified
violation, same receipts.

## 3. Feature and dependency shape

`topo` already forwards `geom-core/interval` as its `interval`
feature; the module is `#[cfg(feature = "interval")]` whole, and
`bvh` is already a `topo` dependency. editor-core's `interval`
feature already forwards topo's? — measure; if not, add
`"topo/interval"` to it. No new crate, no new dependency edge in
either direction (G1 layering untouched: editor-core still depends
on topo, never the reverse).

## 4. Acceptance

1. **The differential is byte-exact.** Every M10-5/M10-6/M10-7 row
   green with NO golden re-blessed; `ClearanceReport::serialize` and
   `MinSeparation::serialize` output identical on every fixture the
   suites run (a probe that diffs the two serializations at the merge
   base and at HEAD, committed as the PR's own receipt). The K
   population under `clearance_margin` / `self_intersection_gap` is
   unchanged in count (the k-lint gate is the register).
2. **The moved engine has a topo-level row**: `min_separation` on a
   hand-built `Body<Interval>` in `crates/topo/tests/` (the box and
   the tube from the shell suites, lifted to `Interval` the way
   `tests/interval_body.rs` does), asserting the bracket the M10-6
   suite already asserts through the document — the same numbers,
   reached without a document, which is the point of the move.
3. **The verifier seam refuses typed**: a `WitnessVerifier` that
   always answers `Err` turns the M10-5 dumbbell's `Violated` into
   `WitnessUnverified`, with the receipt otherwise identical.
4. **Layering is pinned**: `crates/editor-core/Cargo.toml`'s G1 note
   still holds (no kernel crate gains an editor-core edge —
   `cargo tree -i editor-core` from topo is empty), and
   `scripts/gates/*.sh` all pass.
5. `cargo doc` links: every intra-doc link that crossed the seam
   resolves (the rustdoc gate reds otherwise).

## 5. Docs

`topo/src/clearance/mod.rs` gets the module header's body-level half
(the trichotomy's INNER mechanism, the window looseness, the two
funnelled compares, the witness seam); editor-core's header keeps the
document half and points down. `docs/DESIGN.md`'s tier-3 note
already names the home (#1737). `crates/topo/README.md` gains the
row; `crates/editor-core/README.md`'s clearance row is corrected.

## 6. Owed to neighbours

- **M10**: the file is M10's; ask its orchestrator to co-review
  (the joint unit), and land the `Sym<T>` seam of #1725 first.
- **PROPS**: `interval-orthonormal-basis-sign-hull` names a
  workaround inside `chart_frame`; move it verbatim and cite the
  item at the new site.
- **SHELL-4** consumes `min_separation` and the strictly-positive
  bound from `topo` and supplies the always-`Err` verifier.

## 7. Stops

STOP and report if the inner half reads anything of the document
beyond what §1 lists (a hidden dependency is a finding about the
cut, not something to move along with it), or if a golden moves.

## 8. Lane rules

As every SHELL brief: own worktree, own `CARGO_TARGET_DIR`, never
the orchestrator's checkout, one heavy cargo job at a time, no
`Co-Authored-By` trailer in lane commits, push after every coherent
step, hosted CI is the gate (ask for `lane=interval` on the head
commit — every row here lives behind that feature).
