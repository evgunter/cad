//! Delta-2 reviewer probe (reach-delta2-3977): the settled-residue
//! fixture's result bodies dumped for an exact-rational oracle
//! (`residue_oracle.py`). Run with DMUT=nobackstop on the mutant tree so
//! the refused rows still hand back their bodies; the verdict the
//! unmutated door gives is printed by the row itself.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::test_support::{brick, mapped_cube};
use topo::{
    Body, BooleanCoincidence, BooleanDeclarations, BooleanResult, CarrierDesc,
    FacePairDeclaration, face_carrier,
};
use topo::entity::LoopBoundary;

fn face_facing(body: &Body<f64>, facing: f64) -> topo::FaceKey {
    body.faces()
        .map(|(k, _)| k)
        .find(|&k| matches!(face_carrier(body, k), Some(CarrierDesc::Plane { normal, .. }) if normal.z * facing > 0.99))
        .unwrap()
}

fn declared(a: topo::FaceKey, b: topo::FaceKey, class: BooleanCoincidence) -> BooleanDeclarations {
    BooleanDeclarations { coincident_faces: vec![FacePairDeclaration::new(a, b, class)], ..BooleanDeclarations::none() }
}

fn dump(tag: &str, body: &Body<f64>) {
    for (fk, face) in body.faces() {
        let Some(Surface::Plane { origin, normal, .. }) = body.get_surface(face.surface) else {
            println!("RES|{tag}|FACE|{fk:?}|nonplanar");
            continue;
        };
        let mut loops = Vec::new();
        for lk in std::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let l = body.get_loop(lk).unwrap();
            let LoopBoundary::Cycle { first } = l.boundary else { continue };
            let pts: Vec<String> = body
                .loop_cycle(first)
                .unwrap()
                .iter()
                .map(|&he| {
                    let p = body.half_edge_start_point(he).unwrap();
                    format!("{:?},{:?},{:?}", p.x, p.y, p.z)
                })
                .collect();
            loops.push(pts.join(";"));
        }
        println!(
            "RES|{tag}|FACE|{}|{:?},{:?},{:?}|{:?},{:?},{:?}|{}",
            face.sense, origin.x, origin.y, origin.z, normal.x, normal.y, normal.z, loops.join("#")
        );
    }
}

#[test]
fn d2_residue_dump() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mutated = std::env::var("DMUT").as_deref() == Ok("nobackstop");
    println!("RES|eps={:e}|zero={:e}|nobackstop={mutated}", tol.eps(), band.zero());
    let phi = 5.0_f64.to_radians();
    let p = Point3::new(0.5, 0.2, 1.0);
    let block = brick::<f64>((0.0, 3.0), (-2.0, 2.5), (0.0, 1.0), tol);
    let (standing, sunk) = ((1.0, 0.0, -1.0), (0.5, 0.5, 1.0));
    let (rest, cont) = (BooleanCoincidence::REST, BooleanCoincidence::Continuation);
    let rows = [("standing", 1.2, standing, rest), ("standing", -1.2, standing, rest), ("sunk", 1.2, sunk, cont), ("sunk", -1.2, sunk, cont), ("sunk", 2.0, sunk, cont), ("sunk", -2.0, sunk, cont)];
    dump("block", &block);
    for (pose, over_eps, (height, depth, facing), class) in rows {
        let theta = over_eps * band.zero();
        let (ea, eb) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(phi.cos(), phi.sin(), theta * phi.sin()));
        let tool = mapped_cube::<f64>(move |u, v, w| p + ea * u + eb * v + Vec3::new(0.0, 0.0, height * w - depth), tol);
        let (top, face) = (face_facing(&block, 1.0), face_facing(&tool, facing));
        let ab = declared(top, face, class);
        let ba = declared(face, top, class);
        let rt = format!("{pose}|{over_eps}");
        dump(&format!("{rt}|tool"), &tool);
        let va = topo::mass_properties(&block, tol).unwrap().volume;
        let vb = topo::mass_properties(&tool, tol).unwrap().volume;
        println!("RES|{rt}|OPS|vA={va:?}|vB={vb:?}");
        let ops = [
            ("union", topo::union_with(&block, &tool, &ab, tol)),
            ("intersect", topo::intersect_with(&block, &tool, &ab, tol)),
            ("AminusB", topo::subtract_with(&block, &tool, &ab, tol)),
            ("BminusA", topo::subtract_with(&tool, &block, &ba, tol)),
        ];
        for (op, out) in ops {
            match out {
                Ok(BooleanResult::Body(bb)) => {
                    let v = topo::mass_properties(&bb.body, tol).unwrap().volume;
                    println!("RES|{rt}|{op}|VERDICT|builds|f64={v:?}");
                    dump(&format!("{rt}|{op}"), &bb.body);
                }
                Ok(BooleanResult::Empty) => println!("RES|{rt}|{op}|VERDICT|empty"),
                Err(e) => println!("RES|{rt}|{op}|VERDICT|refused|{}", format!("{e:?}").chars().take(200).collect::<String>()),
            }
        }
    }
}
