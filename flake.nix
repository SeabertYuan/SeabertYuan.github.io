{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "website";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            wasm-pack
            wasm-bindgen-cli_0_2_114
            lld
          ];

          buildPhase = ''
            runHook preBuild

            export WASM_PACK_CACHE="$TMPDIR/wasm-pack-cache"

            mkdir -p "$WASM_PACK_CACHE"
            wasm-pack build --target web --mode no-install
            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            mkdir -p $out
            cp index.html $out/
            cp -r pkg $out/
            cp -r pages $out/
            cp -r resources $out/
          '';
        };
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            wasm-bindgen-cli_0_2_114
            wasm-pack
            miniserve
            lld
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      }
    );
}
