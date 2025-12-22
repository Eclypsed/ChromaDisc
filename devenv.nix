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
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
  env.BINDGEN_EXTRA_CLANG_ARGS = builtins.concatStringsSep " " [
    "-I${pkgs.libcdio.dev}/include"
    "-I${pkgs.glibc.dev}/include"
  ];

  # See full reference at https://devenv.sh/reference/options/
}
