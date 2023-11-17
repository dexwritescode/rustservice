use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub body: String,
}
