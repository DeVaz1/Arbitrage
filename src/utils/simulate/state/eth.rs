use super::base::DiffAnalysis;
use ethers::prelude::*;

// Analyze whether the native token is profitable.
pub fn run(tx: &Transaction, trace: &BlockTrace) -> Option<U256> {
    let mut profit = U256::zero();

    if let Some(state_diff) = &trace.state_diff {
        if let Some(account_diff) = state_diff.0.get(&tx.from) {
            let from_account_diff = DiffAnalysis::init(account_diff, Some(tx.nonce));
            if from_account_diff.increase_balance && !from_account_diff.invalid_nonce {
                profit += from_account_diff.balance_diff;
            };

            if let Some(to) = tx.to {
                if let Some(account_diff) = state_diff.0.get(&to) {
                    let to_account_diff = DiffAnalysis::init(account_diff, None);
                    if to_account_diff.increase_balance
                        && !to_account_diff.invalid_nonce
                        && to_account_diff.balance_diff > from_account_diff.balance_diff
                    {
                        profit += to_account_diff.balance_diff;
                    };
                }
            }
        }
    }

    if profit.is_zero() {
        None
    } else {
        Some(profit)
    }
}
