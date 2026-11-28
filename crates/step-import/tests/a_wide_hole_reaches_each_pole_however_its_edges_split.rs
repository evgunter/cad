//! **Where import reaches a pole is a function of the geometry, not of
//! how a file splits its edges.** A 10 mm ball with a groove band cut
//! round it: the band `|latitude| < 0.2` over 300° of azimuth, down to a
//! 6 mm floor. Its outer sphere face holds BOTH poles and is bounded by
//! one hole spanning more than half a turn, so no single azimuth "inside
//! the hole" stands for it. Import makes each pole the tip of one wrap
//! edge, aimed at the hole's point nearest that pole
//! (`normalize/singularity.rs`, `wrap_azimuth`): the same body, at the
//! exact volume, whether each long arc of the groove is one edge or
//! eight, and wherever the slot sits round the axis.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// The outer radius (mm).
const R: f64 = 10.0;
/// The floor's radius (mm).
const RHO: f64 = 6.0;
/// The band's half-height in latitude.
const LAT: f64 = 0.2;
/// The slot's span in azimuth: 300°.
const SPAN: f64 = 5.0 * PI / 3.0;

/// A STEP text under construction: one entity per call, numbered in order.
struct Text(Vec<String>);

impl Text {
    fn add(&mut self, body: String) -> usize {
        self.0.push(body);
        self.0.len()
    }
    fn point(&mut self, p: [f64; 3]) -> usize {
        self.add(format!(
            "CARTESIAN_POINT('',({:?},{:?},{:?}))",
            p[0], p[1], p[2]
        ))
    }
    fn dir(&mut self, v: [f64; 3]) -> usize {
        self.add(format!("DIRECTION('',({:?},{:?},{:?}))", v[0], v[1], v[2]))
    }
    fn frame(&mut self, o: [f64; 3], z: [f64; 3], x: [f64; 3]) -> usize {
        let (o, z, x) = (self.point(o), self.dir(z), self.dir(x));
        self.add(format!("AXIS2_PLACEMENT_3D('',#{o},#{z},#{x})"))
    }
    fn vertex(&mut self, p: [f64; 3]) -> usize {
        let p = self.point(p);
        self.add(format!("VERTEX_POINT('',#{p})"))
    }
    fn circle(&mut self, o: [f64; 3], z: [f64; 3], x: [f64; 3], r: f64) -> usize {
        let f = self.frame(o, z, x);
        self.add(format!("CIRCLE('',#{f},{r:?})"))
    }
    fn line(&mut self, o: [f64; 3], d: [f64; 3]) -> usize {
        let (o, d) = (self.point(o), self.dir(d));
        let v = self.add(format!("VECTOR('',#{d},1.)"));
        self.add(format!("LINE('',#{o},#{v})"))
    }
    fn edge(&mut self, a: usize, b: usize, curve: usize) -> usize {
        self.add(format!("EDGE_CURVE('',#{a},#{b},#{curve},.T.)"))
    }
    /// A face on `surface` bounded by one loop of `(edge, forward)` uses.
    fn face(&mut self, surface: usize, uses: &[(usize, bool)]) -> usize {
        self.face_sensed(surface, uses, true)
    }
    /// [`Self::face`] with its `same_sense` stated.
    fn face_sensed(&mut self, surface: usize, uses: &[(usize, bool)], sense: bool) -> usize {
        let oriented: Vec<String> = uses
            .iter()
            .map(|&(e, fwd)| {
                let s = if fwd { ".T." } else { ".F." };
                format!("#{}", self.add(format!("ORIENTED_EDGE('',*,*,#{e},{s})")))
            })
            .collect();
        let lp = self.add(format!("EDGE_LOOP('',({}))", oriented.join(",")));
        let bound = self.add(format!("FACE_OUTER_BOUND('',#{lp},.T.)"));
        let sense = if sense { ".T." } else { ".F." };
        self.add(format!("ADVANCED_FACE('',(#{bound}),#{surface},{sense})"))
    }

