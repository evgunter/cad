---
id: who-placed-it-is-read-off-the-description-not-the-door
kind: issue
title: Who placed a carrier is read off its description, not the door that placed it, so kernel builders declare and public input reads as the kernel's
status: closed
opened: 2026-10-26
closed: 2026-10-27
priority: P2
cost: M
design: true
refs: [a-recorded-skip-scaffold-rests-declared-by-its-own-pushforward]
---


## Finding

Raised by PR #5797's review (E1b, authority endings), filed without a
code change: it is a design question for a designer pair before E1c.

D3 ends a carrier that misses its face by who placed it
(`crates/geom-brep/README.md:606-608`): the modeler's lever on a
declared carrier, a kernel defect on a derived one. E1b reads "who
placed it" in one place, `geom_brep::description::authority_of`
(`crates/geom-brep/src/description.rs:434`): a `Scaffold` is declared,
a `Chart`/`OnPair` with `declared_by` is declared, everything else is
derived. That reads the DESCRIPTION, not the door that built the edge,
and the two disagree in both directions.

### (a) Kernel builders declare

These kernel builders emit a declared description themselves; no
modeler wrote the curve:

| site | emits | what it places |
|---|---|---|
| `crates/sweep/src/revolve/surfaces.rs:80` `revolved_strut_spec` (callers `revolve/full.rs:837`, `revolve/partial.rs:484`) | `Scaffold(RevolvedPoint …)` | a revolve's strut/rim |
| `crates/sweep/src/revolve/full.rs:947` | `Scaffold` | the full turn's return half |
| `crates/sweep/src/revolve/turn.rs:80` | `Scaffold(RevolvedPoint …)` | the turned strut |
| `crates/sweep/src/extrude.rs:711` `extruded_strut_spec`, `extrude.rs:1413` | `Scaffold(ExtrudedPoint …)` | extrude seams/struts |
| `crates/sweep/src/swept.rs:691` `placed_segment_spec` | `Scaffold(PlacedSegment …)` | profile edges |
| `crates/topo/src/replace_face.rs:2912` | `Scaffold(translate_mapped …)` | replace_face's translated scaffold |
| `crates/topo/src/splitting/finish.rs:1784` | `.declared_by(mc)` | a section boundary keeps a declared curve's authority |
| `crates/topo/src/transform.rs:795,826,832`, `offset_restate.rs:102`, `replace_face.rs:3603,3852,4612` | carried | restatements keep the authority they were handed |
| `crates/step-import/src/adopt.rs:631` | `.declared_by(mapped)` | an imported file's own curve (a modeler's, by the file) |

The sweep rows are the profile's own geometry pushed forward, so
"declared" is defensible there: the sketch entity determined the locus.
`replace_face`'s translation and the section boundary are kernel acts
that keep a declaration they were handed; whether a kernel act that
MOVES a declared curve still leaves the modeler's lever as the ending
is the question. Related: `work/cleavetail/a-recorded-skip-scaffold-rests-declared-by-its-own-pushforward.md`
(a stale scaffold restated declared by its own pushforward).

### (b) At rest: the lever against DESIGN's at-rest sentence

At rest, a declared row on an edge whose certificate said "on chart"
ends in the lever (`topo::validate::off_face_ending`,
`IncidenceRead::ending(authority, Reading::AtRest)`). DESIGN says
(`docs/DESIGN.md:466-467`): "At rest, only a definite contradiction of
what is stored is a defect." The two do not contradict on a definite
miss (it is a contradiction, and the lever names it for a declared
curve), but they say different things about WHO is told: README D3
incidence names the modeler; DESIGN's at-rest sentence names only the
stored contradiction. Which clause governs a declared row at rest is
undecided.

### S9: public input reads as the kernel's

`Body::set_edge_curve` (`crates/topo/src/attach.rs:1324`) is public. A
caller who passes a `Chart` description without `declared_by` reads
`Derived`, so a miss of a curve the caller supplied ends "kernel
defect". The door is the modeler's; the description says the kernel's.

### S6: a declared carrier that misses its own MappedSource

`CertCheck::MappedSource` routes `Miss(Unsized::Defect)`
(`crates/geom-brep/src/certify.rs:8750`) whatever the authority: a
declared carrier that does not match its own declaration's
pushforward ends as a kernel defect. If the declaration came from a
modeler, the mismatch may be the modeler's (a curve and a declaration
that disagree), the same split D3 makes for incidence.

## The question

Should authority be stamped by the door (a builder states who placed
the edge, the public setters stamp Declared) rather than read off the
description? And which clause governs a declared row at rest?

## Not done in E1b

E1b routes every incidence ending through one map
(`IncidenceRead::ending`) keyed by `authority_of`; changing what
`authority_of` reads changes every ending at once, which is why it
waits for the ruling.

### S10: a declared on-pair arc rim

The `OnPairNurbs` lane's arc rim (`CertifyError::OnPairArcRim`,
`crates/geom-brep/src/certify.rs:727`) reads an on-pair edge's rim
against its wall; an exact image's `ChartIncidence` folds into the
pair's SSI limb (`arc_rim_check`, `certify.rs:786`), so the refusal
ends as the pair's residual even when the description carries
`declared_by`. E1b routes only the chart mint's incidence; whether a
declared on-pair edge's miss is the modeler's belongs with the
question above.

### S11: the edge join's chart-class reading

