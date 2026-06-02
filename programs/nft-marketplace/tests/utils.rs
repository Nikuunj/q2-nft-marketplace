use anchor_lang::{
    solana_program::{instruction::Instruction, pubkey::Pubkey},
    system_program::ID as SYSTEM_PROGRAM_ID,
    InstructionData, Key, ToAccountMetas,
};
use anchor_spl::associated_token;
use litesvm::LiteSVM;
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint, MintTo, TOKEN_ID};
use mpl_core::{
    instructions::{CreateCollectionV1Builder, CreateV1Builder},
    ID as MPL_CORE_ID,
};
use nft_marketplace::Listing;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_signer::Signer;
use solana_transaction::Transaction;

pub fn setup() -> (LiteSVM, Keypair) {
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/nft_marketplace.so");
    let bytes_mpl = include_bytes!("./mpl_core_program.so");
    svm.add_program(Pubkey::from(MPL_CORE_ID.to_bytes()), bytes_mpl);
    svm.add_program(nft_marketplace::ID, bytes);
    let payer_address = payer.pubkey().to_bytes().into();
    svm.airdrop(&payer_address, 100_000_000_000).unwrap();
    (svm, payer)
}

// pub fn mint(svm: &mut LiteSVM, payer: &Keypair) -> Pubkey {
//     CreateMint::new(svm, payer)
//         .decimals(6)
//         .authority(&payer.pubkey())
//         .send()
//         .unwrap()
// }

pub fn get_marketplace_pda(name: &String) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"maketplace", name.as_bytes()], &nft_marketplace::id())
}

pub fn get_treasury_pda(marketplace: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"treasury", marketplace.as_ref()], &nft_marketplace::id())
}

pub fn get_reward_mint_pda(marketplace: &Pubkey) -> Pubkey {
    let (reward_mint_pda, _bump) = Pubkey::find_program_address(
        &[b"reward_mint", marketplace.as_ref()],
        &nft_marketplace::id(),
    );

    reward_mint_pda
}

pub fn get_listing_pda(asset: &Pubkey) -> Pubkey {
    let (reward_mint_pda, _bump) =
        Pubkey::find_program_address(&[b"listing", asset.as_ref()], &nft_marketplace::id());

    reward_mint_pda
}

pub fn get_ata(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    associated_token::get_associated_token_address(owner, mint)
}

pub fn get_offer_pda(maker: &Pubkey, listing: &Pubkey) -> Pubkey {
    let (offer_pda, _) = Pubkey::find_program_address(
        &[b"offer", listing.as_ref(), maker.as_ref()],
        &nft_marketplace::id(),
    );

    offer_pda
}

pub fn get_vault_pda(offer: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"offer_vault", offer.as_ref()], &nft_marketplace::id())
}

// pub fn create_ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
//     CreateAssociatedTokenAccount::new(svm, payer, mint)
//         .owner(owner)
//         .send()
//         .unwrap()
// }

// pub fn mint_to(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, ata: &Pubkey, amount: u64) {
//     MintTo::new(svm, payer, mint, ata, amount).send().unwrap();
// }

