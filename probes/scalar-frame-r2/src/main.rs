//! R2 end-to-end exercise of FRAME-WITNESS, written as a user would:
//! against the `pncad` facade only.
use pncad::geom_core::linalg::frame::point_at;
use pncad::geom_core::{Affine3, Band, OrthoFrame, Point3, Tol, UnitVec3, Vec3};
use pncad::profile::SketchPlane;
use pncad::sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};

fn b() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn bits3(v: Vec3<f64>) -> [u64; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

fn aff_bits(a: &Affine3<f64>) -> [u64; 12] {
    let l = a.linear;
    [
        l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y, l.c2.z, a.translation.x,
        a.translation.y, a.translation.z,
    ]
    .map(f64::to_bits)
}

fn main() {
    // ---------- 1. A tube from a datum frame (kernel-direct) ----------
    let axis = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), "probe_axis", b()).unwrap();
    let frame = OrthoFrame::from_aim_and_reference(
        Point3::origin(),
        axis,
        Vec3::unit_x(),
        "probe_uref",
        b(),
    )
    .unwrap();
    let t = tube_along_arc(frame, 3.0, TubeWindow::Full, 0.5, Tol::witness()).unwrap();
    let mp = pncad::topo::mass_properties(&t.body, Tol::witness()).unwrap();
    println!(
        "TUBE volume bits {:#x} faces {}",
        mp.volume.to_bits(),
        t.body.faces().count()
    );

    let h = tube_along_arc_hollow(frame, 3.0, TubeWindow::Full, 0.5, 0.1, Tol::witness()).unwrap();
    let mph = pncad::topo::mass_properties(&h.body, Tol::witness()).unwrap();
    println!(
        "HOLLOW volume bits {:#x} faces {}",
        mph.volume.to_bits(),
        h.body.faces().count()
    );

    // ---------- 2. THE HOLE: `from_aim` mints a NON-orthonormal frame ----------
    let skew = OrthoFrame::from_aim(
        Point3::origin(),
        axis,
        Vec3::new(1.0, 0.0, 1.0), // 45 degrees off perpendicular
        "probe_skew",
        b(),
    )
    .unwrap();
    let (u, v, w) = (skew.u().get(), skew.v().get(), skew.w().get());
    println!(
        "SKEW |u|={} |v|={} |w|={} u.w={} det={}",
        u.norm(),
        v.norm(),
        w.norm(),
        u.dot(w),
        u.cross(v).dot(w)
    );
    match tube_along_arc(skew, 3.0, TubeWindow::Full, 0.5, Tol::witness()) {
        Ok(body) => {
            let m = pncad::topo::mass_properties(&body.body, Tol::witness()).unwrap();
            println!(
                "SKEW TUBE BUILT: faces {} volume {} (honest torus volume {})",
                body.body.faces().count(),
                m.volume,
                2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0 * 0.25
            );
        }
        Err(e) => println!("SKEW TUBE refused: {e}"),
    }
    let sp = SketchPlane::from_frame(skew);
    println!("SKEW PLANE placement {:?}", sp.placement.linear);

    // ---------- 3. SketchPlane from an exact world frame / a skewed hand pair ----------
    let exact = SketchPlane::<f64>::xy();
    println!("XY plane bits {:?}", aff_bits(&exact.placement));
    let skew_pair = OrthoFrame::gram_schmidt(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0), // 45 degrees into u -- does this refuse?
        "probe_gs_u",
        "probe_gs_v",
        b(),
    );
    match skew_pair {
        Ok(f) => println!(
            "SKEWED HAND PAIR ACCEPTED (orthonormalized): u {:?} v {:?}",
            bits3(f.u().get()),
            bits3(f.v().get())
        ),
        Err(e) => println!("SKEWED HAND PAIR refused: {e}"),
    }
    let parallel = OrthoFrame::<f64>::gram_schmidt(
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        "probe_gs_u",
        "probe_gs_v",
        b(),
    );
    println!("PARALLEL PAIR -> {:?}", parallel.err().map(|e| e.to_string()));

    // ---------- 4. point_at placement ----------
    let pa = point_at(
        Point3::new(1.0, 2.0, 3.0),
        Point3::new(4.0, 6.0, 3.0),
        Vec3::unit_z(),
        Tol::witness(),
    )
    .unwrap();
    println!("POINT_AT bits {:?}", aff_bits(&pa));

    // ---------- 5. A MILD skew: does it build a silently-wrong torus? ----------
    let honest = 2.0 * std::f64::consts::PI * std::f64::consts::PI * 3.0 * 0.25;
    for off in [1e-1, 1e-2, 1e-3, 1e-4, 1e-6] {
        let f = OrthoFrame::from_aim(
            Point3::origin(),
            axis,
            Vec3::new(1.0, 0.0, off),
            "probe_mild",
            b(),
        )
        .unwrap();
        match tube_along_arc(f, 3.0, TubeWindow::Full, 0.5, Tol::witness()) {
            Ok(body) => {
                let m = pncad::topo::mass_properties(&body.body, Tol::witness()).unwrap();
                println!(
                    "MILD SKEW off={off:e} BUILT volume {} vs honest {honest} (rel {:e})",
                    m.volume,
                    (m.volume - honest).abs() / honest
                );
            }
            Err(e) => {
                let s = e.to_string();
                println!("MILD SKEW off={off:e} refused: {}", &s[..70.min(s.len())]);
            }
        }
    }

    // ---------- 6. A datum_frame node through the document ----------
    {
        use pncad::document::{
            CancelToken, Datum, Dimension, DocEdit, DocumentId, EvalOptions, Expr, Node,
            ProfileDoc, ProfileProgram, ValuePayload, evaluate,
        };
        let len = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
        let scl = |v: f64| Expr::literal(v, Dimension::Scalar).unwrap();
        let doc = ProfileDoc::empty(DocumentId::derive("r2probe"), Tol::witness());
        let applied = doc
            .apply(
                &DocEdit::<ProfileProgram>::InsertNode {
                    node: Node::Datum(Datum::Frame {
                        origin: [0.0, 0.0, 0.0].map(len),
                        u: [2.0, 0.0, 0.0].map(scl),
                        v: [1.0, 1.0, 0.0].map(scl), // neither unit nor perpendicular
                    }),
                },
                Tol::witness(),
            )
            .unwrap();
        let id = applied.record.minted.unwrap();
        let ev = evaluate::<f64>(
            &applied.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        match ev.value(id).map(|v| &v.payload) {
            Some(ValuePayload::Datum(d)) => println!("DATUM FRAME -> {d:?}"),
            other => println!("DATUM FRAME -> {other:?}"),
        }
    }

    println!("done");
}
