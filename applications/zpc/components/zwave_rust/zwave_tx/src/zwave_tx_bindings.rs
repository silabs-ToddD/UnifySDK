/* automatically generated by rust-bindgen 0.59.1 */

pub const ZWAVE_TX_QOS_MAX_PRIORITY: u32 = 4294967040;
pub const ZWAVE_TX_QOS_RECOMMENDED_TIMING_CRITICAL_PRIORITY: u32 = 268435455;
pub const ZWAVE_TX_QOS_RECOMMENDED_GET_ANSWER_PRIORITY: u32 = 16777215;
pub const ZWAVE_TX_QOS_RECOMMENDED_NODE_INTERVIEW_PRIORITY: u32 = 65535;
pub const ZWAVE_TX_QOS_RECOMMENDED_POLLING_PRIORITY: u32 = 4095;
pub const ZWAVE_TX_QOS_MIN_PRIORITY: u32 = 0;
pub const ZWAVE_TX_RECOMMENDED_QOS_GAP: u32 = 10;
pub type sl_status_t = u32;
#[doc = " Z-Wave NodeID type"]
pub type zwave_node_id_t = u16;
pub type zwave_multicast_group_id_t = u8;
pub type zwave_endpoint_id_t = u8;
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
extern "C" {
    #[doc = " @brief Queue and send frames to Z-Wave nodes."]
    #[doc = ""]
    #[doc = " This function is used to transmitting Z-Wave frames. The provided payload"]
    #[doc = " will be encapsulated into the format described in connection->encapsulation."]
    #[doc = " This module does not verify if the destination actually supports the requested"]
    #[doc = " encapsulation, or if the destination exists at all (NodeID / Endpoint)."]
    #[doc = ""]
    #[doc = " This module will put all frames on a queue. The frames with the highest"]
    #[doc = " qos_priority will be send first. the tx_options and connection parameters"]
    #[doc = " allow all kind of frames to be transmitted, including multicast/broadcast."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @param connection       Connection object describing the source and"]
    #[doc = "                         destination. If either the source or destination"]
    #[doc = "                         endpoints ID are not null, the frame will"]
    #[doc = "                         be Multi Channel encapsulated."]
    #[doc = " @param data_length      Length of the frame to send"]
    #[doc = " @param data             Points to the payload to send"]
    #[doc = " @param tx_options       Transmit options to use."]
    #[doc = " @param on_send_complete Callback function that will be called when the send"]
    #[doc = "                         operation has completed"]
    #[doc = " @param user             User pointer passed in argument of the on_send_complete"]
    #[doc = "                         callback function"]
    #[doc = " @param session          Pointer to location where to write the session id of"]
    #[doc = "                         the queued message. If this is set NULL the session id"]
    #[doc = "                         will not be written"]
    #[doc = ""]
    #[doc = " @returns"]
    #[doc = " - SL_STATUS_OK The transmission request has been accepted and callback will be"]
    #[doc = "                    triggered when the operation is completed."]
    #[doc = " - SL_STATUS_FAIL   The transmission request has been rejected, callback will not"]
    #[doc = "                    be called."]
    pub fn zwave_tx_send_data(
        connection: *const zwave_controller::zwave_controller_connection_info_t,
        data_length: u16,
        data: *const u8,
        tx_options: *const zwave_tx_options_t,
        on_send_complete: on_zwave_tx_send_data_complete_t,
        user: *mut ::std::os::raw::c_void,
        session: *mut zwave_tx_session_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Queues and send a test frame to a Z-Wave node."]
    #[doc = ""]
    #[doc = " This function is used for sending a special test frame that will be"]
    #[doc = " testing the direct range connectivity of a remote Z-Wave Node."]
    #[doc = ""]
    #[doc = " Test frame are automatically send with the lowest priority."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " @param destination_node_id   Z-Wave NodeID of the destination."]
    #[doc = " @param power_level           Powerlevel setting to apply for"]
    #[doc = "                              the transmission. Refer to \\ref rf_power_level_t"]
    #[doc = " @param on_send_complete      Callback function that will be called when"]
    #[doc = "                              the send operation is completed"]
    #[doc = " @param user                  User pointer passed in argument of"]
    #[doc = "                              the on_send_complete callback function"]
    #[doc = " @param session               Pointer to location where to write the"]
    #[doc = "                              session id of the queued message. If"]
    #[doc = "                              this is set NULL the session id will"]
    #[doc = "                              not be written"]
    #[doc = ""]
    #[doc = " @returns"]
    #[doc = " - SL_STATUS_OK The transmission request has been accepted and callback will be"]
    #[doc = "                    triggered when the operation is completed."]
    #[doc = " - SL_STATUS_FAIL   The transmission request has been rejected, callback will not"]
    #[doc = "                    be called."]
    pub fn zwave_tx_send_test_frame(
        destination_node_id: zwave_node_id_t,
        power_level: rf_power_level_t,
        on_send_complete: on_zwave_tx_send_data_complete_t,
        user: *mut ::std::os::raw::c_void,
        session: *mut zwave_tx_session_id_t,
    ) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Abort a queued or ongoing transmission."]
    #[doc = ""]
    #[doc = " Calling this function will attempt to cancel/abort a queued transmission."]
    #[doc = " If the transmission data has been passed to the Z-Wave module, it means that"]
    #[doc = " no further route resolution will be made, however the frame may still get"]
    #[doc = " transmitted and the callback of \\ref zwave_tx_send_data will still be called."]
    #[doc = " As a consequence the transmit status of on_zwave_tx_send_data_complete_t can"]
    #[doc = " be both TRANSMIT_COMPLETE_OK and TRANSMIT_COMPLETE_NO_ACK"]
    #[doc = ""]
    #[doc = " @param  session Session id returned by zwave_tx_send_data"]
    #[doc = ""]
    #[doc = " @returns"]
    #[doc = " - SL_STATUS_OK          if the frame transmission has been aborted"]
    #[doc = "                         (removed from the queue and not sent)"]
    #[doc = " - SL_STATUS_NOT_FOUND   if the session_id is not valid."]
    #[doc = " - SL_STATUS_IN_PROGRESS if the frame transmission was initiated and"]
    #[doc = "                         aborted. The on_send_complete callback will"]
    #[doc = "                         be called."]
    pub fn zwave_tx_abort_transmission(session: zwave_tx_session_id_t) -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Initialize the zwave_tx component."]
    #[doc = ""]
    #[doc = " Z-Wave Tx will register an on_frame_received() callback to the"]
    #[doc = " \\ref zwave_controller"]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK on success."]
    pub fn zwave_tx_init() -> sl_status_t;
}
extern "C" {
    #[doc = " @brief Stop the tx component."]
    #[doc = ""]
    #[doc = " This will stop ongoing transmission and flush all queues. No callbacks"]
    #[doc = " will be called when queues are emptied due to a shutdown."]
    #[doc = " It will also unregister the on_frame_received() callback to the"]
    #[doc = " \\ref zwave_controller"]
    #[doc = ""]
    pub fn zwave_tx_shutdown();
}
extern "C" {
    #[doc = " @brief Log the contents of the TX Queue."]
    #[doc = ""]
    #[doc = " @param  with_contents will print out the content of each"]
    #[doc = " queue element"]
    #[doc = ""]
    pub fn zwave_tx_log_queue(with_contents: bool);
}
extern "C" {
    #[doc = " Setup fixture for Z-Wave TX."]
    #[doc = ""]
    #[doc = " This setup function start the Z-Wave TX Process"]
    #[doc = ""]
    #[doc = " @returns SL_STATUS_OK if successful and SL_STATUS_FAIL if an error occurred"]
    pub fn zwave_tx_fixt_setup() -> sl_status_t;
}
