import csv, math, os
import rclpy
from rclpy.node import Node
from rclpy.executors import ExternalShutdownException
from geometry_msgs.msg import TwistStamped
from nav_msgs.msg import Odometry


def wrap(a):
    return (a + math.pi) % (2 * math.pi) - math.pi


class TB3Base(Node):
    def __init__(self, name):
        super().__init__(name)
        self.pub = self.create_publisher(TwistStamped, '/cmd_vel', 10)
        self.create_subscription(Odometry, '/odom', self.odom_cb, 10)
        self.create_timer(0.1, self._tick)
        self.x = self.y = self.theta = 0.0
        self.have_odom = False
        self.log = []

    def odom_cb(self, msg):
        p, q = msg.pose.pose.position, msg.pose.pose.orientation
        self.x, self.y = p.x, p.y
        self.theta = math.atan2(2 * (q.w * q.z + q.x * q.y),
                                1 - 2 * (q.y * q.y + q.z * q.z))
        self.have_odom = True

    def _tick(self):
        if not self.have_odom:
            return
        self.control()
        self.log.append((self.get_clock().now().nanoseconds * 1e-9,
                         self.x, self.y, self.theta))

    def control(self):
        raise NotImplementedError

    def send(self, v, w, vmax=0.22, wmax=2.84):   # límites del burger
        msg = TwistStamped()
        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = 'base_link'
        msg.twist.linear.x = max(-vmax, min(vmax, v))
        msg.twist.angular.z = max(-wmax, min(wmax, w))
        self.pub.publish(msg)

    def save_csv(self):
        d = os.path.expanduser('~/robotica/resultados')
        os.makedirs(d, exist_ok=True)
        path = os.path.join(d, self.get_name() + '.csv')
        with open(path, 'w', newline='') as f:
            wr = csv.writer(f)
            wr.writerow(['t', 'x', 'y', 'theta'])
            wr.writerows(self.log)
        self.get_logger().info(f'CSV guardado en {path}')


def run(cls):
    rclpy.init()
    node = cls()
    try:
        rclpy.spin(node)
    except (KeyboardInterrupt, ExternalShutdownException):
        pass
    node.save_csv()
    try:
        node.send(0.0, 0.0)
    except Exception:
        pass
    node.destroy_node()
    rclpy.try_shutdown()
