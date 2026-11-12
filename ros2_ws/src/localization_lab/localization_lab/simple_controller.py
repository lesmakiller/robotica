import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist, TwistStamped


class SimpleController(Node):
    def __init__(self):
        super().__init__('simple_controller')
        self.declare_parameter('stamped', True)
        self.stamped = self.get_parameter('stamped').value
        msg_type = TwistStamped if self.stamped else Twist
        self.pub = self.create_publisher(msg_type, '/cmd_vel', 10)
        self.create_timer(0.1, self.move_robot)

    def move_robot(self):
        if self.stamped:
            msg = TwistStamped()
            msg.header.stamp = self.get_clock().now().to_msg()
            msg.header.frame_id = 'base_link'
            cmd = msg.twist
        else:
            msg = Twist()
            cmd = msg
        cmd.linear.x = 0.2
        cmd.angular.z = 0.1
        self.pub.publish(msg)


def main(args=None):
    rclpy.init(args=args)
    node = SimpleController()
    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        rclpy.shutdown()


if __name__ == '__main__':
    main()
