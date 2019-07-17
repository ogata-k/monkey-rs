/// オブジェクトシステム上で管理するための型情報
pub enum ObjectType {
    StringObject(String),
}

/// オブジェクトシステム上で扱うオブジェクト情報
trait Object {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}