{
  self ? builtins.getFlake(builtins.toString(./.))
, disko ? self.inputs.disko
, pkgs ? self.inputs.nixpkgs.legacyPackages.x86_64-linux
}:
let
  lib = pkgs.lib;
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
  makeScript = name: function: config:
    pkgs.runCommandLocal "${name}.sh" {} ''
         ${pkgs.shfmt}/bin/shfmt -i 4 \
          < "${disko.lib.${function} config pkgs}" \
          > $out
    '';
  diskoExamples = pkgs.linkFarm "disko-examples"
    (lib.flatten (
      builtins.map (name:
        let
          config = makeConfig name;
          baseName = lib.removeSuffix ".nix" name;
        in [
          {
            name = "create-${baseName}.sh";
            path = makeScript name "createScriptNoDeps" config;
          }
          {
            name = "mount-${baseName}.sh";
            path = makeScript name "mountScriptNoDeps" config;
          }
          {
            name = "${baseName}.json";
            path = pkgs.runCommandLocal "${name}-json" {} ''
          echo '${builtins.toJSON config}' | ${pkgs.jq}/bin/jq . > $out
        '';
          }
        ]) exampleNames));
in
{
  updateSnapshots = pkgs.writeScriptBin "update-snapshots.sh" ''
    for example in ./examples/*.sh; do
      baseName="$(basename "$example" ".sh")"
      scriptPath="./examples/''${baseName}.sh"
      snapPath="tests/snapshots/snapshot_tests__''${baseName}.snap"

      cat > $snapPath <<EOF
    ---
    source: tests/snapshot_tests.rs
    assertion_line: 35
    expression: create_script(&path)?
    ---
    $(cat ''${scriptPath})
    EOF
    done
  '';

  updateExamples = pkgs.writeScriptBin "update-disko-examples.sh" ''
    cp -v ${diskoExamples}/* ./examples; chmod +w examples/*
  '';
}
