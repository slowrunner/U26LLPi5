
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_Goal__init(msg: *mut AudioNoteSequence_Goal) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Goal>);
    fn irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_Goal {
    /// Number of note_sequence iterations to play, defaults to 1
    pub iterations: i32,

    /// Sequence of notes to play per iteration
    pub note_sequence: super::super::msg::rmw::AudioNoteVector,

}

impl AudioNoteSequence_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const INFINITE: i32 = -1;

}


impl Default for AudioNoteSequence_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_Result__init(msg: *mut AudioNoteSequence_Result) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Result>);
    fn irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub complete: bool,

    /// Number of iterations of note sequence that were played
    pub iterations_played: i32,

    /// Runtime of audio playback
    pub runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for AudioNoteSequence_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_Feedback__init(msg: *mut AudioNoteSequence_Feedback) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Feedback>);
    fn irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub iterations_played: i32,

    /// Runtime of audio playback so far
    pub current_runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for AudioNoteSequence_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__init(msg: *mut AudioNoteSequence_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_FeedbackMessage>);
    fn irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::AudioNoteSequence_Feedback,

}



impl Default for AudioNoteSequence_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_Goal__init(msg: *mut Dock_Goal) -> bool;
    fn irobot_create_msgs__action__Dock_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_Goal>);
    fn irobot_create_msgs__action__Dock_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Dock_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_Result__init(msg: *mut Dock_Result) -> bool;
    fn irobot_create_msgs__action__Dock_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_Result>);
    fn irobot_create_msgs__action__Dock_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_docked: bool,

}



impl Default for Dock_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_Feedback__init(msg: *mut Dock_Feedback) -> bool;
    fn irobot_create_msgs__action__Dock_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_Feedback>);
    fn irobot_create_msgs__action__Dock_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sees_dock: bool,

}



impl Default for Dock_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_FeedbackMessage__init(msg: *mut Dock_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_FeedbackMessage>);
    fn irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Dock_Feedback,

}



impl Default for Dock_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_Goal__init(msg: *mut DriveArc_Goal) -> bool;
    fn irobot_create_msgs__action__DriveArc_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Goal>);
    fn irobot_create_msgs__action__DriveArc_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_Goal {
    /// Whether to arc forward or backward from robot's current position
    pub translate_direction: i8,

    /// Relative angle (radians) for robot to rotate along arc from current heading.  Angles greater than 2 PI will cause the robot to rotate in multiple circles
    pub angle: f32,

    /// Radius of arc (meters) for robot to drive along
    pub radius: f32,

    /// Max translation speed (positive m/s), will cap negative distance to negative speed
    pub max_translation_speed: f32,

}

impl DriveArc_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TRANSLATE_FORWARD: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const TRANSLATE_BACKWARD: i8 = -1;

}


impl Default for DriveArc_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_Result__init(msg: *mut DriveArc_Result) -> bool;
    fn irobot_create_msgs__action__DriveArc_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Result>);
    fn irobot_create_msgs__action__DriveArc_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for DriveArc_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_Feedback__init(msg: *mut DriveArc_Feedback) -> bool;
    fn irobot_create_msgs__action__DriveArc_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Feedback>);
    fn irobot_create_msgs__action__DriveArc_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_angle_travel: f32,

}



impl Default for DriveArc_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_FeedbackMessage__init(msg: *mut DriveArc_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_FeedbackMessage>);
    fn irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::DriveArc_Feedback,

}



impl Default for DriveArc_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_Goal__init(msg: *mut DriveDistance_Goal) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Goal>);
    fn irobot_create_msgs__action__DriveDistance_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_Goal {
    /// Drive a distance (meters) in a straight line
    pub distance: f32,

    /// Max translation speed (positive m/s), will cap negative distance to negative speed
    pub max_translation_speed: f32,

}



impl Default for DriveDistance_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_Result__init(msg: *mut DriveDistance_Result) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Result>);
    fn irobot_create_msgs__action__DriveDistance_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for DriveDistance_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_Feedback__init(msg: *mut DriveDistance_Feedback) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Feedback>);
    fn irobot_create_msgs__action__DriveDistance_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_travel_distance: f32,

}



impl Default for DriveDistance_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_FeedbackMessage__init(msg: *mut DriveDistance_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_FeedbackMessage>);
    fn irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::DriveDistance_Feedback,

}