`boolean::edge_join` reads a `Chart` edge's iso family off a fresh
derivation (`geom_brep::chart_iso_family`). A refusal comes back as
`IsoFamilyRefusal::Image(e)`, whose Display is the authority-blind
`{e}` (`crates/geom-brep/src/pcurve_cache.rs:8231`), and
`JoinRefusal::render` appends the kernel's defect ending to it
(`crates/topo/src/boolean/edge_join.rs`, the `ChartClass(Image(e))`
arm). The edge's authority is at hand there (`c.authority()`), but the
edge already passed its certificate's incidence on this same chart. So
whether a re-derivation's refusal is the modeler's, rather than the
kernel disagreeing with its own certificate, is the same question as
above. E1b leaves the join authority-blind until it is answered.

### S12: the boolean's loop walk

`boolean::ops::walk_refusal` (`crates/topo/src/boolean/ops.rs`) maps
every `CarrierOffChart` to `WalkRefusal::Torn`, and so to
`ClassificationInvariant`, a kernel defect, whatever the authority or
the read. On a face whose rows the at-rest pass certified, that is the
kernel's. On a face whose rows are not owed (`pcurves::not_owed`), a
declared carrier, or one read at a pose, can be at rest unread and
reach this walk on a cylinder, cone or torus operand. It then ends as
the kernel's: the boolean door's instance of the question.

### S13: the import door

`step-import`'s adopt (`crates/step-import/src/adopt.rs:631`) declares
the curves it adopts as `MappedCurve` candidates (`declared_by`). A
re-mint refusal of such a curve, `StepImportError::Pcurves`, renders
`PcurveMintError::render`, and so `render_by(declared, file)`. An
imported curve off its face therefore now reads "move the declared
curve onto the face it bounds", where it read "repair the body" before
E1b. Should a damaged file read the modeler's lever, or the file's
defect ending (`geom_core::DAMAGED_FILE_ENDING`)? The file's author
placed the curve, but the person reading the refusal did not.

### S14: who may stamp an authority on a refusal

`AuthorityKind` is built through `EdgeAuthority::{Derived,
Declared(..)}.kind()`, which any crate can call. The `authority` fields
on `topo::PcurveMintError::Certify` and `topo::EulerOpError::PcurveSplit`
are `pub`. So any door can stamp any kind on a refusal:
`pcurves::certify_refused` is the one constructor in the kernel only by
convention. Should the stamp be unforgeable (a sealed constructor that
reads the edge), or is the convention enough?

## Ruling (Ev, `[ev]` PR 5838, 2026-10-27)

Q1, after Ev asked "how can we just make it not happen?": the designers' joint answer ("looking good").
- Topo's construction tier is the kernel's internal API: the Euler operators, `Body::set_edge_curve`, the edge and face description setters, and their spec types. pncad stops re-exporting it.
- Every curve-and-face pair topo reads at a build is the kernel's own, so a definite incidence miss at a build is a kernel defect.
- A future verb that takes a user's curve refuses it at its own door, with its own lever.
- No ending reads `EdgeAuthority`. It is provenance only. `AuthorityKind`, `DeclaredOffChart`, `DeclaredIncidence` and `render_by` go.
- A read in band (at a pose or undecided) ends in the decision's lever at every door. At rest, a definite miss is a stored contradiction.
- A later unit: a refusal's `Display` carries only its payload, and its ending exists only through `render(at)`, with a gate over the production spelling sites.

Q2: while import healing does not exist, an incidence refusal at the import door ends in the damaged-file ending, without mentioning ε_in. The healing row and the later ε_in wording are tracked in `work/interp/adoption-rebuilds-caches-at-eps-from-eps-in-interpretation.md`.

The text: geom-brep README D3 and DESIGN D4 ¶1 (iv) (this PR). The implementation is `work/pcert/plan.md` E1d, and E1c follows it.


## Implementation

- **E1d (`pcert/door-endings`): the endings.**
  - E1b's authority routing is gone.
  - `IncidenceRead::ending(at)` is the one map: a definite miss ends by the door, and a read in band ends in the geometry's lever.
  - `CertifyError::CarrierOffChart` and `CertCheck::ChartIncidence` serve every carrier.
  - The import door ends a definite miss in the damaged-file ending, but only on the path that reaches the incidence reading: the pcurve re-mint on the adopted body (`StepImportError::Pcurves`). A real STEP file whose curve misses its face refuses earlier, at the description ladder's residual meters (the surface residual, the endpoint residuals, the mapped source), with the set-ε-to-ε_in stopgap where the miss is within ε_in. That stopgap is EXCH's, under Ev's Q2 ruling, and E1d does not change it. Measured on PR 5844 with review B's probe (a cone fixture's half-angle, apex line or rim radius nudged by 1e-12 to 1e-2 m, at three ε_in): no run reached the re-mint. The note is in `work/interp/adoption-rebuilds-caches-at-eps-from-eps-in-interpretation.md`.
- **E1d2 (`pcert/seal-construction`): seals topo's construction tier** behind `topo::construction::Construct`, implemented for `Body` with its 46 doors `pub(crate)` inherent methods. pncad's `topo` is curated without `construction` and the modules that hold only its vocabulary, pinned by three `compile_fail` doctests. A pncad consumer cannot name `Body::set_edge_curve` or any other construction door, so every curve-and-face pair a build door reads is the kernel's. This closes the item.
- **Filed:** `policy-lane-origin-callers-outlives-the-sealed-construction-tier` (`policy_lane::Origin`'s `Callers` arm after the sealing).
