#![feature(prelude_import)]
//! examples/hardware.rs
#![deny(warnings)]
#![no_main]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use panic_semihosting as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use lm3s6965 as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;
    /// User code from within the module
    /// User code end
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        ::cortex_m_semihosting::export::hstdout_str("init\n").unwrap();
        (Shared { s: 0 }, Local {}, init::Monotonics())
    }
    #[allow(non_snake_case)]
    fn idle(mut cx: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex_prelude::*;
        ::cortex_m_semihosting::export::hstdout_str("idle\n").unwrap();
        rtic::pend(Interrupt::SSI0);
        cx.shared.s.lock(|s| {
            ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
                &["enter lock ", "\n"],
                &match (&s,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
            .ok();
            rtic::pend(Interrupt::UART0);
            rtic::pend(Interrupt::SSI0);
            ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
                &["still in lock ", "\n"],
                &match (&s,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
            .ok();
        });
        debug::exit(debug::EXIT_SUCCESS);
        loop {
            cortex_m::asm::nop();
        }
    }
    #[allow(non_snake_case)]
    fn uart0(cx: uart0::Context) {
        use rtic::Mutex as _;
        use rtic::mutex_prelude::*;
        *cx.local.times += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
            &["UART0 called ", " time", "\n"],
            &match (
                &*cx.local.times,
                &if *cx.local.times > 1 { "s" } else { "" },
            ) {
                _args => [
                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                ],
            },
        ))
        .unwrap();
    }
    #[allow(non_snake_case)]
    fn uart1(cx: uart1::Context) {
        use rtic::Mutex as _;
        use rtic::mutex_prelude::*;
        *cx.local.times += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
            &["UART1 called ", " time", "\n"],
            &match (
                &*cx.local.times,
                &if *cx.local.times > 1 { "s" } else { "" },
            ) {
                _args => [
                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                ],
            },
        ))
        .unwrap();
    }
    #[allow(non_snake_case)]
    fn ssi0(cx: ssi0::Context) {
        use rtic::Mutex as _;
        use rtic::mutex_prelude::*;
        *cx.local.times += 1;
        ::cortex_m_semihosting::export::hstdout_fmt(::core::fmt::Arguments::new_v1(
            &["SSI0 called ", " time", "\n"],
            &match (
                &*cx.local.times,
                &if *cx.local.times > 1 { "s" } else { "" },
            ) {
                _args => [
                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                ],
            },
        ))
        .unwrap();
    }
    struct Shared {
        s: u32,
    }
    struct Local {}
    /// Monotonics used by the system
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_Monotonics();
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        /// Core (Cortex-M) peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals
        pub device: lm3s6965::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                device: lm3s6965::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        pub use super::__rtic_internal_Monotonics as Monotonics;
        pub use super::__rtic_internal_init_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `idle` has access to
    pub struct __rtic_internal_idleSharedResources<'a> {
        pub s: shared_resources::s_that_needs_to_be_locked<'a>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_idle_Context<'a> {
        /// Shared Resources this task has access to
        pub shared: idle::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_idle_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_idle_Context {
                shared: idle::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Idle loop
    pub mod idle {
        #[doc(inline)]
        pub use super::__rtic_internal_idleSharedResources as SharedResources;
        pub use super::__rtic_internal_idle_Context as Context;
    }
    mod shared_resources {
        use rtic::export::Priority;
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct s_that_needs_to_be_locked<'a> {
            priority: &'a Priority,
        }
        impl<'a> s_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a Priority) -> Self {
                s_that_needs_to_be_locked { priority }
            }
            #[inline(always)]
            pub unsafe fn priority(&self) -> &Priority {
                self.priority
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `uart0` has access to
    pub struct __rtic_internal_uart0LocalResources<'a> {
        pub times: &'a mut u32,
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `uart0` has access to
    pub struct __rtic_internal_uart0SharedResources<'a> {
        pub s: shared_resources::s_that_needs_to_be_locked<'a>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_uart0_Context<'a> {
        /// Local Resources this task has access to
        pub local: uart0::LocalResources<'a>,
        /// Shared Resources this task has access to
        pub shared: uart0::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_uart0_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_uart0_Context {
                local: uart0::LocalResources::new(),
                shared: uart0::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod uart0 {
        #[doc(inline)]
        pub use super::__rtic_internal_uart0LocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_uart0SharedResources as SharedResources;
        pub use super::__rtic_internal_uart0_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `uart1` has access to
    pub struct __rtic_internal_uart1LocalResources<'a> {
        pub times: &'a mut u32,
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `uart1` has access to
    pub struct __rtic_internal_uart1SharedResources<'a> {
        pub s: shared_resources::s_that_needs_to_be_locked<'a>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_uart1_Context<'a> {
        /// Local Resources this task has access to
        pub local: uart1::LocalResources<'a>,
        /// Shared Resources this task has access to
        pub shared: uart1::SharedResources<'a>,
    }
    impl<'a> __rtic_internal_uart1_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_uart1_Context {
                local: uart1::LocalResources::new(),
                shared: uart1::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod uart1 {
        #[doc(inline)]
        pub use super::__rtic_internal_uart1LocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_uart1SharedResources as SharedResources;
        pub use super::__rtic_internal_uart1_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `ssi0` has access to
    pub struct __rtic_internal_ssi0LocalResources<'a> {
        pub times: &'a mut u32,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_ssi0_Context<'a> {
        /// Local Resources this task has access to
        pub local: ssi0::LocalResources<'a>,
    }
    impl<'a> __rtic_internal_ssi0_Context<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_ssi0_Context {
                local: ssi0::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod ssi0 {
        #[doc(inline)]
        pub use super::__rtic_internal_ssi0LocalResources as LocalResources;
        pub use super::__rtic_internal_ssi0_Context as Context;
    }
    /// app module
    impl<'a> __rtic_internal_idleSharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_idleSharedResources {
                s: shared_resources::s_that_needs_to_be_locked::new(priority),
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_s: rtic::RacyCell<core::mem::MaybeUninit<u32>> =
        rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::s_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 3u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_s.get_mut() as *mut _,
                    self.priority(),
                    CEILING,
                    lm3s6965::NVIC_PRIO_BITS,
                    &MASKS,
                    f,
                )
            }
        }
    }
    const MASKS: [u32; 5] = [
        0,
        0,
        0 | 1 << lm3s6965::Interrupt::UART1 as u32,
        0 | 1 << lm3s6965::Interrupt::UART0 as u32,
        0 | 1 << lm3s6965::Interrupt::SSI0 as u32,
    ];
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_uart0_times: rtic::RacyCell<u32> = rtic::RacyCell::new(0);
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_uart1_times: rtic::RacyCell<u32> = rtic::RacyCell::new(0);
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    static __rtic_internal_local_ssi0_times: rtic::RacyCell<u32> = rtic::RacyCell::new(0);
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART0() {
        const PRIORITY: u8 = 3u8;
        rtic::export::run(PRIORITY, || {
            uart0(uart0::Context::new(&rtic::export::Priority::new(PRIORITY)))
        });
    }
    impl<'a> __rtic_internal_uart0LocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_uart0LocalResources {
                times: &mut *__rtic_internal_local_uart0_times.get_mut(),
            }
        }
    }
    impl<'a> __rtic_internal_uart0SharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_uart0SharedResources {
                s: shared_resources::s_that_needs_to_be_locked::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn UART1() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(PRIORITY, || {
            uart1(uart1::Context::new(&rtic::export::Priority::new(PRIORITY)))
        });
    }
    impl<'a> __rtic_internal_uart1LocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_uart1LocalResources {
                times: &mut *__rtic_internal_local_uart1_times.get_mut(),
            }
        }
    }
    impl<'a> __rtic_internal_uart1SharedResources<'a> {
        #[inline(always)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_uart1SharedResources {
                s: shared_resources::s_that_needs_to_be_locked::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn SSI0() {
        const PRIORITY: u8 = 4u8;
        rtic::export::run(PRIORITY, || {
            ssi0(ssi0::Context::new(&rtic::export::Priority::new(PRIORITY)))
        });
    }
    impl<'a> __rtic_internal_ssi0LocalResources<'a> {
        #[inline(always)]
        pub unsafe fn new() -> Self {
            __rtic_internal_ssi0LocalResources {
                times: &mut *__rtic_internal_local_ssi0_times.get_mut(),
            }
        }
    }
    #[doc(hidden)]
    mod rtic_ext {
        use super::*;
        #[no_mangle]
        unsafe extern "C" fn main() -> ! {
            rtic::export::assert_send::<u32>();
            rtic::export::interrupt::disable();
            let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal().into();
            let _ = [(); ((1 << lm3s6965::NVIC_PRIO_BITS) - 3u8 as usize)];
            core.NVIC.set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
                rtic::export::logical2hw(3u8, lm3s6965::NVIC_PRIO_BITS),
            );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART0,
            );
            let _ = [(); ((1 << lm3s6965::NVIC_PRIO_BITS) - 2u8 as usize)];
            core.NVIC.set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART1,
                rtic::export::logical2hw(2u8, lm3s6965::NVIC_PRIO_BITS),
            );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::UART1,
            );
            let _ = [(); ((1 << lm3s6965::NVIC_PRIO_BITS) - 4u8 as usize)];
            core.NVIC.set_priority(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
                rtic::export::logical2hw(4u8, lm3s6965::NVIC_PRIO_BITS),
            );
            rtic::export::NVIC::unmask(
                you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::interrupt::SSI0,
            );
            #[inline(never)]
            fn __rtic_init_resources<F>(f: F)
            where
                F: FnOnce(),
            {
                f();
            }
            __rtic_init_resources(|| {
                let (shared_resources, local_resources, mut monotonics) =
                    init(init::Context::new(core.into()));
                __rtic_internal_shared_resource_s
                    .get_mut()
                    .write(core::mem::MaybeUninit::new(shared_resources.s));
                rtic::export::interrupt::enable();
            });
            idle(idle::Context::new(&rtic::export::Priority::new(0)))
        }
    }
}
