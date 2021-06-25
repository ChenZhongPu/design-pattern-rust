use crate::MenuItem;

pub trait Menu {
    fn iter(&self) -> std::slice::Iter<'_, MenuItem>;
}