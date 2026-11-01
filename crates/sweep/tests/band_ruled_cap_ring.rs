//! **A cap edge beside a convex ruled crease's cut-off.** The ruled
//! band removes, from each transverse cap, the SLIVER between the
//! cut-off arc (radius `r` about the spine's crossing `c`) and the
//! crease's old vertex `V`. An edge of the cap lying in that sliver — a
//! bore's ring, or a notch in the cycle the cut runs in — would be left
//! on the cap outside the region it bounds, or crossed by the arc, so
//! every edge of the cap other than the two rims the cut shortens is
//! metered, over its own window, against a region enclosing the sliver
//! before any carve: `r ≤ ‖p − c‖ ≤ ρ` (`ρ` the farthest the sliver
//! reaches from `c`, which on this rod is `‖V − c‖`), cut down to the
//! half-plane `(p − c)·(V − c) ≥ floor` the sliver lies in. A cycle
//! not clear of that region is read against the sliver itself: crossing
//! it (`RingClearance`, a measurement), wholly inside it, or neither
//! certified. The last two join predicate 2's verdict: a bore whose
//! rings read inside at both caps, and whose walls read inside the prism
//! the band removes over them, goes with the sliver, its faces, edges
//! and vertices taken by the carve. The cap's inside reading and its
//! depth are pinned at unit level too (`blend::open::end_face`).
//!
//! The fixture is the D-rod (`test_support::rod_d_profile_at`'s D at
//! [`ROD_FLAT`]) with a second profile loop through it, or a channel in
//! its outline. Its upper crease's ball centre is `c = (0.2, √0.12)`
//! and its old vertex `V = (0.3, 0.4)`, so at [`ROD_FILLET`] the
//! annulus is `0.1 ≤ ‖p − c‖ ≤ ‖V − c‖ ≈ 0.1134`.
//!
//! The keyhole twin — a bore beside a cut-off that runs in a cap RING —
//! is `review_band_ruled_ring_probes`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::band_planar_consume::poly;
use crate::common::interval::iv;
use geom_core::{Bounds, Decide, Interval, Point2, Real, Sign, Tol};
use profile::{ProfileLoop, SketchPlane, test_support::bulge_loop};
use sweep::blend::{BlendError, Convexity, fillet_edges};
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, extruded, rod_chord_at, rod_creases, rod_section_cut,
};
use topo::{AtRestPolicy, Body, ContactRecords, validate_geometric, validate_pseudomanifold};

fn tol() -> Tol {
    Tol::witness()
}

fn volume(body: &Body<f64>) -> f64 {
    topo::mass_properties(
        &topo::test_support::finished("the measured body", topo::Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .expect("closed-form props")
    .met()
    .expect("closed-form props")
    .volume
}

/// A round profile loop: two semicircles about `(x, y)`.
fn bore(x: f64, y: f64, a: f64) -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(x + a, y), 1.0),
        (Point2::new(x - a, y), 1.0),
    ])
}

/// [`bore`] at any scalar.
fn round_loop<T: Real>(x: f64, y: f64, a: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    bulge_loop(vec![
        (Point2::new(f(x + a), f(y)), f(1.0)),
        (Point2::new(f(x - a), f(y)), f(1.0)),
    ])
}

/// The D-rod with one round bore through it.
fn bored_d_rod(x: f64, y: f64, a: f64) -> Body<f64> {
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ]);
    extruded(SketchPlane::xy(), vec![d, bore(x, y, a)], ROD_L, tol())
}

/// The upper crease's ball centre `c` and old vertex `V` in the cap.
fn upper_corner(r: f64) -> ((f64, f64), (f64, f64)) {
    let cx = ROD_FLAT - r;
    let c = (cx, ((ROD_R - r).powi(2) - cx * cx).sqrt());
    let v = (ROD_FLAT, (ROD_R.powi(2) - ROD_FLAT.powi(2)).sqrt());
    (c, v)
}

