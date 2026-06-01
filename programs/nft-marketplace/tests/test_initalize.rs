use solana_signer::Signer;

mod utils;

use utils::{initialize, send, setup};

#[test]
fn test_initialize() {
    let (mut svm, payer) = setup();

    let name = String::from("nameisthis");

    let init_ix = initialize(&payer, &name, 500);

    send(&mut svm, &[&payer], &[init_ix]);
}