pub fn send(svm: &mut LiteSVM, payer: &[&Keypair], inx: &[Instruction]) {
    let msg = Message::new(inx, Some(&payer[0].pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let tx = Transaction::new(payer, msg, recent_blockhash);

    svm.send_transaction(tx).unwrap();
}

pub fn create_mpl_collection(svm: &mut LiteSVM, payer: &Keypair) -> Keypair {
    let collection = Keypair::new();

    let ix = CreateCollectionV1Builder::new()
        .collection(collection.pubkey())
        .payer(payer.pubkey())
        .update_authority(Some(payer.pubkey()))
        .name("collectionname".to_string())
        .uri("collectionuri".to_string())
        .instruction();

    send(svm, &[payer, &collection], &[ix]);

    collection
}

pub fn create_mpl_asset(svm: &mut LiteSVM, owner: &Keypair, collection: &Pubkey) -> Keypair {
    let asset = Keypair::new();
    let ix = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(owner.pubkey())
        .collection(Some(*collection))
        .name("assetname".to_string())
        .uri("asseturi".to_string())
        .instruction();
    send(svm, &[owner, &asset], &[ix]);
    asset
}

pub fn initialize(admin: &Keypair, name: &String, fee: u16) -> Instruction {
    let (marketplace_pda, _) = get_marketplace_pda(name);
    let (treasury_pda, _) = get_treasury_pda(&marketplace_pda);
    let reward_mint_pda = get_reward_mint_pda(&marketplace_pda);

    let admin_pub = Pubkey::from(admin.pubkey().to_bytes());

    let init_ix = Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::Initialize {
            admin: admin_pub,
            maketplace: marketplace_pda,
            treasury: treasury_pda,
            reward_mint: reward_mint_pda,
            token_program: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::Initliaze {
            name: name.clone(),
            fee,
        }
        .data(),
    };

    init_ix
}

pub fn list(maker: &Keypair, asset: &Pubkey, collection: &Pubkey, price: u64) -> Instruction {
    let maker_pub = Pubkey::from(maker.pubkey().to_bytes());

    let listing = get_listing_pda(asset);
    let list_ix = Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::List {
            maker: maker_pub,
            asset: *asset,
            collection: Some(*collection),
            listing,
            mpl_core_program: MPL_CORE_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::List { price }.data(),
    };

    list_ix
}

pub fn buy(
    taker: &Keypair,
    maker: &Pubkey,
    asset: &Pubkey,
    collection: &Pubkey,
    name: &String,
) -> Instruction {
    let (marketplace_pda, _) = get_marketplace_pda(name);

    let listing = get_listing_pda(asset);
    let (treasury_pda, _) = get_treasury_pda(&marketplace_pda);
    let reward_mint_pda = get_reward_mint_pda(&marketplace_pda);

    let taker_ata = get_ata(&taker.pubkey(), &reward_mint_pda);

    let buy_ix = Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::Buy {
            taker: taker.pubkey(),
            maker: *maker,
            asset: *asset,
            collection: Some(*collection),
            maketplace: marketplace_pda,
            listing,
            reward_mint: reward_mint_pda,
            take_reward_ata: taker_ata,
            treasury: treasury_pda,
            mpl_core_program: MPL_CORE_ID,
            associated_token_program: associated_token::ID,
            token_program: TOKEN_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::Buy {}.data(),
    };

    buy_ix
}

pub fn make_offer(maker: &Keypair, asset: &Pubkey, amount: u64) -> Instruction {
    let listing = get_listing_pda(asset);
    let offer = get_offer_pda(&maker.pubkey(), &listing);
    let (vault, _) = get_vault_pda(&offer);

    Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::MakeOffer {
            maker: maker.pubkey(),
            listing,
            offer,
            offer_vault: vault,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::MakerOffer { price: amount }.data(),
    }
}

pub fn reject_offer(
    maker: &Keypair,
    offer_maker: &Pubkey,
    asset: &Pubkey,
) -> Instruction {
    let listing = get_listing_pda(asset);

    let offer = get_offer_pda(offer_maker, &listing);

    let (offer_vault, _) = get_vault_pda(&offer);

    Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::RejectOffer {
            maker: maker.pubkey(),
            offer_maker: *offer_maker,
            offer,
            offer_vault,
            listing,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::RejectOffer {}.data(),
    }
}


pub fn close_offer(
    offer_maker: &Keypair,
    maker: &Pubkey,
    asset: &Pubkey,
) -> Instruction {
    let listing = get_listing_pda(asset);

    let offer = get_offer_pda(&offer_maker.pubkey(), &listing);

    let (offer_vault, _) = get_vault_pda(&offer);

    Instruction {
        program_id: nft_marketplace::id(),
        accounts: nft_marketplace::accounts::CloseOffer {
            maker: *maker,
            offer_maker: offer_maker.pubkey(),
            offer,
            offer_vault,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: nft_marketplace::instruction::CloseOffer {}.data(),
    }
}