p='/home/user/cad-mut/crates/topo/src/boolean/sphere_region.rs'
s=open(p).read()
old="                center: center + a * (radius * cos),\n                normal,\n                radius: radius * sin,"
assert s.count(old)==1
s=s.replace(old,"                center,\n                normal,\n                radius,")
open(p,'w').write(s)
