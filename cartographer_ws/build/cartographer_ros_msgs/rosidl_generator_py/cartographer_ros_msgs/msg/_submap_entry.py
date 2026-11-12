# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/SubmapEntry.idl
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

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_SubmapEntry(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'SubmapEntry'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class SubmapEntryConstants(typing.TypedDict):
        pass

    __constants: SubmapEntryConstants = {
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
                'cartographer_ros_msgs.msg.SubmapEntry')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__submap_entry
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__submap_entry
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__submap_entry
            cls._TYPE_SUPPORT = module.type_support_msg__msg__submap_entry
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__submap_entry

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


class SubmapEntry(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_SubmapEntry):
    """Message class 'SubmapEntry'."""

    __slots__ = [
        '_trajectory_id',
        '_submap_index',
        '_submap_version',
        '_pose',
        '_is_frozen',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'trajectory_id': 'int32',
        'submap_index': 'int32',
        'submap_version': 'int32',
        'pose': 'geometry_msgs/Pose',
        'is_frozen': 'boolean',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'Pose'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, *,
                 trajectory_id: typing.Optional[int] = None,  # noqa: E501
                 submap_index: typing.Optional[int] = None,  # noqa: E501
                 submap_version: typing.Optional[int] = None,  # noqa: E501
                 pose: typing.Optional[geometry_msgs.msg.Pose] = None,  # noqa: E501
                 is_frozen: typing.Optional[bool] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.trajectory_id = trajectory_id if trajectory_id is not None else int()
        self.submap_index = submap_index if submap_index is not None else int()
        self.submap_version = submap_version if submap_version is not None else int()
        from geometry_msgs.msg import Pose
        self.pose = pose if pose is not None else Pose()
        self.is_frozen = is_frozen if is_frozen is not None else bool()

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
        if not isinstance(other, SubmapEntry):
            return False
        if self.trajectory_id != other.trajectory_id:
            return False
        if self.submap_index != other.submap_index:
            return False
        if self.submap_version != other.submap_version:
            return False
        if self.pose != other.pose:
            return False
        if self.is_frozen != other.is_frozen:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def trajectory_id(self) -> int:
        """Message field 'trajectory_id'."""
        return self._trajectory_id

    @trajectory_id.setter
    def trajectory_id(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'trajectory_id' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'trajectory_id' field must be an integer in [-2147483648, 2147483647]"

        self._trajectory_id = value

    @builtins.property
    def submap_index(self) -> int:
        """Message field 'submap_index'."""
        return self._submap_index

    @submap_index.setter
    def submap_index(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'submap_index' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'submap_index' field must be an integer in [-2147483648, 2147483647]"

        self._submap_index = value

    @builtins.property
    def submap_version(self) -> int:
        """Message field 'submap_version'."""
        return self._submap_version

    @submap_version.setter
    def submap_version(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'submap_version' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'submap_version' field must be an integer in [-2147483648, 2147483647]"

        self._submap_version = value

    @builtins.property
    def pose(self) -> geometry_msgs.msg.Pose:
        """Message field 'pose'."""
        return self._pose

    @pose.setter
    def pose(self, value: geometry_msgs.msg.Pose) -> None:
        from geometry_msgs.msg import Pose

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Pose), \
                    "The 'pose' field must be a sub message of type 'Pose'"

        self._pose = value

    @builtins.property
    def is_frozen(self) -> bool:
        """Message field 'is_frozen'."""
        return self._is_frozen

    @is_frozen.setter
    def is_frozen(self, value: bool) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, bool), \
                    "The 'is_frozen' field must be of type 'bool'"

        self._is_frozen = value
