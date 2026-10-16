from lib import *
from i2 import pt, arrowhead, arrow, poly_reg, dashes
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
G=(P1,P2,P3); B=(A1,A2,A3)
def face(ic,I,pts,col,stroke=True,w=OW):
    d=I.poly(*pts); ic.fill(d,col)
    if stroke: ic.stroke(d,INK,w)
def cyl3(ic,I,x,y,r,z0,z1,cols=B,w=OW):
    X,Y=I.p(x,y,z1); rx,ry=I.ellipse_r(r); h=(z1-z0)*I.k
    cylinder(ic,X,Y,rx,h,cols,w,ry=ry)
@icon
def boss():
    ic=Icon('boss','Boss'); I=Iso().fit([(0,0,0),(44,44,0),(44,44,26),(0,0,26),(0,44,0),(44,0,0)],58,56)
    I.box(ic,0,0,0,44,44,8,G); cyl3(ic,I,22,22,10,8,26,B); return ic
@icon
def hole():
    ic=Icon('hole','Hole'); I=Iso().fit([(0,0,0),(46,46,0),(0,46,16),(46,0,16)],58,56)
    I.box(ic,0,0,0,46,46,16,G)
    X,Y=I.p(23,23,16); rx,ry=I.ellipse_r(11)
    ic.fill(ell(X,Y,rx,ry),A3); ic.fill(ell(X,Y+3.2,rx*0.74,ry*0.62),A2)
    ic.stroke(ell(X,Y,rx,ry),INK,OW); return ic
@icon
def shell():
    ic=Icon('shell','Shell'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,36),(40,40,36),(0,40,0),(40,0,0)],58,58)
    I.box(ic,0,0,0,40,40,36,G)
    t=9; z=36; dpt=60
    hole=[I.p(t,t,z),I.p(40-t,t,z),I.p(40-t,40-t,z),I.p(t,40-t,z)]
    wall_a=[I.p(t,t,z),I.p(40-t,t,z),I.p(40-t,t,z-dpt),I.p(t,t,z-dpt)]          # back wall facing left
    wall_b=[I.p(t,t,z),I.p(t,40-t,z),I.p(t,40-t,z-dpt),I.p(t,t,z-dpt)]          # back wall facing right
    for poly_pts,col in ((wall_a,A2),(wall_b,A3)):
        part=clip_poly(poly_pts,hole)
        if len(part)>2: ic.fill(poly(part),col)
    ic.stroke(poly(hole),INK,OW)
    # the corner between the two back walls
    a=I.p(t,t,z); b=I.p(t,t,z-dpt); lo=max(a[1],min(b[1],I.p(40-t,40-t,z)[1]))
    ic.stroke(poly([a,(a[0],lo)],False),INK,OW)
    return ic
def shell():
    ic=Icon('shell','Shell'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,36),(40,40,36),(0,40,0),(40,0,0)],58,58)
    I.box(ic,0,0,0,40,40,36,G)
    t=9; z=36; dpt=16
    hole=[I.p(t,t,z),I.p(40-t,t,z),I.p(40-t,40-t,z),I.p(t,40-t,z)]
    wall_a=[I.p(t,t,z),I.p(40-t,t,z),I.p(40-t,t,z-dpt),I.p(t,t,z-dpt)]          # back wall facing left
    wall_b=[I.p(t,t,z),I.p(t,40-t,z),I.p(t,40-t,z-dpt),I.p(t,t,z-dpt)]          # back wall facing right
    floor=[I.p(t,t,z-dpt),I.p(40-t,t,z-dpt),I.p(40-t,40-t,z-dpt),I.p(t,40-t,z-dpt)]
    for poly_pts,col in ((wall_a,A2),(wall_b,A3),(floor,A1)):
        part=clip_poly(poly_pts,hole)
        if len(part)>2: ic.fill(poly(part),col)
    ic.stroke(poly(hole),INK,OW)
    for poly_pts in (wall_a,wall_b):
        part=clip_poly(poly_pts,hole)
    # edges between the walls and the floor
    fl=clip_poly(floor,hole)
    if len(fl)>2: ic.stroke(poly(fl,False),INK,OW)
    return ic
