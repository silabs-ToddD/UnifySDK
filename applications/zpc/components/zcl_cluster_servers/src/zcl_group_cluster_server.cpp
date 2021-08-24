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
//Include from this component
#include "zcl_group_cluster_server.h"
#include "zcl_group_cluster_server.hpp"
#include "zcl_cluster_servers_helpers.hpp"

// UIC components
#include "attribute_store.h"
#include "attribute_store_helper.h"
#include "uic_mqtt.h"
#include "sl_log.h"

// ZPC Components
#include "zpc_attribute_store.h"
#include "zpc_attribute_store_network_helper.h"
#include "attribute_store_defined_attribute_types.h"
#include "dotdot_attributes.h"
#include "zpc_dotdot_mqtt_group_dispatch.h"

//Generic includes
#include <string>
#include <map>
#include <vector>
#include <boost/algorithm/string.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/json_parser.hpp>
namespace bpt = boost::property_tree;

constexpr char LOG_TAG[] = "zcl_group_cluster_server";

////////////////////////////////////////////////////////////////////////////////
// Defines and types
////////////////////////////////////////////////////////////////////////////////
struct endpoint_cache_t {
  zwave_endpoint_id_t endpoint_id;
  std::string unid;
};

// Map of endpoints in our network, to handle deletion
std::map<attribute_store_node_t, endpoint_cache_t> endpoints_in_network;

// Constants
constexpr char GROUP_CLUSTER_NAME[]       = "Groups";
constexpr uint16_t GROUP_CLUSTER_REVISION = 1;

////////////////////////////////////////////////////////////////////////////////
// Local Functions
////////////////////////////////////////////////////////////////////////////////
static void on_endpoint_update(attribute_store_node_t endpoint_id_node,
                               attribute_store_change_t change);

