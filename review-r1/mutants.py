import sys
m=sys.argv[1]
p='/tmp/claude-0/mut/crates/topo/src/boolean/join.rs'
s=open(p).read()
def rep(a,b):
    global s
    assert s.count(a)==1, a
    s=s.replace(a,b)
if m=='1':
    rep('''    let along = match (germ.a_locus, germ.b_locus) {
        (super::Locus::OnEdge(edge), _) => Some((&red.a, edge)),
        (_, super::Locus::OnEdge(edge)) => Some((&red.b, edge)),
        _ => None,
    };''','''    let along = match (germ.a_locus, germ.b_locus) {
        (super::Locus::OnEdge(edge), super::Locus::OnEdge(_)) => Some((&red.a, edge)),
        _ => None,
    };''')
elif m=='2':
    rep('''            Some(match (&ga, &gb) {
                (Sf::Plane { .. }, Sf::Plane { .. }) => GermLane::Planar,''','''            Some(match edge_lane() { Ok(l) => l, Err(_) => match (&ga, &gb) {
                (Sf::Plane { .. }, Sf::Plane { .. }) => GermLane::Planar,''')
    rep('''                _ => edge_lane()?,
            })''','''                _ => edge_lane()?,
            }})''')
elif m=='3':
    rep('''(super::Locus::OnEdge(e), super::Locus::InFace(_)) => (Operand::A, &red.a, e, &gb),''','''(super::Locus::OnEdge(e), super::Locus::InFace(_)) => (Operand::B, &red.a, e, &gb),''')
    rep('''(super::Locus::InFace(_), super::Locus::OnEdge(e)) => (Operand::B, &red.b, e, &ga),''','''(super::Locus::InFace(_), super::Locus::OnEdge(e)) => (Operand::A, &red.b, e, &ga),''')
elif m=='4':
    rep('''            if matches!(holder, Sf::Plane { .. }) {
                return Err(no_arm());
            }''','')
open(p,'w').write(s)
