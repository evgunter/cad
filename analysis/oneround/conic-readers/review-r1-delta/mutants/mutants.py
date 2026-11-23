import subprocess,sys,os
R='/home/user/cad-mut'
C=R+'/crates/geom/src/curves/conic.rs'; V=R+'/crates/geom/src/cover.rs'
ARC_EXT="""            Held::Arc { phase } => {
                let (lo, hi) = offset_extent(window, p, q, phase);
                (at + lo, at + hi)
            }"""
VREF="""    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)"""
M={
 'A01_cover_back_on_derived_centre':(C,"Held::Arc { .. } => Cover::arc_of(*self, window),","Held::Arc { .. } => Cover::arc(&Conic::frame(self), self.semi_axes(), window),"),
 'A02_extent_back_on_derived_centre':(C,ARC_EXT,"            Held::Arc { .. } => window.extent((Conic::frame(self).origin() - origin).dot(n), p, q),"),
 'U01_bulge_halved_in_held_arc':(V,"                * shift;\n            let (x0, x1) = (point(s0), point(s1));","                * shift * T::from_f64(0.5);\n            let (x0, x1) = (point(s0), point(s1));"),
 'U02_piece_dropped':(V,"        starts[1..]\n            .iter()","        starts[1..3]\n            .iter()"),
 'U03_near_without_bulge':(V,"let near = (segment_range(x0, x1, site).0 - bulge.norm()).max(T::zero());","let near = segment_range(x0, x1, site).0;"),
 'U04_far_ends_only':(V,"let far = [x0, x0 + bulge, x1, x1 + bulge]","let far = [x0, x1]"),
 'U05_trough_sign':(C,"let trough = -over_anchor(amp, -k, cross);","let trough = -over_anchor(amp, k, cross);"),
 'U06_v_ref_negated_arc_held_only':(C,VREF,VREF.replace("self.at.v_ref()\n","match self.held {\n            Held::Centre => self.at.v_ref(),\n            Held::Arc { .. } => -self.at.v_ref(),\n        }\n")),
 'U07_ref_forwarders_dropped':(C,"""    fn extent_along(&self, window: &ArcWindow<T>, origin: Point3<T>, n: Vec3<T>) -> (T, T) {
        (**self).extent_along(window, origin, n)
    }
    fn cover_over(&self, window: ParamWindow<T>) -> Option<Cover<T>> {
        (**self).cover_over(window)
    }
""",""),
 'U08_crest_cancelling_form':(C,"k.select_le_zero(amp - k, ratio).min(amp + k.abs())","{ let _ = ratio; (amp - k).min(amp + k.abs()) }"),
 'U09_ceiling_dropped':(C,"k.select_le_zero(amp - k, ratio).min(amp + k.abs())","k.select_le_zero(amp - k, ratio)"),
 'U10_ceiling_at_amp':(C,"k.select_le_zero(amp - k, ratio).min(amp + k.abs())","k.select_le_zero(amp - k, ratio).min(amp)"),
 'U11_ceiling_at_amp_plus_k':(C,"k.select_le_zero(amp - k, ratio).min(amp + k.abs())","k.select_le_zero(amp - k, ratio).min(amp + k)"),
 'U13_select_reversed':(C,"k.select_le_zero(amp - k, ratio).min(amp + k.abs())","k.select_le_zero(ratio, amp - k).min(amp + k.abs())"),
 'U14_bulge_factor_halved_everywhere':(V,"T::from_f64(2.0) * (quarter * T::from_f64(0.25)).sin().powi(2)","(quarter * T::from_f64(0.25)).sin().powi(2)"),
 'U15_bulge_factor_one_minus_cos':(V,"T::from_f64(2.0) * (quarter * T::from_f64(0.25)).sin().powi(2)","T::one() - (quarter * T::from_f64(0.5)).abs().cos()"),
 'U16_ball_reach_uncapped':(V,"let reach = ((hi - lo) * half).abs().min(T::pi());\n        let chord = T::from_f64(2.0) * (reach * half).sin();\n        (\n            self.ellipse","let reach = ((hi - lo) * half).abs();\n        let chord = T::from_f64(2.0) * (reach * half).sin();\n        (\n            self.ellipse"),
 'U17_ball_by_speed_lo':(V,"self.ellipse.speed_hi() * chord,","self.ellipse.speed_lo() * chord,"),
 'U19_bulge_read_at_the_piece_start':(V,".second_derivative_at(s0 + quarter * T::from_f64(0.5))",".second_derivative_at(s0)"),
 'U22_end_offsets_phase_swapped':(C,"let (ku, kv) = offsets(t, phase);\n        p * ku + q * kv","let (ku, kv) = offsets(t, (phase.1, phase.0));\n        p * ku + q * kv"),
 'U23_centre_far_bulge_dropped':(V,"let shift = bulge_factor(quarter);\n        let point = |t: T| self.point(t);","let shift = bulge_factor(quarter) * T::zero();\n        let point = |t: T| self.point(t);"),
}
only=sys.argv[1:]
env=dict(os.environ,CARGO_TARGET_DIR='/home/user/tgt-census')
out=open('/home/user/logs2/mutants.txt','a')
for name,(f,a,b) in M.items():
    if only and name not in only: continue
    s=open(f).read()
    if s.count(a)!=1:
        out.write(f'{name}: PATTERN COUNT {s.count(a)}\n'); out.flush(); continue
    open(f,'w').write(s.replace(a,b))
    d=subprocess.run(['git','diff','--',f],cwd=R,capture_output=True,text=True).stdout
    open(f'/home/user/scratch/mut2/{name}.patch','w').write(d)
    r=subprocess.run(['cargo','nextest','run','-p','geom','--no-fail-fast','-E','not test(four_thousand_samples)'],cwd=R,env=env,capture_output=True,text=True)
    open(f,'w').write(s)
    log=r.stdout+r.stderr
    open(f'/home/user/logs2/mut-{name}.log','w').write(log)
    fails=sorted(set(l.split(') ',1)[1].strip() for l in log.splitlines() if l.strip().startswith('FAIL [') and ') ' in l))
    summ=[l.strip() for l in log.splitlines() if 'Summary' in l or 'could not compile' in l]
    out.write(f'{name}: {summ} FAILS={fails}\n'); out.flush()
out.write('ALLDONE\n')
