{ lib, mpv, rustPlatform, ... }:
rustPlatform.buildRustPackage {
  pname = "rdo";
  version = "0.0.1";

  src = ./..;

  cargoLock = {
    lockFile = ./../Cargo.lock;
  };

  buildInputs = [ mpv ];
  nativeBuildInputs = [ mpv ];

  # postFixup = ''
  #   wrapProgram $out/bin/rdo \
  #     --set PATH ${lib.makeBinPath [mpv]}
  # '';

  meta = {
    description = "Small CLI for radio listening";
    homepage = "https://github.com/pascalj/rdo";
    license = lib.licenses.mit;
    maintainers = [ ];
  };
}
