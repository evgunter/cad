"""Delta-review mutants for PR #3844 at ae4b1dbe. Usage:
python3 probes/delta3844_mutate.py <M> ; run the rows ; git checkout crates/topo/src
"""
import sys
m = sys.argv[1]
OPS = 'crates/topo/src/boolean/ops.rs'
PROPS = 'crates/topo/src/props.rs'

def rep(path, a, b):
    s = open(path).read()
    assert s.count(a) == 1, (m, a)
    open(path, 'w').write(s.replace(a, b))

ARM1 = '            let (mut margin, mut area) = (Interval::zero(), Interval::zero());\n'
if m == 'skip':  # refuse on the walk's f64 sign: no re-derivation
    rep(OPS, ARM1, '            return Err(implausible());\n            #[allow(unreachable_code)]\n' + ARM1)
elif m.startswith('widen'):  # the interval margin widened by W m^3 each side
    w = m[len('widen'):]
    rep(OPS, '            match geom_core::k_stats::decide_invariant(name, margin / area, violation) {',
        f'            let margin = margin + Interval::from_bounds(-{w}, {w});\n'
        '            match geom_core::k_stats::decide_invariant(name, margin / area, violation) {')
elif m == 'plusv_exact':  # the +V arm decided at the exact band (the pre-fix reading)
    rep(OPS, 'Posture::PlusV { band } => (band, "volume_backstop_positive"),',
        'Posture::PlusV { .. } => (crate::splitting::order::exact_band().unwrap(), "volume_backstop_positive"),')
elif m == 'plusv_drop':
    rep(OPS, '    let (violation, name) = posture.violation();\n',
        '    if matches!(posture, Posture::PlusV { .. }) { return Ok(()); }\n    let (violation, name) = posture.violation();\n')
elif m == 'nolift':  # each closed-form face collapsed to a point at its midpoint (no rounding width)
    rep('crates/topo/src/props/quad_lane.rs', '''    super::closed_form_of(
        &surface.map_scalar(Interval::from_certified),
        &loops,
        sense,
        band,
    )
}''', '''    super::closed_form_of(
        &surface.map_scalar(Interval::from_certified),
        &loops,
        sense,
        band,
    )
    .map(|c| {
        let pt = |i: Interval| Interval::from_bounds(0.5 * (i.lo() + i.hi()), 0.5 * (i.lo() + i.hi()));
        FaceContribution { flux: pt(c.flux), area: pt(c.area) }
    })
}''')
elif m == 'nocap':  # the ∪ ≤ A + B arm dropped
    rep(OPS, '            if ba && bb {\n                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);',
        '            if false && ba && bb {\n                let (small, large) = (&mut [(r_, &mut cr)], &mut [(a_, &mut ca), (b_, &mut cb)]);')
elif m == 'nofloor':  # the ∖ ≥ A − B arm dropped
    rep(OPS, '            if ba && bb {\n                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr), (b_, &mut cb)]);',
        '            if false && ba && bb {\n                let (small, large) = (&mut [(a_, &mut ca)], &mut [(r_, &mut cr), (b_, &mut cb)]);')
elif m == 'noarm1':  # arm 1 never fires (the MAJ-1 row's own mutant)
    rep(OPS, '        if geom_core::k_stats::decide_invariant(name, metered, violation) == Ok(Sign::Negative) {',
        '        if false && geom_core::k_stats::decide_invariant(name, metered, violation) == Ok(Sign::Negative) {')
else:
    raise SystemExit(m)
