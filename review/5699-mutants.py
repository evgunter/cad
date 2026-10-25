import sys,subprocess
p='/home/user/rv2/crates/topo/src/props.rs'
M={
'A':('let refs = References::points(body, walk.runs.iter().map(|r| r.face), walk.quad);',
     'let refs = References::points(body, next.iter().map(|&(_, face, _)| face), walk.quad);'),
'B':('let refs = References::points(body, runs.iter().map(|r| r.face), quad);\n        let resume',
     'let refs = References::points(body, runs.iter().filter(|r| r.resumes_after().is_some()).map(|r| r.face), quad);\n        let resume'),
'C':('let Some(carry) = carry_of(&edges, lift(point), lift(c)) else {',
     'let Some(carry) = carry_of(&edges, lift(face_start(body, faces[faces.len() - 1])), lift(c)) else {'),
}
k=sys.argv[1]; s=open(p).read()
o,n=M[k]
if sys.argv[2]=='on':
    assert s.count(o)==1,k; s=s.replace(o,n)
else:
    assert s.count(n)==1,k; s=s.replace(n,o)
open(p,'w').write(s)
