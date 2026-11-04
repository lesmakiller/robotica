import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Imu


class IMUSimulator(Node):
    def __init__(self):
        super().__init__('imu_simulator')
        self.pub = self.create_publisher(Imu, '/imu', 10)
        self.timer = self.create_timer(0.01, self.publish_imu)

    def publish_imu(self):
        msg = Imu()
        # Rotación constante en Z
        msg.angular_velocity.y = 1.0  # rad/s
        self.pub.publish(msg)


def main():
    rclpy.init()
    node = IMUSimulator()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
