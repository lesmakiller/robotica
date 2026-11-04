import math
import numpy as np
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import TwistStamped
from nav_msgs.msg import Odometry


def wrap(a):
    return (a + np.pi) % (2 * np.pi) - np.pi


class FollowPath(Node):
    def __init__(self):
        super().__init__('follow_path')
        self.pub = self.create_publisher(TwistStamped, '/cmd_vel', 10)
        self.create_subscription(Odometry, '/odom', self.odom_callback, 10)
        self.create_timer(0.1, self.control)
        self.x = self.y = self.theta = 0.0
        self.path = np.array([[0, 0], [1, 0], [1.5, 1], [2.5, 1.5]], dtype=float)
        self.i = 1                       # el robot ya está en el punto 0
        self.lookahead, self.goal_tol = 0.15, 0.05
        self.vmax, self.Kd = 0.15, 1.5

    def odom_callback(self, msg):
        self.x = msg.pose.pose.position.x
        self.y = msg.pose.pose.position.y
        q = msg.pose.pose.orientation
        self.theta = math.atan2(2 * (q.w * q.z + q.x * q.y),
                                1 - 2 * (q.y * q.y + q.z * q.z))

    def send(self, v, w):
        m = TwistStamped()
        m.header.stamp = self.get_clock().now().to_msg()
        m.header.frame_id = "base_link"
        m.twist.linear.x = float(v)
        m.twist.angular.z = float(w)
        self.pub.publish(m)

    def control(self):
        last = len(self.path) - 1
        dist = np.hypot(self.path[self.i][0] - self.x, self.path[self.i][1] - self.y)
        if self.i < last and dist < self.lookahead:
            self.i += 1
        elif self.i == last and dist < self.goal_tol:
            self.send(0.0, 0.0)          # llegó a la meta
            return
        tx, ty = self.path[self.i]
        error = wrap(np.arctan2(ty - self.y, tx - self.x) - self.theta)
        v = self.vmax * max(0.0, np.cos(error))    # frena si está mal orientado
        w = max(-2.84, min(2.84, self.Kd * error))
        self.send(v, w)


def main():
    rclpy.init()
    node = FollowPath()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.send(0.0, 0.0)
        node.destroy_node()
        rclpy.try_shutdown()
