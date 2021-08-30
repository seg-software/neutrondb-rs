# NeutronDB Rust
Rust implementation of NeutronDB, a Log-Structured Merge-tree Key-Value Store serialized as Stellar Notation Objects.

## Usage
```

neutrondb = "0.9.2"
stellar-notation = "0.9.3"

```

```

use neutrondb::store;

use stellar_notation::{
    StellarObject, StellarValue
};

```

## Functions

### store
```

let mut my_store = store("my").unwrap();

```

### put
```

let key: String = String::from("key_1");

let value: StellarValue = StellarValue::String(String::from("value_1"));

let object: StellarObject = StellarObject(key, value);

my_store.put(object).unwrap();

```

### get
```

let object: StellarObject = my_store.get("key_1").unwrap();

let value: StellarValue = object.1;

match value {
    StellarValue::String(val) => println!(" * value: {}", val);
    _ => ();
}

```

### delete
```

my_store.delete("key_1").unwrap();

```

## Intermediate Topics
- Flush & Compact in accordance with level capacity

## Future Topics
- Read/Write Performance
- Compression
- Error Correction

## Contribution
Any interested party can contact me through email at itsmereystar@protonmail.com
