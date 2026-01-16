APP_NAME="Cancer Application"
APP_ENV=local
APP_DEBUG=true
APP_URL=http://localhost:8080

SERVER_HOST=127.0.0.1
SERVER_PORT=8080

VITE_PORT=5173

# Build cleanup: auto-remove artifacts older than N days on `ferro serve`
# Set to 0 to disable automatic cleanup (requires cargo-sweep)
CARGO_SWEEP_DAYS=7

# Database (SQLite by default, change to postgres://user:pass@localhost:5432/dbname for PostgreSQL)
DATABASE_URL=sqlite://./database.db
DB_MAX_CONNECTIONS=10
DB_MIN_CONNECTIONS=1
DB_CONNECT_TIMEOUT=30
DB_LOGGING=false

# Mail
MAIL_DRIVER=smtp
MAIL_HOST=localhost
MAIL_PORT=587
MAIL_USERNAME=
MAIL_PASSWORD=
MAIL_FROM_ADDRESS=hello@example.com
MAIL_FROM_NAME="Cancer App"
