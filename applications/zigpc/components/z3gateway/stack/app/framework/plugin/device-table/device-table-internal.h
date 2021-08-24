/***************************************************************************//**
 * @file
 * @brief Internal APIs and defines for the Device Table plugin.
 *******************************************************************************
 * # License
 * <b>Copyright 2018 Silicon Laboratories Inc. www.silabs.com</b>
 *******************************************************************************
 *
 * The licensor of this software is Silicon Laboratories Inc. Your use of this
 * software is governed by the terms of Silicon Labs Master Software License
 * Agreement (MSLA) available at
 * www.silabs.com/about-us/legal/master-software-license-agreement. This
 * software is distributed to you in Source Code format and is governed by the
 * sections of the MSLA applicable to Source Code.
 *
 ******************************************************************************/

#ifndef SILABS_DEVICE_TABLE_INTERNAL_H
#define SILABS_DEVICE_TABLE_INTERNAL_H

uint8_t emAfDeviceTableGetFirstEndpointFromIndex(uint16_t index);

// Internal APIs for printing device information to the CLI
void emAfDeviceTableSave(void);
void emAfDeviceTableLoad(void);
void emAfDeviceTablePrintEUI64(const uint8_t *eui64);
void emAfDeviceTablePrintBuffer(const uint8_t *buffer, uint16_t bufLen);

// Internal APIs for route repair
void emAfDeviceTableInitiateRouteRepair(EmberNodeId nodeId);
bool emAfDeviceTableShouldDeviceLeave(EmberNodeId nodeId);

// Internal APIs for writing CIE address to new security sensor
void emAfDeviceTableSendCieAddressWrite(EmberNodeId nodeId, uint8_t endpoint);

uint16_t emAfFindFreeDeviceTableIndex(void);
void emAfDeviceTableInit(void);
void emAfDeviceTableUpdateNodeIdInEndpoints(EmberNodeId oldId, EmberNodeId newId);

void emAfPluginDeviceTableDeleteEntry(uint16_t index);


// new endpoint APIs
void emAfDeviceTableCopyDeviceTableEntry(uint16_t fromIndex, uint16_t toIndex);
uint8_t emAfDeviceTableNumberOfEndpointsFromIndex(uint16_t index);
EmberAfStatus emAfDeviceTableAddNewEndpoint(uint16_t index, uint8_t newEndpoint);
uint16_t emAfDeviceTableFindNextEndpoint(uint16_t index);
void emAfDeviceTableUpdateNodeId(uint16_t currentNodeId, uint16_t newNodeId);
uint16_t emAfDeviceTableFindIndexNodeIdEndpoint(uint16_t nodeId, uint8_t endpoint);
void emAfDeviceTableUpdateDeviceState(uint16_t index, uint8_t newState);
uint16_t emAfDeviceTableFindFreeDeviceTableIndex(void);
uint16_t emAfDeviceTableFindFirstEndpointNodeId(uint16_t nodeId);
uint16_t emAfDeviceTableFindFirstEndpointIeee(const EmberEUI64 eui64);

#define DEVICE_TABLE_UNKNOWN_ENDPOINT 0

#endif //__DEVICE_TABLE_H
