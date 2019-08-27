// 定数
const NULL_OBJECT: &str = "NULL";
const INTEGER_OBJECT: &str = "INTEGER";
const BOOLEAN_OBJECT: &str = "BOOLEAN";

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

    pub fn is_integer(&self) -> bool {
        &self.object_type == INTEGER_OBJECT
    }
    pub fn is_boolean(&self) -> bool {
        &self.object_type == BOOLEAN_OBJECT
    }
    pub fn is_null(&self) -> bool {
        &self.object_type == NULL_OBJECT
    }
}

impl ToString for ObjectType {
    fn to_string(&self) -> String {
        self.object_type.to_string()
    }
}

/// オブジェクトシステム上で扱うオブジェクト情報
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum Object {
    Null,
    Integer { value: i64 },
    Boolean { value: bool },
}

impl ToString for Object {
    fn to_string(&self) -> String {
        use Object::*;
        match self {
            Null => "null".to_string(),
            Integer { value: v } => format!("{}", v),
            Boolean { value: v } => format!("{}", v),
        }
    }
}

impl Object {
    pub const BOOLEAN_TRUE: Object = Object::Boolean { value: true };
    pub const BOOLEAN_FALSE: Object = Object::Boolean { value: false };
    pub const NULL: Object = Object::Null;

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

    pub fn is_truthy(&self) -> bool{
        let object_type = self.get_type();
        if object_type.is_null(){
            return true;
        }
        if let Object::Boolean { value } = self {
            return *value;
        }

        true
    }
}
