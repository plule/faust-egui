use egui::{RichText, Slider};
use faust_state::{Node, StateHandle};

pub struct DspUi {
    dsp_state: StateHandle,
    params: Vec<(i32, Node)>,
}

impl DspUi {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, dsp_state: StateHandle) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        // Store the params as vec to get a sorted liste
        let mut params: Vec<_> = dsp_state.params().clone().into_iter().collect();
        params.sort_by_key(|p| p.0);

        Self { dsp_state, params }
    }
}

impl eframe::App for DspUi {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { dsp_state, params } = self;
        dsp_state.update();
        egui::CentralPanel::default().show(ctx, |ui| {
            for (idx, node) in params {
                let mut value = *dsp_state.get_param(*idx).unwrap();
                match node.widget_type() {
                    faust_state::WidgetType::Unknown => panic!("There is an unknown widget."),
                    faust_state::WidgetType::Boolean(input) => match input {
                        faust_state::BooleanKind::Button => {
                            if ui
                                .button(RichText::new(node.path()).heading())
                                .is_pointer_button_down_on()
                            {
                                value = 1.0;
                            } else {
                                value = 0.0;
                            }
                        }
                        faust_state::BooleanKind::Toggle => {
                            let mut state = value > 0.5;
                            ui.toggle_value(&mut state, node.path());
                            value = if state { 1.0 } else { 0.0 };
                        }
                    },
                    faust_state::WidgetType::RangedInput(input) => match input.kind {
                        faust_state::RangedInputKind::VerticalSlider => {
                            ui.group(|ui| {
                                ui.label(node.path());
                                ui.add(
                                    Slider::new(&mut value, input.range.clone())
                                        .step_by(input.step.into())
                                        .vertical(),
                                );
                            });
                        }
                        faust_state::RangedInputKind::HorizontalSlider => {
                            ui.group(|ui| {
                                ui.label(node.path());
                                ui.add(
                                    Slider::new(&mut value, input.range.clone())
                                        .step_by(input.step.into()),
                                );
                            });
                        }
                        faust_state::RangedInputKind::NumEntry => {
                            ui.group(|ui| {
                                ui.add(
                                    egui::DragValue::new(&mut value)
                                        .clamp_range(input.range.clone()),
                                );
                                ui.label(node.path());
                            });
                        }
                    },
                    faust_state::WidgetType::RangedOutput(_) => {}
                }
                dsp_state.set_param(*idx, value);
            }

            egui::warn_if_debug_build(ui);
        });
        dsp_state.send();
    }
}
