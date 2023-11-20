#[derive(PartialEq, Debug)]
pub enum NumberType {
    Integer(i32),
    Float(f32),
}

#[derive(PartialEq, Debug)]
pub enum JsonValue {
    Boolean(bool),
    Number(NumberType),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
    Null(Box<Option<JsonValue>>),
}
