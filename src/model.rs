pub struct Todo {
    pub created_time: String,
    pub content: String,
}

impl Todo {
    pub fn new(line: String) -> Todo {
        let v = line.split('\t').collect::<Vec<_>>();
        Todo {
            content: v[0].to_string(),
            created_time: v[1].to_string(),
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}\t{}", self.content, self.created_time)
    }
}
