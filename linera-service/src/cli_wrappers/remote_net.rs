// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    cli_wrappers::{ClientWrapper, Faucet, FaucetOption, LineraNet, LineraNetConfig, Network},
    config::Export,
};
use anyhow::Result;
use async_trait::async_trait;
use linera_base::data_types::Amount;
use std::sync::Arc;
use tempfile::{tempdir, TempDir};

#[cfg(any(test, feature = "test"))]
pub struct RemoteNetTestingConfig {
    faucet: Faucet,
}

#[cfg(any(test, feature = "test"))]
impl RemoteNetTestingConfig {
    pub fn new(faucet_url: Option<&str>) -> Self {
        Self {
            faucet: Faucet::new(String::from(
                faucet_url.unwrap_or("https://faucet.devnet.linera.net"),
            )),
        }
    }
}

#[cfg(any(test, feature = "test"))]
#[async_trait]
impl LineraNetConfig for RemoteNetTestingConfig {
    type Net = RemoteNet;

    async fn instantiate(self) -> Result<(Self::Net, ClientWrapper)> {
        let seed = 37;
        let mut net = RemoteNet::new(Some(seed), &self.faucet)
            .await
            .expect("Creating RemoteNet should not fail");

        let client = net.make_client().await;
        // The tests assume we've created a genesis config with 10
        // chains with 10 tokens each. We create the first chain here
        client
            .wallet_init(&[], FaucetOption::NewChain(&self.faucet))
            .await
            .unwrap();

        // And the remaining 9 here
        for _ in 0..9 {
            client
                .open_and_assign(&client, Amount::from_tokens(10))
                .await
                .unwrap();
        }

        Ok((net, client))
    }
}

/// Remote net
#[cfg(any(test, feature = "test"))]
#[derive(Clone)]
pub struct RemoteNet {
    network: Network,
    testing_prng_seed: Option<u64>,
    next_client_id: usize,
    tmp_dir: Arc<TempDir>,
}

#[cfg(any(test, feature = "test"))]
#[async_trait]
impl LineraNet for RemoteNet {
    async fn ensure_is_running(&mut self) -> Result<()> {
        // Leaving this just returning for now.
        // We would have to connect to each validator in the remote net then run
        // ensure_connected_cluster_is_running
        Ok(())
    }

    async fn make_client(&mut self) -> ClientWrapper {
        let client = ClientWrapper::new(
            self.tmp_dir.clone(),
            self.network,
            self.testing_prng_seed,
            self.next_client_id,
        );
        if let Some(seed) = self.testing_prng_seed {
            self.testing_prng_seed = Some(seed + 1);
        }
        self.next_client_id += 1;
        client
    }

    async fn terminate(&mut self) -> Result<()> {
        // We're not killing the remote net :)
        Ok(())
    }
}

#[cfg(any(test, feature = "test"))]
impl RemoteNet {
    async fn new(testing_prng_seed: Option<u64>, faucet: &Faucet) -> Result<Self> {
        let tmp_dir = Arc::new(tempdir()?);
        let genesis_config = faucet.genesis_config().await?;
        // Write json config to disk
        genesis_config.write(tmp_dir.path().join("genesis.json").as_path())?;
        Ok(Self {
            network: Network::Grpc,
            testing_prng_seed,
            next_client_id: 0,
            tmp_dir,
        })
    }
}
