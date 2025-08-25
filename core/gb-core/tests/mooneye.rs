mod common;

// Acceptance
testcases_mooneye! {
    add_sp_e_timing("acceptance/add_sp_e_timing.gb");

    // boot_div_s("acceptance/boot_div-S.gb"); // SGB is unsupported
    // boot_div_dmg0("acceptance/boot_div-dmg0.gb"); // DMG0 is unsupported

    // boot_div_dmg_abc_mgb("acceptance/boot_div-dmgABCmgb.gb"); // Tested separately

    // boot_div2_s("acceptance/boot_div2-S.gb"); // SGB is unsupported
    // boot_hwio_s("acceptance/boot_hwio-S.gb"); // SGB is unsupported
    // boot_hwio_dmg0("acceptance/boot_hwio-dmg0.gb"); // DMG0 is unsupported

    // boot_hwio_dmg_abc_mgb("acceptance/boot_hwio-dmgABCmgb.gb"); // Tested separately

    // boot_regs_dmg0("acceptance/boot_regs-dmg0.gb"); // DMG0 is unsupported

    boot_regs_dmg_abc("acceptance/boot_regs-dmgABC.gb", dmg);

    // boot_regs_mgb("acceptance/boot_regs-mgb.gb"); // MGB is unsupported
    // boot_regs_sgb("acceptance/boot_regs-sgb.gb"); // SGB is unsupported
    // boot_regs_sgb2("acceptance/boot_regs-sgb2.gb"); SGB is unsupported

    call_cc_timing("acceptance/call_cc_timing.gb");
    call_cc_timing2("acceptance/call_cc_timing2.gb");
    call_timing("acceptance/call_timing.gb");
    call_timing2("acceptance/call_timing2.gb");
    di_timing_gs("acceptance/di_timing-GS.gb");
    div_timing("acceptance/div_timing.gb");
    ei_sequence("acceptance/ei_sequence.gb");
    ei_timing("acceptance/ei_timing.gb");
    halt_ime0_ei("acceptance/halt_ime0_ei.gb");
    halt_ime0_nointr_timing("acceptance/halt_ime0_nointr_timing.gb");
    halt_ime1_timing("acceptance/halt_ime1_timing.gb");
    halt_ime1_timing2_gs("acceptance/halt_ime1_timing2-GS.gb");
    if_ie_registers("acceptance/if_ie_registers.gb");
    intr_timing("acceptance/intr_timing.gb");
    jp_cc_timing("acceptance/jp_cc_timing.gb");
    jp_timing("acceptance/jp_timing.gb");
    ld_hl_sp_e_timing("acceptance/ld_hl_sp_e_timing.gb");
    oam_dma_restart("acceptance/oam_dma_restart.gb");
    oam_dma_start("acceptance/oam_dma_start.gb");
    oam_dma_timing("acceptance/oam_dma_timing.gb");
    pop_timing("acceptance/pop_timing.gb");
    push_timing("acceptance/push_timing.gb");
    rapid_di_ei("acceptance/rapid_di_ei.gb");
    ret_cc_timing("acceptance/ret_cc_timing.gb");
    ret_timing("acceptance/ret_timing.gb");
    reti_intr_timing("acceptance/reti_intr_timing.gb");
    reti_timing("acceptance/reti_timing.gb");
    rst_timing("acceptance/rst_timing.gb");
    bits_mem_oam("acceptance/bits/mem_oam.gb");
    bits_reg_f("acceptance/bits/reg_f.gb");
    bits_unused_hwio_gs("acceptance/bits/unused_hwio-GS.gb", dmg);
    instr_daa("acceptance/instr/daa.gb");
    interrupts_ie_push("acceptance/interrupts/ie_push.gb");
    oam_dma_basic("acceptance/oam_dma/basic.gb");
    oam_dma_reg_read("acceptance/oam_dma/reg_read.gb");
    oam_dma_sources_gs("acceptance/oam_dma/sources-GS.gb"); // ! This should fail on CGB
    // ppu_hblank_ly_scx_timing_gs("acceptance/ppu/hblank_ly_scx_timing-GS.gb");
    ppu_intr_1_2_timing_gs("acceptance/ppu/intr_1_2_timing-GS.gb"); // ! This should fail on CGB
    ppu_intr_2_0_timing("acceptance/ppu/intr_2_0_timing.gb");
    ppu_intr_2_mode0_timing("acceptance/ppu/intr_2_mode0_timing.gb");
    // ppu_intr_2_mode0_timing_sprites("acceptance/ppu/intr_2_mode0_timing_sprites.gb");
    ppu_intr_2_mode3_timing("acceptance/ppu/intr_2_mode3_timing.gb");
    ppu_intr_2_oam_ok_timing("acceptance/ppu/intr_2_oam_ok_timing.gb");
    // ppu_lcdon_timing_gs("acceptance/ppu/lcdon_timing-GS.gb");
    // ppu_lcdon_write_timing_gs("acceptance/ppu/lcdon_write_timing-GS.gb");
    // ppu_stat_irq_blocking("acceptance/ppu/stat_irq_blocking.gb");
    // ppu_stat_lyc_onoff("acceptance/ppu/stat_lyc_onoff.gb");
    ppu_vblank_stat_intr_gs("acceptance/ppu/vblank_stat_intr-GS.gb"); // ! This should fail on CGB
    // serial_boot_sclk_align_dmg_abc_mgb("acceptance/serial/boot_sclk_align-dmgABCmgb.gb");
    timer_div_write("acceptance/timer/div_write.gb");
    timer_rapid_toggle("acceptance/timer/rapid_toggle.gb");
    timer_tim00("acceptance/timer/tim00.gb");
    timer_tim00_div_trigger("acceptance/timer/tim00_div_trigger.gb");
    timer_tim01("acceptance/timer/tim01.gb");
    timer_tim01_div_trigger("acceptance/timer/tim01_div_trigger.gb");
    timer_tim10("acceptance/timer/tim10.gb");
    timer_tim10_div_trigger("acceptance/timer/tim10_div_trigger.gb");
    timer_tim11("acceptance/timer/tim11.gb");
    timer_tim11_div_trigger("acceptance/timer/tim11_div_trigger.gb");
    timer_tima_reload("acceptance/timer/tima_reload.gb");
    timer_tima_write_reloading("acceptance/timer/tima_write_reloading.gb");
    timer_tma_write_reloading("acceptance/timer/tma_write_reloading.gb");
}