def shell():
    ic=Icon('shell','Shell'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,34),(40,40,34),(0,40,0),(40,0,0)],58,56)
    I.box(ic,0,0,0,40,40,34,G,OW,detail=False)
    t=7; z=34
    top=I.poly((t,t,z),(40-t,t,z),(40-t,40-t,z),(t,40-t,z)); ic.fill(top,P3)
    face(ic,I,[(t,t,z),(40-t,t,z),(40-t,t,12),(t,t,12)],P2,False)
    face(ic,I,[(t,t,z),(t,40-t,z),(t,40-t,12),(t,t,12)],P3,False)
    ic.fill(I.poly((t,t,12),(40-t,t,12),(40-t,40-t,12),(t,40-t,12)),A1)
    ic.stroke(top,INK,OW)
    ic.stroke(poly([I.p(t,t,z),I.p(t,t,12),I.p(40-t,t,12)],False),INK,OW)
    ic.stroke(poly([I.p(t,t,12),I.p(t,40-t,12)],False),INK,OW) if False else None
    ic.stroke(poly([I.p(40,40,34),I.p(40,40,0)],False),INK,OW)
    ic.stroke(poly([I.p(0,40,34),I.p(40,40,34),I.p(40,0,34)],False),INK,OW)
    return ic
@icon
def rib():
    ic=Icon('rib','Rib'); I=Iso().fit([(0,0,0),(44,44,0),(0,0,28),(44,44,28),(0,44,0),(44,0,0)],58,56)
    I.box(ic,0,0,0,44,44,6,G)
    I.box(ic,19,3,6,25,41,26,B)
    return ic
@icon
def presspull():
    ic=Icon('presspull','Press pull'); I=Iso().fit([(0,0,0),(44,44,0),(44,44,12),(0,0,12),(0,44,0),(44,0,0),(22,22,40)],58,58)
    I.box(ic,0,0,0,44,44,10,G)
    I.box(ic,10,10,10,34,34,22,B)
    X,Y=I.p(22,22,22)
    ic.stroke(poly([(X,Y-18),(X,Y-3)],False),A3,AW); ic.fill(arrowhead(X,Y-26,0,-1,9,10),A3)
    return ic.recenter()
@icon
def loft():
    ic=Icon('loft','Loft')
    body='M6 46L21.5 16.5L32 19L42.5 16.5L58 46L32 58Z'
    ic.fill(body,A2)
    ic.fill('M6 46L21.5 16.5L24 18Q25 40 32 58Z',A1)
    ic.fill('M58 46L42.5 16.5L40 18Q39 40 32 58Z',A3)
    ic.stroke('M6 46L22 14M42 14L58 46L32 58L6 46',INK,OW)
    ic.fill(ell(32,14,10,5),A1); ic.stroke(ell(32,14,10,5),INK,OW)
    return ic
def split_halves(ic,flat):
    s_=24; g=18; m=3
    pts=[(-m,-g/2-m,-m),(s_+m,s_+g/2+m,s_+m)]
    I=Iso().fit([(-m,-g/2-m,-m),(s_+m,-g/2-m,-m),(s_+m,s_+g/2+m,-m),(-m,s_+g/2+m,s_+m),(s_+m,s_+g/2+m,s_+m)],58,58)
    half=s_/2; zc=s_/2
    def slab(y0,y1):
        if flat: 
            d=I.poly((0,y0,zc),(s_,y0,zc),(s_,y1,zc),(0,y1,zc)); ic.fill(d,P1); ic.stroke(d,INK,OW)
        else: I.box(ic,0,y0,0,s_,y1,s_,G)
    slab(-g/2,half-g/2)                                            # back right half
    pl=I.poly((-m,half,-m),(s_+m,half,-m),(s_+m,half,s_+m),(-m,half,s_+m))
    ic.fill(pl,A1); ic.stroke(pl,A3,AW*0.75)                       # the cutting plane, between the halves
    slab(half+g/2,s_+g/2)                                          # front left half

@icon
def split_body():
    ic=Icon('split_body','Split body'); split_halves(ic,False); return ic.recenter()
@icon
def split_face():
    ic=Icon('split_face','Split face'); split_halves(ic,True); return ic.recenter()
def split():
    ic=Icon('split','Split'); I=Iso().fit([(0,0,0),(34,34,0),(34,34,36),(0,0,36),(0,34,0),(34,0,0),(-6,-6,18),(40,40,18)],58,58)
    I.box(ic,0,0,0,34,34,14,G)
    I.box(ic,0,0,22,34,34,36,G)
    pl=I.poly((-6,-6,18),(40,-6,18),(40,40,18),(-6,40,18)); ic.fill(pl,A1); ic.stroke(pl,A3,AW*0.7)
    return ic
