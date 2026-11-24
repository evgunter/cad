import sys
INJ='crates/topo/src/injective.rs'
VAL='crates/topo/src/validate.rs'
M={
 'lesser_to_greater':[(INJ,"Some(of(x)?.min(of(y)?))","Some(of(x)?.max(of(y)?))")],
 'sum_to_one_sine':[(INJ,"parts([ra, rb], psi.lipschitz(), extent, band)","parts([ra], psi.lipschitz(), extent, band)")],
 'grad_ratio_dropped':[(INJ,"norm.is_certified().then(|| at(norm.lo()) / at(norm.hi()))","norm.is_certified().then(|| at(1.0))")],
 'seam_as_crease':[(INJ,".all(|(_, class)| matches!(class, None | Some(crate::EdgeClass::Crease(_))));",".all(|(_, class)| matches!(class, None | Some(crate::EdgeClass::Crease(_) | crate::EdgeClass::Seam)));")],
 'end_as_crease':[(INJ,".find(|(_, class)| matches!(class, Some(crate::EdgeClass::End(_))))",".find(|(_, class)| matches!(class, Some(crate::EdgeClass::End(_))) && false)"),
                  (INJ,".all(|(_, class)| matches!(class, None | Some(crate::EdgeClass::Crease(_))));",".all(|(_, class)| matches!(class, None | Some(crate::EdgeClass::Crease(_) | crate::EdgeClass::End(_))));")],
 'turn_check_off':[(INJ,"&& read_turn != *turn","&& read_turn != *turn && false")],
 'turn_check_every_cell':[(INJ,"&& p.zero.along(*edge)\n","\n")],
 'facts_not_carried':[(VAL,"facts: std::sync::Arc::clone(&self.facts),","facts: std::sync::Arc::new(crate::Tier3Facts::default()),")],
}
name=sys.argv[1]
for f,a,b in M[name]:
    s=open(f).read()
    n=s.count(a)
    assert n==1,(name,f,a,n)
    open(f,'w').write(s.replace(a,b))
print('applied',name)
