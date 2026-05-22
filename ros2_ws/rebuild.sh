#/bin/bash

echo -e "\n** SOURCE .bashrc"
. ~/.bashrc

echo -e "** CHANGE to ~/U26LLPi5/ros2_ws"
cd ~/U26LLPi5/ros2_ws

echo -e "** CHECK ROS DEPENDENCIES **"
rosdep install -i --from-path src

echo -e "** COLCON BUILD w/SYMLINK-INSTALL"
colcon build --symlink-install

echo -e "** SOURCE BUILT SETUP.BASH"
. install/setup.bash
