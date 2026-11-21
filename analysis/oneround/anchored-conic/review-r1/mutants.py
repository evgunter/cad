"""Reviewer r1's mutants of crates/geom/src/curves/conic.rs (and curves.rs).
python3 mutants.py <name> applies one; python3 mutants.py --list lists them.
Each is an exact replacement that must match once."""
import sys
C='crates/geom/src/curves/conic.rs'; V='crates/geom/src/curves.rs'
M={
 # The anchored ellipse's evaluator reads its derived centre.
 'eval-reads-centre': (C,
  """            Some(anchor) => {
                let (ku, kv) = offsets(theta, self.phase);
                let (u, v) = (self.at.u_ref(), self.at.v_ref());
                anchor + (u * (self.semi_u * ku) + v * (self.semi_v * kv))
            }""",
  """            Some(_anchor) => {
                let f = Conic::frame(self);
                let ((s, c), v_ref) = azimuth::basis(&f, theta);
                f.origin() + (f.u_ref() * (self.semi_u * c) + v_ref * (self.semi_v * s))
            }"""),
 # The quarter turn rotates the pair the wrong way.
 'quarter-wrong-way': (C,
  "phase: (turn.select_le_zero(c0, s0), turn.select_le_zero(s0, -c0)),",
  "phase: (turn.select_le_zero(c0, -s0), turn.select_le_zero(s0, c0)),"),
 # same_content ignores the form.
 'same-ignores-form': (C,
  """        self.born == other.born
            && same(&self.at, &other.at)
            && (self.born == Born::Centred""",
  """        same(&self.at, &other.at)
            && (self.born == Born::Centred"""),
 # map_scalar drops the phase.
 'map-scalar-drops-phase': (C,
  "phase: (f(self.phase.0), f(self.phase.1)),",
  "phase: (U::one(), U::zero()),"),
 # data() leaves the phase out of the walk.
 'data-omits-phase': (V,
  "Some(phase) => {\n                        let [a, b, c, d, f] = walk;\n                        AnalyticData::new([a, b, c, d, f, (D::Phase, Direction(phase))])\n                    }",
  "Some(_phase) => AnalyticData::new(walk),"),
 # The derived centre's sine term has the wrong sign.
 'centre-sine-sign': (C,
  "                        - self.at.v_ref() * (self.semi_v * s0),",
  "                        + self.at.v_ref() * (self.semi_v * s0),"),
 # Each sine is charged half an ulp of what it is.
 'sines-half-charged': (C,
  "error: T::from_f64(2.0 * geom_core::UNIT_ROUNDOFF) * x.abs(),",
  "error: T::from_f64(1.0 * geom_core::UNIT_ROUNDOFF) * x.abs(),"),
 # The anchored azimuth ignores the phase's sine.
 'azimuth-ignores-sine': (C,
  "w.dot(self.at.v_ref()) - (T::zero() - self.semi_v * s0),",
  "w.dot(self.at.v_ref()),"),
 # The anchored azimuth ignores the phase's cosine.
 'azimuth-ignores-cosine': (C,
  "self.semi_u * c0 + w.dot(self.at.u_ref()),",
  "self.semi_u + w.dot(self.at.u_ref()),"),
 # The point at parameter zero is read as the anchor after a re-seam.
 'zero-point-is-anchor': (C,
  "            Some(_) => self.point_at(T::zero()),",
  "            Some(p) => p,"),
 # The half turn keeps the pair.
 'half-keeps-pair': (C,
  "phase: (back.select_le_zero(c0, -c0), back.select_le_zero(s0, -s0)),",
  "phase: (c0, s0),"),
 # The anchored circle's azimuth reads from the anchor as a centre.
 'circle-azimuth-centre': (C,
  "Azimuth::of(w.dot(self.at.v_ref()), self.radius + w.dot(self.at.u_ref()))",
  "Azimuth::of(w.dot(self.at.v_ref()), w.dot(self.at.u_ref()))"),
}
if sys.argv[1]=='--list': print('\n'.join(M)); sys.exit()
f,old,new=M[sys.argv[1]]
s=open(f).read(); assert s.count(old)==1,(sys.argv[1],s.count(old)); open(f,'w').write(s.replace(old,new))
