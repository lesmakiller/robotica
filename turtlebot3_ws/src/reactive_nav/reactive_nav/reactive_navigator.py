import rclpy
from rclpy.node import Node
from rclpy.qos import qos_profile_sensor_data
from sensor_msgs.msg import LaserScan
from geometry_msgs.msg import TwistStamped
import numpy as np


class ReactiveNavigator(Node):
    def __init__(self):
        super().__init__('reactive_navigator')
        self.sub = self.create_subscription(
            LaserScan, '/scan', self.scan_callback, qos_profile_sensor_data)
        self.pub = self.create_publisher(TwistStamped, '/cmd_vel', 10)

    def scan_callback(self, msg):
        r = np.array(msg.ranges, dtype=float)
        r[~np.isfinite(r)] = msg.range_max      # inf/nan -> libre
        n = len(r)
        k = n // 12                              # ±30° aprox.
        front = np.concatenate((r[:k], r[-k:]))  # 0° es el frente
        left = r[k:3*k].min()                    # lado izquierdo
        right = r[-3*k:-k].min()                 # lado derecho

        cmd = TwistStamped()
        cmd.header.stamp = self.get_clock().now().to_msg()
        cmd.header.frame_id = 'base_link'

        if front.min() < 0.5:
            cmd.twist.linear.x = 0.0
            # gira hacia el lado con más espacio
            cmd.twist.angular.z = 0.8 if left > right else -0.8
        else:
            cmd.twist.linear.x = 0.2
            cmd.twist.angular.z = 0.0
        self.pub.publish(cmd)


def main():
    rclpy.init()
    node = ReactiveNavigator()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()
