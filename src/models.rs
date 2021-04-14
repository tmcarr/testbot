use super::schema::descriptions;

#[derive(Insertable)]
#[table_name="descriptions"]
pub struct Description<'a> {
  pub key: &'a str,
  pub value: &'a str,
}