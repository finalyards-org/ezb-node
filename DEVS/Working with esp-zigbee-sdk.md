# Feedback on `esp-zigbee-sdk`


--
**Disclaimer**

The author has nothing but respect to a company providing open source libraries. 

The author is not requesting something, without collaborating back.

The sense of frustration, or negativity is not solely due to the library in question - but working **with AI** that is prone to hallucinations when a library is in flux and/or loosely documented.

Perhaps we, both users and authors of the library, can find ways to improve these aspects?

*- ASko*

---

## Case of `ezb_secur_set_ic_required()`

- TL;DR: <https://github.com/espressif/esp-zigbee-sdk/discussions/929>; you don't need it!

```
/**
 * @brief Enable Trust Center to require install code for joining.
 *
 * @param[in] required  A boolean indicating whether the install code is required for joining.
 *
 * @return - EZB_ERR_NONE: on success; failed otherwise.
 */
ezb_err_t ezb_secur_set_ic_required(bool required);
```

None of the library examples use it (2.0.5). 

The author was in the understanding the `install_code_policy` would be enough to decide, whether open keys (e.g. `"Zigbee09Alliance"`) would work, or actual device specific keys were needed for joining a network.

**Then..**

The AI - with its normal confidence - throws a ball, requesting to add a call to `ezb_secur_set_ic_required`.

It's not unfounded. The doc comment says, after all:

> * @brief Enable Trust Center to require install code for joining.

Turns out, however, that the [call is not needed](https://github.com/espressif/esp-zigbee-sdk/discussions/929) (GitHub Issues).

