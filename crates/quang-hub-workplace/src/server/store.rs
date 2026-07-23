//! Storage abstractions for Cloudflare Workers (D1, KV, R2).
//!
//! Provides a unified interface for reading and writing Workplace entities
//! to D1 (structured data), KV (session/cache data), and R2 (file/blobs).
//! All operations use JSON serialization via serde.

use serde::de::DeserializeOwned;
use serde::Serialize;
use worker::*;

/// The storage backend enum — D1, KV, or R2.
#[derive(Clone)]
pub enum StorageBackend {
    /// Cloudflare D1 (SQLite-compatible relational database).
    D1(D1Database),
    /// Cloudflare KV (key-value store, low-latency).
    KV(KvStore),
    /// Cloudflare R2 (S3-compatible object storage).
    R2(R2Bucket),
}

/// A unified store that wraps D1, KV, and R2.
pub struct WorkplaceStore {
    pub d1: Option<D1Database>,
    pub kv: Option<KvStore>,
    pub r2: Option<R2Bucket>,
    pub env: Env,
}

impl WorkplaceStore {
    /// Create a new store from the Worker environment.
    pub fn new(env: &Env) -> Self {
        Self {
            d1: env.d1("WORKPLACE_DB").ok(),
            kv: env.kv("WORKPLACE_KV").ok(),
            r2: env.bucket("WORKPLACE_R2").ok(),
            env: env.clone(),
        }
    }

    // ── D1 Operations ──

    /// Execute a SQL query on D1 and return the first row deserialized.
    pub async fn d1_query_one<T: DeserializeOwned>(
        &self,
        sql: &str,
        params: Vec<D1Value>,
    ) -> Result<Option<T>> {
        let d1 = self
            .d1
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("D1 database not configured".to_string()))?;

        let statement = d1.prepare(sql);
        let query = statement
            .bind(params.to_vec())
            .map_err(|e| worker::Error::RustError(format!("Bind error: {}", e)))?;

        let result = query
            .first::<T>(None)
            .await
            .map_err(|e| worker::Error::RustError(format!("Query error: {}", e)))?;

