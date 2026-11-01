"""Apply review mutants to the head tree: m1 arc ball off, m2 centring
off, m3 slack off (any combination, comma-separated)."""
import sys
root = '/home/user/cad/crates/topo/src/'
which = sys.argv[1].split(',')


def edit(path, pairs):
    p = root + path
    s = open(p).read()
    for old, new in pairs:
        assert s.count(old) >= 1, (path, old)
        s = s.replace(old, new)
    open(p, 'w').write(s)


if 'm1' in which:
    edit('splitting/containment.rs', [(
        "conic_arc_ball(curve.carrier(), curve.params()).unwrap_or(ball),",
        "ball,")])
if 'm2' in which:
    edit('boolean/solid_contain.rs', [
        ("""        About::Nearest => ((T::zero() - w0p.dot(dp)) / a2)
            .max(T::zero() - span)
            .min(span),""", "        About::Nearest => T::zero(),"),
        ("        About::Nearest => (T::zero() - w0.dot(d)) / a2,",
         "        About::Nearest => T::zero(),"),
        ("    let shift = (T::zero() - w0.dot(d)) / d.norm_squared();",
         "    let shift = T::zero();"),
    ])
if 'm3' in which:
    edit('boolean/solid_contain.rs', [
        ("""        let slack = residual / (T::from_f64(2.0) * disc.sqrt())
            + geom_core::rounding_charge(self.shift.abs() + s);""",
         """        let slack = (residual / (T::from_f64(2.0) * disc.sqrt())
            + geom_core::rounding_charge(self.shift.abs() + s))
            * T::zero();"""),
        ("""            slack: geom_core::rounding_charge((origin - q).norm() + t.abs() * d.norm())
                / denom.abs(),""", "            slack: T::zero(),"),
        ("push(t, geom_core::rounding_charge(t.abs()), RootSide::Torus);",
         "push(t, T::zero(), RootSide::Torus);"),
        ("                        slack: geom_core::rounding_charge(t.abs()),",
         "                        slack: T::zero(),"),
    ])
print('applied', which)
