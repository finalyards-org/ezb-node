# When adding more support

[Supported features](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/introduction.html#supported-features) tells the current support of `esp-zigbee-sdk`.

In particular:

- the ZCL clusters
- [HA device types](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/introduction.html#home-automation-device-types)

>You can also see the [sources directly](https://github.com/espressif/esp-zigbee-sdk/blob/main/components/esp-zigbee-console/src/zb_data/ha.c#L50).


## Comparison to commercial sensors

This project has a certain set of sensors in mind. Here are relevant ones, from Schneider Electric (Wiser) and Sonoff - and their existing support in `esp-zigbee-
sdk`.

| Valmistaja & Anturimalli | Koordinaattorin vaatimus paritukseen (Server Setup / Device Type) | Koordinaattorilta vaaditut Client-klusterit (Mistä otetaan vastaan) | Laitteelta luettavat Server-klusterit (Parituksen jälkeen) | Huomioitavaa / Erityispiirteet |
| :--- | :--- | :--- | :--- | :--- |
| **Schneider Wiser**<br>Ovi- / ikkunatunnistin [link](https://www.zigbee2mqtt.io/devices/CCT591011_AS.html) | Custom Device ID<br>*(IAS CIE -emulointi)* | `ias_zone` (Client) | `basic`, `power_config`, `identify`, `ias_zone` | Vaatii ehdottomasti CIE Zone Enroll -komennon vastaamisen, jotta raportoi tilanvaihdot. |
| **Schneider Wiser**<br>Liiketunnistin (PIR) | Custom Device ID<br>*(IAS CIE -emulointi)* | `ias_zone` (Client),<br>`illuminance_measurement` | `basic`, `power_config`, `identify`, `ias_zone`, `illuminance_measurement` | Sisältää usein valoisuusanturin samassa laitteessa. |
| **Schneider Wiser**<br>Vesivuotoanturi | Custom Device ID<br>*(IAS CIE -emulointi)* | `ias_zone` (Client) | `basic`, `power_config`, `ias_zone` | Raportoi vuodon IAS-hälytyksenä (Zone Status Change Notification). |
| **Sonoff** (SNZB-02 / SNZB-02D)<br>Lämpö- & kosteusanturi | `ESP_ZB_HA_TEMPERATURE_SENSOR_ID` | `temperature_measurement`,<br>`humidity_measurement` | `basic`, `power_config`, `temperature_measurement`, `humidity_measurement` | Erittäin helppo toteuttaa. Vakio HA-laitetyyppi riittää, ei vaadi IAS-käsittelyä. |
| **Sonoff** (SNZB-03)<br>Liiketunnistin | Custom Device ID<br>*(IAS CIE -emulointi)* | `ias_zone` (Client) | `basic`, `power_config`, `ias_zone` | Halpa perusanturi. Käyttää standardia IAS Zonea, toisin kuin monet muut kiinalaiset geneeriset anturit. |
| **Sonoff** (SNZB-04)<br>Ovi- / ikkunatunnistin | Custom Device ID<br>*(IAS CIE -emulointi)* | `ias_zone` (Client) | `basic`, `power_config`, `ias_zone` | Toimii samalla logiikalla kuin Wiserin ovianturi, mutta paristo kestää yleensä vähemmän aikaa.|

*Notes on Zigbee cluster use, source: google.ai (in Finnish)*