impl Default for DriveDistance_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_Goal__init(msg: *mut LedAnimation_Goal) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Goal>);
    fn irobot_create_msgs__action__LedAnimation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_Goal {
    /// Animation to apply
    pub animation_type: i8,

    /// LED values to apply to animation
    pub lightring: super::super::msg::rmw::LightringLeds,

    /// Time to apply animation
    pub max_runtime: builtin_interfaces::msg::rmw::Duration,

}

impl LedAnimation_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const BLINK_LIGHTS: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SPIN_LIGHTS: i8 = 2;

}


impl Default for LedAnimation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_Result__init(msg: *mut LedAnimation_Result) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Result>);
    fn irobot_create_msgs__action__LedAnimation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for LedAnimation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_Feedback__init(msg: *mut LedAnimation_Feedback) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Feedback>);
    fn irobot_create_msgs__action__LedAnimation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for LedAnimation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_FeedbackMessage__init(msg: *mut LedAnimation_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_FeedbackMessage>);
    fn irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::LedAnimation_Feedback,

}



impl Default for LedAnimation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_Goal__init(msg: *mut NavigateToPosition_Goal) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Goal>);
    fn irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_Goal {
    /// Drive to goal position in odometry frame using simple rotate/translate/rotate approach
    pub goal_pose: geometry_msgs::msg::rmw::PoseStamped,

    /// Whether to achieve goal heading for final orientation or just use position
    pub achieve_goal_heading: bool,

    /// Max translation speed (positive m/s), will cap negative distance to negative speed
    pub max_translation_speed: f32,

    /// Max rotation speed (positive rad/s), will cap negative angle to negative speed
    pub max_rotation_speed: f32,

}



impl Default for NavigateToPosition_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_Result__init(msg: *mut NavigateToPosition_Result) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Result>);
    fn irobot_create_msgs__action__NavigateToPosition_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for NavigateToPosition_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_Feedback__init(msg: *mut NavigateToPosition_Feedback) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Feedback>);
    fn irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_Feedback {
    /// Which of the 3 phases above that robot is currently in
    pub navigate_state: i8,

    /// Remaining radians to rotate when robot is ROTATING_TO_GOAL_POSITION or ROTATING_TO_GOAL_ORIENTATION
    pub remaining_angle_travel: f32,

    /// How much distance is left to travel when robot is DRIVING_TO_GOAL_POSITION
    pub remaining_travel_distance: f32,

}

impl NavigateToPosition_Feedback {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const ROTATING_TO_GOAL_POSITION: i8 = 1;

    /// Whether robot is in second phase translating to to goal position
    pub const DRIVING_TO_GOAL_POSITION: i8 = 2;

    /// Whether robot is in third phase rotating to face goal orientation
    pub const ROTATING_TO_GOAL_ORIENTATION: i8 = 3;

}


impl Default for NavigateToPosition_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__init(msg: *mut NavigateToPosition_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_FeedbackMessage>);
    fn irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::NavigateToPosition_Feedback,

}



impl Default for NavigateToPosition_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_Goal__init(msg: *mut RotateAngle_Goal) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Goal>);
    fn irobot_create_msgs__action__RotateAngle_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_Goal {
    /// Rotate for relative angle (radians) from current robot position.  Angles greater than 2 PI will cause the robot to rotate in multiple circles
    pub angle: f32,

    /// Max rotation speed (positive rad/s), will cap negative angle to negative speed
    pub max_rotation_speed: f32,

}



impl Default for RotateAngle_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_Result__init(msg: *mut RotateAngle_Result) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Result>);
    fn irobot_create_msgs__action__RotateAngle_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::PoseStamped,

}



impl Default for RotateAngle_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_Feedback__init(msg: *mut RotateAngle_Feedback) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Feedback>);
    fn irobot_create_msgs__action__RotateAngle_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub remaining_angle_travel: f32,

}



impl Default for RotateAngle_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_FeedbackMessage__init(msg: *mut RotateAngle_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_FeedbackMessage>);
    fn irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::RotateAngle_Feedback,

}



impl Default for RotateAngle_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_Goal__init(msg: *mut Undock_Goal) -> bool;
    fn irobot_create_msgs__action__Undock_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_Goal>);
    fn irobot_create_msgs__action__Undock_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Undock_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_Result__init(msg: *mut Undock_Result) -> bool;
    fn irobot_create_msgs__action__Undock_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_Result>);
    fn irobot_create_msgs__action__Undock_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub is_docked: bool,

}



