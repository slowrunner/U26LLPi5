#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to irobot_create_msgs__msg__AudioNote
/// Play a note (frequency) for a specific duration

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNote {
    /// Frequency of note in Hz.
    pub frequency: u16,

    /// Time note should play for.
    pub max_runtime: builtin_interfaces::msg::Duration,

}



impl Default for AudioNote {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioNote::default())
  }
}

impl rosidl_runtime_rs::Message for AudioNote {
  type RmwMsg = super::msg::rmw::AudioNote;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frequency: msg.frequency,
        max_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.max_runtime)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      frequency: msg.frequency,
        max_runtime: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.max_runtime)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frequency: msg.frequency,
      max_runtime: builtin_interfaces::msg::Duration::from_rmw_message(msg.max_runtime),
    }
  }
}


// Corresponds to irobot_create_msgs__msg__AudioNoteVector
/// Play a sequence of audio notes.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AudioNoteVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub notes: Vec<super::msg::AudioNote>,

    /// If the notes should be appended or overwrite the currently playing notes.
    pub append: bool,

}



impl Default for AudioNoteVector {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::AudioNoteVector::default())
  }
}

impl rosidl_runtime_rs::Message for AudioNoteVector {
  type RmwMsg = super::msg::rmw::AudioNoteVector;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        notes: msg.notes
          .into_iter()
          .map(|elem| super::msg::AudioNote::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        append: msg.append,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        notes: msg.notes
          .iter()
          .map(|elem| super::msg::AudioNote::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      append: msg.append,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      notes: msg.notes
          .into_iter()
          .map(super::msg::AudioNote::from_rmw_message)
          .collect(),
      append: msg.append,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__Button
/// This message gives the status for a button

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Button {
    /// Header stamp is time and frame at which information was collected
    pub header: std_msgs::msg::Header,

    /// Whether or not button is depressed
    pub is_pressed: bool,

    /// The last time a button press was initiated
    pub last_start_pressed_time: builtin_interfaces::msg::Time,

    /// The duration that the button was depressed
    pub last_pressed_duration: builtin_interfaces::msg::Duration,

}



impl Default for Button {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Button::default())
  }
}

impl rosidl_runtime_rs::Message for Button {
  type RmwMsg = super::msg::rmw::Button;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_pressed: msg.is_pressed,
        last_start_pressed_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.last_start_pressed_time)).into_owned(),
        last_pressed_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Owned(msg.last_pressed_duration)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_pressed: msg.is_pressed,
        last_start_pressed_time: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.last_start_pressed_time)).into_owned(),
        last_pressed_duration: builtin_interfaces::msg::Duration::into_rmw_message(std::borrow::Cow::Borrowed(&msg.last_pressed_duration)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_pressed: msg.is_pressed,
      last_start_pressed_time: builtin_interfaces::msg::Time::from_rmw_message(msg.last_start_pressed_time),
      last_pressed_duration: builtin_interfaces::msg::Duration::from_rmw_message(msg.last_pressed_duration),
    }
  }
}


// Corresponds to irobot_create_msgs__msg__DockStatus
/// This message contains information about whether the robot is docked or sees the dock in its sensors.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DockStatus {
    /// Header stamp is when dock info was queried.
    pub header: std_msgs::msg::Header,

    /// Whether robot sees dock in its sensors used for docking
    pub dock_visible: bool,

    /// Whether the robot is docked
    pub is_docked: bool,

}



impl Default for DockStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::DockStatus::default())
  }
}

impl rosidl_runtime_rs::Message for DockStatus {
  type RmwMsg = super::msg::rmw::DockStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        dock_visible: msg.dock_visible,
        is_docked: msg.is_docked,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      dock_visible: msg.dock_visible,
      is_docked: msg.is_docked,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      dock_visible: msg.dock_visible,
      is_docked: msg.is_docked,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__HazardDetection
/// This message describes a detected hazard.
/// The frame ID allows to retrieve the location of the sensor that made the detection.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HazardDetection {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: u8,

}

impl HazardDetection {
    /// The robot has reached its backup limit. It will not drive further backward for safety reasons.
    /// You can disable this limit through the Create3 "safety_override" parameter by setting it
    /// to "backup_only" or "full".
    /// The Create3 webserver can be used to set a default value for the parameter.
    pub const BACKUP_LIMIT: u8 = 0;

