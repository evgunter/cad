import sys
p='/home/user/rv-mut/crates/topo/src/boolean/join.rs'
s=open(p).read()
old="let reach = geom_brep::Reach::Ball(germ_reach(&red.a)?);"
assert old in s
which=sys.argv[1]
if which=='R3': new="let reach = geom_brep::Reach::Ball(geom_brep::ExtentBall::point(match &gb { Sf::Cylinder { origin, .. } => *origin, _ => unreachable!() }));"
elif which=='R3own': new="let reach = geom_brep::Reach::Ball(geom_brep::ExtentBall::point(match &ga { Sf::Cylinder { origin, .. } => *origin, _ => unreachable!() }));"
elif which=='Rb': new="let reach = geom_brep::Reach::Ball(germ_reach(&red.b)?);"
s=s.replace(old,new)
open(p,'w').write(s)
