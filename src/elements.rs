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
    Array(Vec<ElementKind>),
    Object(Vec<(String, ElementKind)>),
    Null(Box<Option<ElementKind>>),
}
