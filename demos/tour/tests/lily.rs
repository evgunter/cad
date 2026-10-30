//! `demo_tour::lily`'s tests.

mod review_probes {
    use core::f64::consts::PI;
    use demo_tour::booleans::finished;
    use demo_tour::lily::*;
    use pncad::authoring::p2;
    use pncad::geom_core::Tol;
    use pncad::geom_core::{Point3, Vec3};
    use pncad::prelude::{Open, SurfaceKind};
    use pncad::profile::{ArcSweep, Center};
    use pncad::sweep::WedgeFrames;
    use pncad::topo::Surface;
    use pncad::topo::{Body, BooleanError, Operand};

    fn pieces() -> Vec<Piece<f64>> {
        plant::<f64>(Tol::witness())
    }

    fn body<'a>(ps: &'a [Piece<f64>], name: &str) -> &'a Body<f64> {
        &ps.iter().find(|p| p.name == name).expect("piece").body
    }

    /// A tube piece's two joint frames, as its revolve recorded them.
    fn caps<'a>(ps: &'a [Piece<f64>], name: &str) -> &'a WedgeFrames<f64> {
        ps.iter()
            .find(|p| p.name == name)
            .expect("piece")
            .caps
            .as_ref()
            .expect("a stem tube is a partial revolve, so it has caps")
    }

    /// The body's single stored torus carrier: (center, axis, R, r, u_ref).
    ///
    /// The half-band assertion below is what licenses the weld path's
    /// [`demo_tour::lily::torus_carrier`] to take the FIRST torus face it finds
    /// and call it the body's: this row is the check that there is
    /// only one carrier to find.
    fn torus(b: &Body<f64>) -> (Point3<f64>, Vec3<f64>, f64, f64, Vec3<f64>) {
        let mut found = None;
        for (_, f) in b.faces() {
            if let Some(Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }) = b.get_surface(f.surface)
            {
                let t = (*center, *axis, *major_radius, *minor_radius, *u_ref);
                if let Some(prev) = &found {
                    // Both torus half-bands must share ONE carrier, and
                    // that means EVERY component of it: a comparison
                    // that skips a component is a comparison two
                    // different tori can pass, which is exactly what
                    // this assertion exists to rule out.
                    let (pc, pa, pr, pm, pu): &(Point3<f64>, Vec3<f64>, f64, f64, Vec3<f64>) = prev;
                    assert!(
                        (*pc - t.0).norm() < 1e-15,
                        "two torus faces, two centers: {pc:?} vs {:?}",
                        t.0
                    );
                    assert!(
                        (*pa - t.1).norm() < 1e-15,
                        "two torus faces, two axes: {pa:?} vs {:?}",
                        t.1
                    );
                    assert!(
                        (*pu - t.4).norm() < 1e-15,
                        "two torus faces, two u_ref: {pu:?} vs {:?}",
                        t.4
                    );
                    assert!(
                        (pr - t.2).abs() < 1e-15 && (pm - t.3).abs() < 1e-15,
                        "two torus faces, two radius pairs: ({pr}, {pm}) vs ({}, {})",
                        t.2,
                        t.3
                    );
                } else {
                    found = Some(t);
                }
            }
        }
        found.expect("a torus wall")
    }

    /// Independently re-derived joint data (reviewer's own turtle
    /// algebra, computed outside this codebase — NOT lifted from
    /// [`Turtle`]): world (x, z) of the two stem joints and the unit
    /// tangents there.
    ///
    /// **GAP (LIB-U5): these stay literals, and the door that would
    /// retire them does not exist yet.** U5's `revolved_caps` answers
    /// where each tube's cap plane IS, which is what `assert_cap`
    /// below now asks — but "where does the PATH put its next joint,
    /// and with what tangent frame?" is a path-frame read-back, U4's
    /// deliverable. The producer here (`Turtle`) is demo-local, so
    /// there is no kernel choice to interrogate yet: these are an
    /// independent oracle for the frames the doors report, and under
    /// the U5 discriminator an independent derivation is a TEST, not
    /// a restatement to migrate. When U4's path-frame door lands,
    /// P1/T1/P2/T2/T3 become queries against it.
    const P1: (f64, f64) = (-0.3640807271660629, 1.87303296707956);
    const T1: (f64, f64) = (-0.374606593415912, 0.9271838545667874);
    const P2: (f64, f64) = (-2.4599453279967154, 1.2322628544225218);
    const T2: (f64, f64) = (0.20791169081775934, -0.9781476007338056);
    const T3: (f64, f64) = (0.17364817766693053, -0.984807753012208);
    /// The flower's globe centre: P2 (the arch's own last spine
    /// point, since the flower is welded there and not set back) plus
    /// the neck's drop plus `FLOWER_TOP` along T2.
    const SPHERE1_C: (f64, f64) = (-2.3668444700923885, 0.7942577551075498);

    /// The degeneracy floor for [`assert_two_cap_planes`], and the
    /// only number in this probe that is a THRESHOLD rather than a
    /// measurement.
    ///
    /// It sits between the two windows in play. Below it is
    /// `assert_cap`'s own arithmetic: 1e-12 on the point-in-plane
    /// residual and 1e-14 on the parallelism, so two frames closer
    /// together than those are literally one plane as far as that
    /// check can see, and its disjunction over the two would be a
    /// single test wearing a disjunction's clothes. Above it is the
    /// scene: the closest the lily's own cap pairs come is the arch's
    /// `sin(10°) ≈ 0.17` of normal angle (a 170° wedge), five orders
    /// clear. A floor here is therefore unreachable by float noise
    /// and nowhere near any real value — it fails only on an actual
    /// collapse, and it is not a second, weaker copy of the placement
    /// claim.
    const CAP_DISTINCT: f64 = 1e-6;

    /// A tube's two joint frames are two DISTINCT planes.
    ///
    /// This is the floor under [`assert_cap`]'s disjunction, and it is
    /// what the disjunction cannot supply for itself: a body whose two
    /// frames had collapsed onto one plane satisfies every
    /// single-target call trivially, because the existential can no
    /// longer tell "both ends are where they should be" from "the ends
    /// are the same plane and one of them happens to be right". The
    /// arch is incidentally covered — its two calls carry different
    /// targets, so a collapse would fail one of them — but the stem
    /// and the pedicel each get ONE call per free end and nothing else
    /// is watching.
    ///
    /// Two planes are the same plane when their normals are parallel
    /// AND they carry the same offset, so distinctness is the
    /// disjunction of the two ways out: a normal angle, or a
    /// separation along the normal. Either alone would be wrong — a
    /// 180° wedge has parallel caps at different offsets, and a
    /// closing wedge has near-coincident caps at a real angle.
    fn assert_two_cap_planes(caps: &WedgeFrames<f64>, what: &str) {
        let (a, b) = (caps.start, caps.end);
        let angle = a.axis.cross(b.axis).norm();
        let offset = (b.origin - a.origin).dot(a.axis).abs();
        assert!(
            angle > CAP_DISTINCT || offset > CAP_DISTINCT,
            "{what}: the two joint frames are ONE plane (normal angle {angle:.3e}, \
             offset {offset:.3e}) — every single-target cap assertion on this body \
             is vacuous until they separate"
        );
    }

    /// One of the tube's two JOINT FRAMES passes through world point
    /// `p` (xz-plane) with normal parallel to `t` — i.e. the tube's
    /// end tangent THERE is `t`.
    ///
    /// The frames come from `sweep::revolved_caps` (LIB-U5). Which of
    /// the two ends answers is the revolve's business, so both are
    /// offered — that
    /// is a two-element check against NAMED caps, not a search of the
    /// whole boundary. What the disjunction is NOT free to do is get
    /// easier as the caps converge, so
    /// [`assert_two_cap_planes`] runs first on every call.
    ///
    /// **The sign is part of the claim.** A cap frame's `axis` is the
    /// stored plane normal in CHART sense (`topo::readback::face_pose`
    /// is explicit that it is not corrected by the face's orientation
    /// sense), and `revolve`'s partial path builds the start cap's
    /// plane from the profile chain REVERSED and the end cap's from
    /// the same chain forward (`sweep::revolve::partial`, phases 1 and
    /// 4). The two therefore disagree by construction, in the one way
    /// that makes both of them point OUT of the solid: the start cap's
    /// normal is minus the tube's tangent there, the end cap's is plus
    /// it. That is a per-cap rule, not a per-call one — it says what
    /// each frame must be IF it is the one at this joint — so it costs
    /// the disjunction nothing, and it closes the hole where an
    /// anti-parallel normal read as "parallel to the tangent".
    fn assert_cap(caps: &WedgeFrames<f64>, p: (f64, f64), t: (f64, f64), what: &str) {
        assert_two_cap_planes(caps, what);
        let tv = Vec3::new(t.0, 0.0, t.1);
        let hit =
            [(caps.start, -1.0), (caps.end, 1.0)]
                .into_iter()
                .any(|(pose, outward): (_, f64)| {
                    let (o, n) = (pose.origin, pose.axis);
                    n.cross(tv).norm() < 1e-14
                        && outward * n.dot(tv) > 0.0
                        && (Point3::new(p.0, 0.0, p.1) - o).dot(n).abs() < 1e-12
                });
        assert!(
            hit,
            "{what}: neither joint frame passes through the joint with the joint \
             tangent, in the cap's own outward sense"
        );
    }

    /// The section's area: two circular segments on one chord, each
    /// `R²(θ − sin θ)/2` for its own radius `R` and turn `θ`.
    fn lance_area(l: Lance) -> f64 {
        segment_area(l, l.ridge) + segment_area(l, l.keel)
    }

    /// The section centroid's signed height above the chord (toward
    /// the ridge). A circular segment's centroid sits
    /// `4R·sin³(θ/2) / (3(θ − sin θ))` from its circle's centre, and
    /// that centre is `R·cos(θ/2)` on the far side of the chord.
    fn lance_centroid_rise(l: Lance) -> f64 {
        let moment = |h: f64| {
            let (r, th) = segment(l, h);
            let rise =
                4.0 * r * (0.5 * th).sin().powi(3) / (3.0 * (th - th.sin())) - r * (0.5 * th).cos();
            segment_area(l, h) * rise
        };
        (moment(l.ridge) - moment(l.keel)) / lance_area(l)
    }

    /// The circular segment between the chord and the arc rising `h`:
    /// `R²(θ − sin θ)/2`.
    fn segment_area(l: Lance, h: f64) -> f64 {
        let (r, th) = segment(l, h);
        0.5 * r * r * (th - th.sin())
    }

    /// The radius and turn of the arc through both margins rising `h`
    /// at the midrib.
    fn segment(l: Lance, h: f64) -> (f64, f64) {
        let c = l.width;
        (
            (0.25 * c * c + h * h) / (2.0 * h),
            4.0 * (2.0 * h / c).atan(),
        )
    }

    /// Claim: G1-by-construction, read from STORED geometry — and the
    /// stored geometry IS the world-coordinate intent, not a
    /// reconstruction of it. Nothing on the tube door's path goes
    /// profile → bulge → radius, so every quantity the caller handed
    /// in comes back bit-for-bit and is asserted with `==`: the
    /// turtle's ring centres and radials, the ring radii, and the tube
    /// radii. Only the DERIVED joint data (cap plane through a joint
    /// point, with the joint tangent as its normal) is windowed, and
    /// only because a cap frame is trigonometry away from the stored
    /// carrier.

    #[test]
    fn stem_joints_are_g1_in_the_stored_geometry() {
        let ps = pieces();
        let (stem, arch, pedicel) = (
            body(&ps, "lily_stem"),
            body(&ps, "lily_arch"),
            body(&ps, "lily_pedicel"),
        );
        // The stem: the turtle stands at the origin facing +z and
        // turns LEFT on a 5 m ring, so the centre is one ring radius
        // along the left normal −x̂ and the start radial is +x̂ — both
        // exact decimals, and both stored as such. The spine axis is
        // −ŷ because a left turn in the xz-plane is a right-handed
        // rotation about −ŷ.
        let (c, a, big_r, r, u) = torus(stem);
        assert_eq!((c.x, c.y, c.z), (-5.0, 0.0, 0.0), "stem ring centre");
        assert_eq!((a.x, a.y, a.z), (0.0, -1.0, 0.0), "stem spine axis");
        assert_eq!((u.x, u.y, u.z), (1.0, 0.0, 0.0), "stem u_ref");
        assert_eq!((big_r, r), (5.0, 0.060), "stem radii");
        // The arch and pedicel ring centres are turtle-walked, so the
        // literals are the reviewer's independent derivation (see the
        // GAP note) — but they are stored EXACTLY as the walk produced
        // them, so the radii, which are authored decimals, are `==`.
        let (c2, a2, big_r2, r2, _) = torus(arch);
        assert!((c2.x - -1.3839829671895292).abs() < 1e-12);
        assert!((c2.z - 1.460965714322057).abs() < 1e-12);
        assert_eq!((a2.x, a2.y, a2.z), (0.0, -1.0, 0.0), "arch spine axis");
        assert_eq!((big_r2, r2), (1.1, 0.052), "arch radii");
        let (_, a3, big_r3, r3, _) = torus(pedicel);
        assert_eq!((a3.x, a3.y, a3.z), (0.0, -1.0, 0.0), "pedicel spine axis");
        assert_eq!((big_r3, r3), (0.42, 0.032), "pedicel radii");
        // Joint 0: the stem's FREE end. The turtle stands at the
        // origin facing +z (`plant`'s `root`), so unlike every joint
        // below this frame is authored and exact rather than walked —
        // no literal to derive, and no reason for the stem's start to
        // be the one frame in the chain nothing looks at.
        assert_cap(caps(&ps, "lily_stem"), (0.0, 0.0), (0.0, 1.0), "stem start");
        // Joint 1: stem end / arch start share point P1 and tangent T1.
        assert_cap(caps(&ps, "lily_stem"), P1, T1, "stem end");
        assert_cap(caps(&ps, "lily_arch"), P1, T1, "arch start");
        // Joint 2: arch end at P2 with tangent T2 (the flower hangs here).
        assert_cap(caps(&ps, "lily_arch"), P2, T2, "arch end");
        // The fork reuses P1; the pedicel's START tangent is the
        // authored (cos150, sin150), not T1 — branch, not continuation.
        assert_cap(
            caps(&ps, "lily_pedicel"),
            P1,
            (f64::cos(deg(150.0)), f64::sin(deg(150.0))),
            "pedicel start",
        );
    }

    /// Claim: the lantern axes ARE the stem tangents, and the globe
    /// sits at attach + (neck drop + top)·dir — read off the stored
    /// sphere and the two cones (neck and pucker), not off the
    /// construction code.
    #[test]
    fn lantern_axes_are_the_stored_stem_tangents() {
        let ps = pieces();
        // The bud is three partial revolves on three tilted axes, so
        // its claim is a different one and lives in
        // `the_buds_three_axes_form_the_authored_tripod`.
        for (name, t, cen, rad) in [("lily_lantern", T2, SPHERE1_C, FLOWER_GLOBE)] {
            let b = body(&ps, name);
            let tv = Vec3::new(t.0, 0.0, t.1);
            let mut saw_sphere = false;
            let mut cones = 0usize;
            for (_, f) in b.faces() {
                match b.get_surface(f.surface) {
                    Some(Surface::Sphere {
                        center,
                        radius,
                        axis,
                        ..
                    }) => {
                        saw_sphere = true;
                        assert!(
                            axis.cross(tv).norm() < 1e-14,
                            "{name}: sphere axis || tangent"
                        );
                        assert!((center.x - cen.0).abs() < 1e-12, "{name} center.x");
                        assert!(center.y.abs() < 1e-15, "{name} center.y");
                        assert!((center.z - cen.1).abs() < 1e-12, "{name} center.z");
                        assert!((radius - rad).abs() < 1e-15, "{name} radius");
                    }
                    Some(Surface::Cone { axis, .. }) => {
                        cones += 1;
                        assert!(
                            axis.cross(tv).norm() < 1e-14,
                            "{name}: cone axis || tangent"
                        );
                    }
                    _ => {}
                }
            }
            assert!(
                saw_sphere && cones == 4,
                "{name}: sphere zone between a neck cone and a conical pucker, \
                 both coaxial with the stem tangent and both halved at the \
                 revolve seam — saw {cones} cone faces"
            );
        }
    }

    /// **The weld's door sequence, measured** — VERBS-LILYWELD PR-2's
    /// opening measurement, carried past the cone's admission to the
    /// boolean.
    ///
    /// The lantern's throat disk against the arch's end cap is an exact
    /// coincident planar Rest pair, which the union decides on its
    /// zero. Past the operand gate (the cone is on its roster) and F7,
    /// the union stops at the crossing layer on the WELD CIRCLE itself:
    /// the lantern's neck rim, which lies on the arch's torus wall as
    /// its terminal meridian circle — a coincidence the circle × torus
    /// lane does not decide. This row is what shows it moving when a
    /// coincidence door learns the circle.
    #[test]
    fn the_weld_refuses_on_the_weld_circle() {
        let tol = Tol::witness();
        let ps = pieces();
        let (lant, arch) = (body(&ps, "lily_lantern"), body(&ps, "lily_arch"));
        let (lant, arch) = (
            &finished("lily_lantern", lant.clone(), tol),
            &finished("lily_arch", arch.clone(), tol),
        );
        let refused = pncad::topo::union(lant, arch, tol).expect_err("the weld still refuses");
        println!("weld: {refused:?}");
        let BooleanError::CurvedPierceUnsupported {
            operand: Operand::A,
            face,
            edge,
            ..
        } = refused
        else {
            panic!("the weld stops at the curved pierce: {refused:?}");
        };
        let carrier = lant
            .get_edge(edge)
            .and_then(|e| lant.get_curve_geom(e.curve))
            .and_then(|g| g.certified())
            .map(|c| c.carrier().clone());
        let kind = arch
            .get_face(face)
            .and_then(|f| arch.get_surface(f.surface))
            .map(|s| s.kind());
        assert!(
            matches!(
                carrier,
                Some(pncad::geom::Curve3::Circle { radius, .. }) if (radius - ARCH_R).abs() < 1e-12
            ) && kind == Some(SurfaceKind::Torus),
            "the refused edge is the weld circle, on the arch's torus wall: \
             {carrier:?} on {kind:?}"
        );
    }

    /// **The lantern's caps arrive whole** — the shape the F7 rule
    /// used to call a defect, gone at its source.
    ///
    /// The lantern has two PLANAR caps that touch the axis, the LIP
    /// disk and the THROAT disk. A full revolve used to cut each at its
    /// two seam meridians into two half-faces on one plane key, which
    /// `merge_coplanar_faces` then repaired; it now sweeps a planar
    /// wall whole (BAND, "one wall per run"), so neither cap carries a
    /// same-key adjacency. Its CURVED walls (two cones, one sphere
    /// zone) are still cut at the seam, and the row tells the two
    /// apart — no planar pair, six curved — so a change that stopped
    /// doing so fails here rather than silently. The body as built is
    /// tier 2 and tier 3 clean (the delta review's MIN-1 asked for
    /// both real validators, not the tier 1 one).
    #[test]
    fn the_lanterns_caps_arrive_whole() {
        let tol = Tol::witness();
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let mut planar_pairs = 0usize;
        let mut curved_pairs = 0usize;
        for (k, _) in lant.edges() {
            let Ok(sides) = pncad::topo::readback::edge_sides(lant, k) else {
                continue;
            };
            if sides.plus.face == sides.minus.face || sides.plus.surface != sides.minus.surface {
                continue;
            }
            match lant.get_surface(sides.plus.surface) {
                Some(Surface::Plane { .. }) => planar_pairs += 1,
                Some(_) => curved_pairs += 1,
                None => {}
            }
        }
        // Each pair is counted from both of its two struts.
        assert_eq!(
            (planar_pairs, curved_pairs),
            (0, 6),
            "two whole planar caps (the lip disk and the throat disk) and three \
             curved walls (two cones, one sphere zone), each split at its seam"
        );
        assert_eq!(
            pncad::topo::validate_closed(lant),
            Ok(()),
            "tier 2 as built"
        );
        assert_eq!(
            pncad::topo::validate_geometric(lant, tol),
            Ok(()),
            "tier 3 as built"
        );
    }

    /// **The weld, as geometry.** The flower's neck circle and the
    /// arch tube's terminal meridian circle are ONE circle, each
    /// computed to closed form off its own body's stored carrier
    /// (`weld_circle`, which the scene also runs live). Checked here
    /// as well, for the reason the other boolean walls are: a scene
    /// claim must not cost a whole render pass to re-measure.
    ///
    /// The circle is then said a third way, against the reviewer's
    /// independent turtle algebra: it is centred on P2 with normal
    /// T2 and radius exactly the arch tube's. That is the whole
    /// content of #1059 — the lantern's axis is the stem's own
    /// tangent, so the cone cut at the tube's minor radius meets the
    /// meridian circle at that station exactly.
    #[test]
    fn the_flower_and_the_arch_share_one_circle() {
        let ps = pieces();
        let (_, circle, res, runner_up) = weld_circle(
            body(&ps, "lily_lantern"),
            body(&ps, "lily_arch"),
            caps(&ps, "lily_arch"),
        );
        println!(
            "weld circle: centre {:?}, r {}, normal {:?}; meridian residuals \
             (off-spine, radius — definitional — normal) {res:?}; nearest \
             DISTINCT station circle (the neck cone's own other nappe) misses \
             by {runner_up:e}",
            circle.c, circle.r, circle.n
        );
        assert_eq!(circle.r, ARCH_R, "the weld circle is the tube's own");
        assert!(
            (circle.c.x - P2.0).abs() < 1e-12 && (circle.c.z - P2.1).abs() < 1e-12,
            "the weld circle is centred on the arch's last spine point"
        );
        assert!(
            circle.c.y.abs() < 1e-15,
            "the weld circle's centre is in the plant's own plane"
        );
        assert!(
            circle.n.cross(Vec3::new(T2.0, 0.0, T2.1)).norm() < 1e-14,
            "the weld circle's normal is the stem tangent there"
        );
    }

    /// **A live refusal, banked where it was met.** The neck a
    /// derivation would pick is the globe's own TANGENT cone at the
    /// truncation circle — half-angle `atan(top / r_top)`, a G1
    /// shoulder, apex at the truncation plane's pole in the sphere.
    /// The authoring algebra will not take it: a leg whose departure
    /// is spelled in COORDINATES and lands within ε of the incoming
    /// tangent is `PathError::JunctionTangent`, and the refusal names
    /// its one recourse — say the tangency STRUCTURALLY, with the
    /// `.tangent()` verb, which makes it exact by construction rather
    /// than by arithmetic that happens to agree.
    ///
    /// So this is not a gap: it is the algebra declining to infer an
    /// intent it has a door for. The scene does not walk through that
    /// door because it does not want a G1 shoulder
    /// ([`FLOWER_NECK_HALF_ANGLE`]); the refusal is pinned here so
    /// that reasoning stays checkable and the margin stays visible.
    #[test]
    fn the_globes_tangent_cone_neck_is_refused_by_the_junction_gate() {
        let tol = Tol::witness();
        let (globe, top) = (FLOWER_GLOBE, FLOWER_TOP);
        let r_top = (globe.powi(2) - top.powi(2)).sqrt();
        // The tangent cone's own half-angle, and the drop that follows.
        let tangent_alpha = (top / r_top).atan();
        let drop = neck_drop(globe, top, ARCH_R, tangent_alpha);
        let out = p2::<f64>(0.0, 0.0);
        let refusal = Open
            .at(out)
            .line_to(p2(ARCH_R, 0.0), tol)
            .expect("throat disk")
            .line_to(p2(r_top, drop), tol)
            .expect("the tangent neck's own leg authors fine")
            .arc_to(
                Center {
                    c: p2(0.0, drop + top),
                    winding: ArcSweep::Ccw,
                    p: p2((globe.powi(2) - 0.36_f64.powi(2)).sqrt(), drop + top + 0.36),
                },
                tol,
            )
            .expect_err("a tangent belly departure is refused");
        println!("the tangent-cone neck answers {refusal:?}");
        assert!(
            matches!(
                refusal,
                pncad::profile::PathError::JunctionTangent { margin, arm }
                    if margin.abs() < 1e-15 && arm > 0.1
            ),
            "the tangent neck must refuse as JunctionTangent with a vanishing \
             turn margin on a real lever arm: {refusal:?}"
        );
    }

    /// The re-authored lantern's CENSUS and its exact mass, against
    /// the closed form of the solid of revolution it is.
    ///
    /// The census is what the re-authoring changed structurally: the
    /// meridian gained one segment (the neck cone) between the throat
    /// disk and the belly, and a FULL revolve emits every CURVED wall as
    /// two half-bands on one carrier, so the neck arrives as two faces
    /// with the seam struts and rim vertices that go with them. Its two
    /// planar caps it sweeps whole (BAND, one wall per run: 10 faces,
    /// 18 edges, 10 vertices became 8, 14, 8). The
    /// exact volume is the same closed form
    /// `finding_13_tessellation_table_reproduces` measures the mesh
    /// against, taken here from the kernel's own `mass_properties`
    /// door rather than from a tessellation — so the two rows are
    /// independent readings of one number.
    #[test]
    fn the_lanterns_census_and_mass_are_the_re_authored_ones() {
        let ps = pieces();
        let lant = body(&ps, "lily_lantern");
        let census = (
            lant.shells().count(),
            lant.faces().count(),
            lant.edges().count(),
            lant.vertices().count(),
        );
        println!("lantern census (shells, faces, edges, vertices) = {census:?}");
        // RE-BASELINED (wall 6 retired: the mouth rolled into one
        // torus band): (1, 8, 14, 8) -> (1, 9, 17, 10), one band face
        // between the zone and the pucker, its two tangency circles
        // each two half-arcs, and their seam vertices.
        assert_eq!(census, (1, 9, 17, 10), "the re-authored lantern's census");
        let props = pncad::topo::mass_properties(
            &demo_tour::booleans::at_rest(lant, Tol::witness()),
            Tol::witness(),
            demo_tour::printed(),
        )
        .expect("mass properties");
        // The literal, DELIBERATELY: this row and
        // `finding_13_tessellation_table_reproduces` must not agree by
        // sharing an expression. That row builds the closed form from
        // the profile's own constants and compares a MESH against it;
        // this one compares the kernel's `mass_properties` door
        // against a transcribed number, so a change to the closed-form
        // expression cannot move both readings together.
        // RE-BASELINED (wall 6 retired): 0.36455193285177373 for the
        // sharp lantern, plus the 1.14119807294e-5 the concave mouth
        // roll adds at r = 0.25 (`mouth_roll_delta_v`, by Pappus).
        let exact = 0.36456334483250313;
        assert!(
            (props.enclosure.volume - exact).abs() < 1e-12,
            "lantern volume {} vs the closed form {exact}",
            props.enclosure.volume
        );
        assert_eq!(
            props.enclosure.volume_pad, 0.0,
            "every lantern face is closed-form"
        );
    }

    /// The volume the mouth's roll ADDS ([`roll_the_mouth`]): the
    /// meridian region between the globe's arc, the pucker's line and
    /// the rolling ball's arc, revolved. The crease is concave, so the
    /// ball rides OUTSIDE both supports: its centre is `globe + roll`
    /// from the globe centre and `roll` off the pucker's line on the
    /// side away from the material. By Green's theorem Pappus's
    /// `2π∬ρ dA` is `π∮ρ² dy` round that region's three sides, the
    /// snowman's waist band ([`demo_tour::snowman::arc_rho2_dy`]) with one
    /// arc a line. Coordinates are the sketch's `(ρ, t)` about the
    /// globe centre; volume does not care where along the axis that is.
    fn mouth_roll_delta_v(globe: f64, mouth: f64, lip_r: f64, lip_drop: f64, roll: f64) -> f64 {
        let m = ((globe * globe - mouth * mouth).sqrt(), mouth);
        let l = (lip_r, mouth + lip_drop);
        let len = (l.0 - m.0).hypot(l.1 - m.1);
        let u = ((l.0 - m.0) / len, (l.1 - m.1) / len);
        // The side away from the axis, the material's outside.
        let n = (u.1, -u.0);
        let a = (m.0 + n.0 * roll, m.1 + n.1 * roll);
        // The ball's centre is `a + s·u` at `globe + roll` from the
        // globe centre: the larger root of a quadratic in s.
        let b = 2.0 * (a.0 * u.0 + a.1 * u.1);
        let c = a.0 * a.0 + a.1 * a.1 - (globe + roll).powi(2);
        let s = (-b + (b * b - 4.0 * c).sqrt()) / 2.0;
        let centre = (a.0 + u.0 * s, a.1 + u.1 * s);
        let on_globe = (
            centre.0 * globe / (globe + roll),
            centre.1 * globe / (globe + roll),
        );
        let on_pucker = (centre.0 - n.0 * roll, centre.1 - n.1 * roll);
        let line =
            |p: (f64, f64), q: (f64, f64)| (q.1 - p.1) * (p.0 * p.0 + p.0 * q.0 + q.0 * q.0) / 3.0;
        (PI * (line(m, on_pucker)
            + demo_tour::snowman::arc_rho2_dy(centre, roll, on_pucker, on_globe)
            + demo_tour::snowman::arc_rho2_dy((0.0, 0.0), globe, on_globe, m)))
        .abs()
    }

    /// Finding 13 re-measured: one chord budget spends wildly
    /// differently per body, and these are the numbers.
    ///
    /// The five analytic rows are the SAME counts the sketch-frame
    /// revolve produced — the tube door changed which parameters are
    /// stored, not which torus they describe, so the tessellator sees
    /// the same surface and splits it the same way. The two SWEPT
    /// blade rows are the other half of the finding, and it runs the
    /// other way: a swept lens skin costs about six and a half times the
    /// stem tube at the same δ (5_374 against 828 at 2e-3). Part of that
    /// is geometry — the lens is curved across, where a straight-sided
    /// section is flat — and part is the sizing lane: a rational wall
    /// is sized per cell of the 16 × 16 refinement its certified bound
    /// is assembled on, and that per-cell grid builds more cells than
    /// the whole-patch schedule would
    /// (`work/tess/rational-wall-cell-grid-costs-more-than-the-whole-patch-schedule.md`).
    /// A torus spends the chord budget per curvature, through
    /// `mesh::sizing::torus_grid_steps`; a rational skin, per cell.
    ///
    /// The LOFTED bodies are deliberately absent from this table. A
    /// loft's wall count and knot structure follow the section list
    /// and the station count rather than one profile, so pinning their
    /// triangle counts here would pin the demo's proportions, not a
    /// tessellation finding — and those proportions are chosen to look
    /// like a plant and will be re-chosen. What IS pinned about them
    /// is geometric: see the taper/roll and sepal-tangency tests.
    #[test]
    fn finding_13_tessellation_table_reproduces() {
        use pncad::mesh::validate::{signed_volume, triangle_count};
        let ps = pieces();
        // RE-BASELINED (PRED, `[mesh.sizing]`): each mesher lane sizes
        // its worst triangle at δ by its own theorem rather than at δ/2,
        // so the stem's and arch's tori and the lantern's sphere zone
        // and cones and its mouth's torus band coarsen — stem
        // 392 -> 188 and 828 -> 476, arch 2_960 -> 1_472, lantern
        // 1_220 -> 832 and 3_038 -> 1_900 (its cones take no rows, its
        // sphere zone sizes its azimuth at its widest latitude). The
        // leaves are NURBS, whose lattice did not move.
        let table = [
            ("lily_stem", 5e-3, 188usize),
            ("lily_stem", 2e-3, 476),
            ("lily_arch", 2e-3, 1_472),
            // RE-BASELINED (BAND, one wall per run): a full revolve
            // sweeps the lantern's two planar caps whole, and each cap
            // as one disc meshes two triangles fewer than as two
            // half-discs — 1_084 -> 1_080 and 2_560 -> 2_556.
            // RE-BASELINED (wall 6 retired): the mouth's torus band,
            // sized per curvature like any torus, adds its own cells —
            // 1_080 -> 1_220 and 2_556 -> 3_038.
            ("lily_lantern", 5e-3, 832),
            ("lily_lantern", 2e-3, 1_900),
            // RE-BASELINED: the swept leaves are fitted at the cubic
            // degree along the path (wall 15 retired) — 6_468 -> 5_576
            // and 5_992 -> 5_164. RE-BASELINED again (PR 4984, one
            // second-order quotient for rational bounds): the blade
            // face's certified second-derivative sups tighten (`muu`
            // 0.138 -> 0.126, `muv` 0.043 -> 0.023 on leaf b), so its
            // per-cell grid drops 96 cells — 5_576 -> 5_374 and
            // 5_164 -> 4_962. The sampled |S − Π| stays under 8.5e-5
            // against the face's 1e-3. RE-BASELINED (#5531, the
            // trimmed lane's CDT in the lattice metric): leaf b's blade
            // face no longer snaps its bands to the patch column count
            // (71 snapped bands -> 0: their aspects are malign in raw
            // UV and benign over the face's metric), so each band keeps
            // its own columns and the grid drops 1_152 -> 1_081 cells —
            // 5_374 -> 5_254. Leaf c measures unchanged.
            ("lily_leaf_b", 2e-3, 5_254),
            ("lily_leaf_c", 2e-3, 4_962),
        ];
        // Measured first, compared once: a row-at-a-time assert stops
        // at the first move and hides the rest, and this table is read
        // as a whole.
        let got: Vec<(&str, f64, usize)> = table
            .iter()
            .map(|&(name, delta, _)| {
                let m = pncad::mesh::tessellate(body(&ps, name), delta, Tol::witness())
                    .expect("tessellate");
                (name, delta, triangle_count(&m))
            })
            .collect();
        println!("finding 13, measured: {got:?}");
        assert_eq!(got, table.to_vec(), "the finding-13 tessellation table");
        // Lantern volume error at both deltas. The exact figure is the
        // authored solid of revolution in closed form — the zone
        // integral between the two truncations, plus the two conical
        // frusta (the NECK above it and the pucker below) — so it is
        // derived from the same numbers the profile is drawn from
        // rather than transcribed:
        //   π[r²(a+b) − (a³+b³)/3] + π·h(R² + Rρ + ρ²)/3 per frustum,
        // plus the material the mouth's roll adds
        // ([`mouth_roll_delta_v`]). It comes to 0.36456334483250313 m³.
        let (globe, top, mouth, lip_r, lip_drop): (f64, f64, f64, f64, f64) =
            (FLOWER_GLOBE, FLOWER_TOP, 0.36, 0.09, 0.16);
        let r_top = (globe.powi(2) - top.powi(2)).sqrt();
        let r_mouth = (globe.powi(2) - mouth.powi(2)).sqrt();
        let frustum = |h: f64, r0: f64, r1: f64| PI * h * r0.mul_add(r1, r0 * r0 + r1 * r1) / 3.0;
        let exact = PI * (globe.powi(2) * (top + mouth) - (top.powi(3) + mouth.powi(3)) / 3.0)
            + frustum(
                neck_drop(globe, top, ARCH_R, FLOWER_NECK_HALF_ANGLE),
                ARCH_R,
                r_top,
            )
            + frustum(lip_drop, r_mouth, lip_r)
            + mouth_roll_delta_v(globe, mouth, lip_r, lip_drop, FLOWER_MOUTH_ROLL);
        let rels: Vec<(f64, f64)> = [5e-3, 2e-3]
            .into_iter()
            .map(|delta| {
                let m = pncad::mesh::tessellate(body(&ps, "lily_lantern"), delta, Tol::witness())
                    .expect("tessellate");
                (delta, ((signed_volume(&m) - exact) / exact).abs())
            })
            .collect();
        println!("lantern volume error, measured: {rels:?}");
        // RE-BASELINED (PRED, `[mesh.sizing]`): the rolled lantern
        // measured 0.012769 and 0.0050019 at the δ/2 sizing and measures
        // 0.015945 and 0.0062491 now — the chords' inscribed deficit
        // grows with the coarser grid, inside the `area·δ` a mesh
        // within δ allows.
        for ((delta, rel), (lo, hi)) in rels.into_iter().zip([(0.0153, 0.0167), (0.0060, 0.0067)]) {
            assert!(rel > lo && rel < hi, "lantern @ {delta:e}: rel {rel}");
        }
        // A swept blade has no analytic wall to compare against, but it
        // has PAPPUS. A rigid section carried in the path's normal
        // frame sweeps A·(centroid arc length). The lens's area and
        // centroid are its two circular segments' closed forms
        // ([`lance_area`], [`lance_centroid_rise`]), and the centroid
        // sits that far above the chord, i.e. OUTSIDE the spine's
        // centre of curvature, so its arc is len + |curl|·rise.
        //
        // Pappus is the IDEAL sweep's volume, and the blade is a skin
        // through [`LEAF_STATIONS`] sections on a cubic interpolant of
        // the arc, so the two differ by the fit. The kernel's certified
        // volume must therefore contain the skin's OWN volume,
        // integrated over its surfaces
        // ([`demo_tour::patch_quadrature`]), at every width: a
        // sound enclosure misses the true volume at none. Pappus is
        // held to the skin at the fit's scale: a cubic spline through
        // stations `Δ` apart in the arc's angle misses it by
        // `(5/384)·Δ⁴` of its own scale, and a section's area is
        // quadratic in its scale, so the volume misses by about twice
        // that. And the mesh must fall SHORT of the skin by between
        // 3e-3 and 6e-3: every chord across the convex lens cuts
        // inside it, and the inscribed deficit measured 4.4e-3 and
        // 5.0e-3 at δ = 2e-3. A mesh that met the skin would not have
        // been measured off a real tessellation; one further off would
        // mean the section rolled about the tangent on its way down
        // the path.
        //
        // `lily_leaf_a` is NOT in this list, and its absence is the
        // point: it is the lofted blade, and it both tapers and rolls.
        // Pappus wants a rigid section carried in the normal frame,
        // which is exactly what a loft stops being. What pins the
        // lofted blade instead is
        // `the_lofted_blade_tapers_and_rolls_in_the_stored_geometry`.
        for (name, leaf) in [("lily_leaf_b", LEAF_B), ("lily_leaf_c", LEAF_C)] {
            let pappus = lance_area(leaf.section)
                * leaf
                    .curl
                    .abs()
                    .mul_add(lance_centroid_rise(leaf.section), leaf.len);
            let b = body(&ps, name);
            // The caps are the authored lens placed rigidly, and each
            // reading is taken about two points: a shell that does not
            // close reads two volumes.
            let cap = |_o: [f64; 3], _n: [f64; 3]| lance_area(leaf.section);
            let about = [[0.0; 3], [leaf.base.x, leaf.base.y, leaf.base.z]];
            let coarse = demo_tour::patch_quadrature::measure(b, 24, &about, cap);
            let fine = demo_tour::patch_quadrature::measure(b, 32, &about, cap);
            let skin_v = fine.volume[0];
            let skin_err = (skin_v - coarse.volume[0])
                .abs()
                .max((skin_v - fine.volume[1]).abs());
            #[allow(clippy::cast_precision_loss)]
            let fit_step = leaf.curl.abs() / (LEAF_STATIONS - 1) as f64;
            let fit_scale = 2.0 * (5.0 / 384.0) * fit_step.powi(4);
            let fit = ((skin_v - pappus) / pappus).abs();
            assert!(
                fit <= fit_scale,
                "{name}: the skin's V = {skin_v} misses Pappus {pappus} by {fit:e} of it, past \
                 the cubic fit's scale {fit_scale:e}"
            );
            let e = pncad::topo::validate_geometric_certificate(b, Tol::witness())
                .expect("the swept leaf is tier 3 clean")
                .measure(demo_tour::printed())
                .unwrap_or_else(|refusal| panic!("{name}: no certified volume: {refusal}"))
                .enclosure
                .enclosure();
            let (lo, hi) = (e.volume_lo, e.volume_hi);
            assert!(
                lo <= skin_v - skin_err && skin_v + skin_err <= hi,
                "{name}: the skin's own V = {skin_v} (± {skin_err:e}) outside the certified \
                 [{lo}, {hi}]"
            );
            let m = pncad::mesh::tessellate(b, 2e-3, Tol::witness()).expect("tessellate");
            let short = (skin_v - signed_volume(&m)) / skin_v;
            assert!(
                short > 3e-3 && short < 6e-3,
                "{name}: mesh short by {short}"
            );
        }
    }

    /// The lofted blade's two claims — TAPER and ROLL — read off the
    /// STORED body, not off the construction code.
    ///
    /// Both are read through the body's two planar cap faces, which
    /// are the loft's end sections placed in the world. For each cap:
    /// its two vertices, the MARGINS, whose chord is the width and
    /// whose direction is the width axis `u`, and `v = n x u`.
    ///
    /// The section's ridge/keel ASYMMETRY is what makes the roll
    /// measurable at all: `v`'s sign is otherwise a coin flip, because
    /// a `u` read off two margins is only defined up to sign. The
    /// ridge is authored strictly deeper than the keel, so the side of
    /// the chord the deeper rim's apex lies on picks the orientation
    /// out uniquely, and the angle from the base frame to the tip frame
    /// is then a full signed turn rather than a turn mod pi.
    #[test]
    fn the_lofted_blade_tapers_and_rolls_in_the_stored_geometry() {
        let ps = pieces();
        let b = body(&ps, "lily_leaf_a");
        let caps = cap_frames(b);
        // The wide cap is the root's, where the sheath carries the
        // blade's base section into the stem unchanged, so it reads
        // the base lens; the TIP cap is the narrow one.
        let (base, tip) = if caps[0].width > caps[1].width {
            (&caps[0], &caps[1])
        } else {
            (&caps[1], &caps[0])
        };
        // TAPER. The authored base lens is 0.170 margin to margin and
        // the tip lens 0.060. A sweep could produce neither number from
        // the other — one profile goes down the path.
        assert!(
            (base.width - 0.170).abs() < 1e-9,
            "base cap width {}",
            base.width
        );
        assert!(
            (tip.width - 0.060).abs() < 1e-9,
            "tip cap width {}",
            tip.width
        );
        // ROLL, isolated. Comparing the blade's own two ends does NOT
        // measure the roll: the caps live in different planes (the
        // spine turns through `curl` between them), so any angle read
        // across them mixes the roll with the spine's own turn. That
        // vanishes if the comparison is made
        // against the SAME blade built with the twist set to zero:
        // identical spine, identical stations, identical sections, so
        // the two tip caps are coplanar and their frames differ by the
        // roll and nothing else.
        let untwisted = leaf_a::<f64>(
            Plan {
                twist: 0.0,
                ..leaf_a_plan()
            },
            Tol::witness(),
        );
        let flat = cap_frames(&untwisted);
        let flat_tip = if flat[0].width > flat[1].width {
            &flat[1]
        } else {
            &flat[0]
        };
        assert!(
            flat_tip.n.cross(tip.n).norm() < 1e-12,
            "the twin's tip cap must be coplanar with the blade's"
        );
        // The SIGN is part of the claim, not an accident: the
        // ridge/keel asymmetry above fixes each `v` uniquely, so this
        // is a full signed turn and a MIRROR-rolled blade (twist
        // -160) must not pass. Asserting `turn.abs()` would let one
        // through; the authored twist is +160, and that is what the
        // stored geometry has to say.
        let turn = signed_angle(flat_tip.v, tip.v, tip.n);
        let want = deg(160.0);
        assert!(
            (turn - want).abs() < 1e-9,
            "blade roll {turn} rad, wanted {want} (the authored twist, SIGN included)"
        );
    }

    /// The long leaf grows from the middle of the lower stem, read off
    /// both stored bodies: the chord midpoint of its root cap, the
    /// spine's first point, lies on the stem tube's core circle, and its
    /// two margins stand outside the tube, so the base clasps the stem
    /// with a margin showing either side. Wall 8 unions the two as they
    /// stand.
    #[test]
    fn the_long_leaf_grows_from_the_middle_of_the_stem() {
        let ps = pieces();
        let caps = cap_frames(body(&ps, "lily_leaf_a"));
        let root = if caps[0].width > caps[1].width {
            &caps[0]
        } else {
            &caps[1]
        };
        let (c, axis, ring, tube, _) = torus(body(&ps, "lily_stem"));
        let off_core = |p: Point3<f64>| {
            let w = p - c;
            let h = w.dot(axis);
            ((w - axis * h).norm() - ring).hypot(h)
        };
        let mid = root.margins[0] + (root.margins[1] - root.margins[0]) * 0.5;
        assert!(
            off_core(mid) < 1e-9,
            "the leaf's root is {:e} off the stem's axis",
            off_core(mid)
        );
        for m in root.margins {
            assert!(
                off_core(m) > tube,
                "a root margin {} from the stem's axis is inside its {tube} tube",
                off_core(m)
            );
        }
    }

    /// The sepals' TANGENCY claim, checked against the globe the
    /// lantern actually stores: every margin and every cap rim apex of
    /// every sepal is at least `FLOWER_GLOBE` from the globe centre, so
    /// no sepal enters
    /// the flower. The doc comment on [`sepals`] argues this for the
    /// SPINE; this measures it on the built solid, section thickness
    /// and all.
    #[test]
    fn the_sepals_stand_outside_the_globe_they_are_tangent_to() {
        let ps = pieces();
        // The globe centre, re-derived from the lantern's own stored
        // sphere rather than from the plant's construction.
        let lant = body(&ps, "lily_lantern");
        let mut centre = None;
        for (_, f) in lant.faces() {
            if let Some(pncad::geom::Surface::Sphere { center, radius, .. }) =
                lant.get_surface(f.surface)
            {
                assert!((radius - FLOWER_GLOBE).abs() < 1e-12);
                centre = Some(*center);
            }
        }
        let g = centre.expect("the lantern stores its globe");
        let mut closest = f64::INFINITY;
        for name in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"] {
            let s = body(&ps, name);
            // The margins, and the apex of each cap's two rims: the
            // keel's apex is the section's deepest point.
            let mut points: Vec<Point3<f64>> = s.vertex_points().map(|(_, p)| p).collect();
            for (_, f) in s.faces() {
                if let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                    s.get_surface(f.surface)
                {
                    let margins: Vec<Point3<f64>> = s
                        .vertex_points()
                        .map(|(_, p)| p)
                        .filter(|p| (*p - origin).dot(normal).abs() < 1e-9)
                        .collect();
                    let mid = margins[0] + (margins[1] - margins[0]) * 0.5;
                    points.extend(rim_apices(s, origin, normal, mid));
                }
            }
            for p in points {
                let d = p.distance(g);
                // 1e-12, not 0: the base keel apex is placed AT the
                // sphere by construction, and it gets there through a
                // rotation composed into the section placement, so it
                // lands within an ulp or two of R rather than on it.
                // Anything that actually entered the flower would be
                // inside by a section thickness, six orders of
                // magnitude past this.
                assert!(
                    d >= FLOWER_GLOBE - 1e-12,
                    "{name}: a rim point is {d} from the globe centre, inside R = {FLOWER_GLOBE}"
                );
                closest = closest.min(d);
            }
        }
        // Two-sided: the sepals must GRAZE, not merely miss. The base
        // section's keel is authored 0.008 and the stand-off is
        // R + keel, so the nearest rim point sits at R + 0 (the keel's
        // apex, on the sphere) — within the float noise of a rotation
        // composed through the placement.
        //
        // 1e-12, the SAME window as the inside bound above, so the
        // pair is a symmetric ±1e-12 collar on R and the claim "the
        // nearest sepal rim point is within 1e-12 of the globe" is the
        // literal assertion rather than a summary of it.
        println!(
            "sepal tangency: nearest rim point {:e} from R",
            closest - FLOWER_GLOBE
        );
        assert!(
            closest < FLOWER_GLOBE + 1e-12,
            "nearest sepal rim point {closest} — tangency claimed, {} of clearance found",
            closest - FLOWER_GLOBE
        );

        // And the OTHER flower. Tangency to the globe a sepal stands
        // on says nothing about the bud 1.44 m up the stem, and the
        // kernel will not say anything either: these are separate
        // bodies, this scene joins none of them (its one union is the
        // rootstock, underground), so one solid passing
        // through another is not a condition any operation here could
        // refuse. It is the SCENE's invariant, so the scene tests it.
        // Sepal 0's radial, unphased, points almost exactly at the bud
        // and a 1.05 m sepal reaches 0.027 m inside it — this assert
        // is what caught that, and reds if the phase is dropped.
        for seg in ["lily_bud_a", "lily_bud_b", "lily_bud_c"] {
            let (bc, br, _) = sphere_of(body(&ps, seg));
            for name in ["lily_sepal_a", "lily_sepal_b", "lily_sepal_c"] {
                let sb = body(&ps, name);
                for (_, p) in sb.vertex_points() {
                    let d = p.distance(bc);
                    assert!(
                        d > br,
                        "{name}: a vertex is {d} inside {seg}'s globe R = {br}"
                    );
                }
            }
        }
    }

    /// The bud's TRIPOD, read off the three segments' stored spheres.
    ///
    /// Three claims, none re-derived from the construction code: each
    /// segment's axis makes the authored `tilt` with the bud's own;
    /// the three are spaced a third of a turn apart around it; and all
    /// three pass through the one attachment point, which is what
    /// makes the tips splay rather than the bellies. The attachment is
    /// derived from stored data alone — the sphere centre sits `top`
    /// along the segment's own axis from it.
    #[test]
    fn the_buds_three_axes_form_the_authored_tripod() {
        let ps = pieces();
        let bud_axis = Vec3::new(T3.0, 0.0, T3.1);
        let segs: Vec<(Point3<f64>, Vec3<f64>)> = ["lily_bud_a", "lily_bud_b", "lily_bud_c"]
            .into_iter()
            .map(|n| {
                let (center, _, axis) = sphere_of(body(&ps, n));
                // Stored axes may point either way along the line;
                // orient them all into the bud.
                let a = if axis.dot(bud_axis) < 0.0 {
                    -axis
                } else {
                    axis
                };
                (center, a)
            })
            .collect();
        for (i, (_, a)) in segs.iter().enumerate() {
            let c = a.dot(bud_axis);
            assert!(
                (c - deg(5.0).cos()).abs() < 1e-12,
                "bud segment {i}: axis leans {} rad, wanted {}",
                c.acos(),
                deg(5.0)
            );
        }
        // A third of a turn apart, measured across the bud axis.
        let across = |a: Vec3<f64>| a.reject_from(bud_axis).normalize();
        for (i, j) in [(0, 1), (1, 2), (2, 0)] {
            let c = across(segs[i].1).dot(across(segs[j].1));
            assert!(
                (c - deg(120.0).cos()).abs() < 1e-12,
                "bud segments {i},{j}: {} rad apart round the axis",
                c.acos()
            );
        }
        // And all three axes pass through ONE attachment point.
        let neck = |(c, a): (Point3<f64>, Vec3<f64>)| c - a * BUD_TOP;
        let n0 = neck(segs[0]);
        for (i, seg) in segs.iter().enumerate().skip(1) {
            let d = (neck(*seg) - n0).norm();
            assert!(d < 1e-12, "bud segment {i}'s neck is {d} from segment 0's");
        }
        // CHIRALITY, which none of the above pins. Tilt, spacing and
        // the shared neck are all LEAN-INVARIANT: the achiral star
        // (`lean` 0, every segment leaning outward along its own
        // radius) satisfies all three exactly as the pinwheel does,
        // because three axes 120 degrees apart look the same however
        // far round they have been carried from the places they
        // belong to. The lean is the angle BETWEEN those two, so it
        // needs the segment's own place — which is stored, as the
        // centre of its wedge: a partial revolve leaves two cap
        // half-planes, and the span is centred on the place.
        //
        // The caps are the planar faces CONTAINING the segment's axis
        // (the mouth's annulus is planar too, but its normal lies
        // ALONG the axis). Each cap's half-plane direction is
        // `n x axis` up to sign, and the sign is settled by asking
        // which way the cap's own vertices lie.
        for (i, name) in ["lily_bud_a", "lily_bud_b", "lily_bud_c"]
            .into_iter()
            .enumerate()
        {
            let b = body(&ps, name);
            let a = segs[i].1;
            let mut halves: Vec<Vec3<f64>> = Vec::new();
            for (_, f) in b.faces() {
                let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                    b.get_surface(f.surface)
                else {
                    continue;
                };
                if normal.dot(a).abs() > 0.5 {
                    continue;
                }
                let h = normal.cross(a).normalize();
                let s: f64 = b
                    .vertex_points()
                    .map(|(_, p)| p)
                    .filter(|p| (*p - origin).dot(normal).abs() < 1e-9)
                    .map(|p| (p - n0).dot(h))
                    .sum();
                halves.push(if s < 0.0 { -h } else { h });
            }
            assert_eq!(halves.len(), 2, "bud segment {i}: two wedge caps");
            // The two half-planes ARE the authored span apart, which
            // is what makes their bisector the segment's own place.
            let sp = halves[0].dot(halves[1]).acos();
            assert!(
                (sp - deg(156.0)).abs() < 1e-9,
                "bud segment {i}: wedge spans {sp} rad"
            );
            let place = (halves[0] + halves[1]).normalize();
            // Read both across the BUD's axis and take the signed
            // angle from the place to the lean. Sign and magnitude
            // are both the claim: 0 would be the star, and the
            // opposite sign the mirror-image pinwheel.
            let across = |v: Vec3<f64>| v.reject_from(bud_axis).normalize();
            let lean = signed_angle(across(place), across(a), bud_axis);
            // NEGATIVE ninety, for the authored quarter turn: `bud`
            // measures its lean in the sketch frame (e1, e2), whose
            // e1 x e2 is MINUS the bud axis, so a positive quarter
            // turn there reads as a negative one about the axis
            // itself. The handedness is the point; the frame it is
            // spelled in is not.
            // What that lean HAS to be, from the authored numbers and
            // nothing else. Two sign reversals sit between the
            // authored quarter turn and this reading, and both are
            // real properties of the stored bud rather than
            // bookkeeping:
            //
            //   * the wedge sweeps AWAY from the place it starts half
            //     a span before (see `bud`), landing centred a full
            //     `span` short of it, so the lean read off the
            //     realized centre is `lean + span` = 246 degrees;
            //   * that angle is measured here about the BUD's axis,
            //     and the frame `bud` spells its angles in is
            //     left-handed about it, so 246 reads as -246, i.e.
            //     +114.
            //
            // The window is 0.05 degrees around it because the wedge
            // lives in the segment's OWN tilted plane and is being
            // read across the bud's: at 5 degrees of tilt that
            // distortion is 0.037 degrees (measured), and it is the
            // only slack here. The three rival arrangements are
            // nowhere near it — the achiral star reads -156, the
            // mirror-image pinwheel -66 — so this is the handedness,
            // pinned.
            let want = deg(360.0) - deg(90.0) - deg(156.0);
            assert!(
                (lean - want).abs() < deg(0.05),
                "bud segment {i}: leans {} deg off its own place, wanted {} \
                 (-156 would be the achiral star, -66 the mirror)",
                lean.to_degrees(),
                want.to_degrees()
            );
        }
    }

    /// The lantern's PUCKER cone faces — every conical face whose
    /// carrier is not the NECK's.
    ///
    /// The lantern has two conical walls since the flower/arch weld
    /// was authored circle-coincident, and a full revolve halves each
    /// wall at its seam, so "the cone" is now four faces on two
    /// carriers. Wall 7's measurement is about the pucker alone (it
    /// is the wall the carving ball is near); the neck sits at the
    /// other end of the flower and folding it into the same box would
    /// measure a different thing. The neck is named by the weld pin,
    /// so the two walls are told apart by carrier and not by a
    /// threshold.
    pub(super) fn pucker_cone_faces(ps: &[Piece<f64>]) -> Vec<pncad::topo::FaceKey> {
        let lant = body(ps, "lily_lantern");
        let (neck, _, _, _) = weld_circle(lant, body(ps, "lily_arch"), caps(ps, "lily_arch"));
        let neck_apex = lant
            .faces()
            .find(|(k, _)| *k == neck)
            .and_then(|(_, f)| match lant.get_surface(f.surface) {
                Some(&Surface::Cone { apex, .. }) => Some(apex),
                _ => None,
            })
            .expect("the neck the weld pin names is a cone");
        lant.faces()
            .filter(|(_, f)| {
                matches!(
                    lant.get_surface(f.surface),
                    Some(&Surface::Cone { apex, .. }) if (apex - neck_apex).norm() > 1e-9
                )
            })
            .map(|(k, _)| k)
            .collect()
    }

    /// Everything a stored sphere holds: (centre, radius, axis,
    /// `u_ref`). Named so the agreement check below can compare the
    /// whole of one against the whole of another.
    type SphereCarrier = (Point3<f64>, f64, Vec3<f64>, Vec3<f64>);

    /// The body's single stored sphere: (centre, radius, axis).
    ///
    /// Single is CHECKED, not assumed. Every caller reads one sphere
    /// face and treats what it stores as the body's, so a second face
    /// carrying a different sphere would make that reading arbitrary —
    /// the same licence [`torus`] asserts for its own two half-bands,
    /// and asserted the same way: over the WHOLE carrier, `u_ref`
    /// included, because a partial comparison is one two different
    /// spheres can pass.
    fn sphere_of(b: &Body<f64>) -> (Point3<f64>, f64, Vec3<f64>) {
        let mut found: Option<SphereCarrier> = None;
        for (_, f) in b.faces() {
            if let Some(pncad::geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }) = b.get_surface(f.surface)
            {
                let s = (*center, *radius, *axis, *u_ref);
                match found {
                    Some(p) => assert!(
                        (p.0 - s.0).norm() < 1e-15
                            && (p.1 - s.1).abs() < 1e-15
                            && (p.2 - s.2).norm() < 1e-15
                            && (p.3 - s.3).norm() < 1e-15,
                        "two sphere faces, two carriers: {p:?} vs {s:?}"
                    ),
                    None => found = Some(s),
                }
            }
        }
        let (center, radius, axis, _) = found.expect("body stores no sphere");
        (center, radius, axis)
    }

    /// One planar cap of a lofted blade, reduced to the numbers the
    /// taper/roll test reads. See that test for why each is taken.
    struct Cap {
        /// The cap plane's stored unit normal.
        n: Vec3<f64>,
        /// Distance between its two margins.
        width: f64,
        /// The in-plane axis perpendicular to the width, signed so it
        /// points from the chord toward the RIDGE (the deeper of the
        /// section's two rises).
        v: Vec3<f64>,
        /// The two margins.
        margins: [Point3<f64>; 2],
    }

    /// Both caps of a lofted blade, each reduced to a [`Cap`].
    ///
    /// **Two, and the arity is checked here rather than at the
    /// callers.** Every caller reads the result positionally — the
    /// wide cap against the narrow one — so a body that handed back
    /// one plane, or three, would be indexed as if it were a blade
    /// and answer with a frame belonging to some other face. The
    /// per-face `on.len() == 8` below cannot catch that: it fires on
    /// the vertex set of whatever plane it is looking at, one plane
    /// too late to say the SET of planes is wrong.
    /// The apices of a lofted blade's section rims on the cap plane
    /// `(origin, normal)` whose chord midpoint is `mid`: each rim is a
    /// stored circle arc through both margins, under a half turn, so
    /// its apex lies on the chord's bisector one radius from its centre,
    /// on the side away from it.
    fn rim_apices(
        b: &Body<f64>,
        origin: Point3<f64>,
        normal: Vec3<f64>,
        mid: Point3<f64>,
    ) -> Vec<Point3<f64>> {
        b.edges()
            .filter_map(|(_, e)| {
                let geom = b.get_curve_geom(e.curve)?.certified()?;
                let pncad::geom::Curve3::Circle { center, radius, .. } = *geom.carrier() else {
                    return None;
                };
                ((center - origin).dot(normal).abs() < 1e-9).then(|| {
                    let out = mid - center;
                    center + out * (radius / out.norm())
                })
            })
            .collect()
    }

    fn cap_frames(b: &Body<f64>) -> Vec<Cap> {
        let mut out = Vec::new();
        for (_, f) in b.faces() {
            let Some(&pncad::geom::Surface::Plane { origin, normal, .. }) =
                b.get_surface(f.surface)
            else {
                continue;
            };
            let on_plane = |p: Point3<f64>| (p - origin).dot(normal).abs() < 1e-9;
            let on: Vec<Point3<f64>> = b
                .vertex_points()
                .map(|(_, p)| p)
                .filter(|p| on_plane(*p))
                .collect();
            assert_eq!(on.len(), 2, "a lens section's vertices are its two margins");
            let width = (on[1] - on[0]).norm();
            let axis = (on[1] - on[0]) / width;
            let mid = on[0] + (on[1] - on[0]) * 0.5;
            // v completes the frame; its SIGN is fixed by asking which
            // side of the chord the deeper rim (the ridge) rises on.
            let mut v = normal.cross(axis);
            let rises: Vec<f64> = rim_apices(b, origin, normal, mid)
                .into_iter()
                .map(|p| (p - mid).dot(v))
                .collect();
            assert_eq!(rises.len(), 2, "a lens section has two rims: {rises:?}");
            let hi = rises.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let lo = rises.iter().copied().fold(f64::INFINITY, f64::min);
            if hi < -lo {
                v = -v;
            }
            out.push(Cap {
                n: normal,
                width,
                v,
                margins: [on[0], on[1]],
            });
        }
        assert_eq!(
            out.len(),
            2,
            "a lofted blade has exactly two planar cap faces"
        );
        out
    }

    /// The signed angle from `a` to `b` about `axis` (all unit, `a`
    /// and `b` perpendicular to `axis` up to float noise).
    fn signed_angle(a: Vec3<f64>, b: Vec3<f64>, axis: Vec3<f64>) -> f64 {
        let s = a.cross(b).dot(axis);
        let c = a.dot(b);
        s.atan2(c)
    }

    /// **The spine curl wall, re-measured against the per-slab
    /// stacking fold.** The wall is no longer at spine turn π and is
    /// no longer about how far the spine goes. The loft's stacking
    /// statement is a fold over adjacent section pairs, each decided
    /// against both its sections' normals, so a blade whose spine
    /// turns a full circle and more builds as long as each of its
    /// slabs advances — and at [`LOFT_STATIONS`] stations each slab
    /// carries 1/16 of the turn.
    ///
    /// What remains is a SAMPLING wall, at per-slab turn π, i.e. total
    /// curl `(stations − 1)·π`. This probe pins it from both sides in
    /// the scene's own vocabulary by lofting the same blade at four
    /// stations, where the wall is at `3π ≈ 9.42` rad, and the
    /// refusal NAMES the slab that walls. At [`LOFT_STATIONS`] the
    /// blade coils past two full turns and still builds, so the
    /// refusal side is pinned at four stations instead.
    ///
    /// The wall is [`lofted_blade`]'s, the door the sepals are built
    /// through, so it is measured on the long leaf's blade alone: its
    /// base, direction, length and plan, without the sheath the scene's
    /// [`leaf_a`] lofts in front of it.
    #[test]
    fn the_spine_curl_wall_re_measured() {
        let (_, _, base, _) = leaf_a_spine(Tol::witness());
        let blade = |curl: f64, stations: usize| {
            try_lofted_blade::<f64>(
                base,
                LEAF_A_DIR,
                LEAF_A_UP,
                LEAF_A_LEN,
                -curl,
                leaf_a_plan(),
                stations,
                Tol::witness(),
            )
        };
        // Through π and far past it: every one of these refused
        // `ReversedStacking` above 3.0 rad under the end-to-end
        // statement, on a summary none of their slabs disagreed with.
        for curl in [0.45, 1.0, 2.0, 2.5, 2.8, 3.0, 3.5, 4.7, 6.0, 9.0, 13.0] {
            let out = blade(curl, LOFT_STATIONS);
            assert!(
                out.is_ok(),
                "curl {curl} rad over {LOFT_STATIONS} stations refused ({:?}) — \
                 each slab turns {:.4} rad, nowhere near the per-slab wall at π; \
                 re-derive this probe and the lofted_blade prose",
                out.err(),
                curl / ((LOFT_STATIONS - 1) as f64),
            );
        }
        // The wall that remains is per-slab, and at THIS scene's
        // sampling it is out of reach: 13.0 rad over 17 stations is
        // 0.81 rad per slab and still builds. Coarsen the sampling and
        // it comes into range — 10.0 rad over FOUR stations is 3.33
        // rad per slab, past π, and the refusal names the slab.
        //
        // Measured while re-deriving this probe, and worth knowing
        // before anyone reads the four-station row as the whole story:
        // at four, five, six and eight stations the blade meets a
        // DIFFERENT wall first — `nurbs_span_meter` escalating inside
        // the Euler assembly (#222's meter) — from 3.0 rad upward, so
        // the build side of the per-slab wall is not reachable at the
        // same station count as its refusal side in this scene. Both
        // sides ARE executed together, on a bare arc spine, in
        // `crates/sweep/tests/bool6_per_slab_stacking.rs`.
        let out = blade(10.0, 4);
        assert!(
            matches!(
                out,
                Err(pncad::sweep::LoftError::ReversedStacking { slab: 0 })
            ),
            "10.0 rad over 4 stations is 3.33 rad per slab: expected the per-slab \
             wall, naming slab 0, got {out:?} — the stacking wall moved; re-derive \
             this probe, the lofted_blade prose, and KERNEL-VERBS together"
        );
    }
}