    /// The file: these entities, `faces` closed into one solid, in mm.
    fn solid(mut self, faces: &[usize]) -> String {
        let z = [0.0, 0.0, 1.0];
        let origin = [0.0, 0.0, 0.0];
        let faces: Vec<String> = faces.iter().map(|f| format!("#{f}")).collect();
        let shell = self.add(format!("CLOSED_SHELL('',({}))", faces.join(",")));
        let brep = self.add(format!("MANIFOLD_SOLID_BREP('',#{shell})"));
        let place = self.frame(origin, z, [1.0, 0.0, 0.0]);
        let n = self.0.len();
        let ctx = n + 20;
        let mut out = String::from(
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('groove ball'),'2;1');\n\
         FILE_NAME('groove','2026-11-25T00:00:00',(''),(''),'','','');\n\
         FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));\nENDSEC;\nDATA;\n",
        );
        for (i, body) in self.0.iter().enumerate() {
            out += &format!("#{} = {body};\n", i + 1);
        }
        out += &format!(
        "#{a} = APPLICATION_CONTEXT('core data for automotive mechanical design processes');
#{b} = APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#{a});
#{c} = PRODUCT_CONTEXT('',#{a},'mechanical');
#{d} = PRODUCT('groove','groove','',(#{c}));
#{e} = PRODUCT_DEFINITION_FORMATION('','',#{d});
#{g} = PRODUCT_DEFINITION_CONTEXT('part definition',#{a},'design');
#{h} = PRODUCT_DEFINITION('design','',#{e},#{g});
#{i} = PRODUCT_DEFINITION_SHAPE('','',#{h});
#{j} = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#{place},#{brep}),#{ctx});
#{k} = SHAPE_DEFINITION_REPRESENTATION(#{i},#{j});
#{u1} = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );
#{u2} = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );
#{u3} = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );
#{u4} = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#{u1},'distance_accuracy_value','confusion accuracy');
#{ctx} = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{u4})) GLOBAL_UNIT_ASSIGNED_CONTEXT((#{u1},#{u2},#{u3})) REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') );
ENDSEC;
END-ISO-10303-21;
",
        a = n + 1,
        b = n + 2,
        c = n + 3,
        d = n + 4,
        e = n + 5,
        g = n + 6,
        h = n + 7,
        i = n + 8,
        j = n + 9,
        k = n + 10,
        u1 = n + 11,
        u2 = n + 12,
        u3 = n + 13,
        u4 = n + 14,
    );
        out
    }
}

/// One logical edge, stated as a run of edges in its own direction.
type Run = Vec<usize>;

fn walked(run: &Run, forward: bool) -> Vec<(usize, bool)> {
    if forward {
        run.iter().map(|&e| (e, true)).collect()
    } else {
        run.iter().rev().map(|&e| (e, false)).collect()
    }
}

