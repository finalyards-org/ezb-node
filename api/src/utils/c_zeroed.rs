

/**
* Continue filling any C struct with zeroes. Implements what the C compiler does, by default, for omitted fields.
*
* @usage
*   ```rust
*       some_struct_s {
*           a: 1,
*           ..c_zeroed()
*       }
*   ```
*/
#[inline(always)]
pub(crate) fn c_zeroed<T>() -> T {
    unsafe { core::mem::zeroed() }
}
