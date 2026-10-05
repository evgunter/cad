import sys
p='/home/user/wt-mut/crates/topo/src/boolean/insert.rs'
muts={
 'root':("fan_holder.map_or(vertex(i), |(v, _)| v),","vertex(i),"),
 'order':("keyed.push((nest(at), depth.is_none()","keyed.push((0, depth.is_none()"),
 'holder':(".min_by_key(|&o| {\n                        let (from, to) = interval(o);",".max_by_key(|&o| {\n                        let (from, to) = interval(o);"),
 'lastcorner':("Some(_) => corner_bound(body, vertex, next, holder)?,","Some(_) => next,"),
}
m=sys.argv[1]
s=open(p).read()
if m=='none':
    pass
else:
    a,b=muts[m]; assert s.count(a)==1,(m,s.count(a)); s=s.replace(a,b)
open(p,'w').write(s)