/// The groove ball, its slot starting at azimuth `from` and each latitude
/// arc stated as `pieces` edges.
fn slot_ball(from: f64, pieces: usize) -> String {
    let mut t = Text(Vec::new());
    let at = |az: f64, lat: f64, r: f64| {
        [
            r * lat.cos() * az.cos(),
            r * lat.cos() * az.sin(),
            r * lat.sin(),
        ]
    };
    let radial = |az: f64| [az.cos(), az.sin(), 0.0];
    let (z, mz) = ([0.0, 0.0, 1.0], [0.0, 0.0, -1.0]);
    let to = from + SPAN;
    let mut corner = std::collections::BTreeMap::new();
    for (ai, az) in [(0, from), (1, to)] {
        for (li, lat) in [(0, -LAT), (1, LAT)] {
            for (ri, r) in [(0, RHO), (1, R)] {
                corner.insert((ai, li, ri), t.vertex(at(az, lat, r)));
            }
        }
    }
    // A latitude arc from `from` to `to`, in `pieces` edges.
    let arc = |t: &mut Text, li: usize, ri: usize| -> Run {
        let (lat, r) = ([-LAT, LAT][li], [RHO, R][ri]);
        let c = t.circle([0.0, 0.0, r * lat.sin()], z, [1.0, 0.0, 0.0], r * lat.cos());
        let mut ends = vec![corner[&(0, li, ri)]];
        for k in 1..pieces {
            ends.push(t.vertex(at(from + SPAN * k as f64 / pieces as f64, lat, r)));
        }
        ends.push(corner[&(1, li, ri)]);
        ends.windows(2).map(|w| t.edge(w[0], w[1], c)).collect()
    };
    let low_out = arc(&mut t, 0, 1);
    let up_out = arc(&mut t, 1, 1);
    let low_in = arc(&mut t, 0, 0);
    let up_in = arc(&mut t, 1, 0);
    // A meridian arc at azimuth index `ai`, from −LAT up to +LAT.
    let meridian = |t: &mut Text, ai: usize, ri: usize| -> Run {
        let az = [from, to][ai];
        let r = [RHO, R][ri];
        let axis = [az.sin(), -az.cos(), 0.0];
        let c = t.circle([0.0, 0.0, 0.0], axis, radial(az), r);
        vec![t.edge(corner[&(ai, 0, ri)], corner[&(ai, 1, ri)], c)]
    };
    let (m0_out, m1_out) = (meridian(&mut t, 0, 1), meridian(&mut t, 1, 1));
    let (m0_in, m1_in) = (meridian(&mut t, 0, 0), meridian(&mut t, 1, 0));
    // A radial line at a corner, from the floor out.
    let spoke = |t: &mut Text, ai: usize, li: usize| -> Run {
        let (az, lat) = ([from, to][ai], [-LAT, LAT][li]);
        let l = t.line(at(az, lat, RHO), at(az, lat, 1.0));
        vec![t.edge(corner[&(ai, li, 0)], corner[&(ai, li, 1)], l)]
    };
    let (s0d, s0u, s1d, s1u) = (
        spoke(&mut t, 0, 0),
        spoke(&mut t, 0, 1),
        spoke(&mut t, 1, 0),
        spoke(&mut t, 1, 1),
    );
    let origin = [0.0, 0.0, 0.0];
    let outer = {
        let f = t.frame(origin, z, [1.0, 0.0, 0.0]);
        t.add(format!("SPHERICAL_SURFACE('',#{f},{R:?})"))
    };
    let floor = {
        let f = t.frame(origin, z, [1.0, 0.0, 0.0]);
        t.add(format!("SPHERICAL_SURFACE('',#{f},{RHO:?})"))
    };
    let semi = PI / 2.0 - LAT;
    let upper = {
        let f = t.frame(origin, z, [1.0, 0.0, 0.0]);
        t.add(format!("CONICAL_SURFACE('',#{f},0.,{semi:?})"))
    };
    let lower = {
        let f = t.frame(origin, mz, [1.0, 0.0, 0.0]);
        t.add(format!("CONICAL_SURFACE('',#{f},0.,{semi:?})"))
    };
    let wall = |t: &mut Text, az: f64, sign: f64| {
        let n = [-az.sin() * sign, az.cos() * sign, 0.0];
        let f = t.frame(origin, n, radial(az));
        t.add(format!("PLANE('',#{f})"))
    };
    let (wall0, wall1) = (wall(&mut t, from, 1.0), wall(&mut t, to, -1.0));
    let cat = |parts: &[Vec<(usize, bool)>]| parts.concat();
    let faces = [
        // The outer sphere: the slot's rim, walked round the hole with the
        // face on its left seen from outside.
        t.face(
            outer,
            &cat(&[
                walked(&m0_out, true),
                walked(&up_out, true),
                walked(&m1_out, false),
                walked(&low_out, false),
            ]),
        ),
        t.face(
            floor,
            &cat(&[
                walked(&low_in, true),
                walked(&m1_in, true),
                walked(&up_in, false),
                walked(&m0_in, false),
            ]),
        ),
        t.face(
            upper,
            &cat(&[
                walked(&up_in, true),
                walked(&s1u, true),
                walked(&up_out, false),
                walked(&s0u, false),
            ]),
        ),
        t.face(
            lower,
            &cat(&[
                walked(&s0d, true),
                walked(&low_out, true),
                walked(&s1d, false),
                walked(&low_in, false),
            ]),
        ),
        t.face(
            wall0,
            &cat(&[
                walked(&m0_in, true),
                walked(&s0u, true),
                walked(&m0_out, false),
                walked(&s0d, false),
            ]),
        ),
        t.face(
            wall1,
            &cat(&[
                walked(&s1d, true),
                walked(&m1_out, true),
                walked(&s1u, false),
                walked(&m1_in, false),
            ]),
        ),
    ];
    t.solid(&faces)
}

/// The ball less its groove (m³): the groove is the shell between the
/// floor and the outer sphere over the band's solid angle,
/// `SPAN · 2 sin LAT`.
fn exact() -> f64 {
    let ball = 4.0 * PI * R * R * R / 3.0;
    let groove = (R * R * R - RHO * RHO * RHO) / 3.0 * SPAN * 2.0 * LAT.sin();
    (ball - groove) * 1e-9
}

