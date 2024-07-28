#[macro_export]
macro_rules! run_for_model {
    ($F:path, $rom:ident) => {
        run_for_model!(dmg, $F, $rom);
        run_for_model!(cgb, $F, $rom);
    };

    (dmg, $F:path, $rom:ident) => {
        if let Err(e) = $F(gb_core::constants::DeviceModel::Dmg, $rom) {
            panic!("DMG failure: {e}");
        }
    };

    (cgb, $F:path, $rom:ident) => {
        if let Err(e) = $F(gb_core::constants::DeviceModel::Cgb, $rom) {
            panic!("CGB failure: {e}");
        }
    };
}

#[macro_export]
macro_rules! testcases_mooneye {
    ($name:ident($path:literal $(, $model:ident)?);) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "mooneye-test-suite/", $path));
            run_for_model!($($model, )? common::mooneye::run, rom);
        }
    };

    (
        $name:ident($path:literal $(, $model:ident)?);
        $($names:ident($paths:literal $(, $models:ident)?);)+
    ) => {
        testcases_mooneye! { $name($path $(, $model)?); }
        testcases_mooneye! { $($names($paths $(, $models)?);)+ }
    };
}

#[macro_export]
macro_rules! testcases_blargg_serial {
    ($name:ident($path:literal $(, $model:ident)?);) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "blargg/", $path));
            run_for_model!($($model, )? common::blargg::run_serial, rom);
        }
    };

    (
        $name:ident($path:literal $(, $model:ident)?);
        $($names:ident($paths:literal $(, $models:ident)?);)+
    ) => {
        testcases_blargg_serial! { $name($path $(, $model)?); }
        testcases_blargg_serial! { $($names($paths $(, $models)?);)+ }
    };
}

#[macro_export]
macro_rules! testcases_blargg_memory {
    ($name:ident($path:literal $(, $model:ident)?);) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "blargg/", $path));
            run_for_model!($($model, )? common::blargg::run_memory, rom);
        }
    };

    (
        $name:ident($path:literal $(, $model:ident)?);
        $($names:ident($paths:literal $(, $models:ident)?);)+
    ) => {
        testcases_blargg_memory! { $name($path $(, $model)?); }
        testcases_blargg_memory! { $($names($paths $(, $models)?);)+ }
    };
}
