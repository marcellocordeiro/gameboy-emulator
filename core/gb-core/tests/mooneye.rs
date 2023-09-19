use std::time::{Duration, Instant};

use gb_core::GameBoy;

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

fn run(rom: &[u8]) {
    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    let start_time = Instant::now();

    loop {
        if Instant::now() - start_time > TIMEOUT {
            panic!("Timeout");
        }

        gb.cpu.step();

        if gb.cpu.memory.read(gb.cpu.registers.program_counter) == BREAK_OPCODE {
            gb.cpu.step();
            break;
        }
    }

    let regs = &gb.cpu.registers;

    let is_fibonacci = regs.accumulator == 0
        && regs.b == 3
        && regs.c == 5
        && regs.d == 8
        && regs.e == 13
        && regs.h == 21
        && regs.l == 34;

    assert!(is_fibonacci, "Fail");
}

macro_rules! testcases {
    (
        $name:ident($path:expr);
    ) => {
        #[test]
        fn $name() {
            let rom = include_bytes!(concat!("../../../external/gameboy-test-roms/", "mooneye-test-suite/", $path, ".gb"));

            run(rom);
        }
    };

    (
        $name:ident($path:expr);
        $(
            $names:ident($paths:expr);
        )+
    ) => {
        testcases! { $name($path); }
        testcases! {
            $(
                $names($paths);
            )+
        }
    };
}

// DMG only tests.
#[cfg(not(feature = "cgb"))]
testcases! {
    boot_regs_dmg_abc("acceptance/boot_regs-dmgABC");
}

// DMG only tests (bootrom is unsupported).
#[cfg(not(any(feature = "cgb", feature = "bootrom")))]
testcases! {
    boot_hwio_dmg_abc_mgb("acceptance/boot_hwio-dmgABCmgb");
}

// CGB only tests.
#[cfg(feature = "cgb")]
testcases! {
    boot_regs_cgb("misc/boot_regs-cgb");
}

// CGB only tests (bootrom is unsupported).
#[cfg(all(feature = "cgb", not(feature = "bootrom")))]
testcases! {
    boot_hwio_c("misc/boot_hwio-C");
}

testcases! {
  // add_sp_e_timing("acceptance/add_sp_e_timing");
  // call_cc_timing("acceptance/call_cc_timing");
  // call_cc_timing2("acceptance/call_cc_timing2");
  // call_timing("acceptance/call_timing");
  // call_timing2("acceptance/call_timing2");
  di_timing_gs("acceptance/di_timing-GS");
  div_timing("acceptance/div_timing");
  ei_sequence("acceptance/ei_sequence");
  ei_timing("acceptance/ei_timing");
  // halt_ime0_ei("acceptance/halt_ime0_ei");
  halt_ime0_nointr_timing("acceptance/halt_ime0_nointr_timing");
  // halt_ime1_timing("acceptance/halt_ime1_timing");
  // halt_ime1_timing2_gs("acceptance/halt_ime1_timing2-GS");
  if_ie_registers("acceptance/if_ie_registers");
  // intr_timing("acceptance/intr_timing");
  // jp_cc_timing("acceptance/jp_cc_timing");
  // jp_timing("acceptance/jp_timing");
  // ld_hl_sp_e_timing("acceptance/ld_hl_sp_e_timing");
  // oam_dma_restart("acceptance/oam_dma_restart");
  // oam_dma_start("acceptance/oam_dma_start");
  // oam_dma_timing("acceptance/oam_dma_timing");
  pop_timing("acceptance/pop_timing");
  // push_timing("acceptance/push_timing");
  rapid_di_ei("acceptance/rapid_di_ei");
  // ret_timing("acceptance/ret_timing");
  // reti_timing("acceptance/reti_timing");
  // ret_cc_timing("acceptance/ret_cc_timing");
  // reti_intr_timing("acceptance/reti_intr_timing");
  // rst_timing("acceptance/rst_timing");
  bits_mem_oam("acceptance/bits/mem_oam");
  bits_reg_f("acceptance/bits/reg_f");
  // bits_unused_hwio_gs("acceptance/bits/unused_hwio-GS");
  instr_daa("acceptance/instr/daa");
  // interrupts_ie_push("acceptance/interrupts/ie_push");
  oam_dma_basic("acceptance/oam_dma/basic");
  oam_dma_reg_read("acceptance/oam_dma/reg_read");
  oam_dma_sources_gs("acceptance/oam_dma/sources-GS");
  // ppu_hblank_ly_scx_timing_gs("acceptance/ppu/hblank_ly_scx_timing-GS");
  ppu_intr_1_2_timing_gs("acceptance/ppu/intr_1_2_timing-GS");
  ppu_intr_2_0_timing("acceptance/ppu/intr_2_0_timing");
  // ppu_intr_2_mode0_timing("acceptance/ppu/intr_2_mode0_timing");
  // ppu_stat_lyc_onoff("acceptance/ppu/stat_lyc_onoff");
  // ppu_intr_2_mode0_timing_sprites("acceptance/ppu/intr_2_mode0_timing_sprites");
  // ppu_intr_2_mode3_timing("acceptance/ppu/intr_2_mode3_timing");
  // ppu_intr_2_oam_ok_timing("acceptance/ppu/intr_2_oam_ok_timing");
  // ppu_lcdon_timing_gs("acceptance/ppu/lcdon_timing-GS");
  // ppu_lcdon_write_timing_gs("acceptance/ppu/lcdon_write_timing-GS");
  // ppu_stat_irq_blocking("acceptance/ppu/stat_irq_blocking");
  ppu_vblank_stat_intr_gs("acceptance/ppu/vblank_stat_intr-GS");
  // serial_boot_sclk_align_dmg_abc_mgb("acceptance/serial/boot_sclk_align-dmgABCmgb");
  timer_div_write("acceptance/timer/div_write");
  // timer_rapid_toggle("acceptance/timer/rapid_toggle");
  timer_tim00("acceptance/timer/tim00");
  timer_tim00_div_trigger("acceptance/timer/tim00_div_trigger");
  timer_tim01("acceptance/timer/tim01");
  timer_tim01_div_trigger("acceptance/timer/tim01_div_trigger");
  timer_tim10("acceptance/timer/tim10");
  timer_tim10_div_trigger("acceptance/timer/tim10_div_trigger");
  timer_tim11("acceptance/timer/tim11");
  timer_tim11_div_trigger("acceptance/timer/tim11_div_trigger");
  timer_tima_reload("acceptance/timer/tima_reload");
  timer_tima_write_reloading("acceptance/timer/tima_write_reloading");
  timer_tma_write_reloading("acceptance/timer/tma_write_reloading");
}