/// The refusal a bore in (or across) the sliver owes: `RingClearance`
/// at a cap face, definitely negative.
fn assert_cap_ring_refusal(body: &Body<f64>, what: &str) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the D's two creases");
    match fillet_edges(
        &sweep::test_support::at_rest(body, tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .map_err(|e| {
        let text = e.error.to_string();
        (e.error, text)
    }) {
        Err((
            BlendError::RingClearance {
                face,
                chain,
                margin,
                bounded: false,
            },
            text,
        )) => {
            assert_eq!(
                chain,
                Convexity::Convex,
                "a cap meter refuses only beside a convex crease"
            );
            assert!(
                text.contains(
                    "lies in the part of a face the blend cuts away with the material it removes"
                ),
                "the sentence says the edge is cut away with the sliver: {text}"
            );
            assert_eq!(
                margin.sign,
                Sign::Negative,
                "{what}: definite, got {margin}"
            );
            let f = body
                .get_face(face)
                .expect("the refusal names a source face");
            assert!(
                !f.rings.is_empty(),
                "{what}: the face named is a cap carrying the bore"
            );
        }
        Err((other, _)) => panic!("{what}: expected RingClearance at the cap, got {other:?}"),
        Ok(out) => panic!(
            "{what}: the carve returned a body keeping the bore's cycle on a cap that no \
             longer covers it ({} faces)",
            out.body.faces().count()
        ),
    }
}

/// **A bore wholly inside the sliver goes with it**: its rings read
/// inside at the caps and its walls inside the prism over them, so the
/// carve takes its faces, edges and vertices with the material the band
/// removes — `ΔV = −2·A·L + π·a²·L`, at tiers 3 and 3′, every output
/// entity a recorded mint or a survivor, and the bore's walls retired.
fn assert_consumed(body: &Body<f64>, a: f64, what: &str) {
    let body = sweep::test_support::finished(what, body.clone(), tol());
    validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    let creases = rod_creases(&body);
    let walls: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - a).abs() < 1e-12
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert!(
        !walls.is_empty(),
        "{what}: the source carries the bore's walls"
    );
    let out = fillet_edges(&body, &creases, ROD_FILLET, tol())
        .unwrap_or_else(|e| panic!("{what}: the bore goes with the sliver, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′, {e:?}"));
    sweep::test_support::assert_naming_totality(&body, &out, &creases, what);
    let rec = out.naming.as_ref().expect("the surgery records its births");
    for w in &walls {
        assert!(
            out.body.get_face(*w).is_none() && rec.dead.faces.contains(w),
            "{what}: the bore's wall {w:?} is consumed and retired"
        );
    }
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "{what}: no cap keeps a ring"
    );
    let dv = volume(&out.body) - volume(&body);
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L
        + core::f64::consts::PI * a * a * ROD_L;
    assert!((dv - want).abs() < 1e-12, "{what}: ΔV {dv} vs {want}");
}

/// **A bore wholly inside the removed sliver goes with it at the
/// closed form** ([`assert_consumed`]). The bore at `(0.297, 0.395)`
/// is `0.003` from the flat, `0.0033` from the wall and `≈ 0.1085` from
/// `c`.
#[test]
fn a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_the_closed_form() {
    assert_consumed(
        &bored_d_rod(0.297, 0.395, 0.002),
        0.002,
        "bore in the sliver",
    );
}

/// **The same bore at the certified scalar** goes with the sliver too:
/// tiers 3 and 3′, both half-walls retired, no cap keeping a ring, and
/// the faces of the unbored rod's carve.
#[test]
fn a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval() {
    // Pinned red since PR 5227: at ε = 1e-12 the curve-pair polish meters its
    // iterate's accumulated width and refuses the cap ring's meeting
    // (`work/cpair/curve-pair-polish-at-interval-reads-the-iterates-width`).
    test_utils::pinned_red::row(
        geom_core::Tol::witness().eps(),
        &[1e-12],
        "the bore at Interval",
        &[
            "Interval: tier 3′, [CensusUnsupported { subject: Entity(Edge(EdgeKey(1v3))), cause: ContactLane(NotCertifiable { what: \"the meeting reading of two curved edges could not settle a count\" }) }",
        ],
        a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval_row,
    );
}

