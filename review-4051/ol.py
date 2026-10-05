import re,collections,sys
def load(f):
    d={}
    for l in open(f):
        if ': ' in l:
            t,r=l.rstrip('\n').split(': ',1); d[t]=r
    return d
h=load(sys.argv[1]); o=load(sys.argv[2])
c=collections.Counter(); miss=0
for t,r in h.items():
    if 'PinchUncrossed' in r:
        if t not in o: miss+=1; continue
        x=o[t]
        if x.startswith('OK'): k='OK '+x.split()[1]
        else:
            kinds=sorted(set(re.findall(r'(RingMeetsOuter|LoopRoleInverted|SplitVertexOrbit|PinchUncrossed|SelfLoopEdge|RingOnCurvedFace)',x)))
            k='ERR '+x.split()[1]+' '+'+'.join(kinds)
        c[k]+=1
print('pinch lines',sum(c.values()),'missing',miss)
for k,n in c.most_common(): print(' ',n,k)
