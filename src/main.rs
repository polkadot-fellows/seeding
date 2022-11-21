#[subxt::subxt(runtime_metadata_path = "./metadata/kusama.scale")]
pub mod kusama {
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use ::subxt::ext::sp_runtime::MultiAddress;
}

use subxt::{ext::sp_runtime::{AccountId32, MultiAddress}, };
use parity_scale_codec::Encode as _;
use std::str::FromStr as _;
use kusama::runtime_types::{
    pallet_utility::pallet::Call as UtilityCall,
    pallet_ranked_collective::pallet::Call as CollectiveCall,
    kusama_runtime::RuntimeCall,
};

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let mut calls = Vec::new();

    // ranks copied from https://github.com/polkadot-fellows/seeding/blob/main/README.md
    // at https://github.com/polkadot-fellows/seeding/commit/c0e3e73035ce892927541d239aa6ad828df9984d
    add_member(&mut calls, "FcxNWVy5RESDsErjwyZmPCW6Z8Y3fbfLzmou34YZTrbcraL", 7)?;  // gavofyork
    add_member(&mut calls, "J8ww78Qx3LVLW54bva3t4SzXcWKMdUWHEZR3V2VNKbmQgE8", 6)?;  // rphmeier
    add_member(&mut calls, "FFFF3gBSSDFSvK2HBq4qgLH75DHqXWPHeCnR1BSksAMacBs", 6)?;  // bkchr
    add_member(&mut calls, "G7YVCdxZb8JLpAm9WMnJdNuojNT84AzU62zmvx5P1FMNtg2", 5)?;  // andresilva
    add_member(&mut calls, "FczL661VujsMFqZT32Y3iBmNpSxUw4gMnq2JtdoFT6rJzr4", 5)?;  // arkpar
    add_member(&mut calls, "Dcm1BqR4N7nHuV43TXdET7pNibt1Nzm42FggPHpxKRven53", 5)?;  // pepyakin
    add_member(&mut calls, "J9nD3s7zssCX7bion1xctAF6xcVexcpy2uwy4jTm9JL8yuK", 5)?;  // jacogr
    add_member(&mut calls, "EGVQCe73TpFyAZx5uKfE1222XfkT3BSKozjgcqzLBnc5eYo", 4)?;  // shawntabrizi
    add_member(&mut calls, "HL8bEp8YicBdrUmJocCAWVLKUaR2dd1y6jnD934pbre3un1", 4)?;  // kianenigma
    add_member(&mut calls, "HyBryanRsB1GGKa9ZfqvRc3XpTDipYyRvxNNyZYfWFcenhd", 4)?;  // xlc
    add_member(&mut calls, "GAWwkmjbLhM5pnAVbdZEkwd3QjHE7kaxJSCi3Ec91Q3QSDW", 3)?;  // ordian
    add_member(&mut calls, "H25aCspunTUqAt4D1gC776vKZ8FX3MvQJ3Jde6qDXPQaFxk", 3)?;  // drahnr
    add_member(&mut calls, "GtLQoW4ZqcjExMPq6qB22bYc6NaX1yMzRuGWpSRiHqnzRb9", 3)?;  // NikVolf
    add_member(&mut calls, "G5MVrgFmBaYei8N6t6DnDrb8JE53wKDkazLv5f46wVpi14y", 3)?;  // athei
    add_member(&mut calls, "HfFpz4QUxfbocHudf8UU7cMgHqkHpf855Me5X846PZAsAYE", 3)?;  // noot
    add_member(&mut calls, "DSbhnaGBytDGRfZTmdcArzCL6T3HQ8gcZxWpF5gLBP6y1Qe", 3)?;  // sorpaas
    add_member(&mut calls, "CzEPpMr7XNS6dK7nQFnQbfnJQYLq7nvULK5kL9U8Zb6CTJm", 3)?;  // cheme
    add_member(&mut calls, "Ddb9puChKMHq4gM6o47E551wAmaNeu6kHngX1jzNNqAw782", 2)?;  // ascjones
    add_member(&mut calls, "GcDZZCVPwkPqoWxx8vfLb4Yfpz9yQ1f4XEyqngSH8ygsL9p", 2)?;  // joepetrowski
    add_member(&mut calls, "GA3yPifemubFga7sTSFtLY2KFFiSRp6Bb8w31FS4xqgAvCz", 2)?;  // KiChjang
    add_member(&mut calls, "HxhDbS3grLurk1dhDgPiuDaRowHY1xHCU8Vu8on3fdg85tx", 2)?;  // shaunxw
    add_member(&mut calls, "HTk3eccL7WBkiyxz1gBcqQRghsJigoDMD7mnQaz1UAbMpQV", 2)?;  // koute
    add_member(&mut calls, "12EXcpt1CwnSAF9d7YWrh91bQw6R5wmCpJUXPWi7vn2CZFpJ", 2)?;  // seunlanlege
    add_member(&mut calls, "D8sM6vKjWaeKy2zCPYWGkLLbWdUtWQrXBTQqr4dSYnVQo21", 2)?;  // edwardmack
    add_member(&mut calls, "GfbnnEgRU94n9ed4RFZ6Z9dBAWs5obykigJSwXKU9hsT2uU", 1)?;  // AurevoirXavier
    add_member(&mut calls, "15akrup6APpRegG1TtWkYVuWHYc37tJ8XPN61vCuHQUi65Mx", 1)?;  // akru
    add_member(&mut calls, "Hj44XnjZui7SQ3A5eBMoJFa4H4nVhiyWnL2i2xw5f1YqzRX", 1)?;  // gilescope
    add_member(&mut calls, "Ea6jhP5gF4r7NqhkEoAXJDgSgYpNQNaTYU6gPsrEGfctaKR", 1)?;  // ggwpez
    add_member(&mut calls, "CbCmCwFkfFkQo7bQtVczYg7sJ3oue6Ez2Z4RMGR8gi8deRk", 1)?;  // EclesioMeloJunior
    add_member(&mut calls, "FJq9JpA9P7EXbmfsN9YiewJaDbQyL6vQyksGtJvzfbn6zf8", 1)?;  // 4meta5
    add_member(&mut calls, "EvoLanodoqDsgHb98Ymbu41uXXKfCPDKxeM6dXHyJ2JoVus", 1)?;  // olanod
    add_member(&mut calls, "EaBqDJJNsZmYdQ4xn1vomPJVNh7fjA6UztZeEjn7ZzdeT7V", 1)?;  // doordashcon
    add_member(&mut calls, "5GxLYcBSsZTdMjbPEQGhJU4LkXFWTzeUnV1sysa467hSkEa8", 1)?;  // qdm12
    add_member(&mut calls, "HZe91A6a1xqbKaw6ofx3GFepJjhVXHrwHEwn6YUDDFphpX9", 1)?;  // ferrell-code
    add_member(&mut calls, "GRy2P3kBEzSHCbmDJfquku1cyUyhZaAqojRcNE4A4U3MnLd", 1)?;  // insipx
    add_member(&mut calls, "HYwiBo7Mcv7uUDg4MUoKm2fxzv4dMLAtmmNfzHV8qcQJpAE", 1)?;  // arrudagates
    add_member(&mut calls, "GAToWXwmQoMmxHKCmFJ615WbhdGRcRfyDZi7pg7PBRpQuNY", 1)?;  // tbaut
    add_member(&mut calls, "DfqY6XQUSETTszBQ1juocTcG9iiDoXhvq1CoVadBSUqTGJS", 1)?;  // Szegoo
    add_member(&mut calls, "5EHD2BAPe3UvVycjG8qUXYGtU2CrfQkh9zPao8xU7x2iPPtH", 1)?;  // kishansagathiya
    add_member(&mut calls, "H5BuqCmucJhUUuvjAzPazeVwVCtUSXVQdc5Dnx2q5zD7rVn", 1)?;  // dharjeezy
    add_member(&mut calls, "GxX7S1pTDdeaGUjpEPPF2we6tgHDhbatFG25pVmVFtGHLH6", 1)?;  // wizdave97
    add_member(&mut calls, "CzuUtvKhZNZBjyAXeYviaRXwrLhVrsupJ9PrWmdq7BJTjGR", 1)?;  // timwu20
    add_member(&mut calls, "FCunn2Rx8JqfT5g6noUKKazph4jLDba5rUee7o3ZmJ362Ju", 1)?;  // zjb0807
    add_member(&mut calls, "HyPMjWRHCpJS7x2SZ2R6M2XG5ZiCiZag4U4r7gBHRsE5mTc", 1)?;  // davxy
    add_member(&mut calls, "H9nUFL5DasuMeAiTC77QyZFCVX39crW6h7knXNrDF4PrSJf", 1)?;  // skunert

    let batch = RuntimeCall::Utility(UtilityCall::batch {
        calls: calls.into_iter().map(RuntimeCall::FellowshipCollective).collect()
    });

    let bytes = batch.encode();

    println!("{}", hex::encode(bytes));

    Ok(())
}

fn add_member(calls: &mut Vec<CollectiveCall>, who: &str, dan: u8) -> Result<(), &'static str> {
    let who = AccountId32::from_str(who)?;

    calls.push(CollectiveCall::add_member { who: MultiAddress::Id(who.clone()) });
    for _ in 1..dan {
        calls.push(CollectiveCall::promote_member { who: MultiAddress::Id(who.clone()) })
    }

    Ok(())
}
