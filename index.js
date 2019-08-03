/* eslint-disable no-console */

// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg/hello_world';`
// will work here one day as well!
const rust = import('./pkg/todomvc');

// Attaches callbacks for the result of the Promise.
rust
  // If the import is resolved, calls the run() function defined in src/lib.rs.
  .then(m => m.run())
  // If the import is rejected, prints to stderr.
  .catch(console.error);
