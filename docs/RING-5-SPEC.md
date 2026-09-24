# RING-5 — the certification doors become an extension trait with a gate on its importers

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-24).** Binds
the implementer of unit RING-5; deleted at merge with a note under
`docs/doc-ledger/`. Read `docs/prompts/implementer-discipline.md` in full
first. The item is `work/scalar/ring-5-certification-doors-as-a-trait.md`.
The ruling is Ev's, in chat, 2026-09-24, choosing option 1 of two after
RING-3 (#3153) landed: *the certification doors move off `Interval`'s
inherent surface into an extension trait in `geom-core`, imported by
name, with a gate on the importers and one `hull` name per meaning* —
declined: a thin view type `Certified(Interval)`. The survey is
`/home/user/scalar-briefs/survey-ring5.md` (at `6d85ace893`); every
count in it is TEXTUAL (a compiler census was refused there) — re-derive
every count at your merge base, and take the compiler census in §1 first.

## 0. Why

RING-3 put the ring's refusal surface on `Interval` as inherent methods
(the second `impl Interval` block, "The certification doors"). One type
now carries two meanings: the backend's set semantics (IEEE 1788 hull,
intersection — right for evaluation, and what the oracle job checks)
and certification's refusal semantics. Nothing mechanical stops
certification code from calling `Real::is_poison` (NaI‖empty — a silent
pass on a `Trv` bracket) or a transcendental (which C9 forbids) wherever
`Real` is in scope. `work/scalar/certification-value-hygiene-has-no-gate.md`
records it with R1/R2's evidence (a compile probe showed the silent
pass). RING-5 makes the certification surface something a file opts into
by name, and gates what such a file may also have in scope.

## 1. First: the compiler census

Before editing anything, take the census the survey could not: mark each
door `#[deprecated]` (or remove it) on a scratch commit, run `cargo
check --workspace --all-targets` at default and `--all-features` and in
`demos/tour`, `demos/wild`, `benches`, and record every call site by
file (production / test) and whether `Real` is in scope there. Diff it
against the survey's §1 list; say what differs. Discard the scratch
commit. This is the unit's population.

## 2. What this unit delivers

1. **The trait.** A sealed trait (the `SpanLocate` precedent) in a child
   module `geom_core::interval::certification` (`crates/geom-core/src/interval/certification.rs`
   — NOT inside `interval.rs`, where ~30 test sites would go E0034), name
   `Certification` unless you find a better one; the name must not end in
   `Bounds` or `Enclosure` (DL4's suffix match would read it as a bracket
   door). Methods: `point`, `zero`, `one`, `hull`, `clamped_to`,
   `contains`, `width`, `mag`, `sqr`, `powi`, and the NaI constructor
   **renamed from `poison` to `refused`** (hygiene item 3: "poison" means
   three things). Implemented only for `Interval`, with RING-3's bodies
   **byte for byte**. Imported by path (`use geom_core::interval::Certification;`);
   **not** re-exported at `geom_core`'s root (a `use geom_core::*` must not
   pull it in). `certification_door_tests` moves with it.
2. **What stays inherent**: `from_bounds`, `repr_bits`, `is_certified`
   (the refusal predicate; evaluation reads it too), and
   **`from_certified`** — the crossing INTO certification arithmetic,
   called from files that legitimately have `Real` in scope
   (`geom/src/net.rs`). State this in the PR body as a decision for Ev.
3. **One `hull` name per meaning.** `hull` exists only on the trait;
   `SpanLocate::enclosure_hull` keeps its name (evaluation); the private
   second name `tangent_hull` is renamed to say it is the evaluation hull
   (e.g. `enclosure_hull_of`); the backend's `DInterval::hull` stays
   private to `interval.rs`.
