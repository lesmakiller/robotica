# generated from rosidl_generator_py/resource/_idl.py.em
# with input from cartographer_ros_msgs:srv/StartTrajectory.idl
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


class Metaclass_StartTrajectory_Request(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'StartTrajectory_Request'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class StartTrajectory_RequestConstants(typing.TypedDict):
        pass

    __constants: StartTrajectory_RequestConstants = {
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
                'cartographer_ros_msgs.srv.StartTrajectory_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__start_trajectory__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__start_trajectory__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__start_trajectory__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__start_trajectory__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__start_trajectory__request

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


class StartTrajectory_Request(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_StartTrajectory_Request):
    """Message class 'StartTrajectory_Request'."""

    __slots__ = [
        '_configuration_directory',
        '_configuration_basename',
        '_use_initial_pose',
        '_initial_pose',
        '_relative_to_trajectory_id',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'configuration_directory': 'string',
        'configuration_basename': 'string',
        'use_initial_pose': 'boolean',
        'initial_pose': 'geometry_msgs/Pose',
        'relative_to_trajectory_id': 'int32',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['geometry_msgs', 'msg'], 'Pose'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, *,
                 configuration_directory: typing.Optional[str] = None,  # noqa: E501
                 configuration_basename: typing.Optional[str] = None,  # noqa: E501
                 use_initial_pose: typing.Optional[bool] = None,  # noqa: E501
                 initial_pose: typing.Optional[geometry_msgs.msg.Pose] = None,  # noqa: E501
                 relative_to_trajectory_id: typing.Optional[int] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        self.configuration_directory = configuration_directory if configuration_directory is not None else str()
        self.configuration_basename = configuration_basename if configuration_basename is not None else str()
        self.use_initial_pose = use_initial_pose if use_initial_pose is not None else bool()
        from geometry_msgs.msg import Pose
        self.initial_pose = initial_pose if initial_pose is not None else Pose()
        self.relative_to_trajectory_id = relative_to_trajectory_id if relative_to_trajectory_id is not None else int()

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
        if not isinstance(other, StartTrajectory_Request):
            return False
        if self.configuration_directory != other.configuration_directory:
            return False
        if self.configuration_basename != other.configuration_basename:
            return False
        if self.use_initial_pose != other.use_initial_pose:
            return False
        if self.initial_pose != other.initial_pose:
            return False
        if self.relative_to_trajectory_id != other.relative_to_trajectory_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def configuration_directory(self) -> str:
        """Message field 'configuration_directory'."""
        return self._configuration_directory

    @configuration_directory.setter
    def configuration_directory(self, value: str) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, str), \
                    "The 'configuration_directory' field must be of type 'str'"

        self._configuration_directory = value

    @builtins.property
    def configuration_basename(self) -> str:
        """Message field 'configuration_basename'."""
        return self._configuration_basename

    @configuration_basename.setter
    def configuration_basename(self, value: str) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, str), \
                    "The 'configuration_basename' field must be of type 'str'"

        self._configuration_basename = value

    @builtins.property
    def use_initial_pose(self) -> bool:
        """Message field 'use_initial_pose'."""
        return self._use_initial_pose

    @use_initial_pose.setter
    def use_initial_pose(self, value: bool) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, bool), \
                    "The 'use_initial_pose' field must be of type 'bool'"

        self._use_initial_pose = value

    @builtins.property
    def initial_pose(self) -> geometry_msgs.msg.Pose:
        """Message field 'initial_pose'."""
        return self._initial_pose

    @initial_pose.setter
    def initial_pose(self, value: geometry_msgs.msg.Pose) -> None:
        from geometry_msgs.msg import Pose

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, Pose), \
                    "The 'initial_pose' field must be a sub message of type 'Pose'"

        self._initial_pose = value

    @builtins.property
    def relative_to_trajectory_id(self) -> int:
        """Message field 'relative_to_trajectory_id'."""
        return self._relative_to_trajectory_id

    @relative_to_trajectory_id.setter
    def relative_to_trajectory_id(self, value: int) -> None:

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, int), \
                    "The 'relative_to_trajectory_id' field must be of type 'int'"
                assert value >= -2147483648 and value < 2147483648, \
                    "The 'relative_to_trajectory_id' field must be an integer in [-2147483648, 2147483647]"

        self._relative_to_trajectory_id = value


