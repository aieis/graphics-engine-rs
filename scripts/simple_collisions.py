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

@dataclass
class Line:
    b : float
    m : float

@dataclass
class VerticalLine:
    x : float

AnyLine = Line | VerticalLine

@dataclass
class Circle:
    x: float
    y: float
    r: float


def line_intersection(l: Line, c: Circle) -> bool:
    y = lambda x: (l.m * x + l.b)
    d = lambda x: (y(x) - c.y)**2 + (x - c.x)**2

    # d'(x) = 2 * y'(x) * (y(x) - c.y) + 2 * (x - c.x)
    # d'(x) = 2 * m * (m*x + b - c.y) + 2 * (x - c.x)

    m = l.m
    b = l.b

    x = (c.x - m*b + m*c.y) / (m*m + 1)

    return d(x) < c.r**2

def intersection(l: AnyLine, c: Circle) -> bool:
    match l:
        case VerticalLine(x):
            return abs(c.x - x) <= c.r
        case _:
            return line_intersection(l, c)


def make_circles():
    circles = []

    y = MAX_RADIUS
    while y < DIM - MAX_RADIUS:
        x = MAX_RADIUS
        y_incr = MAX_RADIUS
        while x < DIM - MAX_RADIUS:
            if random.random() < 0.9:
                x += MIN_RADIUS
                continue

            r = MIN_RADIUS + random.random() * (MAX_RADIUS - MIN_RADIUS)
            circles.append(Circle(x, y, r))

            x += r + MAX_RADIUS + MIN_DELTA
            y_incr = max(y_incr, r + MIN_RADIUS)

        y += y_incr + MIN_DELTA

    return circles

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

def main():
    circles = make_circles()

    im = np.zeros((DIM, DIM, 3), np.uint8)

    st = time.time()
    th = 0

    while (k := cv2.waitKey(16)) != ord('q'):
        im[:,:,:] = 255

        if k == ord('n'):
            circles = make_circles()

        line = make_line(th)

        draw_line(im, line, GREY)

        for circle in circles:
            col = RED if intersection(line, circle) else GREY
            draw_circle(im, circle, col)

        cv2.imshow("Collisions", im)

        ct = time.time()
        et = ct - st
        th += et * DELTA_THETA
        st = ct

        th = math.fmod(th, np.pi)

if __name__ == "__main__":
    main()
