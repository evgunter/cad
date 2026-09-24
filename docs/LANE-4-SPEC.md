# LANE-4 — `PcurveFittedLane` folds into a `FittedLane` door value the policy answers

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-24).** Binds
the implementer of unit LANE-4; deleted at merge with a note under
`docs/doc-ledger/`. Read `docs/prompts/implementer-discipline.md` in full
first. The item is `work/scalar/lane-4-fitted-lane-folded.md`. The ruling
is `work/scalar/H5.md` §RATIFIED ruling 3 (PR 2701): the lane traits go,
replaced by hook values chosen at one per-scalar seam. The precedents on
main are LANE-0 (`geom_brep::OffsetFitLane` answered by
`AtRestPolicy::offset_fit_lane()`), LANE-3 (`topo::ShellDoor` answered by
`AtRestPolicy::shell_door()`) and LANE-4P (#3165: every door value's
wiring pinned by one helper, and `certified_enclosure_impl_census.rs`
enumerating door values). The survey is
`/home/user/scalar-briefs/survey-lane4.md` (at `6709bc3229`; a compile
experiment was refused there, so its "wider than needed" claims are by
reading only). Re-derive every count and line at your merge base.

## 0. The two questions H5 left, answered

**The representation question** ("what is a fitted pcurve cache with no
certificate"): none exists, and none is needed. Both fitted producers
refuse at check 4 (`FittedLaneUnsupported`) before any cache exists, so
a `None` hook is a refusal, byte-identical to today's. No type change;
do NOT add a certificate-less variant (`PcurveCertificate::ssi: None`
already means "closed-form lane"). Say this in one sentence in the PR.

**Where the `None` comes from** (orchestrator's ruling, logged
2026-09-24): **the policy read throughout.** Every consumer — the
construction passes and `validate_pcurves` alike — takes the door from
`AtRestPolicy::fitted_lane()`. No verdict moves at any scalar. The
declined alternative (the `_structural` validators hand `None`) would
make `f64` structural validation refuse a body the `f64` mint built.

## 1. What this unit delivers

1. **`geom_brep::FittedLane<T>`** in a new
   `crates/geom-brep/src/fitted_lane.rs` (beside `offset_fit_lane.rs`):
   `Copy`, three private fn-pointer fields with the shared bodies'
   signatures (`fitted_lane`, `general_image_lane`, `chart_foot_lane`),
   one `const fn certified()` in an `impl<T: Decide + CertifiedBounds>`
   block, readers for the three (visibility as narrow as the callers in
   topo allow — say which). A `compile_fail,E0599` doctest on
   `FittedLane::<Dual64>::certified()`.
2. **`topo::AtRestPolicy`** loses its `PcurveFittedLane` supertrait
   edge and gains `fn fitted_lane() -> Option<FittedLane<Self>>` (five
   arms: `f64`, `Probe`, `Interval`, `Sym<T>` → `Some(certified())`,
   `Dual` → `None`; each arm's reason moved from the deleted impls) and
   **a per-scalar name source**, e.g. `fn scalar_name() -> &'static
   str`, with today's exact strings (`"f64"`, `"telemetry probe"`,
   `"interval"`, `"symbolic"`, `"dual"`). Check whether a same-purpose
   name already exists on the policy before adding one; do NOT reuse
   `editor_core::lane::Lane::NAME` (different spellings) — file the two
   rosters on WIRE.
3. **The geom-brep doors.** `certify_fitted` and `certify_general` take
   a non-`Option` `FittedLane<T>` (they are certified-only doors) plus
   the name where a refusal needs it; `recertify` takes
   `Option<FittedLane<T>>` and the name, its `Fitted | General` arm
   refusing `FittedLaneUnsupported { scalar }` on `None`, every other
   arm unchanged; the `impl` block becomes `impl<T: Decide>`;
   `run_fitted_checks` takes the door. `mint_face` matches the `Option`
   before calling `certify_general`.
4. **`lane_name` → argument.** Its three readers (`pcurve_cache.rs`
   check 4, `pcurves.rs`'s `derive_general_image`, `transform.rs`'s
   `map_approx` — the offset-fit refusal LANE-0 routed through it) read
   the policy's name source or take it as an argument. Every name row
   (`lane0_r2_probes.rs`, `r1_lane0_e2e.rs`, `transform.rs`'s in-crate
   row, `m6_2`'s `"dual"`) stays green with the same text.
5. **The trait is deleted** with its five impls, its re-exports
   (`geom-brep/src/lib.rs`, `topo/src/lib.rs`) and the duplicated orphan
   doc line. The 25 items that already carry `AtRestPolicy` drop the
   term; the 38 that carry only the trait re-spell to `AtRestPolicy`.
   Narrow a site to `Decide` ONLY where the compiler shows it reaches no
   read site (`clear_face_caches`, `chart_edge`, `map_approx` are
   candidates by reading) — say which you narrowed and how you showed it.
6. **`certify_at_dual`** (`topo/tests/fixture/mod.rs`) and its consumer
   (`m6_2_fitted_at_rest.rs`) become: the `compile_fail` doctest (item
   1) and a runtime row reaching the absence at `Dual` (e.g.
   `recertify(.., None, name)` on the lifted image, or
   `mint_pcurves::<Dual64>` on a General-row fixture) asserting the same
   Display substrings (`"dual"`, `"may not certify"`, not `"no bracket"`).
   The four D9 rows (`geometric_cube.rs`, `review_m2_pr3.rs`,
   `extrude_acceptance.rs`, `m5_pr11_quad_props.rs`) and
   `r1_p2_probes.rs`'s `IsoUnsupported` row stay green UNEDITED.
7. **LANE-4P's shape.** `FittedLane` gets a wiring helper
   (`holds_the_certified_fitted_lane` or the file's naming) comparing all
   three fields by `fn_addr_eq`, one `#[test]` row per scalar it is
   formed at (`f64`, `Sym<f64>`, `Probe`, `Interval`), and a `ROSTERS`
   entry (certifying scalars) in
   `crates/topo/tests/certified_enclosure_impl_census.rs`. The census
   must be green because of your rows, not despite them: show it red
   first (door with no roster; roster with no helper; helper missing a
   field), then green.
8. **Prose.** `EvalScalar`'s doc (`editor-core/src/eval/mod.rs`) and
   `scripts/gates/evalscalar-allowlist.sh`'s header stop naming the
   supertrait; `AtRestPolicy`'s doc paragraph about the one lane trait
   riding along retires; DL3's list in `docs/DUAL-DESIGN.md` gains the
   fitted door (naming-only, as LANE-3 did — check `git log -S` for who
   wrote the list; a naming-only re-word lands with the change); `real.rs`
   and any gate header or README that names `PcurveFittedLane`.

**Bit identity.** No `decide(` moves, no refusal text moves, every
corpus green unchanged: the D9 rows, the shell/cup/vessel corpora,
`m6_2`, the sweep digests at the three ε rows. Show a base-vs-head run of
one fitted-pcurve corpus by bits.

## 2. Red-first

Record: the census rows red before your roster/helper and green after;
one forwarder mutant per `FittedLane` field reds its helper row; each of
the five `fitted_lane()` arms flipped (`Some` ↔ `None`) reds a named row;
the name source's `f64` arm mis-spelled reds the `"f64"` rows.

## 3. Not this unit

Unifying the two name rosters (file on WIRE); changing any refusal
variant or its text; the survey's option B (threading the hook as an
argument through ~93 signatures and ~1,500–2,200 call sites — declined,
as LANE-0 declined it).

## 4. Fence

By the survey's §8 owner map: PCERT (`pcurve_cache.rs`), REACH
(`boolean/ops.rs`, `splitting/mod.rs`), TANG/ZIP (`boolean/rest.rs`),
TOPO/ZIP (`merge_faces.rs`), OFFSET/SHELL (`transform.rs`,
`offset_axial.rs`, `offset_together.rs`), TQUERY (`split.rs`), TOPO
(`euler_ring.rs`), CHART (`chart_region.rs`), ATREST (`validate.rs`,
only if touched), BAND/CARVE (`blend/*`, `loft.rs`, `revolve/*`),
CARVE/STRUT (`sweep/src/test_support.rs`), WIRE (`eval/{wire,mod}.rs`,
`verbs/src/run.rs`), PROPS (`real.rs`, `DUAL-DESIGN.md`), GUARD
(`scripts/gates/*` headers and counts), TCOST/TINT (test files), and
the unowned `pcurves.rs`, `props.rs`, both `lib.rs`,
`test_support_fixtures.rs`, `offset_fit_lane.rs`/`fitted_lane.rs`,
`DESIGN.md`, `GENERICS-BUILD-COST.md`. Anything else: stop at the count
and file.

## 5. Collisions and verification

Merge `origin/main` before starting and before opening (RING-5, #3174,
may land first — it edits `topo/src/props.rs`; TOPO's #3160 restructures
`pcurves.rs`'s `walk_loop` — whichever lands second merges the other).
`cargo check --workspace --all-targets` default and `--all-features`;
`demos/tour`, `demos/wild`, `benches`, `sweep/examples` check (their
`Scalar` names `AtRestPolicy`); clippy `-D warnings` default and
`--all-features` on every touched crate; `cargo fmt --check`; every
`scripts/gates/*.sh` + selftests (`bounds-allowlist.sh` counts may move —
say which and why); `work.py lint`; nextest of geom-brep, topo, sweep,
verbs, editor-core, mesh. PR titled `LANE-4: …` (not `[ev]`: DL3's
addition is naming-only — if you find it is not, stop and say so). Watch
the hosted run to green. Review tier: DUAL.
