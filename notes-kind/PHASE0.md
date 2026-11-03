# splice/kind-reads — Phase 0 (main at d1ac6881)

Branch-only notes; removed before the PR leaves draft.

1. slit::carrier_on_surface — HOLDS. `conic_or_line_on` answers `Ok(None)`
   for a Spiric/Nurbs carrier on every analytic surface; `holding`'s
   `Ok(None)` arm returns `Ok(None)` unless the FACE is a spline, so a
   spline/spiric edge on a plane face is "in no face" and skips the
   refusal. `sectors::closed_reach` reads the same `None` as unread.
2. splitting::join::certify_section_area — HOLDS. `curve.carrier().conic()`
   `None => continue` takes Line, Spiric and Nurbs alike (chord).
3. splitting::finish::describe_section_boundary — HOLDS (latent). Reads
   `geom_brep::classify_dihedral`; boolean reads `classify_seam` via
   `ops::seam_face`. The split gate (`classify::carrier_gate`) refuses a
   Sphere/Torus/Nurbs/Approx face, and a Spiric/Nurbs edge, the plane may
   meet, so no spline face or spline/spiric edge reaches the describer
   today: the row's test stays owed.
4. reduce::curved_face_arm + dihedral_side — PREMISE STALE. The arm no
   longer refuses `CurvedBooleanUnsupported` on a NURBS face before a side
   is read: it routes a spline face through
   `curve_face::route(..).door == SplineFace` to `reduce::spline_face`
   (boundary pass; `Along` records the ends and the join glues;
   `Clear` goes to `spline_interior`'s chart meetings). Nothing on that
   path asks a parent face's side, so there is no consumer to wire
   `dihedral_side` into. `dihedral_side` is still called nowhere in topo.
   The witness `a_sheath_grafted_on_its_blade` no longer reaches this
   door: it stops at the rim certificate,
   `EdgeUndescribed{Intersection, Certification{NotTransverse{NOT_A_SAMPLE,
   lever: None, Refused::Zero}}}`, in both orders.
5. certified_endpoint_census — HOLDS, and grew: 16 files moved
   (spline_face 23/23 vs 9/8; section_bounds 12/12 vs 13/13; krawczyk 4/3
   vs 5/4; enclose 21/21 vs 19/19; surfaces/nurbs 7/0 vs 8/2;
   clearance 30/7 vs 25/7; new: pair_section.rs, pair_section/{ruling,
   spline_implicit, spline_implicit/curved, spline_implicit/sweep/critical,
   winds}.rs, ssi/boundary.rs, ssi/residual.rs, geom/net.rs,
   topo/test_support_fixtures.rs).
