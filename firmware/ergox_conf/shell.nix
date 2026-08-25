{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      rustup
      clang
      llvmPackages.bintools
      flip-link
      elf2uf2-rs
      probe-rs-tools
    ];

    RUSTC_VERSION = "stable";
    
    shellHook = ''
      # 初始化 rustup
      export RUSTUP_HOME=''${RUSTUP_HOME:-$HOME/.rustup}
      export CARGO_HOME=''${CARGO_HOME:-$HOME/.cargo}
      
      # 只使用 rustup 管理的 cargo/rustc
      unset CARGO
      unset RUSTC
      
      # 安装工具链（如果没装）
      rustup toolchain install $RUSTC_VERSION 2>/dev/null || true
      rustup default $RUSTC_VERSION
      
      # 安装 target
      rustup target add thumbv6m-none-eabi --toolchain $RUSTC_VERSION
      
      # 确保 PATH 优先使用 rustup
      export PATH="$CARGO_HOME/bin:$RUSTUP_HOME/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin:$PATH"
      
      export KEYBOARD_TOML_PATH="$PWD/keyboard.toml"
      
      echo "Using cargo: $(which cargo)"
      echo "Using rustc: $(which rustc)"
      cargo --version
      rustc --version
      rustup target list --installed
    '';
    
    BINDGEN_EXTRA_CLANG_ARGS = (builtins.map (a: ''-I"${a}/include"'') [
      pkgs.glibc.dev
    ]) ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
    ];
    
    LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  }
