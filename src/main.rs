use config::Config;
use libc::{SIGINT, SIGTERM};
use profile::Profile;
use scheduler::{parse_duration, run};
use signal_hook::iterator::Signals;
use std::{
    fs::{self},
    os::unix::{fs::MetadataExt, process::CommandExt},
    path::PathBuf,
    process::{Command, Stdio},
    sync::Arc,
    thread::sleep,
    time::Duration,
};

mod config;
mod profile;
mod scheduler;

fn main() {
    let conf: Config = Config::parse();

    #[cfg(debug_assertions)]
    {
        let subscriber = tracing_subscriber::fmt().pretty().finish();
        tracing::subscriber::set_global_default(subscriber).ok();
    }

    let home = dirs::home_dir().unwrap().join(".mozilla/firefox");
    let target = home.join(format!("static-{}", &conf.profile.name));
    let profile = home.join(&conf.profile.name);

    if (profile.exists() && !target.exists()) && !profile.is_symlink() {
        tracing::info!(?profile, "setting up profile");
        fs::rename(&profile, &target).unwrap();
    }

    let ram_target = PathBuf::from("/dev/shm").join(format!("ram-{}", &conf.profile.name));

    let ff_profile = Arc::new(Profile::new(target.clone(), ram_target, profile.clone()));

    if conf.profile.autostart {
        if let Some(bin) = conf.profile.bin {
            let command = Command::new(bin);
            run_process(command);
        }
    }

    if let Some(every) = &conf.profile.sync.every {
        let duration = parse_duration(every).expect("It seems like your duration is not valid");

        run(duration, Arc::clone(&ff_profile));
    }

    let mut signals = Signals::new([SIGTERM, SIGINT]).unwrap();

    for sig in signals.forever() {
        match sig {
            SIGTERM | SIGINT => {
                ff_profile.move_ram_to_disk();

                while ff_profile.is_locked() {
                    sleep(Duration::from_secs(1))
                }

                break;
            }
            _ => {}
        }
    }
}

fn get_uid() -> u32 {
    let home = dirs::home_dir().unwrap();
    let metadata = home.metadata().unwrap();

    metadata.uid()
}

fn run_process(mut command: Command) {
    command
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    command.uid(get_uid());

    unsafe {
        command.pre_exec(|| {
            libc::setsid();
            Ok(())
        });

        command.spawn().unwrap();
    }
}
