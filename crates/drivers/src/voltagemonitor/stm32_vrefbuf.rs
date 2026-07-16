use embassy_stm32::pac::{self, vrefbuf};

const VREFBUF_READY_TIMEOUT_SPINS: u32 = 1_000_000;

pub(super) fn enable_vrefbuf() -> bool {
    pac::RCC.apb3enr().modify(|w| {
        w.set_vrefen(true);
    });

    pac::VREFBUF.csr().modify(|w| {
        w.set_envr(false);
    });
    pac::VREFBUF.csr().modify(|w| {
        w.set_hiz(vrefbuf::vals::Hiz::CONNECTED);
        w.set_vrs(vrefbuf::vals::Vrs::VREF3);
        w.set_envr(true);
    });

    for _ in 0..VREFBUF_READY_TIMEOUT_SPINS {
        if pac::VREFBUF.csr().read().vrr() {
            return true;
        }
    }

    false
}
