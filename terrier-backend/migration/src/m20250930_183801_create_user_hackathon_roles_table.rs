use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserHackathonRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserHackathonRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserHackathonRoles::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserHackathonRoles::HackathonId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserHackathonRoles::Role).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserHackathonRoles::Table, UserHackathonRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserHackathonRoles::Table, UserHackathonRoles::HackathonId)
                            .to(Hackathons::Table, Hackathons::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint for user-hackathon combination
        manager
            .create_index(
                Index::create()
                    .name("idx_user_hackathon_unique")
                    .table(UserHackathonRoles::Table)
                    .col(UserHackathonRoles::UserId)
                    .col(UserHackathonRoles::HackathonId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserHackathonRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserHackathonRoles {
    Table,
    Id,
    UserId,
    HackathonId,
    Role,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
}
