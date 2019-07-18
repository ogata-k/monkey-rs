// 定数
pub const INTEGER_OBJECT: &str = "INTEGER";

/// オブジェクトシステム上で管理するための型情報
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct ObjectType {
    object_type: String,
}

impl ObjectType {
    pub fn integer_object_type() -> Self {
        ObjectType {
            object_type: INTEGER_OBJECT.to_string(),
        }
    }
}

/// オブジェクトシステム上で扱うオブジェクト情報
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Object {
    Integer { value: i64 },
}

impl Object {
    pub fn get_type(&self) -> ObjectType {
        match self {
            Object::Integer { value: _ } => ObjectType::integer_object_type(),
        }
    }
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => format!("{}", value),
        }
    }
}