////////////////////////////////////////////////////////////////////////////////
// ZCL Command handlers
////////////////////////////////////////////////////////////////////////////////
sl_status_t add_group(const std::string &unid,
                      zwave_endpoint_id_t endpoint_id,
                      uint16_t group_id,
                      const std::string &group_name)
{
  if (group_id == 0) {
    sl_log_info(LOG_TAG, "GroupID 0 is not valid. Ignoring command.");
    return SL_STATUS_FAIL;
  }

  attribute_store_node_t node_id_node
    = attribute_store_network_helper_get_node_id_node(unid.c_str());

  if (node_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t endpoint_id_node
    = attribute_store_get_node_child_by_value(node_id_node,
                                              ATTRIBUTE_ENDPOINT_ID,
                                              REPORTED_ATTRIBUTE,
                                              &endpoint_id,
                                              sizeof(zwave_endpoint_id_t),
                                              0);

  if (endpoint_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t group_id_node
    = attribute_store_get_node_child_by_value(
      endpoint_id_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
      REPORTED_ATTRIBUTE,
      reinterpret_cast<uint8_t *>(&group_id),
      sizeof(uint16_t),
      0);

  if (group_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    group_id_node
      = attribute_store_add_node(DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
                                 endpoint_id_node);
    attribute_store_set_node_attribute_value(
      group_id_node,
      REPORTED_ATTRIBUTE,
      reinterpret_cast<uint8_t *>(&group_id),
      sizeof(group_id));
  }

  attribute_store_node_t group_name_node
    = attribute_store_get_node_child_by_type(
      group_id_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_NAME,
      0);

  if (group_name_node == ATTRIBUTE_STORE_INVALID_NODE) {
    group_name_node
      = attribute_store_add_node(DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_NAME,
                                 group_id_node);
  }
  // Save the group name
  attribute_store_set_reported_string(group_name_node, group_name.c_str());

  // Make MQTT publications
  publish_group_list_cluster_attribute(endpoint_id_node);
  publish_group_name_cluster_attribute(group_name_node);
  return SL_STATUS_OK;
}

sl_status_t remove_group(const std::string &unid,
                         zwave_endpoint_id_t endpoint_id,
                         uint16_t group_id)
{
  attribute_store_node_t node_id_node
    = attribute_store_network_helper_get_node_id_node(unid.c_str());

  if (node_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t endpoint_id_node
    = attribute_store_get_node_child_by_value(node_id_node,
                                              ATTRIBUTE_ENDPOINT_ID,
                                              REPORTED_ATTRIBUTE,
                                              &endpoint_id,
                                              sizeof(zwave_endpoint_id_t),
                                              0);

  if (endpoint_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t group_id_node
    = attribute_store_get_node_child_by_value(
      endpoint_id_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
      REPORTED_ATTRIBUTE,
      reinterpret_cast<uint8_t *>(&group_id),
      sizeof(uint16_t),
      0);

  if (group_id_node != ATTRIBUTE_STORE_INVALID_NODE) {
    attribute_store_delete_node(group_id_node);
  }

  // Make MQTT publications
  unretain_group_name_publications(unid, endpoint_id, group_id);
  publish_group_list_cluster_attribute(endpoint_id_node);
  return SL_STATUS_OK;
}

sl_status_t remove_all_groups(const std::string &unid,
                              zwave_endpoint_id_t endpoint_id)
{
  attribute_store_node_t node_id_node
    = attribute_store_network_helper_get_node_id_node(unid.c_str());

  if (node_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t endpoint_id_node
    = attribute_store_get_node_child_by_value(node_id_node,
                                              ATTRIBUTE_ENDPOINT_ID,
                                              REPORTED_ATTRIBUTE,
                                              &endpoint_id,
                                              sizeof(zwave_endpoint_id_t),
                                              0);

  if (endpoint_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return SL_STATUS_NOT_FOUND;
  }

  attribute_store_node_t group_id_node = attribute_store_get_node_child_by_type(
    endpoint_id_node,
    DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
    0);

  while (group_id_node != ATTRIBUTE_STORE_INVALID_NODE) {
    uint16_t group_id = 0;
    attribute_store_read_value(group_id_node,
                               REPORTED_ATTRIBUTE,
                               reinterpret_cast<uint8_t *>(&group_id),
                               sizeof(group_id));

    attribute_store_delete_node(group_id_node);
    unretain_group_name_publications(unid, endpoint_id, group_id);

    group_id_node = attribute_store_get_node_child_by_type(
      endpoint_id_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
      0);
  }

  // Make a list publication only when we are done with deletion
  publish_group_list_cluster_attribute(endpoint_id_node);
  return SL_STATUS_OK;
}

///////////////////////////////////////////////////////////////////////////////
// MQTT callback functions
//////////////////////////////////////////////////////////////////////////////
void on_zcl_group_cluster_server_command_received(const char *topic,
                                                  const char *message,
                                                  size_t message_length)
{
  std::vector<std::string> topic_elements;
  boost::algorithm::split(topic_elements, topic, boost::is_any_of("/"));
  std::string unid                = topic_elements[2];
  zwave_endpoint_id_t endpoint_id = 0;
  if (0 == sscanf(topic_elements[3].c_str(), "ep%2" SCNu8, &endpoint_id)) {
    return;
  }

  if (is_zpc_unid(unid.c_str())) {
    sl_log_info(LOG_TAG,
                "Group command received for the ZPC's UNID. Ignoring.");
    return;
  }

  std::string command = topic_elements.back();

  if (command == "AddGroup") {
    // Parse the message. Mandatory fields are GroupId and GroupName
    try {
      std::stringstream ss;
      bpt::ptree pt;
      ss << std::string(message, message_length);
      bpt::json_parser::read_json(ss, pt);
      if (pt.count("GroupId") == 0) {
        sl_log_debug(LOG_TAG, "ZCL AddGroup:: Missing field: GroupId\n");
        return;
      } else if (pt.count("GroupName") == 0) {
        sl_log_debug(LOG_TAG, "ZCL AddGroup:: Missing field: GroupName\n");
        return;
      }

      // Add the group:
      add_group(unid,
                endpoint_id,
                pt.get<uint16_t>("GroupId"),
                pt.get<std::string>("GroupName"));
    } catch (const std::exception &exception) {
      sl_log_info(LOG_TAG,
                  "ZCL AddGroup Command: Unable to parse JSON payload. %s\n",
                  exception.what());
      return;
    }
  } else if (command == "ViewGroup") {
    // publish the group again?
  } else if (command == "GetGroupMembership") {
    // publish the group again?
  } else if (command == "RemoveGroup") {
    try {
      std::stringstream ss;
      bpt::ptree pt;
      ss << std::string(message, message_length);

      bpt::json_parser::read_json(ss, pt);
      if (pt.count("GroupId") == 0) {
        sl_log_debug(LOG_TAG, "ZCL RemoveGroup:: Missing field: GroupId\n");
        return;
      }
      // remove the group:
      remove_group(unid, endpoint_id, pt.get<uint16_t>("GroupId"));
    } catch (const std::exception &exception) {
      sl_log_info(LOG_TAG,
                  "ZCL RemoveGroup Command: Unable to parse JSON payload.%s\n",
                  exception.what());
      return;
    }
  } else if (command == "RemoveAllGroups") {
    remove_all_groups(unid, endpoint_id);
  } else if (command == "AddGroupIfIdentifying") {
    int32_t identify_time = 0;
    identify_time         = dotdot_get_identify_identify_time(unid.c_str(),
                                                      endpoint_id,
                                                      REPORTED_ATTRIBUTE);
    if (identify_time == 0) {
      return;
    }
    // The node is identifying. Same procedure as the Add Group command.
    try {
      std::stringstream ss;
      bpt::ptree pt;
      ss << std::string(message, message_length);
      bpt::json_parser::read_json(ss, pt);
      if (pt.count("GroupId") == 0) {
        sl_log_debug(LOG_TAG,
                     "ZCL AddGroupIfIdentifying:: Missing field: GroupId\n");
        return;
      } else if (pt.count("GroupName") == 0) {
        sl_log_debug(LOG_TAG,
                     "ZCL AddGroupIfIdentifying:: Missing field: GroupName\n");
        return;
      }

      // Add the group:
      add_group(unid,
                endpoint_id,
                pt.get<uint16_t>("GroupId"),
                pt.get<std::string>("GroupName"));
    } catch (const std::exception &exception) {
      sl_log_info(LOG_TAG,
                  "ZCL RemoveGroup Command: Unable to parse JSON payload.%s\n",
                  exception.what());
      return;
    }
  } else {
    sl_log_debug(LOG_TAG,
                 "Received unsupported ZCL Group Server command %s for unid "
                 "%s. Ignoring.",
                 command.c_str(),
                 unid.c_str());
  }
}

/**
 * @brief Receives commands on the ucl/by-group/+/Groups/Commands/+ space and
 * dispatchs them to invidual nodes.
 */
void on_zcl_by_group_group_cluster_server_command_received(
  const char *topic, const char *message, size_t message_length)
{
  if (message_length == 0) {
    return;
  }

  try {
    std::string topic_string(topic);
    std::vector<std::string> topic_elements;
    boost::algorithm::split(topic_elements,
                            topic_string,
                            boost::is_any_of("/"));

    uint16_t group_id = 0;
    if (0 == sscanf(topic_elements[2].c_str(), "%" SCNu16, &group_id)) {
      sl_log_debug(LOG_TAG,
                   "Error parsing Group ID from topic %s. Ignoring",
                   topic);
      return;
    };

    //Leverage the service provided by our ZPC dotdot MQTT group dispatch.
    zpc_dotdot_mqtt_group_dispatch(
      group_id,
      GROUP_CLUSTER_NAME,
      topic_elements.back().c_str(),
      message,
      message_length,
      &on_zcl_group_cluster_server_command_received);
  } catch (const std::exception &e) {
    sl_log_debug(LOG_TAG,
                 "Unable to parse MQTT Topic. Exception: %s\n",
                 e.what());
    return;
  }
}

///////////////////////////////////////////////////////////////////////////////
// MQTT Publication functions
//////////////////////////////////////////////////////////////////////////////
sl_status_t
  publish_group_name_cluster_attribute(attribute_store_node_t group_name_node)
{
  zwave_endpoint_id_t endpoint_id = 0;
  std::string unid;

  attribute_store_node_t endpoint_id_node
    = attribute_store_get_first_parent_with_type(group_name_node,
                                                 ATTRIBUTE_ENDPOINT_ID);

  if (SL_STATUS_OK
      != zcl_cluster_servers_get_unid_endpoint(endpoint_id_node,
                                               endpoint_id,
                                               unid)) {
    return SL_STATUS_FAIL;
  }

  attribute_store_node_t group_id_node
    = attribute_store_get_first_parent_with_type(
      group_name_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID);

  uint16_t group_id = 0;
  if (SL_STATUS_OK
      != attribute_store_read_value(group_id_node,
                                    REPORTED_ATTRIBUTE,
                                    &group_id,
                                    sizeof(uint16_t))) {
    return SL_STATUS_FAIL;
  }

  char group_name[ATTRIBUTE_STORE_MAXIMUM_VALUE_LENGTH] = {};
  uint8_t group_name_size                               = 0;
  if (SL_STATUS_OK
      != attribute_store_get_node_attribute_value(
        group_name_node,
        REPORTED_ATTRIBUTE,
        reinterpret_cast<uint8_t *>(group_name),
        &group_name_size)) {
    sl_log_debug(LOG_TAG, "Error reading group name value");
    return SL_STATUS_FAIL;
  }
  // We save the null termination in the attribute store, but better safe than sorry
  group_name[ATTRIBUTE_STORE_MAXIMUM_VALUE_LENGTH - 1] = '\0';

  std::string reported_topic
    = "ucl/by-unid/" + std::string(unid) + "/ep" + std::to_string(endpoint_id)
      + "/Groups/Attributes/" + std::to_string(group_id) + "/Name/Reported";
  std::string desired_topic
    = "ucl/by-unid/" + std::string(unid) + "/ep" + std::to_string(endpoint_id)
      + "/Groups/Attributes/" + std::to_string(group_id) + "/Name/Desired";

  std::string message = R"({"value": ")" + std::string(group_name) + R"("})";
  uic_mqtt_publish(desired_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  uic_mqtt_publish(reported_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  return SL_STATUS_OK;
}

sl_status_t
  publish_group_list_cluster_attribute(attribute_store_node_t endpoint_id_node)
{
  zwave_endpoint_id_t endpoint_id = 0;
  std::string unid;

  if (SL_STATUS_OK
      != zcl_cluster_servers_get_unid_endpoint(endpoint_id_node,
                                               endpoint_id,
                                               unid)) {
    return SL_STATUS_FAIL;
  }

  std::string reported_topic = "ucl/by-unid/" + std::string(unid) + "/ep"
                               + std::to_string(endpoint_id)
                               + "/Groups/Attributes/GroupList/Reported";
  std::string desired_topic = "ucl/by-unid/" + std::string(unid) + "/ep"
                              + std::to_string(endpoint_id)
                              + "/Groups/Attributes/GroupList/Desired";

  // Open the JSON array
  std::stringstream message_stream;
  message_stream << "{\"value\":";
  message_stream << "[";

  uint32_t child_index                 = 0;
  uint16_t group_id                    = 0;
  attribute_store_node_t group_id_node = attribute_store_get_node_child_by_type(
    endpoint_id_node,
    DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
    child_index);

  while (group_id_node != ATTRIBUTE_STORE_INVALID_NODE) {
    if (SL_STATUS_OK
        == attribute_store_read_value(group_id_node,
                                      REPORTED_ATTRIBUTE,
                                      &group_id,
                                      sizeof(uint16_t))) {
      if (child_index != 0) {
        message_stream << ", ";
      }
      message_stream << std::to_string(group_id);
    }
    child_index += 1;
    group_id_node = attribute_store_get_node_child_by_type(
      endpoint_id_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
      child_index);
  }

  // Close the JSON array
  message_stream << "]";
  message_stream << "}";

  std::string message = message_stream.str();
  // Publish the GroupList. Desired values first.
  uic_mqtt_publish(desired_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  uic_mqtt_publish(reported_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  return SL_STATUS_OK;
}

sl_status_t publish_name_support_cluster_attribute(
  attribute_store_node_t endpoint_id_node)
{
  zwave_endpoint_id_t endpoint_id = 0;
  std::string unid;

  if (SL_STATUS_OK
      != zcl_cluster_servers_get_unid_endpoint(endpoint_id_node,
                                               endpoint_id,
                                               unid)) {
    return SL_STATUS_FAIL;
  }

  std::string reported_topic = "ucl/by-unid/" + unid + "/ep"
                               + std::to_string(endpoint_id)
                               + "/Groups/Attributes/NameSupport/Reported";
  std::string desired_topic = "ucl/by-unid/" + unid + "/ep"
                              + std::to_string(endpoint_id)
                              + "/Groups/Attributes/NameSupport/Desired";

  // Always supported for all endpoints.
  std::string message = R"({"value":{"Supported":true}})";

  // Publish the GroupList. Desired values first.
  uic_mqtt_publish(desired_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  uic_mqtt_publish(reported_topic.c_str(),
                   message.c_str(),
                   message.size(),
                   true);
  return SL_STATUS_OK;
}

void publish_group_cluster_supported_commands(const std::string &unid,
                                              zwave_endpoint_id_t endpoint_id)
{
  // Do not publish any state supported commands for ourselves.
  if (true == is_zpc_unid(unid.c_str())) {
    return;
  }

  // All nodes support all commands here. We skip advertising the "read" commands
  // because they do not do anything
  std::string topic = "ucl/by-unid/" + unid + "/ep"
                      + std::to_string(endpoint_id)
                      + "/Groups/SupportedCommands";
  std::string message
    = R"({"value": ["AddGroup", "RemoveGroup", "RemoveAllGroups","AddGroupIfIdentifying"]})";
  uic_mqtt_publish(topic.c_str(), message.c_str(), message.size(), true);
}

void publish_group_cluster_cluster_revision(const std::string &unid,
                                            zwave_endpoint_id_t endpoint_id)
{
  // Do not publish any state supported commands for ourselves.
  if (true == is_zpc_unid(unid.c_str())) {
    return;
  }
  std::string revision_topic = "ucl/by-unid/" + unid + "/ep"
                               + std::to_string(endpoint_id)
                               + "/Groups/Attributes/ClusterRevision";
  std::string payload = std::string(R"({"value": )")
                        + std::to_string(GROUP_CLUSTER_REVISION)
                        + std::string("}");
  std::string pub_revision_topic_des = revision_topic + "/Desired";
  uic_mqtt_publish(pub_revision_topic_des.c_str(),
                   payload.c_str(),
                   payload.size(),
                   true);
  std::string pub_revision_topic_rep = revision_topic + "/Reported";
  uic_mqtt_publish(pub_revision_topic_rep.c_str(),
                   payload.c_str(),
                   payload.size(),
                   true);
}

void unretain_group_name_publications(const std::string &unid,
                                      zwave_endpoint_id_t endpoint_id,
                                      uint16_t group_id)
{
  std::string topic_prefix
    = "ucl/by-unid/" + unid + "/ep" + std::to_string(endpoint_id)
      + "/Groups/Attributes/" + std::to_string(group_id) + "/Name";
  uic_mqtt_unretain(topic_prefix.c_str());
}

static void
  unretain_all_group_cluster_publications(const std::string &unid,
                                          zwave_endpoint_id_t endpoint_id)
{
  std::string topic_prefix
    = "ucl/by-unid/" + unid + "/ep" + std::to_string(endpoint_id) + "/Groups";
  uic_mqtt_unretain(topic_prefix.c_str());
}

static void
  subscribe_to_node_group_cluster_commands(const std::string &unid,
                                           zwave_endpoint_id_t endpoint_id)
{
  std::string topic_filter = "ucl/by-unid/" + unid + "/ep"
                             + std::to_string(endpoint_id)
                             + "/Groups/Commands/+";
  uic_mqtt_subscribe(topic_filter.c_str(),
                     on_zcl_group_cluster_server_command_received);
}

/**
 * @brief subscribes to commands received in the ucl/by-group/ space for the Group Cluster
 */
static void subscribe_to_group_cluster_commands()
{
  const char topic_filter[] = "ucl/by-group/+/Groups/Commands/+";
  uic_mqtt_subscribe(topic_filter,
                     on_zcl_by_group_group_cluster_server_command_received);
}

///////////////////////////////////////////////////////////////////////////////
// Attribute Store Callback functions
//////////////////////////////////////////////////////////////////////////////
static void on_endpoint_update(attribute_store_node_t endpoint_id_node,
                               attribute_store_change_t change)
{
  if (change == ATTRIBUTE_DELETED) {
    auto it = endpoints_in_network.find(endpoint_id_node);
    if (it != endpoints_in_network.end()) {
      unretain_all_group_cluster_publications(it->second.unid,
                                              it->second.endpoint_id);
      endpoints_in_network.erase(it);
    }
    return;
  }

  if (attribute_store_is_value_defined(endpoint_id_node, REPORTED_ATTRIBUTE)) {
    auto it = endpoints_in_network.find(endpoint_id_node);
    if (it == endpoints_in_network.end()) {
      std::string unid;
      zwave_endpoint_id_t endpoint_id;
      if (SL_STATUS_OK
          != zcl_cluster_servers_get_unid_endpoint(endpoint_id_node,
                                                   endpoint_id,
                                                   unid)) {
        sl_log_warning(
          LOG_TAG,
          "Cannot read UNID/endpoint value for Attribute Store node %d",
          endpoint_id_node);
        return;
      }
      if (is_zpc_unid(unid.c_str())) {
        return;
      }
      // Publish Group Cluster for the endpoint
      publish_name_support_cluster_attribute(endpoint_id_node);
      publish_group_list_cluster_attribute(endpoint_id_node);
      publish_group_cluster_supported_commands(unid, endpoint_id);
      publish_group_cluster_cluster_revision(unid, endpoint_id);
      subscribe_to_node_group_cluster_commands(unid, endpoint_id);

      // Publish all the GroupNames for that endpoint (1..N groups)
      attribute_store_node_t group_id_node   = ATTRIBUTE_STORE_INVALID_NODE;
      attribute_store_node_t group_name_node = ATTRIBUTE_STORE_INVALID_NODE;
      size_t index                           = 0;
      do {
        group_id_node = attribute_store_get_node_child_by_type(
          endpoint_id_node,
          DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
          index++);
        group_name_node = attribute_store_get_node_child_by_type(
          group_id_node,
          DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_NAME,
          0);

        publish_group_name_cluster_attribute(group_name_node);
      } while (group_id_node != ATTRIBUTE_STORE_INVALID_NODE);

      // Add the endpoint to the cache:
      endpoint_cache_t cache = {};
      cache.unid             = unid;
      cache.endpoint_id      = endpoint_id;
      endpoints_in_network.insert(
        std::pair<attribute_store_node_t, endpoint_cache_t>(endpoint_id_node,
                                                            cache));
    }
  }
}

///////////////////////////////////////////////////////////////////////////////
// Public functions.
//////////////////////////////////////////////////////////////////////////////
bool is_unid_endpoint_in_group(const unid_t unid,
                               zwave_endpoint_id_t endpoint_id,
                               uint16_t group_id)
{
  attribute_store_node_t endpoint_node
    = attribute_store_network_helper_get_endpoint_node(unid, endpoint_id);

  attribute_store_node_t group_id_node
    = attribute_store_get_node_child_by_value(
      endpoint_node,
      DOTDOT_ATTRIBUTE_ID_GROUPS_GROUP_ID,
      REPORTED_ATTRIBUTE,
      reinterpret_cast<uint8_t *>(&group_id),
      sizeof(group_id),
      0);

  if (group_id_node == ATTRIBUTE_STORE_INVALID_NODE) {
    return false;
  } else {
    return true;
  }
}

///////////////////////////////////////////////////////////////////////////////
//  Init and teardown functions.
//////////////////////////////////////////////////////////////////////////////
sl_status_t zcl_group_cluster_server_init()
{
  // Updates the Group Cluster state of all endpoints in the network
  sl_status_t init_status = SL_STATUS_OK;

  init_status |= attribute_store_register_callback_by_type_and_state(
    on_endpoint_update,
    ATTRIBUTE_ENDPOINT_ID,
    REPORTED_ATTRIBUTE);

  // Subscribe to incoming commands in the ucl/by-group/# space
  subscribe_to_group_cluster_commands();

  return init_status;
}

void zcl_group_cluster_server_teardown()
{
  // Nothing to do, publications will be cleaned up automatically.
  for (auto it = endpoints_in_network.begin(); it != endpoints_in_network.end();
       ++it) {
    unretain_all_group_cluster_publications(it->second.unid,
                                            it->second.endpoint_id);
  }
  endpoints_in_network.clear();
}
