# EASY SSL

this is a rust lib to create x509 ssl certificate and private key file via an api and can be used in other rust projects.

## Installation

put the crate name and version in your cargo.toml file

```bash
easy_ssl = "0.0.3"
```

## Usage

```rust

use easy_ssl::{Builder,create};

let mut build = Builder::new();

build.set_key_path("D://key.pem".to_string());
build.set_certificate_path("D://cert.pem".to_string());
build.set_key_size(4048);

build.issuer.set_country("IN".to_string());
build.issuer.set_state("UP".to_string());
build.issuer.set_location("GZB".to_string());
build.issuer.set_org("DAACHI".to_string());
build.issuer.set_common_name("https://daachi.in".to_string());

build.subject.set_country("IN".to_string());
build.subject.set_state("UP".to_string());
build.subject.set_location("GZB".to_string());
build.subject.set_org("DAACHI".to_string());
build.subject.set_common_name("127.0.0.1".to_string());   

match create(&mut build) {
   Ok(_)=>{
    common::log("ssl files created successfully");
   },
   Err(_)=>{
    common::error("failed to create ssl files");
   }
}

```

## License
[MIT](https://github.com/gzbakku/fdb/tree/master/cert/README.md)
