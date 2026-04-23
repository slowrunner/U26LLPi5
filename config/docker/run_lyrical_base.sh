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
 arm64v8/ros:lyrical-ros-base
