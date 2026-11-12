#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to cartographer_ros_msgs__msg__BagfileProgress
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BagfileProgress {
    /// Contains general information about the bagfiles processing progress
    pub current_bagfile_name: std::string::String,


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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::BagfileProgress::default())
  }
}

impl rosidl_runtime_rs::Message for BagfileProgress {
  type RmwMsg = super::msg::rmw::BagfileProgress;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_bagfile_name: msg.current_bagfile_name.as_str().into(),
        current_bagfile_id: msg.current_bagfile_id,
        total_bagfiles: msg.total_bagfiles,
        total_messages: msg.total_messages,
        processed_messages: msg.processed_messages,
        total_seconds: msg.total_seconds,
        processed_seconds: msg.processed_seconds,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        current_bagfile_name: msg.current_bagfile_name.as_str().into(),
      current_bagfile_id: msg.current_bagfile_id,
      total_bagfiles: msg.total_bagfiles,
      total_messages: msg.total_messages,
      processed_messages: msg.processed_messages,
      total_seconds: msg.total_seconds,
      processed_seconds: msg.processed_seconds,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      current_bagfile_name: msg.current_bagfile_name.to_string(),
      current_bagfile_id: msg.current_bagfile_id,
      total_bagfiles: msg.total_bagfiles,
      total_messages: msg.total_messages,
      processed_messages: msg.processed_messages,
      total_seconds: msg.total_seconds,
      processed_seconds: msg.processed_seconds,
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__HistogramBucket
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HistogramBucket::default())
  }
}

impl rosidl_runtime_rs::Message for HistogramBucket {
  type RmwMsg = super::msg::rmw::HistogramBucket;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        bucket_boundary: msg.bucket_boundary,
        count: msg.count,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      bucket_boundary: msg.bucket_boundary,
      count: msg.count,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      bucket_boundary: msg.bucket_boundary,
      count: msg.count,
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__LandmarkEntry
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LandmarkEntry {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub tracking_from_landmark_transform: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub translation_weight: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rotation_weight: f64,

}



impl Default for LandmarkEntry {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LandmarkEntry::default())
  }
}

impl rosidl_runtime_rs::Message for LandmarkEntry {
  type RmwMsg = super::msg::rmw::LandmarkEntry;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        tracking_from_landmark_transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.tracking_from_landmark_transform)).into_owned(),
        translation_weight: msg.translation_weight,
        rotation_weight: msg.rotation_weight,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id.as_str().into(),
        tracking_from_landmark_transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.tracking_from_landmark_transform)).into_owned(),
      translation_weight: msg.translation_weight,
      rotation_weight: msg.rotation_weight,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id.to_string(),
      tracking_from_landmark_transform: geometry_msgs::msg::Pose::from_rmw_message(msg.tracking_from_landmark_transform),
      translation_weight: msg.translation_weight,
      rotation_weight: msg.rotation_weight,
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__LandmarkList
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LandmarkList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub landmarks: Vec<super::msg::LandmarkEntry>,

}



impl Default for LandmarkList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LandmarkList::default())
  }
}