impl Default for Undock_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_Feedback__init(msg: *mut Undock_Feedback) -> bool;
    fn irobot_create_msgs__action__Undock_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_Feedback>);
    fn irobot_create_msgs__action__Undock_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Undock_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_FeedbackMessage__init(msg: *mut Undock_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_FeedbackMessage>);
    fn irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Undock_Feedback,

}



impl Default for Undock_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_FeedbackMessage() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Goal() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_Goal__init(msg: *mut WallFollow_Goal) -> bool;
    fn irobot_create_msgs__action__WallFollow_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Goal>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Goal>);
    fn irobot_create_msgs__action__WallFollow_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Goal>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub follow_side: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_runtime: builtin_interfaces::msg::rmw::Duration,

}

impl WallFollow_Goal {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FOLLOW_RIGHT: i8 = -1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const FOLLOW_LEFT: i8 = 1;

}


impl Default for WallFollow_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_Goal__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Goal() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Result() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_Result__init(msg: *mut WallFollow_Result) -> bool;
    fn irobot_create_msgs__action__WallFollow_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Result>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Result>);
    fn irobot_create_msgs__action__WallFollow_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Result>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub runtime: builtin_interfaces::msg::rmw::Duration,

}



impl Default for WallFollow_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_Result__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_Result where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Result() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_Feedback__init(msg: *mut WallFollow_Feedback) -> bool;
    fn irobot_create_msgs__action__WallFollow_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Feedback>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Feedback>);
    fn irobot_create_msgs__action__WallFollow_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_Feedback>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub engaged: bool,

}



impl Default for WallFollow_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_Feedback__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_Feedback() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_FeedbackMessage__init(msg: *mut WallFollow_FeedbackMessage) -> bool;
    fn irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_FeedbackMessage>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_FeedbackMessage>);
    fn irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_FeedbackMessage>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::WallFollow_Feedback,

}



impl Default for WallFollow_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_FeedbackMessage() }
  }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__init(msg: *mut AudioNoteSequence_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Request>);
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::AudioNoteSequence_Goal,

}



impl Default for AudioNoteSequence_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__init(msg: *mut AudioNoteSequence_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Response>);
    fn irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for AudioNoteSequence_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__init(msg: *mut AudioNoteSequence_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Request>);
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for AudioNoteSequence_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__init(msg: *mut AudioNoteSequence_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Response>);
    fn irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AudioNoteSequence_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteSequence_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::AudioNoteSequence_Result,

}



impl Default for AudioNoteSequence_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AudioNoteSequence_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__AudioNoteSequence_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AudioNoteSequence_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AudioNoteSequence_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/AudioNoteSequence_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_SendGoal_Request__init(msg: *mut Dock_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Request>);
    fn irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Dock_Goal,

}



impl Default for Dock_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_SendGoal_Response__init(msg: *mut Dock_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Response>);
    fn irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Dock_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_GetResult_Request__init(msg: *mut Dock_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__Dock_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Request>);
    fn irobot_create_msgs__action__Dock_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Dock_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Dock_GetResult_Response__init(msg: *mut Dock_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__Dock_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__Dock_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Response>);
    fn irobot_create_msgs__action__Dock_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Dock_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Dock_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Dock_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Dock_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Dock_Result,

}



impl Default for Dock_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Dock_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Dock_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Dock_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Dock_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Dock_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Dock_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Dock_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Dock_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_SendGoal_Request__init(msg: *mut DriveArc_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Request>);
    fn irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::DriveArc_Goal,

}



impl Default for DriveArc_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_SendGoal_Response__init(msg: *mut DriveArc_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Response>);
    fn irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for DriveArc_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_GetResult_Request__init(msg: *mut DriveArc_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Request>);
    fn irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for DriveArc_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveArc_GetResult_Response__init(msg: *mut DriveArc_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Response>);
    fn irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveArc_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveArc_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveArc_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveArc_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::DriveArc_Result,

}



impl Default for DriveArc_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveArc_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveArc_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveArc_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveArc_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveArc_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveArc_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveArc_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Request__init(msg: *mut DriveDistance_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Request>);
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::DriveDistance_Goal,

}



impl Default for DriveDistance_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Response__init(msg: *mut DriveDistance_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Response>);
    fn irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for DriveDistance_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_GetResult_Request__init(msg: *mut DriveDistance_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Request>);
    fn irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for DriveDistance_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__DriveDistance_GetResult_Response__init(msg: *mut DriveDistance_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Response>);
    fn irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DriveDistance_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DriveDistance_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::DriveDistance_Result,

}



