#[derive(PartialEq, Debug)]
pub enum NumberKind {
    Integer(i32),
    Float(f32),
}

#[derive(PartialEq, Debug)]
pub enum ElementKind {
    Boolean(bool),
    Number(NumberKind),
    String(String),
}

#[derive(PartialEq, Debug)]
pub enum GenericElementKind<T> {
    Null(Option<T>),
}

#[derive(PartialEq, Debug)]
pub enum ComposedElementKind<T> {
    Generic(GenericElementKind<T>),
    Simple(ElementKind),
}

pub struct Element<T> {
    kind: ComposedElementKind<T>,
    key: String,
    value: T,
}
