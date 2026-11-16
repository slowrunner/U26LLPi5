# generated from rosidl_generator_py/resource/_idl.py.em
# with input from irobot_create_msgs:action/NavigateToPosition.idl
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
    import geometry_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_NavigateToPosition_Goal(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_Goal'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_GoalConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_GoalConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__goal

            from geometry_msgs.msg import PoseStamped
            if PoseStamped._TYPE_SUPPORT is None:
                PoseStamped.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'MAX_TRANSLATION_SPEED__DEFAULT': 0.3,
            'MAX_ROTATION_SPEED__DEFAULT': 1.9,
        }

    @property
    def MAX_TRANSLATION_SPEED__DEFAULT(cls) -> float:
        """Return default value for message field 'max_translation_speed'."""
        return 0.3

    @property
    def MAX_ROTATION_SPEED__DEFAULT(cls) -> float:
        """Return default value for message field 'max_rotation_speed'."""
        return 1.9


class NavigateToPosition_Goal(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_Goal):
    """Message class 'NavigateToPosition_Goal'."""

    __slots__ = [
        '_goal_pose',
        '_achieve_goal_heading',
        '_max_translation_speed',
        '_max_rotation_speed',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'goal_pose': 'geometry_msgs/PoseStamped',
        'achieve_goal_heading': 'boolean',
        'max_translation_speed': 'float',
        'max_rotation_speed': 'float',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'PoseStamped'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
    )

    def __init__(self, *,
                 goal_pose: typing.Optional[geometry_msgs.msg.PoseStamped] = None,  # noqa: E501
                 achieve_goal_heading: typing.Optional[bool] = None,  # noqa: E501
                 max_translation_speed: typing.Optional[float] = None,  # noqa: E501
                 max_rotation_speed: typing.Optional[float] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from geometry_msgs.msg import PoseStamped
        self.goal_pose = goal_pose if goal_pose is not None else PoseStamped()
        self.achieve_goal_heading = achieve_goal_heading if achieve_goal_heading is not None else bool()
        self.max_translation_speed = max_translation_speed if max_translation_speed is not None else NavigateToPosition_Goal.MAX_TRANSLATION_SPEED__DEFAULT
        self.max_rotation_speed = max_rotation_speed if max_rotation_speed is not None else NavigateToPosition_Goal.MAX_ROTATION_SPEED__DEFAULT

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
        if not isinstance(other, NavigateToPosition_Goal):
            return False
        if self.goal_pose != other.goal_pose:
            return False
        if self.achieve_goal_heading != other.achieve_goal_heading:
            return False
        if self.max_translation_speed != other.max_translation_speed:
            return False
        if self.max_rotation_speed != other.max_rotation_speed:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_pose(self) -> geometry_msgs.msg.PoseStamped:
        """Message field 'goal_pose'."""
        return self._goal_pose

    @goal_pose.setter
    def goal_pose(self, value: geometry_msgs.msg.PoseStamped) -> None:
        from geometry_msgs.msg import PoseStamped

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, PoseStamped), \
                    "The 'goal_pose' field must be a sub message of type 'PoseStamped'"

        self._goal_pose = value

    @builtins.property
    def achieve_goal_heading(self) -> bool:
        """Message field 'achieve_goal_heading'."""
        return self._achieve_goal_heading

    @achieve_goal_heading.setter
    def achieve_goal_heading(self, value: bool) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, bool), \
                    "The 'achieve_goal_heading' field must be of type 'bool'"

        self._achieve_goal_heading = value

    @builtins.property
    def max_translation_speed(self) -> float:
        """Message field 'max_translation_speed'."""
        return self._max_translation_speed

    @max_translation_speed.setter
    def max_translation_speed(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'max_translation_speed' field must be of type 'float'"
                assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                    "The 'max_translation_speed' field must be a float in [-3.402823466e+38, 3.402823466e+38]"

        self._max_translation_speed = value

    @builtins.property
    def max_rotation_speed(self) -> float:
        """Message field 'max_rotation_speed'."""
        return self._max_rotation_speed

    @max_rotation_speed.setter
    def max_rotation_speed(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'max_rotation_speed' field must be of type 'float'"
                assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                    "The 'max_rotation_speed' field must be a float in [-3.402823466e+38, 3.402823466e+38]"

        self._max_rotation_speed = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_Result(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_Result'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_ResultConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_ResultConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__result

            from geometry_msgs.msg import PoseStamped
            if PoseStamped._TYPE_SUPPORT is None:
                PoseStamped.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_Result(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_Result):
    """Message class 'NavigateToPosition_Result'."""

    __slots__ = [
        '_pose',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'pose': 'geometry_msgs/PoseStamped',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'PoseStamped'),  # noqa: E501
    )

    def __init__(self, *,
                 pose: typing.Optional[geometry_msgs.msg.PoseStamped] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from geometry_msgs.msg import PoseStamped
        self.pose = pose if pose is not None else PoseStamped()

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
        if not isinstance(other, NavigateToPosition_Result):
            return False
        if self.pose != other.pose:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def pose(self) -> geometry_msgs.msg.PoseStamped:
        """Message field 'pose'."""
        return self._pose

    @pose.setter
    def pose(self, value: geometry_msgs.msg.PoseStamped) -> None:
        from geometry_msgs.msg import PoseStamped

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, PoseStamped), \
                    "The 'pose' field must be a sub message of type 'PoseStamped'"

        self._pose = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import math

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_Feedback(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_Feedback'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_FeedbackConstants(typing.TypedDict):
        ROTATING_TO_GOAL_POSITION: typing.Literal[1]
        DRIVING_TO_GOAL_POSITION: typing.Literal[2]
        ROTATING_TO_GOAL_ORIENTATION: typing.Literal[3]

    __constants: NavigateToPosition_FeedbackConstants = {
        'ROTATING_TO_GOAL_POSITION': 1,
        'DRIVING_TO_GOAL_POSITION': 2,
        'ROTATING_TO_GOAL_ORIENTATION': 3,
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
                'irobot_create_msgs.action.NavigateToPosition_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__feedback

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'ROTATING_TO_GOAL_POSITION': metacls.__constants['ROTATING_TO_GOAL_POSITION'],
            'DRIVING_TO_GOAL_POSITION': metacls.__constants['DRIVING_TO_GOAL_POSITION'],
            'ROTATING_TO_GOAL_ORIENTATION': metacls.__constants['ROTATING_TO_GOAL_ORIENTATION'],
        }

    @property
    def ROTATING_TO_GOAL_POSITION(self) -> typing.Literal[1]:
        """Message constant 'ROTATING_TO_GOAL_POSITION'."""
        return Metaclass_NavigateToPosition_Feedback.__constants['ROTATING_TO_GOAL_POSITION']

    @property
    def DRIVING_TO_GOAL_POSITION(self) -> typing.Literal[2]:
        """Message constant 'DRIVING_TO_GOAL_POSITION'."""
        return Metaclass_NavigateToPosition_Feedback.__constants['DRIVING_TO_GOAL_POSITION']

    @property
    def ROTATING_TO_GOAL_ORIENTATION(self) -> typing.Literal[3]:
        """Message constant 'ROTATING_TO_GOAL_ORIENTATION'."""
        return Metaclass_NavigateToPosition_Feedback.__constants['ROTATING_TO_GOAL_ORIENTATION']


class NavigateToPosition_Feedback(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_Feedback):
    """
    Message class 'NavigateToPosition_Feedback'.

    Constants:
      ROTATING_TO_GOAL_POSITION
      DRIVING_TO_GOAL_POSITION
      ROTATING_TO_GOAL_ORIENTATION
    """

    __slots__ = [
        '_navigate_state',
        '_remaining_angle_travel',
        '_remaining_travel_distance',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'navigate_state': 'int8',
        'remaining_angle_travel': 'float',
        'remaining_travel_distance': 'float',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
    )

    def __init__(self, *,
                 navigate_state: typing.Optional[int] = None,  # noqa: E501
                 remaining_angle_travel: typing.Optional[float] = None,  # noqa: E501
                 remaining_travel_distance: typing.Optional[float] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.navigate_state = navigate_state if navigate_state is not None else int()
        self.remaining_angle_travel = remaining_angle_travel if remaining_angle_travel is not None else float()
        self.remaining_travel_distance = remaining_travel_distance if remaining_travel_distance is not None else float()

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
        if not isinstance(other, NavigateToPosition_Feedback):
            return False
        if self.navigate_state != other.navigate_state:
            return False
        if self.remaining_angle_travel != other.remaining_angle_travel:
            return False
        if self.remaining_travel_distance != other.remaining_travel_distance:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def navigate_state(self) -> int:
        """Message field 'navigate_state'."""
        return self._navigate_state

    @navigate_state.setter
    def navigate_state(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'navigate_state' field must be of type 'int'"
                assert value >= -128 and value < 128, \
                    "The 'navigate_state' field must be an integer in [-128, 127]"

        self._navigate_state = value

    @builtins.property
    def remaining_angle_travel(self) -> float:
        """Message field 'remaining_angle_travel'."""
        return self._remaining_angle_travel

    @remaining_angle_travel.setter
    def remaining_angle_travel(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'remaining_angle_travel' field must be of type 'float'"
                assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                    "The 'remaining_angle_travel' field must be a float in [-3.402823466e+38, 3.402823466e+38]"

        self._remaining_angle_travel = value

    @builtins.property
    def remaining_travel_distance(self) -> float:
        """Message field 'remaining_travel_distance'."""
        return self._remaining_travel_distance

    @remaining_travel_distance.setter
    def remaining_travel_distance(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'remaining_travel_distance' field must be of type 'float'"
                assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                    "The 'remaining_travel_distance' field must be a float in [-3.402823466e+38, 3.402823466e+38]"

        self._remaining_travel_distance = value


if typing.TYPE_CHECKING:
    import irobot_create_msgs.action._navigate_to_position  # noqa: E402, I100, I201, I300
    import unique_identifier_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_SendGoal_Request(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_SendGoal_Request'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_SendGoal_RequestConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_SendGoal_RequestConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__send_goal__request

            from irobot_create_msgs.action import NavigateToPosition
            if NavigateToPosition.Goal._TYPE_SUPPORT is None:
                NavigateToPosition.Goal.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID._TYPE_SUPPORT is None:
                UUID.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_SendGoal_Request(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_SendGoal_Request):
    """Message class 'NavigateToPosition_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'irobot_create_msgs/NavigateToPosition_Goal',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_Goal'),  # noqa: E501
    )

    def __init__(self, *,
                 goal_id: typing.Optional[unique_identifier_msgs.msg.UUID] = None,  # noqa: E501
                 goal: typing.Optional[irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Goal] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from unique_identifier_msgs.msg import UUID
        self.goal_id = goal_id if goal_id is not None else UUID()
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Goal
        self.goal = goal if goal is not None else NavigateToPosition_Goal()

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
        if not isinstance(other, NavigateToPosition_SendGoal_Request):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self) -> unique_identifier_msgs.msg.UUID:
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value: unique_identifier_msgs.msg.UUID) -> None:
        from unique_identifier_msgs.msg import UUID

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, UUID), \
                    "The 'goal_id' field must be a sub message of type 'UUID'"

        self._goal_id = value

    @builtins.property
    def goal(self) -> irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Goal:
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value: irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Goal) -> None:
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Goal

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, NavigateToPosition_Goal), \
                    "The 'goal' field must be a sub message of type 'NavigateToPosition_Goal'"

        self._goal = value


if typing.TYPE_CHECKING:
    import builtin_interfaces.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_SendGoal_Response(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_SendGoal_Response'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_SendGoal_ResponseConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_SendGoal_ResponseConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__send_goal__response

            from builtin_interfaces.msg import Time
            if Time._TYPE_SUPPORT is None:
                Time.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_SendGoal_Response(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_SendGoal_Response):
    """Message class 'NavigateToPosition_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, *,
                 accepted: typing.Optional[bool] = None,  # noqa: E501
                 stamp: typing.Optional[builtin_interfaces.msg.Time] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.accepted = accepted if accepted is not None else bool()
        from builtin_interfaces.msg import Time
        self.stamp = stamp if stamp is not None else Time()

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
        if not isinstance(other, NavigateToPosition_SendGoal_Response):
            return False
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self) -> bool:
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value: bool) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, bool), \
                    "The 'accepted' field must be of type 'bool'"

        self._accepted = value

    @builtins.property
    def stamp(self) -> builtin_interfaces.msg.Time:
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value: builtin_interfaces.msg.Time) -> None:
        from builtin_interfaces.msg import Time

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Time), \
                    "The 'stamp' field must be a sub message of type 'Time'"

        self._stamp = value


if typing.TYPE_CHECKING:
    import irobot_create_msgs.action  # noqa: E402, I100, I201, I300
    import service_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_SendGoal_Event(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_SendGoal_Event'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_SendGoal_EventConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_SendGoal_EventConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_SendGoal_Event')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__send_goal__event
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__send_goal__event
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__send_goal__event
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__send_goal__event
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__send_goal__event

            from service_msgs.msg import ServiceEventInfo
            if ServiceEventInfo._TYPE_SUPPORT is None:
                ServiceEventInfo.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_SendGoal_Event(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_SendGoal_Event):
    """Message class 'NavigateToPosition_SendGoal_Event'."""

    __slots__ = [
        '_info',
        '_request',
        '_response',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'info': 'service_msgs/ServiceEventInfo',
        'request': 'sequence<irobot_create_msgs/NavigateToPosition_SendGoal_Request, 1>',
        'response': 'sequence<irobot_create_msgs/NavigateToPosition_SendGoal_Response, 1>',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['service_msgs', 'msg'], 'ServiceEventInfo'),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_SendGoal_Request'), 1),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_SendGoal_Response'), 1),  # noqa: E501
    )

    def __init__(self, *,
                 info: typing.Optional[service_msgs.msg.ServiceEventInfo] = None,  # noqa: E501
                 request: typing.Optional[collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_SendGoal_Request]] = None,  # noqa: E501
                 response: typing.Optional[collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_SendGoal_Response]] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from service_msgs.msg import ServiceEventInfo
        self.info = info if info is not None else ServiceEventInfo()
        self.request = request if request is not None else []
        self.response = response if response is not None else []

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
        if not isinstance(other, NavigateToPosition_SendGoal_Event):
            return False
        if self.info != other.info:
            return False
        if self.request != other.request:
            return False
        if self.response != other.response:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def info(self) -> service_msgs.msg.ServiceEventInfo:
        """Message field 'info'."""
        return self._info

    @info.setter
    def info(self, value: service_msgs.msg.ServiceEventInfo) -> None:
        from service_msgs.msg import ServiceEventInfo

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, ServiceEventInfo), \
                    "The 'info' field must be a sub message of type 'ServiceEventInfo'"

        self._info = value

    @builtins.property
    def request(self) -> typing.Annotated[typing.Any, list[irobot_create_msgs.action.NavigateToPosition_SendGoal_Request]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'request'."""
        return self._request

    @request.setter
    def request(self, value: collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_SendGoal_Request]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from irobot_create_msgs.action import NavigateToPosition_SendGoal_Request

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     len(value) <= 1 and
                     all(isinstance(v, NavigateToPosition_SendGoal_Request) for v in value) and
                     True), \
                    "The 'request' field must be sequence with length <= 1 and each value of type 'NavigateToPosition_SendGoal_Request'"

        if isinstance(value, list):
            self._request = value
            return
        self._request = list(value)

    @builtins.property
    def response(self) -> typing.Annotated[typing.Any, list[irobot_create_msgs.action.NavigateToPosition_SendGoal_Response]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'response'."""
        return self._response

    @response.setter
    def response(self, value: collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_SendGoal_Response]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from irobot_create_msgs.action import NavigateToPosition_SendGoal_Response

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     len(value) <= 1 and
                     all(isinstance(v, NavigateToPosition_SendGoal_Response) for v in value) and
                     True), \
                    "The 'response' field must be sequence with length <= 1 and each value of type 'NavigateToPosition_SendGoal_Response'"

        if isinstance(value, list):
            self._response = value
            return
        self._response = list(value)


