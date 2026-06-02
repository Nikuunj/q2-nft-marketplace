mod utils;

use solana_signer::Signer;
use utils::*;

#[test]
fn test_list() {
    let (mut svm, payer) = setup();

    let name = String::from("nameisthis");

    let init_ix = initialize(&payer, &name, 500);
    let collection = create_mpl_collection(&mut svm, &payer);
    let asset = create_mpl_asset(&mut svm, &payer, &collection.pubkey());

    let price = 100_000_000;
    let list_ix = list(&payer, &asset.pubkey(), &collection.pubkey(), price);
    send(&mut svm, &[&payer], &[init_ix, list_ix]);
}
