---
id: the-cylindrical-patch-rim-builder-is-written-nine-times
kind: issue
title: The two-rim cylindrical-patch builder is written five times in crates/topo/tests, all as closures a name census cannot see; the src spellings are a second construction
status: review
opened: 2026-09-19
branch: dup/cyl-rim-builder
refs: [topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one, census-cyl-sheet-b-keeps-the-unrepaired-radial-read, the-componentwise-scalar-lift-has-no-shared-home, the-fallible-wall-sheet-wrapper-is-written-twice, topo-cylinder-sheet-geomsources-are-asserted-by-nothing]
---


> **The count in this row's id is the opening one and it was wrong.**
> Re-taken 2026-09-19 at merge base `5b4979ef2`: the class is five, in
> four files, and the `src` spellings it counted are a different
> construction. The re-take, its instruments and the hit list are
> below; the id stays as it was minted (`work/README.md`: ids are
> stable).

## Finding, as opened

- **Where**: nine spellings of one builder, in seven files —
  `crates/topo/src/boolean/boxes.rs` (`rim` `:2285`, `meridian`
  `:3082`), `crates/topo/src/census.rs` (`rim` `:4023`, `rim` `:4531`),
  `crates/topo/src/chart_region.rs` (`rim` `:4371`, inside
  `fn cyl_sheet`), `crates/topo/tests/mate5_cyl_eps_rung.rs` (`rim`
  `:122`), `crates/topo/tests/r1_mate5_probe.rs` (`rim` `:75`),
  `crates/topo/tests/r2_probes.rs` (`rim` `:69`),
  `crates/topo/tests/split_edge_pcurve_rows.rs` (`rim` `:68`).
- **Importance**: medium — nine copies is the largest single count any
  S-DUP row has opened with
- **Confidence**: sure that the four `tests/` copies and the
  `census.rs`/`chart_region.rs` pair are the same construction (their
  closing `mef` blocks are token-identical); the two `boxes.rs`
  spellings were read, not diffed
- **Raised by**: the PR 2843 fix pass, 2026-09-19, out of the
  `find_half_edge(seed.face` sweep run for
  `the-cube-sequence-is-written-five-times-and-twice-inside-src`. Nine
  of that sweep's 26 hits are these. They are not cubes and not quad
  sheets, so they are their own class.

## The shape

A cylindrical patch grown as two rims and a closing wall: a `rim`
closure that seeds or walks a circular arc at one parameter, a second
rim at the other, and then

```
let he = body.find_half_edge(seed.face, e_t.vertex, e_r.vertex).unwrap();
body.mef(
    MefSite::Chords { he1: he, he2: e_b.he_plus },
    EdgeCurveSpec::line_between(p01, p00),
    FaceSurface::Shared(cyl),
    ...,
)
```

That `mef` block is identical, token for token, across
`mate5_cyl_eps_rung.rs`, `r1_mate5_probe.rs`, `r2_probes.rs`,
`split_edge_pcurve_rows.rs`, `census.rs` (both) and
`chart_region.rs` — seven of the nine — differing only in
`Tol::witness()` against a local `tol()`.

## Why no census in links 1–3 saw it

**Every one of the nine is a CLOSURE**, `let rim = |body, v, ccw| { … }`,
except `chart_region.rs`'s, which is a `fn cyl_sheet`. So:

- the **name census** keys on declarations, and a closure has no
  declaration to key on;
- the **arity census** counts call sites and these loop, which is the
  blind spot that row already names;
- the **geometry census** (`mvfs(` with `newell_plane`) misses them
  because the surface here is a cylinder, not a Newell plane.

The instrument that finds them is the structural one —
`git grep -n 'find_half_edge(seed.face'` — and this row is the second
piece of evidence that a structural needle catches what three
name/arity/import censuses do not.

## The census, re-taken 2026-09-19 at merge base `5b4979ef2`

Four instruments, every tracked file, **no path argument** (method
item 3). The number above is wrong in both directions: the CLASS —
one construction, one home — is **five**, not nine, and it includes
one member the row never named.

