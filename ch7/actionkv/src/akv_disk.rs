use libactionkv::{ActionKV, ByteStr, ByteString};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args = std::env::args().collect::<Vec<_>>();
    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_str();
    let key = args.get(3).expect(USAGE).as_bytes();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut a = ActionKV::open(path).expect("unable to open fiel");

    a.load().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = a.get(INDEX_KEY).unwrap().unwrap();
            let index_decoded = bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();
            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&position) => {
                    let kv = a.get_at(position).unwrap();
                    println!("{:?}", kv.value);
                }
            }
        }

        "delete" => a.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            a.insert(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }

        "update" => {
            let value = maybe_value.expect(USAGE).as_bytes();
            a.update(key, value).unwrap();
            store_index_on_disk(&mut a, INDEX_KEY);
        }

        _ => eprintln!("{}", &USAGE),
    }
}