/// The row [`a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval`] runs, under its pin.
fn a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval_row() {
    let tol = tol();
    let body = sweep::test_support::finished(
        "bored",
        extruded(
            SketchPlane::xy(),
            vec![d_loop(), round_loop(0.297, 0.395, 0.002)],
            Interval::from_f64(ROD_L),
            tol,
        ),
        tol,
    );
    let out = fillet_edges(&body, &rod_creases(&body), iv(ROD_FILLET), tol)
        .unwrap_or_else(|e| panic!("the bore goes with the sliver at Interval, got {e}"));
    validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("Interval: tier 3, {e:?}"));
    validate_pseudomanifold(&out.body, &ContactRecords::default(), tol)
        .unwrap_or_else(|e| panic!("Interval: tier 3′, {e:?}"));
    let rec = out.naming.as_ref().expect("the surgery records its births");
    let walls: Vec<topo::FaceKey> = body
        .faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { .. })
            )
        })
        .map(|(k, _)| k)
        .filter(|k| out.body.get_face(*k).is_none())
        .collect();
    assert_eq!(
        (walls.len(), rec.dead.faces.len()),
        (2, 2),
        "Interval: the bore's two half-walls go, and two faces are retired"
    );
    assert!(
        walls.iter().all(|w| rec.dead.faces.contains(w)),
        "Interval: the retired faces are the bore's walls"
    );
    assert!(
        out.body.faces().all(|(_, f)| f.rings.is_empty()),
        "Interval: no cap keeps a ring"
    );
    let unbored = sweep::test_support::finished(
        "unbored",
        extruded(
            SketchPlane::xy(),
            vec![d_loop()],
            Interval::from_f64(ROD_L),
            tol,
        ),
        tol,
    );
    let plain = fillet_edges(&unbored, &rod_creases(&unbored), iv(ROD_FILLET), tol)
        .unwrap_or_else(|e| panic!("the unbored rod carves at Interval, got {e}"));
    let census = |b: &Body<Interval>| (b.faces().count(), b.edges().count(), b.vertices().count());
    assert_eq!(
        census(&out.body),
        census(&plain.body),
        "at Interval the bored rod's carve is the unbored rod's"
    );
}

/// A circle of radius `a` about `(x, y)` as ONE full-turn arc, its
/// vertex at azimuth `az`: a bore whose rings are one edge each and
/// whose wall is one face, closed by one seam line.
fn one_edge_circle(x: f64, y: f64, a: f64, az: f64) -> ProfileLoop<f64> {
    profile::RawLoop::new([(
        Point2::new(x + a * az.cos(), y + a * az.sin()),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(x, y),
            radius: a,
            sweep: core::f64::consts::TAU,
        }),
    )])
}

/// **A one-edge bore goes with the sliver** ([`assert_consumed`]) at any
/// seam azimuth: its wall is one face whose one cycle runs both rings
/// and its seam twice, so the feature is an annulus of one face and the
/// carve takes it as it takes the two-arc bore.
#[test]
fn a_one_edge_bore_inside_the_sliver_goes_with_it_at_any_seam_azimuth() {
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
    ]);
    let plain = extruded(SketchPlane::xy(), vec![d.clone()], ROD_L, tol());
    for az in [0.0, 1.3, 3.0, 4.9] {
        let what = format!("one-edge bore, seam at {az}");
        let body = extruded(
            SketchPlane::xy(),
            vec![d.clone(), one_edge_circle(0.297, 0.395, 0.002, az)],
            ROD_L,
            tol(),
        );
        assert_eq!(
            (
                body.faces().count() - plain.faces().count(),
                body.edges().count() - plain.edges().count()
            ),
            (1, 3),
            "{what}: one wall face; two one-edge rings and a seam"
        );
        assert_consumed(&body, 0.002, &what);
    }
}

