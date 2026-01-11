# cancer-queue

Background job queue system for the Cancer framework.

## Features

- Redis-backed job queues
- Job delays and retries
- Multiple named queues
- Job chaining
- Graceful shutdown

## Usage

```rust
use cancer_queue::{Job, Queueable, dispatch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SendEmail {
    to: String,
    subject: String,
}

#[async_trait::async_trait]
impl Job for SendEmail {
    async fn handle(&self) -> Result<(), cancer_queue::Error> {
        println!("Sending email to {}: {}", self.to, self.subject);
        Ok(())
    }
}

// Dispatch immediately
SendEmail {
    to: "user@example.com".into(),
    subject: "Hello".into(),
}
.dispatch()
.await?;

// Dispatch with delay to specific queue
SendEmail {
    to: "user@example.com".into(),
    subject: "Reminder".into(),
}
.delay(std::time::Duration::from_secs(60))
.on_queue("emails")
.dispatch()
.await?;
```

## Running Workers

```rust
use cancer_queue::{Worker, WorkerConfig};

let worker = Worker::new(WorkerConfig {
    queue: "default".into(),
    ..Default::default()
});

worker.run().await?;
```

## License

MIT
