use assert_cmd::{Command, cargo::cargo_bin};
use assert_fs::{assert::PathAssert, prelude::FileWriteStr};

const TEMPLATE: &str = "# The basic colors
foreground              {text}
background              {base}
selection_foreground    {base}
selection_background    {rosewater}

# Cursor colors
cursor                  {rosewater}
cursor_text_color       {base}

# URL underline color when hovering with mouse
url_color               {rosewater}

# Kitty window border colors
active_border_color     {lavender}
inactive_border_color   {overlay0}
bell_border_color       {yellow}

# OS Window titlebar colors
wayland_titlebar_color system
macos_titlebar_color system

# Tab bar colors
active_tab_foreground   {crust}
active_tab_background   {mauve}
inactive_tab_foreground {text}
inactive_tab_background {mantle}
tab_bar_background      {crust}

# Colors for marks (marked text in the terminal)
mark1_foreground {base}
mark1_background {lavender}
mark2_foreground {base}
mark2_background {mauve}
mark3_foreground {base}
mark3_background {sapphire}";

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

    let colorset_file = assert_fs::NamedTempFile::new("colorset.yaml").unwrap();
    colorset_file.write_str(COLORSET_TO).unwrap();

    cmd.arg("fill")
        .arg(colorset_file.path())
        .write_stdin(TEMPLATE)
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
    colorset_file.write_str(COLORSET_TO).unwrap();

    let template_file = assert_fs::NamedTempFile::new("template.conf").unwrap();
    template_file.write_str(TEMPLATE).unwrap();

    let out_file = assert_fs::NamedTempFile::new("out.conf").unwrap();

    cmd.arg("fill")
        .arg("-i")
        .arg(template_file.path())
        .arg("-o")
        .arg(out_file.path())
        .arg(colorset_file.path())
        .assert()
        .success()
        .stdout("");

    out_file.assert(RESULT);
}
