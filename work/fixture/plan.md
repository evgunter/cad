# FIXTURE — the plan

one body, fixture or helper spelled n times across the suites

Opened 2026-09-20 by TINT's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

## The slate

**29.5 budget points** of dispatchable work against a ceiling of 30
(re-counted 2026-10-26 in the tracker sweep). Live rows only.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P4 | `band-helper-duplicated-across-suites` | E | open | test suites across six crates carry a byte-identical fn band() wrapper; sweep is collapsed, the rest needs a shared home |
| P4 | `body-hash-census-misses-rename-only-duplicates` | E | open | the body-hash duplicate census misses rename-only twins |
| P4 | `cavity-module-doc-restates-a-census-that-is-now-wrong` | E | open | sweep/tests/common/cavity.rs restates the brick/prism census in its own module doc, and both equalities it asserts are false today |
| P4 | `census-answers-no-field-read-for-a-walk-that-reads-a-field` | E | open | the hand-written-impl census answers NoFieldRead for two shapes that do read a field, and its own doc says so |
| P4 | `editor-core-raw-twin-planes-unreconciled` | E | open | seat6/seat7's kernel-direct twins assert an equality nothing enforces: two plane spellings, hand-held |
| P4 | `geom-brep-inline-canonical-frame-surfaces` | E | open | geom-brep tests spell the canonical-frame elementary surfaces inline at 34 use sites |
| P4 | `inline-name-table-reads-bypass-the-fixture-door` | E | open | 127 inline name-table reads in editor-core's suites bypass the fixture's table door |
| P4 | `landing-gathers-source-text-row-has-a-gate-now` | E | open | landing_gathers.rs's source-text row over product.rs's gather counter now duplicates a CI gate on the gatedness axis |
| P4 | `named-copies-of-fixture-doors-under-other-names` | E | open | 121 StableName literals in editor-core's suites spell a fixture authoring door by hand |
| P4 | `orient-module-prose-accumulation` | E | open | `sweep`'s orient.rs has no governor on prose accumulation |
| P4 | `prune-report-rows-are-nine-copies-of-one-assertion` | E | open | Nine near-verbatim assertion blocks across seven test files, because the shared harness is other programs' ground |
| P4 | `reader-census-shared-disposition-survives-partial-reversion` | E | open | A Shared ledger row is checked by one substring, so a site that keeps the import and re-adds hand-rolled helpers stays green |
| P4 | `sweep-boolean-suite-brick-and-prism-copies` | E | open | sweep tests: the prism(pts, h) copy class outside the blend family (the brick half closed by S52) |
| P4 | `sweep-revolve-about-y-helper-and-its-fixtures-spelled-per-suite` | M | open | sweep tests: the revolve-about-y helper and the rectangle meridian are spelled per suite beside test_support::revolved_about_y |
| P4 | `sweep-test-suites-cite-eight-deleted-suite-files` | E | open | sweep tests: sixteen tests→tests citations across thirteen suites, eight naming suite files that no longer exist |
| P4 | `the-near-tangent-pose-is-spelled-four-times` | E | open | The near-tangent prism-corner pose is built four times in tests and the probe, and the copies drift |
| P4 | `the-per-impl-sight-anchor-is-a-suppression-list-that-shrinks` | E | open | the hand-written-impl census's only per-impl sight anchor is its suppression list, so it goes blind as the rows it names are repaired |
| P4 | `the-three-station-arc-loft-fixture-has-eight-spellings` | E | open | The three-station arc loft is spelled eight times across crates/sweep/tests; three were byte-identical |
| P4 | `the-two-vertex-bulge-one-circle-fixture-has-eight-copies` | E | open | The two-vertex bulge-1 circle section is hand-written in eight sweep/profile test files, the newest added by a PR whose subject was retiring hand-spelling |
| P4 | `the-u-cutter-tie-fixture-is-hand-copied-across-the-tree` | E | open | the symmetric U-cutter tie document is hand-built in thirteen test files with no shared builder |
| P4 | `threaded-seam-wait-loops-are-hand-copied-across-four-test-files` | E | open | four viewer test files each spin on a seam with their own copy of the same bounded nap loop |
| P4 | `topo-tests-cube-door-roster-is-a-hand-written-partition-claim` | E | open | cube_doors_agree.rs's door roster is hand-written; a new builder in common joins it silently as nothing |
| P4 | `topo-tests-mapped-cube-call-sites-that-are-brick-with-ceremony` | E | open | Nine mapped_cube call sites pass a diagonal affine map, which is brick spelled the long way |
| P4 | `topo-tests-notched-beams-fixture-is-verbatim-in-two-files` | E | open | The four-box notched_beams fixture is verbatim in crosslap_rest and m5_s1_rest_zip |
| P4 | `topo-tests-self-declared-fixture-copies-census` | E | open | A prose census of self-declared fixture copies in crates/topo/tests |
| P4 | `topo-tests-straddle-seat-hand-copies` | E | open | common::straddle_seat claims one shared builder and at least two suites hand-copy it, plus a five-way crossing-post profile |
| P4 | `topo-tests-unit-cube-has-fifty-spellings` | E | open | The unit cube is spelled 49 times across 19 suites, and three conventions for naming a nullary brick wrapper now coexist |
| P4 | `viewer-error-display-prose-bans-one-identifier-per-arm` | E | open | viewer's error_display prose helper bans one identifier per arm, so a sibling leak passes at 29 call sites |

## Order

**By file, not by row.** Every unit here is "give this shape one home
and convert its call sites", so the cheapest order is whichever suite
you are already editing. `topo-tests-unit-cube-has-fifty-spellings` is
the largest single population (49 spellings, 19 suites, three
conventions) and should be the one that DECIDES the convention; the
others follow it rather than each inventing one.

