# generated from rosidl_generator_py/resource/_idl.py.em
# with input from irobot_create_msgs:msg/InterfaceButtons.idl
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
    import irobot_create_msgs.msg  # noqa: E402, I100, I201, I300
    import std_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_InterfaceButtons(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'InterfaceButtons'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class InterfaceButtonsConstants(typing.TypedDict):
        pass

    __constants: InterfaceButtonsConstants = {
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
                'irobot_create_msgs.msg.InterfaceButtons')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__interface_buttons
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__interface_buttons
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__interface_buttons
            cls._TYPE_SUPPORT = module.type_support_msg__msg__interface_buttons
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__interface_buttons

            from irobot_create_msgs.msg import Button
            if Button._TYPE_SUPPORT is None:
                Button.__import_type_support__()

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


class InterfaceButtons(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_InterfaceButtons):
    """Message class 'InterfaceButtons'."""

    __slots__ = [
        '_header',
        '_button_1',
        '_button_power',
        '_button_2',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'header': 'std_msgs/Header',
        'button_1': 'irobot_create_msgs/Button',
        'button_power': 'irobot_create_msgs/Button',
        'button_2': 'irobot_create_msgs/Button',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['std_msgs', 'msg'], 'Header'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'msg'], 'Button'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'msg'], 'Button'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'msg'], 'Button'),  # noqa: E501
    )

    def __init__(self, *,
                 header: typing.Optional[std_msgs.msg.Header] = None,  # noqa: E501
                 button_1: typing.Optional[irobot_create_msgs.msg.Button] = None,  # noqa: E501
                 button_power: typing.Optional[irobot_create_msgs.msg.Button] = None,  # noqa: E501
                 button_2: typing.Optional[irobot_create_msgs.msg.Button] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from std_msgs.msg import Header
        self.header = header if header is not None else Header()
        from irobot_create_msgs.msg import Button
        self.button_1 = button_1 if button_1 is not None else Button()
        from irobot_create_msgs.msg import Button
        self.button_power = button_power if button_power is not None else Button()
        from irobot_create_msgs.msg import Button
        self.button_2 = button_2 if button_2 is not None else Button()

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
        if not isinstance(other, InterfaceButtons):
            return False
        if self.header != other.header:
            return False
        if self.button_1 != other.button_1:
            return False
        if self.button_power != other.button_power:
            return False
        if self.button_2 != other.button_2:
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
    def button_1(self) -> irobot_create_msgs.msg.Button:
        """Message field 'button_1'."""
        return self._button_1

    @button_1.setter
    def button_1(self, value: irobot_create_msgs.msg.Button) -> None:
        from irobot_create_msgs.msg import Button

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Button), \
                    "The 'button_1' field must be a sub message of type 'Button'"

        self._button_1 = value

    @builtins.property
    def button_power(self) -> irobot_create_msgs.msg.Button:
        """Message field 'button_power'."""
        return self._button_power

    @button_power.setter
    def button_power(self, value: irobot_create_msgs.msg.Button) -> None:
        from irobot_create_msgs.msg import Button

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Button), \
                    "The 'button_power' field must be a sub message of type 'Button'"

        self._button_power = value

    @builtins.property
    def button_2(self) -> irobot_create_msgs.msg.Button:
        """Message field 'button_2'."""
        return self._button_2

    @button_2.setter
    def button_2(self, value: irobot_create_msgs.msg.Button) -> None:
        from irobot_create_msgs.msg import Button

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Button), \
                    "The 'button_2' field must be a sub message of type 'Button'"

        self._button_2 = value
