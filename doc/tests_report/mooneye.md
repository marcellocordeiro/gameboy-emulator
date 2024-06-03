<!--
✅ = :white_check_mark:
❌ = :x:
🔶 = :large_orange_diamond:
▶️ = :arrow_forward:
-->

# mooneye-test-suite

## acceptance

| Test                    | State                  | Comment              |
| ----------------------- | ---------------------- | -------------------- |
| add_sp_e_timing         | :white_check_mark:     |                      |
| boot_div-S              | :arrow_forward:        | SGB                  |
| boot_div-dmg0           | :arrow_forward:        | DMG0                 |
| boot_div-dmgABCmgb      | :x:                    |                      |
| boot_div2-S             | :arrow_forward:        | SGB                  |
| boot_hwio-S             | :arrow_forward:        | SGB                  |
| boot_hwio-dmg0          | :arrow_forward:        | DMG0                 |
| boot_hwio-dmgABCmgb     | :large_orange_diamond: | Bootrom unsupported. |
| boot_regs-dmg0          | :arrow_forward:        | DMG0                 |
| boot_regs-dmgABC        | :white_check_mark:     |                      |
| boot_regs-mgb           | :arrow_forward:        | MGB                  |
| boot_regs-sgb           | :arrow_forward:        | SGB                  |
| boot_regs-sgb2          | :arrow_forward:        | SGB                  |
| call_cc_timing          | :white_check_mark:     |                      |
| call_cc_timing2         | :white_check_mark:     |                      |
| call_timing             | :white_check_mark:     |                      |
| call_timing2            | :white_check_mark:     |                      |
| di_timing-GS            | :large_orange_diamond: | Passes on CGB.       |
| div_timing              | :white_check_mark:     |                      |
| ei_sequence             | :white_check_mark:     |                      |
| ei_timing               | :white_check_mark:     |                      |
| halt_ime0_ei            | :x:                    |                      |
| halt_ime0_nointr_timing | :white_check_mark:     |                      |
| halt_ime1_timing        | :x:                    |                      |
| halt_ime1_timing2-GS    | :x:                    |                      |
| if_ie_registers         | :white_check_mark:     |                      |
| intr_timing             | :x:                    |                      |
| jp_cc_timing            | :white_check_mark:     |                      |
| jp_timing               | :white_check_mark:     |                      |
| ld_hl_sp_e_timing       | :white_check_mark:     |                      |
| oam_dma_restart         | :white_check_mark:     |                      |
| oam_dma_start           | :x:                    |                      |
| oam_dma_timing          | :white_check_mark:     |                      |
| pop_timing              | :white_check_mark:     |                      |
| push_timing             | :white_check_mark:     |                      |
| rapid_di_ei             | :white_check_mark:     |                      |
| ret_cc_timing           | :white_check_mark:     |                      |
| ret_timing              | :white_check_mark:     |                      |
| reti_intr_timing        | :white_check_mark:     |                      |
| reti_timing             | :white_check_mark:     |                      |
| rst_timing              | :white_check_mark:     |                      |

## acceptance/bits

| Test           | State              | Comment |
| -------------- | ------------------ | ------- |
| mem_oam        | :white_check_mark: |         |
| reg_f          | :white_check_mark: |         |
| unused_hwio-GS | :x:                |         |

## acceptance/instr

| Test | State              | Comment |
| ---- | ------------------ | ------- |
| daa  | :white_check_mark: |         |

## acceptance/interrupts

| Test    | State              | Comment |
| ------- | ------------------ | ------- |
| ie_push | :white_check_mark: |         |

## acceptance/oam_dma

| Test       | State                  | Comment        |
| ---------- | ---------------------- | -------------- |
| basic      | :white_check_mark:     |                |
| reg_read   | :white_check_mark:     |                |
| sources-GS | :large_orange_diamond: | Passes on CGB. |

## acceptance/ppu

| Test                        | State                  | Comment        |
| --------------------------- | ---------------------- | -------------- |
| hblank_ly_scx_timing-GS     | :x:                    |                |
| intr_1_2_timing-GS          | :large_orange_diamond: | Passes on CGB. |
| intr_2_0_timing             | :white_check_mark:     |                |
| intr_2_mode0_timing         | :x:                    |                |
| intr_2_mode0_timing_sprites | :x:                    |                |
| intr_2_mode3_timing         | :x:                    |                |
| intr_2_oam_ok_timing        | :x:                    |                |
| lcdon_timing-GS             | :x:                    |                |
| lcdon_write_timing-GS       | :x:                    |                |
| stat_irq_blocking           | :x:                    |                |
| stat_lyc_onoff              | :x:                    |                |
| vblank_stat_intr-GS         | :large_orange_diamond: | Passes on CGB. |