if typing.TYPE_CHECKING:
    import cartographer_ros_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_StartTrajectory_Response(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'StartTrajectory_Response'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class StartTrajectory_ResponseConstants(typing.TypedDict):
        pass

    __constants: StartTrajectory_ResponseConstants = {
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
                'cartographer_ros_msgs.srv.StartTrajectory_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__start_trajectory__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__start_trajectory__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__start_trajectory__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__start_trajectory__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__start_trajectory__response

            from cartographer_ros_msgs.msg import StatusResponse
            if StatusResponse._TYPE_SUPPORT is None:
                StatusResponse.__import_type_support__()

    @classmethod
    def __prepare__(metacls, name: str, bases: tuple[type[typing.Any], ...], /, **kwds: typing.Any) -> collections.abc.MutableMapping[str, object]:
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class StartTrajectory_Response(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_StartTrajectory_Response):
    """Message class 'StartTrajectory_Response'."""

    __slots__ = [
        '_status',
        '_trajectory_id',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'status': 'cartographer_ros_msgs/StatusResponse',
        'trajectory_id': 'int32',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['cartographer_ros_msgs', 'msg'], 'StatusResponse'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, *,
                 status: typing.Optional[cartographer_ros_msgs.msg.StatusResponse] = None,  # noqa: E501
                 trajectory_id: typing.Optional[int] = None,  # noqa: E501
                 check_fields: typing.Optional[bool] = None) -> None:
        if check_fields is not None:
            self._check_fields = check_fields
        else:
            self._check_fields = ros_python_check_fields == '1'
        from cartographer_ros_msgs.msg import StatusResponse
        self.status = status if status is not None else StatusResponse()
        self.trajectory_id = trajectory_id if trajectory_id is not None else int()

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
        if not isinstance(other, StartTrajectory_Response):
            return False
        if self.status != other.status:
            return False
        if self.trajectory_id != other.trajectory_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls) -> dict[str, str]:
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def status(self) -> cartographer_ros_msgs.msg.StatusResponse:
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value: cartographer_ros_msgs.msg.StatusResponse) -> None:
        from cartographer_ros_msgs.msg import StatusResponse

        if self._check_fields:
            if False:  # Done for templating alignment
                pass
            else:
                assert \
                    isinstance(value, StatusResponse), \
                    "The 'status' field must be a sub message of type 'StatusResponse'"

        self._status = value

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


if typing.TYPE_CHECKING:
    import cartographer_ros_msgs.srv  # noqa: E402, I100, I201, I300
    import service_msgs.msg  # noqa: E402, I100, I201, I300


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_StartTrajectory_Event(rosidl_pycommon.interface_base_classes.MessageTypeSupportMeta):
    """Metaclass of message 'StartTrajectory_Event'."""

    _CREATE_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_FROM_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _CONVERT_TO_PY: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _DESTROY_ROS_MESSAGE: typing.ClassVar[typing.Optional[PyCapsule]] = None
    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    class StartTrajectory_EventConstants(typing.TypedDict):
        pass

    __constants: StartTrajectory_EventConstants = {
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
                'cartographer_ros_msgs.srv.StartTrajectory_Event')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__start_trajectory__event
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__start_trajectory__event
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__start_trajectory__event
            cls._TYPE_SUPPORT = module.type_support_msg__srv__start_trajectory__event
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__start_trajectory__event

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


