---
id: sense-sign-multiplies-fold-onto-outward-normal
kind: unit
title: The ten hand multiplies of normal by sense_sign fold onto OutwardNormal, and Face::sense_sign retires
status: closed
opened: 2026-09-15
closed: 2026-09-15
branch: scalar/sense-fold
pr: 2668
---


## What

`D6`'s ruling (PR 2457), second unit, after
`sense-sign-doors-take-the-bit`. The ten `normal * face.sense_sign()`
sites (`emit_topo.rs` `face_plane`, `blend/build.rs` `outward_of`,
`blend/battery.rs` `outward`, `boolean/join.rs` `ring_run_ccw`,
`boolean/rest.rs` `face_carrier`, `boolean/solid_contain.rs`
`face_plane` and `face_geo`, `validate.rs` check 6, `merge_faces.rs`
`merged_outline_ring` and `planes_declared_equal`, `mesh/walk.rs`
`loop_polygon`) fold onto `OutwardNormal::from_chart` /
`face_outward_normal`; where a scalar (an area, a winding) is genuinely
signed, the bit is spelled as a conditional negation. Then
`Face::sense_sign<T>()` retires with `face_normal.rs`'s hand-kept
census. Pin: D9 bit identity over the touched crates' suites. Many
programs' ground (TOPO, BLEND, WIRE, S-MESH, BOOL); announce each.

## An eleventh site, found by unit 1's reviewers (2026-09-15)

`crates/sweep/src/blend/arms.rs` — `Sheet::trace` and `Ruling::trace`
both open `let side = if sense { T::one() } else { -T::one() }` from a
face bit (the `sense` parameter, fed from `battery.rs`'s
`convexity.ball_side(senses.0)`) and store it as
`SupportTrace::{Straight, Round}.side: T`, documented "The material
side, `±1`", consumed a function boundary away in `SupportTrace::contact`
and in the centre closed forms. It is D6 §4's class — a sense CROSSING a
boundary as a scalar `T` — and unit 1's three sweep patterns all miss
it: the bit arrives as a parameter named `sense` (not a `sense_sign`
read), and it leaves as a struct FIELD rather than a parameter.

**Two readings, and this unit decides between them.** Either the field
is a sense wearing a `T` and folds like the other ten, or the closed
forms consume a genuinely signed σ (a curvature/offset direction) that
happens to be seeded from the bit, in which case what is owed is the
bit selecting σ once at the mint and the field keeping its own name.
Read `contact` and the centre forms before choosing. BLEND's ground.

Test-side residue in the same class, to follow its door:
`crates/topo/tests/readback_sense_kind.rs` mints a `±1` from
`pose.sense`.

## Digest receipt: the D9 pin for "nothing's bits move"

The zero-parameter recipe of `work/scalar/rate-pair-in-geom-core.md`
§Digest receipt (build `demos/tour` release, run the binary directly
into the literal relative outdir `tour-out`, digest the sorted per-file
listing and the whole narration), taken at this branch's merge base
`origin/main` `d71bb6a78` before the first code change:

- **1766 emitted files**; digest of the sorted per-file digest
  listing:
  `87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892`
- the tour's **narration**, 729 lines:
  `e930abf542c371677b2c0d87b02c2bf84eb15fbaf62ded6389f48c899ef14d49`

Both are the values RATE-PAIR recorded at `4f71edaea` and at its head.
The head-of-branch pair is recorded in the PR body beside these.

## Closed (2026-09-15) — PR 2668

`Face::sense_sign<T>()` deleted with `face_normal.rs`'s hand-kept
census; a new call does not compile. Every production site folds onto
`OutwardNormal::from_chart` or a `face_normal.rs` door — `pub mod
face_normal`, the keyed `face_outward_normal` (`outward_of` deleted, its
three callers use it), the by-value `plane_outward_normal` for
`merged_outline_ring` and `shell.rs` — and the curved reading has one
home, `geom_brep::implicit_outward_normal` beside `implicit_gradient`
(dihedral, battery, contact_verify call it; the topo alias gone; the
header says the two curved readings differ). The genuinely signed
scalars (`walk.rs` area, `dihedral.rs` `kappa_rel`, `shell.rs`
thickness) spell the bit as a conditional negation, the D6 §0 shape.
`SupportTrace::{Straight, Round}.side` is `bool`; the `R ∓ r` selection
has one home, `sided(side, x)` in `battery.rs` beside `Convexity`
(`Convexity::signed` = `sided(self.blend_sense(), r)`), bit-identical
except a signed zero no consumer reads. `readback.rs` ×3 and
`interrogate.rs` prose name the constructor; the `sigma` oracle has one
copy in `sweep/tests/common/oracles.rs`. Guards: `the_planar_sense_flip_
lives_in_one_place` green unchanged; `no_source_file_folds_the_bit_by_
hand` over `crates/*/src` (`code_only`), pinned to the three sanctioned
scalars, the vector class at zero, blind spots in-file. Mutant at head
(`implicit_outward_normal` folds `true`): 128 red of 3618 over
topo+sweep+mesh+geom-brep. Digests identical at base `37dce8287` and
head (1766 files `87be4dd9…`, narration 729 lines `e930abf5…`).
Reviews: dual, both APPROVE WITH FIXES, no MAJOR; twelve fix-pass items
taken (site 4's opposite-orientation row declined as not cheap, receipt
on the TOPO row: the boolean's scope, not the bit). R2's probes merged
`--no-ff` and cut to one asserting row (the reversed rod wall:
`CurvedSenseInverted{wall}` + `LaminaWedge` on both seams); R1's not
adopted. Rows: TOPO
`declared-opposite-orientation-refusal-is-unreached-by-any-row`
(widened to the class), PROPS `m6-sense-gate-recorded-residuals` (the
rod instance, pinned), `anti-re-fork-row-reds-on-a-doc-pointer-to-the-
door` CLOSED. Routed: TOPO's three-answers row, CENSUS S57, PERF's
sense-inversion row.
