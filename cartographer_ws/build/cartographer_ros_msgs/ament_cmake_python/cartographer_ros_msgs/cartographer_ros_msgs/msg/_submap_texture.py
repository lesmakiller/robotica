# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/SubmapTexture.idl
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

# Member 'cells'
import array  # noqa: E402, I100

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_SubmapTexture(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'SubmapTexture'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class SubmapTextureConstants(typing.TypedDict):
        pass

    __constants: SubmapTextureConstants = {
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
                'cartographer_ros_msgs.msg.SubmapTexture')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__submap_texture
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__submap_texture
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__submap_texture
            cls._TYPE_SUPPORT = module.type_support_msg__msg__submap_texture
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__submap_texture

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


class SubmapTexture(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_SubmapTexture):
    """Message class 'SubmapTexture'."""

    __slots__ = [
        '_cells',
        '_width',
        '_height',
        '_resolution',
        '_slice_pose',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'cells': 'sequence<uint8>',
        'width': 'int32',
        'height': 'int32',
        'resolution': 'double',
        'slice_pose': 'geometry_msgs/Pose',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.BasicType('uint8')),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'Pose'),  # noqa: E501
    )

    def __init__(self, *,
                 cells: typing.Optional[collections.abc.Sequence[int]] = None,  # noqa: E501
                 width: typing.Optional[int] = None,  # noqa: E501
                 height: typing.Optional[int] = None,  # noqa: E501
                 resolution: typing.Optional[float] = None,  # noqa: E501
                 slice_pose: typing.Optional[geometry_msgs.msg.Pose] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.cells = cells if cells is not None else array.array('B', [])
        self.width = width if width is not None else int()
        self.height = height if height is not None else int()
        self.resolution = resolution if resolution is not None else float()
        from geometry_msgs.msg import Pose
        self.slice_pose = slice_pose if slice_pose is not None else Pose()

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
        if not isinstance(other, SubmapTexture):
            return False
        if self.cells != other.cells:
            return False
        if self.width != other.width:
            return False
        if self.height != other.height:
            return False
        if self.resolution != other.resolution:
            return False
        if self.slice_pose != other.slice_pose:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def cells(self) -> typing.Annotated[typing.Any, array.array[int]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'cells'."""
        return self._cells

    @cells.setter
    def cells(self, value: collections.abc.Sequence[int]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from rosidl_buffer import Buffer as _RosidlBuffer
        if isinstance(value, _RosidlBuffer):
            self._cells = value  # type: ignore[assignment]
            return

        if self._check_fields:
            if isinstance(value, array.array):
                assert value.typecode == 'B', \
                    "The 'cells' array.array() must have the type code of 'B'"
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     all(isinstance(v, int) for v in value) and
                     all(val >= 0 and val < 256 for val in value)), \
                    "The 'cells' field must be sequence and each value of type 'int' and each unsigned integer in [0, 255]"

        if isinstance(value, array.array):
            self._cells = value  # type: ignore[assignment]
            return
        # type ignore below fixed in mypy 1.17+ see mypy#19421
        self._cells = array.array('B', value)  # type: ignore[assignment]

    @builtins.property
    def width(self) -> int:
        """Message field 'width'."""
        return self._width

    @width.setter
    def width(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'width' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'width' field must be an integer in [-2147483648, 2147483647]"

        self._width = value

    @builtins.property
    def height(self) -> int:
        """Message field 'height'."""
        return self._height

    @height.setter
    def height(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'height' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'height' field must be an integer in [-2147483648, 2147483647]"

        self._height = value

    @builtins.property
    def resolution(self) -> float:
        """Message field 'resolution'."""
        return self._resolution

    @resolution.setter
    def resolution(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'resolution' field must be of type 'float'"
                assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                    "The 'resolution' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"

        self._resolution = value

    @builtins.property
    def slice_pose(self) -> geometry_msgs.msg.Pose:
        """Message field 'slice_pose'."""
        return self._slice_pose

    @slice_pose.setter
    def slice_pose(self, value: geometry_msgs.msg.Pose) -> None:
        from geometry_msgs.msg import Pose

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Pose), \
                    "The 'slice_pose' field must be a sub message of type 'Pose'"

        self._slice_pose = value
