use sea_orm::entity::prelude::*;
//use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name="ai_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role: String, 
    pub content: String
}


#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}

