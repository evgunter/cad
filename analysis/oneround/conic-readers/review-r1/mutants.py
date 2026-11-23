import subprocess,sys,os,json
R='/home/user/cad'
C=R+'/crates/geom/src/curves/conic.rs'; V=R+'/crates/geom/src/cover.rs'
M={
 'M01_cover_back_on_derived_centre':(C,"Held::Arc { .. } => Cover::arc_of(*self, window),","Held::Arc { .. } => Cover::arc(&Conic::frame(self), self.semi_axes(), window),"),
 'M02_extent_back_on_derived_centre':(C,"""            Held::Arc { phase } => {
                let (lo, hi) = offset_extent(window, p, q, phase);
                (at + lo, at + hi)
            }""","""            Held::Arc { .. } => window.extent((Conic::frame(self).origin() - origin).dot(n), p, q),"""),
 'M03_bulge_halved':(V,"let bulge = -ellipse.second_derivative_at(s0 + half) * shift;","let bulge = -ellipse.second_derivative_at(s0 + half) * shift * T::from_f64(0.5);"),
 'M04_piece_dropped':(V,"""            lo + quarter + quarter,
            lo + quarter + quarter + quarter,
        ];
        starts""","""            lo + quarter + quarter,
        ];
        starts"""),
 'M05_support_ends_only':(C,"(r0.min(r1).min(lo), r0.max(r1).max(hi))\n}","(r0.min(r1), r0.max(r1))\n}"),
 'M06_near_without_bulge':(V,"let near = (segment_range(x0, x1, site).0 - bulge.norm()).max(T::zero());","let near = segment_range(x0, x1, site).0;"),
 'M07_ball_chord_halved':(V,"let chord = T::from_f64(2.0) * (reach * half).sin();","let chord = (reach * half).sin();"),
 'M08_far_ends_only':(V,"let far = [x0, x0 + bulge, x1, x1 + bulge]","let far = [x0, x1]"),
 'M09_trait_v_ref_negated_ellipse':(C,"""    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)""","""    fn v_ref(&self) -> Vec3<T> {
        -self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)"""),
 'M10_trait_u_v_swapped_ellipse':(C,"""    fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)""","""    fn u_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    fn v_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    fn semi_axes(&self) -> (T, T) {
        Ellipse::semi_axes(self)"""),
 'M11_inherent_ellipse_v_ref_negated':(C,"""    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    /// `(semi_u, semi_v)` in stored order.""","""    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        -self.at.v_ref()
    }

    /// `(semi_u, semi_v)` in stored order."""),
 'M12_ref_forward_extent_dropped':(C,"""    fn extent_along(&self, window: &ArcWindow<T>, origin: Point3<T>, n: Vec3<T>) -> (T, T) {
        (**self).extent_along(window, origin, n)
    }
    fn cover_over(&self, window: ParamWindow<T>) -> Option<Cover<T>> {
        (**self).cover_over(window)
    }
""",""),
 'M13_arc_of_frame_is_garbage':(V,"let frame = Conic::frame(&ellipse);","let frame = Conic::frame(&ellipse).translated(Vec3::new(T::from_f64(1e9), T::from_f64(-3e8), T::from_f64(7e7)));"),
 'M14_trough_sign':(C,"let trough = -(-k).select_le_zero(amp + k, ratio);","let trough = -(-k).select_le_zero(amp - k, ratio);"),
 'M15_crest_cancelling_form':(C,"let crest = k.select_le_zero(amp - k, ratio);","let crest = amp - k;"),
 'M16_circle_inherent_axis_negated':(C,"""    /// The unit plane normal ([`Conic::axis`]).
    #[must_use]
    pub fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    /// The unit seam ([`Conic::u_ref`]).
    #[must_use]
    pub fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        self.at.v_ref()
    }

    /// The radius.""","""    /// The unit plane normal ([`Conic::axis`]).
    #[must_use]
    pub fn axis(&self) -> Vec3<T> {
        self.at.axis()
    }

    /// The unit seam ([`Conic::u_ref`]).
    #[must_use]
    pub fn u_ref(&self) -> Vec3<T> {
        self.at.u_ref()
    }

    /// `axis × u_ref` ([`Conic::v_ref`]).
    #[must_use]
    pub fn v_ref(&self) -> Vec3<T> {
        -self.at.v_ref()
    }

    /// The radius."""),
}
only=sys.argv[1:] 
env=dict(os.environ,CARGO_TARGET_DIR='/home/user/tgt-head')
out=open('/home/user/logs/mutants.txt','a')
for name,(f,a,b) in M.items():
    if only and name not in only: continue
    s=open(f).read()
    if s.count(a)!=1:
        out.write(f'{name}: PATTERN COUNT {s.count(a)}\n'); out.flush(); continue
    open(f,'w').write(s.replace(a,b))
    d=subprocess.run(['git','diff','--',f],cwd=R,capture_output=True,text=True).stdout
    open(f'/home/user/scratch/mut/{name}.patch','w').write(d)
    r=subprocess.run(['cargo','nextest','run','-p','geom','--no-fail-fast','-E','not test(four_thousand_samples)'],cwd=R,env=env,capture_output=True,text=True)
    open(f,'w').write(s)
    log=r.stdout+r.stderr
    open(f'/home/user/logs/mut-{name}.log','w').write(log)
    fails=sorted(set(l.split(') ',1)[1].strip() for l in log.splitlines() if l.strip().startswith('FAIL [') and ') ' in l))
    summ=[l.strip() for l in log.splitlines() if 'Summary' in l or 'could not compile' in l]
    out.write(f'{name}: {summ} FAILS={fails}\n'); out.flush()
out.write('ALLDONE\n')
