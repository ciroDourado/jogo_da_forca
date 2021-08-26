#[derive(Deserialize)]
pub struct Significado {
    creator      : Option<String>,
    deleted      : Option<i32>,
    deletor      : Option<String>,
    derived_from : Option<String>,
    last_revision: Option<i32>,
    moderator    : Option<String>,
    normalized   : Option<String>,
    revision_id  : Option<i32>,
    sense        : Option<i32>,
    timestamp    : Option<String>,
    word         : String,
    word_id      : Option<i32>,
    xml          : String
} // struct Significado

type Significados = Vec<Significado>;
