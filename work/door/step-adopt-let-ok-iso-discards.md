---
id: step-adopt-let-ok-iso-discards
kind: issue
title: step-import adopt.rs takes let Ok(iso) at two recognizer sites - S394's undecided half (EXCH's file)
status: closed
opened: 2026-09-06
closed: 2026-09-12
branch: door/step-adopt-iso-discards
refs: [S394, 2095]
---


## What

`S394` (closed by TRIM-1, PR #2095) converted the three `map_err(|_|`
swallows in `pcurve_cache.rs`. Its finding named two more discards of
the same `SplineError` through a different idiom —
`crates/step-import/src/adopt.rs`'s `let Ok(iso) = …` inside the
recognizers — and asked that they be decided separately: a recognizer's
refusal may be an answer ("this shape is not the one I think it is")
rather than a fault. TRIM-1's Closed section left them undecided.
`adopt.rs` is EXCH's file (Track U), so this is the handoff.

## Decision wanted

For each site: is a `SplineError` from `boundary_iso_*` a
"not-this-shape" answer (keep the `let Ok`, say so in a comment) or a
fault that should surface (convert to a typed refusal)?

## Home

Unowned at filing — EXCH's ground; filed by the TRIM orchestrator from
PR #2095's dual (R1 NOTE-3).

## Re-homed to DOOR (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

DOOR collects the rows whose fix is already written in the row — one PR
each, no design question left open. This row is here because a lane can
take it and land it without deciding anything first.

Its class at the cut was **E** — two sites in one file;
keep-with-comment or typed refusal, both options spelled out. The class
is a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## Closed (2026-09-12, `door/step-adopt-iso-discards`)

**Both sites are faults, not answers. Both convert.** The decision was
taken per site as the row asks; the answers came out the same, and the
reason is the door's own doc rather than a reading of the two call
sites.

### The door decides it

`geom_brep::boundary_iso_u` / `boundary_iso_v` are control-net COPIES —
no arithmetic — and their `# Errors` sections say so outright:
*"[`SplineError`] — unreachable for a surface that already validated
(the row's counts match `knots_v` by the surface's own construction),
surfaced rather than swallowed (D4 ¶2)"*
(`crates/geom-brep/src/nurbs_iso.rs:47-52`, `:66-70`).

So the refusal carries no information about the shape a recognizer is
asking after. It says one thing only: **a weight on the extracted
column is not a positive finite number** — which
`geom::NurbsSurface::new` refuses of the whole net at construction
(`crates/geom/src/surfaces/nurbs.rs:509`, `crates/geom/src/net.rs:89`),
so no body this reader assembles can be in that state. Reading it as
"not this shape" converts an impossible kernel fault into a routine
negative.

**A weight violation is the ONLY payload either door can produce**, and
that is narrower than the row (or this lane's first draft) assumed.
Measured against a validation-bypassed `NurbsSurface::new`, on a 3×4
net: a bad weight on an extracted row gives
`NonPositiveWeight`/`NonFiniteWeight` with its `index` counted along
the COLUMN (net index 11 renders as `index: 3` for `boundary_iso_u`,
`index: 2` for `boundary_iso_v`), while a weight interior to the net
returns `Ok` at all four ends. `WeightCountMismatch` cannot arise at
all — extraction slices `control` and `weights` to one length — and
`ControlCountMismatch` cannot either, because a net whose length
disagrees with its knots **panics in the slice** at `end = true`
(`nurbs_iso.rs:59`/`:80`) before any refusal is built, and returns `Ok`
at `end = false`. That panic is disclosed in PR #2406's body for the
orchestrator to place: it is `crates/geom-brep/*` ground, shared with
`loft.rs` and `pcurve_cache.rs`, and it is not this row's to fix.

The rest of the tree already reads it that way and says so at each
site: `sweep::LoftError::SeamStructure` carries it
(`crates/sweep/src/loft.rs:138-149`, `:523`),
`PcurveCertifyError::ChartRow` carries it three times
(`crates/geom-brep/src/pcurve_cache.rs:3564`, `:3959`, `:4091` — S394's
own conversions), and `editor-core`'s nesting row pins that the payload
survives into the rendered message
(`crates/editor-core/tests/lib_doors_node_result.rs:314-323`, `:404`).
`adopt.rs`'s two were the outliers.

### `:711` — the `IsoCurve` candidate rung

The `&&`-chain reads as one question but is four. Three of its
conditions ARE the rung's negatives — the carrier is not NURBS, the
wall is not a described NURBS chart, the column does not match the
carrier bitwise — and each of those leaves the candidate list unchanged
and lets the ladder try the next rung. The extraction is not a
condition at all; it is how the rung gets the thing it compares. The
orchestrator's hypothesis (`:711` may be a legitimate negative) is
**contradicted**: the site's legitimate negatives are the other three
conditions, and folding a fourth, different claim in beside them is
what made it look like one.

What the swallow cost: the withheld candidate never appears in
`StepImportError::Adoption`'s `attempts`, which is deliberately
structured data for a remedy flow. A corrupt wall would surface as a
ladder that quietly ran one rung short.

`iso_curve_candidates` now returns `Result<(), SplineError>` and the
caller attaches the edge id.

### `:877` — the ARC-rim residual gate

Here the `continue` is not even fail-safe in reporting: with both ends
skipped, `best` stays `f64::INFINITY` and the gate returns a refusal
that renders as *"deviates from the rim's circle by up to inf m"*. The
column IS the locus the rim claims to be, so a column that will not
extract leaves the gate with nothing to meter — and charging that to
the rim as a deviation misattributes a wall's broken structure to the
file's arc. The hypothesis is right here, by a different mechanism than
"a skipped candidate changes the answer": the gate cannot admit a bad
body by skipping, but it names the wrong subject.

`arc_rim_on_wall_boundary`'s `Err` is now `ArcRimRefusal`, a private
two-arm enum — `Residual(f64)`, the gate's own verdict, and
`ChartRow(SplineError)`, the structural fault — dispatched at the call
site.

### The refusal

One new arm, `StepImportError::WallColumnStructure { id, source }`,
serves both: same subject (the `EDGE_CURVE` being adopted), same
payload. Tag `wall_column_structure` through `pncad-py`.

### Class correction

The row's dispatch class **E** stands. It is two sites in one file plus
an enum arm; the ripple is compiler- and test-forced, not a design
question.

**A new `pncad-py` enum arm has FOUR rosters, not three**, and the
fourth is not a Rust test: the tag arm (`src/tags.rs`), the tag census
(`src/tests.rs`), the `.pyi` sentence — and
`tests/test_binding_census.py`'s
`test_every_member_of_a_matched_type_is_spelled_or_listed`, which only
the hosted python suite runs. It reddened this branch's first CI run
after three green local crates.

### Residue

`work/exch/arc-rim-gate-reports-a-degenerate-carrier-as-an-infinite-residual.md`
— the same gate's OTHER two `f64::INFINITY`s reach
`RimOffWallBoundary`'s *"deviates … by up to inf m"* without sampling
anything: the degenerate-carrier screen (`adopt.rs:900`) and the
non-evaluating sample (`adopt.rs:941`). `Residual` is a measurement
type carrying a non-measurement sentinel, which is this row's own
argument applied twice more, so the item is filed as the class rather
than as the carrier screen alone.

Measured on the fixture class the gate actually runs for — the native
rational arc loft, radius rewritten to `0.` and to
`-1.4142135623730951` — and both render the misattributed sentence
through `import_step`. It refuses either way, so a diagnostics defect
and not a soundness hole. Not widened into here;
`crates/step-import/*` is EXCH's ground and one PR is one row.
