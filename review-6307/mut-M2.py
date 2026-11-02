p='/home/user/cad-mut/crates/topo/src/boolean/sphere_region.rs'
s=open(p).read()
old="                e2: normal.cross(side),"
assert s.count(old)==1
s=s.replace(old,"                e2: side.cross(normal),")
open(p,'w').write(s)
