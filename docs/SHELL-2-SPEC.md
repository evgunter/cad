# SHELL-2 — `transform_rigid` maps an `Approx` face

**Status: RATIFIED by the SHELL orchestrator (2026-09-04)** as a
faithful elaboration of the composition law geom-brep already pins
and of the never-trust-a-carried-certificate posture (O5, D4 ¶2).
Binds the implementer of unit `SHELL-2` (`work/shell/SHELL-2.md`,
branch `shell/2-transform-approx`); deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md`
in full first.

## 0. What this unit is

`crates/topo/src/transform.rs` maps every analytic surface and
refuses `Surface::Approx(_)` with `TransformError::ApproxSurface`
(`map_surface`, the `Approx` arm). The refusal's own text says why it
is not a limitation of the mathematics: the composition law
`M(S + d·n) = M(S) + d·n_M` holds for a rigid `M`, and
`crates/geom-brep/tests/approx_surface.rs::a_rigid_map_of_an_offset_is_the_offset_of_the_rigid_map`
pins it under certification (the two sup bounds agree to 1e-9). What
blocks the door is that re-deriving the mapped fit's two-limb
certificate is `f64` fit-door work (`geom_brep::certify_offset`,
`offset_fit.rs`), and `map_surface` is generic in `T` with no lane
to reach it.

This unit gives the door that lane. The mapped `Approx` surface is
the mapped base description, the mapped fit net, the same window and
tolerance, and a certificate **re-derived** on the mapped pair —
never the stored certificate, which is a claim about a different
geometry (`EdgeCurve::with_remapped_surfaces` is narrow for the same
reason).

## 1. The lane

Shape 2 of the issue, chosen for a measured reason: `transform_rigid`
has 57 caller files, several of them generic doors
(`topo::boolean::ops`, `editor-core`'s evaluator, `mesh::curved`,
`step-import`) whose own bounds would have to grow if the transform
door grew a NEW bound. So the method joins a lane the door already
binds:

```rust
// crates/geom-brep/src/pcurve_cache.rs (the trait's home) — or a
// sibling file if the trait is re-homed first; the trait, not the
// file, is the contract.
pub trait PcurveFittedLane: Decide {
    // …existing…