/// The D-rod less a round rod of radius `a` whose axis runs from `p` in
/// the cap at `z = 0` to `q` in the cap at `z = L`, past both.
fn tilted_bored_d_rod(p: (f64, f64), q: (f64, f64), a: f64) -> Body<f64> {
    let d = geom_core::Vec3::new(q.0 - p.0, q.1 - p.1, ROD_L);
    let dh = d.normalize();
    let over = 0.05 / dh.z;
    let o = geom_core::Point3::new(p.0, p.1, 0.0) - dh * over;
    let u = (geom_core::Vec3::new(1.0, 0.0, 0.0) - dh * dh.x).normalize();
    let plane = sweep::test_support::sketch_from_axes(o, u, dh.cross(u), tol());
    let rod = extruded(plane, vec![bore(0.0, 0.0, a)], d.norm() + 2.0 * over, tol());
    crate::common::cavity::cut(
        "tilted bore",
        &extruded(SketchPlane::xy(), vec![d_loop()], ROD_L, tol()),
        &rod,
    )
}

/// **A tilted bore inside the sliver at both caps that runs through
/// kept material between them refuses**, as a bound at its wall. Both
/// rings read inside, but the fillet arc bulges into the sliver, so the
/// prism over them is not the band's material. These rows hold
/// `prism_inside`'s line slack and its `diagonal·|a × τ|` term: without
/// them each bore is consumed.
///
/// The fixture's Boolean cuts each cap in an ellipse whose axes differ by
/// `a·(|d|/L − 1)`. Where that is within the tolerance band's escalate
/// width, the cut cannot tell the axes apart and escalates, so the row
/// stands down at that ε.
fn assert_tilted_bore_refuses(what: &str, p: (f64, f64), q: (f64, f64), a: f64) {
    let run = (q.0 - p.0).hypot(q.1 - p.1);
    let axes_apart = a * (run.hypot(ROD_L) / ROD_L - 1.0);
    let escalate = geom_core::Band::linear(tol())
        .expect("the witness band")
        .escalate();
    if axes_apart < escalate {
        test_utils::vacuity::stood_down(
            what,
            &format!(
                "the cap ellipses' axes differ by {axes_apart:e}, inside the band's escalate \
                 width {escalate:e}: the Boolean that cuts the bore escalates"
            ),
        );
        return;
    }
    let body = sweep::test_support::finished(what, tilted_bored_d_rod(p, q, a), tol());
    let got = fillet_edges(&body, &rod_creases(&body), ROD_FILLET, tol()).map(|_| ());
    assert!(
        matches!(
            got,
            Err(ref e) if matches!(e.error, BlendError::FaceClearance { bounded: true, .. })
        ),
        "{what}: refuses as a bound at the wall, got {:?}",
        got.map_err(|e| e.error)
    );
}

/// [`assert_tilted_bore_refuses`]: each ring `≈ 8e-4` inside the sliver,
/// the axis's midpoint `≈ 4.2e-3` inside the kept ball.
#[test]
fn a_tilted_bore_inside_at_both_caps_through_the_ball_mid_length_refuses() {
    assert_tilted_bore_refuses(
        "through the ball mid-length",
        (0.29918, 0.36551),
        (0.26784, 0.42123),
        5e-4,
    );
}

/// [`assert_tilted_bore_refuses`]: a thinner, steeper bore whose middle
/// runs `≈ 5.8e-3` into the kept ball.
#[test]
fn a_thin_steep_bore_inside_at_both_caps_through_the_ball_between_refuses() {
    assert_tilted_bore_refuses("thin and steep", (0.2993, 0.3620), (0.2647, 0.4234), 3e-4);
}

/// **A blind bore wholly inside the sliver refuses**, as a bound at its
/// wall: its floor bounds the wall by an edge that is neither a cap
/// cycle's nor a line between two, so the prism reading does not take
/// it, and the carve takes only a through-feature between the caps.
#[test]
fn a_blind_bore_inside_the_sliver_refuses_as_a_bound() {
    let tool = crate::common::cavity::rod(Point2::new(0.297, 0.395), 0.002, -0.5, 0.5);
    let body = crate::common::cavity::cut(
        "blind bore",
        &extruded(SketchPlane::xy(), vec![d_loop()], ROD_L, tol()),
        &tool,
    );
    let body = sweep::test_support::finished("blind bore", body, tol());
    let got = fillet_edges(&body, &rod_creases(&body), ROD_FILLET, tol()).map(|_| ());
    assert!(
        matches!(
            got,
            Err(ref e) if matches!(e.error, BlendError::FaceClearance { bounded: true, .. })
        ),
        "a blind bore refuses as a bound, got {:?}",
        got.map_err(|e| e.error)
    );
}

