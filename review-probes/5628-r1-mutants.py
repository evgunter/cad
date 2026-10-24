import sys
root="/home/user/cad/crates/topo/src/"
def sub(f, old, new):
    p=root+f; s=open(p).read()
    assert s.count(old)==1, (f, old[:60], s.count(old))
    open(p,'w').write(s.replace(old,new))
m=sys.argv[1]
if m=="M1":  # set walk advances only the first wanted member that moves
    sub("props.rs","""            if !wants[m] {
                continue;
            }
            // Every refinable face""","""            if !wants[m] || moved {
                continue;
            }
            // Every refinable face""")
elif m=="M2":  # Resolution through the funnel, recorded
    sub("props/quad_lane.rs","Some(len) => Ok(quad::mean_boundary_displacement(bounds.flux, bounds.area)? <= len),",
        "Some(len) => { let w = quad::mean_boundary_displacement(bounds.flux, bounds.area)?; Ok(geom_core::k_stats::decide_reported(\"props_quad_resolution_mut\", geom_core::Margin::of(len - w), band).is_ok_and(|d| d.sign != geom_core::Sign::Negative)) }")
elif m=="M3":  # a walk stops refining a face at the first round under 1024 eps
    sub("props.rs",".filter_map(|(slot, run)| run.next_round().map(|round| (slot, run.face, round)))",
        ".filter_map(|(slot, run)| run.next_round().filter(|_| !run.meets(interim_resolution(member.tol))).map(|round| (slot, run.face, round)))")
elif m=="M4":  # advance drops the narrower enclosure
    sub("props.rs","if !geom_brep::props::quad::narrower(was, now) {","if true {")
elif m=="M5":  # classify_shells reads round 0 only
    sub("props.rs","""        let role = shell_role_reading(body, shell_key, band, || {
            shell_volume_role_of(shell_key, &mut certificate, quad)
        })?;""","""        let role = shell_role_reading(body, shell_key, band, || {
            for run in &mut certificate.runs {
                if let FaceState::Quad { stop, .. } = &mut run.state {
                    *stop = LaneStop::Floor;
                }
            }
            shell_volume_role_of(shell_key, &mut certificate, quad)
        })?;""")
elif m=="M6":  # backstop starts from the reporting walk
    sub("boolean/ops.rs","""        crate::props::SignCertificate::at_round_zero(body, &faces, band, tol, Some(lane))
            .map_err(|source| props_refusal(operand, source))""","""        crate::props::measure_to(body, &faces, band, tol, (Some(lane), None), crate::props::Settle::Resolution(quantity::Length::from_meters(1024.0 * tol.eps())))
            .map(|(_, c)| c)
            .map_err(|e| match e { crate::props::MeasureRefusal::Props(source) => props_refusal(operand, source), crate::props::MeasureRefusal::Poisoned(_) => unreachable!() })""")
print("applied", m)
if m=="M3s":  # sign walks only: stop refining a face at the first round under 1024 eps
    sub("props.rs",".filter_map(|(slot, run)| run.next_round().map(|round| (slot, run.face, round)))",
        ".filter_map(|(slot, run)| run.next_round().filter(|_| !(SIGN_MUT.with(|f| f.get()) && run.meets(interim_resolution(member.tol)))).map(|round| (slot, run.face, round)))")
    sub("props.rs","pub(crate) fn walk_set<'b, T: Decide, V>(", "thread_local! { static SIGN_MUT: core::cell::Cell<bool> = const { core::cell::Cell::new(false) }; }\npub(crate) fn walk_set<'b, T: Decide, V>(")
    sub("props.rs","""        let walked = walk_set(&mut walking, certify, |rounds, wants| {""","""        SIGN_MUT.with(|f| f.set(true));
        let walked = walk_set(&mut walking, certify, |rounds, wants| {""")
    sub("props.rs","""        match walked {
            Ok(_) => break,""","""        SIGN_MUT.with(|f| f.set(false));
        match walked {
            Ok(_) => break,""")
    print("applied M3s")
