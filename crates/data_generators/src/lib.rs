use std::fs;
use std::io::Read;

use fs_extra::dir::CopyOptions;

use feather_consts::{SERVER_DOWNLOAD_URL, VERSION_STRING};

mod generators;
mod utils;

pub fn generate_code() {
    generators::generate_all();
}

pub fn extract_vanilla_data() {
    const SERVER_JAR: &str = "server.jar";

    let build_directory = std::env::current_dir().unwrap();
    let _ = std::fs::create_dir(&build_directory);
    let build = build_directory.canonicalize().unwrap();
    let generated = build.join("generated");
    if std::fs::read_to_string(generated.join(".version")).ok() != Some(VERSION_STRING.to_string())
    {
        let _ = std::fs::remove_dir_all(&generated);
        let server_jar_path = build.join(SERVER_JAR);
        if !server_jar_path.is_file() {
            log::info!("Downloading Minecraft server jar");

            let resp = ureq::get(SERVER_DOWNLOAD_URL).call().unwrap();
            let len = resp.header("Content-Length").unwrap().parse().unwrap();
            let mut bytes = Vec::with_capacity(len);
            resp.into_reader().read_to_end(&mut bytes).unwrap();

            fs::write(server_jar_path, bytes).unwrap();
        }

        log::info!("Running vanilla data generators");
        std::process::Command::new("java")
            .args(
                format!(
                    "-DbundlerMainClass=net.minecraft.data.Main -jar {} --all",
                    SERVER_JAR
                )
                .split_whitespace(),
            )
            .current_dir(&build)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        std::fs::write(generated.join(".version"), VERSION_STRING).unwrap();
        std::fs::remove_file(build.join(SERVER_JAR)).unwrap();
        std::fs::remove_dir_all(build.join("libraries")).unwrap();
        std::fs::remove_dir_all(build.join("logs")).unwrap();
        std::fs::remove_dir_all(build.join("versions")).unwrap();

        log::info!("Copying build/generated/reports/worldgen/ to build/worldgen/");
        let worldgen = build.join("worldgen");
        fs_extra::dir::create(&worldgen, true).unwrap();
        println!("{:?}", worldgen.canonicalize().unwrap());
        fs_extra::dir::copy(
            generated.join("reports").join("worldgen"),
            worldgen.parent().unwrap(),
            &CopyOptions::default(),
        )
        .expect("Cannot copy build/generated/reports/worldgen/ to build/worldgen/");
    }
}
