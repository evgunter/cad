# LIB-LBRET spec — LoopBuilder retirement (#377, ratified #386): the §2b route-3 door + rocker migration (binding)

Mandate: execute the ratified retirement package (#386, Evan 👍
2026-08-11): (1) the §2b route-3 straight-arrival binder; (2)
rocker's outline migrates to the lattice under the LB4/LB5
dispositions; (3) LoopBuilder leaves the profile crate's public
surface entirely — test-support only; (4) audit G12 flips. Read
the ratified text FIRST: docs/PATHS-DESIGN.md §2b tail (the LB10
revisit, route 3), docs/PROFILES-V2-DESIGN.md §V6 (the amendment
— full banishment, V4(c) struck), docs/LIB-LOG.md's #377 entry
(LB4/LB5 dispositions).

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time, build slots, no parking + kill-your-own-waiter,
commit+push per chunk, NO Co-Authored-By, no model names,
merge-main-before-open + re-merge on movement, checks STARTED,
cold clippy CI scope both lanes, k-lint discipline, comments
state the INVARIANT).

## 1. Deliverables, in dependency order

1. **The route-3 door**: a distinctly-named straight-arrival
   binder living IN `crates/profile/src/path/arc_fillet.rs`,
   sibling to `at_on`/`to_on` and mirroring their shape (the
   ratified sizing: ~60–100 lines), carrying the compound
   `ArcCarrierScalar` bound exactly where the §2b register
   confines it — the generic doors gain NOTHING. The geometry
   already exists (`arc_fillet::resolve` handles a
   `SideCarrier::Ray` arrival); you are adding the typestate
   door only. Name it per the §2b register's conventions (state
   your choice + reasoning in the report; it joins the §3
   surface table and the program Step vocabulary — record-as-
   you-lower + replay arm + tags, the at_on/to_on precedent
   exactly). Junction/k_stats classification per the ratified
   funnel discipline; refusal rows for the still-illegal shapes.
2. **Rocker migration** (`demos/tour/src/rocker.rs` outline →
   the lattice): under the ratified dispositions — LB4: the
   #289 oracle-equality contract, NOT byte-identity (derived
   corners 0–4 ulps off authored anchors are the natural
   outcome; nothing fits anchors — the demo stops transcribing
   them); LB5: the mid-arc seam RE-ANCHORS (state the topology
   change in a comment at the site — the invariant, not the
   history). The scene's oracle/validation ladder must hold at
   its existing tolerances; if any tolerance must move, STOP and
   report (that is evidence, not an adjustment).
3. **The banishment**: LoopBuilder (and its close_* family)
   leaves the profile crate's public API — relocate to
   test-support beside the differential-twin suites
   (`crates/profile/tests/` — choose the smallest honest
   mechanism: a tests/common module the twin files include, or
   #[cfg(test)]-gated in-crate if the twins need in-crate
   access; NOT a new published crate). The twins keep verifying
   the lattice against it unchanged — their value is the
   independent second implementation. Migrate the cross-crate
   TEST consumers (step-export tests/common, mesh, k-lint
   litmus — census in the #377 investigation) to lattice or
   raw-ProfileLoop spellings. `bulge_from_via`/
   `bulge_from_center` and raw `ProfileLoop` data stay public
   (kernel vocabulary — the bowtie is untouched). pncad prelude
   + authoring docs + guide drop LoopBuilder; validate.rs error
   text that names it re-words to the surviving vocabulary.
4. **Audit G12 flips**: rocker becomes an executed YES row in
   test_north_star.py IF its profile now crosses the Python
   lattice surface (the route-3 verb needs a Python binding +
   stub + ty rows — same-unit, the PYG1 house pattern; it is
   one verb). Counts script-re-derived; G12 → Closed gaps
   (#377 pointer); absence rows flip honestly. If a residue
   blocks the Python row (e.g. multi-loop interplay), the
   honest partition re-states it — do not force.
5. **Guide**: the §2b section's Rust block gains the route-3
   verb where the wall was documented; Python mirror per 4.

## 2. Fence

OUT: any other §2b route, any generic-door bound widening (the
LB3 confinement is the point), NURBS-leg anything, G5/G14,
kernel changes beyond the one door, CI structure, schema.
Anything missing: REPORT, never build.

## 3. Acceptance

- The route-3 door: differential + property rows extended (the
  recorded program replays bit-identically; the twin suite
  covers the new verb); refusals typed.
- Rocker: scene oracle + ladder green at existing tolerances;
  the migration diff reviewed as demo-improvement class.
- Banishment: `grep -r LoopBuilder crates/*/src` returns ZERO
  public-surface hits (doc comments re-worded); every test
  consumer green in its migrated spelling.
- Python suite green (state delta); cargo test -p profile -p
  pncad -p pncad-py; cold clippy CI scope both lanes; hosted CI
  green; zero new [[test]] binaries; stub + ty green.

## 4. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-lbret-report.md with per-phase
figures. Open, do NOT merge. Final message: PR number + report
path + ≤10-line summary. Forks: report, smallest faithful
reading, flag.
