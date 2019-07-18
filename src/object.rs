// 定数
pub const NULL_OBJECT: &str = "NULL";
pub const INTEGER_OBJECT: &str = "INTEGER";
pub const BOOLEAN_OBJECT: &str = "BOOLEAN";

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

    pub fn boolean_object_type() -> Self {
        ObjectType {
            object_type: BOOLEAN_OBJECT.to_string(),
        }
    }

    pub fn null_object_type() -> Self {
        ObjectType {
            object_type: NULL_OBJECT.to_string(),
        }
    }
}

/// オブジェクトシステム上で扱うオブジェクト情報
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Object {
    Null,
    Integer { value: i64 },
    Boolean { value: bool },
}

impl Object {
    pub fn get_type(&self) -> ObjectType {
        match self {
            Object::Null => ObjectType::null_object_type(),
            Object::Integer { value: _ } => ObjectType::integer_object_type(),
            Object::Boolean { value: _ } => ObjectType::boolean_object_type(),
        }
    }
    pub fn inspect(&self) -> String {
        match self {
            Object::Null => "null".to_string(),
            Object::Integer { value } => format!("{}", value),
            Object::Boolean { value } => format!("{}", value),
        }
    }
}
