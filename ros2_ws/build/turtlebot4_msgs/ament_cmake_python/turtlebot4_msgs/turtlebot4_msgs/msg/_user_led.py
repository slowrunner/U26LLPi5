# generated from rosidl_generator_py/resource/_idl.py.em
# with input from turtlebot4_msgs:msg/UserLed.idl
# generated code does not contain a copyright notice

from __future__ import annotations

import collections.abc
import os
import typing

import rosidl_pycommon.interface_base_classes

if typing.TYPE_CHECKING:
    from ctypes import Structure

    class PyCapsule(Structure):
        pass  # don't need to define the full structure


# This is being done at the module level and not on the instance level to avoid looking
# for the same variable multiple times on each instance. This variable is not supposed to
# change during runtime so it makes sense to only look for it once.
ros_python_check_fields = os.getenv('ROS_PYTHON_CHECK_FIELDS', default='')


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_UserLed(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'UserLed'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class UserLedConstants(typing.TypedDict):
        USER_LED_1: typing.Literal[0]
        USER_LED_2: typing.Literal[1]
        COLOR_OFF: typing.Literal[0]
        COLOR_GREEN: typing.Literal[1]
        COLOR_RED: typing.Literal[2]
        COLOR_YELLOW: typing.Literal[3]

    __constants: UserLedConstants = {
        'USER_LED_1': 0,
        'USER_LED_2': 1,
        'COLOR_OFF': 0,
        'COLOR_GREEN': 1,
        'COLOR_RED': 2,
        'COLOR_YELLOW': 3,
    }

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('turtlebot4_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'turtlebot4_msgs.msg.UserLed')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__user_led
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__user_led
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__user_led
            cls._TYPE_SUPPORT = module.type_support_msg__msg__user_led
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__user_led

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'USER_LED_1': metacls.__constants['USER_LED_1'],
            'USER_LED_2': metacls.__constants['USER_LED_2'],
            'COLOR_OFF': metacls.__constants['COLOR_OFF'],
            'COLOR_GREEN': metacls.__constants['COLOR_GREEN'],
            'COLOR_RED': metacls.__constants['COLOR_RED'],
            'COLOR_YELLOW': metacls.__constants['COLOR_YELLOW'],
        }

    @property
    def USER_LED_1(self) -> typing.Literal[0]:
        """Message constant 'USER_LED_1'."""
        return Metaclass_UserLed.__constants['USER_LED_1']

    @property
    def USER_LED_2(self) -> typing.Literal[1]:
        """Message constant 'USER_LED_2'."""
        return Metaclass_UserLed.__constants['USER_LED_2']

    @property
    def COLOR_OFF(self) -> typing.Literal[0]:
        """Message constant 'COLOR_OFF'."""
        return Metaclass_UserLed.__constants['COLOR_OFF']

    @property
    def COLOR_GREEN(self) -> typing.Literal[1]:
        """Message constant 'COLOR_GREEN'."""
        return Metaclass_UserLed.__constants['COLOR_GREEN']

    @property
    def COLOR_RED(self) -> typing.Literal[2]:
        """Message constant 'COLOR_RED'."""
        return Metaclass_UserLed.__constants['COLOR_RED']

    @property
    def COLOR_YELLOW(self) -> typing.Literal[3]:
        """Message constant 'COLOR_YELLOW'."""
        return Metaclass_UserLed.__constants['COLOR_YELLOW']


class UserLed(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_UserLed):
    """
    Message class 'UserLed'.

    Constants:
      USER_LED_1
      USER_LED_2
      COLOR_OFF
      COLOR_GREEN
      COLOR_RED
      COLOR_YELLOW
    """

    __slots__ = [
        '_led',
        '_color',
        '_blink_period',
        '_duty_cycle',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'led': 'uint8',
        'color': 'uint8',
        'blink_period': 'uint32',
        'duty_cycle': 'double',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint32'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
    )

    def __init__(self, *,
                 led: typing.Optional[int] = None,  # noqa: E501
                 color: typing.Optional[int] = None,  # noqa: E501
                 blink_period: typing.Optional[int] = None,  # noqa: E501
                 duty_cycle: typing.Optional[float] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.led = led if led is not None else int()
        self.color = color if color is not None else int()
        self.blink_period = blink_period if blink_period is not None else int()
        self.duty_cycle = duty_cycle if duty_cycle is not None else float()

    def __repr__(self) -> str:
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args: list[str] = []
        for s, t in zip(self.get_fields_and_field_types().keys(), self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    from rosidl_buffer import Buffer as _RosidlBuffer
                    if not isinstance(field, _RosidlBuffer):
                        if self._check_fields:
                            assert fieldstr.startswith('array(')
                        prefix = "array('X', "
                        suffix = ')'
                        fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, UserLed):
            return False
        if self.led != other.led:
            return False
        if self.color != other.color:
            return False
        if self.blink_period != other.blink_period:
            return False
        if self.duty_cycle != other.duty_cycle:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def led(self) -> int:
        """Message field 'led'."""
        return self._led

    @led.setter
    def led(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'led' field must be of type 'int'"
                assert value >= 0 and value < 256, \
                    "The 'led' field must be an unsigned integer in [0, 255]"

        self._led = value

    @builtins.property
    def color(self) -> int:
        """Message field 'color'."""
        return self._color

    @color.setter
    def color(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'color' field must be of type 'int'"
                assert value >= 0 and value < 256, \
                    "The 'color' field must be an unsigned integer in [0, 255]"

        self._color = value

    @builtins.property
    def blink_period(self) -> int:
        """Message field 'blink_period'."""
        return self._blink_period

    @blink_period.setter
    def blink_period(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'blink_period' field must be of type 'int'"
                assert value >= 0 and value < 4294967296, \
                    "The 'blink_period' field must be an unsigned integer in [0, 4294967295]"

        self._blink_period = value

    @builtins.property
    def duty_cycle(self) -> float:
        """Message field 'duty_cycle'."""
        return self._duty_cycle

    @duty_cycle.setter
    def duty_cycle(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'duty_cycle' field must be of type 'float'"
                assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                    "The 'duty_cycle' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"

        self._duty_cycle = value
