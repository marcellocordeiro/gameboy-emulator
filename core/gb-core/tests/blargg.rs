use common::runners::{run_until_memory_status, run_until_serial_passed};
use gb_core::GameBoy;

mod common;

testcases_blargg_serial! {
    // TODO: Add cgb_sound tests?

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

    // TODO: add dmg_sound tests?

    instr_timing("instr_timing/instr_timing.gb");
    // interrupt_time("interrupt_time/interrupt_time.gb"); // CGB only.

    // mem_timing("mem_timing/mem_timing.gb"); // TODO: no programmatic way of stopping.
}

testcases_blargg_memory! {
    mem_timing_2_01_read_timing("mem_timing-2/rom_singles/01-read_timing.gb");
    mem_timing_2_02_write_timing("mem_timing-2/rom_singles/02-write_timing.gb");
    mem_timing_2_03_modify_timing("mem_timing-2/rom_singles/03-modify_timing.gb");
    mem_timing_2("mem_timing-2/mem_timing.gb");
}
