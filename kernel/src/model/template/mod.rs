use uuid::Uuid;

pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub iso: String,
    pub core: usize,
    pub memory: usize,
    pub storage: usize,
}
