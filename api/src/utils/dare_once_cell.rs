
use core::cell::OnceCell;

/**
* A 'OnceCell' implementation that *dares* to claim that it's thread safe. Use this only if you know the
* calls will be coming from the same thread.
*
* @note In the project, used only from the application thread/task.
*/
pub(crate) struct DareOnceCell<T>(OnceCell<T>);

impl<T> DareOnceCell<T> {
    pub const fn new() -> Self {
        Self(OnceCell::new())
    }
}

unsafe impl<T> Sync for DareOnceCell<T> {}

impl<T> DareOnceCell<T> {
    pub fn set(&self, value: T) -> Result<(), T> {
        self.0.set(value)
    }
    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }
}
