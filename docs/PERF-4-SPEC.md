# PERF-4 — D1's tier-1 postcondition runs once per public door

**Status: ratified at dispatch (PERF orchestrator, 2026-09-10).** Binds
the implementer of unit `PERF-4`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. Executes Ev's ruling on `[ev]` PR 2305
(`work/perf/d1-per-op-tier1-sweep-price.md`, "Ruled"): the ruling is
the authority for changing a ratified clause, and this spec is its
faithful elaboration — where the two differ, the ruling wins and the
difference is a finding to report.

## 0. What changes, and what does not

Today every Euler operator and every non-operator structural mutator
ends with `assert_euler_postcondition` (`crates/topo/src/euler.rs:2394`,
26 call sites across `euler.rs`, `euler_kill.rs`, `euler_ring.rs`,
`movefac.rs`, `null.rs`, `split.rs`, `boolean/voids.rs`), which checks
the declared `ArenaDelta` (O(1)) and then runs the **whole-body tier-1
validator** (O(body)); the attach setters `set_face_surface` /
`set_edge_curve` (`attach.rs:93,332`) run the same whole-body sweep
after writing one reference. A composite door — `sweep::extrude`
(`extrude.rs:718` ends with `validate_closed`), `revolve`, `loft`,
`fillet_edges`/`chamfer_edges`, `topo::boolean_op_with`, `shell`,
`offset_*`, `split`, `merge_coplanar_faces`, the instance graft — runs
tens to hundreds of operators and so pays tens to hundreds of
whole-body sweeps for one door, on top of its own door-level check.
Measured (`work/perf/log.md`, kernel and developer lanes): 1470 sweeps
for `die`'s 84 nodes, 45–72 % of a rebuild in the shipped release
profile; 31 % of editor-core's test wall.

After this unit:

- **Inside a door, no operator sweeps the body.** The `ArenaDelta`
  check stays at every operator (it is O(1) and it is the op's own
  declared contract).
- **Every public door sweeps the whole body once, at its end**, in
  debug builds, exactly as the operators did — tier 1 via
  `crate::validate::validate` (a door that already ends with
  `validate_closed` or a stronger tier keeps that; it subsumes tier 1
  and nothing is added).
- **An operator called directly by a consumer is itself a door** and
  still sweeps at its end: the observable guarantee — every public
  mutation path preserves tier 1, checked at every observable
  boundary — is unchanged.
- **Localization on failure is recovered, not paid on success.** A
  door-level failure names the door; to name the operator, the
  per-operator sweep is restored by a scalpel — an opt-in cargo
  feature on `topo` (the `shadow-exec` precedent, `work/perf/plan.md`
  §4.5), or a replay of the door's operator sequence from provenance
  with per-op checks on. Choose the cheaper one that actually names
  the op; say why. The ruling permits either.

What does not change: the operators' preconditions, atomicity, typed
errors, minting order, provenance, and release-build behaviour (no
postcondition is compiled into release today beyond what
`debug_assertions` gates, and that stays as it is — this unit does not
touch `[profile.release]`).

## 1. The mechanism — a surgery scope, explicit at both ends

The operator has to know whether it is inside a door. The shape:

- A debug-only depth on `Body` (`#[cfg(debug_assertions)] surgery:
  Cell<u32>` or the equivalent; zero cost and zero bytes in release —
  say how you keep the struct's release layout and its serialized form
  untouched, since bodies persist through `editor-core`'s document
  format only by content and not by struct layout, and prove it with
  the persist suite).
- `assert_euler_postcondition` keeps the delta check unconditionally
  and runs the tier-1 sweep only when the depth is zero.
- A door opens the scope at entry and closes it at exit **explicitly**:
  `close` decrements and, at depth zero, runs the sweep. A door that
  returns `Err` mid-sequence must still decrement — an RAII guard that
  decrements on drop and a separate explicit `finish` that sweeps is
  the honest split (never sweep in `Drop`: a `debug_assert` firing
  during unwinding aborts the process and hides the original error).
  A scope left open by a bug is itself a bug: at depth zero a door
  that finds the depth already nonzero on entry announces it
  (`unreachable!`, D9 row 4, with the values).
- Nesting is a depth, not a flag, because doors compose (a boolean
  calls `split`, a shell calls `replace_faces_offset`, and the outer
  door is the observable boundary).

