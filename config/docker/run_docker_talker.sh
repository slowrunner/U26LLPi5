#!/bin/bash

sudo docker run -it --net=host --privileged --rm --name ros2_pi --platform linux/arm64/v8 ros:kilted-desktop
