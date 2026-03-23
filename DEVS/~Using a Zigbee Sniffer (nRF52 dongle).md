# Using a Zigbee Sniffer (nRF52 dongle)

><img src=".images/nordic/nRF52840%20Dongle%20promo.png.webp" width=30% />


## Pre-condition

You have the `light` and `switch` nodes working, as described in [Demo with two boards](Demo_with_two_boards.md).

## Requirements

- A PC
- Nordic nRF52 dongle, or devkit

>Note: Somewhere, it was recommended with such dongles to use a ~30cm USB extension cable for less radio interference from the PC.

<!--
Author used:

- Sonoff ZBDongle-E firmware 8.0.2
- nRF... tbd.
-->

## Aim 🏹

To be able to follow the Zigbee radio traffic, helping understand the  protocol and debug your own projects.

## Preparation

Ensure both the Zigbee boards are on, and the `light` changes by pushing the BOOT button on the `switch` board.

>Note: We don't need data connection to the boards.


## Steps (nRF52 dongle; Windows 10 + WSL2)

>Based on: ["Installing nRF Sniffer for 802.15.4"](https://docs.nordicsemi.com/bundle/ug_sniffer_802154/page/UG/sniffer_802154/intro_802154.html) (Nordic docs; updated May'25)
>
>Hint: The instructions are best viewed as a PDF - there's a link on that page. The author got version 0.7.2.

Please pay attention to the original instructions (above); consider these as simply commentary on top of it.


### Requirements

To install Wireshark + nRF52 sniffing, you'll need to install a Wireshark addon, and that means Python.

- For Windows 10, you'll need to separately install it (the author did not)
- macOS and Linux have it, already

So:

- Python "3.10 or later"
- clone [nRF-Sniffer-for-802.15.4](https://github.com/NordicSemiconductor/nRF-Sniffer-for-802.15.4) to a suitable folder
	- this provides you with a `.hex` file you'll need for flashing the dongle
- Wireshark 4.0 or later

We'll go through the steps, one by one. The order might not be the same as in Nordic's doc.

### 1. Flash the Sniffer firmware

This is something you'll need anyways. 

The author did this within Windows, and WSL2 for the git clone.

**<strike>git clone</strike> fetch one file**

The vendor instructions ask you to `git clone` a repo, but all you really need is one file, from:

[https://github.com/NordicSemiconductor/nRF-Sniffer-for-802.15.4/tree/master/nrf802154_sniffer](https://github.com/NordicSemiconductor/nRF-Sniffer-for-802.15.4/tree/master/nrf802154_sniffer)

Pick the `nrf802154_sniffer_nrf52840dongle.hex` somewhere.

**Install nRF Connect for Desktop**

The author did this for Windows 10 Home.

>WARN: The installation is not too smooth. 
>
>- After installing the tool itself, it expects you to `Update` particular sub-applications. Do it?
>- The JLink installer (separate, but launched by the main one) brings a version that's not good enough for the tool. It points you to download the *real* latest version - which left the author with *two* JLink's (remove the previous one manually).
>
>It just feels... kind of a pile of hay.

Once you are past those hurdles (let's check!):

- got the `.hex` file
- got nRF Connect for Desktop installed

..[continue here](https://docs.nordicsemi.com/bundle/swtools_docs/page/app/pc-nrfconnect-programmer/programming_nrf52840_dongle.html) for the actual firmware writing instructions!

Note that: the `RESET` button is on the side of that switch. 

Got it flashed? Good!


### 2. Install and set up Wireshark 4

You'll need Python ("version 3.10 or later") for this, since there's a Wireshark add-on that needs Python.

- Windows 10 would need Python to be installed
- macOS (and Linux) have it

**2.1 Install Wireshark 4**

The author does not like to reveal, which OS they used. 

>Hint: you can install it under WSL2, where Python is easy to manage. If you do this, pass the USB stick via `usbipd attach --wsl -b {xxx}` from Windows to WSL (yet another install but might be useful).

- Add the key `"ZigbeeAlliance09"` - Espressif examples use it.<sup>`|2|`</sup>
- Edit the `Protocol` > `IEEE 802.15.4`: <sup>`|2|`</sup>

	![](.images/ieee_802_15_4.png)

**2.1.1 Install Wireshark add-on**

Vendor doc, chapter 3.2 ("Configuring Wireshark for Zigbee").

The Zigbee option is not available. "Capturing in
Wireshark requires installing an nRF Sniffer plugin." (head to chapter 4.2.1)






## References

- Nordic

	- `|1|`: ["nRF Sniffer for 802.15.4"](https://www.nordicsemi.com/Products/Development-tools/nRF-Sniffer-for-802154) (Nordic website)

		- [actual guidance](https://docs.nordicsemi.com/bundle/ug_sniffer_802154/page/UG/sniffer_802154/intro_802154.html) (Nordic docs; updated May'25)

- Espressif

	- `|2|`: Developing with ESP Zigbee SDK > Debugging > <br />[Sniffer and Wireshark](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/developing.html#sniffer-and-wireshark) (Espressif docs)

	>This e.g. shows that the Espressif examples use `"ZigbeeAlliance09"` key for packet encryption.	