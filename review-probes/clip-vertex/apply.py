import sys
sys.path.insert(0, sys.argv[3])
from mutants import M
p = sys.argv[1]; name = sys.argv[2]
s = open(p).read()
old, new = M[name]
assert s.count(old) == 1, (name, s.count(old))
open(p, 'w').write(s.replace(old, new))
