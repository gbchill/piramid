use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, PoisonError};
use crate::error::{Result, ServerError};

pub trait LockHelper<T> {
    fn read_or_err(&self) -> Result<RwLockReadGuard<'_, T>>;
    fn write_or_err(&self) -> Result<RwLockWriteGuard<'_, T>>;
}

impl<T> LockHelper<T> for RwLock<T> {
    fn read_or_err(&self) -> Result<RwLockReadGuard<'_, T>> {
        self.read()
            .map_err(|e: PoisonError<_>| {
                ServerError::Internal(format!("Lock poisoned: {}", e)).into()
            })
    }

    fn write_or_err(&self) -> Result<RwLockWriteGuard<'_, T>> {
        self.write()
            .map_err(|e: PoisonError<_>| {
                ServerError::Internal(format!("Lock poisoned: {}", e)).into()
            })
    }
}
