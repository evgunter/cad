//! Reviewer probe: print each public door's answer on an all-pending cube.

#[path = "probe_support/cube.rs"]
mod cube;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::Body;

fn s<D: core::fmt::Debug>(d: D) -> String {
    let t = format!("{d:?}");
    t.chars().take(400).collect()
}

#[test]
fn probe_pending_names() {
    let mut body = Body::<f64>::new();
    let _c = cube::build_cube_pub(&mut body);
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let faces = topo::query::all_faces(&body);
    let edges = topo::query::all_edges(&body);
    let f0 = faces[0];
    let e0 = edges[0];
    let mut out: Vec<(String, String)> = Vec::new();
    let mut p = |n: &str, v: String| out.push((n.to_owned(), v));
    p("validate_geometric", s(topo::validate::validate_geometric(&body, tol)));
    p("point_in_solid", s(topo::solid_contain::point_in_solid(&body, Point3::new(0.5, 0.5, 0.5), band, tol)));
    p("transform_rigid", s(topo::transform::transform_rigid(&body, &geom_core::RigidMotion::translation(Vec3::new(1.0, 0.0, 0.0)).unwrap()).map(|_| ())));
    p("mirror_body", s(topo::mirror_body(&body, &topo::Reflection::across(Point3::new(0.0,0.0,0.0), Vec3::new(1.0,0.0,0.0), tol).unwrap(), tol).map(|_| ())));
    p("face_pose", s(topo::readback::face_pose(&body, f0)));
    p("face_stored_surface", s(topo::readback::face_stored_surface(&body, f0)));
    p("face_carrier_kind", s(topo::readback::face_carrier_kind(&body, f0)));
    p("edge_sides", s(topo::readback::edge_sides(&body, e0)));
    p("edge_pose", s(topo::readback::edge_pose(&body, e0)));
    p("edge_carrier_kind", s(topo::readback::edge_carrier_kind(&body, e0)));
    p("face_cover", s(topo::cover::face_cover(&body, f0, band).map(|_| ())));
    p("edge_cover", s(topo::cover::edge_cover(&body, e0).map(|_| ())));
    p("face_outward_normal", s(topo::face_normal::face_outward_normal(&body, f0)));
    p("face_plane", s(topo::face_normal::face_plane(&body, f0)));
    p("face_carrier_desc", s(topo::face_normal::face_carrier_desc(&body, f0).map(|_| ())));
    p("face_surface_kind", s(topo::query::face_surface_kind(&body, f0)));
    p("edge_wedge", s(topo::edge_wedge(&body, e0, band)));
    p("face_nappe", s(topo::face_nappe(&body, f0, band).map(|_| ())));
    p("group_nappe", s(topo::group_nappe(&body, &faces, band).map(|_| ())));
    p("merge_coplanar", s({ let mut b = body.clone(); b.merge_coplanar_faces(tol).map(|_| ()) }));
    let mv = || vec![topo::ChartMove { faces: vec![f0], distance: 0.1 }];
    p("offset_planes_together", s({ let mut b = body.clone(); topo::offset_planes_together(&mut b, &mv(), band, tol).map(|_| ()) }));
    p("offset_charts_together", s({ let mut b = body.clone(); topo::offset_axial::offset_charts_together(&mut b, &mv(), band, tol).map(|_| ()) }));
    p("offset_surfaces_together", s({ let mut b = body.clone(); topo::offset_general::offset_surfaces_together(&mut b, &mv(), band, tol).map(|_| ()) }));
    p("is_axial", s(topo::offset_axial::is_axial(&body, band)));
    p("finish", s(topo::finish(body.clone(), tol).map(|_| ())));
    p("chart_region_overlap", s(topo::chart_region_overlap(&body, faces[0], &body, faces[1], band)));
    p("chart_region_overlap_self", s(topo::chart_region_overlap(&body, faces[0], &body, faces[0], band)));
    p("face_azimuth_window", s(topo::face_azimuth_window_traces(&body, f0, band)));
    p("census_traces", s(topo::census_traces(&body, &topo::boolean::ContactRecords::default(), band, tol, None, topo::CensusStrategy::Realized).0));
    p("classify_unvalidated", s(topo::test_support::classify_unvalidated(&body, tol).map(|_| ())));
    let all = || vec![topo::ChartMove { faces: faces.clone(), distance: 0.1 }];
    p("offset_charts_together_all", s({ let mut b = body.clone(); topo::offset_axial::offset_charts_together(&mut b, &all(), band, tol).map(|_| ()) }));
    p("offset_surfaces_together_all", s({ let mut b = body.clone(); topo::offset_general::offset_surfaces_together(&mut b, &all(), band, tol).map(|_| ()) }));
    p("offset_planes_together_all", s({ let mut b = body.clone(); topo::offset_planes_together(&mut b, &all(), band, tol).map(|_| ()) }));
    let moved = topo::transform::transform_rigid(&body, &geom_core::RigidMotion::translation(Vec3::new(1.0, 0.0, 0.0)).unwrap()).unwrap();
    p("transformed_still_pending", s(topo::readback::face_carrier_kind(&moved, topo::query::all_faces(&moved)[0])));
    p("chart_region_vs_transformed", s(topo::chart_region_overlap(&body, faces[0], &moved, topo::query::all_faces(&moved)[0], band)));
    for (n, v) in &out { println!("DOOR {n}: {v}"); }
}