@icon
def intersect():
    ic=Icon('intersect','Intersect')
    ic.fill(ell(23,32,19),P1); ic.fill(ell(41,32,19),P1)
    lens='M32 %sA19 19 0 0 1 32 %sA19 19 0 0 1 32 %sZ'%(f(32-math.sqrt(19*19-9*9)),f(32+math.sqrt(19*19-9*9)),f(32-math.sqrt(19*19-9*9)))
    ic.fill(lens,A2); ic.stroke(ell(23,32,19),INK,OW); ic.stroke(ell(41,32,19),INK,OW); ic.stroke(lens,A3,OW) if False else None
    return ic
@icon
def draft():
    ic=Icon('draft','Draft'); t=6;H=36
    I=Iso().fit([(0,0,0),(40,40,0),(0,0,H),(40,40,H),(0,40,0),(40,0,0)],58,56)
    ic.fill(I.poly((0,40,0),(40,40,0),(40-t,40-t,H),(t,40-t,H)),A2)
    ic.fill(I.poly((40,0,0),(40,40,0),(40-t,40-t,H),(40-t,t,H)),A3)
    ic.fill(I.poly((t,t,H),(40-t,t,H),(40-t,40-t,H),(t,40-t,H)),P1)
    ic.stroke(I.poly((0,40,0),(40,40,0),(40,0,0),(40-t,t,H),(t,t,H),(t,40-t,H)),INK,OW)
    ic.stroke(poly([I.p(t,40-t,H),I.p(40-t,40-t,H),I.p(40-t,t,H)],False),INK,OW)
    ic.stroke(poly([I.p(40-t,40-t,H),I.p(40,40,0)],False),INK,OW)
    # the original top face, before the sides were drafted
    orig=I.poly((0,0,H),(40,0,H),(40,40,H),(0,40,H))
    ic.stroke(orig,GAP,2.6,dash='0.1 4.6'); ic.stroke(orig,P3,1.5,dash='0.1 4.6')
    return ic
def draft():
    ic=Icon('draft','Draft'); I=Iso().fit([(0,0,0),(40,40,0),(34,34,36),(6,6,36),(0,40,0),(40,0,0)],58,56)
    t=6;H=36
    ic.fill(I.poly((0,40,0),(40,40,0),(40-t,40-t,H),(t,40-t,H)),P2)
    ic.fill(I.poly((40,0,0),(40,40,0),(40-t,40-t,H),(40-t,t,H)),A2)
    ic.fill(I.poly((t,t,H),(40-t,t,H),(40-t,40-t,H),(t,40-t,H)),P1)
    ic.stroke(I.poly((0,40,0),(40,40,0),(40,0,0),(40-t,t,H),(t,t,H),(t,40-t,H)),INK,OW)
    ic.stroke(poly([I.p(t,40-t,H),I.p(40-t,40-t,H),I.p(40-t,t,H)],False),INK,OW)
    ic.stroke(poly([I.p(40-t,40-t,H),I.p(40,40,0)],False),INK,OW)
    return ic
@icon
def thread():
    ic=Icon('thread','Thread')
    X,Y,rx,h=32,12,14,42
    ry=rx/R3
    cylinder(ic,X,Y,rx,h,G,OW,top=True)
    e,pitch=ry,11.5
    def front(k):
        pts=[]
        for i in range(25):
            t=math.pi*i/24                       # the front half of one turn, as in the coil
            pts.append((X+(rx+2.4)*math.cos(t), 22+pitch*(k+t/(2*math.pi))+e*math.sin(t)))
        return poly(pts,False)
    for k in range(3): ic.stroke(front(k),INK,3.2+2*1.6)
    for k in range(3): ic.stroke(front(k),A2,3.2)
    return ic
def thread():
    ic=Icon('thread','Thread')
    X,Y,rx,h=32,12,14,40
    ry=rx/R3
    cylinder(ic,X,Y,rx,h,G,OW,top=True)
    for i in range(4):
        y=Y+12+i*10
        ic.stroke('M%s %sQ%s %s %s %s'%(f(X-rx),f(y+9),f(X),f(y+ry*2+1),f(X+rx),f(y-9)),A3,AW*0.85)
    return ic
