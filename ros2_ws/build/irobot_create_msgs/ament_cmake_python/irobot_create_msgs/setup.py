from setuptools import find_packages
from setuptools import setup

setup(
  name='irobot_create_msgs',
  version='3.0.0',
  packages=find_packages(
      include=('irobot_create_msgs', 'irobot_create_msgs.*')),
)
