use crate::cmd_synthe::do_synthe;

// update all synthesized tags
pub fn do_update(ment_dir: &str) {
    let synthe_path = ment_dir.clone().to_owned() + "/synthe";
    dbg!(&synthe_path);
    let flist = std::fs::read_dir(&synthe_path).unwrap();
    for p in flist {
        let _path = p.unwrap().path();
        let synthed_tag = _path.file_stem().unwrap().to_str().unwrap();
        println!("{}", synthed_tag);
        if synthed_tag == "week" {
            continue;
        }
        do_synthe(&synthed_tag, &ment_dir).expect("synthe command failed");
    }
}
