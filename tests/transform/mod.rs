use assert_cmd::{Command, cargo::cargo_bin};
use assert_fs::{assert::PathAssert, prelude::FileWriteStr};

const COLORSET_FROM: &str = "rosewater: f2d5cf
flamingo: ee9f92
pink: f4b8e4
mauve: ca9ee6
red: e78284
maroon: ea999c
peach: ef9f76
yellow: e5c890
green: a6d189
teal: 81c8be
sky: 99d1db
sapphire: 85c1dc
blue: 8caaee
lavender: babbf1
text: c6d0f5
subtext1: b5bfe2
subtext0: a5adce
overlay2: 949cbb
overlay1: 838ba7
overlay0: 737994
surface2: 626880
surface1: 51576d
surface0: 414559
base: 303446
mantle: 292c3c
crust: 232634";

const COLORSCHEME: &str = "# The basic colors
foreground              #c6d0f5
background              #303446
selection_foreground    #303446
selection_background    #f2d5cf

# Cursor colors
cursor                  #f2d5cf
cursor_text_color       #303446

# URL underline color when hovering with mouse
url_color               #f2d5cf

# Kitty window border colors
active_border_color     #babbf1
inactive_border_color   #737994
bell_border_color       #e5c890

# OS Window titlebar colors
wayland_titlebar_color system
macos_titlebar_color system

# Tab bar colors
active_tab_foreground   #232634
active_tab_background   #ca9ee6
inactive_tab_foreground #c6d0f5
inactive_tab_background #292c3c
tab_bar_background      #232634

# Colors for marks (marked text in the terminal)
mark1_foreground #303446
mark1_background #babbf1
mark2_foreground #303446
mark2_background #ca9ee6
mark3_foreground #303446
mark3_background #85c1dc";

const COLORSET_TO: &str = "rosewater: f2d5cf
flamingo: ee9f92
pink: f4b8e4
mauve: ca9ee6
red: e78284
maroon: ea999c
peach: ef9f76
yellow: e5c890
green: a6d189
teal: 81c8be
sky: 99d1db
sapphire: 85c1dc
blue: 8caaee
lavender: babbf1

text: dcdde0
subtext1: c4c5c8
subtext0: b0b1b4
overlay2: 9d9ea2
overlay1: 8a8b8f
overlay0: 77787b
surface2: 656669
surface1: 535456
surface0: 414243
base: 303030
mantle: 2a2a2a
crust: 242424";

const RESULT: &str = "# The basic colors
foreground              #DCDDE0
background              #303030
selection_foreground    #303030
selection_background    #F2D5CF

# Cursor colors
cursor                  #F2D5CF
cursor_text_color       #303030

# URL underline color when hovering with mouse
url_color               #F2D5CF

# Kitty window border colors
active_border_color     #BABBF1
inactive_border_color   #77787B
bell_border_color       #E5C890

# OS Window titlebar colors
wayland_titlebar_color system
macos_titlebar_color system

# Tab bar colors
active_tab_foreground   #242424
active_tab_background   #CA9EE6
inactive_tab_foreground #DCDDE0
inactive_tab_background #2A2A2A
tab_bar_background      #242424

# Colors for marks (marked text in the terminal)
mark1_foreground #303030
mark1_background #BABBF1
mark2_foreground #303030
mark2_background #CA9EE6
mark3_foreground #303030
mark3_background #85C1DC";

#[test]
fn simple_stdio() {
    let mut cmd = Command::cargo_bin(
        cargo_bin!("colorscheme-transformer")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    let colorset_from_file = assert_fs::NamedTempFile::new("colorset_from.yaml").unwrap();
    colorset_from_file.write_str(COLORSET_FROM).unwrap();

    let colorset_to_file = assert_fs::NamedTempFile::new("colorset_to.yaml").unwrap();
    colorset_to_file.write_str(COLORSET_TO).unwrap();

    cmd.arg("transform")
        .arg(colorset_from_file.path())
        .arg(colorset_to_file.path())
        .write_stdin(COLORSCHEME)
        .assert()
        .success()
        .stdout(RESULT);
}

#[test]
fn simple_files() {
    let mut cmd = Command::cargo_bin(
        cargo_bin!("colorscheme-transformer")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    let colorset_from_file = assert_fs::NamedTempFile::new("colorset_from.yaml").unwrap();
    colorset_from_file.write_str(COLORSET_FROM).unwrap();

    let colorset_to_file = assert_fs::NamedTempFile::new("colorset_to.yaml").unwrap();
    colorset_to_file.write_str(COLORSET_TO).unwrap();

    let input_file = assert_fs::NamedTempFile::new("input.conf").unwrap();
    input_file.write_str(COLORSCHEME).unwrap();

    let out_file = assert_fs::NamedTempFile::new("out.conf").unwrap();

    cmd.arg("transform")
        .arg("-i")
        .arg(input_file.path())
        .arg("-o")
        .arg(out_file.path())
        .arg(colorset_from_file.path())
        .arg(colorset_to_file.path())
        .assert()
        .success()
        .stdout("");

    out_file.assert(RESULT);
}
