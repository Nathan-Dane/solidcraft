import math, os
OUT=os.path.join(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))),'svg','model')
INK='#34465b'; P1='#e2e8ef'; P2='#b8c3cf'; P3='#8e9faf'; GAP='#ffffff'; GUIDE='#6f879c'
A1='#b9ddf8'; A2='#328bd3'; A3='#236ead'
OW=2.55; AW=4.25; NW=5.6
R3=math.sqrt(3)
def f(v):
    s=('%.2f'%v).rstrip('0').rstrip('.')
    return '0' if s in('-0','') else s
def pts(ps): return ' '.join('%s%s %s'%('ML'[i>0],f(x),f(y)) for i,(x,y) in enumerate(ps)) if False else ''.join(('M' if i==0 else 'L')+f(x)+' '+f(y) for i,(x,y) in enumerate(ps))
def poly(ps, close=True): return pts(ps)+('Z' if close else '')
def ell(cx,cy,rx,ry=None):
    ry=rx if ry is None else ry
    return 'M%s %sA%s %s 0 1 0 %s %sA%s %s 0 1 0 %s %sZ'%(f(cx-rx),f(cy),f(rx),f(ry),f(cx+rx),f(cy),f(rx),f(ry),f(cx-rx),f(cy))
def arcpts(cx,cy,rx,ry,t0,t1):
    """arc along ellipse from angle t0 to t1 (degrees, screen coords: 0=right, 90=down). returns path 'M..A..' """
    a0,a1=math.radians(t0),math.radians(t1)
    p0=(cx+rx*math.cos(a0),cy+ry*math.sin(a0)); p1=(cx+rx*math.cos(a1),cy+ry*math.sin(a1))
    large=1 if abs(t1-t0)>180 else 0; sweep=1 if t1>t0 else 0
    return 'M%s %sA%s %s 0 %d %d %s %s'%(f(p0[0]),f(p0[1]),f(rx),f(ry),large,sweep,f(p1[0]),f(p1[1]))
def arcto(rx,ry,x,y,large=0,sweep=1): return 'A%s %s 0 %d %d %s %s'%(f(rx),f(ry),large,sweep,f(x),f(y))
def ellarc(cx,cy,rx,ry,t0,t1): return arcpts(cx,cy,rx,ry,t0,t1)
def rrect(x0,y0,x1,y1,r):
    return ('M%s %sL%s %sA%s %s 0 0 1 %s %sL%s %sA%s %s 0 0 1 %s %sL%s %sA%s %s 0 0 1 %s %sL%s %sA%s %s 0 0 1 %s %sZ'%
     (f(x0+r),f(y0),f(x1-r),f(y0),f(r),f(r),f(x1),f(y0+r),f(x1),f(y1-r),f(r),f(r),f(x1-r),f(y1),f(x0+r),f(y1),f(r),f(r),f(x0),f(y1-r),f(x0),f(y0+r),f(r),f(r),f(x0+r),f(y0)))

class Icon:
    def __init__(s,name,title=None):
        s.name=name; s.title=title or name.replace('_',' ').title(); s.els=[]; s.fam='model'
    def fill(s,d,c,rule=None):
        s.els.append('<path d="%s" fill="%s"%s/>'%(d,c,' fill-rule="evenodd"' if rule else '')); return s
    def stroke(s,d,c=INK,w=OW,fill=None,rule=None):
        s.els.append('<path d="%s" fill="%s" stroke="%s" stroke-width="%s" stroke-linejoin="round" stroke-linecap="round"%s/>'%(d,fill or 'none',c,f(w),' fill-rule="evenodd"' if rule else '')); return s
    def dot(s,x,y,r,c):
        s.els.append('<circle cx="%s" cy="%s" r="%s" fill="%s"/>'%(f(x),f(y),f(r),c)); return s
    def ring(s,x,y,r,c=INK,w=OW,fill=None):
        s.els.append('<circle cx="%s" cy="%s" r="%s" fill="%s" stroke="%s" stroke-width="%s" stroke-linejoin="round" stroke-linecap="round"/>'%(f(x),f(y),f(r),fill or 'none',c,f(w))); return s
    def write(s,fam='model'):
        d=os.path.join(os.path.dirname(OUT),fam); os.makedirs(d,exist_ok=True)
        body='\n'.join('  '+e for e in s.els)
        open(os.path.join(d,s.name+'.svg'),'w').write('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">\n  <title>%s</title>\n%s\n</svg>\n'%(s.title,body))

