import sys, pathlib
R = pathlib.Path('/home/user/cad/crates/geom-core/src')
P, S, Q = R/'sym/point.rs', R/'sym.rs', R/'sym/question.rs'
M = {
 'M9':  [(P, "    out.filter(|x| x.is_certified())\n", "    out\n")],
 'M10': [(S, "                && sess.params.insert(indet_param(symbol.0), (lo, hi)) != Some((lo, hi))\n            {\n                sess.points.clear();",
             "                && sess.params.insert(indet_param(symbol.0), (lo, hi)) != Some((lo, hi))\n            {\n                let _ = 0;")],
 'M10b':[(S, "sess.params.insert(indet_param(symbol.0), (lo, hi)) != Some((lo, hi))", "sess.params.insert(indet_param(symbol.0), (lo, hi)).is_none()")],
 'M2':  [(P, "        Some(d) if Bounds::hi(d) <= 0.0 => &[1],", "        Some(d) if Bounds::hi(d) <= 0.0 => &[2],")],
 'M3':  [(P, "        Some(d) if Bounds::hi(d) <= 0.0 => &[1],\n        Some(d) if Bounds::lo(d) > 0.0 => &[2],", "")],
 'MQ':  [(Q, "    fn proves_zero(self) -> Option<ProvedZero> {\n", "    fn proves_zero(self) -> Option<ProvedZero> {\n        if super::point::refutes(self) { return None; }\n")],
 'MR':  [(S, "        if report::active() && refuted {", "        if report::active() && refuted && false {")],
 'MC':  [(P, "    let f = if z & 1 == 1 { 1.0 - f } else { f };", "    let f = if z & 1 == 1 { 1.0 - f } else { f }; let f = 0.5 + 0.0 * f;")],
 'MNOMEMO': [(P, "    sess.points = memo;", "    let _ = memo;")],
 'MEQ':  [(P, "Some(d) if Bounds::hi(d) <= 0.0 => &[1],", "Some(d) if Bounds::hi(d) < 0.0 => &[1],")],
 'MDUAL':[(S, "impl<T> PointReading for crate::Dual<T> {\n    fn refutes(_: SymId) -> bool {\n        false", "impl<T> PointReading for crate::Dual<T> {\n    fn refutes(n: SymId) -> bool {\n        point::refutes(n)")],
}
name, act = sys.argv[1], sys.argv[2]
for f, a, b in M[name]:
    t = f.read_text()
    if act == 'apply':
        assert t.count(a) == 1, (name, f, t.count(a)); f.write_text(t.replace(a, b))
