/*
* Basic cluster
*
* Each 'Endpoint' has one, at index 0.
*/

use esp_zb_raw::esp_zb_zcl_basic_attr_t::*;

// tbd. Perhaps take attributes from a TOML file

// typedef enum {
//     ESP_ZB_ZCL_ATTR_BASIC_ZCL_VERSION_ID                  = 0x0000,                 /*!<ZCL version attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_APPLICATION_VERSION_ID          = 0x0001,                 /*!< Application version attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_STACK_VERSION_ID                = 0x0002,                 /*!< Stack version attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_HW_VERSION_ID                   = 0x0003,                 /*!< Hardware version attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID            = 0x0004,                 /*!< Manufacturer name attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID             = 0x0005,                 /*!< Model identifier attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_DATE_CODE_ID                    = 0x0006,                 /*!< Date code attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_POWER_SOURCE_ID                 = 0x0007,                 /*!< Power source attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_GENERIC_DEVICE_CLASS_ID         = 0x0008,                 /*!< The GenericDeviceClass attribute defines the field of application of the  GenericDeviceType attribute. */
//     ESP_ZB_ZCL_ATTR_BASIC_GENERIC_DEVICE_TYPE_ID          = 0x0009,                 /*!< The GenericDeviceType attribute allows an application to show an icon on a rich user interface (e.g. smartphone app). */
//     ESP_ZB_ZCL_ATTR_BASIC_PRODUCT_CODE_ID                 = 0x000a,                 /*!< The ProductCode attribute allows an application to specify a code for the product. */
//     ESP_ZB_ZCL_ATTR_BASIC_PRODUCT_URL_ID                  = 0x000b,                 /*!< The ProductURL attribute specifies a link to a web page containing specific product information. */
//     ESP_ZB_ZCL_ATTR_BASIC_MANUFACTURER_VERSION_DETAILS_ID = 0x000c,                 /*!< Vendor specific human readable (displayable) string representing the versions of one of more program images supported on the device. */
//     ESP_ZB_ZCL_ATTR_BASIC_SERIAL_NUMBER_ID                = 0x000d,                 /*!< Vendor specific human readable (displayable) serial number. */
//     ESP_ZB_ZCL_ATTR_BASIC_PRODUCT_LABEL_ID                = 0x000e,                 /*!< Vendor specific human readable (displayable) product label. */
//     ESP_ZB_ZCL_ATTR_BASIC_LOCATION_DESCRIPTION_ID         = 0x0010,                 /*!< Location description attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_PHYSICAL_ENVIRONMENT_ID         = 0x0011,                 /*!< Physical environment attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_DEVICE_ENABLED_ID               = 0x0012,                 /*!< Device enabled attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_ALARM_MASK_ID                   = 0x0013,                 /*!< Alarm mask attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_DISABLE_LOCAL_CONFIG_ID         = 0x0014,                 /*!< Disable local config attribute */
//     ESP_ZB_ZCL_ATTR_BASIC_SW_BUILD_ID                     = 0x4000                  /*!< Manufacturer-specific reference to the version of the software. */
// } esp_zb_zcl_basic_attr_t;

trait BasicCluster where Self: AnyCluster {}

impl BasicCluster {

    pub fn init() -> Self {

    }
}

// ESP_ZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID
const ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID =

//    ESP_ZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID

