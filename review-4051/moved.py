import sys,collections
sys.path.insert(0,'.')
from cls import load
a,b=load(sys.argv[1]),load(sys.argv[2])
c=collections.Counter()
ex={}
for t in a:
    if a[t][0]!=b[t][0]:
        key=' '.join(t.split()[:int(sys.argv[3])])+' '+' '.join(t.split()[-2:])
        c[key]+=1; ex.setdefault(key,t)
for k,n in sorted(c.items()): print(n,k,'| e.g.',ex[k])