#[test]
fn a_groove_round_both_poles_imports_alike_however_its_arcs_split() {
    let tol = Tol::witness();
    let mut seen = Vec::new();
    for from in [PI / 6.0, -5.0 * PI / 6.0] {
        for pieces in [1, 8] {
            let name = format!("slot from {from:.3} rad, {pieces} edge(s) per arc");
            let imported = import_step(&slot_ball(from, pieces), &ImportOptions::default(), tol)
                .unwrap_or_else(|e| panic!("{name}: {}", e.render()));
            let StepImport::Solid {
                body,
                normalizations,
                ..
            } = imported
            else {
                panic!("{name}: a solid");
            };
            let tips = normalizations
                .iter()
                .filter(|n| matches!(n.kind, NormalizationKind::ChartSingularityVertex { .. }))
                .count();
            assert_eq!(tips, 2, "{name}: each pole reached by a wrap edge");
            let b = &*body;
            crate::common::assert_singular_points_are_vertices(&name, b);
            assert_eq!(topo::validate_geometric(b, tol), Ok(()), "{name}: tier 3");
            let volume = topo::test_support::pinned_volume(
                "the groove ball",
                &topo::test_support::finished("the groove ball", b.clone(), tol),
                tol,
            )
            .volume;
            let want = exact();
            assert!(
                (volume - want).abs() <= 1e-9 * want,
                "{name}: volume {volume} vs {want}"
            );
            seen.push((
                name,
                (
                    b.faces().count(),
                    b.edges().count(),
                    b.vertex_points().count(),
                ),
            ));
        }
    }
    // The splits a file states join away: one census over both statements
    // of one slot. (Where the slot spans `u_ref`, each wrap edge lands
    // inside a rim and splits it, so the two slots' censuses differ.)
    for pair in seen.chunks(2) {
        assert_eq!(pair[1].1, pair[0].1, "{}: the census", pair[1].0);
    }
    assert_eq!(seen[0].1, (6, 14, 10), "{}: the census", seen[0].0);
    assert_eq!(seen[2].1, (6, 16, 12), "{}: the census", seen[2].0);
}

/// The crescent groove's caps: `A`, about the pole at polar angle
/// `ALPHA`, and `B`, about an axis tilted `tau` from the pole towards
/// `+x` at polar angle `beta`, poking out of `A` on that side.
const ALPHA: f64 = 0.6;

/// The unit axis of cap `B`.
fn b_axis(tau: f64) -> [f64; 3] {
    [tau.sin(), 0.0, tau.cos()]
}

/// The polar angle of `B`'s rim in the direction `az` from the pole.
fn b_rim(tau: f64, beta: f64, az: f64) -> f64 {
    let (a, c) = (tau.cos(), tau.sin() * az.cos());
    c.atan2(a) + (beta.cos() / a.hypot(c)).acos()
}

/// The azimuth, in (0, π), where the two rims cross.
fn crossing(tau: f64, beta: f64) -> f64 {
    let x = (beta.cos() - tau.cos() * ALPHA.cos()) / tau.sin();
    let y = (ALPHA.sin() * ALPHA.sin() - x * x).sqrt();
    y.atan2(x)
}

/// The ball less cap `A` (the ball beyond the plane at polar angle
/// `ALPHA`), with cap `B` (beyond the plane at `beta` about the tilted
/// axis) kept: a crescent cut from the sphere on the `−x` side, spanning
/// the azimuths `(φ, 2π − φ)` past the crossing `φ`. Its three faces
/// are the sphere, cap `A`'s plane and cap `B`'s, which meet on the
/// line `L` between the two crossings. Each rim arc is stated as
/// `pieces` edges.
fn crescent_ball(tau: f64, beta: f64, pieces: usize) -> String {
    let mut t = Text(Vec::new());
    let (z, origin) = ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0]);
    let b = b_axis(tau);
    let mb = [-b[0], -b[1], -b[2]];
    let b_ref = [tau.cos(), 0.0, -tau.sin()];
    let phi = crossing(tau, beta);
    let on_sphere = |az: f64, theta: f64| {
        [
            R * theta.sin() * az.cos(),
            R * theta.sin() * az.sin(),
            R * theta.cos(),
        ]
    };
    let ends = [on_sphere(phi, ALPHA), on_sphere(-phi, ALPHA)];
    let (p1, p2) = (t.vertex(ends[0]), t.vertex(ends[1]));
    let between = |k: usize| phi + (2.0 * PI - 2.0 * phi) * k as f64 / pieces as f64;
    // `A`'s rim from the first crossing round the `−x` side to the
    // second, its azimuth rising.
    let a_out: Run = {
        let c = t.circle(
            [0.0, 0.0, R * ALPHA.cos()],
            z,
            [1.0, 0.0, 0.0],
            R * ALPHA.sin(),
        );
        let mut v = vec![p1];
        for k in 1..pieces {
            v.push(t.vertex(on_sphere(between(k), ALPHA)));
        }
        v.push(p2);
        v.windows(2).map(|w| t.edge(w[0], w[1], c)).collect()
    };
    // `B`'s rim inside `A`, the same way round.
    let b_out: Run = {
        let centre = [b[0] * R * beta.cos(), 0.0, b[2] * R * beta.cos()];
        let c = t.circle(centre, b, b_ref, R * beta.sin());
        let mut v = vec![p1];
        for k in 1..pieces {
            let az = between(k);
            v.push(t.vertex(on_sphere(az, b_rim(tau, beta, az))));
        }
        v.push(p2);
        v.windows(2).map(|w| t.edge(w[0], w[1], c)).collect()
    };
    let l: Run = {
        let d = [
            ends[1][0] - ends[0][0],
            ends[1][1] - ends[0][1],
            ends[1][2] - ends[0][2],
        ];
        let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        let line = t.line(ends[0], [d[0] / n, d[1] / n, d[2] / n]);
        vec![t.edge(p1, p2, line)]
    };
    let sphere = {
        let f = t.frame(origin, z, [1.0, 0.0, 0.0]);
        t.add(format!("SPHERICAL_SURFACE('',#{f},{R:?})"))
    };
    // Each plane's normal leaves the body: `A`'s into the crescent above
    // it, `B`'s away from `B`'s axis.
    let plane_a = {
        let f = t.frame([0.0, 0.0, R * ALPHA.cos()], z, [1.0, 0.0, 0.0]);
        t.add(format!("PLANE('',#{f})"))
    };
    let plane_b = {
        let at = [b[0] * R * beta.cos(), 0.0, b[2] * R * beta.cos()];
        let f = t.frame(at, mb, b_ref);
        t.add(format!("PLANE('',#{f})"))
    };
    let cat = |parts: &[Vec<(usize, bool)>]| parts.concat();
    let faces = [
        t.face(sphere, &cat(&[walked(&a_out, false), walked(&b_out, true)])),
        t.face(plane_a, &cat(&[walked(&a_out, true), walked(&l, false)])),
        t.face(plane_b, &cat(&[walked(&b_out, false), walked(&l, true)])),
    ];
    t.solid(&faces)
}

