# SEAT — the verb-seat program (plan)

Executes `docs/VERB-SEAT-DESIGN.md` (ratified 2026-08-31, PR #1388,
Evan sign-off in-session): the kernel query seat (§1), one verb
vocabulary (§2), and the lowered parameter-identity channel (§3).
Branch prefix **`seat/`** (orchestrator branch `seat/orchestrator`).
Narrative record and live state: `work/seat/log.md`'s tail, never
this file. Ordinal band **1000–1099** (claimed in
`docs/MODEL-AB-LOG.md` at this program's opening).

**Sequencing (VERB-SEAT-DESIGN VS-Q6):** §1's units first — each
independent of §§2–3 and of each other in design, serialized in
dispatch for merge hygiene; then §2 with the blend pair; §3 rides
the first migrated verb whose consumer needs it.

## Wave 1 — the kernel query seat (design §1)

1. **SEAT-1 (S4, S)** — the blend and shell verbs derive their own
   band. Remove the `band: Band` parameter from
   `sweep::blend::build::{fillet_edges, chamfer_edges}` and
   `topo::{shell, shell_open}`; each derives `Band::linear(tol)` at
   operation entry exactly as `extrude`/`revolve`/`loft_body`/
   `tube_along_arc` already do (`extrude.rs:437`,
   `revolve/mod.rs:680`, `loft.rs:269`, `tube.rs:340`), absorbing
   the `BandError` residue into the op's typed error per that
   precedent. Every call site updates (workspace tests, examples,
   `step-import` tests, `demos/tour`); the demos' `band()` helpers
   and the spacer/diechamfer friction-(3)/(4) notes retire per the
   demo doctrine (workarounds deleted where re-authored, notes
   updated to the remaining findings). No numeric change is
   possible by construction — every existing caller passes exactly
   the linear derivation — so the acceptance is: suites green,
   no `k_stats` site added or renamed, and no remaining public
   kernel verb taking a `Band` beside a `Tol`.

2. **SEAT-2 (S1+S2, M)** — the `topo` query module and the
   `select_where` delegation. New module in `topo`:
   `all_edges`/`all_faces` materializers (deterministic arena
   order); EXACT predicates over `(&Body<T>, key)` — edge carrier
   kind, face surface kind, unordered adjacent-kind pair
   (`CurveKind` moves down beside `Curve3` per its own doc note;
   `SurfaceKind` reused from `geom-brep`, never re-minted); the
   DECIDED datum-distance atom in resolved form (a passed
   plane/axis/point value, `sel_*` funnel sites moved, not
   twinned — the site names are the same names). `editor-core`'s
   `candidate_matches` core (`names/geompred.rs:640`) re-homes onto
   the kernel predicates; `prepare`, the structural `Selector`, the
   GS-Q4 tie trilean and refusal payloads stay upstairs unchanged.
   Prelude re-exports the query doors (the `ContactClass`
   precedent). Kernel-seat demo scans re-author through the doors
   and their finding notes retire: `bodies.rs` spacer friction (1),
   `diechamfer.rs` findings 2–3 (`line_edges`, `all_edges`),
   `klein.rs` finding 8 (`corner_edges`), `bud::rims_between`,
   `teapot::rim_at`/`plane_chart_at`. Acceptance: one
   implementation of each predicate in the tree; `select_where`
   behavior pinned unchanged (same results, same refusals, same
   funnel site names in the census); the re-authored demos render
   byte-identical bodies.

3. **SEAT-3 (S3, M)** — the flush detector at the body seat. `topo`
   gains `find_flush_candidates(&Body<T>, &Body<T>, tol)` returning
   key-level findings, implemented as the C4 `Rest` verifier run in
   candidate-generation mode (`oriented_plane_eq`,
   `topo/src/boolean/plane_eq.rs` — the anti-twin rule holds by
   construction), plus `declare_all` sugar producing
   `BooleanDeclarations`; the no-fusion rule (SELECT-DESIGN GS-Q3)
   applies at this seat identically. The name-level
   `find_flush_candidates` (`editor-core/src/names/flush.rs:187`)
   becomes the derived wrapper (keys → names through the table),
   pinned behavior-identical. The two ~55-line hand declarers
   (`demos/tour/src/booleans.rs`, `topo/tests/common/mod.rs`)
   delete with their call sites moved to the library door;
   `twopeg`/`lily` re-author their declaration assembly through
   detector + declare and their finding notes retire.
   SELECT-DESIGN §3's finding-vocabulary sentence is amended in the
   same unit: names at the document door, keys at the body door,
   one verifier under both. Retires the producer gap of issue 757.
   Note the planar-only scope is inherited deliberately: the
   detector detects what the verifier verifies, and `twopeg`'s
   cylindrical contact re-authors only if the `Rest` verify ladder
   already covers it — the unit MEASURES that and reports rather
   than widening a verify table.

## Wave 2 — one verb vocabulary (design §2)

4. **SEAT-4 (V1–V4 substrate + the blend pair, L)** — the
   kernel-side `Verb<T>` declaration (home per VS-Q1: its own small
   crate above `sweep`/`topo` unless the build-cost survey says
   module-in-`sweep`), the owner-held stable-tag commitment matches
   (V2), the per-verb correspondence module shape in `editor-core`
   (V3), migrated for `Fillet`/`Chamfer` first (V4), with VS-Q5
   (the `RimSide`/`RimSupport` twin's disposition) decided in the
   unit spec. Spec cut at dispatch (`docs/SEAT-4-SPEC.md`); the
   cost of the addition is measured against the `Node::Chamfer`
   baseline and recorded (design §6).

5. **SEAT-5+ — per-verb migrations**, boolean first (it carries
   `BooleanDeclarations` and the §3 consumer), then
   extrude/revolve/split/shell as their own units, each costed.

## Wave 3 — lowered parameter identity (design §3)

6. **SEAT-6 (P1–P3, M/L)** — the opaque `ParamSource` token, the
   per-field side records, attach-at-mint driven by the migrated
   verbs' declared parameter→field flow, propagation by key
   identity, and the first consumer:
   `cylinder_cylinder_section`'s `RadiusEvidence` gains its
   production caller. Coordinated with the VERBS program (the
   cyl×cyl germ lane owns the geometry-side acceptance; the
   channel is this program's). Dispatch after SEAT-4 and after a
   handoff note on issue 1372.

## Side units (rulings and review products, outside the wave cut)

- **SEAT-DV (S)** — issue 1527's ruling (Evan, 2026-09-02:
  "probably a validating constructor, or making the unnormalized
  version unrepresentable"): `DatumValue`'s plane/axis normals
  become unit-by-construction — private fields, constructors that
  normalize or refuse typed; the SEAT-2 `debug_assert` tripwire
  retires with the class it guarded; `editor-core`'s construction
  sites move onto the constructors; the `Decide`-vs-`Real` bound
  question decided in the same pass. Dispatches after SEAT-4's
  implementation phase, on block SEAT-B2.

## Standing constraints

- **Protocol**: implementer dispatches ride the A/B ledger
  (`docs/MODEL-AB-LOG.md` on main at dispatch — block draws,
  banded ordinals 1000–1099, v6 duals, blinding). Briefs point at
  `docs/prompts/implementer-discipline.md` and reviews at
  `docs/prompts/reviewer-style-lane.md`, by path. State-sync rows
  ride the unit's own PR, last, after both reviews.
- **This program's orchestrator runs in a remote container
  session**: hosted CI is the gate and the only producer of
  committed measurements (`memories/local-battery-scope.md`); one
  implementer lane at a time (the build mutex serializes heavy
  cargo anyway, and reduced concurrency is the cheaper lever —
  MODEL-AB-LOG, method-relaxation item 4); the away-channel and
  monitor scripts of the persistent box are not armed here —
  Evan's channel is this session plus issues/PRs.
- **Cross-program courtesy**: SEAT-2/SEAT-3 touch demo scenes that
  VERBS and LIB cite by line number in their registers; the units
  update stale line references they knowingly move, and the log
  records any register row whose evidence line moved.
