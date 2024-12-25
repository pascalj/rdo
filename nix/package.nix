{
  lib,
  mpv,
  rustPlatform,
  ...
}:
rustPlatform.buildRustPackage rec {
  pname = "rdo";
  version = "0.3.1";

  src = lib.cleanSource ./..;

  cargoLock.lockFile = "${src}/Cargo.lock";

  buildInputs = [ mpv ];

  meta = {
    description = "a TUI internet radio client";
    homepage = "https://github.com/pascalj/rdo";
    license = lib.licenses.mit;
    maintainers = [ ];
  };
}
