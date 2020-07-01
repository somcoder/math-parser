# math-parser
A math-parser prototype written in Rust that compiles to wasm.

##### Supported features
* Basic arithmetic operations (+, -, *, /, mod)
* Some units of time (day, hour, minute, second, millisecond)
* Some units of Length (km, m, cm, mm)
* Percentages (with some pitfalls! 😁)

##### Usage
* Install Rust from (https://rustup.rs)
* Install wasm-pack with ```cargo install wasm-pack```
* Navigate to ```math-parser/parser-ui``` and run ```wasm-pack build --target web --out-name wasm --out-dir ./static```
* UI of the project should be available at ```parser-ui/static``` directory. You can use  [this chrome extension](https://chrome.google.com/webstore/detail/web-server-for-chrome/ofhbbkphhbklhfoeikjpcbhemlocgigb) or any other server to serve those static files.
