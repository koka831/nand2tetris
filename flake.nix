{
  description = "develop environment with VM emulators";
  inputs = { nixpkgs.url = "nixpkgs/nixpkgs-unstable"; };
  outputs = { self, nixpkgs, ... }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    nand2tetris = builtins.fetchurl {
        # see https://www.nand2tetris.org/software
        name = "nand2tetris.zip";
        url = "https://drive.google.com/uc?export=download&id=1xZzcMIUETv3u3sdpM_oTJSTetpVee3KZ";
        sha256 = "1nfv7qani0x3vih93gqhhs5yykzvfq4p4zchb0hpkjscsq51djyj";
    };
  in {
    packages.${system} = {
        # `nix run .#setup`
        # download testsuites and unpack
        setup = pkgs.writeScriptBin "setup" ''
          unzip ${nand2tetris}
          cp -r nand2tetris/tools .
          chmod a+x tools/VMEmulator.sh
          chmod a+x tools/CPUmulator.sh
          cp -r nand2tetris/tools/OS .
        '';

        # e.g. `nix run .#compile jack-compiler/tests/fixtures/{project}`
        # generates compiled vm files in `runtime` directory.
        compile = pkgs.writeScriptBin "compile" ''
          mkdir -p runtime
          cp OS/*.vm runtime
          cargo run -p jack-compiler -- "$1"
          cp "$1"/*.vm runtime
          rm "$1"/*.vm
        '';
    };

    devShell.${system} = pkgs.mkShell {
      name = "nand2tetris";
      buildInputs = with pkgs; [ jdk11 ];

      shellHook = ''
        export JAVA_HOME=${pkgs.jdk11}
        # launch VM emulator in 4K
        export GDK_SCALE=2
        # see https://wiki.archlinux.org/title/Java 5.3 GTK LookAndFeel
        export JAVA_TOOL_OPTIONS='-Dawt.useSystemAAFontSettings=on -Dswing.aatext=true -Dswing.defaultlaf=com.sun.java.swing.plaf.gtk.GTKLookAndFeel'
      '';
    };
  };
}
