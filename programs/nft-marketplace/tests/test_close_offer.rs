mod utils;

use solana_keypair::Keypair;
use solana_signer::Signer;
use utils::*;

#[test]
fn test_close_offer() {
    let (mut svm, payer) = setup();

    let name = String::from("nameisthis");

    let init_ix = initialize(&payer, &name, 500);
    let collection = create_mpl_collection(&mut svm, &payer);
    let asset = create_mpl_asset(&mut svm, &payer, &collection.pubkey());

    let price = 100_000_000;
    let list_ix = list(&payer, &asset.pubkey(), &collection.pubkey(), price);
    send(&mut svm, &[&payer], &[init_ix, list_ix]);


    let taker = Keypair::new();
    svm.airdrop(&taker.pubkey(), 100_000_000_000).unwrap();

    let amount = 90_000_000;
    let maker_offer_ix = make_offer(&taker, &asset.pubkey(), amount);

    send(&mut svm, &[&taker], &[maker_offer_ix]);

    let close_offer_ix = close_offer(&taker, &payer.pubkey(), &asset.pubkey());
    send(&mut svm, &[&taker], &[close_offer_ix]);
    
}
