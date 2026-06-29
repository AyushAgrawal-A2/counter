use {
    anchor_lang::{system_program, AccountDeserialize, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Address, Instruction, Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_counter() {
    let program_id = counter::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/counter.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let (counter_pda, counter_bump) = Address::find_program_address(
        &[counter::constants::SEED.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );

    let initialize_ix = Instruction::new_with_bytes(
        program_id,
        &counter::instruction::Initialize { count: 1 }.data(),
        counter::accounts::Initialize {
            signer: payer.pubkey(),
            counter: counter_pda,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[initialize_ix], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer.insecure_clone()])
            .unwrap();
    svm.send_transaction(tx).unwrap();
    let counter_account = svm.get_account(&counter_pda).unwrap();
    let counter =
        counter::state::Counter::try_deserialize(&mut counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.count, 1);
    assert_eq!(counter.bump, counter_bump);

    let increment_ix = Instruction::new_with_bytes(
        program_id,
        &counter::instruction::Increment {}.data(),
        counter::accounts::Increment {
            signer: payer.pubkey(),
            counter: counter_pda,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[increment_ix], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer.insecure_clone()])
            .unwrap();
    svm.send_transaction(tx).unwrap();
    let counter_account = svm.get_account(&counter_pda).unwrap();
    let counter =
        counter::state::Counter::try_deserialize(&mut counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.count, 2);

    let decrement_ix = Instruction::new_with_bytes(
        program_id,
        &counter::instruction::Decrement {}.data(),
        counter::accounts::Decrement {
            signer: payer.pubkey(),
            counter: counter_pda,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[decrement_ix], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer.insecure_clone()])
            .unwrap();
    svm.send_transaction(tx).unwrap();
    let counter_account = svm.get_account(&counter_pda).unwrap();
    let counter =
        counter::state::Counter::try_deserialize(&mut counter_account.data.as_slice()).unwrap();
    assert_eq!(counter.count, 1);

    let prev_payer_lamport = svm.get_account(&payer.pubkey()).unwrap().lamports;
    let close_ix = Instruction::new_with_bytes(
        program_id,
        &counter::instruction::Close {}.data(),
        counter::accounts::Close {
            signer: payer.pubkey(),
            counter: counter_pda,
        }
        .to_account_metas(None),
    );
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[close_ix], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer.insecure_clone()])
            .unwrap();
    svm.send_transaction(tx).unwrap();
    let cur_payer_lamport = svm.get_account(&payer.pubkey()).unwrap().lamports;
    assert!(prev_payer_lamport < cur_payer_lamport);
    let counter_account = svm.get_account(&counter_pda);
    assert!(counter_account.is_none())
}