| # | instrument | what it cannot see |
| --- | --- | --- |
| 1 | structural needle, `git grep -n 'find_half_edge(seed.face'` (24 code hits) | a member that closes its loop by any other route — a `MefSite` other than `Chords`, a `mekr`, or a half-edge taken off a loop walk — and one whose seed binding is not spelled `seed` |
| 2 | closure/name census, `let rim =` \| `let meridian =` \| `fn cyl_sheet` \| `fn wall_sheet` (≈200 hits, mostly noise) | a copy whose closure is called anything else — structurally, always; here it needed a second filter to be readable at all |
| 3 | surface-key census, `git grep -n 'FaceSurface::Shared(cyl'` (10 hits) | a member that binds the key to another name. **Demonstrated**: it misses `boxes.rs`'s two torus patches, which say `Shared(torus)` |
| 4 | denominator-first, every `FaceSurface::Shared(` in every tracked file (46 hits, 25 files) | a patch that mints a fresh surface per face instead of sharing the seed's key |

Instrument 3 is what found the member no census in the row saw, and
instrument 4 is what closed the SCOPE: of the 46 `Shared(` sites,
every one outside `crates/topo` is a plane/sphere/nappe fixture or a
production sweep path, so the class really is confined to `crates/topo`
— re-derived rather than inherited.

## The hit list, and what became of each

**Folded onto one door** — `topo::test_support::cyl_wall_sheet`, in
`crates/topo/src/test_support_fixtures.rs`:

1. `crates/topo/tests/mate5_cyl_eps_rung.rs` `wall_sheet`
2. `crates/topo/tests/r1_mate5_probe.rs` `wall_sheet`
3. `crates/topo/tests/r2_probes.rs` `wall_sheet`
4. `crates/topo/tests/split_edge_pcurve_rows.rs` `wall`
5. `crates/topo/tests/mate5_cyl_eps_rung.rs` `interval_lane::wall` —
   **not in this row's nine**, `#[cfg(feature = "interval")]`, so the
   compiler never type-checked it and no census that read a default
   build could reach it

1–3 were token-identical (1939 bytes each after comment and
whitespace normalisation, one md5). 4 is the same construction with
the body owned and no source. 5 is the same construction at
`Interval`. Three copies of `struct CylFrame` + its `impl`, one
`struct Frame`, and three copies of `fn frame_a` went with them.

**Measured and dispositioned out, each a result:**

6. `crates/topo/src/census.rs` `cyl_sheet` and 7.
   `crates/topo/src/chart_region.rs` `cyl_sheet` — one construction
   written twice (dumps diff empty), and **not** the one the `tests/`
   family builds. Row:
   `topo-src-cyl-sheet-is-one-construction-twice-and-not-the-tests-one`.
8. `crates/topo/src/census.rs` `cyl_sheet_b` — same shape, reparameterised
   frame, and it keeps the pre-repair projection read. Row:
   `census-cyl-sheet-b-keeps-the-unrepaired-radial-read`.
9. `crates/topo/src/boolean/boxes.rs` `revolved_wall` (this row's
   `rim` at `:2285`) — a GENERALISATION: radius is a profile `rho(z)`
   and the rim returns its own surface, which is what lets `cyl_wall`
   and `cone_wall` share it. Not a copy; folding it onto the door would
   lose the cone.
10. `crates/topo/src/boolean/boxes.rs` `torus_wall` (this row's
    `meridian` at `:3082`) — the row guessed "may be a genuine
    sibling". Measured, it is more than that: a **torus** patch with
    four CURVED sides, closing on a curved meridian spec where every
    member of this class closes on `EdgeCurveSpec::line_between`. A
    different class, sharing only the Euler skeleton.
11. `crates/topo/src/boolean/boxes.rs` `lone_circle_annulus` — a torus
    annulus on one meridian circle. Different class.
12. `crates/topo/tests/loop_reparenting_pcurve_rows.rs` `rim_back` /
    `rim_fwd` / `sheet` — **not in this row's nine**; a three-face
    sheet split at a mid rim, whose rims are described
    `EdgeDescriptionSpec::chart(cyl)` rather than as an
    `Intersection`. Not this construction.
