use crate::services::db::Repo;

pub fn init_repo() -> impl Repo {
    // In a more involved example, check for flags:
    crate::services::db::LiteDb::new()
}
