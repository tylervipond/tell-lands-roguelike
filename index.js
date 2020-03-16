const rust = import('./pkg');

rust
  .then(m => m.start())
  .catch(console.error);