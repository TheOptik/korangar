use procedural::*;

use crate::graphics::{InterfaceRenderer, Renderer};
use crate::input::MouseInputMode;
use crate::interface::*;
use crate::inventory::Item;
use crate::network::EquipPosition;

pub struct EquipmentContainer {
    items: Remote<Vec<Item>>,
    weak_self: Option<WeakElementCell>, // TODO: maybe remove?
    state: ContainerState,
}

impl EquipmentContainer {
    pub fn new(items: Remote<Vec<Item>>) -> Self {
        const SLOT_POSITIONS: [EquipPosition; 9] = [
            EquipPosition::HeadTop,
            EquipPosition::HeadMiddle,
            EquipPosition::HeadLower,
            EquipPosition::Armor,
            EquipPosition::Garment,
            EquipPosition::Shoes,
            EquipPosition::LeftHand,
            EquipPosition::RightHand,
            EquipPosition::Ammo,
        ];

        let elements = {
            let items = items.borrow();

            (0..SLOT_POSITIONS.len())
                .map(|index| {
                    let slot = SLOT_POSITIONS[index];

                    let text = Text::default()
                        .with_text(slot.display_name().to_string())
                        .with_foreground_color(|_| Color::monochrome_u8(200))
                        .with_width(dimension!(!))
                        .wrap();

                    let item = items.iter().find(|item| item.equipped_position == slot).cloned();

                    let item_box = ItemBox::new(
                        item,
                        ItemSource::Equipment { position: slot },
                        Box::new(move |mouse_mode| matches!(mouse_mode, MouseInputMode::MoveItem(_, item) if item.equip_position == slot)),
                    );

                    Container::new(vec![item_box.wrap(), text]).wrap()
                })
                .collect()
        };

        let weak_self = None;
        let state = ContainerState::new(elements);

        Self { items, weak_self, state }
    }
}

impl Element for EquipmentContainer {
    fn get_state(&self) -> &ElementState {
        &self.state.state
    }

    fn get_state_mut(&mut self) -> &mut ElementState {
        &mut self.state.state
    }

    fn link_back(&mut self, weak_self: WeakElementCell, weak_parent: Option<WeakElementCell>) {
        self.weak_self = Some(weak_self.clone());
        self.state.link_back(weak_self, weak_parent);
    }

    fn is_focusable(&self) -> bool {
        self.state.is_focusable::<false>()
    }

    fn focus_next(&self, self_cell: ElementCell, caller_cell: Option<ElementCell>, focus: Focus) -> Option<ElementCell> {
        self.state.focus_next::<false>(self_cell, caller_cell, focus)
    }

    fn restore_focus(&self, self_cell: ElementCell) -> Option<ElementCell> {
        self.state.restore_focus(self_cell)
    }

    fn resolve(&mut self, placement_resolver: &mut PlacementResolver, interface_settings: &InterfaceSettings, theme: &InterfaceTheme) {
        let size_constraint = &constraint!(100%, ?);
        self.state.resolve(
            placement_resolver,
            interface_settings,
            theme,
            size_constraint,
            ScreenSize::uniform(3.0),
        );
    }

    fn update(&mut self) -> Option<ChangeEvent> {
        if self.items.consume_changed() {
            let weak_parent = self.state.state.parent_element.take();
            let weak_self = self.weak_self.take().unwrap();

            *self = Self::new(self.items.clone());
            // important: link back after creating elements, otherwise focus navigation and
            // scrolling would break
            self.link_back(weak_self, weak_parent);

            return Some(ChangeEvent::RESOLVE_WINDOW);
        }

        None
    }

    fn hovered_element(&self, mouse_position: ScreenPosition, mouse_mode: &MouseInputMode) -> HoverInformation {
        match mouse_mode {
            MouseInputMode::MoveItem(..) | MouseInputMode::None => self.state.hovered_element(mouse_position, mouse_mode, false),
            _ => HoverInformation::Missed,
        }
    }

    fn render(
        &self,
        render_target: &mut <InterfaceRenderer as Renderer>::Target,
        renderer: &InterfaceRenderer,
        state_provider: &StateProvider,
        interface_settings: &InterfaceSettings,
        theme: &InterfaceTheme,
        parent_position: ScreenPosition,
        screen_clip: ScreenClip,
        hovered_element: Option<&dyn Element>,
        focused_element: Option<&dyn Element>,
        mouse_mode: &MouseInputMode,
        second_theme: bool,
    ) {
        let mut renderer = self
            .state
            .state
            .element_renderer(render_target, renderer, interface_settings, parent_position, screen_clip);

        self.state.render(
            &mut renderer,
            state_provider,
            interface_settings,
            theme,
            hovered_element,
            focused_element,
            mouse_mode,
            second_theme,
        );
    }
}
