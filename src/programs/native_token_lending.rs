use chrono::NaiveDateTime;
use solana_sdk::pubkey::Pubkey;
use spl_token_lending::instruction::LendingInstruction;
use tracing::error;

use crate::{Instruction, InstructionFunction, InstructionProperty, InstructionSet};

pub async fn process_native_token_lending_instruction(
    // The transaction that has this instruction.
    transaction_hash: &String,
    // The unique index of the instruction for the transaction involved.
    instruction_index: &i16,
    // The data relating to this instruction
    data: &[u8],
    // The time the transactions and its block were created
    timestamp: &NaiveDateTime,
    // The parent instruction, if any; Frequently used for InnerInstructions
    parent_index: &i16,
) -> Option<InstructionSet> {
    // Unpack the instruction via the spl_token_swap library
    let unpack_result = LendingInstruction::unpack(data);

    if let Ok(lending_instruction) = unpack_result {
        return match lending_instruction {
            LendingInstruction::InitLendingMarket {
                owner,
                quote_currency,
            } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "init-lending-market".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "owner".to_string(),
                            value: owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "quote_currency".to_string(),
                            value: Pubkey::new_from_array(quote_currency).to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::SetLendingMarketOwner { new_owner } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "set-lending-market-owner".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "new_owner".to_string(),
                            value: new_owner.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::InitReserve {
                liquidity_amount,
                config,
            } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "init-reserve".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidity_amount".to_string(),
                            value: liquidity_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "flash_loan_fee_wad".to_string(),
                            value: config.fees.flash_loan_fee_wad.to_string(),
                            parent_key: "fees".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "borrow_fee_wad".to_string(),
                            value: config.fees.borrow_fee_wad.to_string(),
                            parent_key: "config/fees".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "host_fee_percentage".to_string(),
                            value: config.fees.host_fee_percentage.to_string(),
                            parent_key: "config/fees".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidation_threshold".to_string(),
                            value: config.liquidation_threshold.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "loan_to_value_ratio".to_string(),
                            value: config.loan_to_value_ratio.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "max_borrow_rate".to_string(),
                            value: config.max_borrow_rate.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "min_borrow_rate".to_string(),
                            value: config.min_borrow_rate.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "optimal_borrow_rate".to_string(),
                            value: config.optimal_borrow_rate.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        },
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "optimal_utilization_rate".to_string(),
                            value: config.optimal_utilization_rate.to_string(),
                            parent_key: "config".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::RefreshReserve => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "refresh-reserve".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            LendingInstruction::DepositReserveLiquidity { liquidity_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "deposit-reserve-liquidity".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidity_amount".to_string(),
                            value: liquidity_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                    ]
                })
            }
            LendingInstruction::RedeemReserveCollateral { collateral_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "redeem-reserve-collateral".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "collateral_amount".to_string(),
                            value: collateral_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::InitObligation => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "init-obligation".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![]
                })
            }
            LendingInstruction::RefreshObligation => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "refresh-obligation".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "collateral_amount".to_string(),
                            value: collateral_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::DepositObligationCollateral { collateral_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "deposit-obligation-collateral".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "collateral_amount".to_string(),
                            value: collateral_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::WithdrawObligationCollateral { collateral_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "withdraw-obligation-collateral".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "collateral_amount".to_string(),
                            value: collateral_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
            LendingInstruction::BorrowObligationLiquidity { liquidity_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "borrow-obligation-liquidity".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidity_amount".to_string(),
                            value: liquidity_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                    ]
                })
            }
            LendingInstruction::RepayObligationLiquidity { liquidity_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "repay-obligation-liquidity".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidity_amount".to_string(),
                            value: liquidity_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                    ]
                })
            }
            LendingInstruction::LiquidateObligation { liquidity_amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "liquidate-obligation".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "liquidity_amount".to_string(),
                            value: liquidity_amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        },
                    ]
                })
            }
            LendingInstruction::FlashLoan { amount } => {
                Some(InstructionSet {
                    function: InstructionFunction {
                        tx_instruction_id: instruction.tx_instruction_id.clone(),
                        transaction_hash: instruction.transaction_hash.clone(),
                        parent_index: instruction.parent_index.clone(),
                        program: instruction.program.clone(),
                        function_name: "flash-loan".to_string(),
                        timestamp: instruction.timestamp
                    },
                    properties: vec![
                        InstructionProperty {
                            tx_instruction_id: instruction_index.clone(),
                            transaction_hash: transaction_hash.clone(),
                            parent_index: parent_index.clone(),
                            key: "amount".to_string(),
                            value: amount.to_string(),
                            parent_key: "".to_string(),
                            timestamp: timestamp.clone(),
                        }
                    ]
                })
            }
        };
    }

    error!("{}",
        "[processors/programs/native_token_swap] FATAL: Unrecognised instruction.".to_string());
    None
}