// MBC
testcases! {
  // MBC1
  mbc1_bits_ramg("emulator-only/mbc1/bits_ramg");
  mbc1_bits_bank1("emulator-only/mbc1/bits_bank1");
  mbc1_bits_bank2("emulator-only/mbc1/bits_bank2");
  mbc1_bits_mode("emulator-only/mbc1/bits_mode");
  mbc1_rom_512kb("emulator-only/mbc1/rom_512kb");
  mbc1_rom_1mb("emulator-only/mbc1/rom_1Mb");
  mbc1_rom_2mb("emulator-only/mbc1/rom_2Mb");
  mbc1_rom_4mb("emulator-only/mbc1/rom_4Mb");
  mbc1_rom_8mb("emulator-only/mbc1/rom_8Mb");
  mbc1_rom_16mb("emulator-only/mbc1/rom_16Mb");
  mbc1_ram_64kb("emulator-only/mbc1/ram_64kb");
  mbc1_ram_25kb("emulator-only/mbc1/ram_256kb");
  // mbc1_multicart_rom_8mb("emulator-only/mbc1/multicart_rom_8Mb");

  // MBC2
  mbc2_bits_ramg("emulator-only/mbc2/bits_ramg");
  mbc2_bits_romb("emulator-only/mbc2/bits_romb");
  mbc2_bits_unused("emulator-only/mbc2/bits_unused");
  mbc2_rom_512kb("emulator-only/mbc2/rom_512kb");
  mbc2_rom_1mb("emulator-only/mbc2/rom_1Mb");
  mbc2_rom_2mb("emulator-only/mbc2/rom_2Mb");
  mbc2_ram("emulator-only/mbc2/ram");

  // MBC5
  mbc5_rom_512kb("emulator-only/mbc5/rom_512kb");
  mbc5_rom_1mb("emulator-only/mbc5/rom_1Mb");
  mbc5_rom_2mb("emulator-only/mbc5/rom_2Mb");
  mbc5_rom_4mb("emulator-only/mbc5/rom_4Mb");
  mbc5_rom_8mb("emulator-only/mbc5/rom_8Mb");
  mbc5_rom_16mb("emulator-only/mbc5/rom_16Mb");
  mbc5_rom_32mb("emulator-only/mbc5/rom_32Mb");
  mbc5_rom_64mb("emulator-only/mbc5/rom_64Mb");
}
