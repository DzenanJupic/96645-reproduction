use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    fs::File
};
use rand::Rng;

const LIB_BASE: &str = include_str!("../files/reproduce_it_lib.rs");
const BIN_BASE: &str = include_str!("../files/reproduce_it_bin.rs");
const MOD_A: &str = include_str!("../files/reproduce_it_a.rs");
const MOD_B: &str = include_str!("../files/reproduce_it_b.rs");
const MOD_C: &str = include_str!("../files/reproduce_it_c.rs");
const MOD_D: &str = include_str!("../files/reproduce_it_d.rs");
const CARGO_TOML: &str = include_str!("../reproduction/Cargo.toml");

const NIGHTLY: &str = "nightly-2022-07-12";
const OVERRIDE: &str = "stable-2022-06-30";

trait ExitStatusOk {
    fn exit_status_ok(&self) -> anyhow::Result<()>;
}
impl ExitStatusOk for std::process::ExitStatus {
    fn exit_status_ok(&self) -> anyhow::Result<()> {
        match self.success() {
            true => Ok(()),
            false => Err(anyhow::anyhow!("process exited with status {self:?}")),
        }
    }
}

fn path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn generate_random_file(
    base: &str,
    main: &mut File,
    rng: &mut rand::rngs::ThreadRng,
) -> anyhow::Result<bool> {
    main.write(base.as_bytes())?;

    let mut compile_fail = false;

    for module in [MOD_A, MOD_B, MOD_C, MOD_D] {
        if !compile_fail && rng.gen_bool(0.025) {
            let module = module.as_bytes();
            let range = rng.gen_range(0..module.len());
            main.write(&module[..range])?;
            compile_fail = true;
        } else if rng.gen_bool(0.5) {
            main.write(module.as_bytes())?;
        }

        for _ in 0..rng.gen_range::<usize, _>(0..5) {
            main.write(b"\n// Some comment\n")?;
        }

        if !compile_fail && rng.gen_bool(0.05) {
            main.write(br#"
                async fn incorrect_lifetime<'a>(a: &'a str) -> &'static str { a }

                fn missing_move(a: &str) -> impl std::future::Future<Output = &str> {
                    async { a }
                }

                fn missing_lifetime(a: &str) -> impl std::future::Future<Output = &str> {
                    async move { a }
                }
            "#)?;
            compile_fail = true;
        }
    }

    Ok(compile_fail)
}

fn main() -> anyhow::Result<()> {
    println!("This will download the following toolchains:");
    println!("{NIGHTLY} {OVERRIDE}");
    println!("And download the following crates:");
    println!("\n{CARGO_TOML}");
    println!("Press enter to start fuzzing");
    std::io::stdin().read_line(&mut String::new())?;

    let crate_dir = path("reproduction");

    std::env::set_current_dir(path(""))?;
    Command::new("rustup")
        .args(["toolchain", "install", "--no-self-update", "--profile", "minimal", NIGHTLY])
        .spawn()?
        .wait()?
        .exit_status_ok()?;
    Command::new("rustup")
        .args(["toolchain", "install", "--no-self-update", "--profile", "minimal", OVERRIDE])
        .spawn()?
        .wait()?
        .exit_status_ok()?;
    Command::new("rustup")
        .args(["default", NIGHTLY])
        .spawn()?
        .wait()?
        .exit_status_ok()?;
    Command::new("rustup")
        .args(["override", "set", OVERRIDE])
        .spawn()?
        .wait()?
        .exit_status_ok()?;
    

    loop {
        let mut rng = rand::thread_rng();

        let should_fail_lib = generate_random_file(
            LIB_BASE,
            &mut File::create(crate_dir.join("src/lib.rs"))?,
            &mut rng,
        )?;
        let should_fail_bin = generate_random_file(
            BIN_BASE,
            &mut File::create(crate_dir.join("src/main.rs"))?,
            &mut rng,
        )?;
        let should_fail = should_fail_lib || should_fail_bin;

        let res = if rng.gen_bool(0.7) {
            Command::new("cargo")
                .args(["build", "-p", "reproduction"])
                .spawn()?
                .wait()?
                .exit_status_ok()
        } else {
            Command::new("cargo")
                .args(["check", "-p", "reproduction"])
                .spawn()?
                .wait()?
                .exit_status_ok()
        };

        match res {
            Err(err) if !should_fail => {
                println!("\n\nreproduced it!: {err}");
                std::process::exit(0);
            }
            _ => {}
        }
    }
}
