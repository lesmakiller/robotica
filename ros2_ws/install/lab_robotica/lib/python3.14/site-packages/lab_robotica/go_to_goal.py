import math
import numpy as np
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import TwistStamped
from nav_msgs.msg import Odometry


def wrap(a):
    return (a + np.pi) % (2 * np.pi) - np.pi


class GoToGoal(Node):
    def __init__(self):
        super().__init__('go_to_goal')
        self.pub = self.create_publisher(TwistStamped, '/cmd_vel', 10)
        self.sub = self.create_subscription(Odometry, '/odom', self.odom_callback, 10)
        self.timer = self.create_timer(0.1, self.control)
        self.x = self.y = self.theta = 0.0
        self.Kv, self.Kh = 0.5, 2.0
        self.goal = np.array([2.0, 2.0])

    def odom_callback(self, msg):
        self.x = msg.pose.pose.position.x
        self.y = msg.pose.pose.position.y
        q = msg.pose.pose.orientation
        siny_cosp = 2 * (q.w * q.z + q.x * q.y)
        cosy_cosp = 1 - 2 * (q.y * q.y + q.z * q.z)
        self.theta = math.atan2(siny_cosp, cosy_cosp)

    def control(self):
        dx = self.goal[0] - self.x
        dy = self.goal[1] - self.y
        distance = np.hypot(dx, dy)
        error = wrap(np.arctan2(dy, dx) - self.theta)

        if distance < 0.05:
            v, w = 0.0, 0.0
        else:
            v = self.Kv * distance
            w = self.Kh * error

        msg = TwistStamped()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = "base_link"
        msg.twist.linear.x = float(v)
        msg.twist.angular.z = float(w)
        self.pub.publish(msg)


def main():
    rclpy.init()
    node = GoToGoal()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
