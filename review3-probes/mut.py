import sys
M = {
 'G1_hollow_ignored': ('sectors.rs', '                (false, true) => (false, true),\n', '                (false, true) => return Ok(None),\n'),
 'G2_hollow_nonconvex': ('sectors.rs', '                        if !convex && !reflex {\n                            return Ok(None);\n                        }\n', ''),
 'G3_nesting_reversed': ('vtxfac.rs', 'wedge_classes(&pairs[i].sectors, &pairs[j].sectors, band)', 'wedge_classes(&pairs[j].sectors, &pairs[i].sectors, band)'),
 'G4_fallback_skipped': ('vtxfac.rs', 'layered_alone(pairs, band).unwrap_or_else(alone)', 'layered_alone(pairs, band).unwrap_or_default()'),
 'G5_single_keeps_hollow': ('vtxfac.rs', '.filter_map(|p| p.read.as_ref().filter(|r| !r.hollow))', '.filter_map(|p| p.read.as_ref())'),
 'G6_either_side': ('vtxfac.rs', 'let side = || pairs.iter().filter(move |p| p.side == Some(own));', 'let side = || pairs.iter().filter(move |_p| true);'),
 'G7_outermost_any': ('vtxfac.rs', '            if base.is_some_and(|b| b != here) {\n                return None;\n            }\n', ''),
}
name = sys.argv[1]
f, a, b = M[name]
p = './crates/topo/src/boolean/' + f
s = open('review3-probes/orig/' + f).read()
assert s.count(a) == 1, (name, s.count(a))
if name == 'G2_hollow_nonconvex':
    s = s.replace(a, b)
    a2 = '                (false, true) => (false, true),\n'
    assert s.count(a2) == 1
    s = s.replace(a2, '                (false, _) => (false, true),\n')
else:
    s = s.replace(a, b)
open(p, 'w').write(s)
print('applied', name)
