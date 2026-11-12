# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:msg/Metric.idl
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
    import cartographer_ros_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_Metric(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'Metric'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class MetricConstants(typing.TypedDict):
        TYPE_COUNTER: typing.Literal[0]
        TYPE_GAUGE: typing.Literal[1]
        TYPE_HISTOGRAM: typing.Literal[2]

    __constants: MetricConstants = {
        'TYPE_COUNTER': 0,
        'TYPE_GAUGE': 1,
        'TYPE_HISTOGRAM': 2,
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
                'cartographer_ros_msgs.msg.Metric')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__metric
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__metric
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__metric
            cls._TYPE_SUPPORT = module.type_support_msg__msg__metric
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__metric

            from cartographer_ros_msgs.msg import HistogramBucket
            if HistogramBucket._TYPE_SUPPORT is None:
                HistogramBucket.__import_type_support__()

            from cartographer_ros_msgs.msg import MetricLabel
            if MetricLabel._TYPE_SUPPORT is None:
                MetricLabel.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'TYPE_COUNTER': metacls.__constants['TYPE_COUNTER'],
            'TYPE_GAUGE': metacls.__constants['TYPE_GAUGE'],
            'TYPE_HISTOGRAM': metacls.__constants['TYPE_HISTOGRAM'],
        }

    @property
    def TYPE_COUNTER(self) -> typing.Literal[0]:
        """Message constant 'TYPE_COUNTER'."""
        return Metaclass_Metric.__constants['TYPE_COUNTER']

    @property
    def TYPE_GAUGE(self) -> typing.Literal[1]:
        """Message constant 'TYPE_GAUGE'."""
        return Metaclass_Metric.__constants['TYPE_GAUGE']

    @property
    def TYPE_HISTOGRAM(self) -> typing.Literal[2]:
        """Message constant 'TYPE_HISTOGRAM'."""
        return Metaclass_Metric.__constants['TYPE_HISTOGRAM']


class Metric(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_Metric):
    """
    Message class 'Metric'.

    Constants:
      TYPE_COUNTER
      TYPE_GAUGE
      TYPE_HISTOGRAM
    """

    __slots__ = [
        '_type',
        '_labels',
        '_value',
        '_counts_by_bucket',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'type': 'uint8',
        'labels': 'sequence<cartographer_ros_msgs/MetricLabel>',
        'value': 'double',
        'counts_by_bucket': 'sequence<cartographer_ros_msgs/HistogramBucket>',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.NamespacedType(['cartographer_ros_msgs', 'msg'], 'MetricLabel')),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.NamespacedType(['cartographer_ros_msgs', 'msg'], 'HistogramBucket')),  # noqa: E501
    )

    def __init__(self, *,
                 type: typing.Optional[int] = None,  # noqa: E501, A002
                 labels: typing.Optional[collections.abc.Sequence[cartographer_ros_msgs.msg.MetricLabel]] = None,  # noqa: E501
                 value: typing.Optional[float] = None,  # noqa: E501
                 counts_by_bucket: typing.Optional[collections.abc.Sequence[cartographer_ros_msgs.msg.HistogramBucket]] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.type = type if type is not None else int()
        self.labels = labels if labels is not None else []
        self.value = value if value is not None else float()
        self.counts_by_bucket = counts_by_bucket if counts_by_bucket is not None else []

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
        if not isinstance(other, Metric):
            return False
        if self.type != other.type:
            return False
        if self.labels != other.labels:
            return False
        if self.value != other.value:
            return False
        if self.counts_by_bucket != other.counts_by_bucket:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property  # noqa: A003
    def type(self) -> int:  # noqa: A003
        """Message field 'type'."""
        return self._type

    @type.setter  # noqa: A003
    def type(self, value: int) -> None:  # noqa: A003

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'type' field must be of type 'int'"
                assert value >= 0 and value < 256, \
                    "The 'type' field must be an unsigned integer in [0, 255]"

        self._type = value

    @builtins.property
    def labels(self) -> typing.Annotated[typing.Any, list[cartographer_ros_msgs.msg.MetricLabel]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'labels'."""
        return self._labels

    @labels.setter
    def labels(self, value: collections.abc.Sequence[cartographer_ros_msgs.msg.MetricLabel]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from cartographer_ros_msgs.msg import MetricLabel

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     all(isinstance(v, MetricLabel) for v in value) and
                     True), \
                    "The 'labels' field must be sequence and each value of type 'MetricLabel'"

        if isinstance(value, list):
            self._labels = value
            return
        self._labels = list(value)

    @builtins.property
    def value(self) -> float:
        """Message field 'value'."""
        return self._value

    @value.setter
    def value(self, value: float) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, float), \
                    "The 'value' field must be of type 'float'"
                assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                    "The 'value' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"

        self._value = value

    @builtins.property
    def counts_by_bucket(self) -> typing.Annotated[typing.Any, list[cartographer_ros_msgs.msg.HistogramBucket]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'counts_by_bucket'."""
        return self._counts_by_bucket

    @counts_by_bucket.setter
    def counts_by_bucket(self, value: collections.abc.Sequence[cartographer_ros_msgs.msg.HistogramBucket]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from cartographer_ros_msgs.msg import HistogramBucket

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    ((isinstance(value, collections.abc.Sequence) or
                     isinstance(value, collections.abc.Set)) and
                     not isinstance(value, str) and
                     not isinstance(value, collections.UserString) and
                     all(isinstance(v, HistogramBucket) for v in value) and
                     True), \
                    "The 'counts_by_bucket' field must be sequence and each value of type 'HistogramBucket'"

        if isinstance(value, list):
            self._counts_by_bucket = value
            return
        self._counts_by_bucket = list(value)
