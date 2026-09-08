# PROPS sphere-pole-side — a rim's traversal says which side of it the face lies on

**Binding at dispatch** (PROPS program, `work/props/plan.md` §Lanes,
the sphere lane's first unit; the items are
`work/props/rimless-polar-cap-refuses-degenerateface.md` (issue 1250)
and `work/props/two-face-sphere-split-measures-zero-volume.md` (issue
1598) — read both in full; difficulty logged at spec: **H / NUMERIC**,
block PROPS-B3 slot 0, dual review). Read
`docs/prompts/implementer-discipline.md` in full. Branch
`props/sphere-pole-side`, cut from `main` (MESH-12 landed: the sphere
parse decides both halves of the span premise inside
`sphere_meridian_pole_margins`, and this unit builds on that head).

## The two defects are one gap

`fn sphere` (`crates/geom-brep/src/props/curved.rs:~1560`) measures a
sphere face as the chart rectangle `[u₀, u₁] × [v_lo, v_hi]`, with
`(v_lo, v_hi)` folded from the boundary's LEVELS (rim latitudes,
meridian endpoints, each meridian arc's span-derived pole extremes)
and the material side read by `linear_rim_side` from *which extreme a
rim sits at*. What the parse never reads is the one fact a rim's
TRAVERSAL encodes on its own: which side of the rim the face's
interior lies on. Two inputs break on exactly that silence:

