#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__FinishTrajectory_Request__init(msg: *mut FinishTrajectory_Request) -> bool;
    fn cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Request>);
    fn cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FinishTrajectory_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__FinishTrajectory_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FinishTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for FinishTrajectory_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__FinishTrajectory_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__FinishTrajectory_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FinishTrajectory_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FinishTrajectory_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FinishTrajectory_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/FinishTrajectory_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__FinishTrajectory_Response__init(msg: *mut FinishTrajectory_Response) -> bool;
    fn cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Response>);
    fn cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FinishTrajectory_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<FinishTrajectory_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__FinishTrajectory_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FinishTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,

}



impl Default for FinishTrajectory_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__FinishTrajectory_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__FinishTrajectory_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FinishTrajectory_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__FinishTrajectory_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FinishTrajectory_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FinishTrajectory_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/FinishTrajectory_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__FinishTrajectory_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Request__init(msg: *mut GetTrajectoryStates_Request) -> bool;
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Request>);
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetTrajectoryStates_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__GetTrajectoryStates_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetTrajectoryStates_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for GetTrajectoryStates_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__GetTrajectoryStates_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__GetTrajectoryStates_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetTrajectoryStates_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetTrajectoryStates_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetTrajectoryStates_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/GetTrajectoryStates_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Response__init(msg: *mut GetTrajectoryStates_Response) -> bool;
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Response>);
    fn cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetTrajectoryStates_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetTrajectoryStates_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__GetTrajectoryStates_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetTrajectoryStates_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_states: super::super::msg::rmw::TrajectoryStates,

}



impl Default for GetTrajectoryStates_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__GetTrajectoryStates_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__GetTrajectoryStates_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetTrajectoryStates_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__GetTrajectoryStates_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetTrajectoryStates_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetTrajectoryStates_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/GetTrajectoryStates_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__GetTrajectoryStates_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__ReadMetrics_Request__init(msg: *mut ReadMetrics_Request) -> bool;
    fn cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Request>);
    fn cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReadMetrics_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__ReadMetrics_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReadMetrics_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ReadMetrics_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__ReadMetrics_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__ReadMetrics_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReadMetrics_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReadMetrics_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReadMetrics_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/ReadMetrics_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__ReadMetrics_Response__init(msg: *mut ReadMetrics_Response) -> bool;
    fn cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Response>);
    fn cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ReadMetrics_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ReadMetrics_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__ReadMetrics_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ReadMetrics_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub metric_families: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MetricFamily>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub timestamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for ReadMetrics_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__ReadMetrics_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__ReadMetrics_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ReadMetrics_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__ReadMetrics_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ReadMetrics_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ReadMetrics_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/ReadMetrics_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__ReadMetrics_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__StartTrajectory_Request__init(msg: *mut StartTrajectory_Request) -> bool;
    fn cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>);
    fn cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartTrajectory_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__StartTrajectory_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub configuration_directory: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub configuration_basename: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub use_initial_pose: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub initial_pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub relative_to_trajectory_id: i32,

}



impl Default for StartTrajectory_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__StartTrajectory_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__StartTrajectory_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartTrajectory_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartTrajectory_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/StartTrajectory_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__StartTrajectory_Response__init(msg: *mut StartTrajectory_Response) -> bool;
    fn cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>);
    fn cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StartTrajectory_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<StartTrajectory_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__StartTrajectory_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StartTrajectory_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for StartTrajectory_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__StartTrajectory_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__StartTrajectory_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StartTrajectory_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__StartTrajectory_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StartTrajectory_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StartTrajectory_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/StartTrajectory_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__StartTrajectory_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Request__init(msg: *mut TrajectoryQuery_Request) -> bool;
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Request>);
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryQuery_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__TrajectoryQuery_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryQuery_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,

}



impl Default for TrajectoryQuery_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__TrajectoryQuery_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__TrajectoryQuery_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryQuery_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryQuery_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryQuery_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/TrajectoryQuery_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Response__init(msg: *mut TrajectoryQuery_Response) -> bool;
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Response>);
    fn cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryQuery_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryQuery_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__TrajectoryQuery_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory: rosidl_runtime_rs::Sequence<geometry_msgs::msg::rmw::PoseStamped>,

}



impl Default for TrajectoryQuery_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__TrajectoryQuery_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__TrajectoryQuery_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryQuery_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__TrajectoryQuery_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryQuery_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryQuery_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/TrajectoryQuery_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__TrajectoryQuery_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__SubmapQuery_Request__init(msg: *mut SubmapQuery_Request) -> bool;
    fn cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Request>);
    fn cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmapQuery_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__SubmapQuery_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__SubmapQuery_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__SubmapQuery_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmapQuery_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmapQuery_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmapQuery_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/SubmapQuery_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__SubmapQuery_Response__init(msg: *mut SubmapQuery_Response) -> bool;
    fn cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Response>);
    fn cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmapQuery_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmapQuery_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__SubmapQuery_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapQuery_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap_version: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub textures: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SubmapTexture>,

}



impl Default for SubmapQuery_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__SubmapQuery_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__SubmapQuery_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmapQuery_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__SubmapQuery_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmapQuery_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmapQuery_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/SubmapQuery_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__SubmapQuery_Response() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__WriteState_Request() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__WriteState_Request__init(msg: *mut WriteState_Request) -> bool;
    fn cartographer_ros_msgs__srv__WriteState_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WriteState_Request>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__WriteState_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WriteState_Request>);
    fn cartographer_ros_msgs__srv__WriteState_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WriteState_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<WriteState_Request>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__WriteState_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteState_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub include_unfinished_submaps: bool,

}



impl Default for WriteState_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__WriteState_Request__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__WriteState_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WriteState_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WriteState_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WriteState_Request where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/WriteState_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__WriteState_Request() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__WriteState_Response() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__srv__WriteState_Response__init(msg: *mut WriteState_Response) -> bool;
    fn cartographer_ros_msgs__srv__WriteState_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WriteState_Response>, size: usize) -> bool;
    fn cartographer_ros_msgs__srv__WriteState_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WriteState_Response>);
    fn cartographer_ros_msgs__srv__WriteState_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WriteState_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<WriteState_Response>) -> bool;
}

// Corresponds to cartographer_ros_msgs__srv__WriteState_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WriteState_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::StatusResponse,

}



impl Default for WriteState_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__srv__WriteState_Response__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__srv__WriteState_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WriteState_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__srv__WriteState_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WriteState_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WriteState_Response where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/srv/WriteState_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__srv__WriteState_Response() }
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


