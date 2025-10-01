use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Hackathons::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hackathons::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hackathons::Name).string().not_null())
                    .col(
                        ColumnDef::new(Hackathons::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Hackathons::Description).text())
                    .col(ColumnDef::new(Hackathons::StartDate).timestamp().not_null())
                    .col(ColumnDef::new(Hackathons::EndDate).timestamp().not_null())
                    .col(
                        ColumnDef::new(Hackathons::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Hackathons::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Hackathons::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Hackathons::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Hackathons {
    Table,
    Id,
    Name,
    Slug,
    Description,
    StartDate,
    EndDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
