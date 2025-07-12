mod common;

testcases_blargg_serial! {
    cpu_instrs_01_special("cpu_instrs/individual/01-special.gb");
    cpu_instrs_02_interrupts("cpu_instrs/individual/02-interrupts.gb");
    cpu_instrs_03_op_sp_hl("cpu_instrs/individual/03-op sp,hl.gb");
    cpu_instrs_04_op_r_imm("cpu_instrs/individual/04-op r,imm.gb");
    cpu_instrs_05_op_rp("cpu_instrs/individual/05-op rp.gb");
    cpu_instrs_06_ld_r_r("cpu_instrs/individual/06-ld r,r.gb");
    cpu_instrs_07_jr_jp_call_ret_rst("cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
    cpu_instrs_08_misc_instrs("cpu_instrs/individual/08-misc instrs.gb");
    cpu_instrs_09_op_r_r("cpu_instrs/individual/09-op r,r.gb");
    cpu_instrs_10_bit_ops("cpu_instrs/individual/10-bit ops.gb");
    cpu_instrs_11_op_a_hl("cpu_instrs/individual/11-op a,(hl).gb");
    // cpu_instrs_all("cpu_instrs/cpu_instrs.gb"); // Very slow.

    instr_timing("instr_timing/instr_timing.gb");
    // interrupt_time("interrupt_time/interrupt_time.gb"); // CGB only.

    // mem_timing("mem_timing/mem_timing.gb"); // TODO: no programmatic way of stopping.
}

testcases_blargg_memory! {
    mem_timing_2_01_read_timing("mem_timing-2/rom_singles/01-read_timing.gb");
    mem_timing_2_02_write_timing("mem_timing-2/rom_singles/02-write_timing.gb");
    mem_timing_2_03_modify_timing("mem_timing-2/rom_singles/03-modify_timing.gb");
    mem_timing_2("mem_timing-2/mem_timing.gb");

    dmg_sound_01_registers("dmg_sound/rom_singles/01-registers.gb");
    dmg_sound_02_len_ctr("dmg_sound/rom_singles/02-len ctr.gb");
    // dmg_sound_03_trigger("dmg_sound/rom_singles/03-trigger.gb");
    // dmg_sound_04_sweep("dmg_sound/rom_singles/04-sweep.gb");
    // dmg_sound_05_sweep_details("dmg_sound/rom_singles/05-sweep details.gb");
    // dmg_sound_06_overflow_on_trigger("dmg_sound/rom_singles/06-overflow on trigger.gb");
    // dmg_sound_07_len_sweep_period_sync("dmg_sound/rom_singles/07-len sweep period sync.gb");
    // dmg_sound_08_len_ctr_during_power("dmg_sound/rom_singles/08-len ctr during power.gb");
    // dmg_sound_09_wave_read_while_on("dmg_sound/rom_singles/09-wave read while on.gb");
    // dmg_sound_10_wave_trigger_while_on("dmg_sound/rom_singles/10-wave trigger while on.gb");
    // dmg_sound_11_regs_after_power("dmg_sound/rom_singles/11-regs after power.gb");
    // dmg_sound_12_wave_write_while_on("dmg_sound/rom_singles/12-wave write while on.gb");

    cgb_sound_01_registers("cgb_sound/rom_singles/01-registers.gb");
    cgb_sound_02_len_ctr("cgb_sound/rom_singles/02-len ctr.gb");
    // cgb_sound_03_trigger("cgb_sound/rom_singles/03-trigger.gb");
    // cgb_sound_04_sweep("cgb_sound/rom_singles/04-sweep.gb");
    // cgb_sound_05_sweep_details("cgb_sound/rom_singles/05-sweep details.gb");
    // cgb_sound_06_overflow_on_trigger("cgb_sound/rom_singles/06-overflow on trigger.gb");
    // cgb_sound_07_len_sweep_period_sync("cgb_sound/rom_singles/07-len sweep period sync.gb");
    cgb_sound_08_len_ctr_during_power("cgb_sound/rom_singles/08-len ctr during power.gb");
    // cgb_sound_09_wave_read_while_on("cgb_sound/rom_singles/09-wave read while on.gb");
    // cgb_sound_10_wave_trigger_while_on("cgb_sound/rom_singles/10-wave trigger while on.gb");
    cgb_sound_11_regs_after_power("cgb_sound/rom_singles/11-regs after power.gb");
    // cgb_sound_12_wave("cgb_sound/rom_singles/12-wave.gb");
}
