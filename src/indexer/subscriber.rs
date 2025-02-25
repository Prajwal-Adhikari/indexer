use crate::entities::{
    commit_batch, finalize_batch, finalize_transaction, l1_deposit, l1_withdraw, sent_message,
};
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use eyre::Result;
use futures_util::StreamExt;
use sea_orm::{ActiveValue::Set, DatabaseConnection, EntityTrait};
use tracing::{error, info};
use twine_evm_contracts::evm::{
    ethereum::{l1_message_queue::L1MessageQueue, twine_chain::TwineChain},
    twine::l2_messenger::L2Messenger,
};

pub async fn run_indexer(provider: impl Provider, db: DatabaseConnection) -> Result<()> {
    let filter = Filter::new(); // TODO: Filter contract addresses
    let subscription = provider.subscribe_logs(&filter).await?;
    let mut stream = subscription.into_stream();
    info!("Subscribed to log stream. Listening for events...");

    while let Some(log) = stream.next().await {
        match log.topic0() {
            Some(&L2Messenger::SentMessage::SIGNATURE_HASH) => {
                info!("L2Messenger SentMessage event received");
                match log.log_decode::<L2Messenger::SentMessage>() {
                    Ok(decoded) => {
                        let L2Messenger::SentMessage {
                            from,
                            l2Token,
                            to,
                            l1Token,
                            amount,
                            value,
                            nonce,
                            chainId,
                            blockNumber,
                            gasLimit,
                        } = decoded.inner.data;

                        info!("Decoded SentMessage event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed.");
                        let new_event = sent_message::ActiveModel {
                            tx_hash: Set(format!("{hash}")),
                            from: Set(format!("{from}")),
                            l2_token: Set(format!("{l2Token}")),
                            to: Set(format!("{to}")),
                            l1_token: Set(format!("{l1Token}")),
                            amount: Set(format!("{amount}")),
                            nonce: Set(nonce.try_into().unwrap()),
                            chain_id: Set(chainId.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            gas_limit: Set(gasLimit.try_into().unwrap()),
                            created_at: Set(chrono::Utc::now().into()),
                        };

                        if let Err(e) = sent_message::Entity::insert(new_event).exec(&db).await {
                            error!("Error inserting SentMessage event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding SentMessage event: {:?}", e),
                }
            }
            Some(&L1MessageQueue::QueueDepositTransaction::SIGNATURE_HASH) => {
                info!("DepositTx event received");
                match log.log_decode::<L1MessageQueue::QueueDepositTransaction>() {
                    Ok(decoded) => {
                        let L1MessageQueue::QueueDepositTransaction {
                            nonce,
                            chainId,
                            from,
                            amount,
                            blockNumber,
                            l1Token,
                            l2Token,
                            toTwineAddress,
                        } = decoded.inner.data;

                        info!("Decoded L1 Deposit event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed.");
                        let new_event = l1_deposit::ActiveModel {
                            tx_hash: Set(format!("{hash}")),
                            nonce: Set(nonce.try_into().unwrap()),
                            chain_id: Set(chainId.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            l1_token: Set(format!("{l1Token}")),
                            l2_token: Set(format!("{l2Token}")),
                            from: Set(format!("{from}")),
                            to_twine_address: Set(format!("{toTwineAddress}")),
                            amount: Set(format!("{amount}")),
                            created_at: Set(chrono::Utc::now().into()),
                        };

                        if let Err(e) = l1_deposit::Entity::insert(new_event).exec(&db).await {
                            error!("Error inserting L1 Deposit event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding L1 Deposit event: {:?}", e),
                }
            }
            Some(&L1MessageQueue::QueueWithdrawalTransaction::SIGNATURE_HASH) => {
                info!("Withdraw Tx event received");
                match log.log_decode::<L1MessageQueue::QueueWithdrawalTransaction>() {
                    Ok(decoded) => {
                        let L1MessageQueue::QueueWithdrawalTransaction {
                            nonce,
                            chainId,
                            from,
                            amount,
                            blockNumber,
                            l1Token,
                            l2Token,
                            toTwineAddress,
                        } = decoded.inner.data;

                        info!("Decoded L1 Withdrawal event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed.");
                        let new_event = l1_withdraw::ActiveModel {
                            tx_hash: Set(format!("{hash}")),
                            nonce: Set(nonce.try_into().unwrap()),
                            chain_id: Set(chainId.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            l1_token: Set(format!("{l1Token}")),
                            l2_token: Set(format!("{l2Token}")),
                            from: Set(format!("{from}")),
                            to_twine_address: Set(format!("{toTwineAddress}")),
                            amount: Set(format!("{amount}")),
                            created_at: Set(chrono::Utc::now().into()),
                        };

                        if let Err(e) = l1_withdraw::Entity::insert(new_event).exec(&db).await {
                            error!("Error inserting L1 Withdraw event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding L1 Withdraw event: {:?}", e),
                }
            }
            Some(&TwineChain::CommitBatch::SIGNATURE_HASH) => {
                info!("CommitBatch event received");
                match log.log_decode::<TwineChain::CommitBatch>() {
                    Ok(decoded) => {
                        let TwineChain::CommitBatch {
                            startBlock,
                            endBlock,
                            blockNumber,
                            batchId,
                            batchHash,
                        } = decoded.inner.data;
                        info!("Decoded CommitBatch event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed");
                        println!("{hash}");
                        let new_event = commit_batch::ActiveModel {
                            start_block: Set(startBlock.try_into().unwrap()),
                            end_block: Set(endBlock.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            batch_id: Set(format!("{batchId}")),
                            batch_hash: Set(format!("{batchHash}")),
                            created_at: Set(chrono::Utc::now().into()),
                        };
                        if let Err(e) = commit_batch::Entity::insert(new_event).exec(&db).await {
                            error!("Error inserting commit batch event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding commit batch event: {:?}", e),
                }
            }

            Some(&TwineChain::FinalizedBatch::SIGNATURE_HASH) => {
                info!("FinalizedBatch event received");
                match log.log_decode::<TwineChain::FinalizedBatch>() {
                    Ok(decoded) => {
                        let TwineChain::FinalizedBatch {
                            startBlock,
                            endBlock,
                            blockNumber,
                            batchId,
                            batchHash,
                        } = decoded.inner.data;
                        info!("Decoded FinalizedBatch event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed");
                        println!("{hash}");

                        let new_event = finalize_batch::ActiveModel {
                            start_block: Set(startBlock.try_into().unwrap()),
                            end_block: Set(endBlock.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            batch_id: Set(format!("{batchId}")),
                            batch_hash: Set(format!("{batchHash}")),
                            created_at: Set(chrono::Utc::now().into()),
                        };
                        if let Err(e) = finalize_batch::Entity::insert(new_event).exec(&db).await {
                            error!("Error inserting finalize batch event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding finalize batch event: {:?}", e),
                }
            }

            Some(&TwineChain::FinalizedTransaction::SIGNATURE_HASH) => {
                info!("FinalizedTransaction event received");
                match log.log_decode::<TwineChain::FinalizedTransaction>() {
                    Ok(decoded) => {
                        let TwineChain::FinalizedTransaction {
                            startBlock,
                            endBlock,
                            blockNumber,
                            batchId,
                        } = decoded.inner.data;
                        info!("Decoded FinalizedTransaction event");
                        let hash = log.transaction_hash.expect("tx hash could not be parsed");
                        //prz: just for test
                        println!("{hash}");

                        let new_event = finalize_transaction::ActiveModel {
                            start_block: Set(startBlock.try_into().unwrap()),
                            end_block: Set(endBlock.try_into().unwrap()),
                            block_number: Set(blockNumber.try_into().unwrap()),
                            batch_id: Set(format!("{batchId}")),
                            created_at: Set(chrono::Utc::now().into()),
                        };
                        if let Err(e) = finalize_transaction::Entity::insert(new_event)
                            .exec(&db)
                            .await
                        {
                            error!("Error inserting finalize transaction event: {:?}", e);
                        }
                    }
                    Err(e) => error!("Error decoding finalize transaction event: {:?}", e),
                }
            }

            _ => {}
        }
    }

    Ok(())
}
