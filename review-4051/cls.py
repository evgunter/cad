import sys,re,collections
def cls(r):
    r=r.strip()
    if r.startswith('OK'):
        w=r.split()[1]
        bad = '' if all(x in r for x in ['t2=true','t3p=true','cert=true']) else ' notall'
        return 'OK '+w+bad+(' FACE2V' if 'FACE2V' in r else '')
    if r.startswith('ERR'):
        m=re.match(r'ERR (\w+)(?: \{ (\w+)(?:: |\s*\{ )?(\w+)?)?',r)
        k=m.group(1)
        if k in ('ResultInvalid','Euler','VolumeUncomputable','JoinDesync'):
            inner=re.findall(r'[A-Z]\w+',r[4+len(k):])[:2]
            return 'ERR '+k+' '+'/'.join(inner)
        return 'ERR '+k
    return r[:30]
def load(f):
    d={}
    for line in open(f):
        if ': ' not in line: continue
        t,r=line.rstrip('\n').split(': ',1)
        d[t]=(cls(r),r)
    return d
if __name__=='__main__':
    if len(sys.argv)==2:
        c=collections.Counter(v[0] for v in load(sys.argv[1]).values())
        for k,n in c.most_common(): print(n,k)
    else:
        a,b=load(sys.argv[1]),load(sys.argv[2])
        c=collections.Counter()
        for t in a:
            if t in b and a[t][0]!=b[t][0]:
                c[(a[t][0],b[t][0])]+=1
        print('only-a',len(set(a)-set(b)),'only-b',len(set(b)-set(a)))
        for k,n in c.most_common(): print(n,k[0],'->',k[1])
