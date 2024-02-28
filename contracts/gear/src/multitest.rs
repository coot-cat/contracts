use cw_multi_test::{App, ContractWrapper};

use crate::{execute, instantiate, query};

pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);

        CodeId(app.store_code(Box::new(contract)))
    }

    pub fn id(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {}
