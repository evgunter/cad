# LIB-RETTAIL spec — the retirement's tail: ProfileLoop demotion, bowtie re-home, shim deletion (binding)

Mandate: execute the two rulings Evan issued on #413 (2026-08-12,
recorded in docs/LIB-LOG.md): (1) raw ProfileLoop construction
DEMOTES from the presented surface ("kernel vocabulary should be
private"; the broken-on-purpose bowtie cannot justify a public
authoring tier); (2) the LoopBuilder test-support shim carries a
deletion horizon — delete it now.

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time via local-scripts/with-build-slot.sh, no parking +
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge on movement,
checks STARTED, cold clippy CI scope both lanes + `-p pncad-py
--features python`, k-lint discipline, comments state the
INVARIANT).

## 1. Deliverables

1. **The demotion**: raw `ProfileLoop`/`ProfileVertex`
   CONSTRUCTION (`new`, `polygon`, `builder`, any other minting
   door) leaves the pncad prelude and curated surface. The TYPES
   stay nameable (read-back, error payloads, validation results
   consume them); construction becomes kernel-internal. Census
   first: grep every construction site outside the kernel crates
   (demos, corpus, fixtures, guide) and migrate each to the
   lattice or REPORT why it cannot move. The pncad::profile
   module re-export survives only if construction doors can be
   excluded from it honestly (measure; if module re-export means
   construction leaks, the module re-export narrows to a curated
   subset — the LB13 precedent).
2. **Bowtie re-home**: the deliberately-invalid bowtie leaves
   the tour (a broken-on-purpose scene is not a use case) and
   becomes a validation-suite fixture preserving its exact
   fail-loud contract (authors cleanly at the kernel layer,
   refuses at validation — the ladder rows keep their oracle).
   Tour count and any tour-enumerating tests/docs update
   honestly; the demo-purpose crate-doc block updates if it
   names the bowtie.
3. **Shim deletion**: the ~15 legacy test callers of
   `ProfileLoop::builder`/LoopBuilder migrate to lattice or
   recorded-fixture spellings; `profile/src/test_support.rs`
   DELETES entirely; the differential twins' verification
   target becomes recorded fixtures (bless them in this unit —
   the twins must still fail on a lowering mutation: prove it
   with one deliberate mutation before finalizing).
4. **SWITCH-fence + §V6 closure notes**: the "kernel tour
   scenes stay as-authored" sentence and the bowtie's
   permanent-rawness clause amend to record this unit's ruling
   (cite Evan/#413); ratified-text edits ride this PR with the
   ruling cited — no separate conversation needed (the ruling
   is explicit).
5. **Audit/guide sweep**: any G-row, guide line, or absence
   test that names raw construction or the bowtie updates;
   counts script-re-derived if any row moves.

## 2. Fence

OUT: the §2c fillet-family re-spell (its own unit, gated on
#419's ratification), any lattice vocabulary change, kernel
geometry, schema, CI structure. Anything missing: REPORT.

## 3. Acceptance

- `grep -rn "ProfileLoop::new\|ProfileLoop::polygon\|LoopBuilder"`
  over demos/, docs/, crates/pncad*/ returns ZERO construction
  hits (kernel-internal + validation-fixture uses excepted and
  enumerated in the report).
- test_support.rs no longer exists; profile has no test-support
  feature; twins green against blessed fixtures AND red under a
  deliberate lowering mutation (shown once, reverted).
- Python suite green (state delta); cargo test -p profile -p
  pncad -p pncad-py; cold clippy CI scope all three lanes;
  hosted CI green; zero new [[test]] binaries.

## 4. PR discipline

One PR. Report ≤150 lines to
~/.local/share/cad-work/lib-rettail-report.md with per-phase
figures. Open, do NOT merge. Final message: PR number + report
path + ≤10-line summary. Forks: report, smallest faithful
reading, flag.
