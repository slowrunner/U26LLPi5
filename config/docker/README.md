# ROS2 IN DOCKER ON RASPBERRY PI 5

# ROS 2 Lyrical Luth Setup



# DOCKER INSTALL:

config/docker/install-docker.sh  

=== Docker Install  ====  

?? curl -fsSL https://get.docker.com -o get-docker.sh  
?? sudo sh get-docker.sh  

sudo apt install docker.io  

sudo usermod -aG docker $USER  

sudo apt install util-linux-extra  
newgrp docker  

docker -version  
docker run hello-world  


=== Pull  Pi5 ROS2 Rolling Ridley ===

config/docker/pull_rolling_desktop.sh  
or
docker pull arm64v8/ros:rolling-ros-base  



ros2 --version

config/docker/run_rolling_docker.sh

==== run_rolling_docker.sh ===
#!/bin/bash

docker run -it --net=host \
 -v /dev/snd:/dev/snd \
 -v /dev/input:/dev/input \
 -v /home/ubuntu:/home/ubuntu \
 -v /dev/bus/usb:/dev/bus/usb \
 -v /dev/ttyUSB0:/dev/ttyUSB0 \
 -v /var/lock:/var/lock \
 -e TZ=America/New_York \
 -w /home/ubuntu/U26LLPi5/testparty_ws \
 --privileged \
 --rm \
 arm64v8/ros:rolling-ros-base

=== test with demo nodes

config/docker/run_rolling_docker.sh

ros2 -version

sudo apt update
sudo apt install ros-rolling-demo-nodes-cpp

ros2 run demo_nodes_cpp listener & 
ros2 run demo_nodes_cpp talker


 
