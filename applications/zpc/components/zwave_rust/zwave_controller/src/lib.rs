// License
// <b>Copyright 2021 Silicon Laboratories Inc. www.silabs.com</b>

// The licensor of this software is Silicon Laboratories Inc. Your use of this
// software is governed by the terms of Silicon Labs Master Software License
// Agreement (MSLA) available at
// www.silabs.com/about-us/legal/master-software-license-agreement. This
// software is distributed to you in Source Code format and is governed by the
// sections of the MSLA applicable to Source Code.
#![allow(
    missing_docs,
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    deref_nullptr
)]
// include!(concat!(env!("LIBUIC_DIR"), "/zwave_controller_bindings.rs"));
mod zwave_controller_bindings;
pub use zwave_controller_bindings::*;
