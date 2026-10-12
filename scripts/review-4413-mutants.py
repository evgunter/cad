import sys
V='crates/topo/src/boolean/vtxfac.rs'; M='crates/topo/src/boolean/mod.rs'
ON_PLANE="SideCode::On if datum.shape == super::sectors::FaceShape::Plane => on += 1,"
RET="    Ok(side.map(|side| PartnerSide { side, along }))"
FLIP="    Ok(side.map(|side| PartnerSide { side: match side { SideCode::In => SideCode::Out, SideCode::Out => SideCode::In, s => s }, along }))"
FLIPB="    Ok(side.map(|side| PartnerSide { side: if along { match side { SideCode::In => SideCode::Out, SideCode::Out => SideCode::In, s => s } } else { side }, along }))"
PS="            let read = pierce_side(dir, reach, datum.normal, datum.shape, lever, band)?;"
muts={
 'M1':[(V,ON_PLANE,"SideCode::On => return Ok(None),")],
 'M2':[(V,RET,FLIP)],
 'M2b':[(V,RET,FLIPB)],
 'M3':[(V,"            let lever = datum.lever.within(reach.length());","            let lever = { let _ = datum.lever.within(reach.length()); super::sectors::NO_CURVATURE() };")],
 'M4':[(V,PS,"            let read = pierce_side(dir, reach, datum.normal, datum.shape, lever, band).unwrap_or(super::sectors::PierceSide { code: SideCode::On, tied: true });")],
 'M5':[(V,"if on == 2 || (along && flush) {","if on == 2 || (along && flush && false) {")],
 'M6':[(M,"            && records.iter().any(sectors::PairRecord::coplanar)","            && records.iter().any(sectors::PairRecord::coplanar) && false")],
 'M7':[(V,"if on == 2 || (along && flush) {","if (along && flush) {")],
 'M8':[(V,ON_PLANE,"SideCode::On => on += 1,")],
 'MC':[(V,PS,"            if matches!(reach, super::sectors::Reach::Bisector(_)) { continue; }\n"+PS)],
}
m=sys.argv[1]
for f,a,b in muts[m]:
    s=open(f).read(); assert s.count(a)==1,(m,f,a); open(f,'w').write(s.replace(a,b))
print('applied',m)
