# R2 probe lane (BOOL-8, PR #1508, frozen head 6aa2684f2)

Blinded second reviewer. Probes were ADDITIVE records against a frozen
head; the fix pass ADOPTED the four probe rows into
`crates/profile/tests/path_property.rs`, where they now run on every
CI leg, so this directory's copies of them (`probe-rows.rs`,
`probe-append.patch`) were deleted rather than kept as a second,
unchecked copy of rows the suite already carries. What remains is what
the suite cannot carry: this record, and the two mutation patches —
each applied once, run, and reverted — which are the falsifiability
evidence for the rows' claims. Full findings in the R2 report.

## Probe rows (all green on the frozen head)

- `r2_probe_authored_spellings_cannot_sneak_the_continuation` —
  `.toward` with the exact incoming displacement, `.turn(0)` off a line
  end, `.angle` at the exact incoming angle all refuse
  `JunctionTangent`; `.tangent().line` refuses `SameCarrierJunction`.
  No authored spelling reaches the continuation row.
- `r2_probe_arc_continuations_never_pass_validate` — TWO chained
  continuations off an arc still land at `UndeclaredTangency`; the
  carrier-blindness seam cannot be laundered past the data gate by
  chaining.
- `r2_probe_lily_seam_third_spellings_all_refuse` — third-spelling
  search on the seam wall: (a) `.tangent().tangent_arc_to(Start)`
  degenerates (`TangentLineClose`); (b) the REVERSED traversal hits the
  same wall (the corner/subdivision alternation is direction-blind);
  (c) a continuation landing exactly ON `Start`'s coordinates mints a
  directed point, not a closure, and the leftover zero-length closer
  refuses. The wall held against every spelling tried.
- `r2_probe_bitwise_inheritance_is_transitive` — FIRST RUN FAILED as
  originally written: three equal continuation legs do NOT lay down
  three bit-identical displacements. `0 + d` and `d + d` are exact, so
  the first pair matches; the third endpoint rounds (`2d + d` inexact)
  and its realized displacement differs in the last bit. What is
  inherited bitwise is the `Dir`; the vertex table shows it exactly
  only while the additions are exact. Revised to PIN the boundary:
  `d(0) == d(1)`, `d(1) != d(2)` — green. This bounds the PR row's
  "same displacement, exactly" comment to its fixture (a finding, see
  the report).

## Mutants (each applied alone, suite run, reverted)

- M1 (`mutant-M1.patch`, angle round trip instead of bitwise
  inheritance): exactly one row reds —
  `straight_continuation_inherits_the_tangent_bitwise`. The row's
  control is live; the bitwise claim is falsifiable and pinned.
- M3 (`mutant-M3.patch`, the kernel declares its minted joint): four
  rows red — `straight_continuation_subdivides_a_run_and_validates`,
  `continuation_off_an_arc_is_undeclared_tangency_at_the_data_gate`,
  `r2_probe_arc_continuations_never_pass_validate`,
  `r2_probe_bitwise_inheritance_is_transitive`. "Nothing is declared"
  is falsifiable and pinned.

## Other executions on the frozen head

- Red-first re-verified: `crates/profile/src` reverted to merge base
  7acef7d05 under the new suites → `E0599: no method named 'line' found
  for struct 'PartialPath<f64, HasPos<WithIncoming>, NoAng>'`.
- Ambiguity re-verified: `guided_replay.rs`'s annotation reverted to
  `PartialPath<f64, _, _>` → `E0034: multiple applicable items in
  scope`.
- Profile suite green at ambient default, `CAD_TOLERANCE_EPS=1e-6`,
  `1e-12` (260 = 256 + the 4 probe rows each) and `--features interval`
  (278 = 274 + 4).
- `cargo check --workspace --all-targets` green; `demos` project checks
  green.
- Gate record checked against the head's 21 check runs (17 executed
  green, 4 skipped; interval lane both shards on run 33525909911).
