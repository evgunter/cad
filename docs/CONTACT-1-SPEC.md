# CONTACT-1 — one local material-cone analysis for every touch kind

**Binds one implementer lane.** Deleted at merge; `work/contact/CONTACT-1.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `contact/1-touch-cones`. Rows carried:
`work/contact/touch-kinds-without-a-local-side-analysis-block-the-material-test`
(the unit's subject) and, riding, `work/contact/declared-faces-has-no-cross-solid-check`.
Read both in full. Also read — do not carry —
`work/contact/partial-overlap-with-touch-only-boundaries-clears-at-the-census-gate`,
the next unit on the same file; the interaction is below.

## Where the code is (orchestrator's survey at `1d5922f1`; a hypothesis — check it)

`crates/topo/src/census.rs`, `sweep_cross_solid_backstop`, arm 2:
`Side`/`sides`/`vf_rest`/`ef_rest`/`touch_verdict` (the local analysis),
the `blocks` precondition closure, the `probe` material test, and the box
gate. `blocks` answers `Undecided::TouchUnanalysed` unconditionally for
`VertexVertex`, `VertexOnEdge`, `EdgeEdgeOverlap` and `ConformalPatch`
contacts between the pair's two solids, and for a declared `vv` record.
The touch kinds are `validate::CensusContact`. The one existing
material-side reader elsewhere in the file is `ee_cross_backed`
(`geom_brep::classify_dihedral` → `classify_material_pairing`).

## The invariant

The backstop clears a pair only if the two boundaries do not cross, and
a touch is the degenerate shape of a crossing. So a touch between solids
A and B at a feature point `p` may be admitted as a REST **only if the
two materials' local cones at `p` have disjoint interiors** — the cone of
A being the set of directions from `p` that enter A's material, and the
same for B. Anything else — the cones overlap, or the analysis cannot
tell — is not a rest. The existing `vf_rest` and `ef_rest` are sufficient
tests for that condition for their two kinds (one plane through `p`
separates them). They are not the definition.

**The failure mode is a confident wrong answer**: an analysis that is too
lenient turns today's typed refusal into a clear over overlapping
material. Leniency is the direction to fear; a narrowing that refuses
typed is acceptable when it is named.

## Settled design

**S1. One home.** A single analysis decides every planar touch kind:
VV, VoE, EE-overlap, conformal patch, and the existing VoF and EiF. Either
`vf_rest` and `ef_rest` become cases of it, or it is built from them.
Do not add four sibling functions beside the two that exist; that would
be the defect this unit's review looks for first
(`docs/prompts/reviewer-style-lane.md` §1, last bullet). The local cones
are read from the snapshot's topology around `p`: incident faces, edges
and senses on each side. Every sign is decided with `decide` under the
run band through a `Margin` door. An `Err` result is `TouchInBand`, never
a guess.

**S2. Method is yours, soundness is not.** An exact interior-disjointness
test is best. For an edge feature it reduces to angular intervals in the
plane normal to the edge; for a vertex it is spherical polygons. A
sufficient test is also acceptable: for example, a separating plane
chosen from the face planes incident at `p` on either side. That test
refuses some true rests, such as a peg seated in a concave corner. If
you take a sufficient test, say in the PR what it refuses. Give each
such class its own file on this slate (implementer discipline §6), and
refuse it typed. A curved face or edge at `p` stays `TouchUnreadable`,
as it is today.

**S3. Declared records take the same analysis.** A declared `vv` record
is decided by the same analysis rather than blocking outright. The
declared `faces` pair (`DeclaredFacePair`) is out of scope.

**S4. Refusal wording.** The `Undecided` variants keep their role.
`TouchUnanalysed` should end up describing only what is still
unanalysed, and may go away entirely if nothing is. Every raise site
goes through an `Undecided` variant
(`the_backstop_writes_no_sentence_of_its_own`), and the census header's
reach statement lists what is examined now.

## The guards that matter

- **`a_straddling_part_with_touch_only_crossings_is_blocked_on_an_unanalysed_touch`**
  (`crates/topo/tests/bool4r2_probes.rs`): an L-bracket straddle with a real
  overlap, refused today only because a VoE touch is unanalysed. A correct
  analysis reads that VoE as a crossing, not a rest. **This row must not
  become a clear.** It is expected to change to a non-rest refusal; re-name
  it and re-baseline its wording.
- For **each** of the four kinds, add a rest row that now clears and a
  crossing row that refuses, using the same fixture shape posed two ways.
  Examples: a box resting on another box's edge, corner on corner, two
  boxes sharing an edge segment, face-to-face stacked boxes (conformal
  patch, opposite normals). Each crossing counterpart is a pose where the
  materials overlap near `p` while the exact sweeps still see only
  touches. One such pose is a conformal patch with the SAME outward
  normal on both sides. For a fixture the probe could also have caught,
  vary the pose until the touch analysis is the only thing standing
  between it and a clear, and say so in the row's doc.
- Include one row where `p` is in band, so the result is `TouchInBand`.

## The interaction with the next unit

The box gate clears two half-overlapping cubes before the material test
runs (`two_half_overlapping_cubes_are_cleared_at_the_gate`, pinned as a
wrong clear). That is the next unit's defect; do not fix it here. **But
if any row you add for a crossing only refuses because the gate lets it
through**, say so. That is evidence for the next unit's spec. Fix both
stale pointers to `work/bool/partial-overlap-…` (the probe's doc and the
census header) to name `work/contact/`.

## The ride-along

`Declared::index` must refuse a record whose two faces do not resolve to
two different solids. Give the refusal a typed variant, and thread the
`Result` through its callers. One row for it.

## Measure first

Before writing the analysis, run the existing material-test suites
(`bool4_material_containment`, `bool4r1_probes`, `bool4r2_probes`,
`h14_census_deferrals`, `m9_2b_r2_probes` via `tests/all.rs`). Record
which rows exercise `TouchUnanalysed` today. After the change, report
every row whose outcome moved, with one line per row on why the new
outcome is right. Goldens that move (e.g. `editor-core` `perf12_census_*`)
are re-baselined, and the PR says what moved.

## Out of scope

The box gate (next unit). `DeclaredFacePair`. Curved touches. Arm 1 and
the exact sweeps.

## Territory

`census.rs` is CONTACT's and shared with other programs. Run
`python3 scripts/work.py territory --base origin/main` on your branch and
announce any crossing in the PR.

## Review

Dual (two independent Opus reviewers, `docs/DUAL-REVIEW-PROTOCOL.md`).
The reason: a lenient analysis is a confident wrong answer on the door
that every consumer reads as proof.
