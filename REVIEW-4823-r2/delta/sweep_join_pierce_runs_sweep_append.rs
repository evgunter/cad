
// R2 delta probes (PR 4823 fix pass), R1's `rtk_kiss` ported from
// `flush/touch-key-review`.
fn r2d_kiss(seed: u32) -> (AtRestBody<f64>, AtRestBody<f64>, [f64; 3]) {
    let v = notch343().v;
    let s = f64::from(seed);
    let d = unit([0.3 + 0.1 * s, 0.5 - 0.07 * s, 0.8]);
    let fa = corner_diagonal_frame(d, 0.1 + 0.3 * s);
    let fb = corner_diagonal_frame(d.map(|x| -x), 0.4 + 0.2 * s);
    (
        finished("a", cube_sized(v, fa, [0.0; 3], SIDE)),
        finished("b", cube_sized(v, fb, [0.0; 3], SIDE)),
        v,
    )
}

fn r2d_keys_at(body: &Body<f64>, v: [f64; 3]) -> (usize, usize) {
    let at = crate::common::pinch_cones::vertices_at(body, v);
    let mut keys: Vec<_> = at.iter().map(|&w| body.get_vertex(w).unwrap().point).collect();
    keys.sort();
    keys.dedup();
    (at.len(), keys.len())
}

/// R1's P1 (tilted kiss, 6 seeds × d ∈ {−0.05, +0.05, −0.3}) through
/// the planar and the general offset door.
#[test]
#[ignore = "review probe"]
fn r2d_offset_across_a_tilted_kiss() {
    let band = geom_core::Band::linear(tol()).unwrap();
    for seed in 0..6 {
        let (a, b, v) = r2d_kiss(seed);
        let Ok(BooleanResult::Body(bb)) = topo::union(&a, &b, tol()) else {
            println!("P1 seed={seed}: union not a body");
            continue;
        };
        let body0: Body<f64> = Body::clone(&bb.body);
        let (n, k) = r2d_keys_at(&body0, v);
        for d in [-0.05, 0.05, -0.3] {
            for door in ["planes", "general"] {
                let mut body = body0.clone();
                let mut by: Vec<(topo::SurfaceKey, Vec<topo::FaceKey>)> = Vec::new();
                for (f, data) in body.faces() {
                    match by.iter_mut().find(|(s, _)| *s == data.surface) {
                        Some((_, fs)) => fs.push(f),
                        None => by.push((data.surface, vec![f])),
                    }
                }
                let moves: Vec<topo::ChartMove<f64>> = by
                    .into_iter()
                    .map(|(_, faces)| topo::ChartMove { faces, distance: d })
                    .collect();
                let r = if door == "planes" {
                    topo::offset_planes_together(&mut body, &moves, band, tol()).map(|_| ())
                } else {
                    topo::offset_surfaces_together(&mut body, &moves, band, tol()).map(|_| ())
                };
                let tail = match &r {
                    Ok(()) => format!(
                        "OK vol={:.9} at_v={:?} t3p={}",
                        mass_properties(&body, tol()).map(|m| m.volume).unwrap_or(f64::NAN),
                        r2d_keys_at(&body, v),
                        topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), tol()).is_ok()
                    ),
                    Err(e) => format!("ERR {e:?}").chars().take(120).collect(),
                };
                println!("P1 {door} seed={seed} d={d} verts@v={n} keys@v={k}: {tail}");
            }
        }
    }
}
