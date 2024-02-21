use anyhow::{anyhow, Ok, Result};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// The list of available MEV builders.
/// https://www.mev.to/builders
/// https://www.rated.network/builders?timeWindow=1d&network=mainnet&page=1

#[derive(EnumIter, Debug, Clone, PartialEq, Eq)]
pub enum BlockBuilderEndpoint {
    Flashbots,
    BeaverBuild,
    Rsync,
    Builder0x69,
    GambitLabs,
    EthBuilder,
    Titan,
    BuildAI,
    Payload,
    Lightspeed,
    NFactorial,
    BobaBuilder,
    F1b,
    JetBldr,
    PenguinBuild,
    LokiBuild,
    EdenNetwork,
    TBuilder,
    Eigenphi,
    BlockBleelder,
    ManifoldFinance,
    Pandabuild,
    SmithBot,
    // #[default]
    // All,
}

impl ToString for BlockBuilderEndpoint {
    fn to_string(&self) -> String {
        match self {
            BlockBuilderEndpoint::Flashbots => "flashbots".to_string(),
            BlockBuilderEndpoint::BeaverBuild => "beaverbuild".to_string(),
            BlockBuilderEndpoint::Rsync => "rsync".to_string(),
            BlockBuilderEndpoint::Builder0x69 => "0x69".to_string(),
            BlockBuilderEndpoint::GambitLabs => "gambitlabs".to_string(),
            BlockBuilderEndpoint::EthBuilder => "ethbuilder".to_string(),
            BlockBuilderEndpoint::Titan => "titan".to_string(),
            BlockBuilderEndpoint::BuildAI => "buildai".to_string(),
            BlockBuilderEndpoint::Payload => "payload".to_string(),
            BlockBuilderEndpoint::Lightspeed => "lightspeed".to_string(),
            BlockBuilderEndpoint::NFactorial => "nfactorial".to_string(),
            BlockBuilderEndpoint::BobaBuilder => "bobabuilder".to_string(),
            BlockBuilderEndpoint::F1b => "f1b".to_string(),
            BlockBuilderEndpoint::JetBldr => "jetbldr".to_string(),
            BlockBuilderEndpoint::PenguinBuild => "penguinbuild".to_string(),
            BlockBuilderEndpoint::LokiBuild => "loki".to_string(),
            BlockBuilderEndpoint::EdenNetwork => "edennetwork".to_string(),
            BlockBuilderEndpoint::TBuilder => "tbuilder".to_string(),
            BlockBuilderEndpoint::Eigenphi => "eigenphi".to_string(),
            BlockBuilderEndpoint::BlockBleelder => "blockbleelder".to_string(),
            BlockBuilderEndpoint::ManifoldFinance => "manifoldfinance".to_string(),
            BlockBuilderEndpoint::Pandabuild => "pandabuild".to_string(),
            BlockBuilderEndpoint::SmithBot => "smithbot".to_string(),
        }
    }
}

