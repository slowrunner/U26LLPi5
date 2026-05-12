#!/bin/bash



cd ~/U26LLPi5/testparty_ws

echo "Cleaning testparty_ws (log, install, build)"

rm -rf log
rm -rf install
rm -rf build

echo "Cleaning AMENT and CMAKE PREFIX paths"
unset AMENT_PREFIX_PATH
unset AMENT_CMAKE_PREFIX_PATH
unset CMAKE_PREFIX_PATH

echo "Cleaned"
