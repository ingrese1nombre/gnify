use crate::error::PersistenceError;

mod postgres;
pub use postgres::*;

pub trait Source: Sized {
    type Connection<'r>: 'r;

    fn read<BMC: Read<Self>>(
        &self,
        bmc: BMC,
    ) -> impl std::future::Future<Output = Result<BMC::Output, PersistenceError>> + Send;
    fn write<BMC: Write<Self>>(
        &self,
        bmc: BMC,
    ) -> impl std::future::Future<Output = Result<(), PersistenceError>> + Send;
}

pub trait BMC {
    type Output;
}

pub trait Read<S: Source>: BMC + Send {
    fn read(
        self,
        connection: S::Connection<'_>,
    ) -> impl std::future::Future<Output = Result<Self::Output, PersistenceError>> + Send;
}

pub trait Write<S: Source>: Send {
    fn write(
        self,
        connection: S::Connection<'_>,
    ) -> impl std::future::Future<Output = Result<(), PersistenceError>> + Send;
}
