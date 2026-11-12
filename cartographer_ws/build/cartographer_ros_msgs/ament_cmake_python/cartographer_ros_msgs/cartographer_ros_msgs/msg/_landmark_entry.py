# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/LandmarkEntry.idl
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


class Metaclass_LandmarkEntry(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'LandmarkEntry'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class LandmarkEntryConstants(typing.TypedDict):
        pass

    __constants: LandmarkEntryConstants = {
    }

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('cartographer_ros_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'cartographer_ros_msgs.msg.LandmarkEntry')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__landmark_entry
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__landmark_entry
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__landmark_entry
            cls._TYPE_SUPPORT = module.type_support_msg__msg__landmark_entry
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__landmark_entry

            from geometry_msgs.msg import Pose
            if Pose._TYPE_SUPPORT is None:
                Pose.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class LandmarkEntry(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_LandmarkEntry):
    """Message class 'LandmarkEntry'."""

    __slots__ = [
        '_id',
        '_tracking_from_landmark_transform',
        '_translation_weight',
        '_rotation_weight',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'id': 'string',
        'tracking_from_landmark_transform': 'geometry_msgs/Pose',
        'translation_weight': 'double',
        'rotation_weight': 'double',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'Pose'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
    )

    def __init__(self, *,
                 id: typing.Optional[str] = None,  # noqa: E501, A002
                 tracking_from_landmark_transform: typing.Optional[geometry_msgs.msg.Pose] = None,  # noqa: E501
                 translation_weight: typing.Optional[float] = None,  # noqa: E501
                 rotation_weight: typing.Optional[float] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.id = id if id is not None else str()
        from geometry_msgs.msg import Pose
        self.tracking_from_landmark_transform = tracking_from_landmark_transform if tracking_from_landmark_transform is not None else Pose()
        self.translation_weight = translation_weight if translation_weight is not None else float()
        self.rotation_weight = rotation_weight if rotation_weight is not None else float()

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
        if not isinstance(other, LandmarkEntry):
            return False
        if self.id != other.id:
            return False
        if self.tracking_from_landmark_transform != other.tracking_from_landmark_transform:
            return False
        if self.translation_weight != other.translation_weight:
            return False
        if self.rotation_weight != other.rotation_weight:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property  # noqa: A003
    def id(self) -> str:  # noqa: A003
        """Message field 'id'."""
        return self._id

    @id.setter  # noqa: A003
    def id(self, value: str) -> None:  # noqa: A003

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, str), \
                    "The 'id' field must be of type 'str'"

        self._id = value

    @builtins.property
    def tracking_from_landmark_transform(self) -> geometry_msgs.msg.Pose:
        """Message field 'tracking_from_landmark_transform'."""
        return self._tracking_from_landmark_transform

    @tracking_from_landmark_transform.setter
    def tracking_from_landmark_transform(self, value: geometry_msgs.msg.Pose) -> None:
        from geometry_msgs.msg import Pose

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Pose), \
                    "The 'tracking_from_landmark_transform' field must be a sub message of type 'Pose'"

        self._tracking_from_landmark_transform = value

    @builtins.property
    def translation_weight(self) -> float:
        """Message field 'translation_weight'."""
        return self._translation_weight

    @translation_weight.setter
    def translation_weight(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'translation_weight' field must be of type 'float'"
                assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                    "The 'translation_weight' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"

        self._translation_weight = value

    @builtins.property
    def rotation_weight(self) -> float:
        """Message field 'rotation_weight'."""
        return self._rotation_weight

    @rotation_weight.setter
    def rotation_weight(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'rotation_weight' field must be of type 'float'"
                assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                    "The 'rotation_weight' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"

        self._rotation_weight = value
