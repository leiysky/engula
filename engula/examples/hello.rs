use std::sync::Arc;

use engula::*;

#[tokio::main]
async fn main() -> Result<()> {
    let dirname = "/tmp/engula_test/hello";
    let _ = std::fs::remove_dir_all(dirname);

    // Creates a hybrid storage that reads from a sstable storage and
    // writes to a sstable storage and a parquet storage.
    let fs = Arc::new(LocalFs::new(dirname)?);
    let cache = Arc::new(LruCache::new(4 * 1024 * 1024, 4));
    let sstable_options = SstableOptions::with_cache(cache);
    let sstable_storage = Arc::new(SstableStorage::new(fs.clone(), sstable_options));
    let parquet_options = ParquetOptions::default();
    let parquet_storage = Arc::new(ParquetStorage::new(fs.clone(), parquet_options));
    let storage = Arc::new(HybridStorage::new(
        sstable_storage.clone(),
        vec![sstable_storage, parquet_storage],
    ));

    let journal_options = JournalOptions::default();
    let journal = Arc::new(LocalJournal::new(dirname, journal_options)?);

    let manifest_options = ManifestOptions::default();
    let runtime = Arc::new(LocalCompaction::new(storage.clone()));
    let manifest = Arc::new(LocalManifest::new(
        manifest_options,
        storage.clone(),
        runtime,
    ));

    let options = Options::default();
    let db = Database::new(options, journal, storage, manifest).await?;
    let db = Arc::new(db);

    // Puts to the database concurrently.
    let num_tasks = 4u64;
    let num_entries = 1024u64;
    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let db = db.clone();
        let task = tokio::task::spawn(async move {
            for i in 0..num_entries {
                let v = i.to_be_bytes().to_vec();
                db.put(v.clone(), v.clone()).await.unwrap();
                let got = db.get(&v).await.unwrap();
                assert_eq!(got, Some(v.clone()));
            }
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await?;
    }

    // Verifies the written entries.
    for i in 0..num_entries {
        let v = i.to_be_bytes().to_vec();
        let got = db.get(&v).await.unwrap();
        assert_eq!(got, Some(v.clone()));
    }

    Ok(())
}
