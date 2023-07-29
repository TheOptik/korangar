use std::fmt::Display;
use std::net::Ipv4Addr;
use std::rc::Rc;

use cgmath::{Quaternion, Rad, Vector2, Vector3, Vector4};

use crate::graphics::Color;
use crate::interface::{ElementCell, *};

pub trait PrototypeElement {
    fn to_element(&self, display: String) -> ElementCell;
}

pub trait ElementDisplay {
    fn display(&self) -> String;
}

// workaround for not having negative trait bounds or better specialization
auto trait NoDisplay {}
impl !NoDisplay for f32 {}
impl<T> !NoDisplay for Vector2<T> {}
impl<T> !NoDisplay for Vector3<T> {}
impl<T> !NoDisplay for Vector4<T> {}
impl<T> !NoDisplay for Quaternion<T> {}
impl<T> !NoDisplay for Rad<T> {}
impl !NoDisplay for Ipv4Addr {}

impl<T> ElementDisplay for T
where
    T: Display + NoDisplay,
{
    fn display(&self) -> String {
        self.to_string()
    }
}

impl ElementDisplay for f32 {
    fn display(&self) -> String {
        format!("{self:.1}")
    }
}

impl<T: ElementDisplay> ElementDisplay for Vector2<T> {
    fn display(&self) -> String {
        format!("{}, {}", self.x.display(), self.y.display())
    }
}

impl<T: ElementDisplay> ElementDisplay for Vector3<T> {
    fn display(&self) -> String {
        format!("{}, {}, {}", self.x.display(), self.y.display(), self.z.display())
    }
}

impl<T: ElementDisplay> ElementDisplay for Vector4<T> {
    fn display(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.x.display(),
            self.y.display(),
            self.z.display(),
            self.w.display()
        )
    }
}

impl<T: ElementDisplay> ElementDisplay for Quaternion<T> {
    fn display(&self) -> String {
        format!(
            "{:.1}, {:.1}, {:.1} - {:.1}",
            self.v.x.display(),
            self.v.y.display(),
            self.v.z.display(),
            self.s.display()
        )
    }
}

impl<T: ElementDisplay> ElementDisplay for Rad<T> {
    fn display(&self) -> String {
        self.0.display()
    }
}

impl ElementDisplay for Ipv4Addr {
    fn display(&self) -> String {
        self.to_string()
    }
}

/*impl ElementDisplay for ModelVertexBuffer {

    fn display(&self) -> String {

        use vulkano::buffer::BufferAccess;

        let identifier = self.inner().buffer.key();
        let size = self.inner().buffer.size();
        format!("{} ({})", identifier, size)
    }
}

impl ElementDisplay for Texture {

    fn display(&self) -> String {

        use vulkano::{Handle, VulkanObject};

        let identifier = self.internal_object().as_raw();
        format!("0x{:x}", identifier)
    }
}*/

// workaround for not having negative trait bounds or better specialization
auto trait NoPrototype {}
impl<T> !NoPrototype for std::sync::Arc<T> {}
impl<T> !NoPrototype for Option<T> {}
impl<T, const N: usize> !NoPrototype for [T; N] {}
impl<T> !NoPrototype for &[T] {}
impl<T> !NoPrototype for Vec<T> {}
impl<T> !NoPrototype for Rc<T> {}

impl NoPrototype for &str {}
impl NoPrototype for String {}
impl NoPrototype for Ipv4Addr {}

impl<T> PrototypeElement for T
where
    T: ElementDisplay + NoPrototype,
{
    fn to_element(&self, display: String) -> ElementCell {
        let elements = vec![StaticLabel::new(display).wrap(), StringValue::new(self.display()).wrap()];

        Container::new(elements).wrap()
    }
}

impl PrototypeElement for DimensionConstraint {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = vec![StaticLabel::new(display).wrap()];

        Container::new(elements).wrap()
    }
}

impl PrototypeElement for SizeConstraint {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = vec![StaticLabel::new(display).wrap()];

        Container::new(elements).wrap()
    }
}

impl<T: PrototypeElement> PrototypeElement for std::sync::Arc<T> {
    fn to_element(&self, display: String) -> ElementCell {
        self.as_ref().to_element(display)
    }
}

impl<T: PrototypeElement> PrototypeElement for Option<T> {
    fn to_element(&self, display: String) -> ElementCell {
        if let Some(value) = self {
            return value.to_element(display);
        }

        let elements = vec![StaticLabel::new(display).wrap(), StringValue::new("none".to_string()).wrap()];

        Container::new(elements).wrap()
    }
}

impl<T: PrototypeElement> PrototypeElement for &[T] {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = self
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_element(index.to_string()))
            .collect();

        Expandable::new(display, elements, false).wrap()
    }
}

impl<T: PrototypeElement, const SIZE: usize> PrototypeElement for [T; SIZE] {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = self
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_element(index.to_string()))
            .collect();

        Expandable::new(display, elements, false).wrap()
    }
}

impl<T: PrototypeElement> PrototypeElement for Vec<T> {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = self
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_element(index.to_string()))
            .collect();

        Expandable::new(display, elements, false).wrap()
    }
}

impl PrototypeElement for Color {
    fn to_element(&self, display: String) -> ElementCell {
        let elements = vec![StaticLabel::new(display).wrap(), ColorValue::new(*self).wrap()];

        Container::new(elements).wrap()
    }
}

impl<T: PrototypeElement> PrototypeElement for Rc<T> {
    fn to_element(&self, display: String) -> ElementCell {
        (**self).to_element(display)
    }
}
