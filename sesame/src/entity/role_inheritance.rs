//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1


use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "role_inheritance")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub parent_role_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::ParentRoleId",
        to = "super::roles::Column::RoleId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Roles2,
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::RoleId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Roles1,
}

impl ActiveModelBehavior for ActiveModel {}
