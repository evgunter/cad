# R2 probe lane (BOOL-11, PR #1520, frozen head 0d285cd3f)

Blinded second reviewer. Probes were ADDITIVE against the frozen head:
the five rows in `probe-rows.rs` were appended to
`crates/profile/tests/bool11_probes.rs`, run, and REVERTED (byte
identity re-verified); the editor-core row in
`probe-lifting-door.rs` was appended to
`crates/editor-core/tests/switch_program_vocabulary.rs` the same way.
The four mutants were each applied once, run, and reverted. Full
findings in the R2 report.

## Probe rows (all green on the frozen head)

- `r2_probe_band_edges_at_one_ulp` — accept inclusive at ε, escalate
  at ε ± 1 ulp inside the band, refuse inclusive at K·ε, two-sided.
  Green at ambient, `CAD_TOLERANCE_EPS=1e-6`/`1e-12`,
  `CAD_AMBIGUITY_K=1.001`/`1.0000001`/`2.0`/`1.0` (K = 1.0 exactly is
  rejected by the strict `> 1.0` validation and falls back to the
  default), and `1e-12`+`1.001` combined.
- `r2_probe_the_threshold_does_not_scale_with_the_leg` — the same
  lateral miss classifies identically under legs 1e-3, 1, 1e6 m: the
  no-lever design's observable content.
- `r2_probe_escalation_at_the_geometric_midpoint` — √(ε·K·ε)
  escalates for every legal K.
- `r2_probe_the_along_extent_rides_the_same_band` — behind-and-off
  refuses on the lateral miss; sub-ε ahead is `NonpositiveLeg`; the
  along extent has its own escalation band.
- `r2_probe_the_closer_boundary_is_the_point_forms` — an entry ε off
  the closing ray closes (5 vertices, none minted); K·ε off refuses
  `ContinuationTargetOffRay` before any seam classification.
- `r2_probe_the_lifting_door_refuses_continue_to_typed`
  (editor-core) — `LoopProgram::from_recorded` on a chain carrying
  `Step::ContinueTo` refuses
  `VerbNotInDocumentVocabulary(ContinueTo)`, message naming the verb
  and the schema-version story.

## Mutants (each applied once, run, reverted)

- **M1** (`mutant-M1.patch`) — the closer mints the entry again:
  `the_subdivided_square_closes_and_validates` reds 9 ≠ 8,
  `the_closer_mints_no_vertex_at_the_entry` reds 6 ≠ 5. The disclosed
  duplicate-vertex bug is genuinely pinned by count.
- **M2** (`mutant-M2.patch`) — the closer's seam refusal
  misattributed as `Departure`:
  `a_seam_at_a_subdivision_vertex_still_refuses_as_a_mid_carrier_seam`
  reds on the site assertion.
- **M3** (`mutant-M3.patch`) — `line_to(Start)`'s departure refusal
  misattributed as `Seam`:
  `the_seam_wall_ends_at_the_departure_and_stands_at_the_seam` reds.
  Site attribution is measured in both directions.
- **M4** (`mutant-M4.patch`) — `NOT_IN_DOCUMENT` emptied while the
  gap remains: `every_table_verb_is_a_document_program` reds naming
  `ContinueTo`. The census cannot be quietly suppressed. (The OTHER
  staleness direction — the document catching up while the entry
  remains — is NOT covered by the shipped falsifiability row; see the
  R2 report's MINOR.)

## Reproductions on the frozen head

- 64-ring hunt: `64 rings; undeclared closed 0, declared closed 32`;
  kite `[0, 2, 4, 6]`, rectangle `[1, 3, 5, 7]` at both widths —
  byte-identical to the PR's quoted output.
- Suite counts: 284 (default, 1e-6, 1e-12), 302 (`--features
  interval`) — as claimed.
- Red-first (src reverted to merge base 2f7edd2d, rows in place):
  RED as claimed in kind, but the numbers do not match: 31 printed
  error blocks (rustc tally "50 previous errors"), breakdown
  14×E0599 `continue_to`, 5×E0599 `ContinueTo`, 5×E0026 `site`,
  4×E0433 + 1×E0432 `CloseSite`, 2×E0599 `ContinuationTargetOffRay`
  — versus the PR's "37 errors", 4/5/5/5.
- Gate record: verified against the API — first head df773247c RED on
  exactly the two claimed legs (`display_unit` compile error in
  `m10_3_driver_k_probe_interval.rs`, binding census wanting
  `CloseSite`/`ContinueTarget`); final head 0d285cd3f GREEN with
  `test (interval, eps = 1e-12, 1/2 + 2/2)` executed.
