# erun
A configurable, component-based launcher, menu, and bar application in Rust.

# Build Instructions
Dependencies: `cargo`, `rustc`, and `opengl`. 

1. clone the repo (`git clone https://github.com/quinntyx/erun`)
2. enter the repo (`cd erun`)
3. build the repo (`cargo build --release`)
4. copy `target/release/erun` to `/usr/bin` (or your other place of choice)

All in one: 
```sh
git clone https://github.com/quinntyx/erun
cd erun
cargo build --release
cp target/release/erun /usr/bin
```

# Why
I really like eww. However, as I used it, I found it somewhat lacking, for a few reasons:
1. EWW uses GTK. Gradience has recently been discontinued for GTK, and I'm not too
interested in investing further time in GTK theming and design at the moment in any capacity
2. EWW is *extremely* difficult to use, with an insane learning curve. This is due to a 
combination of factors: 
- Custom language (yuck) used for configuration
- Configuration written in a lisp-based language
- Configuration purely functional
Fundamentally, I found EWW's configuration too esoteric for widespread adoption or any
form of real use. 
3. EWW doesn't have any good alternatives that can match its feature set or flexibility. 
4. I haven't seen any tool of this type that supports MacOS. 

Many applications exist that can do the same job as EWW piecemeal, but there aren't any
that ship something close to EWW's basic feature-set out of the box. This was a problem
for me, so I decided to just write something myself. 

I also use MacOS, but am not satisfied with `Spotlight`, so this project will attempt to
support MacOS as well, which many other launchers and panels don't do. 

However, due to the relatively locked-down nature of MacOS (and, specifically, lack of
support for native OpenGL), some features (like window transparency) may be unsupported
on Mac host systems. 

# Philosophy
`erun` is not, and probably will never be, a ui toolkit. 

