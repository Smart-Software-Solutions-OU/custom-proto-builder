# Custom automated proto builder

## How to compile proto structs

* `cargo run` in the compiler folder.

## Flow

1. Read general configuration file on `config` folder.
2. Loop reading sub-configuration files defined on `config.yaml` and get the corresponding `proto` files, filter them and apply custom `@derive` for each struct / service.
3. Generate the `modules` files that load the proto generated rs files.

## License

This project is distributed under the terms of the Apache License (Version 2.0).

See [LICENSE](https://github.com/tokio-rs/prost/blob/master/LICENSE) for details.

Copyright 2024 Smart Software Solutions OU
