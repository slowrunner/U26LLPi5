import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/ubuntu/U26LLPi5/ros2_ws/install/turtlebot4_diagnostics'
