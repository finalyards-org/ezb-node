/*
* Other node exploration: finding and binding
*/

// tbd. Not sure how this ends up. We likely want to make it more generic, once it works for "color dimmable light device".

use ezn_node_raw::{

};

trait FindAndBindColorDimmableLightDevice {

    //static ezb_err_t zdo_find_ha_color_dimmable_light_device(void)
    // {
    //     ezb_err_t ret            = EZB_ERR_FAIL;
    //     uint16_t  cluster_list[] = {EZB_ZCL_CLUSTER_ID_ON_OFF, EZB_ZCL_CLUSTER_ID_LEVEL, EZB_ZCL_CLUSTER_ID_COLOR_CONTROL};
    //
    //     ezb_zdo_match_desc_req_t req = {
    //         .dst_nwk_addr = 0xFFFD,
    //         .field =
    //             {
    //                 .nwk_addr_of_interest = 0xFFFD,
    //                 .profile_id           = EZB_AF_HA_PROFILE_ID,
    //                 .num_in_clusters      = sizeof(cluster_list) / sizeof(cluster_list[0]),
    //                 .num_out_clusters     = 0,
    //                 .cluster_list         = cluster_list,
    //             },
    //         .cb       = zdo_find_ha_color_dimmable_light_device_result,
    //         .user_ctx = NULL,
    //     };
    //     ret = ezb_zdo_match_desc_req(&req);
    //     if (ret == EZB_ERR_NONE) {
    //         ESP_LOGI(TAG, "Attempt to find HA color dimmable light device");
    //     } else {
    //         ESP_LOGE(TAG, "Failed to find HA color dimmable light device with error(0x%04x)", ret);
    //     }
    //     return ret;
    // }
    async fn find_and_bind(&self) -> u64 {


    }

}
