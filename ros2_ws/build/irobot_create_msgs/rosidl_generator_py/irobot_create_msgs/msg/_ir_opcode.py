# generated from rosidl_generator_py/resource/_idl.py.em
# with input from irobot_create_msgs:msg/IrOpcode.idl
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


class Metaclass_IrOpcode(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'IrOpcode'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class IrOpcodeConstants(typing.TypedDict):
        CODE_IR_FORCE_FIELD: typing.Literal[161]
        CODE_IR_VIRTUAL_WALL: typing.Literal[162]
        CODE_IR_BUOY_GREEN: typing.Literal[164]
        CODE_IR_BUOY_RED: typing.Literal[168]
        CODE_IR_BUOY_BOTH: typing.Literal[172]
        CODE_IR_EVAC_GREEN_FIELD: typing.Literal[244]
        CODE_IR_EVAC_RED_FIELD: typing.Literal[248]
        CODE_IR_EVAC_BOTH_FIELD: typing.Literal[252]
        SENSOR_OMNI: typing.Literal[0]
        SENSOR_DIRECTIONAL_FRONT: typing.Literal[1]

    __constants: IrOpcodeConstants = {
        'CODE_IR_FORCE_FIELD': 161,
        'CODE_IR_VIRTUAL_WALL': 162,
        'CODE_IR_BUOY_GREEN': 164,
        'CODE_IR_BUOY_RED': 168,
        'CODE_IR_BUOY_BOTH': 172,
        'CODE_IR_EVAC_GREEN_FIELD': 244,
        'CODE_IR_EVAC_RED_FIELD': 248,
        'CODE_IR_EVAC_BOTH_FIELD': 252,
        'SENSOR_OMNI': 0,
        'SENSOR_DIRECTIONAL_FRONT': 1,
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
                'irobot_create_msgs.msg.IrOpcode')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__ir_opcode
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__ir_opcode
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__ir_opcode
            cls._TYPE_SUPPORT = module.type_support_msg__msg__ir_opcode
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__ir_opcode

            from std_msgs.msg import Header
            if Header._TYPE_SUPPORT is None:
                Header.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'CODE_IR_FORCE_FIELD': metacls.__constants['CODE_IR_FORCE_FIELD'],
            'CODE_IR_VIRTUAL_WALL': metacls.__constants['CODE_IR_VIRTUAL_WALL'],
            'CODE_IR_BUOY_GREEN': metacls.__constants['CODE_IR_BUOY_GREEN'],
            'CODE_IR_BUOY_RED': metacls.__constants['CODE_IR_BUOY_RED'],
            'CODE_IR_BUOY_BOTH': metacls.__constants['CODE_IR_BUOY_BOTH'],
            'CODE_IR_EVAC_GREEN_FIELD': metacls.__constants['CODE_IR_EVAC_GREEN_FIELD'],
            'CODE_IR_EVAC_RED_FIELD': metacls.__constants['CODE_IR_EVAC_RED_FIELD'],
            'CODE_IR_EVAC_BOTH_FIELD': metacls.__constants['CODE_IR_EVAC_BOTH_FIELD'],
            'SENSOR_OMNI': metacls.__constants['SENSOR_OMNI'],
            'SENSOR_DIRECTIONAL_FRONT': metacls.__constants['SENSOR_DIRECTIONAL_FRONT'],
        }

    @property
    def CODE_IR_FORCE_FIELD(self) -> typing.Literal[161]:
        """Message constant 'CODE_IR_FORCE_FIELD'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_FORCE_FIELD']

    @property
    def CODE_IR_VIRTUAL_WALL(self) -> typing.Literal[162]:
        """Message constant 'CODE_IR_VIRTUAL_WALL'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_VIRTUAL_WALL']

    @property
    def CODE_IR_BUOY_GREEN(self) -> typing.Literal[164]:
        """Message constant 'CODE_IR_BUOY_GREEN'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_BUOY_GREEN']

    @property
    def CODE_IR_BUOY_RED(self) -> typing.Literal[168]:
        """Message constant 'CODE_IR_BUOY_RED'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_BUOY_RED']

    @property
    def CODE_IR_BUOY_BOTH(self) -> typing.Literal[172]:
        """Message constant 'CODE_IR_BUOY_BOTH'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_BUOY_BOTH']

    @property
    def CODE_IR_EVAC_GREEN_FIELD(self) -> typing.Literal[244]:
        """Message constant 'CODE_IR_EVAC_GREEN_FIELD'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_EVAC_GREEN_FIELD']

    @property
    def CODE_IR_EVAC_RED_FIELD(self) -> typing.Literal[248]:
        """Message constant 'CODE_IR_EVAC_RED_FIELD'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_EVAC_RED_FIELD']

    @property
    def CODE_IR_EVAC_BOTH_FIELD(self) -> typing.Literal[252]:
        """Message constant 'CODE_IR_EVAC_BOTH_FIELD'."""
        return Metaclass_IrOpcode.__constants['CODE_IR_EVAC_BOTH_FIELD']

    @property
    def SENSOR_OMNI(self) -> typing.Literal[0]:
        """Message constant 'SENSOR_OMNI'."""
        return Metaclass_IrOpcode.__constants['SENSOR_OMNI']

    @property
    def SENSOR_DIRECTIONAL_FRONT(self) -> typing.Literal[1]:
        """Message constant 'SENSOR_DIRECTIONAL_FRONT'."""
        return Metaclass_IrOpcode.__constants['SENSOR_DIRECTIONAL_FRONT']


class IrOpcode(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_IrOpcode):
    """
    Message class 'IrOpcode'.

    Constants:
      CODE_IR_FORCE_FIELD
      CODE_IR_VIRTUAL_WALL
      CODE_IR_BUOY_GREEN
      CODE_IR_BUOY_RED
      CODE_IR_BUOY_BOTH
      CODE_IR_EVAC_GREEN_FIELD
      CODE_IR_EVAC_RED_FIELD
      CODE_IR_EVAC_BOTH_FIELD
      SENSOR_OMNI
      SENSOR_DIRECTIONAL_FRONT
    """

    __slots__ = [
        '_header',
        '_opcode',
        '_sensor',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'header': 'std_msgs/Header',
        'opcode': 'uint8',
        'sensor': 'uint8',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['std_msgs', 'msg'], 'Header'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, *,
                 header: typing.Optional[std_msgs.msg.Header] = None,  # noqa: E501
                 opcode: typing.Optional[int] = None,  # noqa: E501
                 sensor: typing.Optional[int] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from std_msgs.msg import Header
        self.header = header if header is not None else Header()
        self.opcode = opcode if opcode is not None else int()
        self.sensor = sensor if sensor is not None else int()

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
        if not isinstance(other, IrOpcode):
            return False
        if self.header != other.header:
            return False
        if self.opcode != other.opcode:
            return False
        if self.sensor != other.sensor:
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
    def opcode(self) -> int:
        """Message field 'opcode'."""
        return self._opcode

    @opcode.setter
    def opcode(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'opcode' field must be of type 'int'"
                assert value >= 0 and value < 256, \
                    "The 'opcode' field must be an unsigned integer in [0, 255]"

        self._opcode = value

    @builtins.property
    def sensor(self) -> int:
        """Message field 'sensor'."""
        return self._sensor

    @sensor.setter
    def sensor(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'sensor' field must be of type 'int'"
                assert value >= 0 and value < 256, \
                    "The 'sensor' field must be an unsigned integer in [0, 255]"

        self._sensor = value
