This sample program is based on the Zenkit "CRM For Sales" template.
It computes "expected total deal value", 
a probability-weighted sum of all pending orders for each customer.


To run, follow these steps:


Add the "CRM For Sales" demo template to your Zenkit account, 
and run [zk-codegen](https://github.com/stevelr/zenkit-codegen)
to build the client library.

```sh
zk-codegen -o crm_for_sales -w "CRMForSales"
cd crm_for_sales
cp Cargo.toml.sample Cargo.toml
```

Edit Cargo.toml to add this under `[dependencies]`

```toml
tokio = { version="0.2", features=["rt-core","macros"] }
```

Add this to the bottom of Cargo.toml:

```toml
[[bin]]
name = "deal-value"
path = "deals/main.rs"
```

Copy `deal_value.rs` from this folder to `crm_for_sales/deals/main.rs`

Build: `cargo build`
and run the deal analysis with `./target/debug/deal-value`
            