impl Default for DriveDistance_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__DriveDistance_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__DriveDistance_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DriveDistance_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__DriveDistance_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DriveDistance_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DriveDistance_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/DriveDistance_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Request__init(msg: *mut LedAnimation_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Request>);
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::LedAnimation_Goal,

}



impl Default for LedAnimation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Response__init(msg: *mut LedAnimation_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Response>);
    fn irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for LedAnimation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_GetResult_Request__init(msg: *mut LedAnimation_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Request>);
    fn irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for LedAnimation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__LedAnimation_GetResult_Response__init(msg: *mut LedAnimation_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Response>);
    fn irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LedAnimation_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedAnimation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::LedAnimation_Result,

}



impl Default for LedAnimation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__LedAnimation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__LedAnimation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LedAnimation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__LedAnimation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LedAnimation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LedAnimation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/LedAnimation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__init(msg: *mut NavigateToPosition_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Request>);
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::NavigateToPosition_Goal,

}



impl Default for NavigateToPosition_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__init(msg: *mut NavigateToPosition_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Response>);
    fn irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for NavigateToPosition_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Request__init(msg: *mut NavigateToPosition_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Request>);
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for NavigateToPosition_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Response__init(msg: *mut NavigateToPosition_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Response>);
    fn irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<NavigateToPosition_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NavigateToPosition_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::NavigateToPosition_Result,

}



impl Default for NavigateToPosition_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__NavigateToPosition_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__NavigateToPosition_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for NavigateToPosition_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__NavigateToPosition_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for NavigateToPosition_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for NavigateToPosition_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/NavigateToPosition_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Request__init(msg: *mut RotateAngle_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Request>);
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::RotateAngle_Goal,

}



impl Default for RotateAngle_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Response__init(msg: *mut RotateAngle_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Response>);
    fn irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for RotateAngle_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_GetResult_Request__init(msg: *mut RotateAngle_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Request>);
    fn irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for RotateAngle_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__RotateAngle_GetResult_Response__init(msg: *mut RotateAngle_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Response>);
    fn irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<RotateAngle_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RotateAngle_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::RotateAngle_Result,

}



impl Default for RotateAngle_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__RotateAngle_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__RotateAngle_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for RotateAngle_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__RotateAngle_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for RotateAngle_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for RotateAngle_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/RotateAngle_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_SendGoal_Request__init(msg: *mut Undock_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Request>);
    fn irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Undock_Goal,

}



impl Default for Undock_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_SendGoal_Response__init(msg: *mut Undock_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Response>);
    fn irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Undock_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_GetResult_Request__init(msg: *mut Undock_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__Undock_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Request>);
    fn irobot_create_msgs__action__Undock_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Undock_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__Undock_GetResult_Response__init(msg: *mut Undock_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__Undock_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__Undock_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Response>);
    fn irobot_create_msgs__action__Undock_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Undock_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Undock_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__Undock_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Undock_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Undock_Result,

}



impl Default for Undock_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__Undock_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__Undock_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Undock_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__Undock_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Undock_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Undock_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/Undock_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__Undock_GetResult_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_SendGoal_Request__init(msg: *mut WallFollow_SendGoal_Request) -> bool;
    fn irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Request>);
    fn irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::WallFollow_Goal,

}



impl Default for WallFollow_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_SendGoal_Response__init(msg: *mut WallFollow_SendGoal_Response) -> bool;
    fn irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Response>);
    fn irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_SendGoal_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for WallFollow_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal_Response() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_GetResult_Request__init(msg: *mut WallFollow_GetResult_Request) -> bool;
    fn irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Request>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Request>);
    fn irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Request>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for WallFollow_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult_Request() }
  }
}


#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "irobot_create_msgs__rosidl_generator_c")]
extern "C" {
    fn irobot_create_msgs__action__WallFollow_GetResult_Response__init(msg: *mut WallFollow_GetResult_Response) -> bool;
    fn irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Response>, size: usize) -> bool;
    fn irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Response>);
    fn irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<WallFollow_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<WallFollow_GetResult_Response>) -> bool;
}

// Corresponds to irobot_create_msgs__action__WallFollow_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WallFollow_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::WallFollow_Result,

}



impl Default for WallFollow_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !irobot_create_msgs__action__WallFollow_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to irobot_create_msgs__action__WallFollow_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for WallFollow_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { irobot_create_msgs__action__WallFollow_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for WallFollow_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for WallFollow_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "irobot_create_msgs/action/WallFollow_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult_Response() }
  }
}






