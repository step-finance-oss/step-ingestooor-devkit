use anyhow::Result;
use step_ingestooor_sdk::{
    dooot::Dooot,
    trait_defs::{parser::TransactionParserFilter, transaction::TransactionLike},
};

pub trait TransactionParserTrait {
    fn get_name(&self) -> &'static str;
    fn get_filters(&self) -> Vec<TransactionParserFilter>;
    fn invoke(&self, txn: &impl TransactionLike, dooot: &mut Vec<Dooot>) -> Result<()>;
}
