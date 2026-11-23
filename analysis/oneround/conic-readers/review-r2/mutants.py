import subprocess, sys, os, shutil
R='/home/user/cad/'
M={
 'M1 arc-held cover on the derived centre frame': ('crates/geom/src/curves/conic.rs',
   "            Held::Arc { .. } => Cover::arc_of(*self, window),",
   "            Held::Arc { .. } => Cover::arc(&Conic::frame(self), self.semi_axes(), window),"),
 'M2 arc-held extent on the derived centre': ('crates/geom/src/curves/conic.rs',
   """            Held::Arc { phase } => {
                let (lo, hi) = offset_extent(window, p, q, phase);
                (at + lo, at + hi)
            }""",
   """            Held::Arc { .. } => window.extent((Conic::frame(self).origin() - origin).dot(n), p, q),"""),
 'M3 bulge halved': ('crates/geom/src/cover.rs',
   "let bulge = -ellipse.second_derivative_at(s0 + half) * shift;",
   "let bulge = -ellipse.second_derivative_at(s0 + half) * (shift * T::from_f64(0.5));"),
 'M4 one of four pieces dropped': ('crates/geom/src/cover.rs',
   """            lo + quarter + quarter + quarter,
        ];""",
   """        ];"""),
 'M5 support at the ends only': ('crates/geom/src/curves/conic.rs',
   "    (r0.min(r1).min(lo), r0.max(r1).max(hi))\n}",
   "    let _ = (lo, hi);\n    (r0.min(r1), r0.max(r1))\n}"),
 'M6 near without the bulge': ('crates/geom/src/cover.rs',
   "let near = (segment_range(x0, x1, site).0 - bulge.norm()).max(T::zero());",
   "let near = segment_range(x0, x1, site).0;"),
 'M7 far without the bulged corners': ('crates/geom/src/cover.rs',
   "let far = [x0, x0 + bulge, x1, x1 + bulge]",
   "let far = [x0, x1]"),
 'M8 ball by the smaller semi-axis': ('crates/geom/src/cover.rs',
   "ellipse.speed_hi() * chord,",
   "ellipse.speed_lo() * chord,"),
 'M9 v_ref sign flipped (Conic for Ellipse)': ('crates/geom/src/curves/conic.rs',
   """    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)""",
   """    fn v_ref(&self) -> Vec3<T> {
        -self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)"""),
 'M10 u_ref and v_ref swapped (Conic for Ellipse)': ('crates/geom/src/curves/conic.rs',
   """    fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)""",
   """    fn u_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)"""),
 'M11 crest ratio replaced by A-k (cancelling form)': ('crates/geom/src/curves/conic.rs',
   "    let crest = k.select_le_zero(amp - k, ratio);",
   "    let crest = amp - k;"),
 'M12 chord ball half-chord (sin(reach/2) not 2sin)': ('crates/geom/src/cover.rs',
   "let chord = T::from_f64(2.0) * (reach * half).sin();",
   "let chord = (reach * half).sin();"),
}
which = sys.argv[1:] or list(M)
for name in M:
    if not any(name.startswith(w+' ') for w in which): continue
    f,old,new = M[name]
    path=R+f; orig=open(path).read()
    assert orig.count(old)==1, (name, orig.count(old))
    open(path,'w').write(orig.replace(old,new))
    try:
        env=dict(os.environ, CARGO_TARGET_DIR='/home/user/tgt-head')
        r=subprocess.run(['cargo','test','-q','-p','geom','--lib','--','--test-threads','4','--nocapture',
            'a_far_sections_cover_holds_its_stored_arc','review_sweep','review_interval_support'],cwd=R,env=env,capture_output=True,text=True,timeout=900)
        out=r.stdout+r.stderr
        author = 'FAILED' if 'cover::tests::a_far_sections_cover_holds_its_stored_arc' in out.split('failures:')[-1] else ('BUILD-ERR' if 'error[' in out else 'ok')
        summ=[l for l in out.splitlines() if 'SUMMARY' in l]
        print(f'{name}: author row {author}; '+' | '.join(summ), flush=True)
    finally:
        open(path,'w').write(orig)
