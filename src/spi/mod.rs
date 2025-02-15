use core::marker::PhantomData;

pub mod dma;
pub use dma::SpiDmaExt;

mod spi;
pub use spi::*;