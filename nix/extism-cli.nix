{ buildGoModule, fetchFromGitHub }:
buildGoModule rec {
  pname = "extism-cli";
  version = "0.3.8";
  sourceRoot = "${src.name}/extism";

  src = fetchFromGitHub {
    owner = "extism";
    repo = "cli";
    rev = "v${version}";
    sha256 = "sha256-StMipPMLSQzrhWv0yoKkNiuHMRW7QIhmVZ/M27WDWrM=";
  };

  vendorHash = "sha256-sSKiwYT5EP0FQJbhgv9ZFDwwwvIJ66yMULbj529AZwY=";

  buildPhase = ''
    mkdir -p $out/bin
    go build -o $out/bin/extism
  '';

  installPhase = ''
    chmod +x $out/bin/extism
  '';
}
