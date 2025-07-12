use egui::{Context, FontId, RichText, Ui, Window};
use gb_core::{GameBoy, components::memory::MemoryInterface as _};

#[derive(Debug, Default)]
pub struct State {
    opened: bool,
}

impl State {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("State").clicked() {
            self.toggle();
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !self.opened {
            return;
        }

        Window::new("State")
            .open(&mut self.opened)
            .show(egui_ctx, |ui| {
                let interrupts = gb_ctx.memory().interrupts();
                let registers = gb_ctx.cpu().registers();

                let ie = interrupts.read_enable();
                let r#if = interrupts.read_flags();
                let ime = registers.ime;

                let flags = &registers.f;

                let (af, a, f) = { (registers.get_af(), registers.a, flags.bits()) };
                let (bc, b, c) = { (registers.get_bc(), registers.b, registers.c) };
                let (de, d, e) = { (registers.get_de(), registers.d, registers.e) };
                let (hl, h, l) = { (registers.get_hl(), registers.h, registers.l) };

                let (pc, sp) = { (registers.pc, registers.sp) };

                let flags_line = format!(
                    "Flags: Z: {} | N: {} | H: {} | C: {}",
                    if flags.zero() { "☑" } else { "☐" },
                    if flags.n_add_sub() { "☑" } else { "☐" },
                    if flags.half_carry() { "☑" } else { "☐" },
                    if flags.carry() { "☑" } else { "☐" }
                );

                let text = format!(
                    "\
                    AF: {af:#06X}, A: {a:#04X} | F: {f:#04X}\n\
                    BC: {bc:#06X}, B: {b:#04X} | C: {c:#04X}\n\
                    DE: {de:#06X}, D: {d:#04X} | E: {e:#04X}\n\
                    HL: {hl:#06X}, H: {h:#04X} | L: {l:#04X}\n\
                    \n\
                    PC: {pc:#06X}\n\
                    SP: {sp:#06X}\n\
                    \n\
                    {flags_line}\n\
                    \n\
                    EI: {ime}\n\
                    \n\
                    IE: {ie:#04X}\n\
                    IF: {if:#04X}\
                "
                );

                ui.label(RichText::new(text).font(FontId::monospace(14.0)));
            });
    }
}