1. **A rim-only polar cap** (issue 1250): one full rim circle, no
   meridian, the pole interior. The levels list holds the rim's
   latitude alone, `min_max` gives `lo == hi`, `require_extent`
   refuses `DegenerateFace` — for a face that is not degenerate (a
   ball cut by one plane; the die's pips). The missing extreme is the
   pole on the interior side of the rim, which the traversal knows.
2. **A half-cap and its complement sharing two edges** (issue 1598):
   a half rim `P→Q` and a pole-crossing meridian arc `Q→P`, traversed
   both ways by two faces. Both faces parse to levels `{v₀, +1}` and
   `Δu = π`; the complement is L-shaped in the chart (the other half-cap
   plus the whole band below the rim), not a rectangle, but the parse
   cannot tell: its rim sits at `lo` and its traversal says the
   interior lies BELOW the rim, and nothing compares the two. The
   fluxes cancel and a closed sphere measures `0.0`.

One mechanism serves the first and refuses the second: **a rim's
interior side is a fact of its traversal; the pole's membership is a
fact of the levels; the two must agree, and where the levels are
silent the traversal speaks.**

## The construction

For a sphere rim, define its **interior side** from stored data alone:
the sign `σ ∈ {+1, −1}` such that the face's interior lies toward
`+v` (the `+axis` pole) when `σ = +1`. With the outward normal radial
and the boundary traversed with the interior on the left, a rim
traversed in the `+u` direction has its interior toward `+v`, so
`σ = d_u_sign × (the face's sense as the module already applies it)`.
**Derive the convention from the existing sign machinery
(`rim_dir`, `linear_rim_side`, `boundary_material_sign`, `sense_sign`)
and pin it against the existing exact rows before using it** — CERT-1's
pole rows, the die's caps, `iso_rectangle_door.rs`, `mesh11_arc_branch.rs`:
on every face that measures exactly today, `σ` must agree with the
side `linear_rim_side` reads from the levels. That agreement is the
new predicate, `props_rim_interior_side`, decided per rim through the
funnel (a discrete sign product, definite by construction, like
`linear_rim_side`'s), and it is what item 2 fails.

Then, in `fn sphere`'s rim-bearing arm:

- **Levels silent** (no meridian; every level within the band of one
  rim latitude): push the pole on the interior side, `σ·1`, into the
  levels BEFORE `min_max`/`require_extent` — the cap's extent is
  `[v₀, +1]` or `[−1, v₀]`; `du_of_rims` already sums the full rim to
  `2π`; the closed form `R²·Δu·(sin v_hi − sin v_lo)` then measures the
  cap. `DegenerateFace` stays for a face whose rims all sit at one
  level with opposite traversals (the true zero-extent patch — say
  which row pins it).
- **Levels present**: after `linear_rim_side`, require every rim's
  `σ` to point INTO `[lo, hi]` from where the rim sits (a rim at `lo`
  needs `σ = +1`, at `hi` needs `σ = −1`); a rim whose interior side
  points out of the extent is a boundary the rectangle premise cannot
  hold — refuse `NotIsoRectangle { what: "props_rim_interior_side" }`.
  That is face B of item 2, and every L-shaped complement like it.

`boundary_material_sign` (tier 3's check 6) consumes the same parse;
say whether it needs the predicate too (it should: the same L-shaped
face must not be handed a definite sign), and pin it.

## Deliverables

- The interior-side sign, the predicate, and the two arms above; docs
  at `fn sphere` (`~1520-1560`) rewritten in the present tense — the
  "established / not established" list gains what the traversal
  establishes (discipline §4).
- **Red-first, item 1**: `crates/geom-brep/tests/r2_probe_sphere_polar.rs::probe_polar_cap_no_meridian`'s
  "full cap: REFUSE DegenerateFace" becomes the acceptance —
  `area = 2πR²(1 − sin v₀)` and the flux to 1e-12 for a cap on the
  `+axis` side and one on the `−axis` side, and a rim traversed the
  other way (the cap's complement, the ball minus the cap — ALSO a
  rim-only face, measured as `[−1, v₀]`); quote the red. Through the
  public doors: a ball cut by one plane (`topo`'s boolean or the die's
  pip cut — `crates/sweep/tests/m5_pr12_die.rs`, `m6_surgery.rs` have
  the cap fixtures), tier-3 volume equal to the spherical-cap closed
  form `πh²(3R − h)/3` to 1e-12 with the plane face; and **the flux
  lane answering where it refused** — find where a `DegenerateFace`
  from this face is caught and served by the certified-quadrature
  lane today (`topo::props::mass_properties`, the `pad` channel) and
  pin the flip: the pips measure through the closed form, `pad = 0`.
  Any golden, k-lint baseline or tess census that moves because a
  cap now measures exactly re-baselines with its digits and reason in
  the PR (discipline §3; METER's k-lint baselines are a seam — say
  what moved).
- **Red-first, item 2**: MESH-11's two rows pinning the defect —
  `crates/mesh/tests/mesh11_arc_branch.rs::mass_properties_still_answers_zero_on_the_half_cap`
  and the re-aimed `mesh7r1_probes` row — flip: face A (the half-cap)
  still measures exactly, face B refuses `NotIsoRectangle { what:
  "props_rim_interior_side" }`, and the closed sphere's
  `mass_properties` reports the refusal, not `0.0`; one row per
  traversal direction as the item asks. The three-face split of the
  same sphere still measures `4π/3` exactly. Quote both reds.
- **CERT-1's rows unmoved**: `a_pole_crossing_meridian_arc_measures_the_half_cap_exactly`,
  `the_rimless_hemisphere_split_off_its_poles_still_measures`,
  `a_split_vertex_a_hair_off_the_pole_still_certifies`, the near-polar
  block, and MESH-12's rows — green throughout; name them.
- **D2 addendum**: item 1 is row 2 served (the admission set grows by
  the rim-only cap; state it at `fn sphere`); item 2 turns a WRONG
  ANSWER into a typed refusal — classify it under the addendum
  (an admitted input that was answered wrongly is not row 1's
  "refused by cause"; say which row, or that the addendum lacks the
  row and file it).
- **The cone apex cap** (item 1's sibling): a cone face bounded by one
  rim with the apex interior. MEASURE it (through `fn cone` and tier
  3); if it refuses `DegenerateFace` by the same `lo == hi` path,
  serve it by the same mechanism only if it is the same three-line
  change on the cone's levels (the apex level from the rim's interior
  side); otherwise file it with the measurement.
- Sweep obligation (discipline §5): the shape is *a fact encoded in
  the loop's winding that a per-edge parse cannot see* — every
  `min_max(&b.levels)` consumer (`cylinder`, `cone`, `torus`, the
  sphere), read for a boundary whose levels are silent or whose
  traversal contradicts them; the cylinder/cone rim-only face is
  genuinely extent-less (say so); `boundary_material_sign`'s callers.

## Seams (announced by the orchestrator; list every edit under `## Seams crossed`)

`crates/mesh/tests/mesh11_arc_branch.rs` and `mesh7r1_probes.rs` are
MESH's rows (they flip — for this unit's reason only);
`crates/topo/src/props.rs` and tier 3's check 6 are TOPO's/BOOL's
(a call site, not a re-spell); `crates/sweep/tests/m5_pr12_die.rs`
and `m6_surgery.rs` are fixture reads only; k-lint/K-REPORT baselines
are METER's (re-baseline only, with digits).

## Posture

- ε posture: the new predicate is a discrete sign product, no new
  tolerance read; `require_extent`'s margin is unchanged; say so. No
  `CI-Config:` trailer (inert); no empty commits.
- Bit identity: every face that measured before measures the same
  bits (the closed form is untouched; only the levels list gains an
  entry on a face that was refused, and a refusal is added). Receipt:
  the existing exact rows and the `r2_bytes` digest — state which
  digest lines move (the die, if its caps now take the closed form)
  and why.
- Review: dual, in the experiment.
- **Landing: the items get `pr:` and `status: review`. DO NOT MERGE —
  the orchestrator lands after the dual and its fix pass; the fix pass
  closes both items, deletes this spec at merge with its
  `## Per-merge deletion` section in `docs/DOC-LEDGER.md`.** No
  `Co-Authored-By`; push early to `props/sphere-pole-side`.

## Acceptance

The rim-only cap measures exactly on both sides and both traversals,
through the public doors with `pad = 0`; the two-face sphere's
complement refuses typed and the closed sphere reports it; CERT-1's
and MESH-12's rows unmoved; the interior-side convention pinned
against every existing exact row; the cone sibling measured; every
moved golden re-baselined with its digits; hosted CI green on the
full matrix.
