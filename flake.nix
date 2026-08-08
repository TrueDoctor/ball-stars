{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "clippy" "rust-analyzer" ];
        };

        packages = with pkgs; [
          bacon
          toolchain
          cargo-show-asm
        ];

        devTools = with pkgs; [
          jujutsu
        ];

        runtimeLibs = with pkgs; [
          wayland
          libxkbcommon
          vulkan-loader
        ];

      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages ++ runtimeLibs ++ devTools;

          nativeBuildInputs = with pkgs; [ pkg-config ];

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
        };
      });
}
