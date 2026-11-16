# generated from rosidl_generator_py/resource/_idl.py.em
# with input from turtlebot4_msgs:msg/UserDisplay.idl
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

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_UserDisplay(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'UserDisplay'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class UserDisplayConstants(typing.TypedDict):
        pass

    __constants: UserDisplayConstants = {
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
                'turtlebot4_msgs.msg.UserDisplay')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__user_display
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__user_display
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__user_display
            cls._TYPE_SUPPORT = module.type_support_msg__msg__user_display
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__user_display

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class UserDisplay(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_UserDisplay):
    """Message class 'UserDisplay'."""

    __slots__ = [
        '_ip',
        '_battery',
        '_entries',
        '_selected_entry',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'ip': 'string',
        'battery': 'string',
        'entries': 'string[5]',
        'selected_entry': 'int32',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.Array(rosidl_parser.definition.UnboundedString(), 5),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, *,
                 ip: typing.Optional[str] = None,  # noqa: E501
                 battery: typing.Optional[str] = None,  # noqa: E501
                 entries: typing.Optional[collections.abc.Sequence[str]] = None,  # noqa: E501
                 selected_entry: typing.Optional[int] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.ip = ip if ip is not None else str()
        self.battery = battery if battery is not None else str()
        self.entries = entries if entries is not None else [str() for x in range(5)]
        self.selected_entry = selected_entry if selected_entry is not None else int()

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
        if not isinstance(other, UserDisplay):
            return False
        if self.ip != other.ip:
            return False
        if self.battery != other.battery:
            return False
        if self.entries != other.entries:
            return False
        if self.selected_entry != other.selected_entry:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def ip(self) -> str:
        """Message field 'ip'."""
        return self._ip

    @ip.setter
    def ip(self, value: str) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, str), \
                    "The 'ip' field must be of type 'str'"

        self._ip = value

    @builtins.property
    def battery(self) -> str:
        """Message field 'battery'."""
        return self._battery

    @battery.setter
    def battery(self, value: str) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, str), \
                    "The 'battery' field must be of type 'str'"

        self._battery = value

    @builtins.property
    def entries(self) -> typing.Annotated[typing.Any, list[str]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'entries'."""
        return self._entries

    @entries.setter
    def entries(self, value: collections.abc.Sequence[str]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     len(value) == 5 and
                     all(isinstance(v, str) for v in value) and
                     True), \
                    "The 'entries' field must be sequence with length 5 and each value of type 'str'"

        if isinstance(value, list):
            self._entries = value
            return
        self._entries = list(value)

    @builtins.property
    def selected_entry(self) -> int:
        """Message field 'selected_entry'."""
        return self._selected_entry

    @selected_entry.setter
    def selected_entry(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'selected_entry' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'selected_entry' field must be an integer in [-2147483648, 2147483647]"

        self._selected_entry = value
