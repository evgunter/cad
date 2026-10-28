# SHELL-3 — the clearance engine's body-level half moves into `topo`

Unit: `work/clear/SHELL-3.md` (read its `## Shape` section, and
`work/clear/symbolic-tier-and-clearance-engine.md`'s `## Decided`).
Program: CLEAR. Branch: `clear/shell-3-engine-into-topo` (cut from main after INTENT E merged). Review: one
FULL review (claims plus `docs/prompts/reviewer-style-lane.md`).

## What is true when this lands

1. **`topo::clearance` exists, in every build, and holds the engine's
   body-level half.**
   - It holds everything in `editor_core::clearance` that reads only a
     body, `bvh` and `geom-core`:
     - the carrier windows (`Window`, `window_of`, `refines`, `full_turn`,
       `TURN_CEILING`, `full_latitude`, `boundary_of`);
     - the cells (`Cell`, `CellPair`, `enclosure`, `cell_box`,
       `diameter`, `separation`, `split`, `Task`, `RegionMemo`,
       `Station`, `mid_of`);
     - the `Sweep`;
     - the bound walk and `min_separation`, with their config, receipt
       (`CellReceipt`, `DischargeWidths`, `CellBudget`) and constants
       (`CLEARANCE_MARGIN`, `SELF_INTERSECTION_GAP`,
       `DEFAULT_MAX_CELL_DEPTH`, `DEFAULT_MAX_MIN_SEP_PAIRS`).
   - The engine-level refusals move with them. Split `ClearanceRefusal`
     along the seam: `topo` owns the arms the engine raises, and
     `editor-core` wraps them in its own type beside the selection and
     drive arms.
   - Funnel site names do not change. They are census row keys.
2. **The door is generic over the lane scalar, and the arithmetic
   stays `Interval`** (the decided shape).
   - The walks take `Side<'b, T> { body: &'b Body<T>, faces: Vec<FaceKey> }`
     for `T: Decide + CertifiedBounds`.
   - Each surface, curve and point the door reads is lifted through
     `Interval::from_certified`, as `QuadLane::certified()` does.
   - `topo::cell_region` and `station_region` go generic the same way:
     point-in-face decisions run at `T` through the existing generic
     doors. The compound bound rides the quad lane's
     `scripts/gates/bounds-allowlist.sh` entry, with its reason.
   - A computed station handed to a generic door enters as
     `T::from_computed`, never `from_f64`.
   - At this step every caller passes `Interval`. `MinClearanceLane`,
     `Sym`'s `None` and the drive's refusal are NOT touched; they are
     the next unit's.
3. **`Side` carries no recipe address and no frame.**
   - `RecipeNodeId` and the body index stay in `editor-core`, which
     attaches them to whatever the walk returns.
   - The same-body question becomes an explicit input to the walk, not
     an inference from addresses.
   - `editor-core` reads both operands into one frame before calling.
     INTENT E (merged, `6f1c805c25`) already does this in
     `clearance_with`, through `Framed::computing_among` and `read_in`.
     `verify_witness` repeats the same choice at `f64`. Keep both in
     `editor-core`.
   - `Window`'s `at` and `body` fields are attribution and leave the
     moved type. `editor-core` maps a window index back to its
     selection, and so to its node and body index, for `verify_witness`
     and for refusal payloads.
4. **Witness verification stays in `editor-core`.**
   - `verify_witness` rebuilds the `Doc` at `f64`, so it cannot move.
   - The bound walk takes a verifier: a callback the sweep calls on each
     candidate violation, in its existing shallowest-first order.
   - The sweep's order and stopping rule are unchanged: the first
     verified witness ends it, and an unverified one is the
     `WitnessUnverified` arm as today.
   - SHELL-4 will hand `topo`'s own walk a document-free verifier, so
     the callback is the general form, not an editor-core convenience.
5. **`editor_core::clearance` keeps the document half.** That is
   `ClearanceQuery`, `Selection`, `FaceScope`, `SelectionRefusal`, the
   report, `clearance`, `self_intersection`, `clearance_with`,
   `clearance_over`, `LeafFold`, `LeafAnswer`, `ClearanceMass`,
   `facet_restrict`, the monotone oracle, `witness_point` and
   `verify_witness`. It calls down. The `min_clearance` measure calls
   `topo::clearance::min_separation`.
6. **No behaviour change.**
   - The M10-5/M10-6 suites, `m10_6_certifying_keys.txt` and every
     clearance golden are the differential: none moves.
   - If one moves, stop, find out why, and report it before going on.
     A move is a finding, not a re-pin.
   - The suites stay where they are, except rows that test only moved
     internals, which follow them into `topo`.

## Out of scope

- The self-intersection candidate filter (`work/clear/self-intersection-…`,
  `[ev]` #5866): move the wedge rule as it is.
- The four refusal-text rows: move the texts as they are.
- `Sym`, `MinClearanceLane` and the replay lane: the next unit.

## Ground

- `crates/topo/src/` is TOPO's ground. Run
  `python3 scripts/work.py territory --base origin/main` and announce
  the seam in the PR.
- Cite by name, and keep the module docs true. `clearance.rs`'s header
  splits between the two modules, and each half states only what its
  own module does.
- Read `docs/prompts/implementer-discipline.md` in full first.
- Never arm a waiter or a background chain for your own builds or
  tests. Launch any job that outlives a 600 s foreground call
  `setsid`-detached, then poll it in the foreground.
- CI is the verification of record. This diff seeds `topo`, so allow
  25 min.

## Report

≤150 lines:
- what moved and where;
- the verifier's shape;
- the `ClearanceRefusal` split;
- the sweep hit list (§5 of the discipline) for any reader of the moved
  symbols left behind;
- CI on the final head.