@icon
def coil():
    ic=Icon('coil','Coil')
    R,e,turns,pitch=17.5,5.2,4,10.5
    def pt_at(t):
        return (32+R*math.cos(t), 0+pitch*t/(2*math.pi)+e*math.sin(t))
    # wire runs from t=0.5pi-ish; sample finely and split into back (sin<0) and front (sin>=0) runs
    t0=math.pi; t1=t0+turns*2*math.pi
    n=turns*48
    P=[(t0+(t1-t0)*i/n) for i in range(n+1)]
    ys=[pt_at(t)[1] for t in P]; ymin,ymax=min(ys),max(ys); dy=32-(ymin+ymax)/2
    def run(sel):
        runs=[];cur=[]
        for t in P:
            x,y=pt_at(t); ok=(math.sin(t)>=-1e-9)==sel
            if ok: cur.append((x,y+dy))
            else:
                if cur: runs.append(cur); cur=[]
        if cur: runs.append(cur)
        return runs
    for r in run(False):
        if len(r)>1: ic.stroke(poly(r,False),INK,AW*0.8+2*OW*0.8)
    for r in run(False):
        if len(r)>1: ic.stroke(poly(r,False),A3,AW*0.8)
    for r in run(True):
        if len(r)>1: ic.stroke(poly(r,False),INK,AW*0.8+2*OW*0.8)
    for r in run(True):
        if len(r)>1: ic.stroke(poly(r,False),A2,AW*0.8)
    return ic
def bez(p0,p1,p2,p3,t):
    u=1-t; return tuple(u**3*a+3*u*u*t*b+3*u*t*t*c+t**3*d for a,b,c,d in zip(p0,p1,p2,p3))
def surf_patch(ic,I,n=3,amp=7,size=40,fillc=A1,line=A3,lw=OW,out=A3,ow=AW*0.75):
    def Z(x,y): return amp*math.sin(x*math.pi/size*1.3)*math.cos(y*math.pi/size*0.9)+amp*0.6
    def line_pts(fixed,val,m=14):
        out=[]
        for i in range(m+1):
            t=size*i/m
            x,y=(t,val) if fixed=='y' else (val,t)
            out.append(I.p(x,y,Z(x,y)))
        return out
    top=line_pts('y',0); right=line_pts('x',size); bot=line_pts('y',size)[::-1]; left=line_pts('x',0)[::-1]
    # boundary: y=0 edge (back-right), x=size edge (front-right), y=size edge (front-left), x=0 edge (back-left)
    outline=top+right+bot+left
    ic.fill(poly(outline),fillc)
    for k in range(1,n+1):
        v=size*k/(n+1)
        ic.stroke(poly(line_pts('y',v),False),line,lw); ic.stroke(poly(line_pts('x',v),False),line,lw)
    ic.stroke(poly(outline),out,ow)
    return outline

@icon
def flange():
    ic=Icon('flange','Flange'); I=Iso().fit([(0,0,0),(46,0,0),(46,36,0),(0,36,0),(0,0,26),(46,0,26)],58,56)
    I.box(ic,0,0,0,46,36,4,G)
    I.box(ic,0,0,4,46,4,26,B)
    return ic
def profile(ic,path,w=6.3,fill=A2,out=2.55):
    ic.stroke(path,INK,w+2*out); ic.stroke(path,fill,w)
@icon
def hem():
    ic=Icon('hem','Hem'); profile(ic,'M8 44L36 44A9 9 0 0 0 36 26L12 26'); return ic.recenter()

@icon
def fold():
    ic=Icon('fold','Fold')
    profile(ic,'M6 48L30 48',fill=P2); profile(ic,'M30 48L46 20',fill=A2)
    ic.stroke('M38 40A12 12 0 0 0 33 32',A3,AW*0.8) if False else None
    ic.stroke('M18 36A16 16 0 0 1 28 30',A3,AW) if False else None
    return ic.recenter()

@icon
def unfold():
    ic=Icon('unfold','Unfold')
    ic.stroke('M6 44L26 44L38 22',P3,AW)
    profile(ic,'M6 54L58 54',fill=A2) if False else None
    profile(ic,'M10 52L54 52',fill=A2)
    ic.stroke('M44 16C50 20 52 26 52 34',A3,AW) if False else None
    ic.stroke('M38 22C46 22 52 28 54 38',A3,AW); ic.fill(arrowhead(54,44,0.1,1,8,9),A3)
    return ic.recenter()

@icon
def refold():
    ic=Icon('refold','Refold')
    ic.stroke('M10 52L54 52',P3,AW)
    profile(ic,'M6 44L26 44',fill=P2) if False else None
    profile(ic,'M6 36L28 36',fill=P2); profile(ic,'M28 36L42 12',fill=A2)
    ic.stroke('M50 26C54 32 54 40 50 46',A3,AW); ic.fill(arrowhead(50,50,-0.4,1,8,9),A3)
    return ic.recenter()

