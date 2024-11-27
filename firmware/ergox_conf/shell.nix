##you have to download arrterian.nix-env-selector
##and build with nix-shell

{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustup
      cargo
      clang
      llvmPackages.bintools

      flip-link
      elf2uf2-rs
    ];

  RUSTC_VERSION = "latest";
  #RUSTC_VERSION = "nightly";
  
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
  '';

  ##Add precompiled library to rustc search path
  RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
    ##add libraries here (e.g. pkgs.libvmi)

  ]);

  ##Add glibc, clang, glib and other headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS = (
    ##Includes with normal include path
    builtins.map (a: ''-I"${a}/include"'') [
      pkgs.glibc.dev ##add dev libraries here (e.g. pkgs.libvmi.dev)
    ]
  )  ++ [
  ##Includes with special directory paths
  ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
  ''-I"${pkgs.glib.dev}/include/glib"''
  ''-I${pkgs.glib.out}/lib/glib/include/''
  ];

  ##https://github.com/rust-lang/rust-bindgen#environment-variables
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
}
