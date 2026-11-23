import subprocess, sys, os
R='/home/user/h4/'
C='crates/geom/src/cover.rs'; K='crates/geom/src/curves/conic.rs'
M={
 'D01 bulge halved': (C, "                * shift;\n            let (x0, x1)", "                * (shift * T::from_f64(0.5));\n            let (x0, x1)"),
 'D02 one piece dropped': (C, "        starts[1..]\n            .iter()", "        starts[1..3]\n            .iter()"),
 'D03 near without the bulge': (C, "let near = (segment_range(x0, x1, site).0 - bulge.norm()).max(T::zero());\n            let far = [x0, x0 + bulge", "let near = segment_range(x0, x1, site).0;\n            let far = [x0, x0 + bulge"),
 'D04 far without the bulged corners': (C, "let far = [x0, x0 + bulge, x1, x1 + bulge]", "let far = [x0, x1]"),
 'D05 ball by the smaller semi-axis': (C, "            self.ellipse.speed_hi() * chord,", "            self.ellipse.speed_lo() * chord,"),
 'D06 chord ball halved': (C, "        let chord = T::from_f64(2.0) * (reach * half).sin();\n        (\n            self.ellipse", "        let chord = (reach * half).sin();\n        (\n            self.ellipse"),
 'D07 bulge_factor at h not h/2 (under, 2sin^2 of quarter/8)': (C, "T::from_f64(2.0) * (quarter * T::from_f64(0.25)).sin().powi(2)", "T::from_f64(2.0) * (quarter * T::from_f64(0.125)).sin().powi(2)"),
 'D08 bulge_factor without the 2': (C, "T::from_f64(2.0) * (quarter * T::from_f64(0.25)).sin().powi(2)", "(quarter * T::from_f64(0.25)).sin().powi(2)"),
 'D09 over_anchor clamp dropped (vacuous)': (K, "k.select_le_zero(amp - k, ratio).min(amp + k.abs())", "k.select_le_zero(amp - k, ratio)"),
 'D10 over_anchor clamp at A (cuts k<0)': (K, "k.select_le_zero(amp - k, ratio).min(amp + k.abs())", "k.select_le_zero(amp - k, ratio).min(amp)"),
 'D11 over_anchor clamp at max(A,|k|)': (K, "k.select_le_zero(amp - k, ratio).min(amp + k.abs())", "k.select_le_zero(amp - k, ratio).min(amp.max(k.abs()))"),
 'D12 over_anchor ratio arm only': (K, "k.select_le_zero(amp - k, ratio).min(amp + k.abs())", "ratio.min(amp + k.abs())"),
 'D13 over_anchor ratio halved': (K, "let ratio = cross.powi(2) / (amp + k)", "let ratio = cross.powi(2) * T::from_f64(0.5) / (amp + k)"),
 'D14 trough as the crest (sign of k kept)': (K, "let trough = -over_anchor(amp, -k, cross);", "let trough = -over_anchor(amp, k, cross);"),
 'D15 support at the ends only': (K, "    (r0.min(r1).min(lo), r0.max(r1).max(hi))\n}", "    let _ = (lo, hi);\n    (r0.min(r1), r0.max(r1))\n}"),
 'D16 v_ref sign flipped (Conic for Ellipse)': (K, "    fn v_ref(&self) -> Vec3<T> {\n        self.at.v_ref()\n    }\n\n    fn semi_axes(&self) -> (T, T) {\n        Ellipse::semi_axes(self)", "    fn v_ref(&self) -> Vec3<T> {\n        -self.at.v_ref()\n    }\n\n    fn semi_axes(&self) -> (T, T) {\n        Ellipse::semi_axes(self)"),
 'D18 over_anchor widened by a constant 4e-7 m': (K, "k.select_le_zero(amp - k, ratio).min(amp + k.abs())", "k.select_le_zero(amp - k, ratio).min(amp + k.abs()) + T::from_f64(4e-7)"),
 'D19 Arc::far bulge halved (centre-held)': (C, "        let shift = bulge_factor(quarter);\n        let point = |t: T| self.point(t);", "        let shift = bulge_factor(quarter) * T::from_f64(0.5);\n        let point = |t: T| self.point(t);"),
 'D17 Arc::far bulge zeroed (centre-held)': (C, "        let shift = bulge_factor(quarter);\n        let point = |t: T| self.point(t);", "        let shift = bulge_factor(quarter) * T::zero();\n        let point = |t: T| self.point(t);"),
}
mode=sys.argv[1]; which=sys.argv[2:]
for name in M:
    if which and not any(name.startswith(w+' ') for w in which): continue
    f,old,new=M[name]; path=R+f; orig=open(path).read()
    c=orig.count(old)
    if c!=1: print(name,'PATCH COUNT',c,flush=True); continue
    open(path,'w').write(orig.replace(old,new))
    try:
        env=dict(os.environ, CARGO_TARGET_DIR='/home/user/tgt-h4')
        if mode=='rows':
            cmd=['cargo','nextest','run','-p','geom','--no-fail-fast','--run-ignored','all','-E','test(/held_arc|a_far_sections|review_sweep|delta_h1/)']
        else:
            cmd=['cargo','nextest','run','-p','geom','--no-fail-fast','-E','not test(/review_|delta_/)']
        r=subprocess.run(cmd,cwd=R,env=env,capture_output=True,text=True,timeout=1800)
        out=r.stdout+r.stderr
        fails=sorted(set(l.split(')')[-1].strip() for l in out.splitlines() if l.strip().startswith('FAIL [')))
        summ=[l.strip() for l in out.splitlines() if 'Summary' in l]
        err='BUILD-ERR' if 'error[' in out else ''
        print(f'{name}: {err} {summ[-1] if summ else ""}\n    '+'\n    '.join(fails[:8]),flush=True)
    finally:
        open(path,'w').write(orig)
