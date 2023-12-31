#[macro_export]
macro_rules! testcases_mooneye {
    (
        $name:ident($path:expr);
    ) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "mooneye-test-suite/", $path));

            let mut gb = GameBoy::new();
            gb.load_cartridge(rom.to_vec()).unwrap();

            run_until_break(&mut gb);
            validate_fibonacci(&gb);
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

            let mut gb = GameBoy::new();
            gb.load_cartridge(rom.to_vec()).unwrap();

            run_until_serial_passed(&mut gb);
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

            let mut gb = GameBoy::new();
            gb.load_cartridge(rom.to_vec()).unwrap();

            run_until_memory_status(&mut gb);
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
