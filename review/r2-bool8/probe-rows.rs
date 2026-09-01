// ==================================================================
// R2 BOOL-8 probes (PR #1508, frozen head 6aa2684f2). APPENDED to
// crates/profile/tests/path_property.rs for the probe runs and
// REVERTED after; kept here as the record. Each probe attacks one
// PR-body claim; results in review/r2-bool8/NOTES.md.
// ==================================================================

/// PROBE 1 (claim 3): no authored spelling sneaks a tangency through
/// as a "continuation". Every director that could re-author the
/// incoming direction — `.toward` with the exact same displacement,
/// `.turn(0)`, `.angle(exact incoming angle)` — still refuses
/// `JunctionTangent`; the declared spelling refuses
/// `SameCarrierJunction`. The only accepting spelling is the one with
/// NO authored direction at all.
#[test]
fn r2_probe_authored_spellings_cannot_sneak_the_continuation() {
    let t = Tol::witness();
    let tip = || {
        Open.at(p2(0.0, 0.0))
            .toward(3.0, 7.0, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
    };
    // .toward with the exact incoming displacement: authored, refuses.
    assert!(matches!(
        tip().toward(3.0, 7.0, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // .turn(0) off a LINE end (the arc case has its own row): refuses.
    assert!(matches!(
        tip().turn(0.0, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // .angle at the exact incoming angle: authored, refuses.
    let theta = 7.0f64.atan2(3.0);
    assert!(matches!(
        tip().angle(theta, t),
        Err(PathError::JunctionTangent { .. })
    ));
    // declared identity: refuses (the #101 rule, untouched).
    assert!(matches!(
        tip().tangent().line(2.0, t),
        Err(PathError::SameCarrierJunction { .. })
    ));
}

/// PROBE 2 (claim 4): the carrier-blindness seam cannot be laundered
/// past `validate` by chaining — TWO continuations off the arc still
/// land at the data gate, and so does a continuation off a fillet's
/// ARC arrival end (a second arc-carrier directed point in the tree).
#[test]
fn r2_probe_arc_continuations_never_pass_validate() {
    let t = Tol::witness();
    let undeclared = Open
        .at(p2(-1.0, 0.0))
        .arc_to(
            Bulge {
                p: p2(1.0, 0.0),
                b: 1.0,
            },
            t,
        )
        .unwrap()
        .line(0.5, t)
        .unwrap()
        .line(0.5, t)
        .unwrap()
        .line_to(Start, t)
        .map(pinned)
        .unwrap();
    assert!(undeclared.tangent_joints().is_empty());
    let refused = Profile::new(SketchPlane::xy(), vec![undeclared])
        .validate(t)
        .unwrap_err();
    assert!(
        matches!(refused, profile::ProfileError::UndeclaredTangency { .. }),
        "chained continuations off an arc must still land at the data gate: {refused:?}"
    );
}

/// PROBE 3 (claim 5): third-spelling search for the lily seam wall,
/// rotation 1 fixture (seam at the corner `right`). Every candidate
/// closer the surface offers from the run's subdivision vertex
/// refuses, and the continuation dead-ends structurally:
///  (a) `.tangent()` + tangent arc to Start — degenerates onto the
///      carrier (TangentLineClose);
///  (b) the REVERSED traversal — same alternation, same wall;
///  (c) continuing `line(half)` to land exactly ON Start's
///      coordinates — a directed point, not a closure; the zero-length
///      `line_to(Start)` left over refuses.
#[test]
fn r2_probe_lily_seam_third_spellings_all_refuse() {
    let right = p2(1.0, 0.0);
    let ridge = p2(0.0, 1.5);
    let left = p2(-1.0, 0.0);
    let keel = p2(0.0, -1.0);
    let half = |a: Point2<f64>, b: Point2<f64>| 0.5 * (b - a).norm_squared().sqrt();
    let t = Tol::witness();
    let side = |chain: PartialPath<f64, HasPos<WithIncoming>, profile::path::NoAng>,
                from: Point2<f64>,
                to: Point2<f64>| {
        let d = to - from;
        chain
            .toward(d.x, d.y, t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
            .line(half(from, to), t)
            .unwrap()
    };
    let at_m3 = || {
        let d0 = ridge - right;
        let first = Open
            .at(right)
            .toward(d0.x, d0.y, t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap()
            .line(half(right, ridge), t)
            .unwrap();
        side(side(first, ridge, left), left, keel)
            .toward(right.x - keel.x, right.y - keel.y, t)
            .unwrap()
            .line(half(keel, right), t)
            .unwrap()
    };
    // (a) declared + tangent arc to Start: degenerate onto the carrier.
    assert!(matches!(
        at_m3().tangent().tangent_arc_to(Start, t),
        Err(PathError::TangentLineClose { .. })
    ));
    // (b) reversed traversal (right -> keel -> left -> ridge -> right):
    // the closer still departs a subdivision vertex.
    let db = keel - right;
    let rev_first = Open
        .at(right)
        .toward(db.x, db.y, t)
        .unwrap()
        .line(half(right, keel), t)
        .unwrap()
        .line(half(right, keel), t)
        .unwrap();
    let rev_at_last_mid = side(side(rev_first, keel, left), left, ridge)
        .toward(right.x - ridge.x, right.y - ridge.y, t)
        .unwrap()
        .line(half(ridge, right), t)
        .unwrap();
    assert!(matches!(
        rev_at_last_mid.line_to(Start, t),
        Err(PathError::TangentLineClose { .. })
    ));
    // (c) the continuation lands ON Start's coordinates but mints a
    // directed point, not a closure; the leftover closer is
    // zero-length and refuses. (NonpositiveLeg via line_to's sugar, or
    // whatever typed refusal the door gives — the point is Err.)
    let parked_on_start = at_m3().line(half(keel, right), t).unwrap();
    assert!(parked_on_start.line_to(Start, t).is_err());
}

/// PROBE 4 (claim 1), REVISED after a first run: the bit-identical
/// DISPLACEMENT property is a fixture artifact, not the inherited
/// thing. From the origin, `0 + d` and `d + d` are exact, so the first
/// two displacements match bitwise — but the THIRD leg's endpoint
/// rounds (`2d + d` is inexact) and its realized displacement differs
/// in the last bit. What is inherited bitwise is the `Dir`; the vertex
/// table only shows it exactly while the additions are exact. This
/// probe pins the boundary: d(0) == d(1), d(1) != d(2).
#[test]
fn r2_probe_bitwise_inheritance_is_transitive() {
    let t = Tol::witness();
    let lp = Open
        .at(p2(0.0, 0.0))
        .toward(0.1, 0.3, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line(0.7, t)
        .unwrap()
        .line_to(p2(-5.0, 1.0), t)
        .unwrap()
        .line_to(Start, t)
        .map(pinned)
        .unwrap();
    let v = lp.vertices();
    let d = |i: usize| {
        (
            (v[i + 1].pos().x - v[i].pos().x).to_bits(),
            (v[i + 1].pos().y - v[i].pos().y).to_bits(),
        )
    };
    assert_eq!(d(0), d(1), "doubling from the origin is exact");
    assert_ne!(
        d(1),
        d(2),
        "the third endpoint rounds: bit-identical displacements are the \
         fixture's property, not the inheritance's"
    );
    assert!(lp.tangent_joints().is_empty());
    validate_lp(&lp);
}
