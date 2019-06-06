include!(concat!(env!("OUT_DIR"), "/thisbuild.rs"));

pub fn build_summary() -> String {
    format!(r##"BROWSER APP BUILD SUMMARY
Built by {} in {} on {} on {}
Git version {}
Local changes list: {}
deploy={} console={}
{}
"##,
            VERSION_USER,VERSION_DIR,VERSION_HOSTNAME,VERSION_DATE,
            VERSION_GIT,VERSION_CHANGES,VERSION_POM,
            cfg!(deploy),cfg!(console)
            )
}
