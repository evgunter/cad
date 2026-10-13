---
id: a-widened-rotation-angle-refuses-on-the-plain-interval-lane
kind: issue
title: a widened rotation angle refuses on the plain Interval lane at the first Node::Transform: cos^2+sin^2 is a bracket around 1 and transform_rigid_col0_unit is what notices
status: closed
opened: 2026-09-22
priority: P1
cost: D
refs: [SYM-14]
closed: 2026-10-13
---



## What

**Specifically requested by Ev, 2026-09-22** — found by SYM-14, the
chain demo Ev asked for, measuring the certified lane on it
(`demos/tour/src/chaintol.rs`, the cell's own CI row).

One leaf over the whole declared box, plain `Interval`, on a chain of
ONE link whose single joint carries σ = 0.01 rad:

```
node 7 — the transform op refused: transform: the map's linear part is
not an isometry at tolerance — predicate transform_rigid_col0_unit
refused, definitely or in-band
```

and the same at two, three and four links, always at the FIRST
transform. MEASURED, and pinned per link count by `chaintol.rs`'s
`the_certified_table_says_what_the_header_says`.

**The mechanism below is a reading of that refusal, not a second
measurement**, and is written as such: `eval::wire`'s `transform_map`
builds `Mat3::rotation_about(axis, angle)`, whose columns are built out
of `cos(angle)` and `sin(angle)`; on an interval angle those are two
independent brackets, so `cos² + sin²` would be a bracket AROUND 1
rather than 1, and that is what a column-unit check would refuse on.
It fits the predicate's name and the map's construction, and nobody has
yet read the column norm's own enclosure at the refusal to confirm it.
What IS established without it: the refusal is at the first transform,
at every link count, and it is not a geometry error — the same document
certifies whole on the symbolic lane at one link.

**The symbolic tier discharges exactly that**, and that is the whole
difference between the two lanes on this document: the same one-link
chain CERTIFIES whole at `Sym<Interval>` in 0.16 s, with
`symbolic_zero: 1760`. So the finding is not "the kernel is wrong", it
is that **the plain interval lane cannot carry a rotation by a widened
angle** wider than about `ε/2` rad (measured, "What Phase 1 found"),
and every consumer of a parametric placement is on the symbolic tier
or nowhere.

Worth stating because the tier is a *performance and reach* dial
elsewhere (`DriveConfig::symbolic`, `SymBudget::none()`) — on this
construction it is load-bearing for the answer existing.

## What answers it

First, cheaply: the column norm's enclosure at the refusal, read out —
which turns the reading above into a measurement or replaces it.

Then, either an interval-lane rotation that keeps the Pythagorean identity
(the rotation built so the column norms are exactly 1 by construction,
rather than recomputed from two independent brackets), or the
statement, where `transform_map` is written, that a widened
`SlotId::RotationAngle` is a symbolic-tier-only construction and the
plain lane's refusal is the designed answer.

## Home

SYM — the tier's reach on a construction Ev asked for.

## What Phase 1 found (SYM-17)

Measured on branch `sym/17-probe` (`084302d4cc`), which instruments
`topo`'s `check_rigid` and `eval::wire`'s `transform_map` behind
`SYM17_PROBE` and adds an `#[ignore]`d row, `chaintol`'s `sym17_probe`
(one leaf over the whole declared box, plain `Interval`, the default
ε, `SYM17_LINKS` and `SYM17_SIGMAS` from the environment). Run from
`demos/tour`: `SYM17_PROBE=1 SYM17_SIGMAS=0.01 cargo test --bins
sym17_probe -- --ignored --nocapture`.

### The readout: the item's reading is the cause, and it is the smaller half

At the refusal (one link, σ = 0.01 rad, node `e6c4609824dc`):

- The angle's box is `[−0.0299998, 0.0299998]` (the analyzed box's
  half-width `h` is `2.99998·σ`). `sin` encloses `±0.0299953` and `cos`
  `[0.99955004, 1]`, both tight.
- The axis is the literal `+z`, exactly `(0, 0, 1)`, so
  `Mat3::rotation_about` builds column 0 as exactly `(cos, sin, 0)`:
  `t·0` is zero and adds nothing.
