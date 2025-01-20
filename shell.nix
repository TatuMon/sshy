{ pkgs ? import <nixpkgs> { } }:
let
  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
in pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
  ];
  RUSTC_VERSION = overrides.toolchain.channel;
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
  '';
}
