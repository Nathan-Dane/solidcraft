from lib import *
from i2 import arrowhead
ICONS={}
def icon(fn): ICONS[fn.__name__]=fn; return fn
# The sketch constraint glyphs keep the drawings the app has always used (24 grid), scaled to the 64 grid:
# accent strokes for the geometry the constraint acts on, ink for the helper lines.
K=2.4; O=3.2
def T(pts): return [(K*x+O,K*y+O) for x,y in pts]
def acc(ic,pts): ic.stroke(poly(T(pts),False),A3,AW)
def ink(ic,pts): ic.stroke(poly(T(pts),False),INK,AW)
def circ(ic,x,y,r,c=A3,fill=None): X,Y=T([(x,y)])[0]; ic.stroke(ell(X,Y,K*r),c,AW,fill=fill)
def tri(ic,pts): ic.stroke(poly(T(pts)),A3,AW*0.5,fill=A3)

@icon
def c_hv():
    ic=Icon('c_hv','Horizontal / vertical'); acc(ic,[(10,18),(21,18)]); acc(ic,[(6,4),(6,14)]); return ic.recenter()
def c_hv():
    ic=Icon('c_hv','Horizontal / vertical'); acc(ic,[(4,18),(20,18)]); acc(ic,[(6,4),(6,14)]); return ic.recenter()
@icon
def c_coincident():
    ic=Icon('c_coincident','Coincident'); ink(ic,[(3,18),(12,12)]); ink(ic,[(12,12),(21,16)]); circ(ic,12,12,2.6,A3,A1); return ic.recenter()
@icon
def c_tangent():
    ic=Icon('c_tangent','Tangent'); circ(ic,12,14,6,INK); acc(ic,[(3,8),(21,8)]); return ic.recenter()
@icon
def c_equal():
    ic=Icon('c_equal','Equal'); acc(ic,[(5,9),(19,9)]); acc(ic,[(5,15),(19,15)]); return ic.recenter()
@icon
def c_parallel():
    ic=Icon('c_parallel','Parallel')          # the equal icon, turned 45 degrees
    th=math.radians(-45)
    def R(p): x,y=p[0]-12,p[1]-12; return (12+x*math.cos(th)-y*math.sin(th),12+x*math.sin(th)+y*math.cos(th))
    acc(ic,[R((5,9)),R((19,9))]); acc(ic,[R((5,15)),R((19,15))]); return ic.recenter()
def c_parallel():
    ic=Icon('c_parallel','Parallel'); acc(ic,[(5,19),(13,5)]); acc(ic,[(11,19),(19,5)]); return ic.recenter()
@icon
def c_perpendicular():
    ic=Icon('c_perpendicular','Perpendicular'); acc(ic,[(4,19),(20,19)]); acc(ic,[(12,19),(12,5)]); ink(ic,[(12,15),(16,15),(16,19)]); return ic.recenter()
@icon
def c_fix():
    ic=Icon('c_fix','Fixed'); circ(ic,12,9,4); acc(ic,[(12,13),(12,21)]); ink(ic,[(9,21),(15,21)]); return ic.recenter()
@icon
def c_midpoint():
    ic=Icon('c_midpoint','Midpoint'); ink(ic,[(3,15),(21,9)])
    X,Y=T([(12,12)])[0]; ic.dot(X,Y,7.3,GAP); ic.dot(X,Y,5.6,A3); return ic.recenter()
def c_midpoint():
    ic=Icon('c_midpoint','Midpoint'); ink(ic,[(3,15),(21,9)]); tri(ic,[(12,9),(15,12),(12,15),(9,12)]); return ic.recenter()
@icon
def c_concentric():
    ic=Icon('c_concentric','Concentric'); circ(ic,12,12,8.5); circ(ic,12,12,4); return ic.recenter()
@icon
def c_collinear():
    ic=Icon('c_collinear','Collinear'); acc(ic,[(3,19),(10,12)]); acc(ic,[(14,8),(21,1)]); return ic.recenter()
@icon
def c_symmetry():
    ic=Icon('c_symmetry','Symmetry'); ink(ic,[(12,3),(12,21)]); tri(ic,[(4,8),(9,12),(4,16)]); tri(ic,[(20,8),(15,12),(20,16)]); return ic.recenter()
@icon
def c_smooth():
    ic=Icon('c_smooth','Curvature')
    pts=[(3+18*t/24,12-7*math.sin(2*math.pi*t/24)) for t in range(25)]
    ink(ic,pts[:13]); acc(ic,pts[12:])
    X,Y=T([(12,12)])[0]; ic.dot(X,Y,3.4,INK); return ic.recenter()
@icon
def c_polygon():
    ic=Icon('c_polygon','Regular polygon')
    pts=[(12+8.5*math.cos(i/6*2*math.pi),12+8.5*math.sin(i/6*2*math.pi)) for i in range(6)]
    ic.stroke(poly(T(pts)),A3,AW); ink(ic,[(12,12),(20.5,12)]); X,Y=T([(12,12)])[0]; ic.dot(X,Y,3.4,INK); return ic.recenter()
@icon
def extend():
    ic=Icon('extend','Extend')
    ic.stroke('M7 28L32 28',P3,AW)                                   # the existing line
    ic.stroke('M32 28L57 28',A3,3.6,dash='0.1 5.6')                  # its continuation, dotted
    ic.stroke('M34 44L50 44',A3,AW); ic.fill(arrowhead(57,44,1,0,11,12),A3)   # the way it grows
    return ic.recenter()
