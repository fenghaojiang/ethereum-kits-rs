use std::{default, vec};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use anyhow::{Ok, Result};

/// The list of available MEV builders.
/// https://www.mev.to/builders
/// https://www.rated.network/builders?timeWindow=1d&network=mainnet&page=1

#[derive(EnumIter, Debug, Default, Clone, PartialEq, Eq)]
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

    #[default]
    All, 
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

            BlockBuilderEndpoint::All => "all".to_string(),
        }
        
    }
}

impl BlockBuilderEndpoint {
    pub fn mainnet_endpoint(&self) -> Vec<String> {
        let endpoints = match self {
            BlockBuilderEndpoint::Flashbots => vec![
                "https://relay.flashbots.net/".to_string(),
            ],
            BlockBuilderEndpoint::BeaverBuild => vec![
                "https://rpc.beaverbuild.org/".to_string(),
            ],
            BlockBuilderEndpoint::Rsync => vec![
                "https://rsync-builder.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::Builder0x69 => vec![
                "https://builder0x69.io/".to_string(),
            ],
            BlockBuilderEndpoint::GambitLabs => vec![
                "https://builder.gmbit.co/rpc/".to_string(),
            ],
            BlockBuilderEndpoint::EthBuilder => vec![
                "https://eth-builder.com/".to_string(),
            ],
            BlockBuilderEndpoint::Titan => vec![
                "https://rpc.titanbuilder.xyz/".to_string(), 
            ],
            BlockBuilderEndpoint::BuildAI => vec![
                "https://buildai.net/".to_string(), 
            ],
            BlockBuilderEndpoint::Payload => vec![
                "https://rpc.payload.de/".to_string(), 
            ],
            BlockBuilderEndpoint::Lightspeed => vec![
                "https://rpc.lightspeedbuilder.info/".to_string(),
            ],
            BlockBuilderEndpoint::NFactorial => vec![
                "https://rpc.nfactorial.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::BobaBuilder => vec![
                "https://boba-builder.com/searcher/bundle".to_string(),
            ],
            BlockBuilderEndpoint::F1b => vec![
                "https://rpc.f1b.io/".to_string(),
            ],
            BlockBuilderEndpoint::JetBldr => vec![
                "https://rpc.jetbldr.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::PenguinBuild => vec![
                "https://rpc.penguinbuild.org/".to_string(),
            ],
            BlockBuilderEndpoint::LokiBuild => vec![
                "https://rpc.lokibuilder.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::EdenNetwork => vec![
                "https://api.edennetwork.io/v1/bundle/".to_string(),
            ],
            BlockBuilderEndpoint::TBuilder => vec![
                "https://rpc.tbuilder.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::Eigenphi => vec![
                "https://builder.eigenphi.io/".to_string(),
            ],
            BlockBuilderEndpoint::BlockBleelder => vec![
                "https://blockbeelder.com/rpc/".to_string(),
            ],
            BlockBuilderEndpoint::ManifoldFinance => vec![
                "https://api.securerpc.com/v1/".to_string(),
            ],
            BlockBuilderEndpoint::Pandabuild => vec![
                "https://rpc.pandabuilder.io/".to_string(),
            ],
            BlockBuilderEndpoint::SmithBot => vec![
                "https://smithbot.xyz/".to_string(),
            ],
            BlockBuilderEndpoint::All => {
                let mut endpoints = vec![];
                // let t = BlockBuilderEndpoint::iter().filter(|&builder| builder != BlockBuilderEndpoint::All).
                //     map(|builder| builder.mainnet_endpoint());

                for builder in BlockBuilderEndpoint::iter() {
                    let builder_endpoint = match builder {
                        BlockBuilderEndpoint::All => vec![],
                        _ => self.mainnet_endpoint(),
                    };

                    endpoints.extend(builder_endpoint);
                }

                endpoints
            },
        };

        endpoints
    }

    // pub goerli_testnet_endpoint(&self) -> Result<Vec<String>> {
    //     let endpoints = match self {
    //         BlockBuilderEndpoint::Flashbots => vec![

    //         ],
    //     }

    //     Ok(endpoints)
    // }
    

    // pub sepolia_testnet_endpoint(&self) -> Result<Vec<String>> {

    // }

}

#[test]
fn test_on_mainnet_endpoint() {
    println!("{:?}", BlockBuilderEndpoint::BeaverBuild.mainnet_endpoint());

    println!("{:?}", BlockBuilderEndpoint::All.mainnet_endpoint())
}