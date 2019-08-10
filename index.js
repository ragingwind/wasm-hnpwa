const rust = import('./pkg/wasm_hnpwa.js');

document.addEventListener('app', e => {
  console.log('event', e);
});

rust.then(m => m.run()).catch(console.error);