- `c0·c0 − 1` encloses `[−1.799e-3, +8.997e-4]`, from `cos·cos =
  [0.99910028, 1]` and `sin·sin = [−8.997e-4, +8.997e-4]`.
- The band is `Band::linear` at the default ε: zero `1e-9`, escalate
  `1e-8`.

So the item's reading holds: `cos` and `sin` are independent
brackets, and `c·c + s·s` is evaluated over their product box,
`[cos²h, 1 + sin²h]` and not `1`. One thing widens it further: `dot`
multiplies `sin` by itself as two independent factors, so `sin·sin`
straddles zero (`−sin²h`) where the square is one-sided. That doubles
the lower end: the enclosure is `[−2·sin²h, +sin²h]`, width
`3·sin²h ≈ 27σ²`. Read at two more σ: `[−1.79997e-5, 8.99983e-6]` at
`1e-3`, and `[−1.79997e-7, 8.99986e-8]` at `1e-4`.

**`col0_unit` is not the check that bounds the plain lane.** It is the
first of the seven in `check_rigid`'s order, and it is second order in
σ. The orthogonality margin `c0·c1 = cos·(−sin) + sin·cos` is first
order, `±2·sin h ≈ ±6σ`: `±0.05999` at σ = 0.01, `±6.0e-3` at `1e-3`,
`±6.0e-4` at `1e-4`. `det − 1` is `[−2.25e-3, 1.35e-3]` at 0.01.

The largest σ that passes, bisected on the same row:

- `col0_unit` alone passes from σ = 7.4e-6 (refuses at 7.5e-6).
  Prediction from `2·sin²h = ε`: 7.454e-6.
- The whole map passes, and the **whole leaf certifies**, at σ =
  1.6e-10, at one link and at four. At 1.7e-10 it refuses on
  `transform_rigid_col01_orth` ("it may shear"). Prediction from
  `2·sin h = ε`: 1.667e-10.

So the plain lane does carry a widened rotation angle, if the angle's
box half-width is under about `ε/2` rad. `chaintol.rs`'s "does not
carry a widened rotation angle at all" is false at that scale, as was
this item's "at any width" before this section corrected it. The threshold is the
dimensionless margin read against the metre band, which is ledger
row F10's open arm.

### The candidates, measured at one and four links (σ = 0.01)

**(a) A rotation that keeps the identity by construction.**
- **Respelling the entries cannot pass.** Any spelling whose entries
  are tight enclosures of `cos` and `sin` over the box gives
  `c0·c1 ⊇ [−2·sin h, 2·sin h]`, because that is the range of
  `−c·s + s·c` over independent `c` and `s`. The orthogonality check
  above refuses it at first order. A parametrization with unit columns
  (the half-angle rational form, say) changes `col0_unit` and leaves
  `col01_orth` as it is.
- **The certificate carried from the construction is what can pass.**
  Over ℝ, `rotation_about(axis, θ)` re-normalizes its axis, so it is
  exactly orthogonal with determinant +1 at every θ in the box. It is
  sound to not re-derive that from the entries, with finiteness still
  asked, since an infinite angle poisons `sin`/`cos`.
- **Measured with the seven linear checks skipped** (`SYM17_SKIP_LINEAR`):
  the plain lane still refuses at the **same first transform**, one
  step later. The edge re-certification's `carrier_endpoint_start`
  encloses `[0, 8.9988e-5]` against `(1e-9, 1e-8)`, on `EdgeKey(1v1)`,
  at one link (node `e6c4609824dc`, 5 refused) and at four (node
  `da2481212205`, 26 refused). Its text says "this is a kernel defect;
  report it", which is worse than today's.
- **That wall is SHELF's**
  `transform-rigid-recertifies-images-enclosed-apart`: the images of one
  widened map are enclosed apart. Its first candidate, "transport the
  source certificate across the isometry", is (a)'s certificate
  extended from the map to the edges.
- **What (a) alone buys is ~30× in σ at one link and 3–6× at four.**
  One link certifies whole at 5e-9 and refuses at 9e-9 (pcurve
  `EnvelopeTerm(FidelityU)`, `[0, 1.38e-9]`). Four links certify at
  5e-10 and refuse at 1e-9. Shipped, both certify at 1.6e-10.