mod verbs_gate_r1_probes {
    //! Reviewer probes (PR #1001 r1): the wall-7 steering correction,
    //! measured with the reviewer's own arithmetic rather than the
    //! kernel's boxes.

    use core::f64::consts::PI;
    use demo_tour::booleans::finished;
    use demo_tour::lily::*;
    use demo_tour::scalar::Scalar;
    use pncad::geom_core::Point3;
    use pncad::geom_core::Tol;
    use pncad::topo::Surface;
    use pncad::topo::{Body, BooleanError, Operand};

    /// The wall-7 finding, re-derived with the reviewer's own
    /// arithmetic. A slab box of the pucker (max-generator radius along
    /// its whole axial range) overlaps the carving ball's box, but the
    /// pucker's EXACT frustum never comes within the ball's radius of
    /// it, while the sphere ZONE's carrier does meet the ball. So a
    /// (Cone, Sphere) pair named on boxes is pure box looseness; the
    /// geometry the model cares about is sphere-on-sphere.
    #[test]
    fn wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let lant = &pieces
            .iter()
            .find(|p| p.name == "lily_lantern")
            .expect("lantern piece")
            .body;
        // The carving ball of wall 7.
        let (bc, br) = (
            demo_tour::lily::WALL7_BALL_CENTER,
            demo_tour::lily::WALL7_BALL_RADIUS,
        );
        let pucker = super::review_probes::pucker_cone_faces(&pieces);
        let mut min_frustum_gap = f64::INFINITY;
        let mut zone_hit = false;
        let mut zone = None;
        for (k, f) in lant.faces() {
            match lant.get_surface(f.surface) {
                Some(&Surface::Cone {
                    apex,
                    axis,
                    half_angle,
                    ..
                }) if pucker.contains(&k) => {
                    let mut v_lo = f64::INFINITY;
                    let mut v_hi = f64::NEG_INFINITY;
                    for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                        let Some(l) = lant.get_loop(lk) else { continue };
                        let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else {
                            continue;
                        };
                        for he in lant.loop_cycle(first).expect("walkable lantern loop") {
                            let v = lant.get_half_edge(he).expect("half-edge").start;
                            let p = *lant
                                .get_vertex(v)
                                .and_then(|vd| lant.get_point(vd.point))
                                .expect("vertex point");
                            let h = (p - apex).dot(axis);
                            v_lo = v_lo.min(h);
                            v_hi = v_hi.max(h);
                        }
                    }
                    let w = bc - apex;
                    let h = w.dot(axis);
                    let rad = w.reject_from(axis).norm();
                    let seg = |h0: f64, r0: f64, h1: f64, r1: f64| -> f64 {
                        let (dx, dy) = (h1 - h0, r1 - r0);
                        let t = (((h - h0) * dx + (rad - r0) * dy) / (dx * dx + dy * dy))
                            .clamp(0.0, 1.0);
                        let (px, py) = (h0 + t * dx, r0 + t * dy);
                        ((h - px).powi(2) + (rad - py).powi(2)).sqrt()
                    };
                    let ta = half_angle.tan();
                    let d = seg(v_lo, v_lo.abs() * ta, v_hi, v_hi.abs() * ta) - br;
                    min_frustum_gap = min_frustum_gap.min(d);
                }
                Some(&Surface::Sphere { center, radius, .. }) => {
                    let d = (center - bc).norm();
                    if d <= radius + br && d + br >= radius {
                        zone_hit = true;
                        zone = Some((center, radius));
                    }
                }
                _ => {}
            }
        }
        println!(
            "wall-7 probe: min frustum-to-ball surface gap = {min_frustum_gap:.4} \
             (positive = clear); sphere zone carrier meets ball = {zone_hit}"
        );
        assert!(
            min_frustum_gap > 0.0,
            "the pucker's exact frustum reaches the ball after all — the box-artifact \
             reading is wrong, re-derive"
        );
        assert!(
            zone_hit,
            "the carving ball no longer meets the sphere zone — the wall is not even \
             asking a sphere-on-sphere question any more"
        );

        // The cone arm boxes the FRUSTUM its axial window cuts rather
        // than a slab pinned at the window's widest radius. This row
        // prints that AABB against the ball's and the per-axis overlap,
        // and holds the box clear of the ball below.
        let (fa, fb) = frustum_aabb(lant, &pucker, (bc, br));
        let overlap = |lo_a: f64, hi_a: f64, lo_b: f64, hi_b: f64| hi_a.min(hi_b) - lo_a.max(lo_b);
        let per_axis = [
            overlap(fa.0.x, fa.1.x, fb.0.x, fb.1.x),
            overlap(fa.0.y, fa.1.y, fb.0.y, fb.1.y),
            overlap(fa.0.z, fa.1.z, fb.0.z, fb.1.z),
        ];
        let tightest = per_axis.iter().fold(f64::INFINITY, |m, v| m.min(*v));
        println!(
            "wall-7 probe: frustum AABB {fa:?} vs ball AABB {fb:?}; per-axis overlap \
             {per_axis:?} (tightest {tightest:.4}) — the exact loci clear by \
             {min_frustum_gap:.4}, so this overlap is AABB looseness on a tilted \
             frustum, not contact"
        );
        let ball_body = finished("the wall-7 ball", ball::<f64>(bc, br, tol), tol);
        let lant = &finished("lily_lantern", lant.clone(), tol);
        // **The measured outcome, and it is neither branch the review
        // anticipated.** With the axial window taken from the
        // boundary's own locus, the pucker's box clears the ball by
        // {tightest} on its tightest axis, so the pair-scoped gate
        // ADMITS — and what answers is not a germ class at all but
        // the operand-shape precondition. WHICH faces, corrected at
        // M9-5 and re-measured here: NOT the zone's two half-bands —
        // same-key CURVED adjacency is the canonical maximal form —
        // but the lantern's two AXIS-TOUCHING PLANAR CAPS.
        //
        // With the caps swept whole, the crossing layer's circle × sphere
        // roots, the radical-plane join and its chords' arcs, taken from
        // the germs, for a section tilted against the zone's polar axis, the carve
        // builds under every op, and each answer meets the lens the
        // ball and the zone's sphere share, from the radii and the
        // centre distance alone. What stops the scene is drawing it.
        assert!(
            tightest < 0.0,
            "the pucker's box must clear the ball's for the gate to admit; it does \
             not, so this row's reading of the carve below is wrong"
        );
        let (zc, zr) = zone.expect("the zone's sphere");
        let d = (zc - bc).norm();
        let x = (d * d + zr * zr - br * br) / (2.0 * d);
        let cap = |r: f64, h: f64| PI * h * h * (3.0 * r - h) / 3.0;
        let lens = cap(zr, zr - x) + cap(br, br - (d - x));
        let (va, vb) = (volume(lant, tol), 4.0 / 3.0 * PI * br.powi(3));
        let built = |label: &str, r: Result<pncad::topo::BooleanResult<f64>, BooleanError>| {
            let r = r.unwrap_or_else(|e| panic!("{label}: the carve builds, got {e:?}"));
            let body = r.body().expect("a body").body.clone();
            assert_eq!(
                pncad::topo::validate_geometric(&body, tol),
                Ok(()),
                "{label}"
            );
            body
        };
        let carved = built("A ∖ B", pncad::topo::subtract(lant, &ball_body, tol));
        for (label, body, want) in [
            ("A ∖ B", carved.clone(), va - lens),
            (
                "A ∪ B",
                built("A ∪ B", pncad::topo::union(lant, &ball_body, tol)),
                va + vb - lens,
            ),
            (
                "A ∩ B",
                built("A ∩ B", pncad::topo::intersect(lant, &ball_body, tol)),
                lens,
            ),
            (
                "B ∖ A",
                built("B ∖ A", pncad::topo::subtract(&ball_body, lant, tol)),
                vb - lens,
            ),
        ] {
            let got = volume(&body, tol);
            println!("wall-7 probe: {label} volume {got}, lens oracle {want}");
            assert!(
                (got - want).abs() <= 1e-9 * want.max(1.0),
                "{label}: volume {got} against the lens oracle {want}"
            );
        }
        // One seam draws: its section crosses the zone's seam meridians,
        // and the trimmed lane's sphere arm meshes the faces it bounds.
        let draw = pncad::mesh::tessellate(&carved, 2e-3, tol)
            .unwrap_or_else(|e| panic!("the carve draws, got {e:?}"));
        pncad::mesh::validate::check_mesh(&draw)
            .unwrap_or_else(|e| panic!("the carve's mesh checks, got {e:?}"));
        let carved_area = pncad::topo::mass_properties(
            &demo_tour::booleans::at_rest(&carved, tol),
            tol,
            demo_tour::printed(),
        )
        .expect("the carve's area integrates")
        .enclosure
        .surface_area
        .f();
        pncad::mesh::validate::check_volume(&draw, va - lens, carved_area, 2e-3)
            .unwrap_or_else(|e| panic!("the carve's mesh encloses the carve, got {e:?}"));
        // All three do not: the seam a third of a turn round lies inside
        // one half-band of the zone, a ring of that face.
        let seamed = demo_tour::lily::seamed_lantern(lant.clone().into_body(), tol)
            .unwrap_or_else(|e| panic!("the three seams carve, got {e:?}"));
        let drawn = pncad::mesh::tessellate(&seamed, 2e-3, tol);
        assert!(
            matches!(
                drawn,
                Err(pncad::mesh::TessellateError::RingOnCurvedFace { .. })
            ),
            "the seamed lantern's ringed zone face has no tessellation lane — got {:?}",
            drawn.map(|m| m.positions.len())
        );
    }

    /// The plant's other two boolean walls, as a TEST — same reason
    /// the bottle's are (`klein::verbs_gate_r1_probes`): the gate
    /// names a PAIR, so a change under the boxes moves them, and
    /// re-measuring must not cost a whole render pass.
    #[test]
    fn the_plants_other_boolean_walls_name_the_pairs_the_scene_claims() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let by = |name: &str| {
            &pieces
                .iter()
                .find(|p| p.name == name)
                .expect("named lily piece")
                .body
        };
        let operand = |name: &str| finished(name, by(name).clone(), tol);
        let (stem, arch, lant) = (
            &operand("lily_stem"),
            &operand("lily_arch"),
            &operand("lily_lantern"),
        );

        demo_tour::booleans::try_union(stem, arch, tol).expect("the stem's two arcs glue");

        // **The weld itself has no torus contact**, measured
        // off the two loci: the stem tube's end circle has radius
        // `STEM_R` = 0.060 and the arch's start disc radius `ARCH_R` =
        // 0.052, concentric and coplanar on the weld plane, so the two
        // stand `0.060 − 0.052 = 0.008 m` apart.
        //
        // The arch's cap planes, named by position: the weld disc is
        // the one within 2 m of the world origin (where the stem
        // starts).
        let weld = arch
            .faces()
            .filter_map(|(_, f)| match arch.get_surface(f.surface) {
                Some(&Surface::Plane { origin, .. }) => Some(origin),
                _ => None,
            })
            .find(|&o| (o - pncad::geom_core::Point3::origin()).norm() <= 2.0)
            .expect("the arch carries a weld cap");
        let ring_c = pncad::geom_core::Point3::new(-5.0, 0.0, 0.0);
        let to_stem_wall = |p: pncad::geom_core::Point3<f64>| {
            let rel = p - ring_c;
            let theta = rel.z.atan2(rel.x).clamp(0.0, deg(22.0));
            let on = pncad::geom_core::Point3::new(
                ring_c.x + 5.0 * theta.cos(),
                0.0,
                ring_c.z + 5.0 * theta.sin(),
            );
            // The UNSIGNED distance to the tube SURFACE: a point of
            // the weld disc sits inside the tube's radius, so the
            // signed form would report how far in, not how far off.
            ((p - on).norm() - STEM_R).abs()
        };
        // The weld disc, sampled: its own plane's two in-plane
        // directions are ±y and the perpendicular to the end tangent.
        let e2 = {
            let n = pncad::geom_core::Vec3::new(deg(22.0).sin(), 0.0, -deg(22.0).cos());
            n.normalize()
        };
        let e1 = pncad::geom_core::Vec3::new(0.0, 1.0, 0.0);
        let mut sep = f64::INFINITY;
        for ir in 0..=24 {
            let r = ARCH_R * f64::from(ir) / 24.0;
            for ip in 0..96 {
                let psi = 2.0 * PI * f64::from(ip) / 96.0;
                sep = sep.min(to_stem_wall(weld + (e1 * psi.cos() + e2 * psi.sin()) * r));
            }
        }
        assert!(
            (sep - (STEM_R - ARCH_R)).abs() < 1e-6,
            "the weld's true separation is its annular gap {}, measured {sep}",
            STEM_R - ARCH_R
        );

        let welded = pncad::topo::union(lant, arch, tol)
            .expect_err("the lantern still cannot be welded to the arch");
        println!("lily wall 2: {welded:?}");
        assert!(
            matches!(
                welded,
                BooleanError::CurvedPierceUnsupported {
                    operand: Operand::A,
                    ..
                }
            ),
            "wall 2 must stop at the crossing layer on the lantern's weld circle, \
             which lies on the arch's torus wall: {welded:?}"
        );
    }

    /// **The rootstock, measured**: the corm/foot socket is the
    /// corm's one bore wall against the foot's three arcs, all
    /// cylindrical, no planar pair, and the union glues it — one shell,
    /// valid at tier 3, its volume the two parts' sum (the interiors
    /// are disjoint) and the closed form of a sphere slab plus the
    /// foot standing out of it.
    #[test]
    fn the_rootstock_is_one_shell_at_the_parts_sum() {
        let tol = Tol::witness();
        let pieces = plant::<f64>(tol);
        let by = |name: &str| {
            &pieces
                .iter()
                .find(|p| p.name == name)
                .expect("named lily piece")
                .body
        };
        let (corm, foot) = rootstock_parts::<f64>(tol);
        let (corm, foot) = (&corm, &foot);
        let rootstock = by("lily_rootstock");
        assert_eq!(rootstock.shells().count(), 1, "one rootstock shell");
        assert_eq!(
            pncad::topo::validate_geometric(rootstock, tol),
            Ok(()),
            "the rootstock is valid at tier 3"
        );
        let (got, parts) = (
            volume(rootstock, tol),
            volume(corm, tol) + volume(foot, tol),
        );
        assert!(
            (got - parts).abs() <= 1e-12 * parts,
            "the rootstock's volume is the parts' sum: {got} vs {parts}"
        );
        // The slab of the corm's sphere between its two cut planes,
        // π[R²h − h³/3] from −base to +shoulder about the centre, plus
        // the foot's cylinder where it stands out of that slab.
        let (r, sh, ba) = (CORM_GLOBE, CORM_SHOULDER, CORM_BASE);
        let slab = PI * (r * r * (sh + ba) - (sh.powi(3) + ba.powi(3)) / 3.0);
        let stands_out = PI * STEM_R * STEM_R * (-FOOT_BOTTOM_Z - (sh + ba));
        let closed = slab + stands_out;
        assert!(
            (got - closed).abs() <= 1e-12 * closed,
            "the rootstock's volume is the slab-plus-foot closed form: {got} vs {closed}"
        );
    }

    /// The axis-aligned box of the named cone frusta of the lantern,
    /// and the carving ball's — the kernel's own two constructions,
    /// re-derived here so the residual looseness is measured by an
    /// outside consumer rather than read out of the module under
    /// test. `faces` is which cone the caller means; the lantern
    /// carries two walls' worth.
    /// One axis-aligned box as (lo, hi).
    type Aabb = (Point3<f64>, Point3<f64>);

    fn frustum_aabb(
        lant: &Body<f64>,
        faces: &[pncad::topo::FaceKey],
        ball: (Point3<f64>, f64),
    ) -> (Aabb, Aabb) {
        let (bc, br) = ball;
        let mut lo = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut hi = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
        for (k, f) in lant.faces() {
            if !faces.contains(&k) {
                continue;
            }
            let Some(&Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            }) = lant.get_surface(f.surface)
            else {
                continue;
            };
            let (mut v_lo, mut v_hi) = (f64::INFINITY, f64::NEG_INFINITY);
            for lk in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
                let Some(l) = lant.get_loop(lk) else { continue };
                let pncad::topo::LoopBoundary::Cycle { first } = l.boundary else {
                    continue;
                };
                for he in lant.loop_cycle(first).expect("walkable lantern loop") {
                    let v = lant.get_half_edge(he).expect("half-edge").start;
                    let p = *lant
                        .get_vertex(v)
                        .and_then(|vd| lant.get_point(vd.point))
                        .expect("vertex point");
                    let h = (p - apex).dot(axis);
                    v_lo = v_lo.min(h);
                    v_hi = v_hi.max(h);
                }
            }
            let ta = half_angle.tan();
            let h0 = v_lo.max(0.0).min(v_hi);
            let comp = |ai: f64, oi: f64| {
                let k = ta * (1.0 - ai * ai).max(0.0).sqrt();
                let g = |t: f64| (oi + t * ai - t.abs() * k, oi + t * ai + t.abs() * k);
                [g(v_lo), g(v_hi), g(h0)]
                    .into_iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |acc, (l, h)| {
                        (acc.0.min(l), acc.1.max(h))
                    })
            };
            let (x0, x1) = comp(axis.x, apex.x);
            let (y0, y1) = comp(axis.y, apex.y);
            let (z0, z1) = comp(axis.z, apex.z);
            lo = Point3::new(lo.x.min(x0), lo.y.min(y0), lo.z.min(z0));
            hi = Point3::new(hi.x.max(x1), hi.y.max(y1), hi.z.max(z1));
        }
        (
            (lo, hi),
            (
                Point3::new(bc.x - br, bc.y - br, bc.z - br),
                Point3::new(bc.x + br, bc.y + br, bc.z + br),
            ),
        )
    }
}
