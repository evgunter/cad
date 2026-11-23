import sys, pathlib
R = pathlib.Path("/home/user/cad")
M = {
 "MA_tip_above": ("crates/topo/src/boolean/fate.rs", "r.reach.below()).reduce(Lever::min)", "r.reach.above()).reduce(Lever::min)"),
 "MD_travel_names_swapped": ("crates/topo/src/boolean/join.rs", '[name, "bool_join_arc_clear"],', '["bool_join_arc_clear", name],'),
 "ME_resolved_run_band": ("crates/topo/src/validate.rs", "decide_reported(name, Margin::of(rounding), share)", "decide_reported(name, Margin::of(rounding), { let _ = share; band })"),
 "MF_offer_k_wider": ("crates/geom-core/src/predicate.rs", "Band::from_thresholds(eps, k).unwrap_or_else", "Band::from_thresholds(eps * k, k).unwrap_or_else"),
 "MH_tip_no_halves_check": ("crates/topo/src/boolean/fate.rs", "    if halves == 0 {\n        return Err(unread());\n    }\n", ""),
 "MI_apex_no_gap_err": ("crates/geom-brep/src/intersect.rs", "(gap.abs() + gap_err) / sine", "{ let _ = gap_err; gap.abs() / sine }"),
 "MO_at_wedge_no_filter": ("crates/geom-brep/src/dihedral.rs", "        .offered_band()\n        .filter(|_| escalation.re_quotes())\n", "        .offered_band()\n"),
 "MQ_tip_least_halfturn_only_proxy_max": ("crates/topo/src/boolean/fate.rs", "r.reach.below()).reduce(Lever::min)", "r.reach.below()).reduce(Lever::max)"),
 "MR_pairing_offer_from_run": ("crates/geom-brep/src/dihedral.rs", "wedge_reads_zero_at_the_offer(\"material_pairing_offer_wedge\", wedge, offered)", "wedge_reads_zero_at_the_offer(\"material_pairing_offer_wedge\", wedge, { let _ = offered; band })"),
 "MS_step_finer_wider": ("crates/geom-core/src/predicate.rs", "Self::new(self.zero / (self.escalate / self.zero), self.zero)", "Self::new(self.zero / (self.escalate / self.zero).sqrt(), self.zero)"),
}
name = sys.argv[1]
f, a, b = M[name]
p = R / f
s = p.read_text()
assert s.count(a) == 1, (name, s.count(a))
p.write_text(s.replace(a, b))
print("planted", name, "in", f)
