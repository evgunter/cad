import subprocess, sys, os
R='/home/user/cad'
M = {
 'M1_reverse_no_negate': ('crates/profile/src/lib.rs', 'Self::Line(d) => Self::Line(-d),', 'Self::Line(d) => Self::Line(d),'),
 'M2_reverse_no_swap': ('crates/profile/src/lib.rs', '            Self::Arc { start, end } => Self::Arc {\n                start: end,\n                end: start,\n            },', '            Self::Arc { start, end } => Self::Arc { start, end },'),
 'M3_lift_rederives': ('crates/profile/src/validate.rs', '            directions: self.directions.map(U::from_f64),', '            directions: match self.directions { SegmentDirections::Line(_) => SegmentDirections::Line(((self.end - self.start) / self.start.distance(self.end)).map(U::from_f64)), d => d.map(U::from_f64) },'),
 'M4_check_accepts_all': ('crates/profile/src/seg.rs', '    let miss = (o + d * len).distance(p);', '    if true { return Ok(()); }\n    let miss = (o + d * len).distance(p);'),
 'M5_flip_sigma': ('crates/profile/src/path.rs', '    let away = |t: Vec2<T>| Vec2::new(t.y, -t.x) * sense.sign::<T>();', '    let away = |t: Vec2<T>| Vec2::new(t.y, -t.x) * -sense.sign::<T>();'),
 'M6_point_target_director': ('crates/profile/src/path.rs', '    let head = core.head()?;\n    core.push_line(end)?;', '    let head = core.head()?;\n    core.push_line(end)?;\n    core.direct_leaving(core.verts.len() - 2, ang);'),
 'M7_circle_radials_swapped': ('crates/profile/src/path.rs', '[Vec2::new(one, zero), Vec2::new(-one, zero)];', '[Vec2::new(-one, zero), Vec2::new(one, zero)];'),
 'M8_bulge_sign_ignored': ('crates/geom-core/src/arc.rs', '        let scale = sign / (T::one() + x2);', '        let scale = (sign * sign) / (T::one() + x2);'),
 'M9_no_radial_registration': ('crates/profile/src/lib.rs', '            if let Some(radials) = self.radials {\n                for (fact', '            if let Some(radials) = self.radials.filter(|_| false) {\n                for (fact'),
 'M10_onto_no_swap': ('crates/profile/src/path.rs', '        radials: built.radials.map(|[from, to]| [to, from]),', '        radials: built.radials,'),
 'M11_split_end_radial_k': ('crates/profile/src/path.rs', '            let arc = arc.radial([radials[k], radials[(k + 1) % n]]);', '            let arc = arc.radial([radials[k], radials[k]]);'),
 'M13_tangent_records_none': ('crates/profile/src/path.rs', '            .radial([away(u), away(end_ang.unit)])', '            .sensed(sense)'),
 'M14_bulge_records_none': ('crates/profile/src/lib.rs', '            Some(sign) => built.radial(Arc2::from_chord_radials(start, end, bulge, sign).1),', '            Some(_sign) => built,'),
 'M12_axis_sign_flip': ('crates/profile/src/seg.rs', '        [Some(Sign::Zero), Some(s)] => unit(s).map_or(quotient, |s| Vec2::new(T::zero(), s)),', '        [Some(Sign::Zero), Some(s)] => unit(s).map_or(quotient, |s| Vec2::new(T::zero(), -s)),'),
}
env = dict(os.environ, CARGO_TARGET_DIR='/home/user/review-profile-directions-target', CARGO_PROFILE_DEV_OPT_LEVEL='1', CARGO_PROFILE_TEST_OPT_LEVEL='1', CARGO_PROFILE_TEST_DEBUG='line-tables-only', CARGO_PROFILE_DEV_DEBUG='line-tables-only')
env.pop('CAD_DIR_PROBE', None)
names = sys.argv[1:] or list(M)
for name in names:
    f, old, new = M[name]
    p = os.path.join(R, f); s = open(p).read()
    assert s.count(old) == 1, (name, s.count(old))
    open(p, 'w').write(s.replace(old, new))
    try:
        pk = ['-p', 'profile'] + (['-p', 'geom-core'] if 'geom-core' in f else [])
        r = subprocess.run(['cargo', 'nextest', 'run', *pk, '--no-fail-fast', '-E', 'test(/recorded_directions|path_differential|from_chord_radials|register_radials/)'], cwd=R, env=env, capture_output=True, text=True)
        out = r.stdout + r.stderr
        fails = [l.strip() for l in out.splitlines() if l.strip().startswith('FAIL [')]
        summ = [l.strip() for l in out.splitlines() if 'Summary' in l or l.startswith('error')]
        print(name, 'RED' if fails else ('BUILD-ERR' if 'error' in ''.join(summ) or not summ else 'GREEN(survived)'), summ[-1:] , *sorted(set(x.split(')')[-1].strip() for x in fails)), sep='\n  ', flush=True)
    finally:
        open(p, 'w').write(s)
