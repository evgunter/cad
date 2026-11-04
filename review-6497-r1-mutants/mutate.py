import sys, pathlib
M = {
 "M1_horn_snap_removed": ("crates/sweep/src/revolve/axis.rs",
   "let major = if matches!(reach, Sign::Zero) {", "let major = if false {"),
 "M2_singular_at_no_sheet": ("crates/topo/src/boolean/edge_join.rs",
   "geom::Surface::Torus { center, axis, .. } | geom::Surface::Spindle { center, axis, .. } => {",
   "geom::Surface::Torus { center, axis, .. } => {"),
 "M7a_both_close_takes_left": ("crates/geom-brep/src/props/curved.rs",
   "(Ok(_), Ok(_)) => Ok(MaterialSign::Unencoded),", "(Ok(l), Ok(_)) => Ok(l),"),
 "M7b_apex_end_azimuth_not_middle": ("crates/geom-brep/src/props/curved.rs",
   "if pieces[(k + n - 1) % n].apex_after.is_some() {", "if false && pieces[(k + n - 1) % n].apex_after.is_some() {"),
 "M7c_fold_ignores_apex": ("crates/geom-brep/src/props/curved.rs",
   "let same = a.apex_after.is_none()\n            && ea.forward", "let same = ea.forward"),
 "M7d_apex_joint_flat_zero": ("crates/geom-brep/src/props/curved.rs",
   "flat: (pb - pa).norm(),", "flat: (pb - pa).norm() * T::zero(),"),
 "M7e_no_apex_edge_turns": ("crates/geom-brep/src/props/curved.rs",
   "edges_turns.extend(pieces.iter().filter_map(|p| p.apex_after)", "edges_turns.extend(pieces.iter().filter_map(|p| p.apex_after).filter(|_| false)"),
 "M7f_jump_lever_minor": ("crates/geom-brep/src/props/curved.rs",
   "EdgeTurn::exact(j.du, j.lever),", "EdgeTurn::exact(j.du, minor),"),
 "M7g_one_sided_err_swallow_to_err": ("crates/geom-brep/src/props/curved.rs",
   "(Ok(side), Err(_)) | (Err(_), Ok(side)) => Ok(side),", "(Ok(_), Err(e)) | (Err(e), Ok(_)) => Err(e),"),
}
name, mode = sys.argv[1], sys.argv[2]
f, a, b = M[name]
p = pathlib.Path("/home/user/cad")/f
s = p.read_text()
if mode == "apply":
    assert s.count(a) == 1, (name, s.count(a)); p.write_text(s.replace(a, b))
else:
    assert s.count(b) == 1; p.write_text(s.replace(b, a))