Take these as drive-bys wherever possible (`work/README.md`, "The
tracker is not comprehensive"). Dispatching twenty PRs for twenty
fixture moves would cost more than the duplication does.

## What SUITE's plan said that still binds

SUITE folded into this program on 2026-10-26 with three live rows
(`band-helper-duplicated-across-suites`,
`prune-report-rows-are-nine-copies-of-one-assertion`,
`threaded-seam-wait-loops-are-hand-copied-across-four-test-files`).
Two of its exit criteria are the right standing questions for every unit
here, whatever this program's own exit criteria turn out to be:

- **A duplication row closes by a shared home, not by a deletion**, and
  at every call site the unit touched the reviewer asks whether the
  shared helper still makes that suite's intent readable, or whether the
  suite has become a call into a fixture nobody reads.
- **Each unit states what its sweep pattern could not match.** A sweep
  whose blind spot is unstated is an unverified claim, not a negative
  result.

SUITE's units minted, or nearly minted, a fresh copy of the defect they
were closing every time, and each time a reader who had not written the
fix caught it. A unit here gets a reader who did not write the fix.

## Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.

## From HELPER (merged 2026-11-20)

one test helper, oracle or walk with several private homes

Opened 2026-09-20 by TINT's priority-seam cut
(`work/README.md`, Track size). Nothing dispatched.

### The slate

**21.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P1 | `mass-properties-bit-comparison-has-thirty-spellings` | D | compare a MassProperties by bits is written ~31 ways and they disagree about the pads |
| P3 | `D287` | E | Convert four topo/src source-text readers onto test_utils::source and lower the reader census ceiling |
| P3 | `D383` | E | Close the bound-domination class onto test_utils::tightness: unswept members, discarded ratios, soundness slack |
| P3 | `D386` | E | The locate-and-read-a-literal composite is implemented twice and has already drifted; test_utils::source has no home for it |
| P3 | `chamfered-cube-and-steiner-oracles-outside-sweep` | E | The chamfered-cube and Steiner closed forms are restated outside crates/sweep too |
| P3 | `m10-p-lift-interval-plane-is-sketch-plane-map-by-hand` | E | editor-core/tests/m10_p_lift.rs interval_plane rebuilds a SketchPlane from twelve from_f64 calls, which is plane.map(Interval::from_f64) |
| P3 | `pointwise-net-rebuilds-outside-d385s-lifts` | E | 44 test sites rebuild a validated NURBS net through new(..).unwrap() where map_points is the door |
| P3 | `three-part-resolver-stub-residues-resist-the-shared-fixture` | E | asm2a_instantiate's seam-refusing StubStore is the one part-resolver stub left beside the shared fixture |
| P4 | `D384` | E | Give the trv()/healthy() finite-bracket fixture one home for its three verbatim copies |
| P4 | `D385` | E | Replace hand-written NURBS and profile-vertex payload lifts in test crates with map_scalar, deciding per file |
| P4 | `cross-crate-test-helper-copies-outside-the-geometry-fixtures` | E | cross-crate duplicated test helpers outside the geometry-fixture rows: validated/vp at 10 byte-identical copies, and twelve more classes no row names |
| P4 | `geom-curve-test-frames-hand-roll-the-helper-axis-cone` | E | test-side frame helpers hand-roll a world-axis cone instead of calling the kernel's basis door |
| P4 | `origin-anchored-fan-oracle-copies` | E | Origin-anchored fan/volume oracle copies remain across test and doc templates - post-issue-1362 enumeration |
| P4 | `re-exec-child-harness-is-copied-per-suite-and-greens-when-it-does-not-run` | E | The current_exe re-exec harness is copied per suite, and every copy greens when the child runs no tests |
| P4 | `run-with-a-prior-evaluation-has-seven-private-copies` | E | run(doc, prior, ...) has seven private copies across editor-core's suites and no shared home |
| P4 | `source-lacks-an-item-body-carve-and-shared-means-any-mention` | E | test_utils::source has no item-body carve (five hand-rolled copies) and the census's Shared check accepts any mention of the crate |
| P4 | `sweep-per-step-differencing-helper-unhomed` | E | turning_orientation's spine_chords is private to one suite while orient.rs routes stack readers to the end-to-end spelling |
| P4 | `sweep-test-support-two-wrapper-conventions` | E | sweep::test_support carries two conventions for the scalar and its stated reason is false at three of five sites |
| P4 | `the-face-to-solid-walk-is-spelled-per-test-file` | E | The face to shell to solid walk is spelled once per test file in topo/tests and sweep/tests (tint and tcost ground) |
| P4 | `the-half-edge-to-face-walk-is-spelled-per-test-file` | E | The half-edge to face walk is spelled once per test file across 91 sites in five crates' suites |

### Order

`mass-properties-bit-comparison-has-thirty-spellings` first: ~31 ways
to compare a `MassProperties` by bits **that disagree with each other**
is a correctness question wearing a duplication row's clothes, and it
is P1 for that reason.

Then `re-exec-child-harness-is-copied-per-suite-and-greens-when-it-does-not-run`
— a harness that greens when it does not run is a guard defect, and
every copy has it. The two walk rows (91 sites and its sibling) are the
largest populations and `topo` already exports the door, so they are
conversions rather than designs.

### Review posture

OPEN, for this program's first dispatch. TINT inherits protocol v7
(`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual on triaged-in units
only, opus/opus outside it. Nobody has re-asked the triage question for
this slate, so the first orchestrator answers it here rather than
inheriting an answer.
