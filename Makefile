run:
	cargo run

run-dev:
	cargo watch -x 'run'

build-release:
	cargo build --release

database-create:
	sqlx database create

database-drop:
	sqlx database drop

migration-run:
	sqlx migrate run

migration-revert:
	sqlx migrate revert