## acceptance/serial

| Test                      | State | Comment |
| ------------------------- | ----- | ------- |
| boot_sclk_align-dmgABCmgb | :x:   |         |

## acceptance/timer

| Test                 | State              | Comment |
| -------------------- | ------------------ | ------- |
| div_write            | :white_check_mark: |         |
| rapid_toggle         | :x:                |         |
| tim00                | :white_check_mark: |         |
| tim00_div_trigger    | :white_check_mark: |         |
| tim01                | :white_check_mark: |         |
| tim01_div_trigger    | :white_check_mark: |         |
| tim10                | :white_check_mark: |         |
| tim10_div_trigger    | :white_check_mark: |         |
| tim11                | :white_check_mark: |         |
| tim11_div_trigger    | :white_check_mark: |         |
| tima_reload          | :white_check_mark: |         |
| tima_write_reloading | :white_check_mark: |         |
| tma_write_reloading  | :white_check_mark: |         |

## emulator-only/mbc1

| Test              | State              | Comment |
| ----------------- | ------------------ | ------- |
| bits_bank1        | :white_check_mark: |         |
| bits_bank2        | :white_check_mark: |         |
| bits_mode         | :white_check_mark: |         |
| bits_ramg         | :white_check_mark: |         |
| multicart_rom_8Mb | :x:                |         |
| ram_256kb         | :white_check_mark: |         |
| ram_64kb          | :white_check_mark: |         |
| rom_16Mb          | :white_check_mark: |         |
| rom_1Mb           | :white_check_mark: |         |
| rom_2Mb           | :white_check_mark: |         |
| rom_4Mb           | :white_check_mark: |         |
| rom_512kb         | :white_check_mark: |         |
| rom_8Mb           | :white_check_mark: |         |

## emulator-only/mbc2

| Test        | State              | Comment |
| ----------- | ------------------ | ------- |
| bits_ramg   | :white_check_mark: |         |
| bits_romb   | :white_check_mark: |         |
| bits_unused | :white_check_mark: |         |
| ram         | :white_check_mark: |         |
| rom_1Mb     | :white_check_mark: |         |
| rom_2Mb     | :white_check_mark: |         |
| rom_512kb   | :white_check_mark: |         |

## emulator-only/mbc5

| Test      | State              | Comment |
| --------- | ------------------ | ------- |
| rom_16Mb  | :white_check_mark: |         |
| rom_1Mb   | :white_check_mark: |         |
| rom_2Mb   | :white_check_mark: |         |
| rom_32Mb  | :white_check_mark: |         |
| rom_4Mb   | :white_check_mark: |         |
| rom_512kb | :white_check_mark: |         |
| rom_64Mb  | :white_check_mark: |         |
| rom_8Mb   | :white_check_mark: |         |

## madness

| Test                     | State | Comment |
| ------------------------ | ----- | ------- |
| mgb_oam_dma_halt_sprites | todo  |         |

## manual-only

| Test            | State | Comment |
| --------------- | ----- | ------- |
| sprite_priority | todo  |         |

## misc

| Test              | State                  | Comment           |
| ----------------- | ---------------------- | ----------------- |
| boot_div-A        | :arrow_forward:        | AGS               |
| boot_div-cgb0     | :arrow_forward:        | CGB0              |
| boot_div-cgbABCDE | :large_orange_diamond: | Requires bootrom. |
| boot_hwio-C       | :large_orange_diamond: | Requires bootrom. |
| boot_regs-A       | :arrow_forward:        | AGS               |
| boot_regs-cgb     | :white_check_mark:     |                   |

## misc/bits

| Test          | State | Comment |
| ------------- | ----- | ------- |
| unused_hwio-C | :x:   |         |

## misc/ppu

| Test               | State | Comment |
| ------------------ | ----- | ------- |
| vblank_stat_intr-C | :x:   |         |

## utils

| Test           | State | Comment |
| -------------- | ----- | ------- |
| bootrom_dumper | N/A   |         |
| dump_boot_hwio | N/A   |         |
