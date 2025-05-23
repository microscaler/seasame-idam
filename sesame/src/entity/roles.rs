//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1


use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub role_id: Uuid,
    pub org_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organizations::Entity",
        from = "Column::OrgId",
        to = "super::organizations::Column::OrgId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Organizations,
    #[sea_orm(has_many = "super::role_permissions::Entity")]
    RolePermissions,
    #[sea_orm(has_many = "super::user_roles::Entity")]
    UserRoles,
}

impl Related<super::organizations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organizations.def()
    }
}

impl Related<super::role_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolePermissions.def()
    }
}

impl Related<super::user_roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserRoles.def()
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permissions::Relation::Permissions.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::role_permissions::Relation::Roles.def().rev())
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_roles::Relation::Users.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_roles::Relation::Roles.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
