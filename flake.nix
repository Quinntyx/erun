rec {
    description = "erun. A configurable, component-based launcher, menu, and bar application in Rust.";
    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs";
    };

    outputs = { self, nixpkgs, ... } : 
    let 
        inherit description;
        dev_shell = system : toolchain : 
        let
            pkgs = import nixpkgs { inherit system; };
            inherit description;
        in pkgs.mkShell {
            packages = with pkgs; [ 
                rustup
                cargo
                git
                # (runCommand "dev-shell" {}
                # ''
                #     echo -e "Currently in dev shell for \033[1;37m${description}\033[0m on platform \033[1;32m${system}\033[0m"
                #     echo -e "\033[1;37mCurrent Toolchain: \033[1;32m$(rustup default)\033[0m"
                #     echo -e "\033[1;37mFlake Default Toolchain: \033[1;32m${toolchain}\033[0m"
                # '') 
            ];

            shellHook = ''
            echo "Initializing Rust Environment"
            rustup toolchain install ${toolchain} &> /dev/null
            rustup default nightly-aarch64-apple-darwin &> /dev/null
            rustup update &> /dev/null
            echo -e "\033[1;37mLoaded Toolchain: \033[1;32m$(rustup default)\033[0m"
            echo -e "You are now developing: \033[1;37m${description}\033[0m"
            echo -e "Have fun! Type \033[1;37m'exit'\033[0m to exit." 
            exec zsh
            '';
        };
    in {
        # devShells.aarch64-darwin.default = (make_platform "aarch64-darwin" "nightly-aarch64-apple-darwin").devShells.aarch64-darwin.default;
        devShells.aarch64-darwin.default = dev_shell "aarch64-darwin" "nightly-aarch64-apple-darwin";
        # make_platform "x86_64-linux" "nightly-x86_64-linux-gnu";

    };
}
