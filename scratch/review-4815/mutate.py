import sys
p='/home/user/review-alt/crates/topo/src/split.rs'
s=open(p).read()
m=sys.argv[1]
def rep(a,b):
    global s
    assert a in s, (m,a)
    s=s.replace(a,b,1)
if m=='M1':
    rep("        let geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness } =\n            spec.description\n        else {\n            return Ok(spec);\n        };",
        "        if true { return Ok(spec); }\n        let geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness } =\n            spec.description\n        else {\n            return Ok(spec);\n        };")
elif m=='M2':
    rep("                geom_brep::MustCarryDescription::Conventional => {\n                    geom_brep::EdgeDescriptionSpec::chart(chart)\n                }",
        "                geom_brep::MustCarryDescription::Conventional => {\n                    let _ = chart;\n                    geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness }\n                }")
elif m=='M3':
    rep("[p_u, p_new], band)?", "[p_u, p_v], band)?")
    rep("[p_new, p_v], band)?", "[p_u, p_v], band)?")
elif m=='M4':
    rep("let chart = crate::pcurves::half_edge_face(self, hp.key()).1.surface;",
        "let chart = crate::pcurves::half_edge_face(self, hm.key()).1.surface;")
elif m=='M5':
    rep("""                .map_err(|refusal| {
                    refusal.route(""","""                .or_else(|refusal| match refusal {
                    geom_brep::MustCarryRefusal::InBand(_) => Ok(geom_brep::MustCarryDescription::Intrinsic(
                        geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness })),
                    r => Err(r),
                })
                .map_err(|refusal| {
                    refusal.route(""")
elif m=='M6':
    # Route every description through the rule, over the parent's two faces.
    rep("""        let geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness } =
            spec.description
        else {
            return Ok(spec);
        };
        let [ka, kb] = pair.keys();""","""        let (pair, witness) = match spec.description {
            geom_brep::EdgeDescriptionSpec::TangentIntersection { pair, witness } => (pair, witness),
            _ => {
                let e = self.get_edge(edge).unwrap();
                let a = crate::pcurves::half_edge_face(self, e.he_plus).1.surface;
                let b = crate::pcurves::half_edge_face(self, e.he_minus).1.surface;
                let (s0, s1) = (spec.param_start, spec.param_end);
                let mid = spec.carrier.eval((s0 + s1) * T::from_f64(0.5));
                (geom_brep::SurfacePair::new(a, b), mid)
            }
        };
        let [ka, kb] = pair.keys();""")
else:
    raise SystemExit('unknown')
open(p,'w').write(s)
