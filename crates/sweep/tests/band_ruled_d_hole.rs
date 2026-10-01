//! **A D-shaped through-hole's creases** — the ruled band on a crease
//! whose two ends sit in the caps' RINGS rather than their outer
//! cycles.
//!
//! The fixture is one extrude of two loops: a square outer loop and a D
//! ring (the chord `x = flat`, the major arc of radius [`ROD_R`] about
//! the origin). The hole's two ruling edges are concave cylinder–plane
//! creases, each ending at a vertex of the D ring on each cap, so the
//! crease's cap rims — the chord and the arc on that cap — are ring
//! edges of the cap face.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::fillet_edges;
use sweep::test_support::{
    ROD_FILLET, ROD_L, ROD_R, assert_naming_totality, rod_chord_at, rod_creases, rod_section_cut,
};
use sweep::{Extrusion, extrude};
use topo::{Body, mass_properties, validate_geometric};

fn tol() -> Tol {
    Tol::witness()
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    assert_eq!(p.volume_pad, 0.0, "the inventory is closed-form");
    p.volume
}

fn square(h: f64) -> ProfileLoop<f64> {
    bulge_loop(
        [(-h, -h), (h, -h), (h, h), (-h, h)]
            .into_iter()
            .map(|(x, y)| (Point2::new(x, y), 0.0))
            .collect(),
    )
}

/// The block `[−1, 1]² × [0, ROD_L]` with a D-shaped hole through it:
/// the rod's D profile at `flat`, as a ring of the profile.
fn d_hole_block(flat: f64) -> Body<f64> {
    let c = rod_chord_at(flat);
    let d = bulge_loop(vec![
        (Point2::new(flat, c.half), c.wall_bulge),
        (Point2::new(flat, -c.half), 0.0),
    ]);
    let p = Profile::new(SketchPlane::xy(), vec![square(1.0), d])
        .validate(tol())
        .expect("the D-holed profile validates");
    extrude(&p, Extrusion::Distance(ROD_L), tol())
        .expect("the D-holed profile extrudes")
        .body
}

/// **The fixture holds together**, and its creases' ends sit in the
/// caps' rings: each cap carries exactly one ring, and every crease end
/// is a vertex of it, not of the cap's outer cycle.
#[test]
fn the_d_hole_block_is_valid_and_its_creases_end_in_the_caps_rings() {
    let body = d_hole_block(0.3);
    validate_geometric(&body, tol()).expect("the D-holed block is tier-3 valid");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the D's two ruling corners");
    let rings: Vec<_> = body
        .faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .flat_map(|(_, f)| f.rings.clone())
        .collect();
    assert_eq!(
        rings.len(),
        2,
        "both caps, and only they, carry the D once each"
    );
    for &crease in &creases {
        let e = body.get_edge(crease).unwrap();
        for he in [e.he_plus, e.he_minus] {
            let v = body.get_half_edge(he).unwrap().start;
            // The crease's own halves lie in the supports; the cap's
            // half-edges out of `v` are its rims.
            let in_a_ring = body
                .half_edges()
                .any(|(_, h)| h.start == v && rings.contains(&h.parent_loop));
            assert!(in_a_ring, "crease {crease:?}'s end {v:?} is a ring vertex");
        }
    }
}

/// **Both creases of the D hole carve**: two bands, the census delta of
/// two cut-off bands, tier 3, naming totality — and `ΔV = +2·A·L`, the
/// concave band adding back exactly what the same ball rolling inside
/// the D-rod removes (the hole's void is the rod's material). At the
/// rod's radius, at a flat past the axis, and at `R/r = 2` — the
/// concave near-osculating family, which this crease was the ring case
/// of.
#[test]
fn a_d_hole_fillets_both_creases_at_the_rod_closed_form() {
    for (flat, r) in [(0.3, ROD_FILLET), (-0.2, ROD_FILLET), (0.3, ROD_R / 2.0)] {
        let source = d_hole_block(flat);
        let what = format!("D hole at flat {flat}, r {r}");
        let creases = rod_creases(&source);
        let (v0, e0, f0) = census(&source);
        let vol0 = volume(&source);
        let out = fillet_edges(&source, &creases, r, tol())
            .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
        assert_eq!(out.blend_faces.len(), 2, "{what}: one band per crease");
        assert!(out.corner_faces.is_empty() && out.band_faces.is_empty());
        assert_eq!(
            census(&out.body),
            (v0 + 4, e0 + 6, f0 + 2),
            "{what}: the census delta of two cut-off bands"
        );
        validate_geometric(&out.body, tol())
            .unwrap_or_else(|e| panic!("{what}: tier 3, got {e:?}"));
        assert_naming_totality(&source, &out, &creases, &what);
        // Each cap is cut off IN its ring: the ring is the chord's and
        // the wall arc's surviving middles plus the two cut-off arcs, the
        // latter circles of the band's radius, and the cap's outer
        // square is untouched.
        let caps: Vec<_> = out
            .body
            .faces()
            .filter(|(_, f)| !f.rings.is_empty())
            .map(|(_, f)| (f.outer, f.rings.clone()))
            .collect();
        assert_eq!(
            caps.len(),
            2,
            "{what}: both caps keep the rounded D as a ring"
        );
        for (outer, rings) in caps {
            assert_eq!(rings.len(), 1, "{what}: one ring per cap");
            let edges_of = |lp| -> Vec<topo::EdgeKey> {
                out.body
                    .half_edges()
                    .filter(|(_, h)| h.parent_loop == lp)
                    .map(|(_, h)| h.edge)
                    .collect()
            };
            assert_eq!(
                edges_of(outer).len(),
                4,
                "{what}: the cap's square is untouched"
            );
            let ring = edges_of(rings[0]);
            let radii: Vec<Option<f64>> = ring
                .iter()
                .map(|&k| {
                    let e = out.body.get_edge(k).unwrap();
                    match *out.body.get_curve_geom(e.curve)?.certified()?.carrier() {
                        geom::Curve3::Circle { radius, .. } => Some(radius),
                        _ => None,
                    }
                })
                .collect();
            let cut_offs = radii
                .iter()
                .filter(|x| x.is_some_and(|rr| (rr - r).abs() < 1e-12))
                .count();
            assert_eq!(
                (ring.len(), cut_offs),
                (4, 2),
                "{what}: chord, wall arc and two cut-off arcs of radius r, got {radii:?}"
            );
        }
        let dv = volume(&out.body) - vol0;
        let a = rod_section_cut(ROD_R, flat, r);
        assert!(
            (dv - 2.0 * a * ROD_L).abs() < 1e-12,
            "{what}: ΔV = +2·A·L, measured {dv} vs {}",
            2.0 * a * ROD_L
        );
    }
}
