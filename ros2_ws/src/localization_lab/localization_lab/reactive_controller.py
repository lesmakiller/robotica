import numpy as np
import rclpy
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import LaserScan
from geometry_msgs.msg import Twist, TwistStamped


class ReactiveController(Node):
    def __init__(self):
        super().__init__('reactive_controller')
        self.declare_parameter('stamped', True)
        self.declare_parameter('v_max', 0.15)     # m/s
        self.declare_parameter('w_max', 0.8)      # rad/s
        self.declare_parameter('d_stop', 0.45)    # m, distancia para girar
        self.declare_parameter('d_cap', 1.5)      # m, ignora lo lejano
        self.declare_parameter('k_wall', 0.8)     # ganancia de centrado

        self.stamped = self.get_parameter('stamped').value
        msg_type = TwistStamped if self.stamped else Twist
        self.pub = self.create_publisher(msg_type, '/cmd_vel', 10)
        self.sub = self.create_subscription(
            LaserScan, '/scan', self.on_scan, qos_profile_sensor_data)
        self.turn_dir = 0   # recuerda el sentido de giro para no oscilar

    @staticmethod
    def sector_min(ranges, angles, lo, hi, default):
        m = (angles >= lo) & (angles <= hi)
        vals = ranges[m]
        return float(vals.min()) if vals.size else default

    def on_scan(self, scan: LaserScan):
        p = lambda n: self.get_parameter(n).value
        r = np.array(scan.ranges, dtype=float)
        r = np.where(np.isfinite(r) & (r >= scan.range_min), r, scan.range_max)
        a = scan.angle_min + np.arange(r.size) * scan.angle_increment
        a = np.arctan2(np.sin(a), np.cos(a))      # a [-pi, pi], 0 = frente

        cap = p('d_cap')
        front = min(self.sector_min(r, a, -0.52, 0.52, cap), cap)    # ±30°
        left  = min(self.sector_min(r, a, 0.52, 1.57, cap), cap)     # 30°-90°
        right = min(self.sector_min(r, a, -1.57, -0.52, cap), cap)

        if front < p('d_stop'):                   # obstáculo: parar y girar
            if self.turn_dir == 0:
                self.turn_dir = 1 if left > right else -1   # hacia el lado libre
            v, w = 0.0, self.turn_dir * p('w_max')
        else:                                     # libre: avanzar y centrarse
            self.turn_dir = 0
            v = p('v_max') if front > 0.8 else 0.5 * p('v_max')
            w = float(np.clip(p('k_wall') * (left - right),
                              -p('w_max'), p('w_max')))
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
        cmd.linear.x = v
        cmd.angular.z = w
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