    /// The robot has bumped against an obstacle
    pub const BUMP: u8 = 1;

    /// The robot detected a cliff
    pub const CLIFF: u8 = 2;

    /// The wheels of the robot are stalled against an obstacle
    pub const STALL: u8 = 3;

    /// The wheels of the robot are fully dropped
    pub const WHEEL_DROP: u8 = 4;

    /// The robot detects an obstacle in close proximity
    pub const OBJECT_PROXIMITY: u8 = 5;

}


impl Default for HazardDetection {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HazardDetection::default())
  }
}

impl rosidl_runtime_rs::Message for HazardDetection {
  type RmwMsg = super::msg::rmw::HazardDetection;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        type_: msg.type_,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      type_: msg.type_,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      type_: msg.type_,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__HazardDetectionVector
/// This message contains a vector of detected hazards.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HazardDetectionVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub detections: Vec<super::msg::HazardDetection>,

}



impl Default for HazardDetectionVector {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::HazardDetectionVector::default())
  }
}

impl rosidl_runtime_rs::Message for HazardDetectionVector {
  type RmwMsg = super::msg::rmw::HazardDetectionVector;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        detections: msg.detections
          .into_iter()
          .map(|elem| super::msg::HazardDetection::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        detections: msg.detections
          .iter()
          .map(|elem| super::msg::HazardDetection::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      detections: msg.detections
          .into_iter()
          .map(super::msg::HazardDetection::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to irobot_create_msgs__msg__InterfaceButtons
/// This message is the status of the 3 interface buttons on the Create faceplate

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceButtons {
    /// Header stamp is time at which information was collected
    pub header: std_msgs::msg::Header,

    /// Left button on faceplate marked with 1 dot
    pub button_1: super::msg::Button,

    /// Center Power button on faceplate
    pub button_power: super::msg::Button,

    /// Right button on faceplate marked with 2 dots
    pub button_2: super::msg::Button,

}



impl Default for InterfaceButtons {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::InterfaceButtons::default())
  }
}

impl rosidl_runtime_rs::Message for InterfaceButtons {
  type RmwMsg = super::msg::rmw::InterfaceButtons;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        button_1: super::msg::Button::into_rmw_message(std::borrow::Cow::Owned(msg.button_1)).into_owned(),
        button_power: super::msg::Button::into_rmw_message(std::borrow::Cow::Owned(msg.button_power)).into_owned(),
        button_2: super::msg::Button::into_rmw_message(std::borrow::Cow::Owned(msg.button_2)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        button_1: super::msg::Button::into_rmw_message(std::borrow::Cow::Borrowed(&msg.button_1)).into_owned(),
        button_power: super::msg::Button::into_rmw_message(std::borrow::Cow::Borrowed(&msg.button_power)).into_owned(),
        button_2: super::msg::Button::into_rmw_message(std::borrow::Cow::Borrowed(&msg.button_2)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      button_1: super::msg::Button::from_rmw_message(msg.button_1),
      button_power: super::msg::Button::from_rmw_message(msg.button_power),
      button_2: super::msg::Button::from_rmw_message(msg.button_2),
    }
  }
}


// Corresponds to irobot_create_msgs__msg__IrIntensity
/// This message provides the ir intensity readings

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrIntensity {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub value: i16,

}



impl Default for IrIntensity {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IrIntensity::default())
  }
}

impl rosidl_runtime_rs::Message for IrIntensity {
  type RmwMsg = super::msg::rmw::IrIntensity;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        value: msg.value,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      value: msg.value,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      value: msg.value,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__IrIntensityVector
/// This message provides the ir intensity readings

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrIntensityVector {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub readings: Vec<super::msg::IrIntensity>,

}



impl Default for IrIntensityVector {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IrIntensityVector::default())
  }
}

