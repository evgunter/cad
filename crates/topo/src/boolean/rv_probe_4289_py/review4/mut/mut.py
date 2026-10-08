# Review 4 mutants: apply one to sectors.rs (python3 mut.py NAME), revert with git checkout.
import sys
P = '/home/user/probe/crates/topo/src/boolean/sectors.rs'
M = {
 # implementer's: facing levered at the direction alone
 'FacLever': ("let facing = least_lever([(lever, one), (l_start, half), (l_end, half)]);",
              "let facing = least_lever([(lever, one)]);"),
 # facing reading dropped
 'NoFacing': ('        ("bool_cone_facing", Margin::levered(d.dot(middle), facing)),\n', ''),
 # crossing point levered at the direction's reach only (how well it is located dropped)
 'CrossLd': ("geom_core::is_finite_length(T::from_f64(1.0) / located).then(|| (x, l_d.min(located)))",
             "geom_core::is_finite_length(T::from_f64(1.0) / located).then(|| (x, l_d))"),
 # arc_side's span gate dropped
 'NoSpan': ('    match decide("bool_cone_arc_span", gate, band).map_err(escalate)? {\n        Sign::Positive => {}\n        _ => return Ok(None),\n    }',
            '    let _ = gate;'),
 # p decided on S's plane taken as on the direction's side (no crossing)
 'POnSame': ("                (SideCode::In | SideCode::Out, Some(code)) if code == side => continue,",
             "                (SideCode::In | SideCode::Out, Some(code)) if code == side => continue,\n                (SideCode::On, Some(_)) => continue,"),
 # an in-band direction on a face, within its sector, read as off the face
 'ErrOffFace': ("            Err(e) if on_face()? => return Err(e),\n", ""),
 # a crossing in_sector can't decide: taken as outside
 'CrossErrOut': ("                        Err(e) => {\n                            escalation.get_or_insert(e);\n                            continue 'reference;\n                        }\n                    }\n                }\n            };",
                 "                        Err(_) => continue,\n                    }\n                }\n            };"),
 # pointed gate dropped
 'NoPointedGate': ('let away = match decide("bool_cone_pointed", Margin::levered(mean.norm(), arm), band) {',
                   'let away = match decide("bool_cone_pointed", Margin::levered(T::from_f64(1.0), arm), band) {'),
 # in_sector bound term: levered at the bound's reach alone (direction dropped)
 'BndOnly': ("let past = |x: T, l_bound: T| Margin::levered(x, least_lever([(lever, one), (l_bound, one)]));",
             "let past = |x: T, l_bound: T| Margin::levered(x, least_lever([(l_bound, one)]));"),
 # 398d9382's lever: in_sector's bound term dropped
 'BndLever': ("let past = |x: T, l_bound: T| Margin::levered(x, least_lever([(lever, one), (l_bound, one)]));",
              "let past = |x: T, l_bound: T| Margin::levered(x, least_lever([(lever, one)]));"),
 # arc_side: p's term dropped
 'ArcNoP': ("        (arc.p_arm, d.cross(b).norm()),\n", ""),
}
name = sys.argv[1]; a, b = M[name]
s = open(P).read(); assert s.count(a) == 1, (name, s.count(a)); open(P, 'w').write(s.replace(a, b))
print('applied', name)
