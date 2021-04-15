use super::schema::descriptions;

#[derive(Queryable, AsChangeset)]
pub struct Description<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "descriptions"]
pub struct NewDescription<'a> {
    pub key: &'a str,
    pub value: &'a str,
}
