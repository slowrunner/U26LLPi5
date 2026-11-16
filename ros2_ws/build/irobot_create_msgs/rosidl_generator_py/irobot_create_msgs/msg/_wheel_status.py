# generated from rosidl_generator_py/resource/_idl.py.em
# with input from irobot_create_msgs:msg/WheelStatus.idl
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


if typing.TYPE_CHECKING:
    import std_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_WheelStatus(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'WheelStatus'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class WheelStatusConstants(typing.TypedDict):
        pass

    __constants: WheelStatusConstants = {
    }

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('irobot_create_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'irobot_create_msgs.msg.WheelStatus')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__wheel_status
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__wheel_status
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__wheel_status
            cls._TYPE_SUPPORT = module.type_support_msg__msg__wheel_status
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__wheel_status

            from std_msgs.msg import Header
            if Header._TYPE_SUPPORT is None:
                Header.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class WheelStatus(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_WheelStatus):
    """Message class 'WheelStatus'."""

    __slots__ = [
        '_header',
        '_current_ma_left',
        '_current_ma_right',
        '_pwm_left',
        '_pwm_right',
        '_wheels_enabled',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'header': 'std_msgs/Header',
        'current_ma_left': 'int16',
        'current_ma_right': 'int16',
        'pwm_left': 'int16',
        'pwm_right': 'int16',
        'wheels_enabled': 'boolean',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['std_msgs', 'msg'], 'Header'),  # noqa: E501
        rosidl_parser.definition.BasicType('int16'),  # noqa: E501
        rosidl_parser.definition.BasicType('int16'),  # noqa: E501
        rosidl_parser.definition.BasicType('int16'),  # noqa: E501
        rosidl_parser.definition.BasicType('int16'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, *,
                 header: typing.Optional[std_msgs.msg.Header] = None,  # noqa: E501
                 current_ma_left: typing.Optional[int] = None,  # noqa: E501
                 current_ma_right: typing.Optional[int] = None,  # noqa: E501
                 pwm_left: typing.Optional[int] = None,  # noqa: E501
                 pwm_right: typing.Optional[int] = None,  # noqa: E501
                 wheels_enabled: typing.Optional[bool] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from std_msgs.msg import Header
        self.header = header if header is not None else Header()
        self.current_ma_left = current_ma_left if current_ma_left is not None else int()
        self.current_ma_right = current_ma_right if current_ma_right is not None else int()
        self.pwm_left = pwm_left if pwm_left is not None else int()
        self.pwm_right = pwm_right if pwm_right is not None else int()
        self.wheels_enabled = wheels_enabled if wheels_enabled is not None else bool()

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
        if not isinstance(other, WheelStatus):
            return False
        if self.header != other.header:
            return False
        if self.current_ma_left != other.current_ma_left:
            return False
        if self.current_ma_right != other.current_ma_right:
            return False
        if self.pwm_left != other.pwm_left:
            return False
        if self.pwm_right != other.pwm_right:
            return False
        if self.wheels_enabled != other.wheels_enabled:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def header(self) -> std_msgs.msg.Header:
        """Message field 'header'."""
        return self._header

    @header.setter
    def header(self, value: std_msgs.msg.Header) -> None:
        from std_msgs.msg import Header

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Header), \
                    "The 'header' field must be a sub message of type 'Header'"

        self._header = value

    @builtins.property
    def current_ma_left(self) -> int:
        """Message field 'current_ma_left'."""
        return self._current_ma_left

    @current_ma_left.setter
    def current_ma_left(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'current_ma_left' field must be of type 'int'"
                assert value >= -32768 and value < 32768, \
                    "The 'current_ma_left' field must be an integer in [-32768, 32767]"

        self._current_ma_left = value

    @builtins.property
    def current_ma_right(self) -> int:
        """Message field 'current_ma_right'."""
        return self._current_ma_right

    @current_ma_right.setter
    def current_ma_right(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'current_ma_right' field must be of type 'int'"
                assert value >= -32768 and value < 32768, \
                    "The 'current_ma_right' field must be an integer in [-32768, 32767]"

        self._current_ma_right = value

    @builtins.property
    def pwm_left(self) -> int:
        """Message field 'pwm_left'."""
        return self._pwm_left

    @pwm_left.setter
    def pwm_left(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'pwm_left' field must be of type 'int'"
                assert value >= -32768 and value < 32768, \
                    "The 'pwm_left' field must be an integer in [-32768, 32767]"

        self._pwm_left = value

    @builtins.property
    def pwm_right(self) -> int:
        """Message field 'pwm_right'."""
        return self._pwm_right

    @pwm_right.setter
    def pwm_right(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'pwm_right' field must be of type 'int'"
                assert value >= -32768 and value < 32768, \
                    "The 'pwm_right' field must be an integer in [-32768, 32767]"

        self._pwm_right = value

    @builtins.property
    def wheels_enabled(self) -> bool:
        """Message field 'wheels_enabled'."""
        return self._wheels_enabled

    @wheels_enabled.setter
    def wheels_enabled(self, value: bool) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, bool), \
                    "The 'wheels_enabled' field must be of type 'bool'"

        self._wheels_enabled = value