`erun` is, however, intended to be a competent alternative to EWW (elkowar's wacky widgets)
in a way that doesn't mandate you learn a new (functional and lispy) programming language. 

In keeping with this philosophy, `erun` will have a primary goal of keeping the configuration
process as simple as possible, with sane defaults and as many features optional as possible.

`erun` will also, within reason, attempt to support as many different config file formats
as possible, ensuring a minimal learning curve for any prospective users while granting
maximum flexibility and ease of use. 

These formats are currently planned to have first-class support prior to 1.0: 
- [x] ron (Rusty Object Notation)
- [ ] kdl (Cuddly Data Language)
- [ ] json
    - [ ] Hjson (Human json)
- [ ] yaml
- [ ] toml

# Performance
I haven't run any benchmarks, as the software is in extremely early development at the
moment. However, some performance downsides are to be expected, as `egui` is used in this
project, which is an immediate mode UI toolkit as opposed to the retained-state UI toolkit
used in EWW (GTK) or similar projects. 

As the project goes on, I'll make sure to benchmark `erun` properly and implement
good-by-default performance settings to make sure that no noticeable performance
degradation can be seen as a fault of the tool itself. 

# Roadmap/Planned Features
- [ ] Shell Integration
    - [ ] Custom list elements from `stdin`
        - [ ] Custom list elements for multi-list Windows `stdin`
- [ ] Runner
    - [ ] Auto resize window based on number of hits (possibly related to transparent window), see Spotlight on Mac
    - [ ] multiple columns in scrolling list (pagenated similar to `rofi` aesthetic)
    - [x] custom run command support
        - [x] default to `open`
    - [x] custom run args support
    - [x] option to make window close after running process
        - [x] fork process to outlive parent runner process
            - [x] `bridge` settings to override this if runner child needs stdio
                - [x] `bridge_stdin`
                - [x] `bridge_stdout`
                - [x] `bridge_stderr`
    - [ ] 
- [ ] Bar & Panel
    - [ ] reserve WM space
    - [ ] spawn window at coordinates
    - [ ] spawn window on specfic monitor
        - [ ] default monitor to `PRIMARY`
    - [ ] spawn window with specified dimensions
    - [ ] some way to set `WM_IGNORE` for tiling wms that try to manage the window
    - [x] set window as not resizable
    - [x] set window as no decorations
    - [x] hide window from taskbar
    - [x] hide window from alt-tab
    - [x] spawn inactive window (can't receive focus)
- [ ] Tests & Examples
    - [ ] Write automatic serialization tests for all examples in `test_cfg`
    - [x] Generate examples using serde
    - [ ] Figure out `#[serde(skip_serialize_if)]` to improve serialization in cases when
    generating examples is important and all fields need to be represented (no skipping)
- [ ] CLI
    - [ ] `erun open -` deserialize window struct from `stdin`
    - [ ] `erun open` with no file argument default to reading `erun.ron` file from current directory, 
    iterates through possible extensions, then checks `~/.config/erun/erun.ron` and other extensions, then
    checks `~/.config/erun.ron` and other extensions
    - [ ] `erun open [FILE] [FORMAT?]` multi-format support in CLI
        - [ ] format default to `infer` based on file extension
        - [ ] some form of inference engine based on file content, for use with `stdin` deserialization
    - [ ] `erun example [EXAMPLE] [FORMAT?]` multi-format examples in CLI 
        - [ ] infer user's preferred format from the located closest config file that would open with `erun open`
    - [ ] `erun help [COMMAND?]` outputs for subcommands
    - [x] `erun help` output for main process
- [ ] Core
    - [x] make everything `#[serde(default)]` to clean up config format
    - [x] ron support
    - [ ] kdl support
    - [ ] json support
        - [ ] Hjson support
    - [ ] yaml support
    - [ ] toml support
    - [ ] components
        - [ ] Image
            - [ ] Actually Show an image instead of crashing with a `todo!()`
            - [ ] cache web URIs on local disk and convert from `http` or `https` URIs to `bytes` URIs for performance (maybe a flag?)
        - [x] Frame
            - [x] Margin
            - [x] Padding
            - [ ] Fill
            - [ ] Outline
            - [ ] Multiple Children
                - [ ] possible `taffy` integration with css grid or flex layouts
        - [ ] Label
        - [ ] Variable
        - [ ] Button
        - [x] List
            - [ ] Filter search
            - [x] search box (currently useless)
            - [ ] pass filter box content to ListProvider for custom behavior
            - [x] smooth scrolling
            - [ ] pagenation / multi column layout
            - [x] ListProvider
                - [x] ListProvider::Applications for system apps
                    - [x] icon support
                    - [ ] MacOS icon support (reading ICNS files)
                    - [x] MacOS desktop file support
                    - [x] Linux desktop file support
                    - [ ] Windows start menu applications support
                - [ ] ListProvider::Stdio for reading from std IO
                    - [ ] some way of getting icons
                - [ ] ListProvider::Command for running a command
                    - [ ] Command updated based on events
                    - [ ] Events:
                        - [ ] Search field updated (user input)
                            - [ ] flag to pass user input to the command actually running to update
                            - [ ] flag to disable filtering so that the command getting user input can do it manually
                        - [ ] New Frame (Poll)
                        - [ ] Fork thread and poll on regular time intervals
                        - [ ] Run once at init
    - [ ] serialize colors as hex codes instead of rgba tuples
    - [x] serialize `px` and `pt` as a suffixed string instead of an unwieldy enum
- [ ] Style
    - [ ] ability to customize fonts
    - [ ] ability to customize `Widget` settings
    - [ ] style struct
    - [x] background colors
    - [x] foreground colors
    - [x] panel fill
    - [x] window rounding (TODO: Test this, it's untested but "done")
    - [ ] set zoom factor
- [ ] Misc
    - [x] spawn fullscreen window
    - [x] spawn maximized window
    - [x] spawn mouse passthrough window
    - [x] spawn invisible window

