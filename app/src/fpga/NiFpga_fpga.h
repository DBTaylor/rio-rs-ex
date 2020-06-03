/*
 * Generated with the FPGA Interface C API Generator 16.0.0
 * for NI-RIO 16.0.0 or later.
 */

#ifndef __NiFpga_fpga_h__
#define __NiFpga_fpga_h__

#ifndef NiFpga_Version
   #define NiFpga_Version 1600
#endif

#include "NiFpga.h"

/**
 * The filename of the FPGA bitfile.
 *
 * This is a #define to allow for string literal concatenation. For example:
 *
 *    static const char* const Bitfile = "C:\\" NiFpga_fpga_Bitfile;
 */
#define NiFpga_fpga_Bitfile "NiFpga_fpga.lvbitx"

/**
 * The signature of the FPGA bitfile.
 */
static const char* const NiFpga_fpga_Signature = "7100ED41CF70D00C35169EC9CB8882EA";

typedef enum
{
   NiFpga_fpga_IndicatorBool_bool_out_0 = 0x1802A,
   NiFpga_fpga_IndicatorBool_bool_out_1 = 0x18026,
   NiFpga_fpga_IndicatorBool_bool_out_2 = 0x18022,
   NiFpga_fpga_IndicatorBool_bool_out_3 = 0x1801E,
   NiFpga_fpga_IndicatorBool_bool_out_4 = 0x1801A,
   NiFpga_fpga_IndicatorBool_bool_out_5 = 0x18016,
   NiFpga_fpga_IndicatorBool_bool_out_6 = 0x18012,
   NiFpga_fpga_IndicatorBool_bool_out_7 = 0x1800E,
} NiFpga_fpga_IndicatorBool;

typedef enum
{
   NiFpga_fpga_IndicatorSgl_indicator_0 = 0x1805C,
   NiFpga_fpga_IndicatorSgl_indicator_1 = 0x1804C,
} NiFpga_fpga_IndicatorSgl;

typedef enum
{
   NiFpga_fpga_ControlBool_bool_0 = 0x1804A,
   NiFpga_fpga_ControlBool_bool_1 = 0x18046,
   NiFpga_fpga_ControlBool_bool_2 = 0x18042,
   NiFpga_fpga_ControlBool_bool_3 = 0x1803E,
   NiFpga_fpga_ControlBool_bool_4 = 0x1803A,
   NiFpga_fpga_ControlBool_bool_5 = 0x18036,
   NiFpga_fpga_ControlBool_bool_6 = 0x18032,
   NiFpga_fpga_ControlBool_bool_7 = 0x1802E,
} NiFpga_fpga_ControlBool;

typedef enum
{
   NiFpga_fpga_ControlSgl_control_0 = 0x18054,
   NiFpga_fpga_ControlSgl_control_1 = 0x18000,
} NiFpga_fpga_ControlSgl;

typedef enum
{
   NiFpga_fpga_IndicatorArrayBool_bool_arr_out = 0x18006,
} NiFpga_fpga_IndicatorArrayBool;

typedef enum
{
   NiFpga_fpga_IndicatorArrayBoolSize_bool_arr_out = 8,
} NiFpga_fpga_IndicatorArrayBoolSize;

typedef enum
{
   NiFpga_fpga_IndicatorArraySgl_arr_indicator = 0x18058,
} NiFpga_fpga_IndicatorArraySgl;

typedef enum
{
   NiFpga_fpga_IndicatorArraySglSize_arr_indicator = 2,
} NiFpga_fpga_IndicatorArraySglSize;

typedef enum
{
   NiFpga_fpga_ControlArrayBool_bool_arr = 0x1800A,
} NiFpga_fpga_ControlArrayBool;

typedef enum
{
   NiFpga_fpga_ControlArrayBoolSize_bool_arr = 8,
} NiFpga_fpga_ControlArrayBoolSize;

typedef enum
{
   NiFpga_fpga_ControlArraySgl_arr_control = 0x18050,
} NiFpga_fpga_ControlArraySgl;

typedef enum
{
   NiFpga_fpga_ControlArraySglSize_arr_control = 2,
} NiFpga_fpga_ControlArraySglSize;

typedef enum
{
   NiFpga_fpga_TargetToHostFifoSgl_target_to_host = 0,
} NiFpga_fpga_TargetToHostFifoSgl;

typedef enum
{
   NiFpga_fpga_HostToTargetFifoSgl_host_to_target = 1,
} NiFpga_fpga_HostToTargetFifoSgl;

#endif
