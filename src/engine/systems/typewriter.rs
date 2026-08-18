use hecs::World;
use crate::engine::components::{Text, Typewriter};

pub fn typewriter_system(world: &mut World, dt: f32) {
    for (text, tw) in world.query_mut::<(&mut Text, &mut Typewriter)>() {
        tw.timer += dt;

        let total = tw.full_text.chars().count();
        if total == 0 {
            text.content.clear();
            continue;
        }

        let step = (tw.timer * tw.chars_per_sec) as usize;

        if tw.going_forward {
            tw.visible = step.min(total);
            if tw.visible >= total {
                if tw.looped {
                    tw.going_forward = false;
                    tw.timer = 0.0;
                } else {
                    tw.visible = total;
                }
            }
        } else {
            let back = step.min(total);
            tw.visible = total.saturating_sub(back);
            if tw.visible == 0 && tw.looped {
                tw.going_forward = true;
                tw.timer = 0.0;
            }
        }

        text.content = tw.full_text.chars().take(tw.visible).collect();
    }
}