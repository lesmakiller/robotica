import math
from .common import TB3Base, wrap, run


class LineFollower(TB3Base):
    def __init__(self):
        super().__init__('line_follower')
        # Línea a*x + b*y + c = 0 (en el marco /odom, que parte en (0,0))
        for k, v in [('a', 1.0), ('b', -2.0), ('c', 1.0),
                     ('kd', 1.0), ('kh', 2.0), ('v', 0.15),
                     ('length', 4.0)]:          # metros a recorrer sobre la línea
            self.declare_parameter(k, v)
        self.start = None
        self.done = False

    def control(self):
        p = lambda n: self.get_parameter(n).value
        a, b, c = p('a'), p('b'), p('c')
        th_line = math.atan2(a, -b)             # orientación de la línea

        if self.start is None:
            self.start = (self.x, self.y)

        # avance a lo largo de la línea desde el punto de partida
        s = ((self.x - self.start[0]) * math.cos(th_line)
             + (self.y - self.start[1]) * math.sin(th_line))

        if s >= p('length'):
            if not self.done:
                self.get_logger().info(f'Fin: recorrido {s:.2f} m sobre la línea')
                self.done = True
            self.send(0.0, 0.0)
            return

        d = (a * self.x + b * self.y + c) / math.hypot(a, b)   # distancia con signo
        w = p('kd') * d + p('kh') * wrap(th_line - self.theta)
        self.send(p('v'), w)


def main():
    run(LineFollower)