// MBC
testcases_mooneye! {
    // MBC1
    mbc1_bits_ramg("emulator-only/mbc1/bits_ramg.gb");
    mbc1_bits_bank1("emulator-only/mbc1/bits_bank1.gb");
    mbc1_bits_bank2("emulator-only/mbc1/bits_bank2.gb");
    mbc1_bits_mode("emulator-only/mbc1/bits_mode.gb");
    mbc1_rom_512kb("emulator-only/mbc1/rom_512kb.gb");
    mbc1_rom_1mb("emulator-only/mbc1/rom_1Mb.gb");
    mbc1_rom_2mb("emulator-only/mbc1/rom_2Mb.gb");
    mbc1_rom_4mb("emulator-only/mbc1/rom_4Mb.gb");
    mbc1_rom_8mb("emulator-only/mbc1/rom_8Mb.gb");
    mbc1_rom_16mb("emulator-only/mbc1/rom_16Mb.gb");
    mbc1_ram_64kb("emulator-only/mbc1/ram_64kb.gb");
    mbc1_ram_256kb("emulator-only/mbc1/ram_256kb.gb");
    // mbc1_multicart_rom_8mb("emulator-only/mbc1/multicart_rom_8Mb.gb");

    // MBC2
    mbc2_bits_ramg("emulator-only/mbc2/bits_ramg.gb");
    mbc2_bits_romb("emulator-only/mbc2/bits_romb.gb");
    mbc2_bits_unused("emulator-only/mbc2/bits_unused.gb");
    mbc2_rom_512kb("emulator-only/mbc2/rom_512kb.gb");
    mbc2_rom_1mb("emulator-only/mbc2/rom_1Mb.gb");
    mbc2_rom_2mb("emulator-only/mbc2/rom_2Mb.gb");
    mbc2_ram("emulator-only/mbc2/ram.gb");

    // MBC5
    mbc5_rom_512kb("emulator-only/mbc5/rom_512kb.gb");
    mbc5_rom_1mb("emulator-only/mbc5/rom_1Mb.gb");
    mbc5_rom_2mb("emulator-only/mbc5/rom_2Mb.gb");
    mbc5_rom_4mb("emulator-only/mbc5/rom_4Mb.gb");
    mbc5_rom_8mb("emulator-only/mbc5/rom_8Mb.gb");
    mbc5_rom_16mb("emulator-only/mbc5/rom_16Mb.gb");
    mbc5_rom_32mb("emulator-only/mbc5/rom_32Mb.gb");
    mbc5_rom_64mb("emulator-only/mbc5/rom_64Mb.gb");
}

