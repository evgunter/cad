p='/home/user/cad-mut/crates/topo/src/boolean/sphere_region.rs'
s=open(p).read()
old="            rays.chain(paths),"
assert s.count(old)==1
s=s.replace(old,"            rays.chain(paths.take(0)),")
open(p,'w').write(s)
