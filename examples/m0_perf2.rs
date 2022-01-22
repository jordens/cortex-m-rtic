//! examples/m0_perf.rs

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
mod app {

    //  use cortex_m_semihosting::debug;
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {
        s1: u32, // shared with ceiling 1
        s2: u32, // shared with ceiling 2
        s3: u32, // shared with ceiling 3
        s4: u32, // shared with ceiling 4
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        (
            Shared {
                s1: 0,
                s2: 0,
                s3: 0,
                s4: 0,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[inline(never)]
    #[idle(shared = [s1, s2, s3])]
    fn idle(mut cx: idle::Context) -> ! {
        // rtic::pend(Interrupt::GPIOC);
        // cx.shared.s3.lock(|s| {
        //     *s += 1;
        // });

        // cx.shared.s2.lock(|_s| {
        //     // *s += 1;
        // });

        cx.shared.s1.lock(|_s| {
            // *s += 1;
        });

        // cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }

    // MASK = 0000_0001
    #[task(binds = GPIOA, priority = 2, local = [times: u32 = 0], shared = [s2, s3])]
    fn t0(cx: t0::Context) {
        *cx.local.times += 1;
    }

    // MASK = 0000_0010
    #[task(binds = GPIOB, priority = 3, local = [times: u32 = 0], shared = [s3, s4])]
    fn t1(mut cx: t1::Context) {
        *cx.local.times += 1;

        cx.shared.s4.lock(|s| {
            *s += 1;
        });
    }

    // MASK = 0000_0100
    #[task(binds = GPIOC, priority = 4, local = [times: u32 = 0], shared = [s4])]
    fn t2(mut cx: t2::Context) {
        *cx.local.times += 1;

        cx.shared.s4.lock(|s| {
            *s += 1;
        });
    }

    // MASK = 0000_1000
    #[task(binds = GPIOD, priority = 1, local = [times: u32 = 0], shared = [s1, s4])]
    fn t3(mut cx: t3::Context) {
        // *cx.local.times += 1;

        cx.shared.s4.lock(|s| {
            *s += 1;
        });
    }
}
