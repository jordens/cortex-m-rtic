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
        s2: u32, // shared with ceiling 2
        s3: u32, // shared with ceiling 3
        s4: u32, // shared with ceiling 4
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

        (
            Shared {
                s2: 0,
                s3: 0,
                s4: 0,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[idle(shared = [s2, s3])]
    fn idle(mut cx: idle::Context) -> ! {
        // interrupts are enabled again; the `UART0` handler runs at this point

        hprintln!("idle p0 started").ok();
        rtic::pend(Interrupt::GPIOC);
        cx.shared.s3.lock(|s| {
            hprintln!("idle enter lock s3 {}", s).ok();
            hprintln!("idle pend t0").ok();
            rtic::pend(Interrupt::GPIOA); // t0 p2, with shared ceiling 3
            hprintln!("idle pend t1").ok();
            rtic::pend(Interrupt::GPIOB); // t1 p3, with shared ceiling 3
            hprintln!("idle pend t2").ok();
            rtic::pend(Interrupt::GPIOC); // t2 p4, no sharing
            hprintln!("idle still in lock s3 {}", s).ok();
        });
        hprintln!("\nback in idle").ok();

        cx.shared.s2.lock(|s| {
            hprintln!("enter lock s2 {}", s).ok();
            hprintln!("idle pend t0").ok();
            rtic::pend(Interrupt::GPIOA); // t0 p2, with shared ceiling 2
            hprintln!("idle pend t1").ok();
            rtic::pend(Interrupt::GPIOB); // t1 p3, no sharing
            hprintln!("idle pend t2").ok();
            rtic::pend(Interrupt::GPIOC); // t2 p4, no sharing
            hprintln!("idle still in lock s2 {}", s).ok();
        });
        hprintln!("\nidle exit").ok();
        //  rtic::pend(Exception::SysTick);

        debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator

        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = GPIOA, priority = 2, local = [times: u32 = 0], shared = [s2, s3])]
    fn t0(cx: t0::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t0 p2 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .ok();
        hprintln!("t0 p2 exit").ok();
    }

    #[task(binds = GPIOB, priority = 3, local = [times: u32 = 0], shared = [s3, s4])]
    fn t1(mut cx: t1::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t1 p3 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .ok();

        cx.shared.s4.lock(|s| {
            hprintln!("t1 enter lock s4 {}", s).ok();
            hprintln!("t1 pend t0").ok();
            rtic::pend(Interrupt::GPIOA); // t0 p2, with shared ceiling 2
            hprintln!("t1 pend t2").ok();
            rtic::pend(Interrupt::GPIOC); // t2 p4, no sharing
            hprintln!("t1 still in lock s4 {}", s).ok();
        });

        hprintln!("t1 p3 exit").ok();
    }

    #[task(binds = GPIOC, priority = 4, local = [times: u32 = 0], shared = [s4])]
    fn t2(mut cx: t2::Context) {
        // Safe access to local `static mut` variable
        *cx.local.times += 1;

        hprintln!(
            "t2 p4 called {} time{}",
            *cx.local.times,
            if *cx.local.times > 1 { "s" } else { "" }
        )
        .unwrap();

        cx.shared.s4.lock(|s| {
            hprintln!("enter lock s4 {}", s).ok();
            *s += 1;
        });
        hprintln!("t3 p4 exit").ok();
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
