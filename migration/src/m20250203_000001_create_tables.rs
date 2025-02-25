use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create L1Deposit table
        manager
            .create_table(
                Table::create()
                    .table(L1Deposit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(L1Deposit::TxHash)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(L1Deposit::Nonce).big_unsigned().not_null())
                    .col(ColumnDef::new(L1Deposit::ChainId).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(L1Deposit::BlockNumber)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(L1Deposit::L1Token).string_len(42).not_null())
                    .col(ColumnDef::new(L1Deposit::L2Token).string_len(42).not_null())
                    .col(ColumnDef::new(L1Deposit::From).string_len(42).not_null())
                    .col(
                        ColumnDef::new(L1Deposit::ToTwineAddress)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(ColumnDef::new(L1Deposit::Amount).string_len(64).not_null())
                    .col(
                        ColumnDef::new(L1Deposit::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create L1Withdraw table
        manager
            .create_table(
                Table::create()
                    .table(L1Withdraw::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(L1Withdraw::TxHash)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(L1Withdraw::Nonce).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(L1Withdraw::ChainId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(L1Withdraw::BlockNumber)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(L1Withdraw::L1Token)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(L1Withdraw::L2Token)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(ColumnDef::new(L1Withdraw::From).string_len(42).not_null())
                    .col(
                        ColumnDef::new(L1Withdraw::ToTwineAddress)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(ColumnDef::new(L1Withdraw::Amount).string_len(64).not_null())
                    .col(
                        ColumnDef::new(L1Deposit::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create SentMessage table
        manager
            .create_table(
                Table::create()
                    .table(SentMessage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SentMessage::TxHash)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SentMessage::From).string_len(42).not_null())
                    .col(
                        ColumnDef::new(SentMessage::L2Token)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(ColumnDef::new(SentMessage::To).text().not_null())
                    .col(
                        ColumnDef::new(SentMessage::L1Token)
                            .string_len(42)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SentMessage::Amount)
                            .string_len(64)
                            .not_null(),
                    )
                    .col(ColumnDef::new(SentMessage::Nonce).big_unsigned().not_null())
                    .col(
                        ColumnDef::new(SentMessage::ChainId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SentMessage::BlockNumber)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SentMessage::GasLimit)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(L1Deposit::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        //prz: Cretae CommitBatch table
        manager
            .create_table(
                Table::create()
                    .table(CommitBatch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommitBatch::BatchId)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommitBatch::StartBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitBatch::EndBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitBatch::BlockNumber)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitBatch::BatchHash)
                            .string_len(66)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommitBatch::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create FinalizeBatch table
        manager
            .create_table(
                Table::create()
                    .table(FinalizeBatch::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FinalizeBatch::BatchId)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FinalizeBatch::StartBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeBatch::EndBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeBatch::BlockNumber)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeBatch::BatchHash)
                            .string_len(66)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeBatch::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create FinalizeTransaction table
        manager
            .create_table(
                Table::create()
                    .table(FinalizeTransaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FinalizeTransaction::BatchId)
                            .string_len(66)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FinalizeTransaction::StartBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeTransaction::EndBlock)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeTransaction::BlockNumber)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FinalizeTransaction::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(L1Deposit::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(L1Withdraw::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SentMessage::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CommitBatch::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FinalizeBatch::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FinalizeTransaction::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum L1Deposit {
    Table,
    TxHash,         // bytes32 -> char_len(66) (0x-prefixed hex)
    Nonce,          // uint64 -> big_unsigned()
    ChainId,        // uint64 -> big_unsigned()
    BlockNumber,    // uint64 -> big_unsigned()
    L1Token,        // address -> char_len(42) (0x-prefixed hex)
    L2Token,        // address -> char_len(42)
    From,           // address -> char_len(42)
    ToTwineAddress, // address -> char_len(42)
    Amount,         // uint256 -> char_len(64)
    CreatedAt,      // timestamp_with_time_zone() -> DEFAULT CURRENT_TIMESTAMP
}

#[derive(DeriveIden)]
enum L1Withdraw {
    Table,
    TxHash,         // bytes32 -> char_len(66) (0x-prefixed hex)
    Nonce,          // uint64 -> big_unsigned()
    ChainId,        // uint64 -> big_unsigned()
    BlockNumber,    // uint64 -> big_unsigned()
    L1Token,        // address -> char_len(42) (0x-prefixed hex)
    L2Token,        // address -> char_len(42)
    From,           // address -> char_len(42)
    ToTwineAddress, // address -> char_len(42)
    Amount,         // uint256 -> char_len(64)
    CreatedAt,      // timestamp_with_time_zone() -> DEFAULT CURRENT_TIMESTAMP
}

#[derive(DeriveIden)]
enum SentMessage {
    Table,
    TxHash,      // bytes32 -> char_len(66) (0x-prefixed hex)
    From,        // address -> char_len(42) (0x-prefixed hex)
    L2Token,     // address -> char_len(42)
    To,          // string -> char_len(64) (message recipient)
    L1Token,     // address -> char_len(42)
    Amount,      // uint256 -> char_len(64)
    Nonce,       // uint64 -> big_unsigned()
    ChainId,     // uint64 -> big_unsigned()
    BlockNumber, // uint64 -> big_unsigned()
    GasLimit,    // uint64 -> big_unsigned()
    CreatedAt,   // timestamp_with_time_zone() -> DEFAULT CURRENT_TIMESTAMP
}

#[derive(Iden)]
pub enum CommitBatch {
    Table,
    StartBlock,
    EndBlock,
    BlockNumber,
    BatchId,
    BatchHash,
    CreatedAt,
}

#[derive(Iden)]
pub enum FinalizeBatch {
    Table,
    StartBlock,
    EndBlock,
    BlockNumber,
    BatchId,
    BatchHash,
    CreatedAt,
}

#[derive(Iden)]
pub enum FinalizeTransaction {
    Table,
    StartBlock,
    EndBlock,
    BlockNumber,
    BatchId,
    CreatedAt,
}
