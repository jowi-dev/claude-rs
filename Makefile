dev-env:
	nix-shell -p openssl llvm pkg-config gcc sqlite --command fish