if typing.TYPE_CHECKING:
    from typing_extensions import TypeAlias  # noqa: I100, I300


class Metaclass_NavigateToPosition_SendGoal(rosidl_pycommon.interface_base_classes.ServiceTypeSupportMeta):
    """Metaclass of service 'NavigateToPosition_SendGoal'."""

    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('irobot_create_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'irobot_create_msgs.action.NavigateToPosition_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__navigate_to_position__send_goal

            from irobot_create_msgs.action import _navigate_to_position
            if _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Request._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Request.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Response._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Response.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Event._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_SendGoal_Event.__import_type_support__()


class NavigateToPosition_SendGoal(rosidl_pycommon.interface_base_classes.BaseService[
    NavigateToPosition_SendGoal_Request,
    NavigateToPosition_SendGoal_Response
], metaclass=Metaclass_NavigateToPosition_SendGoal):
    Request: TypeAlias = NavigateToPosition_SendGoal_Request
    Response: TypeAlias = NavigateToPosition_SendGoal_Response
    Event: TypeAlias = NavigateToPosition_SendGoal_Event

    # Should eventually be typing.NoReturn. See mypy#14044
    def __init__(self) -> None:
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_GetResult_Request(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_GetResult_Request'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_GetResult_RequestConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_GetResult_RequestConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID._TYPE_SUPPORT is None:
                UUID.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_GetResult_Request(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_GetResult_Request):
    """Message class 'NavigateToPosition_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, *,
                 goal_id: typing.Optional[unique_identifier_msgs.msg.UUID] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from unique_identifier_msgs.msg import UUID
        self.goal_id = goal_id if goal_id is not None else UUID()

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
        if not isinstance(other, NavigateToPosition_GetResult_Request):
            return False
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self) -> unique_identifier_msgs.msg.UUID:
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value: unique_identifier_msgs.msg.UUID) -> None:
        from unique_identifier_msgs.msg import UUID

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, UUID), \
                    "The 'goal_id' field must be a sub message of type 'UUID'"

        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_GetResult_Response(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_GetResult_Response'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_GetResult_ResponseConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_GetResult_ResponseConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__get_result__response

            from irobot_create_msgs.action import NavigateToPosition
            if NavigateToPosition.Result._TYPE_SUPPORT is None:
                NavigateToPosition.Result.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_GetResult_Response(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_GetResult_Response):
    """Message class 'NavigateToPosition_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'status': 'int8',
        'result': 'irobot_create_msgs/NavigateToPosition_Result',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_Result'),  # noqa: E501
    )

    def __init__(self, *,
                 status: typing.Optional[int] = None,  # noqa: E501
                 result: typing.Optional[irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Result] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.status = status if status is not None else int()
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Result
        self.result = result if result is not None else NavigateToPosition_Result()

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
        if not isinstance(other, NavigateToPosition_GetResult_Response):
            return False
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self) -> int:
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'status' field must be of type 'int'"
                assert value >= -128 and value < 128, \
                    "The 'status' field must be an integer in [-128, 127]"

        self._status = value

    @builtins.property
    def result(self) -> irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Result:
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value: irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Result) -> None:
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Result

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, NavigateToPosition_Result), \
                    "The 'result' field must be a sub message of type 'NavigateToPosition_Result'"

        self._result = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_GetResult_Event(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_GetResult_Event'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_GetResult_EventConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_GetResult_EventConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_GetResult_Event')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__get_result__event
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__get_result__event
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__get_result__event
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__get_result__event
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__get_result__event

            from service_msgs.msg import ServiceEventInfo
            if ServiceEventInfo._TYPE_SUPPORT is None:
                ServiceEventInfo.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_GetResult_Event(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_GetResult_Event):
    """Message class 'NavigateToPosition_GetResult_Event'."""

    __slots__ = [
        '_info',
        '_request',
        '_response',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'info': 'service_msgs/ServiceEventInfo',
        'request': 'sequence<irobot_create_msgs/NavigateToPosition_GetResult_Request, 1>',
        'response': 'sequence<irobot_create_msgs/NavigateToPosition_GetResult_Response, 1>',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['service_msgs', 'msg'], 'ServiceEventInfo'),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_GetResult_Request'), 1),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_GetResult_Response'), 1),  # noqa: E501
    )

    def __init__(self, *,
                 info: typing.Optional[service_msgs.msg.ServiceEventInfo] = None,  # noqa: E501
                 request: typing.Optional[collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_GetResult_Request]] = None,  # noqa: E501
                 response: typing.Optional[collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_GetResult_Response]] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from service_msgs.msg import ServiceEventInfo
        self.info = info if info is not None else ServiceEventInfo()
        self.request = request if request is not None else []
        self.response = response if response is not None else []

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
        if not isinstance(other, NavigateToPosition_GetResult_Event):
            return False
        if self.info != other.info:
            return False
        if self.request != other.request:
            return False
        if self.response != other.response:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def info(self) -> service_msgs.msg.ServiceEventInfo:
        """Message field 'info'."""
        return self._info

    @info.setter
    def info(self, value: service_msgs.msg.ServiceEventInfo) -> None:
        from service_msgs.msg import ServiceEventInfo

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, ServiceEventInfo), \
                    "The 'info' field must be a sub message of type 'ServiceEventInfo'"

        self._info = value

    @builtins.property
    def request(self) -> typing.Annotated[typing.Any, list[irobot_create_msgs.action.NavigateToPosition_GetResult_Request]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'request'."""
        return self._request

    @request.setter
    def request(self, value: collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_GetResult_Request]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from irobot_create_msgs.action import NavigateToPosition_GetResult_Request

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     len(value) <= 1 and
                     all(isinstance(v, NavigateToPosition_GetResult_Request) for v in value) and
                     True), \
                    "The 'request' field must be sequence with length <= 1 and each value of type 'NavigateToPosition_GetResult_Request'"

        if isinstance(value, list):
            self._request = value
            return
        self._request = list(value)

    @builtins.property
    def response(self) -> typing.Annotated[typing.Any, list[irobot_create_msgs.action.NavigateToPosition_GetResult_Response]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'response'."""
        return self._response

    @response.setter
    def response(self, value: collections.abc.Sequence[irobot_create_msgs.action.NavigateToPosition_GetResult_Response]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from irobot_create_msgs.action import NavigateToPosition_GetResult_Response

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     len(value) <= 1 and
                     all(isinstance(v, NavigateToPosition_GetResult_Response) for v in value) and
                     True), \
                    "The 'response' field must be sequence with length <= 1 and each value of type 'NavigateToPosition_GetResult_Response'"

        if isinstance(value, list):
            self._response = value
            return
        self._response = list(value)


class Metaclass_NavigateToPosition_GetResult(rosidl_pycommon.interface_base_classes.ServiceTypeSupportMeta):
    """Metaclass of service 'NavigateToPosition_GetResult'."""

    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('irobot_create_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'irobot_create_msgs.action.NavigateToPosition_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__navigate_to_position__get_result

            from irobot_create_msgs.action import _navigate_to_position
            if _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Request._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Request.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Response._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Response.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Event._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_GetResult_Event.__import_type_support__()


class NavigateToPosition_GetResult(rosidl_pycommon.interface_base_classes.BaseService[
    NavigateToPosition_GetResult_Request,
    NavigateToPosition_GetResult_Response
], metaclass=Metaclass_NavigateToPosition_GetResult):
    Request: TypeAlias = NavigateToPosition_GetResult_Request
    Response: TypeAlias = NavigateToPosition_GetResult_Response
    Event: TypeAlias = NavigateToPosition_GetResult_Event

    # Should eventually be typing.NoReturn. See mypy#14044
    def __init__(self) -> None:
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_NavigateToPosition_FeedbackMessage(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'NavigateToPosition_FeedbackMessage'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class NavigateToPosition_FeedbackMessageConstants(typing.TypedDict):
        pass

    __constants: NavigateToPosition_FeedbackMessageConstants = {
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
                'irobot_create_msgs.action.NavigateToPosition_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__navigate_to_position__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__navigate_to_position__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__navigate_to_position__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__navigate_to_position__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__navigate_to_position__feedback_message

            from irobot_create_msgs.action import NavigateToPosition
            if NavigateToPosition.Feedback._TYPE_SUPPORT is None:
                NavigateToPosition.Feedback.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID._TYPE_SUPPORT is None:
                UUID.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class NavigateToPosition_FeedbackMessage(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_NavigateToPosition_FeedbackMessage):
    """Message class 'NavigateToPosition_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'irobot_create_msgs/NavigateToPosition_Feedback',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['irobot_create_msgs', 'action'], 'NavigateToPosition_Feedback'),  # noqa: E501
    )

    def __init__(self, *,
                 goal_id: typing.Optional[unique_identifier_msgs.msg.UUID] = None,  # noqa: E501
                 feedback: typing.Optional[irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Feedback] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from unique_identifier_msgs.msg import UUID
        self.goal_id = goal_id if goal_id is not None else UUID()
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Feedback
        self.feedback = feedback if feedback is not None else NavigateToPosition_Feedback()

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
        if not isinstance(other, NavigateToPosition_FeedbackMessage):
            return False
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self) -> unique_identifier_msgs.msg.UUID:
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value: unique_identifier_msgs.msg.UUID) -> None:
        from unique_identifier_msgs.msg import UUID

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, UUID), \
                    "The 'goal_id' field must be a sub message of type 'UUID'"

        self._goal_id = value

    @builtins.property
    def feedback(self) -> irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Feedback:
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value: irobot_create_msgs.action._navigate_to_position.NavigateToPosition_Feedback) -> None:
        from irobot_create_msgs.action._navigate_to_position import NavigateToPosition_Feedback

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, NavigateToPosition_Feedback), \
                    "The 'feedback' field must be a sub message of type 'NavigateToPosition_Feedback'"

        self._feedback = value


class Metaclass_NavigateToPosition(rosidl_pycommon.interface_base_classes.ActionTypeSupportMeta):
    """Metaclass of action 'NavigateToPosition'."""

    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('irobot_create_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'irobot_create_msgs.action.NavigateToPosition')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__navigate_to_position

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from irobot_create_msgs.action import _navigate_to_position
            if _navigate_to_position.Metaclass_NavigateToPosition_SendGoal._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_SendGoal.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_GetResult._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_GetResult.__import_type_support__()
            if _navigate_to_position.Metaclass_NavigateToPosition_FeedbackMessage._TYPE_SUPPORT is None:
                _navigate_to_position.Metaclass_NavigateToPosition_FeedbackMessage.__import_type_support__()


class _NavigateToPosition_Impl(rosidl_pycommon.interface_base_classes.BaseImpl[
        NavigateToPosition_SendGoal,
        NavigateToPosition_GetResult,
        NavigateToPosition_FeedbackMessage
]):

    # The send_goal service using a wrapped version of the goal message as a request.
    SendGoalService: TypeAlias = NavigateToPosition_SendGoal
    # The get_result service using a wrapped version of the result message as a response.
    GetResultService: TypeAlias = NavigateToPosition_GetResult
    # The feedback message with generic fields which wraps the feedback message.
    FeedbackMessage: TypeAlias = NavigateToPosition_FeedbackMessage

    # The generic service to cancel a goal.
    from action_msgs.srv._cancel_goal import CancelGoal
    CancelGoalService: TypeAlias = CancelGoal
    # The generic message for get the status of a goal.
    from action_msgs.msg._goal_status_array import GoalStatusArray
    GoalStatusMessage: TypeAlias = GoalStatusArray


class NavigateToPosition(rosidl_pycommon.interface_base_classes.BaseAction[
    NavigateToPosition_Goal,
    NavigateToPosition_Result,
    NavigateToPosition_Feedback,
    _NavigateToPosition_Impl
], metaclass=Metaclass_NavigateToPosition):

    # The goal message defined in the action definition.
    Goal: TypeAlias = NavigateToPosition_Goal
    # The result message defined in the action definition.
    Result: TypeAlias = NavigateToPosition_Result
    # The feedback message defined in the action definition.
    Feedback: TypeAlias = NavigateToPosition_Feedback

    Impl: TypeAlias = _NavigateToPosition_Impl

    # Should eventually be typing.NoReturn. See mypy#14044
    def __init__(self) -> None:
        raise NotImplementedError('Action classes can not be instantiated')