    /// The certificate of an offset description's fit, re-derived on
    /// the given (base, fit) pair — `None` when this scalar has no
    /// fit lane. `Some(Err)` is the fit door's typed refusal.
    fn remap_certificate(
        description: &geom::SurfaceDescription<Self>,
        fit: &NurbsSurface<Self>,
        window: geom::ApproxWindow,
        tolerance: f64,
        band: Band,
    ) -> Option<Result<geom::OffsetCertificate, OffsetFitError>>;
}
```

`f64` answers through `certify_offset` (as `recertify_approx` does);
`Probe`, `Interval` and `Dual<T>` answer `None` — explicit arms, no
default body, so a scalar added later has to say what it can do.
**Seam with M10-7 (PR #1725, open):** that PR adds `Sym<T>` impls in
`pcurve_cache.rs`; whichever merges second adds the arm (`Sym`
delegates to its base scalar like its siblings). Say in the PR body
which side you were on.

`map_surface`'s `Approx` arm then becomes: map the description
(`SurfaceDescription::Offset { base, d }` — the base net's control
points by the affine map, weights and knots untouched, `d`
unchanged), map the fit net the same way, and mint through
`ApproxSurface::certify` with the lane as the certifier. A `None`
lane refuses typed:

```rust
/// This scalar cannot re-derive an approximating surface's
/// certificate, and a certificate is never carried across a map.
ApproxLaneUnsupported { lane: &'static str },
```

— replacing `ApproxSurface`, whose text described the missing lane
rather than a scalar without one. A `Some(Err)` refuses
`ApproxRecertify { source: OffsetFitError }` with the fit door's
refusal verbatim.

**The NURBS net map** (`NurbsSurface<T>` under an `Affine3<T>`) needs
a library door; the composition test's `map_net` is a test helper.
Home: beside the surface type in `crates/geom` (the same place
`map_scalar` lives), as a rigid-only door whose docs say weights and
knots are invariants of an affine map. If a door already exists
under another name, use it and say so.

## 2. Out of scope, stated

- `Surface::Nurbs` / `Curve3::Nurbs` still refuse `NurbsPlaceholder`;
  only its MESSAGE changes, to be true (the evaluators exist; what is
  missing is the door's carrier re-certification for a NURBS carrier
  and the pcurve pass over a NURBS chart — name those, not "poison").
  Mapping them is TRIM's ground (`geom-brep/{edge_nurbs,nurbs_iso}.rs`).
- Nothing about `shell`, `offset`, or `replace_face`.
- No change to which scalars can BUILD an `Approx` face.

## 3. Acceptance (rows in `crates/topo/tests/` or beside the OFF-C
consumer suite `crates/sweep/tests/verbs_offc_consumer.rs`, whose
`prism_with_approx_walls` fixture is the body to move)

1. **The Approx-faced prism moves.** For both signs of `d`: the
   transformed body validates at tier 3 (which re-derives every
   `Approx` certificate itself — so validation is the independent
   check of the re-derivation), every `Approx` face is still
   `Approx` (kind preserved), its description's `d` and tolerance
   are unchanged, its window is unchanged, and the mapped
   certificate's `hull_sup` and `on_locus_max` agree with the
   original's to 1e-9 (a residual is a distance; a rigid map
   preserves it).
2. **Volume and area are rigid invariants**: mass properties before
   and after agree to the props lane's own certified width — read
   the enclosures, do not compare floats raw.
3. **Composition, at the body**: `offset(transform(prism))` versus
   `transform(offset(prism))` where the OFF-C door can build both —
   if it cannot (the per-chart door's oblique-corner refusal), say
   which and pin the surface-level law instead by reading the
   mapped face's description against a fresh fit.
4. **Every other scalar refuses typed**: at `Interval` (behind the
   `interval` feature — ask for `CI-Config: lane=interval` on the
   head commit and say in the PR which lane gated) the same call
   refuses `ApproxLaneUnsupported` naming the lane, BEFORE any
   surface has been written (`transform_rigid` clones and maps into
   `out`; a refusal must not have mutated the caller's body — it
   takes `&Body<T>`, so pin that `out` is dropped, not that the
   input changed).
5. **A non-rigid map still refuses `NotRigid` first**, on the
   `Approx` body too (the order of the gates is part of the door's
   contract).
6. **The pcurve pass**: `the_approx_face_mints_its_iso_caches`'s
   claim holds on the MAPPED body — every half-edge of a mapped
   `Approx` face carries a stored cache re-derived against the
   mapped surface (`crates/topo/src/pcurves.rs` already routes
   `Approx` charts; measure that the transform's pcurve pass
   reaches them, and STOP if it refuses — that is a finding about
   the pass, not this unit's fix).
7. **A degraded fit does not survive the map**: take
   `a_degraded_fit_on_a_face_goes_red_at_tier_three`'s corruption
   (a fit edited after certification) and transform the body — the
   door refuses `ApproxRecertify`, never ships the mapped
   degradation with a fresh-looking certificate.

## 4. Docs

`transform.rs` module docs: the "What maps how" list gains the
`Approx` line (description and fit mapped, certificate re-derived
through the scalar's lane, refused typed where none) and the NURBS
line is corrected. `docs/KERNEL-VERBS.md`: if it carries the
`Approx`-transform refusal as a row, retire the row.

## 5. Owed to neighbours (report, do not act)

- **TRIM** (`geom-brep/pcurve_cache.rs` is its file): one trait
  method and four impl arms; nothing else in the file moves.
- **M10-7** (#1725): the `Sym` arm seam, §1.
- **PERF** (`tier3-approx-regrid-per-face-cost`): tier-3 on the
  mapped body pays the per-face grid twice per row here (before and
  after); if the rows are slow, say how slow — it is that issue's
  measurement, not a reason to weaken the rows.

## 6. Stops

STOP and report if: the pcurve pass refuses a mapped `Approx` chart
(§3.6); `ApproxSurface::certify`'s certifier shape cannot take the
lane without a second constructor (report the shape you would need);
or the mapped certificate's sup bounds disagree with the original by
more than 1e-9 on the prism (that is the composition law failing at
the body, and it is not this unit's to explain away).

## 7. Lane rules

Own worktree, own `CARGO_TARGET_DIR` outside the checkout, never the
orchestrator's checkout, read other branches with `git show`. One
heavy cargo job at a time on this box. NO `Co-Authored-By` trailer in
lane commits (A/B blinding); if one lands in a pushed commit, note it
in the PR body and carry on. Push after every coherent step; hosted
CI is the gate; poll its run in the foreground.
