/******************************************************************************
 * # License
 * <b>Copyright 2021 Silicon Laboratories Inc. www.silabs.com</b>
 ******************************************************************************
 * The licensor of this software is Silicon Laboratories Inc. Your use of this
 * software is governed by the terms of Silicon Labs Master Software License
 * Agreement (MSLA) available at
 * www.silabs.com/about-us/legal/master-software-license-agreement. This
 * software is distributed to you in Source Code format and is governed by the
 * sections of the MSLA applicable to Source Code.
 *
 *****************************************************************************/

#ifndef TRANSPORT_SERVICE_H
#define TRANSPORT_SERVICE_H

#include "stdint.h"
#include "stdbool.h"
#include "zwave_utils.h"
#ifdef __cplusplus
extern "C" {
#endif

typedef zwave_node_id_t ts_node_id_t;
#define COMMAND_CLASS_TRANSPORT_SERVICE_V2 0x55
/**
 * @brief
 * Transport service calls this callback to notify upper layers of
 * completion of transmit of whole payload 
 * @param status 0 for success 1 for failure
 * @param user user data to send back 
 */
typedef void (*on_transport_service_send_data_complete_t)(uint8_t status,
                                                          void *user);

/**
 * @brief
 * This callback is sent to lower layers and should be called when the 
 * transmission of what Transport service asked is either success or failure
 * @param status 0 for success 1 for failure
 * @param user user data to send back 
 */
typedef void (*on_lower_layer_send_data_complete_t)(uint8_t status, void *user);

/**
 * @brief
 * Lower layer Send Data function which Transport service should use for
 * transmitting broken down smaller fragments
 * @param source            Source node id
 * @param dest              Destination node id 
 * @param payload           payload.
 * @param payload_len       Length of payload.
 
 */
typedef uint8_t (*send_data_t)(
  ts_node_id_t source,
  ts_node_id_t dest,
  const uint8_t *payload,
  const uint16_t payload_len,
  const uint8_t no_of_expected_responses,
  const on_lower_layer_send_data_complete_t on_lower_layer_send_data_complete);

/**
 * @brief
 * This function is called when transport service is done receiving the 
 * whole payload and the payload needs to be hanlded by upper layer command
 * handlers 
 * @param source            Source node id
 * @param dest              Destination node id 
 * @param frame             payload.
 * @param frame_len         Length of payload.
 
 */
typedef void (*upper_layer_command_handler_t)(ts_node_id_t source,
                                              ts_node_id_t dest,
                                              const uint8_t *frame,
                                              uint16_t frame_data);

/**
 * @brief Initialize the Transport Service
 *
 * @param max_fragment_size Maximum fragment size that can be transmitted by
 *                          layers below 
 * @param on_transport_service_send_data_complete_t Callback 
 * @param send_data_t Underlying send Data 
 * @return 0 if successful 1 if an error occurred
 */
void transport_service_init(
  ts_node_id_t my_node_id,
  uint8_t max_fragment_size,
  const upper_layer_command_handler_t upper_layer_command_handler,
  const send_data_t send_data);

typedef enum { SINGLECAST, BROADCAST, MULTICAST } receive_type;

typedef enum {
  TRANSPORT_SERVICE_SEND_SUCCESS,
  TRANSPORT_SERVICE_SEND_FAILURE,
  TRANSPORT_SERVICE_BUSY,
  TRANSPORT_SERVICE_WILL_OVERFLOW
} transport_service_send_data_return_code_t;
/**
 * @brief Sending a long frame with Transport service 
 * 
 * This function will break the data payload and send it with Transport service
 * protocol
 * 
 * @param source            Source node id
 * @param dest              Destination node id 
 * @param payload           Payload 
 * @param payload_len       Length of payload.
 * @param max_fragment_size Maximum fragment size that can be transmitted by
 *                          layers below 
 * @return transport_service_send_data_return_code_t
 */
transport_service_send_data_return_code_t transport_service_send_data(
  ts_node_id_t source,
  ts_node_id_t dest,
  const uint8_t *payload,
  const uint16_t payload_len,
  const on_transport_service_send_data_complete_t on_send_complete);

/**
 * @brief Transport service RX function 
 * 
 * This function will assemble and handle the request of new frames to complete
 * the receiving of larger payloads with Transport service protocol 
 * 
 * @param source          Source node id 
 * @param dest            Destination node id 
 * @param rx_options      Receive options
 * @param frame_data      frame data
 * @param frame_length    frame length
 *
 * @return bool 
 *   - true on success
 *   - false on failure
 */
bool transport_service_on_frame_received(ts_node_id_t source,
                                         ts_node_id_t dest,
                                         receive_type rx_type,
                                         const uint8_t *frame_data,
                                         uint8_t frame_length);

#ifdef __cplusplus
}
#endif

#endif
