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

const RESULT: &str = "# The basic colors
foreground              #{text}
background              #{base}
selection_foreground    #{base}
selection_background    #{rosewater}

# Cursor colors
cursor                  #{rosewater}
cursor_text_color       #{base}

# URL underline color when hovering with mouse
url_color               #{rosewater}

# Kitty window border colors
active_border_color     #{lavender}
inactive_border_color   #{overlay0}
bell_border_color       #{yellow}

# OS Window titlebar colors
wayland_titlebar_color system
macos_titlebar_color system

# Tab bar colors
active_tab_foreground   #{crust}
active_tab_background   #{mauve}
inactive_tab_foreground #{text}
inactive_tab_background #{mantle}
tab_bar_background      #{crust}

# Colors for marks (marked text in the terminal)
mark1_foreground #{base}
mark1_background #{lavender}
mark2_foreground #{base}
mark2_background #{mauve}
mark3_foreground #{base}
mark3_background #{sapphire}";

#[test]
fn simple_stdio() {
    let mut cmd = Command::cargo_bin(
        cargo_bin!("colorscheme-transformer")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
    .unwrap();

    let colorset_file = assert_fs::NamedTempFile::new("colorset.yaml").unwrap();
    colorset_file.write_str(COLORSET_FROM).unwrap();

    cmd.arg("make")
        .arg(colorset_file.path())
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

    let colorset_file = assert_fs::NamedTempFile::new("colorset.yaml").unwrap();
    colorset_file.write_str(COLORSET_FROM).unwrap();

    let input_file = assert_fs::NamedTempFile::new("input.conf").unwrap();
    input_file.write_str(COLORSCHEME).unwrap();

    let out_file = assert_fs::NamedTempFile::new("out.conf").unwrap();

    cmd.arg("make")
        .arg("-i")
        .arg(input_file.path())
        .arg("-o")
        .arg(out_file.path())
        .arg(colorset_file.path())
        .assert()
        .success()
        .stdout("");

    out_file.assert(RESULT);
}
