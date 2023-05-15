import init, { decrypt, encrypt, generate_key } from './assets/wasm.js';

async function run() {
  await init();
}

run();

window.generate = function generate() {
  document.getElementById('key').value = generate_key();
}

window.copy = function copy() {
  const key = document.getElementById('key');
  key.select();
  key.setSelectionRange(0, 99999); // For mobile devices
  navigator.clipboard.writeText(key.value);
}

window.encrypttext = function encrypttext() {
  const input = document.getElementById('encryptInput').value;
  const key = document.getElementById('encryptkey').value;
  const element = document.getElementById('encryptOutput');
  try {
    element.style.color = "black";
    element.value = encrypt(input, key);
  } catch (error) {
    element.style.color = "red";
    element.value = "Problem with encrypting, are you using a valid key?";
  }
}

window.decrypttext = function decrypttext() {
  const input = document.getElementById('decryptInput').value;
  const key = document.getElementById('decryptkey').value;
  const element = document.getElementById('decryptOutput');
  try {
    element.style.color = "black";
    element.value = decrypt(input, key);
  } catch (error) {
    element.style.color = "red";
    element.value = "Problem with decrypting, are you using the same key you encrypted the data with?";
  }
}