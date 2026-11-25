import sys, re
ER='crates/geom-brep/src/curve_face/end_root.rs'
RD='crates/topo/src/boolean/reduce.rs'
def rep(f, a, b, count=1):
    s=open(f).read()
    assert s.count(a)>=1, (f,a)
    s=s.replace(a,b,count); open(f,'w').write(s)
m=sys.argv[1]
if m=='1': rep(ER,'for p in m..n.len() {','for p in m..=m {')
elif m=='2': rep(ER,'charge.push(n[p - 1].abs() + zero * g / pow(run, p - 1));','charge.push(T::zero());')
elif m=='3': rep(ER,'charge.push(n[p - 1].abs() + zero * g / pow(run, p - 1));','charge.push(n[p - 1].abs());')
elif m=='4':
    rep(ER,'    charge: &\'a [T],\n    p: usize,\n}','    charge: &\'a [T],\n    p: usize,\n    m: usize,\n}')
    rep(ER,'            charge: &charge,\n            p,\n','            charge: &charge,\n            p,\n            m,\n')
    rep(ER,'let lead = n[p].abs() * T::from_f64(0.5);','let lead = n[self.m].abs() * T::from_f64(0.5);')
elif m=='5':
    rep(ER,'    let mut first = None;\n    for p in m..n.len() {\n        if p > m {\n            charge.push(n[p - 1].abs() + zero * g / pow(run, p - 1));\n        }',
           '    let mut first = None;\n    for q in m..n.len() { charge.push(n[q].abs() + zero * g / pow(run, q)); }\n    for p in (m..n.len()).rev() {')
elif m=='6': rep(RD,'proven("junction_end_reach", reach * half - spread)','true')
elif m=='7': rep(RD,'let chord = (T::one() + (bend_rim * spread).powi(2) * third).powi(2);','let chord = T::one() + T::zero() * third;')
elif m=='8': rep(RD,'&& proven("junction_end_chord", half - bend_rim * spread)','')
elif m=='9': rep(RD,'    if !junction_reach_held(window.spread, bend_rim, reach) {\n        return Err(no_window);\n    }\n','    let _ = reach;\n')
elif m=='10': rep(ER,'charge.push(n[p - 1].abs() + zero * g / pow(run, p - 1));','charge.push(n[p - 1].abs() + zero * g / pow(run, p - 1) * T::from_f64(0.5));')
elif m=='11': rep(RD,'let chord = (T::one() + (bend_rim * spread).powi(2) * third).powi(2);','let chord = (T::one() + (bend_rim * spread).powi(2) * third * T::from_f64(0.5)).powi(2);')
elif m=='12': rep(RD,'let chord = (T::one() + (bend_rim * spread).powi(2) * third).powi(2);','let chord = (T::one() + (bend_rim * spread).powi(2) * third).powi(2) * T::from_f64(0.5) + T::from_f64(0.5);')
