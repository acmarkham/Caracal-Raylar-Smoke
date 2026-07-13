use super::{StorageDriver, StorageError, StorageResult};
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::BlockDevice;
use heapless::String;

impl<D, const SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageDriver<D, SIZE, CACHE, PATH_LEN>
where
    D: BlockDevice<SIZE>,
{
    pub async fn create_directory(&mut self, path: &str) -> StorageResult<(), D::Error> {
        let path = normalize_path::<PATH_LEN, D::Error>(path)?;
        self.fs
            .create_directory(path.as_str())
            .await
            .map_err(StorageError::from)
    }

    pub async fn create_file(&mut self, path: &str) -> StorageResult<(), D::Error> {
        let path = normalize_path::<PATH_LEN, D::Error>(path)?;
        let options = OpenOptions::new().create_new(true).write(true);
        let mut file = self
            .fs
            .open(path.as_str(), options)
            .await
            .map_err(StorageError::from)?;
        file.flush(&mut self.fs).await.map_err(StorageError::from)
    }
}

pub(crate) fn validate_path<E>(path: &str) -> StorageResult<(), E> {
    if path.trim().is_empty() || path.as_bytes().contains(&0) {
        return Err(StorageError::InvalidPath);
    }

    Ok(())
}

pub(crate) fn normalize_path<const PATH_LEN: usize, E>(
    path: &str,
) -> StorageResult<String<PATH_LEN>, E> {
    validate_path(path)?;

    let mut out = String::new();
    if !path.starts_with(['/', '\\']) {
        out.push('/').map_err(|_| StorageError::InvalidPath)?;
    }
    out.push_str(path).map_err(|_| StorageError::InvalidPath)?;
    Ok(out)
}
