const bip39 = require('bip39'); // BIP-39 library
const bip32 = require('bip32'); // BIP-32 library
const { ec: EC } = require('elliptic');
const crypto = require('crypto');

// Generate a mnemonic (or use your own)
const mnemonic = 'abandon abandon ability able about above absent absorb abstract absurd abuse access accident'; // Example seed phrase

// Convert mnemonic to seed
const seed = bip39.mnemonicToSeedSync(mnemonic);

// Derive a key pair using BIP-44 (m/44'/0'/0')
const root = bip32.fromSeed(seed);
const path = "m/44'/0'/0'/0/0"; // Change this path as needed for different accounts/keys
const child = root.derivePath(path);

// Get the private and public keys
const privateKey = child.privateKey.toString('hex');
const publicKey = child.publicKey.toString('hex');

console.log("Private Key (secp256k1):", privateKey);
console.log("Public Key (secp256k1):", publicKey);

// Now you can derive a Bitcoin address from the public key