class StartTrajectory_Event(rosidl_pycommon.interface_base_classes.BaseMessage, metaclass=Metaclass_StartTrajectory_Event):
    """Message class 'StartTrajectory_Event'."""

    __slots__ = [
        '_info',
        '_request',
        '_response',
        '_check_fields',
    ]

    _fields_and_field_types: dict[str, str] = {
        'info': 'service_msgs/ServiceEventInfo',
        'request': 'sequence<cartographer_ros_msgs/StartTrajectory_Request, 1>',
        'response': 'sequence<cartographer_ros_msgs/StartTrajectory_Response, 1>',
    }

    # This attribute is used to store an rosidl_parser.definition variable
    # related to the data type of each of the components the message.
    SLOT_TYPES: tuple[rosidl_parser.definition.AbstractType, ...] = (
        rosidl_parser.definition.NamespacedType(['service_msgs', 'msg'], 'ServiceEventInfo'),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['cartographer_ros_msgs', 'srv'], 'StartTrajectory_Request'), 1),  # noqa: E501
        rosidl_parser.definition.BoundedSequence(rosidl_parser.definition.NamespacedType(['cartographer_ros_msgs', 'srv'], 'StartTrajectory_Response'), 1),  # noqa: E501
    )

    def __init__(self, *,
                 info: typing.Optional[service_msgs.msg.ServiceEventInfo] = None,  # noqa: E501
                 request: typing.Optional[collections.abc.Sequence[cartographer_ros_msgs.srv.StartTrajectory_Request]] = None,  # noqa: E501
                 response: typing.Optional[collections.abc.Sequence[cartographer_ros_msgs.srv.StartTrajectory_Response]] = None,  # noqa: E501
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
        if not isinstance(other, StartTrajectory_Event):
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
    def request(self) -> typing.Annotated[typing.Any, list[cartographer_ros_msgs.srv.StartTrajectory_Request]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'request'."""
        return self._request

    @request.setter
    def request(self, value: collections.abc.Sequence[cartographer_ros_msgs.srv.StartTrajectory_Request]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from cartographer_ros_msgs.srv import StartTrajectory_Request

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
                     all(isinstance(v, StartTrajectory_Request) for v in value) and
                     True), \
                    "The 'request' field must be sequence with length <= 1 and each value of type 'StartTrajectory_Request'"

        if isinstance(value, list):
            self._request = value
            return
        self._request = list(value)

    @builtins.property
    def response(self) -> typing.Annotated[typing.Any, list[cartographer_ros_msgs.srv.StartTrajectory_Response]]:   # typing.Annotated can be remove after mypy 1.16+ see mypy#3004
        """Message field 'response'."""
        return self._response

    @response.setter
    def response(self, value: collections.abc.Sequence[cartographer_ros_msgs.srv.StartTrajectory_Response]) -> None:
        if isinstance(value, collections.abc.Set):
            import warnings
            warnings.warn(
                'Using set or subclass of set is deprecated,'
                ' please use a subclass of collections.abc.Sequence like list',
                DeprecationWarning)
        from cartographer_ros_msgs.srv import StartTrajectory_Response

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
                     all(isinstance(v, StartTrajectory_Response) for v in value) and
                     True), \
                    "The 'response' field must be sequence with length <= 1 and each value of type 'StartTrajectory_Response'"

        if isinstance(value, list):
            self._response = value
            return
        self._response = list(value)


if typing.TYPE_CHECKING:
    from typing_extensions import TypeAlias  # noqa: I100, I300


class Metaclass_StartTrajectory(rosidl_pycommon.interface_base_classes.ServiceTypeSupportMeta):
    """Metaclass of service 'StartTrajectory'."""

    _TYPE_SUPPORT: typing.ClassVar[typing.Optional[PyCapsule]] = None

    @classmethod
    def __import_type_support__(cls) -> None:
        try:
            from rosidl_generator_py import import_type_support  # type: ignore[attr-defined]
            module = import_type_support('cartographer_ros_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'cartographer_ros_msgs.srv.StartTrajectory')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__start_trajectory

            from cartographer_ros_msgs.srv import _start_trajectory
            if _start_trajectory.Metaclass_StartTrajectory_Request._TYPE_SUPPORT is None:
                _start_trajectory.Metaclass_StartTrajectory_Request.__import_type_support__()
            if _start_trajectory.Metaclass_StartTrajectory_Response._TYPE_SUPPORT is None:
                _start_trajectory.Metaclass_StartTrajectory_Response.__import_type_support__()
            if _start_trajectory.Metaclass_StartTrajectory_Event._TYPE_SUPPORT is None:
                _start_trajectory.Metaclass_StartTrajectory_Event.__import_type_support__()


class StartTrajectory(rosidl_pycommon.interface_base_classes.BaseService[
    StartTrajectory_Request,
    StartTrajectory_Response
], metaclass=Metaclass_StartTrajectory):
    Request: TypeAlias = StartTrajectory_Request
    Response: TypeAlias = StartTrajectory_Response
    Event: TypeAlias = StartTrajectory_Event

    # Should eventually be typing.NoReturn. See mypy#14044
    def __init__(self) -> None:
        raise NotImplementedError('Service classes can not be instantiated')
