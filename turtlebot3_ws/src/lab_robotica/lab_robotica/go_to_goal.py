import math
from .common import TB3Base, wrap, run


class GoToGoal(TB3Base):
    def __init__(self):
        super().__init__('go_to_goal')
        for k, v in [('goal_x', 2.0), ('goal_y', 2.0), ('kv', 0.5),
                     ('kh', 2.0), ('tol', 0.05)]:
            self.declare_parameter(k, v)

    def control(self):
        p = lambda n: self.get_parameter(n).value
        dx, dy = p('goal_x') - self.x, p('goal_y') - self.y
        dist = math.hypot(dx, dy)
        if dist < p('tol'):
            self.send(0.0, 0.0)
            return
        err = wrap(math.atan2(dy, dx) - self.theta)
        self.send(p('kv') * dist, p('kh') * err)


def main():
    run(GoToGoal)