Alternatives you may take instead, if measured or argued better: a
door-level "sweep at end" with the operators keeping a `_unchecked`
twin is REJECTED (two spellings of every operator, the shape D9's
conventions and `reviewer-style-lane.md` Q1 exist to refuse); staging
into a fresh body and committing on success is the shape some doors
already use and is orthogonal.

## 2. The closure property, re-pinned

`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
(`crates/topo/src/review_m1_pr5_internal.rs:385`) walks every public
`&mut self` / `&mut Body` door and requires the literal
`assert_euler_postcondition(` or an allowlist entry with its reason.
After this unit the property is the same and the needle is different:
a door either (a) ends with the per-op assertion (an operator called
directly), or (b) opens a surgery scope and closes it with the sweep,
or (c) is composed of doors that do, or (d) writes fields tier 1 does
not constrain — and the walk must be able to tell (a)–(b) from a door
that opened a scope and never closed it. Extend `source_walk` and the
test so both spellings are recognized and an open-without-close is a
failure; keep the allowlist honest in both directions as it is now.
The same holds for `sweep`'s doors: they are public mutation paths in
another crate, and their door-level `validate_closed` calls are the
sweep — check that every `sweep` door that runs operators either
already ends with one or gains one.

`review_d18_probes.rs` and `review_m1_pr2/release_corruption.rs` cite
`assert_euler_postcondition`'s messages and premise; re-read both
against the new behaviour and fix what rots (a message that now fires
once per door, a premise that assumed per-op).

## 3. The pin, and the measurement

- **Nothing tier-1-invalid becomes reachable.** Every existing test
  that plants corruption and expects the postcondition to fire must
  still fire — at the door instead of the op — and the row that pins
  it says so. Add one row per door class (an operator direct, a
  `sweep` door, a `topo` composite, a nested composite) that plants a
  mid-sequence corruption and shows the door-level sweep catching it,
  plus the scalpel naming the op.
- **Bit identity.** Every body, name table and mesh across the corpus,
  the tour and the gallery documents is byte-identical (assertions
  decide nothing; the only behavioural change is which assertion
  fires); the render lanes and the tess-budget baseline do not move.
- **Measurement** (report before/after with spread, 4 vCPU under the
  build slot): `die` and `die_composed_tour` full rebuild in release
  with the shipped profile (`da_on`) — expect the 78–82 ms and 90–95
  ms rows to approach their `da_off` figures (36 and 26 ms) with the
  per-node door sweep as the residual; `demos/wild` (226 → ~54 ms);
  the editor-core, topo and sweep test binaries' nextest execution
  wall in CI's profile (the developer lane's table: 10.85 / 3.86 /
  10.63 s); `benches/` `kernel/build/extrude` and `kernel/boolean/
  two_bricks` in the bench profile (debug assertions off — expect no
  move; a move is a finding).

## 4. The documents

This unit revises the ratified text, present tense only:

- `docs/DESIGN.md` D1 (the debug-postcondition clause) and D9's
  closure-property paragraph ("every public mutation path preserves
  tier 1 — the Euler operators by the soundness theorem, the
  non-operator structural mutators by declaring the same debug
  postcondition …"): state the once-per-door rule and cite the ruling
  by item id (`work/perf/d1-per-op-tier1-sweep-price`, Ev, PR 2305) in
  one clause; no history narration.
- `crates/topo/src/euler.rs` module docs ("Debug postconditions (D1's
  ratified clause)") and `attach.rs`'s.
- `work/perf/plan.md` §1.3's row for this cost center is deleted by
  the orchestrator at merge; do not edit the plan.

## 5. Out of fence

`[profile.release]`; the validators themselves; any operator's
semantics; `sweep`'s door-level tier choice (a door that checks tier 2
keeps checking tier 2). TOPO territory (`crates/topo/*` less its
keep-outs) and SEAT/BLEND ground in `sweep` — announced in
`work/perf/log.md`; TOPO's dispatched unit is the census readback door
and does not touch these files, but merge `origin/main` before opening
the PR and re-check.

## 6. Report

≤150 lines: the scope mechanism and how release layout is proven
untouched, the localization scalpel chosen and why, the door census
(every public door and which of (a)–(d) it is, with the walk's new
needle), the corruption rows, and the measurements of §3.
