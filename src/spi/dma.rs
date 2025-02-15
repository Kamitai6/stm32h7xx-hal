//! # Serial Peripheral Interface - DMA

use crate::stm32::SPI1;
use crate::stm32::SPI2;
use crate::stm32::SPI3;
use crate::stm32::SPI4;
use crate::stm32::SPI5;
use crate::stm32::SPI6;

/// Marker for SPI TX Channel
pub struct TxChannel<SPI>(SPI);

/// Marker for SPI RX Channel
pub struct RxChannel<SPI>(SPI);

/// Trait to extend SPI peripherals
pub trait SpiDmaExt<SPI>: Sized {
    fn dma_tx(self) -> TxChannel<SPI>;
    fn dma_rx(self) -> RxChannel<SPI>;
}

macro_rules! spi_dma {
    ( $($SPIX:ident, $_Rec:ident: [$_spiX_tx:ident, $_spiX_rx:ident]),+ ) => {
        $(
            impl SpiDmaExt<$SPIX> for $SPIX {
                fn dma_tx(
                    self,
                ) -> TxChannel<$SPIX> {
                    TxChannel(self)
                }
                fn dma_rx(
                    self,
                ) -> RxChannel<$SPIX> {
                    RxChannel(self)
                }
            }
            impl core::ops::Deref for TxChannel<$SPIX> {
                type Target = $SPIX;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl core::ops::Deref for RxChannel<$SPIX> {
                type Target = $SPIX;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        )+
    }
}

spi_dma! {
    SPI1, Spi1: [spi1_tx, spi1_rx],
    SPI2, Spi2: [spi2_tx, spi2_rx],
    SPI3, Spi3: [spi3_tx, spi3_rx],
    SPI4, Spi4: [spi4_tx, spi4_rx],
    SPI5, Spi5: [spi5_tx, spi5_rx],
    SPI6, Spi6: [spi6_tx, spi6_rx]
}