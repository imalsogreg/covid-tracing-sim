{ pkgs, rustPlatform }:

rustPlatform.buildRustPackage rec {
  name = "covid-tracing-sim";
  version = "0.1.0";
  buildDepends = [pkgs.openssl pkgs.pkg-config pkgs.dhall-to-json pkgs.dhall];
  cargoSha256 = "1p8pwgpx1awpkcrn45y7fpyqz6vlxc287q9jnhk2l08gv0jxrksx";
  src = pkgs.stdenv.lib.cleanSourceWith {
    src = pkgs.stdenv.lib.cleanSource ./.;
    filter = name: type:
      let baseName = baseNameOf (toString name);
      in  baseName != "target";
  };

  meta = with pkgs.stdenv.lib; {
    description = "A simulation of covid tracker usage";
    homepage = https://github.com/imalsogreg/covid-tracing-sim;
    license = licenses.bsd3;
    maintainers = [ maintainers.imalsogreg ];
    platforms = platforms.all;
  };
}
