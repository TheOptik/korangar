use std::cmp::PartialOrd;

use derive_new::new;
use num::traits::NumOps;
use num::{NumCast, Zero};
use procedural::*;

use crate::interface::*;

#[derive(new)]
pub struct NumberWindow<T> {
    name: String,
    inner_pointer: *const T,
    minimum_value: T,
    maximum_value: T,
    change_event: Option<ChangeEvent>,
}

impl<T: Zero + NumOps + NumCast + Copy + PartialOrd + 'static> PrototypeWindow for NumberWindow<T> {
    fn to_window(&self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, avalible_space: Size) -> Window {
        let elements: Vec<ElementCell> = vec![
            cell!(Headline::new("value".to_string(), Headline::DEFAULT_SIZE)),
            cell!(Slider::new(
                unsafe { &(*self.inner_pointer) as *const T },
                self.minimum_value,
                self.maximum_value,
                self.change_event
            )),
        ];

        Window::new(
            window_cache,
            interface_settings,
            avalible_space,
            self.name.clone(),
            None,
            elements,
            constraint!(200 > 250 < 300, ?),
            true,
        )
    }
}
