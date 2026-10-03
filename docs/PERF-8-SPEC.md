# PERF-8 — per-face mass-property fluxes in parallel, with the K-funnel composing

**Status: ratified at dispatch (PERF orchestrator, 2026-09-12).** Binds
the implementer of unit `PERF-8`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is `work/perf/mass-properties-are-serial-per-face.md`.

## 0. The finding this executes

`topo::props` walks faces serially: `mass_properties_impl` calls
`face_flux` per face in arena order and `fold_runs` sums in arena
order (`crates/topo/src/props.rs`); since PERF-6 `sign_certified` does
the same per round over the open faces. The per-face lane is the
whole cost on curved bodies (`loft_prism` 157 ms; the round spout
11–18 s per door; the tour's 78 gated bodies 3.1 s after PR 2440).
D9's addendum names this "the canonical idiom-2 example": idiom 1 per
face, then the arena-order fold that already exists.

**The constraint that makes this a unit and not a loop edit.** The
lanes decide through named predicates (`props_quad_*`,
`positive_volume*`) in `geom_core::k_stats`, whose recording is
THREAD-LOCAL: `FRAMES` (the open `Bracket`'s verdict/escalation
vector, "in decision order" — NAMING-DESIGN N5, read by the verdict-
diff engine and the subdivision driver) and, under the `probe`
feature, `SINK` (the samples `tools/k-lint` counts). A face decided
on a rayon worker records into that worker's frame stack and sink,
not the caller's: verdict logs would silently lose the props
predicates and k-lint's population would shrink with the thread
count. `editor-core`'s existing parallel maps (`mc.rs:473`,
`eval/mod.rs` behind `EvalOptions::parallel`) do not answer this —
say in the report whether they lose recordings today (a finding to
file, not to fix here).

## 1. What this unit delivers

1. **A composing door on the funnel** (`geom_core::k_stats`, PROPS/
   TOPO announce in `work/perf/log.md`; geom-core is shared ground —
   keep the edit to the funnel and name it in the PR): run a closure
   on the current thread under a detached frame and hand back what
   it recorded (`Recorded` plus, under `probe`, its samples), and
   append a `Recorded` (and samples) to the current thread's open
   frame and sink in the order given. The names are yours; the
   contract is that a walk which runs faces on workers and appends
   their recordings in arena order produces a verdict log, an
   escalation log and a sample population **byte-identical** to the
   serial walk's at any thread count. `Bracket` semantics for nested
   brackets are unchanged.
2. **The walks as idiom 1 + idiom 2**: `mass_properties_impl`'s face
   loop and `sign_certified`'s per-round loop over open faces run as
   indexed parallel maps (one slot per face in arena order), each
   face under its own detached frame; `fold_runs` stays the arena-
   order sequential fold, and the recordings are appended in the same
   pass, in arena order. `T: Decide` must be `Send + Sync` where the
   map needs it — state what that requires of the interval scalar.
3. **The escalation path is identical.** Today a face's indeterminate
   or refusal stops the walk at that face in arena order and later
   faces are never decided. The map decides every face; the fold
   appends recordings only for faces BEFORE the first refusing face
   in arena order and drops the rest, reports that face, and the
   logs match the serial walk's exactly. Say at the site what extra
   work the failure path does.
4. `topo` gains `rayon` (workspace dependency). One spelling of each
   loop: a one-thread pool is the serial path; no serial twin.

## 2. The pin

- **Verdict-log identity at any thread count**: a row that runs tier
  3 and `mass_properties` on the corpus bodies with curved walls
  (`loft_prism`, the arc lofts, the digest roster of PERF-6) under an
  explicit 1-thread and a 4-thread pool inside a `Bracket`, and
  asserts the `Recorded` vectors equal, element for element, and the
  results' bits equal; the same row on a body that escalates or
  refuses (the thin strip of PERF-6, a poisoned face), asserting the
  log equals the serial one and the same face is named.
- **k-lint's population unchanged**: `docs/k-report-data/`'s
  reporting-path predicate counts and the k-lint gate rows on the PR;
  under the `probe` feature, a row asserting the sample count on a
  roster body is the same at 1 and 4 threads.
- **PERF-6's pins unchanged**: the reporting-door digest, `sign_
  certified_plus_v`, `tcost_k3_certificate` — bit for bit.
- `benches/` mass-property rows at 1 and 4 threads.

## 3. Measurement to report

Release, 4 vCPU under the slot, medians of 3 with spread, at
`RAYON_NUM_THREADS=1` and `=4`: `mass_properties` and `validate_
geometric` on `loft_prism`, the arc lofts at 1e7/1e9/1e11·ε, and the
round spout (build `demos/tour` and time its two doors on the teapot
scene, the fix pass of PR 2440 gives the harness); the tour's 78-body
gate+measure aggregate. The 1-thread cost must be within noise of
today (the detached frame's cost is the thing to watch).

## 4. Out of fence

The lanes' quadrature (`quad.rs`); the schedule and target; the fold's
arithmetic; the closed-form lanes; tier 3′'s aggregate gate;
`editor-core`'s parallel maps (report, do not fix). PROPS/TOPO
territory plus the funnel door in `geom-core`, announced in
`work/perf/log.md`.

## 5. Report

≤120 lines: the funnel door's names and contract, the two maps'
shapes, the escalation-path argument, what `Send + Sync` cost, the
pins, the measurements of §3, deviations, findings outside the fence.