/// **A hair either side of the arc and a hair off the flat.** On the
/// ray from `c` towards `V`, a bore of radius `0.004` whose near side is
/// `1e-4` short of the arc crosses it, and refuses as a measurement at
/// the cap; one `1e-4` past it, and one `1e-4` off the flat inside the
/// sliver, lie wholly inside and go with it.
#[test]
fn a_bore_a_hair_across_the_slivers_boundary_crosses_and_one_inside_it_goes_with_it() {
    let (c, v) = upper_corner(ROD_FILLET);
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let (a, hair) = (0.004, 1e-4);
    let on_ray = |d: f64| (c.0 + d * dx / n, c.1 + d * dy / n);
    let (x, y) = on_ray(ROD_FILLET + a - hair);
    assert_cap_ring_refusal(&bored_d_rod(x, y, a), "a hair short of the arc");
    let (x, y) = on_ray(ROD_FILLET + a + hair);
    assert_consumed(&bored_d_rod(x, y, a), a, "a hair past the arc");
    assert_consumed(
        &bored_d_rod(ROD_FLAT - 0.002 - hair, 0.395, 0.002),
        0.002,
        "a hair off the flat",
    );
}

/// **A cap carrying a bore inside the sliver and a bore across its arc
/// refuses as the crossing, in either order** the profile lists them:
/// a crossing anywhere outranks the inside bore, which does not go.
#[test]
fn a_cap_with_an_inside_bore_and_a_crossing_bore_crosses_in_either_order() {
    let (c, v) = upper_corner(ROD_FILLET);
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let across = (c.0 + ROD_FILLET * dx / n, c.1 + ROD_FILLET * dy / n, 0.003);
    let inside = (0.297, 0.395, 0.002);
    for (what, [p, q]) in [
        ("inside first", [inside, across]),
        ("across first", [across, inside]),
    ] {
        let body = extruded(
            SketchPlane::xy(),
            vec![d_loop(), bore(p.0, p.1, p.2), bore(q.0, q.1, q.2)],
            ROD_L,
            tol(),
        );
        assert_cap_ring_refusal(&body, what);
    }
}

/// **A bore straddling the cut-off arc refuses**: centred ON the arc,
/// on the ray from `c` towards `V`, so it is part removed and part kept.
#[test]
fn a_bore_across_the_d_rods_cut_off_arc_refuses_ring_clearance() {
    let (c, v) = upper_corner(ROD_FILLET);
    let (dx, dy) = (v.0 - c.0, v.1 - c.1);
    let n = dx.hypot(dy);
    let (x, y) = (c.0 + ROD_FILLET * dx / n, c.1 + ROD_FILLET * dy / n);
    assert_cap_ring_refusal(&bored_d_rod(x, y, 0.004), "bore across the arc");
}

