# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/TrajectoryStates.idl
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

# Member 'trajectory_id'
# Member 'trajectory_state'
import array  # noqa: E402, I100

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_TrajectoryStates(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'TrajectoryStates'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class TrajectoryStatesConstants(typing.TypedDict):
        ACTIVE: typing.Literal[0]
        FINISHED: typing.Literal[1]
        FROZEN: typing.Literal[2]
        DELETED: typing.Literal[3]

    __constants: TrajectoryStatesConstants = {
        'ACTIVE': 0,
        'FINISHED': 1,
        'FROZEN': 2,
        'DELETED': 3,
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
                'cartographer_ros_msgs.msg.TrajectoryStates')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__trajectory_states
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__trajectory_states
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__trajectory_states
            cls._TYPE_SUPPORT = module.type_support_msg__msg__trajectory_states
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__trajectory_states

            from std_msgs.msg import Header
            if Header._TYPE_SUPPORT is None:
                Header.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'ACTIVE': metacls.__constants['ACTIVE'],
            'FINISHED': metacls.__constants['FINISHED'],
            'FROZEN': metacls.__constants['FROZEN'],
            'DELETED': metacls.__constants['DELETED'],
        }

    @property
    def ACTIVE(self) -> typing.Literal[0]:
        """Message constant 'ACTIVE'."""
        return Metaclass_TrajectoryStates.__constants['ACTIVE']

    @property
    def FINISHED(self) -> typing.Literal[1]:
        """Message constant 'FINISHED'."""
        return Metaclass_TrajectoryStates.__constants['FINISHED']

    @property
    def FROZEN(self) -> typing.Literal[2]:
        """Message constant 'FROZEN'."""
        return Metaclass_TrajectoryStates.__constants['FROZEN']

    @property
    def DELETED(self) -> typing.Literal[3]:
        """Message constant 'DELETED'."""
        return Metaclass_TrajectoryStates.__constants['DELETED']


class TrajectoryStates(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_TrajectoryStates):
    """
    Message class 'TrajectoryStates'.

    Constants:
      ACTIVE
      FINISHED
      FROZEN
      DELETED
    """

    __slots__ = [
        '_header',
        '_trajectory_id',
        '_trajectory_state',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'header': 'std_msgs/Header',
        'trajectory_id': 'sequence<int32>',
        'trajectory_state': 'sequence<uint8>',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['std_msgs', 'msg'], 'Header'),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.BasicType('int32')),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.BasicType('uint8')),  # noqa: E501
    )

    def __init__(self, *,
                 header: typing.Optional[std_msgs.msg.Header] = None,  # noqa: E501
                 trajectory_id: typing.Optional[collections.abc.Sequence[int]] = None,  # noqa: E501
                 trajectory_state: typing.Optional[collections.abc.Sequence[int]] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from std_msgs.msg import Header
        self.header = header if header is not None else Header()
        self.trajectory_id = trajectory_id if trajectory_id is not None else array.array('i', [])
        self.trajectory_state = trajectory_state if trajectory_state is not None else array.array('B', [])

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
        if not isinstance(other, TrajectoryStates):
            return False
        if self.header != other.header:
            return False
        if self.trajectory_id != other.trajectory_id:
            return False
        if self.trajectory_state != other.trajectory_state:
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
    def trajectory_id(self) -> typing.Annotated[typing.Any, array.array[int]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'trajectory_id'."""
        return self._trajectory_id

    @trajectory_id.setter
    def trajectory_id(self, value: collections.abc.Sequence[int]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)

        if self._check_fields:
            if isinstance(value, array.array):
                assert value.typecode == 'i', \
                    "The 'trajectory_id' array.array() must have the type code of 'i'"
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     all(isinstance(v, int) for v in value) and
                     all(val >= -2147483648 and val < 2147483648 for val in value)), \
                    "The 'trajectory_id' field must be sequence and each value of type 'int' and each integer in [-2147483648, 2147483647]"

        if isinstance(value, array.array):
            self._trajectory_id = value
            return
        # type ignore below fixed in mypy 1.17+ see mypy#19421
        self._trajectory_id = array.array('i', value)  # type: ignore[assignment]

    @builtins.property
    def trajectory_state(self) -> typing.Annotated[typing.Any, array.array[int]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'trajectory_state'."""
        return self._trajectory_state

    @trajectory_state.setter
    def trajectory_state(self, value: collections.abc.Sequence[int]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from rosidl_buffer import Buffer as _RosidlBuffer
        if isinstance(value, _RosidlBuffer):
            self._trajectory_state = value  # type: ignore[assignment]
            return

        if self._check_fields:
            if isinstance(value, array.array):
                assert value.typecode == 'B', \
                    "The 'trajectory_state' array.array() must have the type code of 'B'"
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     all(isinstance(v, int) for v in value) and
                     all(val >= 0 and val < 256 for val in value)), \
                    "The 'trajectory_state' field must be sequence and each value of type 'int' and each unsigned integer in [0, 255]"

        if isinstance(value, array.array):
            self._trajectory_state = value  # type: ignore[assignment]
            return
        # type ignore below fixed in mypy 1.17+ see mypy#19421
        self._trajectory_state = array.array('B', value)  # type: ignore[assignment]
