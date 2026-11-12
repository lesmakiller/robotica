#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to cartographer_ros_msgs__srv__FinishTrajectory_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FinishTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for FinishTrajectory_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FinishTrajectory_Request::default())
  }
}

impl rosidl_runtime_rs::Message for FinishTrajectory_Request {
  type RmwMsg = super::srv::rmw::FinishTrajectory_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_id: msg.trajectory_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      trajectory_id: msg.trajectory_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_id: msg.trajectory_id,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__FinishTrajectory_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FinishTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,

}



impl Default for FinishTrajectory_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::FinishTrajectory_Response::default())
  }
}

impl rosidl_runtime_rs::Message for FinishTrajectory_Response {
  type RmwMsg = super::srv::rmw::FinishTrajectory_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__GetTrajectoryStates_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetTrajectoryStates_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetTrajectoryStates_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetTrajectoryStates_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetTrajectoryStates_Request {
  type RmwMsg = super::srv::rmw::GetTrajectoryStates_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__GetTrajectoryStates_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetTrajectoryStates_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_states: super::msg::TrajectoryStates,

}



impl Default for GetTrajectoryStates_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetTrajectoryStates_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetTrajectoryStates_Response {
  type RmwMsg = super::srv::rmw::GetTrajectoryStates_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        trajectory_states: super::msg::TrajectoryStates::into_rmw_message(std::borrow::Cow::Owned(msg.trajectory_states)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
        trajectory_states: super::msg::TrajectoryStates::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trajectory_states)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
      trajectory_states: super::msg::TrajectoryStates::from_rmw_message(msg.trajectory_states),
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__ReadMetrics_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReadMetrics_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ReadMetrics_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReadMetrics_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ReadMetrics_Request {
  type RmwMsg = super::srv::rmw::ReadMetrics_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__ReadMetrics_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReadMetrics_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub metric_families: Vec<super::msg::MetricFamily>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: builtin_interfaces::msg::Time,

}



impl Default for ReadMetrics_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ReadMetrics_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ReadMetrics_Response {
  type RmwMsg = super::srv::rmw::ReadMetrics_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        metric_families: msg.metric_families
          .into_iter()
          .map(|elem| super::msg::MetricFamily::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        timestamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.timestamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
        metric_families: msg.metric_families
          .iter()
          .map(|elem| super::msg::MetricFamily::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        timestamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.timestamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
      metric_families: msg.metric_families
          .into_iter()
          .map(super::msg::MetricFamily::from_rmw_message)
          .collect(),
      timestamp: builtin_interfaces::msg::Time::from_rmw_message(msg.timestamp),
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__StartTrajectory_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub configuration_directory: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub configuration_basename: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub use_initial_pose: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub initial_pose: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub relative_to_trajectory_id: i32,

}



impl Default for StartTrajectory_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartTrajectory_Request::default())
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Request {
  type RmwMsg = super::srv::rmw::StartTrajectory_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        configuration_directory: msg.configuration_directory.as_str().into(),
        configuration_basename: msg.configuration_basename.as_str().into(),
        use_initial_pose: msg.use_initial_pose,
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.initial_pose)).into_owned(),
        relative_to_trajectory_id: msg.relative_to_trajectory_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        configuration_directory: msg.configuration_directory.as_str().into(),
        configuration_basename: msg.configuration_basename.as_str().into(),
      use_initial_pose: msg.use_initial_pose,
        initial_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.initial_pose)).into_owned(),
      relative_to_trajectory_id: msg.relative_to_trajectory_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      configuration_directory: msg.configuration_directory.to_string(),
      configuration_basename: msg.configuration_basename.to_string(),
      use_initial_pose: msg.use_initial_pose,
      initial_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.initial_pose),
      relative_to_trajectory_id: msg.relative_to_trajectory_id,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__StartTrajectory_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for StartTrajectory_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::StartTrajectory_Response::default())
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Response {
  type RmwMsg = super::srv::rmw::StartTrajectory_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        trajectory_id: msg.trajectory_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      trajectory_id: msg.trajectory_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
      trajectory_id: msg.trajectory_id,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__TrajectoryQuery_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryQuery_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for TrajectoryQuery_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TrajectoryQuery_Request::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryQuery_Request {
  type RmwMsg = super::srv::rmw::TrajectoryQuery_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_id: msg.trajectory_id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      trajectory_id: msg.trajectory_id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_id: msg.trajectory_id,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__TrajectoryQuery_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: Vec<geometry_msgs::msg::PoseStamped>,

}



impl Default for TrajectoryQuery_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::TrajectoryQuery_Response::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryQuery_Response {
  type RmwMsg = super::srv::rmw::TrajectoryQuery_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        trajectory: msg.trajectory
          .into_iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
        trajectory: msg.trajectory
          .iter()
          .map(|elem| geometry_msgs::msg::PoseStamped::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
      trajectory: msg.trajectory
          .into_iter()
          .map(geometry_msgs::msg::PoseStamped::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__SubmapQuery_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapQuery_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap_index: i32,

}



impl Default for SubmapQuery_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmapQuery_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SubmapQuery_Request {
  type RmwMsg = super::srv::rmw::SubmapQuery_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_id: msg.trajectory_id,
        submap_index: msg.submap_index,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      trajectory_id: msg.trajectory_id,
      submap_index: msg.submap_index,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_id: msg.trajectory_id,
      submap_index: msg.submap_index,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__SubmapQuery_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap_version: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub textures: Vec<super::msg::SubmapTexture>,

}



impl Default for SubmapQuery_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SubmapQuery_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SubmapQuery_Response {
  type RmwMsg = super::srv::rmw::SubmapQuery_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
        submap_version: msg.submap_version,
        textures: msg.textures
          .into_iter()
          .map(|elem| super::msg::SubmapTexture::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      submap_version: msg.submap_version,
        textures: msg.textures
          .iter()
          .map(|elem| super::msg::SubmapTexture::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
      submap_version: msg.submap_version,
      textures: msg.textures
          .into_iter()
          .map(super::msg::SubmapTexture::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__WriteState_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub include_unfinished_submaps: bool,

}



impl Default for WriteState_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WriteState_Request::default())
  }
}

impl rosidl_runtime_rs::Message for WriteState_Request {
  type RmwMsg = super::srv::rmw::WriteState_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
        include_unfinished_submaps: msg.include_unfinished_submaps,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      include_unfinished_submaps: msg.include_unfinished_submaps,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
      include_unfinished_submaps: msg.include_unfinished_submaps,
    }
  }
}


// Corresponds to cartographer_ros_msgs__srv__WriteState_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::StatusResponse,

}



