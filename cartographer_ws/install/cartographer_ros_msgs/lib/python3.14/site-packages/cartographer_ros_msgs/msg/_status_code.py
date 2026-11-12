# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/StatusCode.idl
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

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_StatusCode(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'StatusCode'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class StatusCodeConstants(typing.TypedDict):
        OK: typing.Literal[0]
        CANCELLED: typing.Literal[1]
        UNKNOWN: typing.Literal[2]
        INVALID_ARGUMENT: typing.Literal[3]
        DEADLINE_EXCEEDED: typing.Literal[4]
        NOT_FOUND: typing.Literal[5]
        ALREADY_EXISTS: typing.Literal[6]
        PERMISSION_DENIED: typing.Literal[7]
        RESOURCE_EXHAUSTED: typing.Literal[8]
        FAILED_PRECONDITION: typing.Literal[9]
        ABORTED: typing.Literal[10]
        OUT_OF_RANGE: typing.Literal[11]
        UNIMPLEMENTED: typing.Literal[12]
        INTERNAL: typing.Literal[13]
        UNAVAILABLE: typing.Literal[14]
        DATA_LOSS: typing.Literal[15]

    __constants: StatusCodeConstants = {
        'OK': 0,
        'CANCELLED': 1,
        'UNKNOWN': 2,
        'INVALID_ARGUMENT': 3,
        'DEADLINE_EXCEEDED': 4,
        'NOT_FOUND': 5,
        'ALREADY_EXISTS': 6,
        'PERMISSION_DENIED': 7,
        'RESOURCE_EXHAUSTED': 8,
        'FAILED_PRECONDITION': 9,
        'ABORTED': 10,
        'OUT_OF_RANGE': 11,
        'UNIMPLEMENTED': 12,
        'INTERNAL': 13,
        'UNAVAILABLE': 14,
        'DATA_LOSS': 15,
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
                'cartographer_ros_msgs.msg.StatusCode')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__status_code
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__status_code
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__status_code
            cls._TYPE_SUPPORT = module.type_support_msg__msg__status_code
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__status_code

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'OK': metacls.__constants['OK'],
            'CANCELLED': metacls.__constants['CANCELLED'],
            'UNKNOWN': metacls.__constants['UNKNOWN'],
            'INVALID_ARGUMENT': metacls.__constants['INVALID_ARGUMENT'],
            'DEADLINE_EXCEEDED': metacls.__constants['DEADLINE_EXCEEDED'],
            'NOT_FOUND': metacls.__constants['NOT_FOUND'],
            'ALREADY_EXISTS': metacls.__constants['ALREADY_EXISTS'],
            'PERMISSION_DENIED': metacls.__constants['PERMISSION_DENIED'],
            'RESOURCE_EXHAUSTED': metacls.__constants['RESOURCE_EXHAUSTED'],
            'FAILED_PRECONDITION': metacls.__constants['FAILED_PRECONDITION'],
            'ABORTED': metacls.__constants['ABORTED'],
            'OUT_OF_RANGE': metacls.__constants['OUT_OF_RANGE'],
            'UNIMPLEMENTED': metacls.__constants['UNIMPLEMENTED'],
            'INTERNAL': metacls.__constants['INTERNAL'],
            'UNAVAILABLE': metacls.__constants['UNAVAILABLE'],
            'DATA_LOSS': metacls.__constants['DATA_LOSS'],
        }

    @property
    def OK(self) -> typing.Literal[0]:
        """Message constant 'OK'."""
        return Metaclass_StatusCode.__constants['OK']

    @property
    def CANCELLED(self) -> typing.Literal[1]:
        """Message constant 'CANCELLED'."""
        return Metaclass_StatusCode.__constants['CANCELLED']

    @property
    def UNKNOWN(self) -> typing.Literal[2]:
        """Message constant 'UNKNOWN'."""
        return Metaclass_StatusCode.__constants['UNKNOWN']

    @property
    def INVALID_ARGUMENT(self) -> typing.Literal[3]:
        """Message constant 'INVALID_ARGUMENT'."""
        return Metaclass_StatusCode.__constants['INVALID_ARGUMENT']

    @property
    def DEADLINE_EXCEEDED(self) -> typing.Literal[4]:
        """Message constant 'DEADLINE_EXCEEDED'."""
        return Metaclass_StatusCode.__constants['DEADLINE_EXCEEDED']

    @property
    def NOT_FOUND(self) -> typing.Literal[5]:
        """Message constant 'NOT_FOUND'."""
        return Metaclass_StatusCode.__constants['NOT_FOUND']

    @property
    def ALREADY_EXISTS(self) -> typing.Literal[6]:
        """Message constant 'ALREADY_EXISTS'."""
        return Metaclass_StatusCode.__constants['ALREADY_EXISTS']

    @property
    def PERMISSION_DENIED(self) -> typing.Literal[7]:
        """Message constant 'PERMISSION_DENIED'."""
        return Metaclass_StatusCode.__constants['PERMISSION_DENIED']

    @property
    def RESOURCE_EXHAUSTED(self) -> typing.Literal[8]:
        """Message constant 'RESOURCE_EXHAUSTED'."""
        return Metaclass_StatusCode.__constants['RESOURCE_EXHAUSTED']

    @property
    def FAILED_PRECONDITION(self) -> typing.Literal[9]:
        """Message constant 'FAILED_PRECONDITION'."""
        return Metaclass_StatusCode.__constants['FAILED_PRECONDITION']

    @property
    def ABORTED(self) -> typing.Literal[10]:
        """Message constant 'ABORTED'."""
        return Metaclass_StatusCode.__constants['ABORTED']

    @property
    def OUT_OF_RANGE(self) -> typing.Literal[11]:
        """Message constant 'OUT_OF_RANGE'."""
        return Metaclass_StatusCode.__constants['OUT_OF_RANGE']

    @property
    def UNIMPLEMENTED(self) -> typing.Literal[12]:
        """Message constant 'UNIMPLEMENTED'."""
        return Metaclass_StatusCode.__constants['UNIMPLEMENTED']

    @property
    def INTERNAL(self) -> typing.Literal[13]:
        """Message constant 'INTERNAL'."""
        return Metaclass_StatusCode.__constants['INTERNAL']

    @property
    def UNAVAILABLE(self) -> typing.Literal[14]:
        """Message constant 'UNAVAILABLE'."""
        return Metaclass_StatusCode.__constants['UNAVAILABLE']

    @property
    def DATA_LOSS(self) -> typing.Literal[15]:
        """Message constant 'DATA_LOSS'."""
        return Metaclass_StatusCode.__constants['DATA_LOSS']


class StatusCode(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_StatusCode):
    """
    Message class 'StatusCode'.

    Constants:
      OK
      CANCELLED
      UNKNOWN
      INVALID_ARGUMENT
      DEADLINE_EXCEEDED
      NOT_FOUND
      ALREADY_EXISTS
      PERMISSION_DENIED
      RESOURCE_EXHAUSTED
      FAILED_PRECONDITION
      ABORTED
      OUT_OF_RANGE
      UNIMPLEMENTED
      INTERNAL
      UNAVAILABLE
      DATA_LOSS
    """

    __slots__ = [
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
    )

    def __init__(self, *,
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'

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
        if not isinstance(other, StatusCode):
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)
