> **Historical reference snapshot**
>
> This source is preserved for study and reuse. It is not maintained or verified
> to build or run with current dependencies. Historical instructions below may
> rely on unavailable services, repositories or infrastructure. CI and GitHub
> Actions configuration have been removed, together with repository synchronization
> scripts and private contact/tracking references where present.
>
> Original Step Finance contributions are licensed under [MIT](LICENSE).
> Third-party portions retain their applicable upstream terms; see
> [third-party notices](THIRD_PARTY_NOTICES.md). The snapshot as a whole is
> not represented as exclusively MIT-licensed.

# Step Ingestooor DevKit

## Overview

The Step Ingestooor DevKit is a toolkit for creating parsers that process blockchain transactions and generate `Dooot` objects. This guide will help you implement a new parser by focusing on the `invoke` function and writing tests for it.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

### Implementing a New Parser

1. **Define Your Parser Struct**

   Create a new file for your parser in the `src/parsers/` directory. For example, `src/parsers/my_parser.rs`:

   ```rust
   use anyhow::Result;
   use step_ingestooor_sdk::{
       dooot::Dooot,
       trait_defs::{parser::TransactionParserFilter, transaction::TransactionLike},
   };

   use crate::defs::TransactionParserTrait;

   pub struct MyParser;

   impl TransactionParserTrait for MyParser {
       fn get_name(&self) -> &'static str {
           "MyParser"
       }

       fn get_filters(&self) -> Vec<TransactionParserFilter> {
           // Define your filters here
           vec![TransactionParserFilter::ProgramId(PROGRAM_ID.into())]
       }

       fn invoke(&self, txn: &impl TransactionLike, dooot: &mut Vec<Dooot>) -> Result<()> {
           // Implement your parsing logic here
           Ok(())
       }
   }
   ```

2. **Write your parser, creating the required dooots**

   Implement the `invoke` function to parse the transaction and generate `Dooot` objects. The `invoke` function takes a reference to a transaction and a mutable reference to a vector of `Dooot` objects. You can see an example of this in action at `src/parsers/sol_balance.rs`

3. **Write tests for your parser**

   Get a transaction message based on the filters you provide to the broadcastooor service. Use `DecodedTransaction::load_from_file` to load the transaction from a file. Then, create a new instance of your parser and call the `invoke` function with the transaction and a mutable reference to a vector of `Dooot` objects. Finally, assert that the `Dooot` objects are as expected.

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_my_parser() {
           let txn = DecodedTransaction::load_from_file("./tests/protocol/txn.json").unwrap();
           let mut dooots = Vec::new();
           let parser = MyParser;
           parser.invoke(&txn, &mut dooots).unwrap();
           assert_eq!(dooots.len(), 1);
       }
   }
   ```