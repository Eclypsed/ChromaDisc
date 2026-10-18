{
  pkgs,
  ...
}:
{
  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    libcdio
    pkg-config
    glibc
    llvmPackages.libclang
    bacon
    cargo-nextest
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = builtins.concatStringsSep " " [
    "-I${pkgs.libcdio.dev}/include"
    "-I${pkgs.glibc.dev}/include"
  ];

  git-hooks.hooks = {
    rustfmt.enable = true;
  };

  # See full reference at https://devenv.sh/reference/options/
}
