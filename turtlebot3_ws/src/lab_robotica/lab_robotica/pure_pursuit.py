import math
import numpy as np
from .common import TB3Base, wrap, run


class PurePursuit(TB3Base):
    def __init__(self):
        super().__init__('pure_pursuit')
        self.declare_parameter('path', [0.0, 0.0, 0.5, 1.0, 1.5, 1.25, 2.0, 2.5])
        for k, v in [('lookahead', 0.2), ('kh', 2.0), ('v', 0.15), ('tol', 0.05)]:
            self.declare_parameter(k, v)
        self.path = np.array(self.get_parameter('path').value).reshape(-1, 2)
        self.idx = 0

    def control(self):
        p = lambda n: self.get_parameter(n).value
        last = len(self.path) - 1
        tx, ty = self.path[self.idx]
        dist = math.hypot(tx - self.x, ty - self.y)
        if self.idx < last and dist < p('lookahead'):
            self.idx += 1
            tx, ty = self.path[self.idx]
            dist = math.hypot(tx - self.x, ty - self.y)
        if self.idx == last and dist < p('tol'):
            self.send(0.0, 0.0)
            return
        err = wrap(math.atan2(ty - self.y, tx - self.x) - self.theta)
        # frena si el objetivo está de lado/atrás
        self.send(p('v') * max(0.0, math.cos(err)), p('kh') * err)


def main():
    run(PurePursuit)