impl Default for WriteState_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::WriteState_Response::default())
  }
}

impl rosidl_runtime_rs::Message for WriteState_Response {
  type RmwMsg = super::srv::rmw::WriteState_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Owned(msg.status)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: super::msg::StatusResponse::into_rmw_message(std::borrow::Cow::Borrowed(&msg.status)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: super::msg::StatusResponse::from_rmw_message(msg.status),
    }
  }
}






#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__FinishTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct FinishTrajectory;

impl rosidl_runtime_rs::Service for FinishTrajectory {
    type Request = FinishTrajectory_Request;
    type Response = FinishTrajectory_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__GetTrajectoryStates
#[allow(missing_docs, non_camel_case_types)]
pub struct GetTrajectoryStates;

impl rosidl_runtime_rs::Service for GetTrajectoryStates {
    type Request = GetTrajectoryStates_Request;
    type Response = GetTrajectoryStates_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__ReadMetrics
#[allow(missing_docs, non_camel_case_types)]
pub struct ReadMetrics;

impl rosidl_runtime_rs::Service for ReadMetrics {
    type Request = ReadMetrics_Request;
    type Response = ReadMetrics_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__StartTrajectory
#[allow(missing_docs, non_camel_case_types)]
pub struct StartTrajectory;

impl rosidl_runtime_rs::Service for StartTrajectory {
    type Request = StartTrajectory_Request;
    type Response = StartTrajectory_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__TrajectoryQuery
#[allow(missing_docs, non_camel_case_types)]
pub struct TrajectoryQuery;

impl rosidl_runtime_rs::Service for TrajectoryQuery {
    type Request = TrajectoryQuery_Request;
    type Response = TrajectoryQuery_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__SubmapQuery
#[allow(missing_docs, non_camel_case_types)]
pub struct SubmapQuery;

impl rosidl_runtime_rs::Service for SubmapQuery {
    type Request = SubmapQuery_Request;
    type Response = SubmapQuery_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery() }
    }
}




#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__WriteState() -> *const std::ffi::c_void;
}

// Corresponds to cartographer_ros_msgs__srv__WriteState
#[allow(missing_docs, non_camel_case_types)]
pub struct WriteState;

impl rosidl_runtime_rs::Service for WriteState {
    type Request = WriteState_Request;
    type Response = WriteState_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__cartographer_ros_msgs__srv__WriteState() }
    }
}


