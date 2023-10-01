#[macro_export]
macro_rules! testcases_mooneye {
    (
        $name:ident($path:expr);
    ) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "mooneye-test-suite/", $path));

            let gb = run_until_break(rom);
            validate_fibonacci(gb);
        }
    };

    (
        $name:ident($path:expr);
        $(
            $names:ident($paths:expr);
        )+
    ) => {
        testcases_mooneye! { $name($path); }
        testcases_mooneye! {
            $(
                $names($paths);
            )+
        }
    };
}

#[macro_export]
macro_rules! testcases_blargg_serial {
    (
        $name:ident($path:expr);
    ) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "blargg/", $path));

            let _ = run_until_serial_passed(rom);
        }
    };

    (
        $name:ident($path:expr);
        $(
            $names:ident($paths:expr);
        )+
    ) => {
        testcases_blargg_serial! { $name($path); }
        testcases_blargg_serial! {
            $(
                $names($paths);
            )+
        }
    };
}

#[macro_export]
macro_rules! testcases_blargg_memory {
    (
        $name:ident($path:expr);
    ) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "blargg/", $path));

            let _ = run_until_memory_status(rom);
        }
    };

    (
        $name:ident($path:expr);
        $(
            $names:ident($paths:expr);
        )+
    ) => {
        testcases_blargg_memory! { $name($path); }
        testcases_blargg_memory! {
            $(
                $names($paths);
            )+
        }
    };
}
