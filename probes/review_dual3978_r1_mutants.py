import sys
m=sys.argv[1]
def sub(path, a, b):
    s=open(path).read(); assert s.count(a)==1, (m, a[:40], s.count(a)); open(path,'w').write(s.replace(a,b))
sc='crates/topo/src/boolean/section_cert.rs'; ops='crates/topo/src/boolean/ops.rs'
if m=='M1': sub(sc,'at == Some(FaceContainment::Out) && clear(side)','at.is_some() && clear(side)')
if m=='M3': sub(sc,'T::from_f64(band.escalate())','T::from_f64(band.zero())')
if m=='M4': sub(ops,'Ok(finite && !face_boundary_meets(body, face, &ball, pad)?)','let _ = face_boundary_meets(body, face, &ball, pad)?;\n    Ok(finite)')
if m=='M6': sub(ops,'                                    return Err(esc(SphereQuestion::AgainstPlane)(diag));','                                    let _ = diag; continue;')
if m=='M7': sub(ops,'''                                        .is_some()
                                        {
                                            return Err(BooleanError::SpheresMeet {''','''                                        .is_some() && false
                                        {
                                            return Err(BooleanError::SpheresMeet {''')
if m=='M8':  # touch spread forgets the +m slack and the sqrt bound uses spread=m only for sphere_plane
    sub(sc,'(cs - n * d, (m * (rho + rho + m)).sqrt() + m)','(cs - n * d, m)')
if m=='M9':  # touch centre for sphere×sphere at c1 + k*r1 (the touch on sphere 1) instead of circle centre - should be fine
    sub(sc,'(c1 + k * x, (r1 * r2 * m * T::from_f64(2.0) / dd).sqrt() + m)','(c1 + k * x, (r1 * r2 * m * T::from_f64(2.0) / dd).sqrt() * T::from_f64(0.1))')
