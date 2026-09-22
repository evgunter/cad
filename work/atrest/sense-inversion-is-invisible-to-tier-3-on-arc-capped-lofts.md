---
id: sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts
kind: issue
title: inverting every face's sense on an arc-capped loft leaves tier 3 green with an unchanged positive enclosure
status: dispatched
opened: 2026-09-12
priority: P0
cost: H
parent: ATREST-2
---


## The finding

Found by a PERF-6 reviewer while probing the +V check, and OUTSIDE
that unit's fence: it is a fact about what `sense` means on an
arc-capped loft, not about when check 7 stops.

`Body::flipped_face_sense_for_tests` applied to EVERY face of the
three-station `arc_section` loft (the fixture
`crates/sweep/tests/common/mod.rs` calls `arc_prism`)
leaves tier 3 GREEN, and leaves the body's volume enclosure unchanged
and positive. The same whole-body inversion applied to
`square_prism` or to `step-export`'s `loft_prism` refuses
`LoopRoleInverted`.

So on a body whose caps are bounded by arcs, the stored sense bit can
be inverted on every face at once with no tier-3 consequence and no
change to the quantity check 7 reads. Either the bit means something
those bodies' faces do not carry, or it means something tier 3 does
not check on them.

## Why it is not a check-7 bug

The +V invariant reads a volume ENCLOSURE, and the enclosure did not
move: the quadrature lane's Green form is winding-derived end to end
(the signed UV area IS `s_f·|Ω|` through the stored loop traversal),
which `geom-brep`'s `props::quad` module docs state as a deliberate
property — no sense bit enters it. Flipping the bit therefore cannot
change the flux, and check 7 is reading the same body it was.

What is unexplained is the OTHER half: why the same inversion reaches
`LoopRoleInverted` on the polygonal lofts and not here, and what a
consumer is entitled to conclude from a sense bit that no at-rest
check on these bodies reads.

## Measured (2026-09-20)

Three answers, each pinned by a row in
`crates/sweep/tests/m5_s10_face_sense.rs`. The bodies are
`sweep::loft_body` output, which `topo` cannot reach, so the rows sit
in the S10 sense-acceptance suite rather than in `tier3_tests`.

