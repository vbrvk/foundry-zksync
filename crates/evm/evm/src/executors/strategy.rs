use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use alloy_primitives::{Address, U256};
use foundry_cheatcodes::strategy::{CheatcodeInspectorStrategyExt, EvmCheatcodeInspectorStrategy};
use foundry_evm_core::backend::{
    strategy::{BackendStrategy, EvmBackendStrategy},
    BackendResult,
};
use foundry_zksync_compiler::DualCompiledContracts;
use revm::DatabaseRef;

use super::Executor;

pub trait ExecutorStrategy: Debug + Send + Sync {
    fn set_balance(
        &mut self,
        executor: &mut Executor,
        address: Address,
        amount: U256,
    ) -> BackendResult<()>;

    fn set_nonce(
        &mut self,
        executor: &mut Executor,
        address: Address,
        nonce: u64,
    ) -> BackendResult<()>;

    fn new_backend_strategy(&self) -> Arc<Mutex<dyn BackendStrategy>>;
    fn new_cheatcode_inspector_strategy(
        &self,
        dual_compiled_contracts: DualCompiledContracts,
    ) -> Arc<Mutex<dyn CheatcodeInspectorStrategyExt>>;

    // TODO perhaps need to create fresh strategies as well
}

#[derive(Debug, Default, Clone)]
pub struct EvmExecutorStrategy {}

impl ExecutorStrategy for EvmExecutorStrategy {
    fn set_balance(
        &mut self,
        executor: &mut Executor,
        address: Address,
        amount: U256,
    ) -> BackendResult<()> {
        trace!(?address, ?amount, "setting account balance");
        let mut account = executor.backend().basic_ref(address)?.unwrap_or_default();
        account.balance = amount;
        executor.backend_mut().insert_account_info(address, account);

        Ok(())
    }

    fn set_nonce(
        &mut self,
        executor: &mut Executor,
        address: Address,
        nonce: u64,
    ) -> BackendResult<()> {
        let mut account = executor.backend().basic_ref(address)?.unwrap_or_default();
        account.nonce = nonce;
        executor.backend_mut().insert_account_info(address, account);

        Ok(())
    }

    fn new_backend_strategy(&self) -> Arc<Mutex<dyn BackendStrategy>> {
        Arc::new(Mutex::new(EvmBackendStrategy))
    }

    fn new_cheatcode_inspector_strategy(
        &self,
        _dual_compiled_contracts: DualCompiledContracts,
    ) -> Arc<Mutex<dyn CheatcodeInspectorStrategyExt>> {
        Arc::new(Mutex::new(EvmCheatcodeInspectorStrategy))
    }
}
