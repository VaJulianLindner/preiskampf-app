pub enum DetailOperations {
    Create,
    Read,
    Update,
    Delete,
}

impl DetailOperations {
    pub fn from_string(str: String) -> Option<Self> {
        match str.as_str() {
            "anlegen" => Some(DetailOperations::Create),
            "detail" => Some(DetailOperations::Read),
            "update" => Some(DetailOperations::Update),
            "entfernen" => Some(DetailOperations::Delete),
            _ => None,
        }
    }

    pub fn _to_string(&self) -> String {
        match self {
            DetailOperations::Create => "anlegen".to_string(),
            DetailOperations::Read => "detail".to_string(),
            DetailOperations::Update => "update".to_string(),
            DetailOperations::Delete => "entfernen".to_string()
        }
    }
}