13. `boxes.rs:2202`, `chart_region.rs:4203`/`:4916`,
    `chart_region_r2_probes.rs:93`, `tests/box_with_hole.rs`,
    `cube_by_hand.rs`, `m3_pr3_split.rs`, `review_m3_pr3_rings.rs` —
    the `e_cd`/`e_bc` cube and quad-sheet spellings, already owned by
    `the-cube-sequence-is-written-five-times-and-twice-inside-src` and
    `the-quad-sheet-helper-is-written-three-times-across-two-chart-region-files`.

## What was unmeasured, measured

**1. Do they build bit-identical bodies at equal arguments?** Dumped,
not read: each spelling was reproduced verbatim in a throwaway in-crate
probe, run at `u ∈ [0.2, 1.4] × v ∈ [0, 1]` on the canonical unit
cylinder, and every vertex, edge, half-edge, loop, face, shell, solid,
point, curve, surface, **surface source** and pcurve row was formatted
with its key, sorted and diffed.

- `tests/` members 1–3 against 4: **identical on all 45 dump lines but
  one** — the cylinder key's `GeomSource`, which 4 does not set. That
  is the whole difference, and it is now the door's `source` parameter.
- `src` members 6 against 7: **empty diff**, `set_face_sense(face,
  true)` included.
- `tests/` family against `src` family: **not identical, and not
  close** — 3 solids / 4 faces / 6 vertices against 1 / 2 / 4. The
  `tests/` members mint a scaffold `mvfs` per rim because
  `Body::add_surface` is `pub(crate)` and a `tests/` binary cannot
  reach it. **The two constructions diverged along the crate boundary,
  not along intent**, and that is the finding: merging them changes
  what `census.rs`'s solid-and-face rows measure, so it is a unit of
  its own rather than a step in this one.

**2. Where the shared home goes.** `crates/topo`'s existing
`test_support_fixtures.rs`, behind `topo::test_support`. **It cost
nothing in the manifest**: the module is already
`#[cfg(any(test, feature = "test-support"))]`, the `test-support`
feature already exists, and `crates/topo/Cargo.toml`'s self
dev-dependency already turns it on for exactly this crate's own tests.
No feature is added, so `scripts/gates/test-features-dev-only.sh` has
nothing new to read — the precedent
`sweep-test-support-brick-is-still-a-second-box-construction` (PR
#2877) measured is that the gate refuses a test-only feature on a
`[dependencies]` edge, and there is no new edge here. The fold moves
the `tests/` half only; the `src` half stays where it is for the reason
above, so this is NOT the `mesh`-style two-sided fold the row expected.

One thing the home did cost: `crates/topo/src/review_m1_pr5_internal.rs`'s
two door tables read every `pub fn … &mut …` under `topo/src`, so a
fixture builder moved into `src` becomes a mutation door and must
declare how tier 1 survives it. It reds until it does — which is a
live probe the fold did not have to plant.

**3. Must callers supply the surface?** No. `FaceSurface::Shared(cyl)`
re-uses the key the builder itself minted on the seed face, so the
caller supplies the **frame**, not the surface, and the door mints from
it. The `prism_ops`/`FaceGeometry` precedent does not apply: face
geometry varies between prism call sites and the cylinder does not
between sheet call sites. What the `src` pair takes as an
`Option<SurfaceKey>` is a different question — *sharing one key between
two sheets* — and it belongs with row 6/7.

## What the fold left standing, on purpose

`try_wall_sheet` — a `catch_unwind` stand-down wrapper — stays
token-identical in `r1_mate5_probe.rs` and `r2_probes.rs`; both now
wrap the one door, which is what turns it from two builders into one
duplication. Row:
`the-fallible-wall-sheet-wrapper-is-written-twice`.

## The proof

A mutation that reverses nothing — the descending rim's carrier axis
left unnegated — reds **23 rows across all four folded suites**
(`mate5_cyl_eps_rung` 9 including `interval_lane`, `r1_mate5_probe` 7,
`r2_probes` 4, `split_edge_pcurve_rows` 3), 594 of 617 passing. Every
folded site is live, the `interval` member included.

A second mutation is the finding: **deleting the source write left all
617 rows green**, at both lanes, although three suites document the
distinct `GeomSource` as their subject. Row:
`topo-cylinder-sheet-geomsources-are-asserted-by-nothing`.
