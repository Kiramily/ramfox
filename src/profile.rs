use std::{
    fs::{read_dir, remove_file},
    io,
    os::unix::fs,
    path::{Path, PathBuf},
    sync::{LockResult, Mutex, MutexGuard},
};

pub struct Profile {
    guard: Mutex<()>,
    disk_target: PathBuf,
    ram_target: PathBuf,
}

impl Profile {
    pub fn new(disk_target: PathBuf, ram_target: PathBuf, symlink: PathBuf) -> Self {
        if !ram_target.exists() {
            tracing::info!(?ram_target, "profile is not yet in ram");
            copykitty::copy(&disk_target, &ram_target, None).expect("could not copy profile to ram")
        }

        if !symlink.is_symlink() {
            tracing::info!("symlinking profile");
            fs::symlink(&ram_target, symlink).expect("failed to symlink");
        }

        Self {
            guard: Mutex::new(()),
            disk_target,
            ram_target,
        }
    }

    pub fn move_ram_to_disk(&self) {
        let lock = self.lock().unwrap();

        tracing::info!("saving profile");
        copykitty::copy(&self.ram_target, &self.disk_target, None).unwrap();

        self.cleanup_disk_profile();

        drop(lock)
    }

    fn cleanup_disk_profile(&self) {
        self.clean_up_directory(&self.ram_target, &self.disk_target)
            .unwrap();
    }

    #[allow(clippy::only_used_in_recursion)]
    fn clean_up_directory(&self, ram: &Path, disk: &Path) -> io::Result<()> {
        if !disk.exists() {
            tracing::error!(?disk, "does not exists");

            return Ok(());
        }

        for entry in read_dir(disk)? {
            let entry = entry?;
            let path = entry.path();

            let target = ram.join(path.file_name().unwrap());

            if path.is_dir() {
                self.clean_up_directory(&path, &target)?;
            } else if !target.exists() {
                tracing::debug!(?path, "removing");
                match remove_file(path) {
                    Ok(_) => tracing::info!("removed orphaned file"),
                    Err(err) => tracing::error!(?err),
                }
            }
        }

        Ok(())
    }

    /// Checks if the mutex is currently locked. to prevent pre-mature exit.
    pub fn is_locked(&self) -> bool {
        self.guard.try_lock().is_err()
    }

    #[inline(always)]
    fn lock(&self) -> LockResult<MutexGuard<'_, ()>> {
        self.guard.lock()
    }
}
