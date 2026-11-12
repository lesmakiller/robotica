#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__BagfileProgress() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__BagfileProgress__init(msg: *mut BagfileProgress) -> bool;
    fn cartographer_ros_msgs__msg__BagfileProgress__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BagfileProgress>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__BagfileProgress__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BagfileProgress>);
    fn cartographer_ros_msgs__msg__BagfileProgress__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BagfileProgress>, out_seq: *mut rosidl_runtime_rs::Sequence<BagfileProgress>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__BagfileProgress
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Licensed under the Apache License, Version 2.0 (the 'License');
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an 'AS IS' BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BagfileProgress {
    /// Contains general information about the bagfiles processing progress
    pub current_bagfile_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_bagfile_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub total_bagfiles: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub total_messages: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub processed_messages: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub total_seconds: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub processed_seconds: f32,

}



impl Default for BagfileProgress {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__BagfileProgress__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__BagfileProgress__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BagfileProgress {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__BagfileProgress__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__BagfileProgress__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__BagfileProgress__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BagfileProgress {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BagfileProgress where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/BagfileProgress";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__BagfileProgress() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__HistogramBucket() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__HistogramBucket__init(msg: *mut HistogramBucket) -> bool;
    fn cartographer_ros_msgs__msg__HistogramBucket__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<HistogramBucket>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__HistogramBucket__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<HistogramBucket>);
    fn cartographer_ros_msgs__msg__HistogramBucket__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<HistogramBucket>, out_seq: *mut rosidl_runtime_rs::Sequence<HistogramBucket>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__HistogramBucket
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HistogramBucket {
    /// A histogram bucket counts values x for which:
    ///   previous_boundary < x <= bucket_boundary
    /// holds.
    pub bucket_boundary: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub count: f64,

}



impl Default for HistogramBucket {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__HistogramBucket__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__HistogramBucket__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for HistogramBucket {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__HistogramBucket__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__HistogramBucket__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__HistogramBucket__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for HistogramBucket {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for HistogramBucket where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/HistogramBucket";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__HistogramBucket() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__LandmarkEntry() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__LandmarkEntry__init(msg: *mut LandmarkEntry) -> bool;
    fn cartographer_ros_msgs__msg__LandmarkEntry__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LandmarkEntry>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__LandmarkEntry__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LandmarkEntry>);
    fn cartographer_ros_msgs__msg__LandmarkEntry__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LandmarkEntry>, out_seq: *mut rosidl_runtime_rs::Sequence<LandmarkEntry>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__LandmarkEntry
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LandmarkEntry {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_from_landmark_transform: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub translation_weight: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rotation_weight: f64,

}



impl Default for LandmarkEntry {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__LandmarkEntry__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__LandmarkEntry__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LandmarkEntry {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkEntry__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkEntry__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkEntry__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LandmarkEntry {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LandmarkEntry where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/LandmarkEntry";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__LandmarkEntry() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__LandmarkList() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__LandmarkList__init(msg: *mut LandmarkList) -> bool;
    fn cartographer_ros_msgs__msg__LandmarkList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LandmarkList>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__LandmarkList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LandmarkList>);
    fn cartographer_ros_msgs__msg__LandmarkList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LandmarkList>, out_seq: *mut rosidl_runtime_rs::Sequence<LandmarkList>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__LandmarkList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LandmarkList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub landmarks: rosidl_runtime_rs::Sequence<super::super::msg::rmw::LandmarkEntry>,

}



impl Default for LandmarkList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__LandmarkList__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__LandmarkList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LandmarkList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__LandmarkList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LandmarkList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LandmarkList where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/LandmarkList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__LandmarkList() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__MetricFamily() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__MetricFamily__init(msg: *mut MetricFamily) -> bool;
    fn cartographer_ros_msgs__msg__MetricFamily__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MetricFamily>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__MetricFamily__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MetricFamily>);
    fn cartographer_ros_msgs__msg__MetricFamily__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MetricFamily>, out_seq: *mut rosidl_runtime_rs::Sequence<MetricFamily>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__MetricFamily
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MetricFamily {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub metrics: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Metric>,

}



impl Default for MetricFamily {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__MetricFamily__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__MetricFamily__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MetricFamily {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricFamily__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricFamily__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricFamily__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MetricFamily {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MetricFamily where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/MetricFamily";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__MetricFamily() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__MetricLabel() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__MetricLabel__init(msg: *mut MetricLabel) -> bool;
    fn cartographer_ros_msgs__msg__MetricLabel__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MetricLabel>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__MetricLabel__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MetricLabel>);
    fn cartographer_ros_msgs__msg__MetricLabel__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MetricLabel>, out_seq: *mut rosidl_runtime_rs::Sequence<MetricLabel>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__MetricLabel
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MetricLabel {

    // This member is not documented.
    #[allow(missing_docs)]
    pub key: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: rosidl_runtime_rs::String,

}



impl Default for MetricLabel {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__MetricLabel__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__MetricLabel__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MetricLabel {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricLabel__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricLabel__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__MetricLabel__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MetricLabel {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MetricLabel where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/MetricLabel";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__MetricLabel() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__Metric() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__Metric__init(msg: *mut Metric) -> bool;
    fn cartographer_ros_msgs__msg__Metric__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Metric>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__Metric__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Metric>);
    fn cartographer_ros_msgs__msg__Metric__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Metric>, out_seq: *mut rosidl_runtime_rs::Sequence<Metric>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__Metric
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metric {

    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub labels: rosidl_runtime_rs::Sequence<super::super::msg::rmw::MetricLabel>,

    /// TYPE_COUNTER or TYPE_GAUGE
    pub value: f64,

    /// TYPE_HISTOGRAM
    pub counts_by_bucket: rosidl_runtime_rs::Sequence<super::super::msg::rmw::HistogramBucket>,

}

impl Metric {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_COUNTER: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_GAUGE: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TYPE_HISTOGRAM: u8 = 2;

}


impl Default for Metric {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__Metric__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__Metric__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Metric {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__Metric__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__Metric__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__Metric__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Metric {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Metric where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/Metric";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__Metric() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__StatusCode() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__StatusCode__init(msg: *mut StatusCode) -> bool;
    fn cartographer_ros_msgs__msg__StatusCode__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StatusCode>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__StatusCode__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StatusCode>);
    fn cartographer_ros_msgs__msg__StatusCode__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StatusCode>, out_seq: *mut rosidl_runtime_rs::Sequence<StatusCode>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__StatusCode
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StatusCode {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}

impl StatusCode {
    /// Definition of status code constants equivalent to the gRPC API.
    /// https://developers.google.com/maps-booking/reference/grpc-api/status_codes
    pub const OK: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CANCELLED: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNKNOWN: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INVALID_ARGUMENT: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DEADLINE_EXCEEDED: u8 = 4;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const NOT_FOUND: u8 = 5;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ALREADY_EXISTS: u8 = 6;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PERMISSION_DENIED: u8 = 7;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESOURCE_EXHAUSTED: u8 = 8;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FAILED_PRECONDITION: u8 = 9;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ABORTED: u8 = 10;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OUT_OF_RANGE: u8 = 11;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNIMPLEMENTED: u8 = 12;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INTERNAL: u8 = 13;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNAVAILABLE: u8 = 14;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DATA_LOSS: u8 = 15;

}


impl Default for StatusCode {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__StatusCode__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__StatusCode__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StatusCode {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusCode__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusCode__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusCode__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StatusCode {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StatusCode where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/StatusCode";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__StatusCode() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__StatusResponse() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__StatusResponse__init(msg: *mut StatusResponse) -> bool;
    fn cartographer_ros_msgs__msg__StatusResponse__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<StatusResponse>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__StatusResponse__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<StatusResponse>);
    fn cartographer_ros_msgs__msg__StatusResponse__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<StatusResponse>, out_seq: *mut rosidl_runtime_rs::Sequence<StatusResponse>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__StatusResponse
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StatusResponse {
    /// A common message type to indicate the outcome of a service call.
    pub code: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for StatusResponse {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__StatusResponse__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__StatusResponse__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for StatusResponse {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusResponse__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusResponse__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__StatusResponse__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for StatusResponse {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for StatusResponse where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/StatusResponse";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__StatusResponse() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapEntry() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__SubmapEntry__init(msg: *mut SubmapEntry) -> bool;
    fn cartographer_ros_msgs__msg__SubmapEntry__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmapEntry>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__SubmapEntry__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmapEntry>);
    fn cartographer_ros_msgs__msg__SubmapEntry__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmapEntry>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmapEntry>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__SubmapEntry
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2016 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapEntry {

    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap_index: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap_version: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_frozen: bool,

}



impl Default for SubmapEntry {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__SubmapEntry__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__SubmapEntry__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmapEntry {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapEntry__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapEntry__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapEntry__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmapEntry {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmapEntry where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/SubmapEntry";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapEntry() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapList() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__SubmapList__init(msg: *mut SubmapList) -> bool;
    fn cartographer_ros_msgs__msg__SubmapList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmapList>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__SubmapList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmapList>);
    fn cartographer_ros_msgs__msg__SubmapList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmapList>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmapList>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__SubmapList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2016 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap: rosidl_runtime_rs::Sequence<super::super::msg::rmw::SubmapEntry>,

}



impl Default for SubmapList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__SubmapList__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__SubmapList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmapList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmapList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmapList where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/SubmapList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapList() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapTexture() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__SubmapTexture__init(msg: *mut SubmapTexture) -> bool;
    fn cartographer_ros_msgs__msg__SubmapTexture__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SubmapTexture>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__SubmapTexture__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SubmapTexture>);
    fn cartographer_ros_msgs__msg__SubmapTexture__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SubmapTexture>, out_seq: *mut rosidl_runtime_rs::Sequence<SubmapTexture>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__SubmapTexture
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2017 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapTexture {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cells: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub width: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub height: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub resolution: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub slice_pose: geometry_msgs::msg::rmw::Pose,

}



impl Default for SubmapTexture {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__SubmapTexture__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__SubmapTexture__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SubmapTexture {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapTexture__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapTexture__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__SubmapTexture__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SubmapTexture {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SubmapTexture where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/SubmapTexture";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__SubmapTexture() }
  }
}


#[link(name = "cartographer_ros_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__TrajectoryStates() -> *const std::ffi::c_void;
}

#[link(name = "cartographer_ros_msgs__rosidl_generator_c")]
extern "C" {
    fn cartographer_ros_msgs__msg__TrajectoryStates__init(msg: *mut TrajectoryStates) -> bool;
    fn cartographer_ros_msgs__msg__TrajectoryStates__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryStates>, size: usize) -> bool;
    fn cartographer_ros_msgs__msg__TrajectoryStates__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryStates>);
    fn cartographer_ros_msgs__msg__TrajectoryStates__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryStates>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryStates>) -> bool;
}

// Corresponds to cartographer_ros_msgs__msg__TrajectoryStates
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright 2018 The Cartographer Authors
///
/// Licensed under the Apache License, Version 2.0 (the 'License');
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an 'AS IS' BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: rosidl_runtime_rs::Sequence<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_state: rosidl_runtime_rs::Sequence<u8>,

}

impl TrajectoryStates {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ACTIVE: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FINISHED: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FROZEN: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DELETED: u8 = 3;

}


impl Default for TrajectoryStates {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !cartographer_ros_msgs__msg__TrajectoryStates__init(&mut msg as *mut _) {
        panic!("Call to cartographer_ros_msgs__msg__TrajectoryStates__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryStates {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__TrajectoryStates__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__TrajectoryStates__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { cartographer_ros_msgs__msg__TrajectoryStates__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryStates {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryStates where Self: Sized {
  const TYPE_NAME: &'static str = "cartographer_ros_msgs/msg/TrajectoryStates";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__cartographer_ros_msgs__msg__TrajectoryStates() }
  }
}