impl BlockBuilderEndpoint {
    pub fn mainnet_endpoint(&self) -> Result<String> {
        let endpoints = match self {
            BlockBuilderEndpoint::Flashbots => "https://relay.flashbots.net/".to_string(),
            BlockBuilderEndpoint::BeaverBuild => "https://rpc.beaverbuild.org/".to_string(),
            BlockBuilderEndpoint::Rsync => "https://rsync-builder.xyz/".to_string(),
            BlockBuilderEndpoint::Builder0x69 => "https://builder0x69.io/".to_string(),
            BlockBuilderEndpoint::GambitLabs => "https://builder.gmbit.co/rpc/".to_string(),
            BlockBuilderEndpoint::EthBuilder => "https://eth-builder.com/".to_string(),
            BlockBuilderEndpoint::Titan => "https://rpc.titanbuilder.xyz/".to_string(),
            BlockBuilderEndpoint::BuildAI => "https://buildai.net/".to_string(),
            BlockBuilderEndpoint::Payload => "https://rpc.payload.de/".to_string(),
            BlockBuilderEndpoint::Lightspeed => "https://rpc.lightspeedbuilder.info/".to_string(),
            BlockBuilderEndpoint::NFactorial => "https://rpc.nfactorial.xyz/".to_string(),
            BlockBuilderEndpoint::BobaBuilder => {
                "https://boba-builder.com/searcher/bundle".to_string()
            }
            BlockBuilderEndpoint::F1b => "https://rpc.f1b.io/".to_string(),
            BlockBuilderEndpoint::JetBldr => "https://rpc.jetbldr.xyz/".to_string(),
            BlockBuilderEndpoint::PenguinBuild => "https://rpc.penguinbuild.org/".to_string(),
            BlockBuilderEndpoint::LokiBuild => "https://rpc.lokibuilder.xyz/".to_string(),
            BlockBuilderEndpoint::EdenNetwork => {
                "https://api.edennetwork.io/v1/bundle/".to_string()
            }
            BlockBuilderEndpoint::TBuilder => "https://rpc.tbuilder.xyz/".to_string(),
            BlockBuilderEndpoint::Eigenphi => "https://builder.eigenphi.io/".to_string(),
            BlockBuilderEndpoint::BlockBleelder => "https://blockbeelder.com/rpc/".to_string(),
            BlockBuilderEndpoint::ManifoldFinance => "https://api.securerpc.com/v1/".to_string(),
            BlockBuilderEndpoint::Pandabuild => "https://rpc.pandabuilder.io/".to_string(),
            BlockBuilderEndpoint::SmithBot => "https://smithbot.xyz/".to_string(),
        };

        Ok(endpoints)
    }

    pub fn goerli_testnet_endpoint(&self) -> Result<String> {
        let endpoint = match self {
            BlockBuilderEndpoint::Flashbots => "https://relay-goerli.flashbots.net/".to_string(),
            BlockBuilderEndpoint::BuildAI => "https://buildai.net/goerli/".to_string(),
            BlockBuilderEndpoint::EdenNetwork => {
                "https://goerli.edennetwork.io/v1/bundle/".to_string()
            }
            _ => "not supported".to_string(),
        };

        if endpoint.eq("not supported") {
            return Err(anyhow!("{} not support for goerli", self.to_string()));
        }

        Ok(endpoint)
    }

    pub fn sepolia_testnet_endpoint(&self) -> Result<String> {
        let endpoint = match self {
            BlockBuilderEndpoint::Flashbots => "https://relay-sepolia.flashbots.net".to_string(),
            _ => "not supported".to_string(),
        };

        if endpoint.eq("not supported") {
            return Err(anyhow!("{} not support for sepolia", self.to_string()));
        }

        Ok(endpoint)
    }
}

pub enum Network {
    Mainnet,
    Goerli,
    Sepolia,
}

pub fn all_block_builder_endpoints(network: Network) -> Vec<String> {
    // let item = BlockBuilderEndpoint::iter().map(|builder| builder.mainnet_endpoint().).collect();
    let mut endpoints = vec![];

    for builder in BlockBuilderEndpoint::iter() {
        let builder_endpoint = match network {
            Network::Mainnet => builder
                .mainnet_endpoint()
                .map_or("".to_string(), |endpoint| endpoint),
            Network::Goerli => builder
                .goerli_testnet_endpoint()
                .map_or("".to_string(), |endpoint| endpoint),
            Network::Sepolia => builder
                .sepolia_testnet_endpoint()
                .map_or("".to_string(), |endpoint| endpoint),
        };

        if builder_endpoint.is_empty() {
            continue;
        }

        endpoints.push(builder_endpoint);
    }

    endpoints
}

#[test]
fn test_on_mainnet_endpoints() {
    println!("{:?}", BlockBuilderEndpoint::BeaverBuild.mainnet_endpoint());
    println!("{:?}", all_block_builder_endpoints(Network::Mainnet));
}

#[test]
fn test_on_goerli_testnet_endpoints() {
    println!(
        "{:?}",
        BlockBuilderEndpoint::Flashbots.goerli_testnet_endpoint()
    );
    println!("{:?}", all_block_builder_endpoints(Network::Goerli));
}

#[test]
fn test_on_sepolia_testnet_endpoints() {
    println!(
        "{:?}",
        BlockBuilderEndpoint::Flashbots.sepolia_testnet_endpoint()
    );
    println!("{:?}", all_block_builder_endpoints(Network::Sepolia));
}