/// The ball less cap `A`, with `A ∩ B` kept (m³). The lens `A ∩ B` is
/// summed by Simpson's rule over heights `z`: each slice is the disc of
/// radius `√(R² − z²)` cut by `B`'s plane, a circular segment in closed
/// form.
fn crescent_exact(tau: f64, beta: f64) -> f64 {
    let cap = |h: f64| PI * h * h * (3.0 * R - h) / 3.0;
    let (h_a, h_b) = (R * ALPHA.cos(), R * beta.cos());
    let slice = |z: f64| {
        let rho = (R * R - z * z).max(0.0).sqrt();
        // The part of the slice with x past `B`'s plane.
        let d = (h_b - tau.cos() * z) / tau.sin();
        if d >= rho {
            0.0
        } else if d <= -rho {
            PI * rho * rho
        } else {
            rho * rho * (d / rho).acos() - d * (rho * rho - d * d).sqrt()
        }
    };
    let n = 400_000;
    let h = (R - h_a) / n as f64;
    let mut sum = slice(h_a) + slice(R);
    for k in 1..n {
        sum += slice(h_a + h * k as f64) * if k % 2 == 1 { 4.0 } else { 2.0 };
    }
    let lens = sum * h / 3.0;
    (4.0 * PI * R * R * R / 3.0 - cap(R - h_a) + lens) * 1e-9
}

#[test]
fn a_crescent_round_both_poles_imports_alike_however_its_arcs_split() {
    let tol = Tol::witness();
    for (tau, beta) in [(0.02, 0.59), (0.15, 0.5)] {
        let mut seen = Vec::new();
        for pieces in [1, 6] {
            let name = format!("crescent τ {tau}, β {beta}, {pieces} edge(s) per arc");
            let imported = import_step(
                &crescent_ball(tau, beta, pieces),
                &ImportOptions::default(),
                tol,
            )
            .unwrap_or_else(|e| panic!("{name}: {}", e.render()));
            let StepImport::Solid { body, .. } = imported else {
                panic!("{name}: a solid");
            };
            let b = &*body;
            crate::common::assert_singular_points_are_vertices(&name, b);
            assert_eq!(topo::validate_geometric(b, tol), Ok(()), "{name}: tier 3");
            let volume = topo::test_support::pinned_volume(
                "the crescent ball",
                &topo::test_support::finished("the crescent ball", b.clone(), tol),
                tol,
            )
            .volume;
            let want = crescent_exact(tau, beta);
            assert!(
                (volume - want).abs() <= 1e-9 * want,
                "{name}: volume {volume} vs {want}"
            );
            seen.push((
                b.faces().count(),
                b.edges().count(),
                b.vertex_points().count(),
            ));
        }
        assert_eq!(
            seen[1], seen[0],
            "crescent τ {tau}: one census for both statements"
        );
    }
}
