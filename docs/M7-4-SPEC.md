# M7-4 spec — the wild corpus (binding)

Mandate (docs/M7-PLAN.md unit 4, green-lit by Evan 👍 on #190,
2026-08-05): import suitably-licensed STEP files nobody here
authored. Substrate: the hunt + empirical triage at
`~/.local/share/cad-work/wild-corpus/` (28 files, 4 license-
verified veins, per-file triage in results*.tsv, layered
scratch-surgery proof, ranked unlock list — read inventory.md
FIRST). Headline the unit builds on: 0/28 imported as-is, 28/28
refused TYPED, zero panics or hangs — the gaps are dialect, not
geometry, and the fail-loud contract held on every foreign file.
This spec is binding: deviations are REPORTED (numbered, with
the executed blocker), never improvised.

## 0. The fence (M7-1-SPEC §0 carries over whole)

All work in `crates/step-import`. The committed corpus is the
inventory §5 shortlist (8 imports-class + 5 refusal fixtures)
under `tests/fixtures/wild/<vein>/`, each with a provenance
header comment (source URL, license, generator) and license
texts beside each vein; a crate-level NOTICE file carries the
NIST acknowledgement line. License dispositions (orchestrator-
ruled, flagged for Evan in the PR): STEPcode data files ride the
repo's BSD-3 (repo license governs its committed assets);
cq_red_cube rides cadquery's Apache-2.0 (the maintainers'
assertion); NIST files are public domain with a NOTICE
acknowledgement. If any candidate's provenance looks worse on
close reading, DROP it and report — never stretch a license.

## 1. Scope: the measured unlocks, in ranked order

**Leg A — parser dialect (unlock 2).** Strings wrapped across
raw newlines (ST-Developer column-72 wrapping) and CRLF line
endings parse correctly; comments inside entity records are
skipped. `\X2\` escapes stay a typed refusal (gates one file —
a committed refusal fixture, not worth a decoder for
unconsumed annotation text).

**Leg B — unit-context tolerance (unlock 1, 17/28 first-blocks).**
- `CONVERSION_BASED_UNIT` with an in-file conversion factor
  (inch = declared 25.4 mm, degree = declared π/180 rad):
  resolve the factor FROM THE FILE's own conversion expression,
  never from a hardcoded table; a conversion unit whose factor
  entity is missing or non-length/angle refuses typed. This
  retires the M7-1 conversion-unit refusal by the S9 flip
  pattern (the refusal test flips with its history).
- Unit clusters unreferenced by the geometry's own context
  (mass/density SI_UNITs) are ignorable — the resolver checks
  the units the GEOMETRIC_REPRESENTATION_CONTEXT actually
  references, not every unit instance in the file.
- `GEOMETRIC_REPRESENTATION_CONTEXT(2)` parametric "2D SPACE"
  contexts (pcurve DEFINITIONAL_REPRESENTATIONs; 145 in one
  build123d file) are recognized as non-solid content and
  skipped without consuming a length-unit requirement.

**Leg C — VECTOR magnitude ≠ 1 (unlock 3).** Accept any
positive magnitude: normalize the direction, fold the magnitude
into the line's parameter scaling (the file's line parameter is
not arc length; the kernel's is — the derived parameter
intervals rescale, certification re-pins as always). Retires the
M7-1 refusal via S9 flip. Magnitude ≤ 0 or non-finite refuses.

**Leg D — assemblies with rigid transforms.** Extend M7-2's
identity-only assembly traversal: ITEM_DEFINED_TRANSFORMATION
composing a RIGID motion (rotation + translation; orthonormal,
det = +1 at ε_in) applies to the solid via the kernel's own
rigid-transform door. det = −1 (mirror) or scaling refuses
typed naming the transform entity (mirroring has orientation
implications the import must not guess at).

**Leg E — stretch, evidence-gated.** EDGE_CURVE same_sense
`.F.` (two wild files surface it): implement ONLY if one of the
8 imports-class targets needs it to reach green — compose the
sense into the half-edge direction derivation, with both
orientation controls (the M7-2 review's box/sphere probes)
re-run. Otherwise it stays a typed refusal with a committed
fixture and the need recorded. Report which branch was taken
and why.

## 2. Acceptance rows (named tests; all binding)

1. **Wild-imports row**: all 8 shortlist imports-class fixtures
   import; validity ladder green at default ε (or a documented
   in-band escalation, per the M7-2 declining-vs-false rule);
   censuses match expectations derived from the FreeCAD oracle
   run on the ORIGINAL files (recorded in the fixture's expect
   data with the oracle run cited); volume matches the oracle's
   within a stated tolerance (wild files have no closed forms —
   the oracle is the only independent volume; state that
   honestly).
2. **Cross-dialect fixed point**: each imported wild body →
   `step_string` → re-import → censuses/volumes bit-identical;
   second export byte-identical to the first.
3. **Refusal fixtures**: the 5 shortlist refusal files refuse
   typed with their exact classes pinned (NURBS face, trimmed
   splines, `\X2\`, 2D-context-only, wrapped-strings+NURBS);
   messages name entities.
4. **No-panic sweep**: every committed wild fixture (all 13)
   through `import_step` asserting result-not-panic under a
   catch-unwind harness — the wild contract, pinned forever.
5. **Regressions**: own-corpus (M7-1) and FreeCAD-corpus (M7-2)
   suites green UNCHANGED; the two S9 flips (conversion units,
   vector magnitude) carry their history.
6. **ε_in**: inch-file declared uncertainty scales through the
   conversion factor correctly (assert the value); override
   wins.

## 3. Constraints (M7-1-SPEC §3 + M7-2 amendments carry over)

f64 only; fail loud with structured refusal data; flat ε_in
budget (the #188 amendment); no new deps without the
release-age policy; Decide/K note and the #89 watch unchanged —
wild corpora are exactly where landings are expected; report,
never retune. The triage harness from the hunt is disposable —
the committed tests stand alone.

## 4. Local battery scope

Crate suites at default ε foreground as you build; one
`cargo check --workspace`; the FreeCAD oracle locally on the 8
imports-class fixtures once (this machine has it). Hosted CI is
the gate.