- **Where it would live.** A carried-rigidity map type in `geom-core`'s
  `linalg` (`mat.rs`/`affine.rs`, FLUX's ground, not PROPS'). A second
  door into `topo::transform_rigid` that takes it in place of
  `check_rigid` (`transform.rs`, claimed by OFFSET, SHELF and SHELL).
  The mint in `eval::wire`'s `transform_map` (WIRE's).
- **What it changes for other callers.** `check_rigid`'s doc names it
  "the one home of the rule … so the two cannot come to hold a map to
  different standards", and a carried certificate is a second
  standard. No existing caller's decision changes unless it moves to
  the new door. The natural movers are every editor-side map built
  from `rotation_about`: the transform node, a circular pattern's step,
  and the mate solve's derived offset. On `Sym<Interval>` the seven
  margins would stop being asked, and the 1760 `symbolic_zero` count
  would drop. That count was not measured.

**(b) The rigid checks asked at a tolerance the enclosure fits.**
Measured with `SYM17_BAND_ZERO`, which replaces the band of the seven
linear checks only. At a zero band of `0.059` the map still refuses on
`col01_orth`. At `0.06` it passes, and the lane meets the same edge
re-certification wall as (a), at one and at four links.

It is **not sound**. A band of 6% admits a map that shears or scales
by up to that much, and `transform_rigid` then copies radii and
semi-axes verbatim (`map_carrier`, `map_surface`). The check guards
the TRUE map's rigidity, and an enclosure of `±0.06` says nothing
about it. Scaling the band to the angle's own width cannot tell an
honest widened rotation from a non-rigid map with the same
enclosure. Only the construction can, and that is (a).

**(c) The statement that a widened `SlotId::RotationAngle` is the
symbolic tier's.** It needs no new machinery. The refusal text names
the branch today: "it may scale an axis … Recourse: use only a
rotation and a translation", said to an author who used only a
rotation and a translation. Saying the cause needs the node to know
that its map is a rotation of a widened angle. `wire_transform` knows
the step is `Step::Rigid` and holds the angle's enclosure.

The measurement sharpens the statement. The plain lane carries an
angle box of half-width under about `ε/2` rad (1.6e-10 σ here, at one
link and at four), and wider is the symbolic tier's.

### Recommendation

