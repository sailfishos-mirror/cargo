use crate::prelude::*;
use cargo_test_support::project;
use cargo_test_support::str;

const BIDI_MANIFEST: &str = "
[package]
name = \"foo\"
version = \"0.0.1\"
edition = \"2015\"
description = \"a \u{202e}description\u{202a} here\"  # this is a \u{202b}tricky\u{202c} comment
homepage = \"a \u{202e}homepage\u{202a} there\"  # this is a \u{202b}tricky\u{202c} comment
repository = \"a \u{202e}repository\u{202a} everywhere\"  # this is a \u{202b}tricky\u{202c} comment
";

#[cargo_test]
fn bidi_comments_warn() {
    let manifest = format!(
        "
{BIDI_MANIFEST}

[lints.cargo]
text_direction_codepoint_in_comment = \"warn\"
text_direction_codepoint_in_literal = \"allow\"
"
    );

    let p = project()
        .file("Cargo.toml", &manifest)
        .file("src/lib.rs", "")
        .build();

    p.cargo("check -Zcargo-lints")
        .masquerade_as_nightly_cargo(&["cargo-lints"])
        .with_stderr_data(str![[r#"
[WARNING] unknown lint: `text_direction_codepoint_in_comment`
  --> Cargo.toml:13:1
   |
13 | text_direction_codepoint_in_comment = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = [NOTE] `cargo::unknown_lints` is set to `warn` by default
[WARNING] unknown lint: `text_direction_codepoint_in_literal`
  --> Cargo.toml:14:1
   |
14 | text_direction_codepoint_in_literal = "allow"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[CHECKING] foo v0.0.1 ([ROOT]/foo)
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [ELAPSED]s

"#]])
        .run();
}

#[cargo_test]
fn bidi_literals_warn() {
    let manifest = format!(
        "
{BIDI_MANIFEST}

[lints.cargo]
text_direction_codepoint_in_comment = \"allow\"
text_direction_codepoint_in_literal = \"warn\"
"
    );

    let p = project()
        .file("Cargo.toml", &manifest)
        .file("src/lib.rs", "")
        .build();

    p.cargo("check -Zcargo-lints")
        .masquerade_as_nightly_cargo(&["cargo-lints"])
        .with_stderr_data(str![[r#"
[WARNING] unknown lint: `text_direction_codepoint_in_comment`
  --> Cargo.toml:13:1
   |
13 | text_direction_codepoint_in_comment = "allow"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = [NOTE] `cargo::unknown_lints` is set to `warn` by default
[WARNING] unknown lint: `text_direction_codepoint_in_literal`
  --> Cargo.toml:14:1
   |
14 | text_direction_codepoint_in_literal = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[CHECKING] foo v0.0.1 ([ROOT]/foo)
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [ELAPSED]s

"#]])
        .run();
}

#[cargo_test]
fn bidi_real_workspace() {
    let workspace_manifest = format!(
        "
[workspace]
members = [\"bar\"]

{BIDI_MANIFEST}

[lints.cargo]
text_direction_codepoint_in_comment = \"warn\"
text_direction_codepoint_in_literal = \"warn\"
"
    );

    let member_manifest = format!(
        "
[package]
name = \"bar\"
version = \"0.0.1\"
edition = \"2015\"
"
    );

    let p = project()
        .file("Cargo.toml", &workspace_manifest)
        .file("src/lib.rs", "")
        .file("bar/Cargo.toml", &member_manifest)
        .file("bar/src/lib.rs", "")
        .build();

    p.cargo("check -Zcargo-lints")
        .masquerade_as_nightly_cargo(&["cargo-lints"])
        .with_stderr_data(str![[r#"
[WARNING] unknown lint: `text_direction_codepoint_in_comment`
  --> Cargo.toml:16:1
   |
16 | text_direction_codepoint_in_comment = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = [NOTE] `cargo::unknown_lints` is set to `warn` by default
[WARNING] unknown lint: `text_direction_codepoint_in_literal`
  --> Cargo.toml:17:1
   |
17 | text_direction_codepoint_in_literal = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[CHECKING] foo v0.0.1 ([ROOT]/foo)
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [ELAPSED]s

"#]])
        .run();
}

#[cargo_test]
fn bidi_virtual_workspace() {
    let workspace_manifest = format!(
        "
[workspace]
members = [\"bar\"]

[workspace.package]
description = \"a \u{202e}description\u{202a} here\"  # this is a \u{202b}tricky\u{202c} comment
homepage = \"a \u{202e}homepage\u{202a} there\"  # this is a \u{202b}tricky\u{202c} comment
repository = \"a \u{202e}repository\u{202a} everywhere\"  # this is a \u{202b}tricky\u{202c} comment

[workspace.lints.cargo]
text_direction_codepoint_in_comment = \"warn\"
text_direction_codepoint_in_literal = \"warn\"
"
    );

    let member_manifest = format!(
        "
[package]
name = \"bar\"
version = \"0.0.1\"
edition = \"2015\"
description.workspace = true
homepage.workspace = true
repository.workspace = true

[lints]
workspace = true
"
    );

    let p = project()
        .file("Cargo.toml", &workspace_manifest)
        .file("src/lib.rs", "")
        .file("bar/Cargo.toml", &member_manifest)
        .file("bar/src/lib.rs", "")
        .build();

    p.cargo("check -Zcargo-lints")
        .masquerade_as_nightly_cargo(&["cargo-lints"])
        .with_stderr_data(str![[r#"
[WARNING] unknown lint: `text_direction_codepoint_in_comment`
  --> Cargo.toml:11:1
   |
11 | text_direction_codepoint_in_comment = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = [NOTE] `cargo::unknown_lints` is set to `warn` by default
[WARNING] unknown lint: `text_direction_codepoint_in_literal`
  --> Cargo.toml:12:1
   |
12 | text_direction_codepoint_in_literal = "warn"
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[CHECKING] bar v0.0.1 ([ROOT]/foo/bar)
[FINISHED] `dev` profile [unoptimized + debuginfo] target(s) in [ELAPSED]s

"#]])
        .run();
}
