# kvariants

A Rust crate for https://github.com/hfhchan/irg/blob/master/kVariants.md


## Install

TODO: Publish to crate.io


## Usage

```rs
use kvariants::KVARIANTS;

let c = '澚';

let kvariant = match KVARIANTS.get(&c) {
    Some(kvariant) => kvariant.destination_ideograph,
    None => c,
};

assert_eq!(kvariant, '澳');
```

## Fetch latest dictionary from upstream

The dictionary file is vendored into `dictionaries/source/` and can be updated with `bin/sync_dictionaries`.
