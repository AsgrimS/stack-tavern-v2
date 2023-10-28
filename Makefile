run:
	cargo leptos watch

tailwind:
	pnpm watch

format:
	pnpm format-css && cargo fmt --all

format-check:
	pnpm check-css && cargo fmt --all -- --check