#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct AudioNoteSequence_SendGoal;

impl rosidl_runtime_rs::Service for AudioNoteSequence_SendGoal {
    type Request = AudioNoteSequence_SendGoal_Request;
    type Response = AudioNoteSequence_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__AudioNoteSequence_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct AudioNoteSequence_GetResult;

impl rosidl_runtime_rs::Service for AudioNoteSequence_GetResult {
    type Request = AudioNoteSequence_GetResult_Request;
    type Response = AudioNoteSequence_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__AudioNoteSequence_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Dock_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__Dock_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Dock_SendGoal;

impl rosidl_runtime_rs::Service for Dock_SendGoal {
    type Request = Dock_SendGoal_Request;
    type Response = Dock_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Dock_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Dock_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__Dock_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Dock_GetResult;

impl rosidl_runtime_rs::Service for Dock_GetResult {
    type Request = Dock_GetResult_Request;
    type Response = Dock_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Dock_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__DriveArc_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct DriveArc_SendGoal;

impl rosidl_runtime_rs::Service for DriveArc_SendGoal {
    type Request = DriveArc_SendGoal_Request;
    type Response = DriveArc_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveArc_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__DriveArc_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct DriveArc_GetResult;

impl rosidl_runtime_rs::Service for DriveArc_GetResult {
    type Request = DriveArc_GetResult_Request;
    type Response = DriveArc_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveArc_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct DriveDistance_SendGoal;

impl rosidl_runtime_rs::Service for DriveDistance_SendGoal {
    type Request = DriveDistance_SendGoal_Request;
    type Response = DriveDistance_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveDistance_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__DriveDistance_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct DriveDistance_GetResult;

impl rosidl_runtime_rs::Service for DriveDistance_GetResult {
    type Request = DriveDistance_GetResult_Request;
    type Response = DriveDistance_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__DriveDistance_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct LedAnimation_SendGoal;

impl rosidl_runtime_rs::Service for LedAnimation_SendGoal {
    type Request = LedAnimation_SendGoal_Request;
    type Response = LedAnimation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__LedAnimation_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__LedAnimation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct LedAnimation_GetResult;

impl rosidl_runtime_rs::Service for LedAnimation_GetResult {
    type Request = LedAnimation_GetResult_Request;
    type Response = LedAnimation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__LedAnimation_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateToPosition_SendGoal;

impl rosidl_runtime_rs::Service for NavigateToPosition_SendGoal {
    type Request = NavigateToPosition_SendGoal_Request;
    type Response = NavigateToPosition_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__NavigateToPosition_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__NavigateToPosition_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct NavigateToPosition_GetResult;

impl rosidl_runtime_rs::Service for NavigateToPosition_GetResult {
    type Request = NavigateToPosition_GetResult_Request;
    type Response = NavigateToPosition_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__NavigateToPosition_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct RotateAngle_SendGoal;

impl rosidl_runtime_rs::Service for RotateAngle_SendGoal {
    type Request = RotateAngle_SendGoal_Request;
    type Response = RotateAngle_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__RotateAngle_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__RotateAngle_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct RotateAngle_GetResult;

impl rosidl_runtime_rs::Service for RotateAngle_GetResult {
    type Request = RotateAngle_GetResult_Request;
    type Response = RotateAngle_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__RotateAngle_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Undock_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__Undock_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Undock_SendGoal;

impl rosidl_runtime_rs::Service for Undock_SendGoal {
    type Request = Undock_SendGoal_Request;
    type Response = Undock_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Undock_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Undock_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__Undock_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Undock_GetResult;

impl rosidl_runtime_rs::Service for Undock_GetResult {
    type Request = Undock_GetResult_Request;
    type Response = Undock_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__Undock_GetResult() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__WallFollow_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct WallFollow_SendGoal;

impl rosidl_runtime_rs::Service for WallFollow_SendGoal {
    type Request = WallFollow_SendGoal_Request;
    type Response = WallFollow_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__WallFollow_SendGoal() }
    }
}




#[link(name = "irobot_create_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to irobot_create_msgs__action__WallFollow_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct WallFollow_GetResult;

impl rosidl_runtime_rs::Service for WallFollow_GetResult {
    type Request = WallFollow_GetResult_Request;
    type Response = WallFollow_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__irobot_create_msgs__action__WallFollow_GetResult() }
    }
}


