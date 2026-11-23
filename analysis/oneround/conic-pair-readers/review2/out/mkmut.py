import subprocess
W='/home/user/cad-mut'; OUT='/home/user/cad/analysis/oneround/conic-pair-readers/review2/mutants'
C='crates/geom/src/curves/conic.rs'; P='crates/geom-brep/src/curve_pair/pose.rs'; Wn='crates/geom-brep/src/curve_pair/pose/window.rs'; PD='crates/geom-brep/src/curve_face/pose.rs'
M={
'S1-foot-seed-from-centre':(Wn,'''        self.b
            .azimuth(q)
            .principal_as(super::row::BRANCH, Band::least())''','''        #[allow(clippy::disallowed_methods)]
        let (x, y) = self.b.unit_of(q - self.b.frame().origin());
        geom::Azimuth::of(y, x).principal_as(super::row::BRANCH, Band::least())'''),
'S2-level-ddp-from-centre':(P,'''        let ddp = self.a.second_derivative_at(theta);
        let (f, (gx, gy)) = self.b.level_at(p);''','''        #[allow(clippy::disallowed_methods)]
        let ddp = self.a.frame().origin() - p;
        let (f, (gx, gy)) = self.b.level_at(p);'''),
'S3-unit-in-plane-from-centre':(C,'''        let (du, dv) = self.unit_of(p - self.at.origin());
        match self.held {
            Held::Centre => (du, dv),
            Held::Arc { phase: (c0, s0) } => (c0 + du, s0 + dv),
        }''','''        self.unit_of(p - Conic::frame(self).origin())'''),
'R1-over-level-dropped':(P,'''        let level = T::zero() - plane.height_of(o);
        let (lo, hi) = h.extremes();''','''        let level = T::zero() * plane.height_of(o);
        let (lo, hi) = h.extremes();'''),
'R4-level-jet-third-sign':(P,'''            T::zero() - (gx * x1 + gy * y1) + six * (x1 * x2 / sx + y1 * y2 / sy),''','''            (gx * x1 + gy * y1) + six * (x1 * x2 / sx + y1 * y2 / sy),'''),
'R9-window-height-fourth-zero':(Wn,'''        up::<16, _>(self.height.swing())''','''        up::<16, _>(self.height.swing() * T::zero())'''),
'T1-roots-old-tie':(C,'''    let nearer = (hi.abs() - lo.abs()).select_le_zero(crest_at, trough_at);''','''    let _ = (lo, hi);
    let nearer = h.select_le_zero(crest_at, trough_at);'''),
'T2-roots-nearer-swapped':(C,'''    let nearer = (hi.abs() - lo.abs()).select_le_zero(crest_at, trough_at);''','''    let nearer = (hi.abs() - lo.abs()).select_le_zero(trough_at, crest_at);'''),
'N1-graze-side-swapped':(PD,'''                Margin::of((hi.abs() - lo.abs()).select_le_zero(hi - r, lo + r)),''','''                Margin::of((hi.abs() - lo.abs()).select_le_zero(lo + r, hi - r)),'''),
'N2-arc-crest-halved':(C,'''                let (trough, crest) = offset_crests(self.p, self.q, phase);
                (self.at + trough, self.at + crest)''','''                let (trough, crest) = offset_crests(self.p, self.q, phase);
                (self.at + trough, self.at + crest * T::from_f64(0.5))'''),
'N5-level-fourth-drops-x0':(P,'''        fourth: up::<36, _>(side(ub, x0.0, squares.0) + side(vb, x0.1, squares.1)),''','''        fourth: up::<36, _>(side(ub, x0.0 * T::zero(), squares.0) + side(vb, x0.1 * T::zero(), squares.1)),'''),
'N7-one-curve-gap-uncharged':(P,'''    gap + geom_core::rounding_charge(reach(e) + reach(f) + gap)''','''    let _ = reach;
    gap'''),
'N8-slant-floor-turn-swapped':(P,'''        let turn = d_from.select_le_zero(h_lo, h_hi);''','''        let turn = d_from.select_le_zero(h_hi, h_lo);'''),
'N9-height-of-arc-from-centre':(C,'''    fn height_of(&self, p: Point3<T>) -> T {
        crate::frame::plane_offset(self.at.origin(), self.at.axis(), p)
    }''','''    fn height_of(&self, p: Point3<T>) -> T {
        crate::frame::plane_offset(Conic::frame(self).origin(), self.at.axis(), p)
    }'''),
'N10-plane-door-arc-graze-belly-from-hi-only':(PD,'''    let reach = hi.min(T::zero() - lo);''','''    let reach = hi.max(T::zero() - lo).min(hi.min(T::zero() - lo) + r * T::from_f64(1e-12));'''),
}
for name,(f,old,new) in M.items():
    src=open(f'{W}/{f}').read()
    assert src.count(old)==1,(name,src.count(old))
    open(f'{W}/{f}','w').write(src.replace(old,new))
    d=subprocess.run(['git','diff'],cwd=W,capture_output=True,text=True).stdout
    open(f'{OUT}/{name}.patch','w').write(d)
    subprocess.run(['git','checkout','--','.'],cwd=W,check=True)
    print(name,len(d.splitlines()))
