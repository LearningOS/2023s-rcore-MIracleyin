//! Uniprocessor interior mutability primitives
use core::cell::{RefCell, RefMut};

/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.
pub struct UPSafeCell<T> { // 运行在单核上安全使用可变全局变量
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {} // 保证：1. 内核仅在单核上，无需在意多核情况 2. 提供运行时借用检查，满足基本约束，保证内存安全

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> { // 访问数据时，无论读写，都要调用这个方法，并在之后销毁标记，才能开始对该数据的下一次访问，不允许多个读操作同时存在
        self.inner.borrow_mut()
    }
}
