const LIBS: &[(&str, &str)] = &[
    ("liblz4", "1.9.0"),
    ("libzstd", "1.5.0"),
    ("libarchive", "3.2.0"),
    ("libb2", "0.98.1"),
];

fn main() {
    for (lib, version) in LIBS {
        pkg_config::Config::new()
            .atleast_version(version)
            .probe(lib)
            .expect(&format!("Unable to find {lib}"));
    }
}