@icon
def flat():
    ic=Icon('flat','Flat pattern'); I=Iso().fit([(0,0,0),(52,0,0),(52,36,0),(0,36,0),(0,0,4)],58,56)
    I.box(ic,0,0,0,52,36,4,G)
    for x in (17,35):
        ic.stroke(poly([I.p(x,3,4),I.p(x,33,4)],False),A3,AW)
    return ic
@icon
def bendtable():
    ic=Icon('bendtable','Bend table')
    fr=rrect(8,10,56,54,3); ic.fill(fr,P1)
    ic.fill('M9.3 11.3L54.7 11.3L54.7 22.3L9.3 22.3Z',A2)
    ic.stroke(fr,INK,OW); ic.stroke('M8 23L56 23M8 38L56 38M25 23L25 54M40 23L40 54',INK,OW)
    ic.stroke('M29 49L29 43L37 43L37 29',A3,AW) if False else None
    ic.stroke('M28 33L33 33L37 28 ',A3,AW*0.0001) if False else None
    ic.stroke('M43 50L48 50L53 42',A3,AW*0.85)
    return ic
@icon
def sheet_rules():
    ic=Icon('sheet_rules','Sheet metal rules'); I=Iso().fit([(0,0,0),(46,0,0),(46,34,0),(0,34,0),(0,0,4)],58,34)
    I.oy+=11
    I.box(ic,0,0,0,46,34,4,G)
    ic.stroke('M10 12L54 12',INK,OW)
    for x in (14,22,30,38,46): ic.stroke(poly([(x,12),(x,17 if x%16!=14 else 20)],False),A3,AW*0.7)
    return ic
def plastic_base(ic,I):
    """bottom and two back walls, one thickness; every flat face is a single polygon, so lines only appear where the surface changes angle"""
    t=5; h=32; S=40
    def face(pts,col):
        d=I.poly(*pts); ic.fill(d,col); ic.stroke(d,INK,OW)
    face([(t,t,t),(S,t,t),(S,S,t),(t,S,t)],P1)                         # floor
    face([(t,t,t),(S,t,t),(S,t,h),(t,t,h)],P2)                         # inside of the back right wall
    face([(t,t,t),(t,S,t),(t,S,h),(t,t,h)],P3)                         # inside of the back left wall
    face([(0,0,h),(S,0,h),(S,t,h),(t,t,h),(t,S,h),(0,S,h)],P1)         # top of both walls
    face([(S,0,0),(S,S,0),(S,S,t),(S,t,t),(S,t,h),(S,0,h)],P3)         # right outside, one face
    face([(0,S,0),(S,S,0),(S,S,t),(t,S,t),(t,S,h),(0,S,h)],P2)         # left outside, one face

@icon
def plastic_assign():
    ic=Icon('plastic_assign','Plastic assign'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,32),(40,40,32),(0,40,0),(40,0,0)],58,58)
    plastic_base(ic,I)
    cyl3(ic,I,24,24,7,5,20,B)                                 # a boss stands in front: the rule is assigned to it
    return ic.recenter()

@icon
def plastic_rule():
    ic=Icon('plastic_rule','Plastic rule'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,32),(40,40,32),(0,40,0),(40,0,0)],58,58)
    plastic_base(ic,I)
    # a ruler lies in front
    ang=math.radians(-28); c,sn=math.cos(ang),math.sin(ang); cx,cy,L,W=34,44,30,10
    def R(x,y): return (cx+x*c-y*sn,cy+x*sn+y*c)
    body=poly([R(-L/2,-W/2),R(L/2,-W/2),R(L/2,W/2),R(-L/2,W/2)])
    ic.stroke(body,A3,3.2,fill=A1)
    for i,x in enumerate(range(-10,11,5)):
        a=R(x,-W/2+1.2); b=R(x,-W/2+(4.6 if i%2==0 else 3.2)); ic.stroke(poly([a,b],False),A3,2.0)
    return ic
