{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        rustc
        cargo

        pkg-config
        openssl
        gnupg
    ];

    shellHook = ''
        export RUST_BACKTRACE=1
    '';
}
