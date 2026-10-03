import sys, pathlib
R = pathlib.Path('/home/user/ver3999/crates/geom-brep/src/ssi')
OA, ENDS, SSI = R/'one_arc.rs', R/'ends.rs', R.parent/'ssi.rs'
M = {
 'm1_chart_link_true': [(OA, "let linked = |found: Option<bool>| match found {\n        Some(true) => Ok(()),",
                              "let linked = |found: Option<bool>| match found {\n        _ if true => Ok(()),\n        Some(true) => Ok(()),")],
 'm2_r3_link_true': [(OA, "        if shared {\n", "        if shared || true {\n")],
 'm3a_count_is_some': [(OA, "            Some(2) => {}\n            Some(n) => return Err(Shortfall::Count(n)),",
                             "            Some(_) => {}\n            #[allow(unreachable_patterns)]\n            Some(n) => return Err(Shortfall::Count(n)),")],
 'm3b_count_ge2': [(OA, "            Some(2) => {}\n", "            Some(n) if n >= 2 => {}\n")],
 'm4a_ambiguous_joint_no_zero': [(OA, "            (_, true) => return None,", "            (_, true) => {}")],
 'm4b_unknown_end_as_true': [(OA, "let sign_at = |s: f64| sign(phi_over(boxes, plane, piece(s, s)));",
                                   "let sign_at = |s: f64| sign(phi_over(boxes, plane, piece(s, s))).or(Some(true));")],
 'm4c_unknown_end_as_false': [(OA, "let sign_at = |s: f64| sign(phi_over(boxes, plane, piece(s, s)));",
                                    "let sign_at = |s: f64| sign(phi_over(boxes, plane, piece(s, s))).or(Some(false));")],
 'm5_monotone_no_slope': [(OA, "        if sign(slope).is_some() {", "        if sign(slope).is_some() || true {")],
 'm6_face_depth_drops': [(OA, "                if depth >= EXIT_DEPTH {\n                    return None;", "                if depth >= EXIT_DEPTH {\n                    continue;")],
 'm7_krawczyk_nonstrict': [(OA, "k.lo() > s.lo() && k.hi() < s.hi()", "k.lo() >= s.lo() && k.hi() <= s.hi()")],
 'm8_chart_lane_atrest': [(ENDS, "            certify::Lane::Chart {\n                plane: self.plane,\n                wall: *wall,\n                pcurve,\n            },",
                                "            certify::Lane::AtRest {\n                a: &SsiOperand::Analytic(self.plane),\n                b: self.wall,\n                pcurve_b: Some(pcurve),\n            },")],
 'm9_spatial_lane_atrest': [(SSI, "        certify::Lane::Spatial {\n            pair: (a, b),\n            slab: domain.slab(),\n        },",
                                  "        certify::Lane::AtRest {\n            a: &SsiOperand::Analytic(a),\n            b: &SsiOperand::Analytic(b),\n            pcurve_b: None,\n        },")],
 'm10_edge_depth_drops': [(OA, "        if depth >= EXIT_DEPTH {\n            return None;", "        if depth >= EXIT_DEPTH {\n            continue;")],
}
name = sys.argv[1]
for f, old, new in M[name]:
    s = f.read_text()
    assert s.count(old) == 1, (name, f, s.count(old))
    f.write_text(s.replace(old, new))
print('applied', name)
