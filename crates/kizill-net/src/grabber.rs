// grabber goes to internet, grabs stuff, checks hash, puts in pantry
// if pantry already have it, grabber stay home. no wasted bandwidth

use crate::hasher;
use crate::pantry::Pantry;
use anyhow::{Context, Result};
use futures::stream::{FuturesUnordered, StreamExt};
use std::path::PathBuf;
use std::sync::Arc;

pub struct Grabber {
    http: reqwest::Client,
    pantry: Arc<Pantry>,
    how_many_at_once: usize,
}

/// one thing we want. url, hash, where it goes if its not cached
pub struct GrabOrder {
    pub url: String,
    pub sha1: String,
    pub dest: PathBuf,
}

impl Grabber {
    pub fn new(pantry: Pantry, how_many_at_once: usize) -> Self {
        let http = reqwest::Client::builder()
            .user_agent("Kizill_CRAFT/0.1 (+github.com/kizill-top-jr/Kizill_CRAFT)")
            .build()
            .expect("http client build failed. cursed");

        Self {
            http,
            pantry: Arc::new(pantry),
            how_many_at_once: how_many_at_once.max(1),
        }
    }

    /// main move. one order. pantry first, net second
    pub async fn grab_one(&self, order: GrabOrder) -> Result<()> {
        // already in pantry? just serve it. net stay cold
        if self.pantry.already_got_good_one(&order.sha1).await {
            self.pantry.serve_to(&order.sha1, &order.dest).await?;
            tracing::debug!("pantry save: {}", order.sha1);
            return Ok(());
        }

        // not in pantry. go fetch
        tracing::info!("net fetch: {}", order.url);
        let bytes = self
            .http
            .get(&order.url)
            .send()
            .await
            .with_context(|| format!("net said no to {}", order.url))?
            .error_for_status()
            .with_context(|| format!("net said error status for {}", order.url))?
            .bytes()
            .await
            .with_context(|| format!("cant read body from {}", order.url))?;

        // check hash before trusting
        hasher::bytes_match_the_vibe(&bytes, &order.sha1)?;

        // stash in pantry, then serve to game
        self.pantry.stash_bytes(&order.sha1, &bytes).await?;
        self.pantry.serve_to(&order.sha1, &order.dest).await?;

        Ok(())
    }

    /// bulk grab. keep pool full, dont wait. async go brrr
    pub async fn grab_many(&self, orders: Vec<GrabOrder>) -> Result<GrabSummary> {
        let total = orders.len();
        let mut pool = FuturesUnordered::new();
        let mut queue = orders.into_iter();
        let mut done = 0;
        let mut failed = Vec::new();

        // fill pool
        for _ in 0..self.how_many_at_once {
            if let Some(order) = queue.next() {
                pool.push(self.clone_grab(order));
            }
        }

        while let Some(result) = pool.next().await {
            match result {
                Ok(()) => done += 1,
                Err(e) => failed.push(e.to_string()),
            }

            if let Some(order) = queue.next() {
                pool.push(self.clone_grab(order));
            }
        }

        Ok(GrabSummary {
            total,
            grabbed: done,
            failed,
        })
    }

    fn clone_grab(
        &self,
        order: GrabOrder,
    ) -> impl std::future::Future<Output = Result<()>> + '_ {
        async move { self.grab_one(order).await }
    }
}

pub struct GrabSummary {
    pub total: usize,
    pub grabbed: usize,
    pub failed: Vec<String>,
}

impl GrabSummary {
    pub fn all_good(&self) -> bool {
        self.failed.is_empty()
    }

    pub fn report(&self) {
        tracing::info!(
            "grab done: {}/{} ok, {} failed",
            self.grabbed,
            self.total,
            self.failed.len()
        );
        for f in &self.failed {
            tracing::error!("grab fail: {}", f);
        }
    }
}
// pomogite pls
