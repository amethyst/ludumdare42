use amethyst::ecs::*;
use amethyst::input::{Axis, Button, InputEvent, InputHandler};
use amethyst::renderer::VirtualKeyCode;

const CONTROLS_BLACKLIST: [VirtualKeyCode; 1] = [VirtualKeyCode::Escape];

/// The resource secifying what config should be changed on next input
pub enum ChangeControl {
    Axis { name: String, positive: bool },
    Action { name: String },
    None,
}

impl Default for ChangeControl {
    fn default() -> Self {
        ChangeControl::None
    }
}

/// The system handling the control change
pub struct ChangeControlListener;

impl<'a> System<'a> for ChangeControlListener {
    type SystemData = (
        Read<'a, ChangeControl>,
        Write<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (change, mut handler): Self::SystemData) {
        if let ChangeControl::None = *change {
            return;
        }
        let new_key = {
            handler
                .keys_that_are_down()
                .skip_while(|k| CONTROLS_BLACKLIST.contains(k))
                .next()
        };
        if let Some(new_key) = new_key {
            if let ChangeControl::Axis {
                ref name,
                ref positive,
            } = *change
            {
                let axis = handler
                    .bindings
                    .remove_axis(name)
                    .unwrap_or_else(|| panic!("Unknown input axis '{}' to change", name));
                if *positive {
                    if let Axis::Emulated { neg: old_neg, .. } = axis {
                        handler.bindings.insert_axis(
                            name.clone(),
                            Axis::Emulated {
                                pos: Button::Key(new_key),
                                neg: old_neg,
                            },
                        );
                    } else {
                        panic!("Input axis '{}' is not Emulated", name);
                    }
                } else {
                    if let Axis::Emulated { pos: old_pos, .. } = axis {
                        handler.bindings.insert_axis(
                            name.clone(),
                            Axis::Emulated {
                                pos: old_pos,
                                neg: Button::Key(new_key),
                            },
                        );
                    } else {
                        panic!("Input axis '{}' is not Emulated", name);
                    }
                }
            } else if let ChangeControl::Action { ref name } = *change {
                let current_key = *handler
                    .bindings
                    .action_bindings(name)
                    .unwrap_or_else(|| panic!("Unknown action binding '{}' to change", name))
                    .first()
                    .unwrap_or_else(|| panic!("No binding for action binding '{}'", name));
                handler.bindings.remove_action_binding(name, current_key);
                handler
                    .bindings
                    .insert_action_binding(name.clone(), Button::Key(new_key));
            }
        }
    }
}
