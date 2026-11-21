"""Reviewer r1 delta-read mutants, against b303a32a6. python3 mutants2.py <name> | --list"""
import sys
C='crates/geom/src/curves/conic.rs'; V='crates/geom/src/curves.rs'; M_='crates/mesh/src/memo.rs'; S='crates/step-import/src/build.rs'
M={
 'same-ignores-form': (C,"(Held::Centre, Held::Arc { .. }) | (Held::Arc { .. }, Held::Centre) => false,","(Held::Centre, Held::Arc { .. }) | (Held::Arc { .. }, Held::Centre) => true,"),
 'same-ignores-phase': (C,"same_scalar(p.0, q.0) && same_scalar(p.1, q.1)","{ let _ = (p, q, &same_scalar); true }"),
 'map-scalar-drops-phase': (C,"held: self.held.with_phase(|(c0, s0)| (f(c0), f(s0))),","held: self.held.with_phase(|_| (U::one(), U::zero())),"),
 'data-omits-phase': (V,"Some(phase) => {\n                        let [a, b, c, d, f] = walk;\n                        AnalyticData::new([a, b, c, d, f, (D::Phase, Direction(phase))])\n                    }","Some(_phase) => AnalyticData::new(walk),"),
 'azimuth-ignores-sine': (C,"w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0),","w.dot(self.at.v_ref()),"),
 'azimuth-ignores-cosine': (C,"self.semi_u * c0 + w.dot(self.at.u_ref()),","self.semi_u + w.dot(self.at.u_ref()),"),
 'mapped-drops-phase': (C,"            at: map(&self.at),\n            ..*self","            at: map(&self.at),\n            held: self.held.with_phase(|_| (T::one(), T::zero())),\n            ..*self"),
 'reversal-keeps-sine': (C,"held: self.held.with_phase(|(c0, s0)| (c0, -s0)),","held: self.held.with_phase(|(c0, s0)| (c0, s0)),"),
 'quarter-wrong-way': (C,"(turn.select_le_zero(c0, s0), turn.select_le_zero(s0, -c0))","(turn.select_le_zero(c0, -s0), turn.select_le_zero(s0, c0))"),
 'half-keeps-pair': (C,"(back.select_le_zero(c0, -c0), back.select_le_zero(s0, -s0))","(c0, s0)"),
 'square-uncharged': (C,"    let sq = transcendental(sh).square();","    let sq = { let t = transcendental(sh); Rounded { value: t.value * t.value, error: T::from_f64(2.0) * t.value.abs() * t.error } };"),
 'sine-uncharged': (C,"        transcendental(s) - Rounded::exact(s0),","        Rounded::exact(s) - Rounded::exact(s0),"),
 'half-sine-uncharged': (C,"    let sq = transcendental(sh).square();","    let sq = Rounded::exact(sh).square();"),
 'centre-sine-sign': (C,"                    - self.at.v_ref() * (self.semi_v * s0),","                    + self.at.v_ref() * (self.semi_v * s0),"),
 'eval-reads-centre': (C,"""            Held::Arc { phase } => {
                let (ku, kv) = offsets(theta, phase);
                let (u, v) = (self.at.u_ref(), self.at.v_ref());
                self.at.origin() + (u * (self.semi_u * ku) + v * (self.semi_v * kv))
            }""","""            Held::Arc { .. } => {
                let f = Conic::frame(self);
                let ((s, c), v_ref) = azimuth::basis(&f, theta);
                f.origin() + (f.u_ref() * (self.semi_u * c) + v_ref * (self.semi_v * s))
            }"""),
 'memo-mark-removed': (M_,"                        self.tag(datum.name().as_bytes());","                        let _ = datum;"),
 'sig-drops-phase': (S,"                    data.into_iter()\n                        .flat_map(|(_, value)| value.scalars())","                    data.into_iter()\n                        .filter(|(d, _)| *d != geom::CurveDatum::Phase)\n                        .flat_map(|(_, value)| value.scalars())"),
}
if sys.argv[1]=='--list': print('\n'.join(M)); sys.exit()
f,old,new=M[sys.argv[1]]
s=open(f).read(); assert s.count(old)==1,(sys.argv[1],s.count(old)); open(f,'w').write(s.replace(old,new))