// Acceptance
// Bootrom is unsupported
#[cfg(not(feature = "bundled-bootrom"))]
testcases_mooneye! {
    boot_div_dmg_abc_mgb("acceptance/boot_div-dmgABCmgb.gb");
    // boot_hwio_dmg_abc_mgb("acceptance/boot_hwio-dmgABCmgb.gb");
}

// TODO: compare screenshots
// manual-only
// testcases_mooneye! {
//     sprite_priority("manual-only/sprite_priority.gb");
// }

// Misc
testcases_mooneye! {
    // boot_div_a("misc/boot_div-A.gb"); // AGS is unsupported

    // boot_div_cgb0("misc/boot_div-cgb0.gb"); // CGB0 is unsupported

    // boot_div_cgb_abcde("misc/boot_div-cgbABCDE.gb"); // CGB only. Tested separately
    boot_hwio_c("misc/boot_hwio-C.gb", cgb); // CGB only

    // boot_regs_a("misc/boot_regs-A.gb"); // AGS is unsupported

    boot_regs_cgb("misc/boot_regs-cgb.gb", cgb); // CGB only
    // bits_unused_hwio_c("misc/bits/unused_hwio-C.gb");  // TODO: why is this failing?
    // ppu_vblank_stat_intr_c("misc/ppu/vblank_stat_intr-C.gb");
}

// Misc
// CGB only tests (requires bootrom).
// #[cfg(feature = "bundled-bootrom")]
// testcases_mooneye! {
//     // boot_div_cgb_abcde("misc/boot_div-cgbABCDE.gb", cgb);
// }

// From: https://github.com/Gekkio/mooneye-test-suite
//
// ## Test naming
//
// Some tests are expected to pass only a single console model:
//
// * dmg = Game Boy
// * mgb = Game Boy Pocket
// * sgb = Super Game Boy
// * sgb2 = Super Game Boy 2
// * cgb = Game Boy Color
// * agb = Game Boy Advance
// * ags = Game Boy Advance SP
//
// In addition to model differences, SoC revisions can affect the behaviour.
// Revision 0 refers always to the initial version of a SoC (e.g. CPU CGB). AGB
// and AGS use the same SoC models, but in two different packages. The following
// SoC models have several revisions:
//
// * DMG: 0, A, B, C
// * CGB: 0, A, B, C, D, E
// * AGB: 0, A, A E, B, B E. Revision E also exists, but only in Game Boy Micro
//   (OXY) so it is out of this project's scope. However, A E and B E are most
//   likely actually just E revision in A or B-compatible package.
//
// In general, hardware can be divided to a couple of groups based on their
// behaviour. Some tests are expected to pass on a single or multiple groups:
//
// * G = dmg+mgb
// * S = sgb+sgb2
// * C = cgb+agb+ags
// * A = agb+ags
//
// For example, a test with GS in the name is expected to pass on dmg+mgb +
// sgb+sgb2.
