#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-create-builder.rs");
    t.pass("tests/03-call-setters.rs");
    t.pass("tests/04-call-build.rs");
    t.pass("tests/05-method-chaining.rs");
    t.pass("tests/06-optional-field.rs");
    t.pass("tests/07-repeated-field.rs");
    //t.compile_fail("tests/08-unrecognized-attribute.rs");
    //t.pass("tests/09-redefined-prelude-types.rs");
}

// #[cfg(test)]
// mod test {
//     use derive_builder::Builder;
//
//     #[derive(Builder)]
//     pub struct Command {
//         executable: String,
//         #[builder(each = "arg")]
//         args: Vec<String>,
//         #[builder(each = "env")]
//         env: Vec<String>,
//         current_dir: Option<String>,
//     }
//
//     fn main() {
//         let command = Command::builder()
//             .executable("cargo".to_owned())
//             .arg("build".to_owned())
//             .arg("--release".to_owned())
//             .build()
//             .unwrap();
//         dbg!(&command);
//         assert_eq!(command.executable, "cargo");
//         assert_eq!(command.args, vec!["build", "--release"]);
//     }
// }
//
// #[cfg(test)]
// mod expanded {}