**None of the three candidate causes above is it.** The rim roles are
stored, not derived; the arc cap's outer/ring role IS computed, and
this is the one the source had to be read for, because nothing in
`loft_body` computes it: the producer is `profile`'s validation. Its
containment pass builds the forest, takes the unique depth-0 loop as
`LoopRole::Outer` and every other as `LoopRole::Hole`, and
`canonicalize_loop` then REVERSES the loop where its signed area
disagrees with the role (`want_ccw` for `Outer`). Both steps run on an
arc-bearing loop exactly as on a polygon — containment and signed area
are arc-aware — and the loft carries the resulting role and winding
through. So the role is computed, once, upstream, and it is
containment-derived rather than winding-derived. (The winding-derived
ASSIGNER, `merge_faces::loop_winding`, runs at boolean-merge time and
never touches a body straight out of `loft_body`; it is
`work/zip/verbs-1031b-assigner-checker-divergence.md`'s subject, not
this one's.) And nothing gates the arc body out earlier —
`validate_geometric` on the honest arc loft is `Ok(())`, all nine
checks run. The cause is
narrower and entirely mechanical, and it is a CONJUNCTION of two
classes `work/verdict/m6-sense-gate-recorded-residuals.md` already
records (its residuals 3 and 4), reached here by a body that carries
both at once.

### 1. The raiser, and what goes the other way

`LoopRoleInverted` has exactly one at-rest producer: tier 3's **check
6, planar arm** in `topo::validate`, the Newell arm that falsifies a
loop's stored winding against `plane_outward_normal(face, normal)` —
the one place the planar bit is read as a claim. Its gate is
`all_lines`: the arm skips any loop whose certified carriers are not
all `geom::Curve3::Line`.

Measured, whole-body inversion of each loft:

- **square prism** — two planar caps, each a four-line loop, four
  spline-chart walls. Refusal set: exactly two `LoopRoleInverted`,
  one per cap, and nothing else. **The four walls contribute no
  refusal.**
- **arc prism** (`arc_section`, one bulged vertex) — two planar caps,
  each a loop of three lines and one `Circle`, four spline-chart
  walls. `all_lines` is false on both caps, so the arm skips them;
  the walls are skipped as before. Refusal set: **empty**.

So the discriminant is the cap alone, and the predicate that goes the
other way is `all_lines` on the cap's stored carriers. The arc cap is
not special as a *cap* — it is special as a loop with a conic carrier,
which is residual 4's class; the walls are silent on **both** bodies,
which is residual 3's.

### 2. Does any at-rest check read `Face::sense` on this body?

No. There are four readers in the at-rest battery and all four are
gated shut:

| reader | how it reads the bit | gate | why the arc loft misses it |
|---|---|---|---|
| check 6, planar arm | `plane_outward_normal` | `all_lines` | every planar face carries a `Circle` |
| check 6, curved arm | `(side == Sign::Positive) != face.sense` | skips `Plane` and `Surface::spline_chart()` | every face is one or the other |
| tier 3, check 4's MATERIAL arm | `sense_plus` / `sense_minus` on a definitely-smooth edge | `nurbs_adjacent` short-circuits to `ContactMark::Unmarked` | every edge has a spline-chart face |
| check 7 | `props::curved_face(surface, &outer, face.sense, band)` | reached only by a face with an iso boundary and no certified quad lane (the rimless sphere band) | no loft face reaches it; the reporting door gives the **same reading** under the inversion — bit-identical where it computes, the same typed refusal at the tight ε where these rational walls honestly run out of budget |

The `nurbs_adjacent` entry is the one not previously written down. It
is a deliberate, documented exemption BY KIND (implicit-form gradients
are poison on a spline chart, so `classify_dihedral` cannot run), and
its short-circuit precedes the material arm — so on a body every edge
of which touches a spline chart, which is every loft, the arm's
`Face::sense` read is unreachable too. That is an answer, not a new
defect: with no derived contact class there is no second encoding to
compare the bit against.

### 3. Is an inverted body reachable through the PUBLIC API?

**Yes — this is a real gap, not a test-door artefact.**
`topo::Body::set_face_sense` is `pub`, is not `#[doc(hidden)]`, and is
the door the constructors themselves use to attach the honest bit. The
pinned row builds the inverted arc loft with that door alone, from an
integration test outside `topo`, and `validate_geometric` returns
`Ok(())` on the result, at every ε row, with the enclosure unmoved
(and positive wherever the fixed schedule computes it).
The same public door on the square loft IS refused, which is the
control: the door writes the bit, so the silence is the checks'.

A second public route reaches the same state from outside the process:
`step-import`'s adoption phase copies `advanced_face.same_sense`
verbatim, and its pre-adoption inversion guard covers **cylinder and
cone wall faces only** — an imported solid whose arc-bounded planar
caps or NURBS walls are stated inverted is adopted and certifies.
Filed on EXCH's slate as
`step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard`.

### 4. The class, and where else it stands

The shape this unit kept meeting is **prose that justifies itself by a
check that does not reach the population it is talking about**. The
check is real, its name is spelled right, and the sentence is still
false, because the population in front of it is one the check skips.
Three instances, all found from this one measurement:

1. **`step-import`'s `normalize.rs`** — the pre-adoption inversion
   guard justifies its own narrowness by saying adoption "would copy
   both encodings verbatim and the kernel's tier-3 curved sense gate
   (check 6) would refuse the built body". It would not, for an
   arc-bounded planar cap or a NURBS wall. Filed on EXCH's slate as
   `step-import-adopts-an-inverted-same-sense-outside-the-cylinder-cone-guard`,
   which also cites the crate-level contract sentence in
   `crates/step-import/src/lib.rs` that the row actually falsifies.
2. **`topo`'s `set_face_sense` rustdoc** — *"Callers must keep the two
   encodings of orientation coherent (the bit and the loop winding —
   tier 3's check 6 falsifies planar disagreement at rest)."* Check 6
   falsifies nothing for a planar loop with a conic carrier, and this
   unit's row 3 is the executed witness. A doc over-claim from birth
   rather than drift: the `all_lines` skip is banner-documented as
   deliberate and residual 4 has recorded the exemption since
   2026-08-07, so narrowing the sentence erases no intended invariant.
   Narrowed by this unit, in `crates/topo/src/attach.rs` — TOPO's
   ground, announced as a seam in the PR.
3. **`mesh`'s `planar.rs`** — *"`topo`'s tier-3 validator refuses such
   a body by name (check 6, `LoopRoleInverted`) … the rule is
   ratified, its violation is refusable upstream, and the mesher
   assumes a body that has been through the door."* A whole crate's
   assume-don't-certify posture resting on an upstream refusal that
   does not exist for arc-bounded planar faces. TESS's ground, and the
   fix is not the sentence: what the crate's posture becomes when the
   upstream refusal is absent is TESS's design call. Filed on TESS's
   slate as `planar-mesher-posture-rests-on-a-refusal-check-6-does-not-make`.

Instance 3 is the one this unit had already READ and dismissed, in the
same pass that falsified its premise — recorded here because the
class's failure mode is exactly that: the sentence looks correct
because the check it names exists.

### The rows, and how each goes red

In `crates/sweep/tests/m5_s10_face_sense.rs`:

- `only_the_line_bounded_cap_refuses_a_whole_body_sense_inversion` —
  answer 1. Red when check 6's planar arm widens past line carriers
  (`work/zip/verbs-1031b-assigner-checker-divergence.md`'s open
  question), when the curved arm stops exempting spline charts — that
  raises `CurvedSenseInverted`, which the row's `other => panic!` arm
  catches, so it is still red but by a different value — or when a
  bulged loft segment stops minting an arc carrier.

  The row compares `(face, loop)` PAIRS, not face keys:
  `LoopRoleInverted` names both, check 6's planar arm runs over
  `face.rings` as well as the outer loop, and a face that refused on
  two of its loops would appear twice. It re-derives all four of the
  arm's entry conditions — planar surface, outer-plus-rings, `Cycle`
  boundary, `all_lines` — because omitting the planarity filter is a
  FALSE RED the day `loft_body` mints `Line` carriers for straight
  rails (the walls would enter the line-bounded set while check 6's
  behaviour had not changed at all), and answering "yes" for the
  non-`Cycle` boundary the arm SKIPS inverts the predicate's meaning.
  That the re-derivation is a third spelling of `all_lines` is
  recorded on `work/zip/verbs-1031b-assigner-checker-divergence.md`.
- `every_sense_reading_gate_shuts_on_the_arc_loft` — answer 2. Red
  when any of the four gates opens: a loft that mints an analytic
  cylinder for a circular-arc segment breaks both the
  `Plane`-or-spline assertion and the per-edge nurbs-adjacency one; a
  quadrature that folds the bit into a loft face's flux breaks the
  same-reading enclosure.
- `the_public_sense_door_builds_an_inverted_arc_loft_tier_3_accepts` —
  answer 3. Red when `set_face_sense` stops being public (a
  compile break), when a gate catches the public-door inversion on the
  arc loft, or when one stops catching it on the square control. The
  control asserts the exact refusal SET, not `is_err`: an `is_err`
  would still pass the day the square prism started refusing for an
  unrelated reason. The per-face bit comparison is KEYED, not a
  positional `zip` over two `faces()` iterators — the `zip` is sound
  today (`slotmap::SlotMap`'s derived `Clone` copies the slot vector,
  and `faces()` is documented as slot-index order) but rests on an
  unstated premise that the keyed form does not need.

Both gap-pinning assertions are deliberately monotone against the
kernel improving: they fail the day the gap closes, and the repair is
to re-cut the row, never to loosen it.

**Verified red by perturbation, and the perturbation ISOLATES.**
Swapping the arc prism for the unbulged square prism makes **all
three** rows fail, each at exactly the assertion that carries its
answer: row 1 at the `MEASURED GAP` `is_ok`, row 2 at the
planar-arm-examines-nothing assertion, row 3 at its own `MEASURED GAP`
`is_ok`. That swap alone does not isolate a cause, because the crate's
two shared prisms differ in three things at once — bulge, station count
(3 vs 2) and v-degree (2 vs 1). So the 2x2 was run:

| section | stations | v-degree | whole-body inversion refuses | loops check 6's planar arm examines |
|---|---|---|---|---|
| arc | 3 | 2 | `Ok(())` | 0 |
| square | 2 | 1 | 2 x `LoopRoleInverted` | 2 |
| square | 3 | 2 | 2 x `LoopRoleInverted` | 2 |
| arc | 2 | 1 | `Ok(())` | 0 |

Station count and v-degree move nothing; the outcome is constant along
each row of the carrier. **The `Circle` carrier on the cap loop is the
whole difference**, which is the sentence answer 1 rests on, now
measured rather than inferred from the two fixtures. (Re-derived
2026-09-21; the same measurement had been reported by this unit's
reviewer.)

Prior art, not duplicated: `m5_s11_concave_sense.rs`'s
`loft_concave_arc_walls_face_out_and_a_flip_is_invisible_below` already
pins a SINGLE lofted wall's flip being invisible. What is new here is
the whole-body inversion, the cap half, the gate enumeration and the
public-door reachability.
