mod config;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use colored::Colorize;

use self::config::compiler_list::CompilerList;
use self::config::executable::protocol::Protocol;
use self::config::Config;

///
/// The compiler downloader.
///
#[derive(Debug)]
pub struct Downloader {
    /// The `reqwest` HTTP client.
    http_client: reqwest::blocking::Client,
    /// The compiler-bin JSON list metadata.
    compiler_list: Option<CompilerList>,
}

impl Downloader {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(http_client: reqwest::blocking::Client) -> Self {
        Self {
            http_client,
            compiler_list: None,
        }
    }

    ///
    /// Downloads the compilers described in the config.
    ///
    pub fn download(mut self, config_path: &Path) -> anyhow::Result<Config> {
        let config_file = std::fs::File::open(config_path).map_err(|error| {
            anyhow::anyhow!(
                "Binaries download config {:?} opening error: {}",
                config_path,
                error
            )
        })?;
        let config_reader = std::io::BufReader::new(config_file);
        let config: Config = serde_json::from_reader(config_reader).map_err(|error| {
            anyhow::anyhow!(
                "Binaries download config {:?} parsing error: {}",
                config_path,
                error
            )
        })?;

        let platform_directory = config.get_remote_platform_directory()?;

        for (version, executable) in config.binaries.iter() {
            if !executable.is_enabled {
                continue;
            }

            let source_path = executable
                .source
                .replace("${PLATFORM}", platform_directory.as_str())
                .replace("${VERSION}", version.as_str())
                + (std::env::consts::EXE_SUFFIX);

            let destination_path = executable
                .destination
                .replace("${VERSION}", version.as_str());
            let destination_path = PathBuf::from_str(
                format!("{}{}", destination_path, std::env::consts::EXE_SUFFIX).as_str(),
            )
            .map_err(|_| {
                anyhow::anyhow!("Executable `{}` destination is invalid", destination_path)
            })?;

            let data = match executable.protocol {
                Protocol::File => {
                    if source_path == destination_path.to_string_lossy() {
                        println!(
                            "    {} executable {:?}. The source and destination are the same.",
                            "Skipping".bright_green().bold(),
                            destination_path,
                        );
                        continue;
                    }

                    println!(
                        "     {} executable `{}` => {:?}",
                        "Copying".bright_green().bold(),
                        source_path,
                        destination_path,
                    );

                    std::fs::copy(source_path.as_str(), executable.destination.as_str()).map_err(
                        |error| {
                            anyhow::anyhow!(
                                "Executable {:?} copying error: {}",
                                source_path.as_str(),
                                error
                            )
                        },
                    )?;
                    continue;
                }
                Protocol::HTTPS => {
                    if destination_path.exists() {
                        println!(
                            "    {} executable {:?}. Already exists.",
                            "Skipping".bright_green().bold(),
                            destination_path,
                        );
                        continue;
                    }

                    let source_url =
                        reqwest::Url::from_str(source_path.as_str()).expect("Always valid");
                    println!(
                        " {} executable `{}` => {:?}",
                        "Downloading".bright_green().bold(),
                        source_url,
                        destination_path,
                    );
                    self.http_client.get(source_url).send()?.bytes()?
                }
                Protocol::CompilerBinList => {
                    if destination_path.exists() {
                        println!(
                            "    {} executable {:?}. Already exists.",
                            "Skipping".bright_green().bold(),
                            destination_path,
                        );
                        continue;
                    }

                    let compiler_list_path = PathBuf::from(source_path.as_str());
                    let compiler_list = self.compiler_list.get_or_insert_with(|| {
                        CompilerList::try_from(compiler_list_path.as_path())
                            .expect("compiler-bin JSON list downloading error")
                    });
                    if compiler_list.releases.is_empty() {
                        return Ok(config);
                    }

                    let source_executable_name =
                        match compiler_list.releases.get(version.to_string().as_str()) {
                            Some(source_executable_name) => source_executable_name,
                            None => anyhow::bail!(
                                "Executable for version v{} not found in the compiler JSON list",
                                version
                            ),
                        };
                    let mut source_path = compiler_list_path;
                    source_path.pop();
                    source_path.push(source_executable_name);

                    let source_url =
                        reqwest::Url::from_str(source_path.to_str().expect("Always valid"))
                            .expect("Always valid");
                    println!(
                        " {} executable `{}` => {:?}",
                        "Downloading".bright_green().bold(),
                        source_url,
                        destination_path,
                    );
                    self.http_client.get(source_url).send()?.bytes()?
                }
            };

            let mut destination_folder = destination_path.clone();
            destination_folder.pop();
            std::fs::create_dir_all(destination_folder)?;

            std::fs::write(&destination_path, data)?;

            #[cfg(target_family = "unix")]
            std::fs::set_permissions(&destination_path, std::fs::Permissions::from_mode(0o755))?;
        }

        Ok(config)
    }
}