4. **The files separate.** Every production file that calls a trait-only
   door imports the trait and has no `Real` in scope (production code
   only). The survey found two that mix:
   - `topo/src/props.rs`: `mod quad_lane` imports no `Real`; the file
     does. Move `quad_lane` to `topo/src/props/quad_lane.rs` as a PURE
     move (no body edits); split its `bounds-allowlist.sh` entry
     accordingly (the `props.rs` count after LANE-4P, #3165, lands).
   - `geom-brep/src/ssi/certify.rs`: `probe_tube_chart` mixes evaluation
     on `T` and certification arithmetic; move its certification tail and
     `zero_free_lower_bound` into `ssi/enclose.rs` (no `Real` there) as a
     PURE move — RING-3's refused-window early return (`certify.rs` ~677)
     and every row that pins it must survive unchanged.
   Any other mixed file the compiler census finds: separate it the same
   way, or stop at the count and say why it cannot be.
5. **The gate** `scripts/gates/certification-doors.sh`, in the house
   style (`interval-square-allowlist.sh`, `bounds-allowlist.sh`, `lib.sh`'s
   resolver for the production view — never a local brace counter), with
   `--selftest`:
   - an allowlist of the files that import the trait (production code);
   - reds on: an importer not on the list; a listed file that no longer
     imports it; in a listed file's production code, `Real` in any
     position (import, path, bound), a glob import, `super::*`,
     `enclosure_hull`/`SpanLocate`, `.is_poison(`;
   - one selftest plant per rule, each firing ALONE;
   - wired into CI and the gate roster exactly as the existing gates are
     (read how `bounds-allowlist.sh` is wired; `CI half parity + gate
     wiring` checks the roster);
   - its header states its blind spots: test code is exempt; holders that
     call no door (e.g. `geom/src/curves/nurbs.rs`'s `rational_span_bound`,
     which reads `is_certified` and `lo`/`hi`) are outside it.
6. **The census** (`crates/geom-core/tests/certified_endpoint_census.rs`):
   re-key its door list on the trait; add a row that the gate's allowlist
   and the census's importer set agree (fail loud on zero entries); widen
   its `gated_to!` to every crate it walks (hygiene item 6).
7. **Prose.** The three hand-written door lists in `interval.rs` and the
   census module doc point at the trait instead of restating it (hygiene
   item 7); `interval_backend_differential.rs`'s module doc says honestly
   what it exercises (the `sqr`/`powi` delegates) and what it cannot see
   (hull/clamped_to/contains/width/mag) (hygiene item 5, doc half).
   **C9** (`crates/geom-brep/README.md`) says the certification doors are
   the `Certification` trait's, imported by name, gated — Ev's ratified
   text: the PR is `[ev]`, with a before/after decision section and the
   `git log -S` writers (RING-3's re-word is `542f44da5d`'s tree).
8. **The hygiene row** closes with RING-5 for items 1, 2, 3, 6, 7 and the
   doc half of 5; items 4 (public "ring" names) and the rest of 5 (a
   door-level differential) are split into their own rows (or stay, with
   the row re-titled) — say which.

**Bit-preserving by construction**: every body moves unchanged and every
re-resolution (`zero`/`one`/`powi` in files without the trait now going
through `Real`) lands on a body that was already a delegate. Show it:
every corpus green unchanged, and the certification rows (the census,
`certified_door.rs`, `ring2_r2_probes`, the `bracket_seam_tests`, the
`ssi/certify.rs` rows) green unchanged.

## 3. Red-first

Record in the PR body: each gate rule's selftest plant firing alone; a
plant in real tree code for three rules (a `use geom_core::Real;` in an
importer, an `.is_poison()` in an importer, an importer off the list)
red, then restored; the census's cross-read row red on a planted
disagreement.

## 4. Fence

PROPS (`interval.rs` + the new child module, `real.rs` if a doc line
moves, `lib.rs` only if needed, `spline/*`, `geom/src/net.rs` import
only), SSI (`ssi/certify.rs`, `ssi/enclose.rs`, `ssi.rs` import only),
PROPS+QUAD (`props/quad.rs` import), ENCL/OFFSET/SHELL (`offset_fit.rs`,
`patch_bound.rs`, `offset_meters.rs` imports), CHORD+TESS
(`mesh/src/{chords,nurbs_cert}.rs` imports), TCOST/TINT (the census, test
files the compiler names), GUARD (the new gate, `bounds-allowlist.sh`'s
entry split, the gate roster/CI wiring), the unowned `topo/src/props.rs`
(the `quad_lane` move) and `crates/geom-brep/README.md` (C9). Anything
else: stop at the count and file.

## 5. Verification and PR

Merge `origin/main` before opening (LANE-4P, #3165, must have landed —
it edits `topo/src/props.rs` and `bounds-allowlist.sh`). `cargo check
--workspace --all-targets` default and `--all-features`; `demos/tour`,
`demos/wild`, `benches` check; clippy `-D warnings` default and
`--all-features` on every touched crate; `cargo fmt --check`; every
`scripts/gates/*.sh` + selftests; `python3 scripts/check-python-lint.py`
if Python is touched; `work.py lint`; nextest of geom-core, geom,
geom-brep, mesh, topo, sweep. PR titled `[ev] RING-5: …`, body per the
discipline with the decision section (C9 before/after; `from_certified`
inherent; `poison` → `refused`) first. Watch the hosted run to green.
Review tier: DUAL (orchestrator dispatches it).