impl rosidl_runtime_rs::Message for IrIntensityVector {
  type RmwMsg = super::msg::rmw::IrIntensityVector;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        readings: msg.readings
          .into_iter()
          .map(|elem| super::msg::IrIntensity::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        readings: msg.readings
          .iter()
          .map(|elem| super::msg::IrIntensity::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      readings: msg.readings
          .into_iter()
          .map(super::msg::IrIntensity::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to irobot_create_msgs__msg__IrOpcode
/// This message describes a detected ir opcode.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IrOpcode {
    /// time when we saw the opcode
    pub header: std_msgs::msg::Header,

    /// opcode
    pub opcode: u8,

    /// the sensor which saw the opcode
    pub sensor: u8,

}

impl IrOpcode {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_FORCE_FIELD: u8 = 161;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_VIRTUAL_WALL: u8 = 162;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_GREEN: u8 = 164;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_RED: u8 = 168;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_BUOY_BOTH: u8 = 172;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_GREEN_FIELD: u8 = 244;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_RED_FIELD: u8 = 248;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const CODE_IR_EVAC_BOTH_FIELD: u8 = 252;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_OMNI: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const SENSOR_DIRECTIONAL_FRONT: u8 = 1;

}


impl Default for IrOpcode {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::IrOpcode::default())
  }
}

impl rosidl_runtime_rs::Message for IrOpcode {
  type RmwMsg = super::msg::rmw::IrOpcode;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        opcode: msg.opcode,
        sensor: msg.sensor,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      opcode: msg.opcode,
      sensor: msg.sensor,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      opcode: msg.opcode,
      sensor: msg.sensor,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__KidnapStatus
/// This message provides the robot kidnap status

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct KidnapStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_kidnapped: bool,

}



impl Default for KidnapStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::KidnapStatus::default())
  }
}

impl rosidl_runtime_rs::Message for KidnapStatus {
  type RmwMsg = super::msg::rmw::KidnapStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_kidnapped: msg.is_kidnapped,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_kidnapped: msg.is_kidnapped,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_kidnapped: msg.is_kidnapped,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__LedColor
/// This message is to set RGB values (0-255) for an LED

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LedColor {

    // This member is not documented.
    #[allow(missing_docs)]
    pub red: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub green: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub blue: u8,

}



impl Default for LedColor {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LedColor::default())
  }
}

impl rosidl_runtime_rs::Message for LedColor {
  type RmwMsg = super::msg::rmw::LedColor;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        red: msg.red,
        green: msg.green,
        blue: msg.blue,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      red: msg.red,
      green: msg.green,
      blue: msg.blue,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      red: msg.red,
      green: msg.green,
      blue: msg.blue,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__LightringLeds
/// This message sets the RGB values of the lightright LEDs

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LightringLeds {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// If you look at the lightring as an analog clock with 12 at the front
    /// of the robot
    /// led index 0 is 4 o'clock,
    /// led index 1 is 2 o'clock,
    /// led index 2 is 12 o'clock,
    /// led index 3 is 10 o'clock,
    /// led index 4 is 8 o'clock,
    /// led index 5 is 6 o'clock
    pub leds: [super::msg::LedColor; 6],

    /// Whether to override system's default light patterns with given command
    /// Set to false to return control of lights to system
    pub override_system: bool,

}



impl Default for LightringLeds {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LightringLeds::default())
  }
}

impl rosidl_runtime_rs::Message for LightringLeds {
  type RmwMsg = super::msg::rmw::LightringLeds;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        leds: msg.leds
          .map(|elem| super::msg::LedColor::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned()),
        override_system: msg.override_system,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        leds: msg.leds
          .iter()
          .map(|elem| super::msg::LedColor::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect::<Vec<_>>()
          .try_into()
          .unwrap(),
      override_system: msg.override_system,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      leds: msg.leds
        .map(super::msg::LedColor::from_rmw_message),
      override_system: msg.override_system,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__Mouse
/// This message contains a mouse raw measurement.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Mouse {
    /// Header stamp should be acquisition time of mouse measure.
    pub header: std_msgs::msg::Header,

    /// Motions in the X dimension in meters.
    pub integrated_x: f32,

    /// Motions in the Y dimension in meters.
    pub integrated_y: f32,

    /// Value incremented every time mouse stops tracking
    pub frame_id: u32,

    /// Surface quality level. It's proportional to the number of valid features visible by the sensor.
    pub last_squal: u8,

}



impl Default for Mouse {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Mouse::default())
  }
}

