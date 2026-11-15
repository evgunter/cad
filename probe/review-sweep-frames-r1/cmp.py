import re,sys
def load(f):
    cases={};cur=None
    for l in open(f):
        if l.startswith('CASE'): cur=l.strip(); cases[cur]=[]
        elif cur and l.startswith('  '): cases[cur].append(l.rstrip())
    return cases
fl=re.compile(r'-?\d+\.\d+(?:e-?\d+)?|-?\d+e-?\d+')
A=load('probe-base.txt');B=load('probe-head.txt')
TOL=1e-9
for c in A:
    a,b=A[c],B.get(c)
    if b is None: print('MISSING',c);continue
    if len(a)!=len(b): print('LEN',c,len(a),len(b)); print('  base:',a[0][:200]); print('  head:',b[0][:200]); continue
    worst=0;notes=[]
    for x,y in zip(a,b):
        if 'PLANEORIGIN' in x:
            fx=[float(v) for v in fl.findall(x)][1:];fy=[float(v) for v in fl.findall(y)][1:]
            d=max(abs(p-q) for p,q in zip(fx,fy))
            if d>1e-6: notes.append('origin moved %.3g'%d)
            continue
        kx=x.split('|')[0];ky=y.split('|')[0]
        tx=fl.sub('#',kx);ty=fl.sub('#',ky)
        if tx!=ty: notes.append('TEXT '+kx[:150]+' || '+ky[:150]);continue
        fx=[float(v) for v in fl.findall(kx)];fy=[float(v) for v in fl.findall(ky)]
        d=max([abs(p-q)/max(1,abs(p)) for p,q in zip(fx,fy)]+[0])
        worst=max(worst,d)
        if d>TOL: notes.append('DIFF %.3g: %s || %s'%(d,kx[:230],ky[:230]))
        # residual columns
        for k in ('vres','endres'):
            mx=re.search(k+r' ([0-9.e+-]+)',x);my=re.search(k+r' ([0-9.e+-]+)',y)
            if mx and my and float(my.group(1))>10*float(mx.group(1))+1e-13: notes.append('%s worse base %s head %s'%(k,mx.group(1),my.group(1)))
    om=sum(1 for n in notes if n.startswith('origin'))
    rest=[n for n in notes if not n.startswith('origin')]
    print(c,'| worst rel diff %.2g | origins moved %d | other %d'%(worst,om,len(rest)))
    for n in rest[:6]: print('    ',n)
