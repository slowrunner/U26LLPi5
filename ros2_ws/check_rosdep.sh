#/bin/bash

echo -e "\n** SOURCE .bashrc"
. ~/.bashrc

echo -e "** CHANGE to ~/U26LLPi5/ros2_ws"
cd ~/U26LLPi5/ros2_ws

echo -e "** CHECK ROS DEPENDENCIES **"
# rosdep install -i --from-path src
rosdep install --from-paths src --ignore-src -r -y --skip-keys \
  "depthai-ros depthai_ros_msgs slam_toolbox nav2_simple_commander navigation2 btcpp_ros2_interfaces behaviortree_ros2 turtlebot4_bringup turtlebot4_navigation turtlebot4_node nav2_bringup nav2_common"