/// **A bore definitely clear of the sliver carves as before**, and the
/// bore is carried through untouched: `ΔV = −2·A·L` exactly, the D-rod's
/// own closed form. Two clear readings: a bore at the axis, beyond the
/// annulus's reach (`‖c‖ − a − ρ > 0`), and one INSIDE the ball's
/// section at `c` itself (`r − a > 0`), which is kept material.
#[test]
fn a_bore_clear_of_the_d_rods_sliver_carves_at_the_closed_form() {
    let (c, _) = upper_corner(ROD_FILLET);
    for (x, y, a, what) in [
        (0.0, 0.0, 0.1, "bore at the axis"),
        (c.0, c.1, 0.05, "bore inside the ball's section"),
    ] {
        let body = bored_d_rod(x, y, a);
        validate_geometric(&body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let creases = rod_creases(&body);
        let vol0 = volume(&body);
        let out = fillet_edges(
            &sweep::test_support::at_rest(&body, tol()),
            &creases,
            ROD_FILLET,
            tol(),
        )
        .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
        validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let dv = volume(&out.body) - vol0;
        let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
        assert!((dv - want).abs() < 1e-12, "{what}: ΔV {dv} vs {want}");
    }
}

/// The D-rod's own outline.
fn d_loop<T: Real>() -> ProfileLoop<T> {
    let (c, f) = (rod_chord_at(ROD_FLAT), T::from_f64);
    bulge_loop(vec![
        (Point2::new(f(ROD_FLAT), f(c.half)), f(c.wall_bulge)),
        (Point2::new(f(ROD_FLAT), f(-c.half)), f(0.0)),
    ])
}

/// Both creases of `body` carve at [`ROD_FILLET`], tier 3 accepts the
/// result, and `ΔV = −2·A·L` exactly: whatever else the caps carry is
/// carried through untouched.
fn assert_carves_at_the_closed_form(body: &Body<f64>, what: &str) {
    validate_geometric(body, tol()).unwrap_or_else(|e| panic!("{what}: source tier 3, {e:?}"));
    let creases = rod_creases(body);
    assert_eq!(creases.len(), 2, "{what}: the D's two creases");
    let vol0 = volume(body);
    let out = fillet_edges(
        &sweep::test_support::at_rest(body, tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .unwrap_or_else(|e| panic!("{what}: both creases carve, got {e}"));
    validate_geometric(&out.body, tol()).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
    let dv = volume(&out.body) - vol0;
    let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
    assert!((dv - want).abs() < 1e-12, "{what}: ΔV {dv} vs {want}");
}

/// **A square drive hole on the D-shaft's axis carves.** Its `x = +h`
/// side lies on a line that passes within the sliver's reach of `c`
/// (at `h = 0.2` it runs through `c`'s `x`), so a meter reading the
/// side's whole carrier line refused all three; the SIDE, a segment
/// ending at `y = ±h`, stays short of the corner, and each edge is
/// metered over its own window.
#[test]
fn a_square_hole_on_the_d_shafts_axis_carves_at_the_closed_form() {
    for h in [0.1, 0.15, 0.2] {
        let sq = poly(&[(-h, -h), (h, -h), (h, h), (-h, h)]);
        let body = extruded(SketchPlane::xy(), vec![d_loop(), sq], ROD_L, tol());
        assert_carves_at_the_closed_form(&body, &format!("square hole h = {h}"));
    }
}

/// **A bore in the annulus but away from the corner carves**: behind
/// the ball centre (on the ray from `V` through `c`) and beside it
/// (either way across that ray), each `0.106` from `c`. All three lie
/// inside the annulus `r ≤ ‖p − c‖ ≤ ‖V − c‖` the sliver lies in, in
/// kept material; the half-plane `(p − c)·(V − c) ≥ floor` the sliver
/// also lies in puts them out of reach. The two beside `c` read
/// `(p − c)·(V − c) ≈ 0`, which clears only if `floor` is the sliver's
/// own (`≈ 0.085·‖V − c‖` here) — so they fail if the cut-off arc were
/// taken the wrong way round the section circle, whose far side would
/// drag `floor` down to `−r`.
#[test]
fn a_bore_in_the_annulus_away_from_the_corner_carves_at_the_closed_form() {
    let (c, v) = upper_corner(ROD_FILLET);
    let n = (v.0 - c.0).hypot(v.1 - c.1);
    let (ux, uy) = ((v.0 - c.0) / n, (v.1 - c.1) / n);
    let a = 0.003;
    assert!(
        0.106 - a > ROD_FILLET && 0.106 + a < n,
        "inside the annulus"
    );
    for (dx, dy, what) in [
        (-ux, -uy, "behind the ball centre"),
        (-uy, ux, "beside the ball centre, wallward"),
        (uy, -ux, "beside the ball centre, flatward"),
    ] {
        let (x, y) = (c.0 + 0.106 * dx, c.1 + 0.106 * dy);
        assert_carves_at_the_closed_form(&bored_d_rod(x, y, a), what);
    }
}

/// **A channel in the D's own outline reaching into the sliver
/// refuses.** An L-shaped channel is cut in from the flat below the
/// upper foot, up past the ball's section, and along to a tip at
/// `x = 0.295, y ∈ [0.385, 0.39]` — inside the sliver, `≈ 0.1045` from
/// `c`. Its edges belong to the cycle the cut-off runs in, so the arc
/// would be `mef`'d across them; each is metered like any other cap
/// edge, and the tip's refuses at the cap.
#[test]
fn a_channel_in_the_cut_cycle_reaching_into_the_sliver_refuses_ring_clearance() {
    let c = rod_chord_at(ROD_FLAT);
    let d = bulge_loop(vec![
        (Point2::new(ROD_FLAT, c.half), c.wall_bulge),
        (Point2::new(ROD_FLAT, -c.half), 0.0),
        (Point2::new(ROD_FLAT, 0.2), 0.0),
        (Point2::new(0.15, 0.2), 0.0),
        (Point2::new(0.15, 0.39), 0.0),
        (Point2::new(0.295, 0.39), 0.0),
        (Point2::new(0.295, 0.385), 0.0),
        (Point2::new(0.16, 0.385), 0.0),
        (Point2::new(0.16, 0.21), 0.0),
        (Point2::new(ROD_FLAT, 0.21), 0.0),
    ]);
    let body = extruded(SketchPlane::xy(), vec![d], ROD_L, tol());
    validate_geometric(&body, tol()).expect("source tier 3");
    let creases = rod_creases(&body);
    assert_eq!(creases.len(), 2, "the D's two creases");
    match fillet_edges(
        &sweep::test_support::at_rest(&body, tol()),
        &creases,
        ROD_FILLET,
        tol(),
    )
    .map_err(|e| {
        let text = e.error.to_string();
        (e.error, text)
    }) {
        Err((
            BlendError::RingClearance {
                face,
                chain,
                margin,
                bounded: false,
            },
            text,
        )) => {
            assert_eq!(
                chain,
                Convexity::Convex,
                "a cap meter refuses only beside a convex crease"
            );
            assert!(
                text.contains(
                    "lies in the part of a face the blend cuts away with the material it removes"
                ),
                "the sentence says the edge is cut away with the sliver: {text}"
            );
            assert_eq!(margin.sign, Sign::Negative, "definite, got {margin}");
            let f = body
                .get_face(face)
                .expect("the refusal names a source face");
            assert!(
                f.rings.is_empty()
                    && matches!(
                        body.get_surface(f.surface),
                        Some(geom::Surface::Plane { normal, .. }) if normal.z.abs() > 0.5
                    ),
                "the face named is a cap, the channel in its outer cycle"
            );
        }
        Err((other, _)) => panic!("expected RingClearance at the cap, got {other:?}"),
        Ok(out) => panic!(
            "the cut-off arc was mef'd across the channel's edges ({} faces)",
            out.body.faces().count()
        ),
    }
}

/// A rectangular hole `[x0, x1] × [y0, y1]` through the D-rod.
fn rect_holed_d_rod<T: Decide + AtRestPolicy>(x0: f64, y0: f64, x1: f64, y1: f64) -> Body<T> {
    let hole = poly(&[(x0, y0), (x1, y0), (x1, y1), (x0, y1)]);
    extruded(
        SketchPlane::xy(),
        vec![d_loop(), hole],
        T::from_f64(ROD_L),
        tol(),
    )
}

/// Three rectangular holes, each with an edge that runs from inside the
/// ball's section (`‖p − c‖ < r`, kept material) out to where the
/// sliver's half-planes put it out of reach. The top edge of the first,
/// at `y = c_y + 0.05`, is inside the section from `x = 0.1134` on and
/// runs left to `x = 0.1`; its nearest point to the sliver, the corner
/// `(0.275, 0.3964)`, is `≈ 0.0099` short of the arc. The second and
/// third reach out of the section on the far side of `c` from `V` and
/// below the upper foot.
const TWO_FACED_HOLES: [(f64, f64, f64, f64, &str); 3] = [
    (0.1, 0.35, 0.275, 0.3964, "a hole level with the corner"),
    (0.12, 0.37, 0.27, 0.41, "a hole above the ball centre"),
    (
        0.24,
        0.27,
        0.28,
        0.39,
        "a hole beside the flat, below the foot",
    ),
];

/// **A straight edge that misses the sliver by leaving its enclosure
/// through two faces carves** ([`TWO_FACED_HOLES`]): no one face of the
/// enclosure clears the whole edge, while every point of it is clear of
/// one face or another, and a straight edge is read point by point.
#[test]
fn a_hole_whose_edge_leaves_the_slivers_enclosure_by_two_faces_carves_at_the_closed_form() {
    for (x0, y0, x1, y1, what) in TWO_FACED_HOLES {
        assert_carves_at_the_closed_form(&rect_holed_d_rod(x0, y0, x1, y1), what);
    }
}

/// **The same holes carve at the certified scalar**, their volume
/// enclosing `ΔV = −2·A·L`. Two of their edges run along the section's
/// axes, which are floor directions of the enclosure, so the reading
/// holds only if a crossing whose divisor is within rounding of zero
/// is not spread over the whole edge.
#[test]
fn a_hole_whose_edge_leaves_the_slivers_enclosure_by_two_faces_carves_at_interval() {
    let tol = tol();
    for (x0, y0, x1, y1, what) in TWO_FACED_HOLES {
        let body =
            sweep::test_support::finished(what, rect_holed_d_rod::<Interval>(x0, y0, x1, y1), tol);
        let creases = rod_creases(&body);
        assert_eq!(creases.len(), 2, "{what}: the D's two creases");
        let out = fillet_edges(&body, &creases, iv(ROD_FILLET), tol)
            .unwrap_or_else(|e| panic!("{what}: both creases carve at Interval, got {e:?}"));
        validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("{what}: tier 3, {e:?}"));
        let vol = |b: &Body<Interval>| {
            topo::mass_properties(
                &topo::test_support::finished("the measured body", topo::Body::clone(b), tol),
                tol,
                topo::test_support::pinned_resolution(tol),
            )
            .expect("interval props")
            .met()
            .expect("interval props")
            .volume
        };
        let dv = vol(&out.body) - vol(&body);
        let want = -2.0 * rod_section_cut(ROD_R, ROD_FLAT, ROD_FILLET) * ROD_L;
        assert!(
            dv.lo() <= want + 1e-12 && want - 1e-12 <= dv.hi() && dv.hi() - dv.lo() < 1e-6,
            "{what}: ΔV {dv:?} vs {want}"
        );
    }
}

/// **The enclosure's half-plane leaves no wedge near either foot**: a
/// bore straddling the cut-off arc just past a foot, away from `V`,
/// lies in kept material, and the half-plane towards `V` alone would
/// take some of it in. The floors on the section's own axes close that
/// wedge, so each carves: `δ` past the flat's foot along the flat, and
/// past the wall's foot along the wall, each bore half the gap between
/// the arc and the face it runs beside.
#[test]
fn a_bore_on_the_arc_just_past_a_foot_carves_at_the_closed_form() {
    let (c, _) = upper_corner(ROD_FILLET);
    let wall = c.1.atan2(c.0);
    for delta in [2.0f64, 5.0, 20.0].map(f64::to_radians) {
        for (angle, gap, side) in [
            (
                -delta,
                &(|p: (f64, f64)| ROD_FLAT - p.0) as &dyn Fn((f64, f64)) -> f64,
                "flat",
            ),
            (
                wall + delta,
                &|p: (f64, f64)| ROD_R - p.0.hypot(p.1),
                "wall",
            ),
        ] {
            let p = (
                c.0 + ROD_FILLET * angle.cos(),
                c.1 + ROD_FILLET * angle.sin(),
            );
            let a = 0.5 * gap(p);
            assert_carves_at_the_closed_form(
                &bored_d_rod(p.0, p.1, a),
                &format!(
                    "a bore of radius {a:.2e}, {:.1}° past the {side}'s foot",
                    delta.to_degrees()
                ),
            );
        }
    }
}