        Ok(result)
    }

    /// Execute a SQL query on D1 and return all rows deserialized.
    pub async fn d1_query_all<T: DeserializeOwned>(
        &self,
        sql: &str,
        params: Vec<D1Value>,
    ) -> Result<Vec<T>> {
        let d1 = self
            .d1
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("D1 database not configured".to_string()))?;

        let statement = d1.prepare(sql);
        let query = statement
            .bind(params.to_vec())
            .map_err(|e| worker::Error::RustError(format!("Bind error: {}", e)))?;

        let result = query
            .all()
            .await
            .map_err(|e| worker::Error::RustError(format!("Query error: {}", e)))?;

        let rows: Vec<T> = result.results::<T>()?;
        Ok(rows)
    }

    /// Execute a SQL statement (INSERT, UPDATE, DELETE) on D1.
    pub async fn d1_execute(&self, sql: &str, params: Vec<D1Value>) -> Result<u64> {
        let d1 = self
            .d1
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("D1 database not configured".to_string()))?;

        let statement = d1.prepare(sql);
        let query = statement
            .bind(params.to_vec())
            .map_err(|e| worker::Error::RustError(format!("Bind error: {}", e)))?;

        let result = query
            .run()
            .await
            .map_err(|e| worker::Error::RustError(format!("Execute error: {}", e)))?;

        Ok(result.rows_written() as u64)
    }

    /// Insert a JSON-serializable entity into D1.
    pub async fn d1_insert<T: Serialize>(&self, table: &str, id: &str, entity: &T) -> Result<()> {
        let json = serde_json::to_string(entity)
            .map_err(|e| worker::Error::RustError(format!("Serialize error: {}", e)))?;

        let sql = format!(
            "INSERT INTO {} (id, data, created_at, updated_at) VALUES (?, ?, datetime('now'), datetime('now'))",
            table
        );

        self.d1_execute(
            &sql,
            vec![D1Value::from(id.to_string()), D1Value::from(json)],
        )
        .await?;

        Ok(())
    }

    /// Update a JSON-serializable entity in D1.
    pub async fn d1_update<T: Serialize>(&self, table: &str, id: &str, entity: &T) -> Result<()> {
        let json = serde_json::to_string(entity)
            .map_err(|e| worker::Error::RustError(format!("Serialize error: {}", e)))?;

        let sql = format!(
            "UPDATE {} SET data = ?, updated_at = datetime('now') WHERE id = ?",
            table
        );

        self.d1_execute(
            &sql,
            vec![D1Value::from(json), D1Value::from(id.to_string())],
        )
        .await?;

        Ok(())
    }

    /// Delete an entity from D1 by ID.
    pub async fn d1_delete(&self, table: &str, id: &str) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?", table);
        self.d1_execute(&sql, vec![D1Value::from(id.to_string())])
            .await?;
        Ok(())
    }

    // ── KV Operations ──

    /// Store a value in KV with an optional TTL.
    pub async fn kv_put<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: Option<u64>,
    ) -> Result<()> {
        let kv = self
            .kv
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("KV namespace not configured".to_string()))?;

        let json = serde_json::to_string(value)
            .map_err(|e| worker::Error::RustError(format!("Serialize error: {}", e)))?;

        let mut put = kv.put(key, json)?;
        if let Some(ttl) = ttl_seconds {
            put = put.expiration_ttl(ttl);
        }
        put.execute().await?;
        Ok(())
    }

    /// Get a value from KV and deserialize it.
    pub async fn kv_get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let kv = self
            .kv
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("KV namespace not configured".to_string()))?;

        let value = kv.get(key).json::<T>().await?;
        Ok(value)
    }

    /// Delete a key from KV.
    pub async fn kv_delete(&self, key: &str) -> Result<()> {
        let kv = self
            .kv
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("KV namespace not configured".to_string()))?;

        kv.delete(key).await?;
        Ok(())
    }

    // ── R2 Operations ──

    /// Upload an object to R2.
    pub async fn r2_put(&self, key: &str, data: &[u8], content_type: &str) -> Result<()> {
        let r2 = self
            .r2
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("R2 bucket not configured".to_string()))?;

        let mut body = Vec::from(data);
        let metadata = R2PutOptions::default()
            .with_content_type(content_type)
            .map_err(|e| worker::Error::RustError(format!("R2 metadata error: {}", e)))?;

        r2.put(key, body, metadata).await?;
        Ok(())
    }

    /// Get an object from R2.
    pub async fn r2_get(&self, key: &str) -> Result<Option<R2Object>> {
        let r2 = self
            .r2
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("R2 bucket not configured".to_string()))?;

        let object = r2.get(key).execute().await?;
        Ok(object)
    }

    /// Delete an object from R2.
    pub async fn r2_delete(&self, key: &str) -> Result<()> {
        let r2 = self
            .r2
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("R2 bucket not configured".to_string()))?;

        r2.delete(key).await?;
        Ok(())
    }

    /// List objects in R2 with a given prefix.
    pub async fn r2_list(&self, prefix: &str) -> Result<Vec<String>> {
        let r2 = self
            .r2
            .as_ref()
            .ok_or_else(|| worker::Error::RustError("R2 bucket not configured".to_string()))?;

        let result = r2.list().prefix(prefix).execute().await?;
        let keys: Vec<String> = result.objects.iter().map(|o| o.key.clone()).collect();
        Ok(keys)
    }
}

// ── JSON helper ──

/// Parse a request body as JSON, returning a user-friendly error on failure.
pub async fn parse_json_body<T: DeserializeOwned>(req: &mut Request) -> Result<T> {
    let body = req
        .text()
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to read request body: {}", e)))?;

    if body.is_empty() {
        return Err(worker::Error::RustError("Empty request body".to_string()));
    }

    let parsed: T = serde_json::from_str(&body)
        .map_err(|e| worker::Error::RustError(format!("Invalid JSON: {}", e)))?;

    Ok(parsed)
}
