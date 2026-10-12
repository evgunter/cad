
    use super::*;
    use geom_core::Tol;
    fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
        HarmChan {
            c0: pt(c0),
            ca: pt(ca),
            cb: pt(cb),
            cl: pt(cl),
        }
    }
    fn edge(u: HarmChan, v: HarmChan, t1: f64, forward: bool) -> TrimEdgeQ {
        TrimEdgeQ {
            u,
            v,
            t0: pt(0.0),
            t1: pt(t1),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        }
    }
    fn run<T: Decide>(loops: &[&[TrimEdgeQ]], eps: f64) -> Result<FaceCutBounds, PropsError> {
        let band = geom_core::Band::linear_at(Tol::witness(), eps).unwrap();
        cylinder_cut_face::<T>(pt(1.0), Interval::zero(), loops, eps, band)
    }
    fn report(what: &str, r: &Result<FaceCutBounds, PropsError>, exact: f64) {
        match r {
            Ok(o) => eprintln!(
                "PROBE {what}: area {:?} flux {:?} exact {exact} encl_area={} encl_flux={}",
                o.area,
                o.flux,
                o.area.lo() <= exact.abs() && exact.abs() <= o.area.hi(),
                o.flux.lo() <= exact && exact <= o.flux.hi()
            ),
            Err(e) => eprintln!("PROBE {what}: ERR {e:?}"),
        }
    }
    const TAU: f64 = core::f64::consts::TAU;

    /// A band whose rims each wind twice (u over [0, 4π]) — the loops
    /// self-overlap; the true band (as a point set) has area 2π·h.
    #[test]
    fn probe_double_winding_pair() {
        let eps = Tol::witness().get().eps;
        let b = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(0.0, 0.0, 0.0, 0.0),
            2.0 * TAU,
            true,
        );
        let t = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(1.0, 0.0, 0.0, 0.0),
            2.0 * TAU,
            false,
        );
        report("double_winding_f64", &run::<f64>(&[&[b], &[t]], eps), TAU);
        report(
            "double_winding_iv",
            &run::<Interval>(&[&[b], &[t]], eps),
            TAU,
        );
        // 1.5 turns each: neither loop closes on the cylinder.
        let b = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(0.0, 0.0, 0.0, 0.0),
            1.5 * TAU,
            true,
        );
        let t = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(1.0, 0.0, 0.0, 0.0),
            1.5 * TAU,
            false,
        );
        report("one_and_half_f64", &run::<f64>(&[&[b], &[t]], eps), TAU);
        report("one_and_half_iv", &run::<Interval>(&[&[b], &[t]], eps), TAU);
        // 0.3 turn + 0.3 turn back on a different level: not closed loops.
        let b = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(0.0, 0.0, 0.0, 0.0),
            0.3 * TAU,
            true,
        );
        let t = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(1.0, 0.0, 0.0, 0.0),
            0.3 * TAU,
            false,
        );
        report("open_fragments_f64", &run::<f64>(&[&[b], &[t]], eps), 0.0);
    }

    /// Oblique cuts near 0 and near π/2 (k = tan θ), lifted off origin,
    /// both senses, f64 and interval, three eps rows.
    #[test]
    fn probe_oblique_bands() {
        for eps in [1e-6, 1e-9, 1e-12] {
            for k in [0.0, 1e-12, 1e-6, 0.4, 5.0, 50.0, 1e3] {
                for lift in [0.0, -7.5, 1e4] {
                    for sense in [false, true] {
                        let h = 2.0 + k; // keep rims apart
                        let b = edge(
                            chan(0.0, 0.0, 0.0, 1.0),
                            chan(lift, 0.0, 0.0, 0.0),
                            TAU,
                            !sense,
                        );
                        let t = edge(
                            chan(0.0, 0.0, 0.0, 1.0),
                            chan(lift + h, k, 0.0, 0.0),
                            TAU,
                            sense,
                        );
                        let he = (lift + h) - lift; // stored height
                        let exact = if sense { -he * TAU } else { he * TAU };
                        let rf = run::<f64>(&[&[b], &[t]], eps);
                        let ri = run::<Interval>(&[&[b], &[t]], eps);
                        for (w, r) in [("f64", &rf), ("iv", &ri)] {
                            match r {
                                Ok(o)
                                    if o.flux.lo() <= exact
                                        && exact <= o.flux.hi()
                                        && o.area.lo() <= exact.abs()
                                        && exact.abs() <= o.area.hi() => {}
                                _ => report(
                                    &format!(
                                        "oblique eps {eps} k {k} lift {lift} sense {sense} {w}"
                                    ),
                                    r,
                                    exact,
                                ),
                            }
                        }
                    }
                }
            }
        }
    }

    /// Seamed face: rectangle u∈[0,2π] v∈[0,1] with seam edges, plus a
    /// junction near the seam; plus a band with a hole.
    #[test]
    fn probe_seamed_and_holed() {
        let eps = Tol::witness().get().eps;
        let pi = core::f64::consts::PI;
        let seam_face = [
            edge(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(0.0, 0.0, 0.0, 0.0),
                TAU,
                true,
            ),
            edge(
                chan(TAU, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                true,
            ),
            edge(
                chan(0.0, 0.0, 0.0, 1.0),
                chan(1.0, 0.0, 0.0, 0.0),
                TAU,
                false,
            ),
            edge(
                chan(0.0, 0.0, 0.0, 0.0),
                chan(0.0, 0.0, 0.0, 1.0),
                1.0,
                false,
            ),
        ];
        report("seam_face", &run::<f64>(&[&seam_face], eps), TAU);
        // the same seamed face with its top edge stored a period off (u in [2π, 4π]).
        let mut shifted = seam_face;
        shifted[2] = edge(
            chan(TAU, 0.0, 0.0, 1.0),
            chan(1.0, 0.0, 0.0, 0.0),
            TAU,
            false,
        );
        report("seam_face_top_shifted", &run::<f64>(&[&shifted], eps), TAU);
        // band with circular hole of radius 0.3 at (u=1, v=1), rims at v=0 and v=2+0.4cos u.
        let b = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(0.0, 0.0, 0.0, 0.0),
            TAU,
            true,
        );
        let t = edge(
            chan(0.0, 0.0, 0.0, 1.0),
            chan(2.0, 0.4, 0.0, 0.0),
            TAU,
            false,
        );
        let hole = edge(
            chan(1.0, 0.3, 0.0, 0.0),
            chan(1.0, 0.0, 0.3, 0.0),
            TAU,
            false,
        );
        let exact = 2.0 * TAU - pi * 0.09;
        report(
            "band_with_hole",
            &run::<f64>(&[&[b], &[t], &[hole]], eps),
            exact,
        );
        report(
            "band_with_hole_iv",
            &run::<Interval>(&[&[b], &[t], &[hole]], eps),
            exact,
        );
        // hole first?  (outer loop index 0 is a winding rim; hole stored with the face's winding → should refuse)
        let bad = edge(
            chan(1.0, 0.3, 0.0, 0.0),
            chan(1.0, 0.0, 0.3, 0.0),
            TAU,
            true,
        );
        report(
            "band_with_hole_wrong_way",
            &run::<f64>(&[&[b], &[t], &[bad]], eps),
            exact,
        );
        // a band whose rims are split into two edges with the junction near the seam (u = 2π − 1e-9)
        let s = TAU - 1e-9;
        let b1 = edge(chan(0.0, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), s, true);
        let b2 = edge(chan(s, 0.0, 0.0, 1.0), chan(0.0, 0.0, 0.0, 0.0), 1e-9, true);
        let t1 = edge(chan(0.0, 0.0, 0.0, 1.0), chan(1.0, 0.0, 0.0, 0.0), s, false);
        let t2 = edge(
            chan(s - TAU, 0.0, 0.0, 1.0),
            chan(1.0, 0.0, 0.0, 0.0),
            1e-9,
            false,
        );
        report(
            "band_seam_junction",
            &run::<f64>(&[&[b1, b2], &[t1, t2]], eps),
            TAU,
        );
        report(
            "band_seam_junction_iv",
            &run::<Interval>(&[&[b1, b2], &[t1, t2]], eps),
            TAU,
        );
        // winding rim stored a period off for one edge
        let b2s = edge(
            chan(s + TAU, 0.0, 0.0, 1.0),
            chan(0.0, 0.0, 0.0, 0.0),
            1e-9,
            true,
        );
        report(
            "band_rim_edge_period_off",
            &run::<f64>(&[&[b1, b2s], &[t1, t2]], eps),
            TAU,
        );
    }

