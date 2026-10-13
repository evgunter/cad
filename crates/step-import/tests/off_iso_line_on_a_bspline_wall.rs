//! probe (step 0)
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

type P = (f64, f64, f64);

struct W(Vec<String>);

impl W {
    fn emit(&mut self, body: String) -> u64 {
        let id = self.0.len() as u64 + 1;
        self.0.push(format!("#{id} = {body}"));
        id
    }
    fn pt(&mut self, p: P) -> u64 {
        self.emit(format!("CARTESIAN_POINT('', ({}, {}, {}));", f(p.0), f(p.1), f(p.2)))
    }
    fn dir(&mut self, d: P) -> u64 {
        self.emit(format!("DIRECTION('', ({}, {}, {}));", f(d.0), f(d.1), f(d.2)))
    }
    fn vert(&mut self, p: P) -> u64 {
        let c = self.pt(p);
        self.emit(format!("VERTEX_POINT('', #{c});"))
    }
    fn edge(&mut self, v1: u64, p1: P, v2: u64, p2: P) -> u64 {
        let d = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
        let n = (d.0 * d.0 + d.1 * d.1 + d.2 * d.2).sqrt();
        let c = self.pt(p1);
        let dir = self.dir((d.0 / n, d.1 / n, d.2 / n));
        let vec = self.emit(format!("VECTOR('', #{dir}, 1.0);"));
        let line = self.emit(format!("LINE('', #{c}, #{vec});"));
        self.emit(format!("EDGE_CURVE('', #{v1}, #{v2}, #{line}, .T.);"))
    }
    fn parabola(&mut self, v1: u64, v2: u64, cps: [P; 3]) -> u64 {
        let ids: Vec<String> = cps.iter().map(|&c| format!("#{}", self.pt(c))).collect();
        let curve = self.emit(format!(
            "B_SPLINE_CURVE_WITH_KNOTS('', 2, ({}), .UNSPECIFIED., .F., .F., (3, 3), (0.0, 1.0), .UNSPECIFIED.);",
            ids.join(", ")
        ));
        self.emit(format!("EDGE_CURVE('', #{v1}, #{v2}, #{curve}, .T.);"))
    }
    fn face(&mut self, surf: u64, sense: &str, uses: &[(u64, bool)]) -> u64 {
        let oes: Vec<String> = uses
            .iter()
            .map(|&(ec, fwd)| {
                let s = if fwd { ".T." } else { ".F." };
                format!("#{}", self.emit(format!("ORIENTED_EDGE('', *, *, #{ec}, {s});")))
            })
            .collect();
        let lp = self.emit(format!("EDGE_LOOP('', ({}));", oes.join(", ")));
        let b = self.emit(format!("FACE_OUTER_BOUND('', #{lp}, .T.);"));
        self.emit(format!("ADVANCED_FACE('', (#{b}), #{surf}, {sense});"))
    }
    fn placement(&mut self, at: P, n: P, r: P) -> u64 {
        let c = self.pt(at);
        let z = self.dir(n);
        let xr = self.dir(r);
        self.emit(format!("AXIS2_PLACEMENT_3D('', #{c}, #{z}, #{xr});"))
    }
    fn plane(&mut self, at: P, n: P, r: P) -> u64 {
        let ax = self.placement(at, n, r);
        self.emit(format!("PLANE('', #{ax});"))
    }
}

fn f(x: f64) -> String {
    format!("{x:.17e}")
}