impl rosidl_runtime_rs::Message for LandmarkList {
  type RmwMsg = super::msg::rmw::LandmarkList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        landmarks: msg.landmarks
          .into_iter()
          .map(|elem| super::msg::LandmarkEntry::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        landmarks: msg.landmarks
          .iter()
          .map(|elem| super::msg::LandmarkEntry::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      landmarks: msg.landmarks
          .into_iter()
          .map(super::msg::LandmarkEntry::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__MetricFamily
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MetricFamily {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub description: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub metrics: Vec<super::msg::Metric>,

}



impl Default for MetricFamily {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MetricFamily::default())
  }
}

impl rosidl_runtime_rs::Message for MetricFamily {
  type RmwMsg = super::msg::rmw::MetricFamily;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        description: msg.description.as_str().into(),
        metrics: msg.metrics
          .into_iter()
          .map(|elem| super::msg::Metric::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        description: msg.description.as_str().into(),
        metrics: msg.metrics
          .iter()
          .map(|elem| super::msg::Metric::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      description: msg.description.to_string(),
      metrics: msg.metrics
          .into_iter()
          .map(super::msg::Metric::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__MetricLabel
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MetricLabel {

    // This member is not documented.
    #[allow(missing_docs)]
    pub key: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: std::string::String,

}



impl Default for MetricLabel {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::MetricLabel::default())
  }
}

impl rosidl_runtime_rs::Message for MetricLabel {
  type RmwMsg = super::msg::rmw::MetricLabel;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        key: msg.key.as_str().into(),
        value: msg.value.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        key: msg.key.as_str().into(),
        value: msg.value.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      key: msg.key.to_string(),
      value: msg.value.to_string(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__Metric
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Metric {

    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub labels: Vec<super::msg::MetricLabel>,

    /// TYPE_COUNTER or TYPE_GAUGE
    pub value: f64,

    /// TYPE_HISTOGRAM
    pub counts_by_bucket: Vec<super::msg::HistogramBucket>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Metric::default())
  }
}

impl rosidl_runtime_rs::Message for Metric {
  type RmwMsg = super::msg::rmw::Metric;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_,
        labels: msg.labels
          .into_iter()
          .map(|elem| super::msg::MetricLabel::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        value: msg.value,
        counts_by_bucket: msg.counts_by_bucket
          .into_iter()
          .map(|elem| super::msg::HistogramBucket::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      type_: msg.type_,
        labels: msg.labels
          .iter()
          .map(|elem| super::msg::MetricLabel::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      value: msg.value,
        counts_by_bucket: msg.counts_by_bucket
          .iter()
          .map(|elem| super::msg::HistogramBucket::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_,
      labels: msg.labels
          .into_iter()
          .map(super::msg::MetricLabel::from_rmw_message)
          .collect(),
      value: msg.value,
      counts_by_bucket: msg.counts_by_bucket
          .into_iter()
          .map(super::msg::HistogramBucket::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__StatusCode
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StatusCode::default())
  }
}

impl rosidl_runtime_rs::Message for StatusCode {
  type RmwMsg = super::msg::rmw::StatusCode;

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


// Corresponds to cartographer_ros_msgs__msg__StatusResponse
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StatusResponse {
    /// A common message type to indicate the outcome of a service call.
    pub code: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for StatusResponse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StatusResponse::default())
  }
}

impl rosidl_runtime_rs::Message for StatusResponse {
  type RmwMsg = super::msg::rmw::StatusResponse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__SubmapEntry
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub pose: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_frozen: bool,

}



impl Default for SubmapEntry {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SubmapEntry::default())
  }
}

impl rosidl_runtime_rs::Message for SubmapEntry {
  type RmwMsg = super::msg::rmw::SubmapEntry;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        trajectory_id: msg.trajectory_id,
        submap_index: msg.submap_index,
        submap_version: msg.submap_version,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.pose)).into_owned(),
        is_frozen: msg.is_frozen,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      trajectory_id: msg.trajectory_id,
      submap_index: msg.submap_index,
      submap_version: msg.submap_version,
        pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.pose)).into_owned(),
      is_frozen: msg.is_frozen,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      trajectory_id: msg.trajectory_id,
      submap_index: msg.submap_index,
      submap_version: msg.submap_version,
      pose: geometry_msgs::msg::Pose::from_rmw_message(msg.pose),
      is_frozen: msg.is_frozen,
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__SubmapList
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub submap: Vec<super::msg::SubmapEntry>,

}



impl Default for SubmapList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SubmapList::default())
  }
}

impl rosidl_runtime_rs::Message for SubmapList {
  type RmwMsg = super::msg::rmw::SubmapList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        submap: msg.submap
          .into_iter()
          .map(|elem| super::msg::SubmapEntry::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        submap: msg.submap
          .iter()
          .map(|elem| super::msg::SubmapEntry::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      submap: msg.submap
          .into_iter()
          .map(super::msg::SubmapEntry::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__SubmapTexture
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SubmapTexture {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cells: Vec<u8>,


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
    pub slice_pose: geometry_msgs::msg::Pose,

}



impl Default for SubmapTexture {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SubmapTexture::default())
  }
}

impl rosidl_runtime_rs::Message for SubmapTexture {
  type RmwMsg = super::msg::rmw::SubmapTexture;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cells: msg.cells.as_slice().into(),
        width: msg.width,
        height: msg.height,
        resolution: msg.resolution,
        slice_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.slice_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cells: msg.cells.as_slice().into(),
      width: msg.width,
      height: msg.height,
      resolution: msg.resolution,
        slice_pose: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.slice_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cells: msg.cells.into(),
      width: msg.width,
      height: msg.height,
      resolution: msg.resolution,
      slice_pose: geometry_msgs::msg::Pose::from_rmw_message(msg.slice_pose),
    }
  }
}


// Corresponds to cartographer_ros_msgs__msg__TrajectoryStates
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryStates {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_id: Vec<i32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub trajectory_state: Vec<u8>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TrajectoryStates::default())
  }
}

impl rosidl_runtime_rs::Message for TrajectoryStates {
  type RmwMsg = super::msg::rmw::TrajectoryStates;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        trajectory_id: msg.trajectory_id.as_slice().into(),
        trajectory_state: msg.trajectory_state.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        trajectory_id: msg.trajectory_id.as_slice().into(),
        trajectory_state: msg.trajectory_state.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      trajectory_id: msg.trajectory_id.into(),
      trajectory_state: msg.trajectory_state.into(),
    }
  }
}


