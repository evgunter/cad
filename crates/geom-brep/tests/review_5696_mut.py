import sys
M = {
 'A_w_once': ('crates/geom-brep/src/props/rational_rim.rs', 'let scale = w.mul(w).recip().scale(half_rate);', 'let scale = w.recip().scale(half_rate);'),
 'B_no_ref': ('crates/geom-brep/src/props/rational_rim.rs', '[(c.x - r.x) * w, (c.y - r.y) * w, (c.z - r.z) * w, w]', '{ let _ = r; [c.x * w, c.y * w, c.z * w, w] }'),
 'C_rem_order': ('crates/geom-brep/src/props/spiric.rs', 'let remainder = over[k].0[ORDER - 1] * weight[ORDER - 1];', 'let remainder = over[k].0[ORDER - 1] * weight[ORDER - 2];'),
 'D_no_invlen': ('crates/geom-brep/src/props/rational_rim.rs', 'let half_rate = span.inv_len * pt(0.5);', 'let half_rate = pt(0.5);'),
 'G_point_rem': ('crates/geom-brep/src/props/spiric.rs', 'let remainder = over[k].0[ORDER - 1] * weight[ORDER - 1];', 'let remainder = at[k].0[ORDER - 1] * weight[ORDER - 1];'),
 'H_no_rem': ('crates/geom-brep/src/props/spiric.rs', 'let remainder = over[k].0[ORDER - 1] * weight[ORDER - 1];', 'let remainder = Interval::zero() * weight[ORDER - 1];'),
 'I_recip_trunc': ('crates/geom-brep/src/props/rational_rim.rs', 'let scale = w.mul(w).recip().scale(half_rate);', 'let ww = w.mul(w).recip(); let scale = Jet::cst(pt(ww.value().lo()) + Interval::zero()).add(ww.sub(ww)).scale(half_rate);'),
 'J_alpha': ('crates/geom-brep/src/props/rational_rim.rs', '/ (pt(knots[i + p]) - pt(knots[i]));', '/ (pt(knots[i + p + 1]) - pt(knots[i]));'),
 'K_window': ('crates/geom-brep/src/props/rational_rim.rs', 'let (lo, hi) = (a.max(t0), b.min(t1));', 'let (lo, hi) = (a, b);'),
}
name, mode = sys.argv[1], sys.argv[2]
f, a, b = M[name]
p = '/home/user/rv/' + f
s = open(p).read()
if mode == 'apply':
    assert s.count(a) == 1, name; s = s.replace(a, b)
else:
    assert s.count(b) == 1, name; s = s.replace(b, a)
open(p, 'w').write(s)
