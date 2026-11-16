import numpy as np
import rclpy
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import LaserScan
from geometry_msgs.msg import Twist, TwistStamped
import random

class ReactiveController(Node):
    def __init__(self):
        super().__init__('reactive_controller')
        self.declare_parameter('stamped', True)
        self.declare_parameter('v_max', 0.12)     # m/s
        self.declare_parameter('w_max', 0.8)      # rad/s
        self.declare_parameter('d_stop', 0.6)     # m, distancia para girar
        self.declare_parameter('d_cap', 1.5)      # m, ignora lo lejano
        self.declare_parameter('k_wall', 0.8)     # ganancia de centrado
        self.declare_parameter('d_slow', 1.0)     # m, empieza a frenar
        self.declare_parameter('d_crit', 0.20)    # m, emergencia
        self.declare_parameter('d_side', 0.30)    # m, repulsión lateral
        self.declare_parameter('half_width', 0.20)   # m, semiancho del pasillo
        self.declare_parameter('d_center', 1.0)   # m, centrar solo si ambos lados están más cerca
        self.wander_w = 0.0
        self.create_timer(4.0, self.new_wander)

        self.stamped = self.get_parameter('stamped').value
        msg_type = TwistStamped if self.stamped else Twist
        self.pub = self.create_publisher(msg_type, '/cmd_vel', 10)
        self.sub = self.create_subscription(
            LaserScan, '/scan', self.on_scan, qos_profile_sensor_data)
        self.turn_dir = 0   # recuerda el sentido de giro para no oscilar

    def new_wander(self):
        self.wander_w = random.uniform(-0.15, 0.15)   # evita órbitas repetidas

    @staticmethod
    def sector_min(ranges, angles, lo, hi, default):
        m = (angles >= lo) & (angles <= hi)
        vals = ranges[m]
        return float(vals.min()) if vals.size else default

    def on_scan(self, scan: LaserScan):
        p = lambda n: self.get_parameter(n).value
        r = np.array(scan.ranges, dtype=float)
        # +inf = libre; -inf/NaN/menor al mínimo = demasiado cerca
        r = np.where(np.isposinf(r), scan.range_max, r)
        r = np.where(~np.isfinite(r) | (r < scan.range_min), 0.0, r)
        a = scan.angle_min + np.arange(r.size) * scan.angle_increment
        a = np.arctan2(np.sin(a), np.cos(a))      # 0 = frente

        cap = p('d_cap')
        sm = self.sector_min
        x = r * np.cos(a)                  # distancia hacia adelante
        y = r * np.sin(a)                  # desviación lateral
        corr = (x > 0.0) & (np.abs(y) < p('half_width'))
        front = min(float(x[corr].min()) if corr.any() else cap, cap)
        #front = min(sm(r, a, -0.79, 0.79, cap), cap)     # ±45°
        left  = min(sm(r, a, 0.79, 2.09, cap), cap)      # 45°–120°
        right = min(sm(r, a, -2.09, -0.79, cap), cap)
        rear  = min(sm(r, a, 2.09, 3.15, cap), cap)
        rear  = min(rear, sm(r, a, -3.15, -2.09, cap))
        near  = float(r.min())

        v, w = 0.0, 0.0
        if near < p('d_crit'):                    # emergencia: alejarse
            if front <= rear and rear > 0.30:
                v = -0.05
            w = self.turn_dir * 0.5 if self.turn_dir else (0.5 if left > right else -0.5)
        elif front < p('d_stop'):                 # obstáculo: parar y girar
            if self.turn_dir == 0:
                self.turn_dir = 1 if left > right else -1
            w = self.turn_dir * p('w_max')
        elif self.turn_dir != 0 and front < p('d_stop') + 0.25:
            w = self.turn_dir * p('w_max')        # histéresis: termina el giro
        else:
            self.turn_dir = 0
            s = np.clip((front - p('d_stop')) / (p('d_slow') - p('d_stop')), 0.0, 1.0)
            v = p('v_max') * (0.3 + 0.7 * s)
            # w = p('k_wall') * (left - right)
            w = self.wander_w
            if left < p('d_center') and right < p('d_center'):
                w += p('k_wall') * (left - right)     # centrar solo en pasillos
            if left < p('d_side'):                # repulsión lateral
                w -= 1.0 * (p('d_side') - left)
            if right < p('d_side'):
                w += 1.0 * (p('d_side') - right)
            w = float(np.clip(w, -p('w_max'), p('w_max')))
        self.publish(v, w)

    def publish(self, v, w):
        if self.stamped:
            msg = TwistStamped()
            msg.header.stamp = self.get_clock().now().to_msg()
            msg.header.frame_id = 'base_link'
            cmd = msg.twist
        else:
            msg = Twist()
            cmd = msg
        cmd.linear.x = float(v)
        cmd.angular.z = float(w)
        self.pub.publish(msg)


def main(args=None):
    rclpy.init(args=args)
    node = ReactiveController()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.publish(0.0, 0.0)    # detener al salir
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main() 
