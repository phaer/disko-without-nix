let
  flake = builtins.getFlake(builtins.toString(./.));
  disko = flake.inputs.disko;
  pkgs = flake.inputs.nixpkgs.legacyPackages.x86_64-linux;
  lib = flake.inputs.nixpkgs.lib;

  examplePath = "${disko}/example/";
  exampleNames = lib.attrNames
    (lib.filterAttrs
      (n: v: v == "regular" && n != "config.nix")
      (builtins.readDir examplePath));

  makeConfig = example:
    import "${examplePath}/${example}" {
      disks = [ "/dev/sdx" "/dev/sdy" "/dev/sdz" ];
      inherit lib;
    };
  makeScript = config:
    disko.lib.createScriptNoDeps config pkgs;
in
pkgs.linkFarm "disko-examples"
  (lib.flatten (
    builtins.map (name:
    let
      config = makeConfig name;
    in [
      {
        name = "${lib.removeSuffix ".nix" name}.sh";
        path = makeScript config;
      }
      {
        name = "${lib.removeSuffix ".nix" name}.json";
        path = pkgs.runCommandLocal "${name}-json" {} ''
          echo '${builtins.toJSON config}' | ${pkgs.jq}/bin/jq . > $out
        '';
      }
    ]) exampleNames))
