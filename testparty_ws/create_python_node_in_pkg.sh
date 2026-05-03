#!/bin/bash

basedir=U26LLPi5
rosws=testparty_ws
rosdist=lyrical

echo -e "\n*** Switching to ~/${basedir}/${rosws}"
cd ~/$basedir/$rosws

echo -e "\n*** Sourcing /opt/ros/${rosdist}/setup.bash"
. /opt/ros/$rosdist/setup.bash

echo -e "\n*** Sourcing install/setup.bash"
. ~/$basedir/$rosws/install/setup.bash


if [ "$#" -ne 2 ]; then
  echo "USAGE: ./create_python_node_in_pkg.sh node_name pkg_name"
  exit
fi
echo -e "\n*** CREATE ROS 2 NODE $1 IN PACKAGE $2"
echo -e "ros2 pkg create --build-type ament_python --license Apache-2.0 --node-name $1 $2"
pushd src
ros2 pkg create --build-type ament_python --license Apache-2.0 --node-name $1 $2
popd
