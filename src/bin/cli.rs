use srenamer::rename_map;
use std::path::Path;

fn main() {
    let res = rename_map(
        &Path::new("C:/Users/teoor/Documents/sr_test").to_path_buf(),
        &Path::new("The.Expanse.S05E01.Exodus.1080p.10bit.WEBRip.6CH.x265.HEVC-PSA.mkv")
            .to_path_buf(),
        &Path::new("The Expanse S05E01 Exodus.mkv").to_path_buf(),
    );
    for (k, v) in &res {
        println!("{:?}: {:?}", k, v);
    }
}
