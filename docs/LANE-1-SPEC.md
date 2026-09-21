# LANE-1 — `PropsQuadLane` deleted: the quadrature door is a parameter, the certified name keeps its quadrature, a `_structural` twin carries the `None`

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit LANE-1; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/lane-1-props-quad-lane-deleted.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 3 (PR 2701), whose mechanism and
door-rename paragraph this unit applies to the first of the three
kernel lane traits. LANE-0 (PR 2981, merged) is the precedent for the
shape of a hook and for where a PER-SCALAR fact is read
(`topo::AtRestPolicy`); this unit's fact is not per-scalar — it is
"can this scalar certify", a bound — so the type system carries it.

## 0. The finding, and what the tree says

`PropsQuadLane` (`crates/topo/src/props.rs:1872-1913`; `: Decide +
geom_brep::PcurveFittedLane + crate::chart_region::ChartRegionLane`)
has two methods. `datum_lo` is `geom_core::Bounds::lo` in all five
impls (`:1917-2007`) and has one call site (`validate.rs:3714`).
`quad_cut_face` is `quad_lane::cut_face(..).map(Some)` in four impls
(`f64`, `Probe`, `Interval`, `Sym<T: CertifiedBounds>`) and `Ok(None)`
in the `Dual` arm (`:2009-2018`); the single site that calls it is
`reporting_hook<T: PropsQuadLane>` (`:300-310`), reached by
`mass_properties` → `mass_properties_with` (`:272-288`),
`classify_shells` → `classify_shells_of` (`:1683-1729`) and, through
`lane_certificate<T: PropsQuadLane>` (`validate.rs:3130-3136`), by the
lane-keeping tier-3′ family: `validate_pseudomanifold` (`:5259`),
`validate_pseudomanifold_certificate` (`:5301`), `contact_marks`
(`:3471`), `contact_marks_declared` (`:3488`), `tier3_local_checks`
(`:3138`). The certified function already has ruling 3's shape:
`quad_lane::cut_face<T: Decide + Bounds + CertifiedEnclosure>`
(`props.rs:2487`, private module `:2362`; its comment `:2367-2380` says
"the module stays uninstantiable at a dual with or without the
lane"). The certified side of the same pass exists beside it:
`sign_certified<T: Decide + CertifiedBounds>` / `certified_hook`
(`:365-382`) and the `_certified` twins `validate_geometric_certified`
(private, `validate.rs:2959`), `validate_pseudomanifold_certified`
(`:5355`), `validate_pseudomanifold_certificate_certified` (`:5333`),
`contact_marks_certified` (`:3505`), `contact_marks_declared_certified`
(`:3520`); the `_structural` twins `validate_geometric_structural`
(`:2885`) and `_structural_declared` (`:2900`) hand the check-7 hook
`&|_, _, _| None` (`:2940`). So the trait's whole content is a
runtime-shaped answer to a question the bound already asks, plus one
accessor for `Bounds::lo`. `AtRestPolicy: PropsQuadLane` (`:2067`) is
how `EvalScalar` and every `T: AtRestPolicy` site inherit
`PcurveFittedLane` and `ChartRegionLane` (`crates/editor-core/src/eval/mod.rs:2324-2340`;
`scripts/gates/evalscalar-allowlist.sh:12`). The survey with every
citation is `/home/user/scalar-briefs/survey-lane1.md` on the box.

## 1. What this unit delivers

**The trait goes.** `PropsQuadLane`, its five impls, `reporting_hook`,
`lane_certificate` and the `pub use` at `crates/topo/src/lib.rs:370`
are deleted; `crates/pncad/src/prelude.rs:538` stops re-exporting the
name; `crates/pncad-py/tests/test_binding_census.py`'s `"PropsQuadLane":
INTERIOR` entry (`:2587`, also `:1615`, `:2408`) goes with it.
`AtRestPolicy`'s supertrait list becomes `Decide + geom_core::Bounds +
geom_brep::PcurveFittedLane + crate::chart_region::ChartRegionLane`
(the two lane traits it inherited through the deleted one, until
LANE-2 and LANE-4 delete those), so nothing bounded `AtRestPolicy`
changes; `offset_fit_lane`, `gate_at_rest`, `gate_at_rest_declared`
stay as LANE-0 left them.

**The quadrature door is a parameter, in the hook shape LANE-0
landed.** A value type `topo::QuadLane<T>` (`Copy`; the one field is
the `fn` pointer to `quad_lane::cut_face`'s signature; one constructor
`QuadLane::certified()` in an `impl<T: Decide + CertifiedBounds>
QuadLane<T>` block — holding a value IS the statement that `T`
certifies; a `Dual` cannot construct one). `mass_properties_with`,
`classify_shells_of` and the tier-3′ family's private bodies take
`Option<QuadLane<T>>` where they took the trait; `None` takes today's
`Dual` path exactly (the same `VolumeUncomputable` / no-lane answers,
the same payloads and `Display`).

**Ruling 3's door rename, applied uniformly.** The certified name
stays on the `CertifiedBounds` door and keeps its quadrature for every
`f64` caller by name; a `_structural` twin carries the hook's `None`;
the existing `_certified` twins fold into the plain names:

| door today | after |
|---|---|
| `mass_properties<T: PropsQuadLane>` | `mass_properties<T: Decide + CertifiedBounds>` — `Some(QuadLane::certified())` by name; `mass_properties_structural<T: Decide + Bounds>` — `None` |
| `classify_shells<T: PropsQuadLane>` (+ `_of`) | `classify_shells<T: Decide + CertifiedBounds>`; `classify_shells_structural<T: Decide + Bounds>` |
| `validate_pseudomanifold<T: PropsQuadLane>` (lane-keeping, runs at `Dual`) and `validate_pseudomanifold_certified<T: Decide + CertifiedBounds>` | `validate_pseudomanifold<T: Decide + CertifiedBounds>` (the `_certified` body, under the plain name) and `validate_pseudomanifold_structural<T: Decide + Bounds>` (today's plain body with `None`) |
| `validate_pseudomanifold_certificate` / `_certificate_certified` | `validate_pseudomanifold_certificate` (certified) / `_certificate_structural` |
| `contact_marks` / `_certified`, `contact_marks_declared` / `_declared_certified` | `contact_marks` / `contact_marks_structural`, `contact_marks_declared` / `_declared_structural` |
| `tier3_local_checks<T: PropsQuadLane>` (`pub(crate)`) | takes `Option<QuadLane<T>>` beside its `Option<OffsetFitLane<T>>`; its callers pass what their door is |
| `validate_geometric*`, `validate_geometric_structural*` | already the shape; the private `validate_geometric_certified` folds into `validate_geometric`'s body if it is a duplicate, else stays private |
| `mass_properties_closed_form<T: Decide>` (`pub(crate)`, `:727`) | `pub` (the plan's word); `_of` stays `pub(crate)` unless a caller outside the crate needs it (say which) |

Every `Dual` caller in the tree moves to the `_structural` name by
name: `crates/sweep/tests/cert_m2r1_passes.rs:129-179` and
`crates/editor-core/tests/cert_m2r1_corpus.rs:13-66` (`dump::<Dual64>`
calling `mass_properties`, `validate_pseudomanifold`, `contact_marks`),
`crates/editor-core/src/checks.rs:1085-1106` (`connectedness` →
`classify_shells` under `run_checks<T: AtRestPolicy>`, which runs at
`Dual`), and whichever of `crates/verbs/src/run.rs:451`,
`crates/editor-core/src/verbs/shell.rs:226`, `topo/src/shell.rs:945,970`,
`replace_face.rs:1041,1073,1327`, `offset_axial.rs:376`,
`offset_together.rs:154` are instantiated at `Dual` (read each: a door
that is only ever reached at a certifying scalar takes the certified
name and the `Decide + CertifiedBounds` bound; a door on a `Dual` path
takes `_structural` or the `Option` parameter — list every one with its
disposition in the PR body). `AtRestPolicy for f64`'s gates already
call `validate_geometric` / `validate_pseudomanifold_certified`
(`props.rs:2128-2150`): they call the plain certified name after the
fold; the `Dual` arm's `NotRunAtThisScalar` is unchanged. Nothing
dispatches on the scalar at runtime; no blanket impl.

**`datum_lo` → `Bounds::lo`** at `validate.rs:3714` (the torus
tube-radius representability read). That makes a live `lo(` call in
`validate.rs`, which `scripts/gates/bounds-allowlist.sh` and the
allowlist prose in `crates/geom-core/src/real.rs:1229-1270` currently
say does not exist ("No `lo`/`hi` call appears in `validate.rs`",
`:1236`; the `datum_lo` instance named at `:1241-1242`): re-word the
allowlist entry to the read that now exists and why it is not a
laundering (it compares a datum's lower bound to zero for
representability, the value never crosses into a certificate) —
naming-only, DL4's gate line unchanged; the gate must pass with the
entry, not by exclusion.

**The identity test.** `crates/topo/tests/quad_lane_is_the_certified_lane.rs`
(the source-text pin that every `quad_cut_face` body is the certified
quadrature, `:73-127`) is deleted with the trait — its subject is gone.
What replaces the census it carried ("exactly the five impls"): a
`compile_fail` doctest that `mass_properties::<Dual64>` (and
`QuadLane::<Dual64>::certified()`) does not type-check, and a row that
`mass_properties_structural::<Dual64>` answers what today's `Dual` arm
answered, bit for bit on the corpus body those two `dump` helpers use.

**What must not change:** every certificate, verdict and refusal at
`f64`, `Probe`, `Interval` and `Sym` — the certified doors call the
same `quad_lane::cut_face` on the same inputs (D9: both
`span_bit_identity` suites, every render cell, `docs/tess-budget-data/`,
the K roster — no `decide(` moves, no predicate is added; if the
`_structural` twins' `None` path decides anything the trait's `Dual`
arm did not, say so with the k-lint before/after); every answer at
`Dual` (the `_structural` twins reproduce today's `Ok(None)` /
`NotRunAtThisScalar` / `VolumeUncomputable` outcomes and texts); the
public Python doors (`crates/pncad-py/src/py/value.rs:324-326,365-489`
call the `topo::` doors monomorphically at `f64` — the certified names,
unchanged in behaviour); `demos/tour/src/scalar.rs:113-127`'s `Scalar`
trait (bounded `AtRestPolicy`, impls `f64` and `Probe`) and its ~60
`mass_properties` / 6 `classify_shells` sites, which stay type-correct
under the certified bound.

## 2. Docs

`docs/DUAL-DESIGN.md:86` (DL3's "+V through `PropsQuadLane`, whose
`Dual` arm refuses") is re-worded to the door that now refuses at
`Dual` (the `_structural` twin's `None`) — naming-only, CLAUDE.md's
carve-out; cite the sentence and `git log -S` its commit in the PR
body. `validate.rs:5250-5258` (H-R3: keeping the tier-3′ door callable
at a `Dual` "is the capability H-R3 protects") is re-worded to name
`validate_pseudomanifold_structural` as the door that carries that
capability — the capability is unchanged, the name moved; if H-R3
lives in a ratified page (`crates/topo/README.md` or ATREST's docs —
find it), the same carve-out applies and the PR body says where it
looked. `props.rs:1822-1866`'s lane-split docs go with the trait; what
survives of them (the certified/structural split, "no `Dual` in the
quadrature") moves onto `QuadLane`'s doc and `quad_lane`'s module
comment, present tense. `docs/GENERICS-BUILD-COST.md:387` lists the
trait among the lane traits: drop it from the list with a one-clause
note. `crates/verbs/README.md:210`, `crates/verbs/src/verb.rs:212`,
`crates/editor-core/src/eval/mod.rs:2317`, `crates/topo/src/chart_region.rs:443`,
`crates/geom-brep/src/pcurve_cache.rs:1119,1149`: prose that names the
trait as the present, one clause each. `docs/MODEL-AB-LOG.md:2926` and
`docs/DOC-LEDGER.md:4398` are dated records: untouched.

## 3. The pin

- The `compile_fail` doctest and the `_structural`-at-`Dual` row above.
- Red-first: with `mass_properties`'s `Some(QuadLane::certified())`
  replaced by `None` and nothing else changed, the `f64` props rows
  that assert quadrature (name them) go red; restored, green. The same
  for `validate_pseudomanifold`'s check 7.
- The `QuadLane` wiring pinned by pointer identity
  (`std::ptr::fn_addr_eq` against `quad_lane::cut_face`), the shape
  LANE-0's `wiring_rows` landed — in `props.rs`'s own test module,
  since the field is private.
- D9 as in §1; the two `dump` helpers' corpora at `Dual64` byte for
  byte against the merge base (the `_structural` twins).

## 4. Sweep

The class: every site that names `PropsQuadLane` (the survey counts
32 in `validate.rs`, 3 `shell.rs`, 4 `replace_face.rs`, 2 each in
`offset_axial.rs`/`offset_together.rs`, 3 `verbs/run.rs`, 2 each in
`editor-core/src/checks.rs` and `verbs/shell.rs`, the prelude, the
binding census, 8 prose in `geom-core/src/real.rs`, 2 in
`pcurve_cache.rs`, and 11 test files / 42 rows) — dispositioned one by
one in the PR body: certified name / `_structural` / `Option`
parameter / prose. The sites relying on `T: AtRestPolicy ⟹ T:
PropsQuadLane` (`checks.rs:996,1042`, `eval/mod.rs:2324-2355`,
`boolean/ops.rs` ×9, `transform.rs` ×3, `verbs/run.rs:237`,
`sweep/src/test_support.rs:503`, `eval/wire.rs` ×13,
`demos/tour/src/scalar.rs:113`) keep compiling by construction (the
supertraits move up); confirm with `cargo check` on the workspace AND
on `demos/tour` (its own cargo root). Re-derive the counts at your
merge base.

## 5. Fence

This program claims no paths. This unit reaches ATREST's
`crates/topo/src/validate.rs`; SHELL's `shell.rs`, `replace_face.rs`,
`offset_together.rs` and (with CURVED/OFFSET) `offset_axial.rs`; REACH's
`boolean/ops.rs` (the `closed_form` callers, if the visibility change
touches them); CHART's `chart_region.rs` (prose); the unowned
`topo/src/props.rs` and `topo/src/lib.rs`; LIB's `crates/pncad/src/prelude.rs`
and LIB/BIND's `crates/pncad-py/tests/test_binding_census.py`; WIRE's
`crates/verbs/*` and `editor-core/src/{checks.rs,eval/mod.rs,verbs/shell.rs}`;
PROPS' `crates/geom-core/src/real.rs` (allowlist prose) and
`docs/DUAL-DESIGN.md` (DL3's sentence); GUARD's
`scripts/gates/bounds-allowlist.sh` and `evalscalar-allowlist.sh` (one
entry each, naming-only); PCERT's `pcurve_cache.rs` (prose);
TCOST/TINT's eleven test files; `docs/GENERICS-BUILD-COST.md`. Not
`demos/` (zero occurrences; `cargo check` only). Announced by the
orchestrator in `work/scalar/log.md` and on the PR. Merge `origin/main`
immediately before opening the PR; `python3 scripts/work.py territory
--base origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p topo -p sweep -p editor-core -p verbs -p
pncad` at default, `-p topo -p sweep` under `--features probe` and
`--features interval`; the pncad-py suite (`crates/pncad-py`, the wheel
and `test_binding_census.py`); `cargo check --workspace --all-targets`
at default and `--all-features`, and `cargo check` in `demos/tour`;
`cargo clippy` on the touched crates `--all-targets -- -D warnings` at
default and `--all-features` (narrowed: disk is shared — say so;
hosted `clippy` rows are the record); `cargo fmt --all --check` plus
`--check` in `demos/tour`, `demos/wild`, `benches`;
`scripts/gates/bounds-allowlist.sh`, `evalscalar-allowlist.sh`,
`probe-suite-census.sh`, `check-interval-cfg-additive.py`,
`scripts/doc-gate.sh`; `python3 scripts/work.py lint`. Private
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; delete the target before
reporting; scratch under `/home/user/scalar-lane1-scratch/`. Hosted CI
is the verification of record; poll to conclusion in the foreground
and report the run id. Report ≤120 lines: the door table as landed,
the site-by-site disposition of §4, the `Dual` callers moved, the
red-first evidence, the wiring pin, the D9 statement with the k-lint
counts, the docs re-worded with their `git log -S` commits, deviations,
rows filed. Commits are plain one-line messages: no trailers, no model
or vendor names anywhere in commits, files or the PR body (strip any
auto-appended footer through the MCP write path and verify the live
body).
