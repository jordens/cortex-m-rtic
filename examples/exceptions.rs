//! examples/hardware.rs

// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;

// const _A: u8 = lm3s6965::Interrupt::UART0 as u8;

#[rtic::app(device = lm3s6965)]
mod app {
    // use cortex_m::peripheral::Exception;

    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {
        s: u32,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        // Pends the UART0 interrupt but its handler won't run until *after*
        // `init` returns because interrupts are disabled
        // rtic::pend(Interrupt::UART0); // equivalent to NVIC::pend
        // cortex_m::peripheral::SCB::set_pendst();
        // cortex_m::peripheral::SCB::set_pendsv();
        // cortex_m::asm::svc();
        hprintln!("init").unwrap();

        (Shared { s: 0 }, Local {}, init::Monotonics())
    }

    #[idle(shared = [s])]
    fn idle(mut cx: idle::Context) -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle").unwrap();
        rtic::pend(Interrupt::GPIOC);
        cx.shared.s.lock(|s| {
            hprintln!("enter lock {}", s).ok();
            rtic::pend(Interrupt::GPIOA);
            rtic::pend(Interrupt::GPIOC); // t2

            // cortex_m::peripheral::SCB::set_pendsv();
            hprintln!("still in lock {}", s).ok();
        });
        //  rtic::pend(Exception::SysTick);

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = GPIOA, priority = 3, local = [times: u32 = 0], shared = [s])]
    fn t0(cx: t0::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t0 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .unwrap();
    }

    #[task(binds = GPIOB, priority = 2, local = [times: u32 = 0], shared = [s])]
    fn t1(cx: t1::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t1 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .unwrap();
    }

    #[task(binds = GPIOC, priority = 4, local = [times: u32 = 0])]
    fn t2(cx: t2::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t2 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .unwrap();
    }

    // #[task(binds = SysTick, priority = 2, local = [times: u32 = 0])]
    // fn systick(cx: systick::Context) {
    //     // Safe access to local `static mut` variable
    //     *cx.local.times += 1;

    //     hprintln!(
    //         "Systick called {} time{}",
    //         *cx.local.times,
    //         if *cx.local.times > 1 { "s" } else { "" }
    //     )
    //     .unwrap();
    // }

    // #[task(binds = PendSV, priority = 4, local = [times: u32 = 0])]
    // fn pend_sv(cx: pend_sv::Context) {
    //     // Safe access to local `static mut` variable
    //     *cx.local.times += 1;

    //     hprintln!(
    //         "PendSV called {} time{}",
    //         *cx.local.times,
    //         if *cx.local.times > 1 { "s" } else { "" }
    //     )
    //     .unwrap();
    // }

    // #[task(binds = SVCall, priority = 1, local = [times: u32 = 0])]
    // fn sv_call(cx: sv_call::Context) {
    //     // Safe access to local `static mut` variable
    //     *cx.local.times += 1;

    //     hprintln!(
    //         "PendSV called {} time{}",
    //         *cx.local.times,
    //         if *cx.local.times > 1 { "s" } else { "" }
    //     )
    //     .unwrap();
    // }
}
