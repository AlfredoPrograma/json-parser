pub enum ElementKind {
    Boolean,
    Null,
    Number,
    String,
}

pub struct Element<'a, T> {
    kind: ElementKind,
    key: &'a str,
    value: T,
}