def plastic_assign():
    ic=Icon('plastic_assign','Plastic assign'); I=Iso().fit([(0,0,0),(40,40,0),(0,0,30),(40,40,30),(0,40,0),(40,0,0)],58,56)
    I.box(ic,0,0,0,40,40,30,G,OW,detail=False)
    t=6; z=30
    top=I.poly((t,t,z),(40-t,t,z),(40-t,40-t,z),(t,40-t,z)); ic.fill(top,P3)
    face(ic,I,[(t,t,z),(40-t,t,z),(40-t,t,12),(t,t,12)],P2,False)
    face(ic,I,[(t,t,z),(t,40-t,z),(t,40-t,12),(t,t,12)],P3,False)
    ic.fill(I.poly((t,t,12),(40-t,t,12),(40-t,40-t,12),(t,40-t,12)),A1)
    ic.stroke(top,INK,OW)
    ic.stroke(poly([I.p(t,t,z),I.p(t,t,12),I.p(40-t,t,12)],False),INK,OW)
    ic.stroke(poly([I.p(40,40,30),I.p(40,40,0)],False),INK,OW)
    ic.stroke(poly([I.p(0,40,30),I.p(40,40,30),I.p(40,0,30)],False),INK,OW)
    cyl3(ic,I,20,20,5,12,22,B)
    return ic
@icon
def surface():
    ic=Icon('surface','Surface'); I=Iso().fit([(0,0,0),(40,0,0),(40,40,0),(0,40,0),(0,0,12),(40,40,12)],58,52)
    surf_patch(ic,I); return ic
@icon
def appearance():
    ic=Icon('appearance','Appearance')
    d='M32 6C32 6 12 28 12 42A20 20 0 0 0 52 42C52 28 32 6 32 6Z'
    ic.fill(d,A2); ic.stroke(d,A3,AW); ic.stroke('M21 42A11 11 0 0 0 28 52',A1,AW)
    return ic
@icon
def pipe():
    ic=Icon('pipe','Pipe'); h=7.2; r=18; ex=3.4
    # flat cut at the top end (x = 52) is replaced by the opening; the bottom end is slightly convex
    d=('M%s 52L%s 32A%s %s 0 0 1 32 %sL52 %sL52 %sL32 %sA%s %s 0 0 0 %s 32L%s 52Q%s 55.6 %s 52Z'%
       (f(14-h),f(14-h),f(r+h),f(r+h),f(14-h),f(14-h),f(14+h),f(14+h),f(r-h),f(r-h),f(14+h),f(14+h),f(14),f(14-h)))
    ic.fill(d,A2)
    ic.stroke('M%s 50L%s 32A%s %s 0 0 1 32 %sL49 %s'%(f(14-h+2.8),f(14-h+2.8),f(r+h-2.8),f(r+h-2.8),f(14-h+2.8),f(14-h+2.8)),A1,2.4)
    ic.stroke(d,INK,OW)
    ic.fill(ell(52,14,ex,h),A1); ic.stroke(ell(52,14,ex,h),INK,OW)       # the open end, rim lit
    ic.fill(ell(52.4,14,ex*0.55,h-2.6),A3)                                # the inside of the tube
    return ic
def pipe():
    ic=Icon('pipe','Pipe'); h=7.2; r=18
    d=('M%s 52L%s 32A%s %s 0 0 1 32 %sL52 %sL52 %sL32 %sA%s %s 0 0 0 %s 32L%s 52Z'%
       (f(14-h),f(14-h),f(r+h),f(r+h),f(14-h),f(14-h),f(14+h),f(14+h),f(r-h),f(r-h),f(14+h),f(14+h)))
    ic.fill(d,A2)
    ic.stroke('M%s 50L%s 32A%s %s 0 0 1 32 %sL50 %s'%(f(14-h+2.8),f(14-h+2.8),f(r+h-2.8),f(r+h-2.8),f(14-h+2.8),f(14-h+2.8)),A1,2.4)
    ic.stroke(d,INK,OW)
    return ic
def pipe():
    ic=Icon('pipe','Pipe')
    d='M14 52L14 32A18 18 0 0 1 32 14L52 14'
    ic.stroke(d,INK,17.4); ic.stroke(d,A2,12.3)
    ic.stroke('M19 50L19 33A13 13 0 0 1 32 20L50 20',A1,3.2) if False else ic.stroke('M10.5 50L10.5 32A21.5 21.5 0 0 1 32 10.5L50 10.5',A1,2.4)
    ic.fill(ell(14,52,6,3),A3); ic.stroke(ell(14,52,6,3),INK,OW*0.8)
    ic.fill(ell(52,14,3,6),A3); ic.stroke(ell(52,14,3,6),INK,OW*0.8)
    return ic
