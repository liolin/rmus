use anyhow::Result;
use async_std::task;
use rand::prelude::*;
use sqlx::{
    migrate::{Migrate, Migrator},
    Connection, SqliteConnection,
};
use std::{
    fs::{self, File},
    path::Path,
};

pub fn test_against_database<F>(test: F)
where
    F: FnOnce(&String) -> (),
{
    let test_dir = "/tmp/rmus_tests";
    let database_file = task::block_on(async { create_database(&test_dir).await }).unwrap();
    let database_url = format!("sqlite://{}", database_file);

    // run test
    test(&database_url);

    teardown_database(database_file).unwrap();
}

async fn create_database(test_dir: &str) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; 10];
    rng.fill_bytes(bytes.as_mut_slice());

    fs::create_dir_all(test_dir)?;
    let database_file = format!("{}/rmus_{}.db", test_dir, hex::encode(bytes));
    File::create(&database_file)?;
    let mut conn = SqliteConnection::connect(&format!("sqlite://{}", database_file)).await?;
    let migrator = Migrator::new(Path::new("./migrations")).await?;

    conn.ensure_migrations_table().await?;
    for migration in migrator.iter() {
        if migration.migration_type.is_down_migration() {
            continue;
        }

        conn.apply(migration).await.unwrap();
    }
    conn.close();

    Ok(database_file)
}

fn teardown_database(database_url: String) -> Result<()> {
    fs::remove_file(database_url)?;
    Ok(())
}
