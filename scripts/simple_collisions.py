import time
import random
import math
from dataclasses import dataclass

import cv2
import numpy as np


DELTA_THETA = np.pi / 4


MIN_RADIUS = 40.0
MAX_RADIUS = 80.0
MIN_DELTA  = 20.0

DIM = 1024

LINE_RATE = 0.5

@dataclass
class Vec2:
    x: float
    y: float

@dataclass
class Line:
    b : float
    m : float

@dataclass
class VerticalLine:
    x : float

@dataclass
class Circle:
    x: float
    y: float
    r: float

@dataclass
class LineSegment:
    v0: Vec2
    v1: Vec2


AnyLine = Line | VerticalLine
Shape   = Circle | LineSegment

def circle_intersection_sloped_line(l: Line, c: Circle) -> bool:
    y = lambda x: (l.m * x + l.b)
    d = lambda x: (y(x) - c.y)**2 + (x - c.x)**2

    # d'(x) = 2 * y'(x) * (y(x) - c.y) + 2 * (x - c.x)
    # d'(x) = 2 * m * (m*x + b - c.y) + 2 * (x - c.x)

    m = l.m
    b = l.b

    x = (c.x - m*b + m*c.y) / (m*m + 1)

    return d(x) < c.r**2

def circle_intersection(l: AnyLine, c: Circle) -> bool:
    match l:
        case VerticalLine(x):
            return abs(c.x - x) <= c.r
        case _:
            return circle_intersection_sloped_line(l, c)

def line_segment_intersection(l: AnyLine, ls: LineSegment):
    match l:
        case VerticalLine(_):
            d = Vec2(0, 1)
        case Line(_, m):
            d = Vec2(1, m)

    # Point of intersection:
    # O + d*t = v0 + a*u where u is v1 - v0

    # O - v0 (O = (0, 0))
    p = Vec2(-ls.v0.x, -ls.v0.y)

    u = Vec2(ls.v1.x - ls.v0.x, ls.v1.y - ls.v0.y)

    # The strategy is to make  a system of linear equations then solve Ax = b where x = [a t]

    t = (p.y / u.y - p.x / u.x) / (d.y / u.y - d.x / u.x)

    # a + (d.x  / u.x) * t = p.x / u.x
    a = (p.x / u.x) - (d.x / u.x) * t

    return 0 <= a <= 1

def make_shapes():
    shapes: list[Shape] = []

    y = MAX_RADIUS
    while y < DIM - MAX_RADIUS:
        x = MAX_RADIUS
        y_incr = MAX_RADIUS
        while x < DIM - MAX_RADIUS:
            if random.random() < 0.9:
                x += MIN_RADIUS
                continue

            r = MIN_RADIUS + random.random() * (MAX_RADIUS - MIN_RADIUS)

            if random.random() < LINE_RATE:
                v0 = Vec2(x, y)
                m  = (random.random() + 0.1) / 1.1 * (-1 if random.random() < 0.5 else 1)
                v1 = Vec2(x + r, y + m*r)
                shapes.append(LineSegment(v0, v1))

            else:
                shapes.append(Circle(x, y, r))

            x += r + MAX_RADIUS + MIN_DELTA
            y_incr = max(y_incr, r + MIN_RADIUS)

        y += y_incr + MIN_DELTA

    return shapes


def make_line(th: float):
    if math.fmod(th, np.pi/2) < 1e-3:
        return VerticalLine(DIM / 2)

    # h = DIM / 2
    # h = mh + b
    # b = h - mh

    m = math.tan(th)
    b = (DIM / 2) * (1 - m)
    return Line(b, m)


RED  = (0, 0, 255)
GREY = (51, 51, 51)


def draw_circle(im: np.ndarray, c: Circle, col: tuple[int, int, int]):
    cv2.circle(im, (int(c.x), int(c.y)), int(c.r), col, -1)

def draw_line_segment(im: np.ndarray, l: LineSegment, col: tuple[int, int, int]):
    start = (int(l.v0.x + DIM / 2), int(l.v0.y + DIM/2))
    end   = (int(l.v1.x + DIM / 2), int(l.v1.y + DIM/2))
    cv2.line(im, start, end, col, 3)


def draw_line(im: np.ndarray, l: AnyLine, col: tuple[int, int, int]):
    start = (0, 0)
    end   = (DIM, DIM)
    match l:
        case VerticalLine(x):
            start = (int(x), 0)
            end   = (int(x), DIM)
        case Line(b, m):
            start = (0, int(b))
            end   = (DIM, int(m*DIM+b))

    cv2.line(im, start, end, col, 3)


def draw_shape(im: np.ndarray, shape: Shape, col: tuple[int, int, int]):
    match shape:
        case LineSegment():
            return draw_line_segment(im, shape, col)

        case Circle():
            return draw_circle(im, shape, col)


def shape_intersection(l: AnyLine, shape: Shape):
    match shape:
        case LineSegment():
            return line_segment_intersection(l, shape)
        case Circle():
            return circle_intersection(l, shape)


def main():
    shapes = make_shapes()

    im = np.zeros((DIM, DIM, 3), np.uint8)

    st = time.time()
    th = 0

    while (k := cv2.waitKey(16)) != ord('q'):
        im[:,:,:] = 255

        if k == ord('n'):
            shapes = make_shapes()

        line = make_line(th)

        draw_line(im, line, GREY)

        for shape in shapes:
            col = RED if shape_intersection(line, shape) else GREY
            draw_shape(im, shape, col)

        cv2.imshow("Collisions", im)

        ct = time.time()
        et = ct - st
        th += et * DELTA_THETA
        st = ct

        th = math.fmod(th, np.pi)

if __name__ == "__main__":
    main()
