{ pkgs ? import <nixpkgs> {} }:
    pkgs.mkShell {
        packages = with pkgs;[
            rustup
            mdbook
        ];
    }