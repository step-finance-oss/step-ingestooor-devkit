use anyhow::{Context, Result};
use step_ingestooor_sdk::{
    dooot::{Dooot, SolBalanceDooot},
    trait_defs::{parser::TransactionParserFilter, transaction::TransactionLike},
};

use crate::defs::TransactionParserTrait;

pub struct SolBalance;

impl TransactionParserTrait for SolBalance {
    fn get_name(&self) -> &'static str {
        "SolBalance"
    }

    fn get_filters(&self) -> Vec<TransactionParserFilter> {
        // `None` filter since we want every single transaction to be processed
        vec![TransactionParserFilter::None]
    }

    fn invoke(&self, txn: &impl TransactionLike, dooot: &mut Vec<Dooot>) -> Result<()> {
        let time = txn.get_block_time();
        let pre_balances = txn.get_pre_balances();
        let post_balances = txn.get_post_balances();
        for (idx, acct) in txn.get_account_keys().iter().enumerate() {
            if !acct.writable {
                // Skip read-only accounts
                continue;
            }

            // Get the pre and post balances for the account.
            // Indexing into balances should be safe since we are iterating over the account keys
            let pre_balance = pre_balances.get(idx).context(format!(
                "UNREACHABLE - Cannot find pre_balance in txn for acct {}",
                acct.pubkey
            ))?;

            let post_balance = post_balances.get(idx).context(format!(
                "UNREACHABLE - Cannot find post_balance in txn for acct {}",
                acct.pubkey
            ))?;

            if pre_balance == post_balance {
                // No update, skip
                continue;
            }

            let balance_dooot = Dooot::SolBalance(SolBalanceDooot {
                account_pubkey: acct.pubkey.clone(),
                balance: (*post_balance).into(),
                time,
            });

            dooot.push(balance_dooot);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use step_ingestooor_sdk::{decoded_transaction::DecodedTransaction, dooot::Dooot};

    use crate::defs::TransactionParserTrait;

    #[tokio::test]
    async fn test_parser_balances() {
        // New dooots vector to store the dooots
        let mut dooots = vec![];
        // Initialize the parser
        let parser = super::SolBalance;
        // Load the test transaction from the file
        let txn = DecodedTransaction::load_from_file("./tests/sol_balance/sol_balance.json")
            .expect("Could not load transaction");

        // Invoke the parser to parse the transaction,
        // and store the dooots in the dooots vector
        parser.invoke(&txn, &mut dooots).unwrap();

        // Check the dooots vector to see if the parser
        // has correctly parsed the transaction
        let first_dooot = &dooots[0];
        let Dooot::SolBalance(dooot) = first_dooot else {
            panic!("Expected SolBalance dooot, got {:?}", first_dooot);
        };
        assert_eq!(
            dooot.account_pubkey,
            "HzuK5PCN6gi8gaKHZwRMhXS4sJiHyUFM3dtBHXLykVQU"
        );
        assert_eq!(dooot.balance, Decimal::from(1504951441u64));

        let second_dooot = &dooots[1];
        let Dooot::SolBalance(dooot) = second_dooot else {
            panic!("Expected SolBalance dooot, got {:?}", second_dooot);
        };
        assert_eq!(
            dooot.account_pubkey,
            "CKttbFuPAN6pEnX3Wz5Fva2U23gWyq5h4ekb8dyncURX"
        );
        assert_eq!(dooot.balance, Decimal::from(130567965520u64));
    }
}
