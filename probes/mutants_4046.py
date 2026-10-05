"""Apply one claim-6 mutant to crates/topo/src/boolean/sphere_region.rs (frozen head b793623189)."""
import sys
p = 'crates/topo/src/boolean/sphere_region.rs'
s = open(p).read()
m = sys.argv[1]
if m == 'flip':
    a = "Sign::Negative => Ray::Inside(true),\n                Sign::Positive => Ray::Inside(false),"
    b = "Sign::Negative => Ray::Inside(false),\n                Sign::Positive => Ray::Inside(true),"
elif m == 'noseam':
    a = "    arcs.retain(|a| !seams.contains(&a.edge));\n"
    b = "    let _ = &seams;\n"
elif m == 'parity':
    # decide by the parity of non-vertex crossings instead of the closest one's heading
    a = "        let y = self.arcs[arc].at(theta) - self.center;\n        let y = y / y.norm();"
    b = ("        let _ = (arc, theta);\n"
         "        if hits.iter().any(|h| h.at_vertex) { return Ok(Ray::Abandoned); }\n"
         "        return Ok(Ray::Inside(hits.len() % 2 == 1));\n"
         "        #[allow(unreachable_code)]\n"
         "        let y = self.arcs[arc].at(theta) - self.center;\n        let y = y / y.norm();")
else:
    raise SystemExit(m)
assert s.count(a) == 1, m
open(p, 'w').write(s.replace(a, b))
