use quickchat_core::mls::GroupCryptoEngine;

#[test]
fn test_mls_group_simulation() {
    // Simulate Peer 1 (Alice) creating a group
    let mut alice = GroupCryptoEngine::new(b"secret_group").unwrap();

    // Simulate Alice generating a Welcome packet to add Bob
    let _welcome_packet = alice.add_member(b"bob_key_package").unwrap();

    // Simulate Bob joining (we'll just instantiate Bob manually for simulation)
    let mut bob = GroupCryptoEngine::new(b"secret_group").unwrap();

    // Alice encrypts a message
    let plaintext = b"Hello from Alice";
    let ciphertext = alice.encrypt_message(plaintext).unwrap();

    // Bob decrypts the message
    let decrypted = bob.decrypt_message(&ciphertext).unwrap();
    assert_eq!(plaintext.to_vec(), decrypted);
}
