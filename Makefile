dev-env:
	nix-shell -p rustup openssl llvm pkg-config gcc sqlite --command fish
