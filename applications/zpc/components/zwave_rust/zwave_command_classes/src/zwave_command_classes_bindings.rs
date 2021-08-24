/* automatically generated by rust-bindgen 0.59.1 */
#![allow(deref_nullptr)]
pub type sl_status_t = u32;
#[doc = " Node mask"]
pub type zwave_nodemask_t = [u8; 501usize];
#[doc = " Z-Wave NodeID type"]
pub type zwave_node_id_t = u16;
pub type zwave_node_list_t = zwave_nodemask_t;
pub type zwave_multicast_group_id_t = u8;
pub type zwave_endpoint_id_t = u8;
pub type zwave_command_class_t = u16;
pub type datastore_attribute_id_t = u32;
pub type attribute_store_node_t = datastore_attribute_id_t;
#[doc = " RSSI value array used in zwapi_tx_report_t. Each value is an RSSI feedback"]
#[doc = " constant defined above."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rssi_val {
    pub incoming: [::std::os::raw::c_schar; 5usize],
}
#[test]
fn bindgen_test_layout_rssi_val() {
    assert_eq!(
        ::std::mem::size_of::<rssi_val>(),
        5usize,
        concat!("Size of: ", stringify!(rssi_val))
    );
    assert_eq!(
        ::std::mem::align_of::<rssi_val>(),
        1usize,
        concat!("Alignment of ", stringify!(rssi_val))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rssi_val>())).incoming as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rssi_val),
            "::",
            stringify!(incoming)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _S_ROUTE_LINK_ {
    pub from: u8,
    pub to: u8,
}
#[test]
fn bindgen_test_layout__S_ROUTE_LINK_() {
    assert_eq!(
        ::std::mem::size_of::<_S_ROUTE_LINK_>(),
        2usize,
        concat!("Size of: ", stringify!(_S_ROUTE_LINK_))
    );
    assert_eq!(
        ::std::mem::align_of::<_S_ROUTE_LINK_>(),
        1usize,
        concat!("Alignment of ", stringify!(_S_ROUTE_LINK_))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_S_ROUTE_LINK_>())).from as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_S_ROUTE_LINK_),
            "::",
            stringify!(from)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_S_ROUTE_LINK_>())).to as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(_S_ROUTE_LINK_),
            "::",
            stringify!(to)
        )
    );
}
pub type S_ROUTE_LINK = _S_ROUTE_LINK_;
#[repr(u32)]
#[doc = " Transport routing scheme state define definitions"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _E_ROUTING_SCHEME_ {
    ROUTINGSCHEME_IDLE = 0,
    #[doc = "< Direct"]
    ROUTINGSCHEME_DIRECT = 1,
    #[doc = "< ApplicationStaticRoute"]
    ROUTINGSCHEME_CACHED_ROUTE_SR = 2,
    #[doc = "< ResponseRoute/lastworkingRoute"]
    ROUTINGSCHEME_CACHED_ROUTE = 3,
    ROUTINGSCHEME_CACHED_ROUTE_NLWR = 4,
    #[doc = "< ReturnRoute/controllerAutoRoute"]
    ROUTINGSCHEME_ROUTE = 5,
    #[doc = "< directResort"]
    ROUTINGSCHEME_RESORT_DIRECT = 6,
    #[doc = "< Explore"]
    ROUTINGSCHEME_RESORT_EXPLORE = 7,
}
#[doc = " Transport routing scheme state define definitions"]
pub use self::_E_ROUTING_SCHEME_ as E_ROUTING_SCHEME;
#[doc = " @brief Detailed report and data about Z-Wave transmissions"]
#[doc = ""]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zwapi_tx_report {
    #[doc = "< Passed 10 ms ticks"]
    pub wTransmitTicks: u16,
    #[doc = "< Repeaters in route, zero for direct range"]
    pub bRepeaters: u8,
    #[doc = " rssi_values per hop for direct and routed frames. Contains repeaters + 1"]
    #[doc = " values."]
    pub rssi_values: rssi_val,
    pub bACKChannelNo: u8,
    pub bLastTxChannelNo: u8,
    pub bRouteSchemeState: E_ROUTING_SCHEME,
    pub pLastUsedRoute: [u8; 5usize],
    pub bRouteTries: u8,
    pub bLastFailedLink: S_ROUTE_LINK,
}
#[test]
fn bindgen_test_layout_zwapi_tx_report() {
    assert_eq!(
        ::std::mem::size_of::<zwapi_tx_report>(),
        24usize,
        concat!("Size of: ", stringify!(zwapi_tx_report))
    );
    assert_eq!(
        ::std::mem::align_of::<zwapi_tx_report>(),
        4usize,
        concat!("Alignment of ", stringify!(zwapi_tx_report))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).wTransmitTicks as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(wTransmitTicks)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).bRepeaters as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bRepeaters)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).rssi_values as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(rssi_values)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).bACKChannelNo as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bACKChannelNo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwapi_tx_report>())).bLastTxChannelNo as *const _ as usize
        },
        9usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bLastTxChannelNo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwapi_tx_report>())).bRouteSchemeState as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bRouteSchemeState)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).pLastUsedRoute as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(pLastUsedRoute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).bRouteTries as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bRouteTries)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwapi_tx_report>())).bLastFailedLink as *const _ as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(zwapi_tx_report),
            "::",
            stringify!(bLastFailedLink)
        )
    );
}
impl Default for zwapi_tx_report {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[doc = " @brief Detailed report and data about Z-Wave transmissions"]
#[doc = ""]
#[doc = ""]
pub type zwapi_tx_report_t = zwapi_tx_report;
impl rf_power_level_t {
    pub const MINIMUM_POWER: rf_power_level_t = rf_power_level_t::MINUS_9_DBM;
}
#[repr(u32)]
#[doc = " RF power level values used with zwapi_set_rf_power_level() and"]
#[doc = " zwapi_get_rf_power_level()"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rf_power_level_t {
    #[doc = "< Max power possible"]
    NORMAL_POWER = 0,
    #[doc = "< Normal power - 1dB (mapped to minus2dB)"]
    MINUS_1_DBM = 1,
    #[doc = "< Normal power - 2dB"]
    MINUS_2_DBM = 2,
    #[doc = "< Normal power - 3dB (mapped to minus4dB)"]
    MINUS_3_DBM = 3,
    #[doc = "< Normal power - 4dB"]
    MINUS_4_DBM = 4,
    #[doc = "< Normal power - 5dB (mapped to minus6dB)"]
    MINUS_5_DBM = 5,
    #[doc = "< Normal power - 6dB"]
    MINUS_6_DBM = 6,
    #[doc = "< Normal power - 7dB (mapped to minus8dB)"]
    MINUS_7_DBM = 7,
    #[doc = "< Normal power - 8dB"]
    MINUS_8_DBM = 8,
    #[doc = "< Normal power - 9dB (mapped to minus10dB)"]
    MINUS_9_DBM = 9,
}
#[doc = " @brief Handle that can be used for aborting ongoing transmissions"]
#[doc = " or identifying TX Queue elements."]
pub type zwave_tx_session_id_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zwave_tx_options {
    #[doc = " The \\ref zwave_tx_process will wait for the expected responses to a frame before"]
    #[doc = " transmitting the next frame. It will time out and resume TX operations after a"]
    #[doc = " recommended backoff time for the expected number of responses"]
    #[doc = " If S2 uses the VERIFY_DELIVERY mechanism, the number of expected responses should be"]
    #[doc = " equal or greater than 1."]
    #[doc = " If is possible to specify that multiple responses are expected."]
    #[doc = " In case of Supervision with status update flag, the number_of_responses should"]
    #[doc = " be set to 1, as only one frame will be returned immediately for sure, the other(s) one will"]
    #[doc = " come later after an arbitrary time."]
    #[doc = " If the use_parent_frame_options and valid_parent_session_id fields are set to true"]
    #[doc = " and a valid parent_session_id is provided in the parent_session_id, this value will"]
    #[doc = " be overwritten with the number_of_responses value of the parent."]
    pub number_of_responses: u8,
    #[doc = " Maximum time in ms this transmission is allowed to spend in queue waiting"]
    #[doc = " to be processed before it is dropped. Discard timeout of 0 means to never"]
    #[doc = " drop the frame."]
    #[doc = " If the use_parent_frame_options and valid_parent_session_id fields are set to true"]
    #[doc = " and a valid parent_session_id is provided in the parent_session_id, this value will"]
    #[doc = " be overwritten with the discard_timeout_ms value of the parent."]
    pub discard_timeout_ms: u32,
    #[doc = " Priority of transmission element. Frames with higher numbers are sent"]
    #[doc = " first."]
    #[doc = " If the valid_parent_session_id field is set to true and a valid parent_session_id is"]
    #[doc = " provided in the parent_session_id, this value will be overwritten with the"]
    #[doc = " qos_priority value of the parent + 1."]
    pub qos_priority: u32,
    #[doc = " This flag indicates if the message should be attempted to be sent without"]
    #[doc = " beaming or route resolution. This option should only be used for FL nodes."]
    #[doc = " If the transmission fails, the message will be re-queued and attempted"]
    #[doc = " again later."]
    pub fasttrack: bool,
    #[doc = " zwave_tx_session_id_t of the parent frame. A parent frame is a frame that caused"]
    #[doc = " this frame to be added to the queue. Child frames are sent before their parent."]
    pub parent_session_id: zwave_tx_session_id_t,
    #[doc = " This flag indicates if the parent_session_id field is supposed to contain"]
    #[doc = " a valid value."]
    #[doc = " If this field is set to true, qos_priority will be overwritten so that the"]
    #[doc = " frame is sent before the parent."]
    pub valid_parent_session_id: bool,
    #[doc = " This flag indicates if the number_of_responses and discard_timeout_ms"]
    #[doc = " fields should be determined from the parent_session_id"]
    #[doc = " If the valid_parent_session_id field is additionally set to true,"]
    #[doc = " the number_of_responses and discard_timeout_ms and will be overwritten."]
    pub use_parent_frame_options: bool,
    #[doc = " This flag indicates if the frame is to be sent as a test frame"]
    #[doc = " Test frame was intended to be used to test link reliability, the"]
    #[doc = " Z-Wave API will send a test frame without any routing and with 9600 kbit/s"]
    #[doc = " transmission speed. The payload will also be ignored."]
    pub is_test_frame: bool,
    #[doc = " This value indicates if the a test frame must be sent"]
    #[doc = " with a particular Tx Power. This value will be ignored if the"]
    #[doc = " is_test_frame flag is set to false."]
    pub rf_power: rf_power_level_t,
    #[doc = " This flag can be used for tracking multicast/singlecast follow-ups"]
    #[doc = " transmissions."]
    #[doc = " For Singlecast messages (remote.is_multicast = false), this must be set"]
    #[doc = " to ZWAVE_TX_INVALID_GROUP if the frame is not a follow-up frame, else to"]
    #[doc = " the group_id for which it is a follow-up."]
    #[doc = " For Multicast messages (remote.is_multicast = true), if this field is"]
    #[doc = " set with a group_id different than 0, the Tx Queue will use the"]
    #[doc = " TRANSMIT_OPTION_MULTICAST_AS_BROADCAST and the multicast will be"]
    #[doc = " sent as a \"singlecast to the broadcast destination\""]
    pub group_id: zwave_multicast_group_id_t,
    #[doc = " Is this the first Singlecast follow-up frame ?"]
    #[doc = " Only set this to true if queuing the first follow-up frame. User components"]
    #[doc = " SHOULD always set this to false and let the Tx Queue handle the singlecast"]
    #[doc = " follow-ups."]
    pub is_first_follow_up: bool,
    #[doc = " The TX Queue can automatically queue Follow-up messages following a"]
    #[doc = " multicast. If you wish to activate this functionality, set this field"]
    #[doc = " to true."]
    pub send_follow_ups: bool,
}
#[test]
fn bindgen_test_layout_zwave_tx_options() {
    assert_eq!(
        ::std::mem::size_of::<zwave_tx_options>(),
        40usize,
        concat!("Size of: ", stringify!(zwave_tx_options))
    );
    assert_eq!(
        ::std::mem::align_of::<zwave_tx_options>(),
        8usize,
        concat!("Alignment of ", stringify!(zwave_tx_options))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).number_of_responses as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(number_of_responses)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).discard_timeout_ms as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(discard_timeout_ms)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_tx_options>())).qos_priority as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(qos_priority)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_tx_options>())).fasttrack as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(fasttrack)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).parent_session_id as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(parent_session_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).valid_parent_session_id as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(valid_parent_session_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).use_parent_frame_options as *const _
                as usize
        },
        25usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(use_parent_frame_options)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_tx_options>())).is_test_frame as *const _ as usize },
        26usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(is_test_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_tx_options>())).rf_power as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(rf_power)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_tx_options>())).group_id as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(group_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).is_first_follow_up as *const _ as usize
        },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(is_first_follow_up)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_tx_options>())).send_follow_ups as *const _ as usize
        },
        34usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_tx_options),
            "::",
            stringify!(send_follow_ups)
        )
    );
}
impl Default for zwave_tx_options {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type zwave_tx_options_t = zwave_tx_options;
#[doc = " @brief Callback function of zwave_tx_send_data(), indicating the result of the operation"]
#[doc = ""]
#[doc = " This function is called when an element in the TX Queue has been transmitted"]
#[doc = " (or attempted to be transmitted) and indicates the status of the operation."]
#[doc = ""]
#[doc = " @param status  Indicates how the transmission operation was completed."]
#[doc = "                Refer for \\ref zwapi_transmit_complete_codes for details."]
#[doc = " @param tx_info zwapi_tx_report_t reported by the \\ref ZWAPI. It"]
#[doc = "                contains transmission details, refer to \\ref zwapi_tx_report_t."]
#[doc = " @param user    User pointer provided in \\ref zwave_tx_send_data()"]
pub type on_zwave_tx_send_data_complete_t = ::std::option::Option<
    unsafe extern "C" fn(
        status: u8,
        tx_info: *const zwapi_tx_report_t,
        user: *mut ::std::os::raw::c_void,
    ),
>;
#[doc = " @brief Describing receive parameters of a frame."]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct zwave_rx_receive_options_t {
    #[doc = " Received frame status flags"]
    #[doc = ""]
    #[doc = " - RECEIVE_STATUS_ROUTED_BUSY A response route is locked by the application"]
    #[doc = " - RECEIVE_STATUS_LOW_POWER Received at low output power level"]
    #[doc = " - RECEIVE_STATUS_TYPE_SINGLE Received a single cast frame"]
    #[doc = " - RECEIVE_STATUS_TYPE_BROAD Received a broadcast frame"]
    #[doc = " - RECEIVE_STATUS_TYPE_MULTI Received a multicast frame"]
    #[doc = " - RECEIVE_STATUS_TYPE_EXPLORE Received an explore frame"]
    #[doc = " - RECEIVE_STATUS_FOREIGN_FRAME The received frame is not addressed to this"]
    #[doc = "   node (Only valid in promiscuous mode)"]
    #[doc = " - RECEIVE_STATUS_FOREIGN_HOMEID The received frame is received from a"]
    #[doc = "   foreign HomeID. Only Controllers in Smart Start AddNode mode can receive this"]
    pub status_flags: u8,
    #[doc = " RSSI measurement of the received frame This is a signed 8-bit value."]
    #[doc = ""]
    #[doc = " Values from RSSI_RESERVED_START to 124 are reserved."]
    #[doc = " - All values below RSSI_RESERVED_START are received power in dBm."]
    #[doc = " - RSSI_NOT_AVAILABLE - RSSI measurement not available"]
    #[doc = " - RSSI_MAX_POWER_SATURATED - Receiver saturated. RSSI too high to measure precisely"]
    #[doc = " - RSSI_BELOW_SENSITIVITY - No signal detected. The RSSI is too low to measure precisely."]
    pub rssi: i8,
    #[doc = " Node list for z-wave multicast frames."]
    #[doc = ""]
    pub nodes_in_multicast: zwave_node_list_t,
}
#[test]
fn bindgen_test_layout_zwave_rx_receive_options_t() {
    assert_eq!(
        ::std::mem::size_of::<zwave_rx_receive_options_t>(),
        503usize,
        concat!("Size of: ", stringify!(zwave_rx_receive_options_t))
    );
    assert_eq!(
        ::std::mem::align_of::<zwave_rx_receive_options_t>(),
        1usize,
        concat!("Alignment of ", stringify!(zwave_rx_receive_options_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_rx_receive_options_t>())).status_flags as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_rx_receive_options_t),
            "::",
            stringify!(status_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<zwave_rx_receive_options_t>())).rssi as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_rx_receive_options_t),
            "::",
            stringify!(rssi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<zwave_rx_receive_options_t>())).nodes_in_multicast as *const _
                as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(zwave_rx_receive_options_t),
            "::",
            stringify!(nodes_in_multicast)
        )
    );
}
impl Default for zwave_rx_receive_options_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    #[doc = " @brief Initialize the Multi Channel Command Class (handler)"]
    #[doc = " *"]
    #[doc = " @returns SL_STATUS_OK if successful"]
    #[doc = " @returns SL_STATUS_FAIL if an error occurred"]
    pub fn zwave_command_class_multi_channel_init() -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Resolves a Non-secure NIF for an Endpoint ID > 0."]
    #[doc = ""]
    #[doc = " Refer to @ref attribute_resolver_function_t for details."]
    #[doc = ""]
    #[doc = " @param node                The attribute store node that is being resolved"]
    #[doc = " @param [out] frame         Pointer where to write the frame data to resolve"]
    #[doc = "                            the attribute"]
    #[doc = " @param [out] frame_length  Pointer where to write length of the frame data"]
    #[doc = "                            that has been writen to the frame pointer."]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK if the frame was written"]
    pub fn zwave_command_class_multi_channel_capability_get(
        node: attribute_store_node_t,
        frame: *mut u8,
        frame_length: *mut u16,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Handles incoming Multi Channel encapsulated commands"]
    #[doc = ""]
    #[doc = " @param connection_info Info about the connection properties of this frame."]
    #[doc = " @param rx_options Info about the transport properties of this frame."]
    #[doc = " @param frame_data The data payload of this frame."]
    #[doc = " @param frame_length The length of this frame."]
    #[doc = " @returns sl_status_t representing the outcome of receiving the frame"]
    #[doc = "          In case where the command is controlled, it should be set to SL_STATUS_OK,"]
    #[doc = "          even when encountering some parsing errors"]
    #[doc = "          In case where the command is supported, it should be set to the @ref sl_status_t"]
    #[doc = "          corresponding to the correct time Status code. Refer to @ref zwave_command_handler_t"]
    pub fn zwave_command_class_multi_channel_handler(
        connection_info: *const zwave_controller::zwave_controller_connection_info_t,
        rx_options: *const zwave_rx_receive_options_t,
        frame_data: *const u8,
        frame_length: u16,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Setup fixture for the Supervision Command Class."]
    #[doc = ""]
    #[doc = " This setup will register the Supervision command handler"]
    #[doc = " to the Z-Wave CC framework,"]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK if successful"]
    #[doc = " @returns SL_STATUS_FAIL if an error occurred"]
    pub fn zwave_command_class_supervision_init() -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Handle incoming Supervision encapsulated commands."]
    #[doc = ""]
    #[doc = " @param connection_info Info about the connection properties of this frame."]
    #[doc = " @param frame_data The data payload of this frame."]
    #[doc = " @param frame_length The length of this frame."]
    #[doc = " @returns sl_status_t representing the outcome of receiving the frame"]
    #[doc = "          In case where the command is controlled, it should be set to SL_STATUS_OK,"]
    #[doc = "          even when encountering some parsing errors"]
    #[doc = "          In case where the command is supported, it should be set to the @ref sl_status_t"]
    #[doc = "          corresponding to the correct Supervision Status code. Refer to @ref zwave_command_handler_t"]
    pub fn zwave_command_class_supervision_support_handler(
        connection_info: *const zwave_controller::zwave_controller_connection_info_t,
        frame_data: *const u8,
        frame_length: u16,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Take a frame and create a Supervision Session."]
    #[doc = ""]
    #[doc = " This function is used to transmit Z-Wave frames using Supervision"]
    #[doc = " encapsulation. The provided payload will be encapsulated in a"]
    #[doc = " Supervision session, then sent out."]
    #[doc = ""]
    #[doc = " The Supervision module will wait for a Supervision Report"]
    #[doc = " before it callbacks the application."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @param connection       Refer to @ref zwave_tx_send_data"]
    #[doc = " @param data_length      Refer to @ref zwave_tx_send_data"]
    #[doc = " @param data             Refer to @ref zwave_tx_send_data"]
    #[doc = " @param tx_options       Refer to @ref zwave_tx_send_data"]
    #[doc = " @param on_supervision_complete Refer to @ref zwave_tx_send_data."]
    #[doc = "                                Note: The status parameter"]
    #[doc = "                                in the on_supervision_complete callback"]
    #[doc = "                                will be the Supervision Report Status"]
    #[doc = "                                nd not the Transmit Status."]
    #[doc = " @param user             Refer to @ref zwave_tx_send_data"]
    #[doc = " @param session          Refer to @ref zwave_tx_send_data"]
    #[doc = ""]
    #[doc = " @returns"]
    #[doc = " - SUCCESS The transmission request has been accepted and callback will be"]
    #[doc = "           triggered when the operation is completed."]
    #[doc = " - FAILURE The transmission request has been rejected, callback will not"]
    #[doc = "           be called."]
    pub fn zwave_command_class_supervision_send_data(
        connection: *const zwave_controller::zwave_controller_connection_info_t,
        data_length: u16,
        data: *const u8,
        tx_options: *const zwave_tx_options_t,
        on_supervision_complete: on_zwave_tx_send_data_complete_t,
        user: *mut ::std::os::raw::c_void,
        session: *mut zwave_tx_session_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Abort a queued or ongoing transmission."]
    #[doc = ""]
    #[doc = " Refer to @ref zwave_tx_abort_transmission for a"]
    #[doc = " detailed description"]
    #[doc = ""]
    #[doc = " @param  session Refer to @ref zwave_tx_abort_transmission"]
    #[doc = ""]
    #[doc = " @returns refer to @ref zwave_tx_abort_transmission"]
    pub fn zwave_command_class_supervision_abort_send_data(
        session: zwave_tx_session_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Request that a node is \"Waked\" Up on demand"]
    #[doc = " at the next communication."]
    #[doc = ""]
    #[doc = " Note that the Supervision module will only ensure that"]
    #[doc = " the node supports Supervision Command Class, version 2"]
    #[doc = " It will not verify that it supports the Wake Up Command Class,"]
    #[doc = " version 3 or the Wake-Up on demand capability."]
    #[doc = ""]
    #[doc = " @param node_id The node ID of the node to Wake-Up on demand"]
    #[doc = ""]
    #[doc = " @returns sl_status indicating if the request was accepted."]
    pub fn zwave_command_class_supervision_wake_on_demand(node_id: zwave_node_id_t) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Cancel a request that a node is \"Waked\" Up on demand"]
    #[doc = " at the next communication."]
    #[doc = ""]
    #[doc = " @param node_id The node ID of the node that no longer is requested"]
    #[doc = " to Wake-Up on demand"]
    #[doc = ""]
    #[doc = " @returns sl_status"]
    #[doc = " - SL_STATUS_OK indicating if the node is not (or no longer) part"]
    #[doc = "   of the Wake On Demand node list."]
    pub fn zwave_command_class_supervision_stop_wake_on_demand(
        node_id: zwave_node_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Close a supervision session based on the Z-Wave Tx session ID"]
    #[doc = ""]
    #[doc = " @param tx_session_id   The Z-Wave TX session ID associated to the Supervision"]
    #[doc = "                        Session that must be closed."]
    #[doc = ""]
    #[doc = " @returns sl_status"]
    #[doc = " - SL_STATUS_OK if the Session ID was closed."]
    #[doc = " - SL_STATUS_NOT_FOUND if there was no session ID found for the"]
    #[doc = "                       Z-Wave TX Session ID."]
    pub fn zwave_command_class_supervision_close_session_by_tx_session(
        tx_session_id: zwave_tx_session_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Intitialize the Version Command Class control APIs"]
    #[doc = ""]
    #[doc = " This setup will register the Version Command Class handler"]
    #[doc = " to the Z-Wave CC framework, register rule to the \\ref attribute_resolver and"]
    #[doc = " callbacks to the \\ref attribute_store."]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK if successful"]
    #[doc = " @returns SL_STATUS_FAIL if an error occurred"]
    pub fn zwave_command_class_version_init() -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Setup fixture for @ref command_classes."]
    #[doc = ""]
    #[doc = " This setup function will call the setup/initialization functions for"]
    #[doc = " every command class in this module."]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK if successful"]
    #[doc = " @returns SL_STATUS_FAIL if an error occurred"]
    pub fn zwave_command_classes_init() -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Find the Z-Wave Endpoint ID Node attribute based on a const"]
    #[doc = " zwave_controller::zwave_controller_connection_info_t object"]
    #[doc = ""]
    #[doc = " @param connection_info"]
    #[doc = " @return attribute_store_node_t"]
    pub fn zwave_command_class_get_endpoint_node(
        connection_info: *const zwave_controller::zwave_controller_connection_info_t,
    ) -> attribute_store_node_t;
}
extern "C" {
    #[doc = " @brief Find the Z-Wave NodeID Node attribute based on a const"]
    #[doc = " zwave_controller::zwave_controller_connection_info_t object"]
    #[doc = ""]
    #[doc = " @param connection_info"]
    #[doc = " @return attribute_store_node_t"]
    pub fn zwave_command_class_get_node_id_node(
        connection_info: *const zwave_controller::zwave_controller_connection_info_t,
    ) -> attribute_store_node_t;
}
extern "C" {
    #[doc = " @brief Gets the Attribute Store Endpoint ID node for a given"]
    #[doc = "        Z-Wave NodeID / Endpoint ID in our network"]
    #[doc = ""]
    #[doc = " @param node_id"]
    #[doc = " @param endpoint_id"]
    #[doc = " @return attribute_store_node_t"]
    pub fn zwave_command_class_get_endpoint_id_node(
        node_id: zwave_node_id_t,
        endpoint_id: zwave_endpoint_id_t,
    ) -> attribute_store_node_t;
}
extern "C" {
    #[doc = " @brief function to send report frames with default tx options."]
    #[doc = ""]
    #[doc = " @param connection_info Info about the connection properties of this frame."]
    #[doc = " @param report_size     The length of this frame."]
    #[doc = " @param report_data     The data payload of the report frame."]
    #[doc = ""]
    #[doc = " @returns sl_status_t representing the outcome of sending the report frame."]
    pub fn zwave_command_class_send_report(
        connection_info: *const zwave_controller::zwave_controller_connection_info_t,
        report_size: u16,
        report_data: *const u8,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief function helper function to help Command Classes decide if they should"]
    #[doc = " create the supporting node attributes on Command Class version attribute update."]
    #[doc = ""]
    #[doc = " @param command_class The Command Class that is to be verified"]
    #[doc = " @param updated_node  Attribute Store node that was updated."]
    #[doc = "                      (it can be anything under an endpoint.)"]
    #[doc = ""]
    #[doc = " @returns true if the handler must not create attributes and filter the command"]
    #[doc = " class. false if the handler must go ahead and create its attributes."]
    pub fn is_zwave_command_class_filtered_for_root_device(
        command_class: zwave_command_class_t,
        updated_node: attribute_store_node_t,
    ) -> bool;
}
