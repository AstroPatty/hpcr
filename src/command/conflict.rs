use crate::error::HpcrError;
use crate::runtime::{BindMount, EnvVar};

pub fn check_bind_conflicts(
    facility_name: &str,
    facility_binds: &[BindMount],
    user_binds: &[BindMount],
) -> Result<(), HpcrError> {
    for user in user_binds {
        for facility in facility_binds {
            if user.dst == facility.dst {
                return Err(HpcrError::BindConflict {
                    dst: user.dst.display().to_string(),
                    facility: facility_name.to_owned(),
                });
            }
        }
    }
    Ok(())
}

pub fn check_env_conflicts(
    facility_name: &str,
    facility_envs: &[EnvVar],
    user_envs: &[EnvVar],
) -> Result<(), HpcrError> {
    for user in user_envs {
        for facility in facility_envs {
            if user.key == facility.key {
                return Err(HpcrError::EnvConflict {
                    key: user.key.clone(),
                    facility: facility_name.to_owned(),
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn bind(src: &str, dst: &str) -> BindMount {
        BindMount {
            src: PathBuf::from(src),
            dst: PathBuf::from(dst),
        }
    }

    fn env(key: &str, value: &str) -> EnvVar {
        EnvVar {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    #[test]
    fn bind_conflict_on_dst() {
        let facility = [bind("/host/mpi", "/opt/mpi")];
        let user = [bind("/my/mpi", "/opt/mpi")];
        assert!(check_bind_conflicts("test", &facility, &user).is_err());
    }

    #[test]
    fn bind_no_conflict_different_dst() {
        let facility = [bind("/opt/mpi", "/opt/mpi")];
        let user = [bind("/scratch", "/data")];
        assert!(check_bind_conflicts("test", &facility, &user).is_ok());
    }

    #[test]
    fn bind_empty_user() {
        let facility = [bind("/opt/mpi", "/opt/mpi")];
        assert!(check_bind_conflicts("test", &facility, &[]).is_ok());
    }

    #[test]
    fn env_conflict_on_key() {
        let facility = [env("MPICH_GPU_SUPPORT_ENABLED", "1")];
        let user = [env("MPICH_GPU_SUPPORT_ENABLED", "0")];
        assert!(check_env_conflicts("test", &facility, &user).is_err());
    }

    #[test]
    fn env_no_conflict_different_key() {
        let facility = [env("MPICH_GPU_SUPPORT_ENABLED", "1")];
        let user = [env("MY_VAR", "hello")];
        assert!(check_env_conflicts("test", &facility, &user).is_ok());
    }

    #[test]
    fn env_key_case_sensitive() {
        let facility = [env("MPICH_GPU", "1")];
        let user = [env("mpich_gpu", "1")];
        assert!(check_env_conflicts("test", &facility, &user).is_ok());
    }
}
