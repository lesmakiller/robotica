import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/lesmakiller-u/robotica/turtlebot3_ws/install/turtlebot3_teleop'