impl rosidl_runtime_rs::Message for Mouse {
  type RmwMsg = super::msg::rmw::Mouse;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        integrated_x: msg.integrated_x,
        integrated_y: msg.integrated_y,
        frame_id: msg.frame_id,
        last_squal: msg.last_squal,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      integrated_x: msg.integrated_x,
      integrated_y: msg.integrated_y,
      frame_id: msg.frame_id,
      last_squal: msg.last_squal,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      integrated_x: msg.integrated_x,
      integrated_y: msg.integrated_y,
      frame_id: msg.frame_id,
      last_squal: msg.last_squal,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__SlipStatus
/// This message provides the robot slip status

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SlipStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_slipping: bool,

}



impl Default for SlipStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::SlipStatus::default())
  }
}

impl rosidl_runtime_rs::Message for SlipStatus {
  type RmwMsg = super::msg::rmw::SlipStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_slipping: msg.is_slipping,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_slipping: msg.is_slipping,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_slipping: msg.is_slipping,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__StopStatus
/// This message provides the robot stopped status

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StopStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_stopped: bool,

}



impl Default for StopStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::StopStatus::default())
  }
}

impl rosidl_runtime_rs::Message for StopStatus {
  type RmwMsg = super::msg::rmw::StopStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        is_stopped: msg.is_stopped,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      is_stopped: msg.is_stopped,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      is_stopped: msg.is_stopped,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__WheelStatus
/// This message contains status on the wheels

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelStatus {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::Header,

    /// Current measurement for left wheel in milliamps
    pub current_ma_left: i16,

    /// Current measurement for right wheel in milliamps
    pub current_ma_right: i16,

    /// PWM % duty cycle measurement (where int16::max is +100%) for left wheel
    pub pwm_left: i16,

    /// PWM % duty cycle measurement (where int16::max is +100%) for right wheel
    pub pwm_right: i16,

    /// Whether wheels are enabled or disabled (disabled when E-Stopped)
    pub wheels_enabled: bool,

}



impl Default for WheelStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelStatus::default())
  }
}

impl rosidl_runtime_rs::Message for WheelStatus {
  type RmwMsg = super::msg::rmw::WheelStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        current_ma_left: msg.current_ma_left,
        current_ma_right: msg.current_ma_right,
        pwm_left: msg.pwm_left,
        pwm_right: msg.pwm_right,
        wheels_enabled: msg.wheels_enabled,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      current_ma_left: msg.current_ma_left,
      current_ma_right: msg.current_ma_right,
      pwm_left: msg.pwm_left,
      pwm_right: msg.pwm_right,
      wheels_enabled: msg.wheels_enabled,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      current_ma_left: msg.current_ma_left,
      current_ma_right: msg.current_ma_right,
      pwm_left: msg.pwm_left,
      pwm_right: msg.pwm_right,
      wheels_enabled: msg.wheels_enabled,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__WheelTicks
/// This message contains wheel ticks raw measurements.
/// It can be used both for publishing integrated as well as absolute encoder readings.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelTicks {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::Header,

    /// Count of encoder ticks for left wheel.
    pub ticks_left: i32,

    /// Count of encoder ticks for right wheel.
    pub ticks_right: i32,

}



impl Default for WheelTicks {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelTicks::default())
  }
}

impl rosidl_runtime_rs::Message for WheelTicks {
  type RmwMsg = super::msg::rmw::WheelTicks;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        ticks_left: msg.ticks_left,
        ticks_right: msg.ticks_right,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      ticks_left: msg.ticks_left,
      ticks_right: msg.ticks_right,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      ticks_left: msg.ticks_left,
      ticks_right: msg.ticks_right,
    }
  }
}


// Corresponds to irobot_create_msgs__msg__WheelVels
/// This message contains wheel velocity measurements.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WheelVels {
    /// Header stamp should be acquisition time of measure.
    pub header: std_msgs::msg::Header,

    /// Velocity measure for left wheel in rad/sec.
    pub velocity_left: f32,

    /// Velocity measure for right wheel in rad/sec.
    pub velocity_right: f32,

}



impl Default for WheelVels {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::WheelVels::default())
  }
}

impl rosidl_runtime_rs::Message for WheelVels {
  type RmwMsg = super::msg::rmw::WheelVels;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        velocity_left: msg.velocity_left,
        velocity_right: msg.velocity_right,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      velocity_left: msg.velocity_left,
      velocity_right: msg.velocity_right,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      velocity_left: msg.velocity_left,
      velocity_right: msg.velocity_right,
    }
  }
}