**(c), sharpened as above, with (a) recorded on SHELF's slate as the
route that would lift it.** Cost:
- `eval::wire` (WIRE's ground): the statement at `transform_map`, and
  a refusal that names the widened angle as the cause. That is one
  `NodeErrorKind` reading or variant. Its persist and class mappings
  sit beside `NotRigid`'s (`persist/check.rs`, `eval/class.rs`).
- The pins: `chaintol.rs`'s `assert_row` plain-lane arm and its header
  table and prose, plus `refusal_concision_chains` if the variant is
  new.
- Effort M.

(a) is the right design in the long run, since rigidity belongs to
the construction. But on this chain it buys nothing at σ = 0.01
without SHELF's transport (measured above), and alone it trades
today's refusal for one that calls itself a kernel defect. Landing it
is SHELF's design item, together with the transport. (b) is unsound.

**Both stop rules apply.** (a) lands in FLUX's and SHELF's/SHELL's
code, as a second standard beside `check_rigid`, and (c) states a
scope of the plain lane, which is Ev's to rule.

## Question, and Ev's answer

What the plain interval lane owes a rigid placement whose angle is an
interval, and where rigidity is known.

SYM-17's Phase 1 measured the cause; it is on branch
`sym/17-rotation-readout`, with the probe on `sym/17-probe`.
`topo::transform_rigid` re-derives rigidity from the matrix entries.
`cos` and `sin` are independent brackets, so column orthogonality
misses by about ±6σ. The plain lane therefore carries only an angle
box narrower than about ε/2 rad. On main the symbolic lane certifies
the one-link chain over the whole study at σ = 0.01.

Phase 1's own pick was (c): state that a widened angle is
symbolic-tier-only, and have the refusal name that cause.

Two designers weighed the question independently and converged after
one round. Both recommend a rigid-motion witness type `Rigid3<T>` in
`geom-core`'s linalg, which `transform_rigid` takes:
- It is rigid by construction from rotations about a decided axis,
  translations, products and transposed inverses. Each of those
  constructors decides only finiteness.
- Today's `check_rigid` becomes its one decided constructor, used for
  a map that arrives as twelve numbers (a literal frame, a STEP
  placement).
- An undecided margin there stays undecided.
- A product of admitted frames is not decided again.

Both reject (c). E12 already states the plain lane's scope for every
identity in one parameter, and a refusal naming the angle would be a
cause written from theory. The witness lands on its own; SHELF's
`transform-rigid-recertifies-images-enclosed-apart`, carrying edge
certificates across the map, is its own question. No ratified decision
changes.

**Ev's answer (2026-10-12): the witness, merged with `OrthoFrame`, and
closed under the rigid group's action.** Ev asked two follow-ups first:
whether the idea generalises (mates especially), and why the two
designers' unifications could not be combined. The answer is the
combination:
- **One type.** `OrthoFrame` merged with the rigid-motion witness. It
  keeps the frame's accessors (`origin`, `u`/`v`/`w` as `UnitVec3`)
  and its decided mints. It gains the motion's mints by construction:
  rotation about a `UnitVec3`, rotation about an axis through a point,
  translation, exact half-turns, compose, and the inverse as the
  transpose. Each mint decides only finiteness. One decided door,
  today's `check_rigid`, admits a raw `Affine3`, and returns an
  undecided margin as undecided.
- **The action.** The type acts on `UnitVec3` and on itself, so
  revolves (`revolve/partial.rs`), sweeps (`swept.rs`) and patterns
  (`eval/wire/stepped.rs`) mint rigid motions too.
- **Callers.** `topo::transform_rigid` takes the type and asks nothing
  about rigidity. Placements (`Step::Rigid`, `chain_motion`) and the
  mate solve's representatives and inverses mint it. A literal `Frame`
  goes through the decided door. `editor-core`'s persisted `Frame`
  stays a general linear part, so an improper map stays representable
  and can be refused.
- **Products of admitted frames** are not decided again.
- **No general structure-group mechanism.** The kernel acts with one
  group. Mirror is built inside its own door (MIRROR-DESIGN P2), and
  nothing scales.

The remaining identity losses are a relation between two separately
enclosed objects, and one parameter reaching a result by two paths.
Both are the symbolic tier's (E12), and SHELF's
`transform-rigid-recertifies-images-enclosed-apart` is its own
question. The mates' re-measurement is retired by INTENT stage 3
("a mate places and never checks"). The type should land before
stage 3 rewrites `coset.rs`.


## What Phase 2 built (SYM-17)

Ev's answer, built on `sym/17-rotation-readout` (PR #4197):
`geom_core::RigidMotion` replaces `OrthoFrame`, with mints by
construction, the decided door `RigidMotion::check_rigid`, and the
action on directions and on itself; `topo::transform_rigid` takes the
witness. A `Transform` step, a stepped pattern and a revolve's turn
mint their motion; a literal frame, a STEP placement and a pose the
mate solve composed are decided at the door.

Measured on the chain's one-leaf plain-lane row (default ε, release),
before on `main` `0fdbf00755`, after on the branch:

| links | before: certifies up to σ | after: certifies up to σ | after: refuses at σ = 0.01 on |
|---|---|---|---|
| 1 | 1.666e-10 (then `col01_orth`) | 6.49e-9 (then `pcurve_envelope`) | `carrier_endpoint_start`, `[0, 9.0e-5]` |
| 4 | 1.666e-10 (then `col01_orth`) | 6.73e-10 (then `pcurve_envelope`) | `carrier_endpoint_start`, `[0, 9.0e-5]` |

The next wall is SHELF's
`transform-rigid-recertifies-images-enclosed-apart`, as Phase 1
predicted. Its refusal ends "this is a kernel defect; report it", which
is `work/issues/an-undecided-margin-over-an-interval-box-ends-as-a-kernel-defect.md`.
The mate solve's representatives and inverses are INTENT's
`the-mate-solve-s-representatives-and-inverses-mint-the-rigid-motion`
(the stop rule applied: stage 3 B is rewriting `mate/`).

Closed by SYM-17 Phase 2 (PR #4197), 2026-10-13.
