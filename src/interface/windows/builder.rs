use procedural::dimension_bound;

use crate::interface::*;

pub struct NotSet;
pub struct Set;
pub struct SetWith<T>(T);

/// Type state window builder. This builder utilizes the type system to prevent
/// calling the same method multiple times, calling `build` before the mandatory
/// methods have been called, and to enforce some conditional logic. Namely, the
/// `closable` method can only be called if the window has a title.
#[must_use = "WindowBuilder must be finalized"]
pub struct WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> {
    title: Option<String>,
    closable: bool,
    class: Option<String>,
    size_bound: SIZE,
    elements: ELEMENTS,
    background_color: Option<ColorSelector>,
    theme_kind: ThemeKind,
    marker: PhantomData<(TITLE, CLOSABLE, CLASS, BACKGROUND, THEME)>,
}

impl WindowBuilder<NotSet, NotSet, NotSet, NotSet, NotSet, NotSet, NotSet> {
    pub fn new() -> Self {
        Self {
            title: None,
            closable: false,
            class: None,
            size_bound: NotSet,
            elements: NotSet,
            background_color: None,
            theme_kind: ThemeKind::default(),
            marker: PhantomData,
        }
    }
}

impl<CLASS, CLOSABLE, SIZE, ELEMENTS, BACKGROUND, THEME> WindowBuilder<NotSet, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> {
    pub fn with_title(self, title: impl Into<String>) -> WindowBuilder<Set, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> {
        WindowBuilder {
            title: Some(title.into()),
            marker: PhantomData,
            ..self
        }
    }
}

impl<CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> WindowBuilder<Set, NotSet, CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> {
    /// NOTE: This function is only available if `with_title` has been called on
    /// the builder.
    pub fn closable(self) -> WindowBuilder<Set, Set, CLASS, SIZE, ELEMENTS, BACKGROUND, THEME> {
        WindowBuilder {
            closable: true,
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, SIZE, ELEMENTS, BACKGROUND, THEME> WindowBuilder<TITLE, CLOSABLE, NotSet, SIZE, ELEMENTS, BACKGROUND, THEME> {
    pub fn with_class(self, class: impl Into<String>) -> WindowBuilder<TITLE, CLOSABLE, Set, SIZE, ELEMENTS, BACKGROUND, THEME> {
        WindowBuilder {
            class: Some(class.into()),
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, SIZE, ELEMENTS, BACKGROUND, THEME> WindowBuilder<TITLE, CLOSABLE, NotSet, SIZE, ELEMENTS, BACKGROUND, THEME> {
    pub fn with_class_option(self, class: Option<String>) -> WindowBuilder<TITLE, CLOSABLE, Set, SIZE, ELEMENTS, BACKGROUND, THEME> {
        WindowBuilder {
            class,
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, CLASS, ELEMENTS, BACKGROUND, THEME> WindowBuilder<TITLE, CLOSABLE, CLASS, NotSet, ELEMENTS, BACKGROUND, THEME> {
    pub fn with_size_bound(
        self,
        size_bound: SizeBound,
    ) -> WindowBuilder<TITLE, CLOSABLE, CLASS, SetWith<SizeBound>, ELEMENTS, BACKGROUND, THEME> {
        WindowBuilder {
            size_bound: SetWith(size_bound),
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, CLASS, SIZE, BACKGROUND, THEME> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, NotSet, BACKGROUND, THEME> {
    pub fn with_elements(
        self,
        elements: Vec<ElementCell>,
    ) -> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, SetWith<Vec<ElementCell>>, BACKGROUND, THEME> {
        WindowBuilder {
            elements: SetWith(elements),
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, THEME> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, NotSet, THEME> {
    pub fn with_background_color(
        self,
        background_color: ColorSelector,
    ) -> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, Set, THEME> {
        WindowBuilder {
            background_color: Some(background_color),
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND, NotSet> {
    pub fn with_theme_kind(self, theme_kind: ThemeKind) -> WindowBuilder<TITLE, CLOSABLE, CLASS, SIZE, ELEMENTS, BACKGROUND, Set> {
        WindowBuilder {
            theme_kind,
            marker: PhantomData,
            ..self
        }
    }
}

impl<TITLE, CLOSABLE, CLASS, BACKGROUND, THEME>
    WindowBuilder<TITLE, CLOSABLE, CLASS, SetWith<SizeBound>, SetWith<Vec<ElementCell>>, BACKGROUND, THEME>
{
    /// Take the builder and turn it into a [`Window`].
    /// NOTE: This method is only available if `with_size_bound` and
    /// `with_elements` has been called on the builder.
    pub fn build(self, window_cache: &WindowCache, interface_settings: &InterfaceSettings, available_space: ScreenSize) -> Window {
        let Self {
            title,
            closable,
            class,
            size_bound,
            elements,
            background_color,
            theme_kind,
            ..
        } = self;

        let size_bound = size_bound.0;
        let mut elements = elements.0;

        if closable {
            let close_button = CloseButton::default().wrap();
            elements.insert(0, close_button);
        }

        if let Some(title) = title {
            let width_bound = match closable {
                true => dimension_bound!(70%),
                false => dimension_bound!(!),
            };

            let drag_button = DragButton::new(title, width_bound).wrap();
            elements.insert(0, drag_button);
        }

        let container_size_bound = SizeBound {
            width: Dimension::Relative(100.0),
            minimum_width: size_bound.minimum_width.map(|_| Dimension::Super),
            maximum_width: size_bound.maximum_width.map(|_| Dimension::Super),
            height: Dimension::Flexible,
            minimum_height: size_bound.minimum_height.map(|_| Dimension::Super),
            maximum_height: size_bound.maximum_height.map(|_| Dimension::Super),
        };
        let elements = vec![Container::new(elements).with_size(container_size_bound).wrap()];

        // Very imporant: give every element a link to its parent to allow propagation
        // of events such as scrolling.
        elements.iter().for_each(|element| {
            let weak_element = Rc::downgrade(element);
            element.borrow_mut().link_back(weak_element, None);
        });

        let (cached_position, cached_size) = class
            .as_ref()
            .and_then(|window_class| window_cache.get_window_state(window_class))
            .unzip();

        let size = cached_size
            .map(|size| size_bound.validated_window_size(size, available_space, interface_settings.scaling.get()))
            .unwrap_or_else(|| {
                size_bound
                    .resolve_window(available_space, available_space, interface_settings.scaling.get())
                    .finalize_or(0.0)
            });

        let position = cached_position
            .map(|position| size_bound.validated_position(position, size, available_space))
            .unwrap_or(ScreenPosition::from_size((available_space - size) / 2.0));

        Window {
            window_class: class,
            position,
            size_bound,
            size,
            elements,
            popup_element: None,
            closable,
            background_color,
            theme_kind,
        }
    }
}
