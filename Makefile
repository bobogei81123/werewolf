build:
	cd client && yarn run build
	cd server && cargo build --release

doc:
	cargo doc --workspace

dev:
	cd client && yarn run dev &
	cd server && RUST_LOG="actix_web=info" systemfd --no-pid -s http::8000 -- cargo watch -x run
