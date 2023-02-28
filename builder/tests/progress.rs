#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // t.pass("tests/01-parse.rs");
    // t.pass("tests/02-create-builder.rs");
    // t.pass("tests/03-call-setters.rs");
    // t.pass("tests/04-call-build.rs");
    // t.pass("tests/05-method-chaining.rs");
    // t.pass("tests/06-optional-field.rs");
    t.pass("tests/07-repeated-field.rs");
    //t.compile_fail("tests/08-unrecognized-attribute.rs");
    //t.pass("tests/09-redefined-prelude-types.rs");
}

#[cfg(test)]
mod test {
    use derive_builder::Builder;

    #[derive(Builder)]
    #[allow(unused, dead_code)]
    pub struct Command {
        executable: String,
        #[builder(each = "arg")]
        args: Vec<String>,
        env: Vec<String>,
        current_dir: Option<String>,
    }

    #[allow(unused, dead_code)]
    fn main() {
        let command = Command::builder()
            .executable("cargo".to_owned())
            .args(vec!["build".to_owned(), "--release".to_owned()])
            .env(vec![])
            .build()
            .unwrap();
        assert!(command.current_dir.is_none());

        let command = Command::builder()
            .executable("cargo".to_owned())
            .args(vec!["build".to_owned(), "--release".to_owned()])
            .env(vec![])
            .current_dir("..".to_owned())
            .build()
            .unwrap();
        assert!(command.current_dir.is_some());
    }
}

#[cfg(test)]
mod expanded {}