fn tetra_step() -> String {
    let mut w = W(Vec::new());
    let (o, x, q, a): (P, P, P, P) = ((0.0, 0.0, 0.0), (1.0, 0.0, 0.25), (1.0, 1.0, 0.0), (1.0, 0.0, 1.0));
    let (vo, vx, vq, va) = (w.vert(o), w.vert(x), w.vert(q), w.vert(a));
    let c = 0.25;
    let oq = w.edge(vo, o, vq, q);
    let xq = w.parabola(vx, vq, [x, (1.0, 0.5, 0.0), q]);
    let ox = w.parabola(vo, vx, [o, (0.5, 0.0, 0.0), x]);
    let xa = w.edge(vx, x, va, a);
    let ao = w.edge(va, a, vo, o);
    let qa = w.edge(vq, q, va, a);
    // The wall: the parabolic cylinder (u, v) -> (u, v, c (u - v)^2),
    // ruled along u = v + s, so its diagonal u = v is the straight
    // line (t, t, 0). Normal ~ +z.
    let (a2, b2) = ([0.0, 0.0, 1.0], [0.0, 0.5, 1.0]);
    let mut rows = Vec::new();
    for i in 0..3 {
        let mut row = Vec::new();
        for j in 0..3 {
            let z = c * (a2[i] - 2.0 * b2[i] * b2[j] + a2[j]);
            row.push(format!("#{}", w.pt((b2[i], b2[j], z))));
        }
        rows.push(format!("({})", row.join(", ")));
    }
    let wall = w.emit(format!(
        "B_SPLINE_SURFACE_WITH_KNOTS('', 2, 2, ({}), .UNSPECIFIED., .F., .F., .F., (3, 3), (3, 3), (0.0, 1.0), (0.0, 1.0), .UNSPECIFIED.);",
        rows.join(", ")
    ));
    let s3 = 1.0 / 3f64.sqrt();
    let s2 = 1.0 / 2f64.sqrt();
    let p_y0 = w.plane(o, (0.0, -1.0, 0.0), (1.0, 0.0, 0.0));
    let p_x1 = w.plane(x, (1.0, 0.0, 0.0), (0.0, 1.0, 0.0));
    let p_slant = w.plane(o, (-s3, s3, s3), (s2, s2, 0.0));
    let faces = [
        w.face(wall, ".F.", &[(oq, true), (xq, false), (ox, false)]),
        w.face(p_y0, ".T.", &[(ox, true), (xa, true), (ao, true)]),
        w.face(p_x1, ".T.", &[(xq, true), (qa, true), (xa, false)]),
        w.face(p_slant, ".T.", &[(ao, false), (qa, false), (oq, false)]),
    ];
    let fl: Vec<String> = faces.iter().map(|k| format!("#{k}")).collect();
    let shell = w.emit(format!("CLOSED_SHELL('', ({}));", fl.join(", ")));
    let brep = w.emit(format!("MANIFOLD_SOLID_BREP('tetra', #{shell});"));
    let ax = w.placement((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0));
    let lu = w.emit("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) );".into());
    let au = w.emit("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($, .RADIAN.) );".into());
    let su = w.emit("( NAMED_UNIT(*) SI_UNIT($, .STERADIAN.) SOLID_ANGLE_UNIT() );".into());
    let um = w.emit(format!("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.0E-9), #{lu}, 'distance_accuracy_value', '');"));
    let ctx = w.emit(format!("( GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{um})) GLOBAL_UNIT_ASSIGNED_CONTEXT((#{lu}, #{au}, #{su})) REPRESENTATION_CONTEXT('Context #1', '3D') );"));
    let rep = w.emit(format!("ADVANCED_BREP_SHAPE_REPRESENTATION('', (#{ax}, #{brep}), #{ctx});"));
    let app = w.emit("APPLICATION_CONTEXT('core data for automotive mechanical design processes');".into());
    w.emit(format!("APPLICATION_PROTOCOL_DEFINITION('international standard', 'automotive_design', 2000, #{app});"));
    let pc = w.emit(format!("PRODUCT_CONTEXT('', #{app}, 'mechanical');"));
    let prod = w.emit(format!("PRODUCT('tetra', 'tetra', '', (#{pc}));"));
    let pdf = w.emit(format!("PRODUCT_DEFINITION_FORMATION('', '', #{prod});"));
    let pdc = w.emit(format!("PRODUCT_DEFINITION_CONTEXT('part definition', #{app}, 'design');"));
    let pd = w.emit(format!("PRODUCT_DEFINITION('design', '', #{pdf}, #{pdc});"));
    let pds = w.emit(format!("PRODUCT_DEFINITION_SHAPE('', '', #{pd});"));
    w.emit(format!("SHAPE_DEFINITION_REPRESENTATION(#{pds}, #{rep});"));
    format!(
        "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''), '2;1');\nFILE_NAME('tetra.step', '1970-01-01T00:00:00', (''), (''), 'probe', '', '');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n{}\nENDSEC;\nEND-ISO-10303-21;\n",
        w.0.join("\n")
    )
}

#[test]
fn probe() {
    let text = tetra_step();
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { mut body, .. }) => {
            let band = geom_core::Band::linear(Tol::witness()).unwrap();
            for (he, _) in body.half_edges() {
                let face = body.face_of_half_edge(he).unwrap();
                let nurbs = matches!(body.get_surface(body.get_face(face).unwrap().surface), Some(geom::Surface::Nurbs(_)));
                if !nurbs { continue; }
                let e = body.get_half_edge(he).unwrap().edge;
                let desc = body.get_curve_geom(body.get_edge(e).unwrap().curve).and_then(|c| c.certified()).map(|c| format!("{:?}", c.description()).chars().take(50).collect::<String>());
                eprintln!("wall {he:?} desc {desc:?} pcurve_of {}", format!("{:?}", topo::pcurve_of(&body, he, band)).chars().take(90).collect::<String>());
            }
            eprintln!("mint_pcurves: {:?}", topo::mint_pcurves(&mut body, Tol::witness()));
            eprintln!("rows: {}", body.half_edges().filter(|(he, _)| body.pcurve(*he).is_some()).count());
        }
        Ok(other) => eprintln!("other: {other:?}"),
        Err(err) => eprintln!("refused: {err}\n{err:?}"),
    }
}