# ---- isometric helpers ----
class Iso:
    """x -> right-down, y -> left-down, z up. k = screen units per unit."""
    def __init__(s,k=1.0,ox=32,oy=32): s.k=k; s.ox=ox; s.oy=oy
    def p(s,x,y,z=0): return (s.ox+(x-y)*R3/2*s.k, s.oy+(x+y)*0.5*s.k-z*s.k)
    def fit(s,pts3d,maxw=56,maxh=56):
        s.k=1.0; s.ox=0; s.oy=0
        P=[s.p(*q) for q in pts3d]; w=max(p[0] for p in P)-min(p[0] for p in P); h=max(p[1] for p in P)-min(p[1] for p in P)
        s.k=min(maxw/w,maxh/h); P=[s.p(*q) for q in pts3d]
        s.ox=32-(max(p[0] for p in P)+min(p[0] for p in P))/2; s.oy=32-(max(p[1] for p in P)+min(p[1] for p in P))/2
        return s
    def poly(s,*ps): return poly([s.p(*q) for q in ps])
    def ellipse_r(s,r): return (R3/2*s.k*r*math.sqrt(2), 0.5*s.k*r*math.sqrt(2))
    def box(s,ic,x0,y0,z0,x1,y1,z1,cols=(P1,P2,P3),w=OW,detail=True,ink=INK):
        """draw filled box with outline; faces: top, left(+y), right(+x)"""
        top=s.poly((x0,y0,z1),(x1,y0,z1),(x1,y1,z1),(x0,y1,z1))
        left=s.poly((x0,y1,z1),(x1,y1,z1),(x1,y1,z0),(x0,y1,z0))
        right=s.poly((x1,y0,z1),(x1,y1,z1),(x1,y1,z0),(x1,y0,z0))
        ic.fill(left,cols[1]).fill(right,cols[2]).fill(top,cols[0])
        sil=s.poly((x0,y0,z1),(x1,y0,z1),(x1,y0,z0),(x1,y1,z0),(x0,y1,z0),(x0,y1,z1))
        ic.stroke(sil,ink,w)
        if detail:
            ic.stroke(poly([s.p(x0,y1,z1),s.p(x1,y1,z1),s.p(x1,y0,z1)],False),ink,w)
            ic.stroke(poly([s.p(x1,y1,z1),s.p(x1,y1,z0)],False),ink,w)

def cylinder(ic,X,Y,rx,h,cols=(P1,P2,P3),w=OW,top=True,ry=None,sh=0.35):
    """vertical cylinder; (X,Y) = centre of the top ellipse; side shading strip on the right"""
    ry=rx/R3 if ry is None else ry
    body='M%s %sL%s %s%sL%s %sZ'%(f(X-rx),f(Y),f(X-rx),f(Y+h),arcto(rx,ry,X+rx,Y+h,0,0),f(X+rx),f(Y))
    ic.fill(body,cols[1])
    xs=X+sh*rx; yb=ry*math.sqrt(1-sh*sh)
    strip='M%s %sL%s %s%sL%s %sZ'%(f(xs),f(Y),f(xs),f(Y+h+yb),arcto(rx,ry,X+rx,Y+h,0,0),f(X+rx),f(Y))
    ic.fill(strip,cols[2])
    ic.stroke(body,INK,w)
    if top: ic.stroke(ell(X,Y,rx,ry),INK,w,fill=cols[0])
def disc_dot_row(): pass
