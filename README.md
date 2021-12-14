# htmlq
Like [`jq`](https://stedolan.github.io/jq/), but for HTML. Uses [CSS selectors](https://developer.mozilla.org/en-US/docs/Learn/CSS/Introduction_to_CSS/Selectors) to extract bits of content from HTML files.

## Installation

### [Cargo](https://crates.io/crates/htmlq)

```sh
cargo install htmlq
```

### [FreeBSD pkg](https://www.freshports.org/textproc/htmlq)

```sh
pkg install htmlq
```

### [Homebrew](https://formulae.brew.sh/formula/htmlq)

```sh
brew install htmlq
```

### [Scoop](https://scoop.sh/)

```sh
scoop install htmlq
```

## Usage

```console
$ htmlq -h
htmlq 0.4.0
Michael Maclean <michael@mgdm.net>
Runs CSS selectors on HTML

USAGE:
    htmlq [FLAGS] [OPTIONS] [--] [selector]...

FLAGS:
    -B, --detect-base          Try to detect the base URL from the <base> tag in the document. If not found, default to
                               the value of --base, if supplied
    -h, --help                 Prints help information
    -w, --ignore-whitespace    When printing text nodes, ignore those that consist entirely of whitespace
    -p, --pretty               Pretty-print the serialised output
    -t, --text                 Output only the contents of text nodes inside selected elements
    -V, --version              Prints version information

OPTIONS:
    -a, --attribute <attribute>         Only return this attribute (if present) from selected elements
    -b, --base <base>                   Use this URL as the base for links
    -f, --filename <FILE>               The input file. Defaults to stdin
    -o, --output <FILE>                 The output file. Defaults to stdout
    -r, --remove-nodes <SELECTOR>...    Remove nodes matching this expression before output. May be specified multiple
                                        times

ARGS:
    <selector>...    The CSS expression to select [default: html]
$
```

## Examples

### Using with cURL to find part of a page by ID

```console
$ curl --silent https://www.rust-lang.org/ | htmlq '#get-help'
<div class="four columns mt3 mt0-l" id="get-help">
        <h4>Get help!</h4>
        <ul>
          <li><a href="https://doc.rust-lang.org">Documentation</a></li>
          <li><a href="https://users.rust-lang.org">Ask a Question on the Users Forum</a></li>
          <li><a href="http://ping.rust-lang.org">Check Website Status</a></li>
        </ul>
        <div class="languages">
            <label class="hidden" for="language-footer">Language</label>
            <select id="language-footer">
                <option title="English (US)" value="en-US">English (en-US)</option>
<option title="French" value="fr">Français (fr)</option>
<option title="German" value="de">Deutsch (de)</option>

            </select>
        </div>
      </div>
```

### Find all the links in a page

```console
$ curl --silent https://www.rust-lang.org/ | htmlq --attribute href a
/
/tools/install
/learn
/tools
/governance
/community
https://blog.rust-lang.org/
/learn/get-started
https://blog.rust-lang.org/2019/04/25/Rust-1.34.1.html
https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
[...]
```

### Get the text content of a post

```console
$ curl --silent https://nixos.org/nixos/about.html | htmlq  --text .main

          About NixOS

NixOS is a GNU/Linux distribution that aims to
improve the state of the art in system configuration management.  In
existing distributions, actions such as upgrades are dangerous:
upgrading a package can cause other packages to break, upgrading an
entire system is much less reliable than reinstalling from scratch,
you can’t safely test what the results of a configuration change will
be, you cannot easily undo changes to the system, and so on.  We want
to change that.  NixOS has many innovative features:

[...]
```

### Remove a node before output

There's a big SVG image in this page that I don't need, so here's how to remove it.

```console
$ curl --silent https://nixos.org/ | ./target/debug/htmlq '.whynix' --remove-nodes svg
<ul class="whynix">
      <li>

        <h2>Reproducible</h2>
        <p>
          Nix builds packages in isolation from each other. This ensures that they
          are reproducible and don't have undeclared dependencies, so <strong>if a
            package works on one machine, it will also work on another</strong>.
        </p>
      </li>
      <li>

        <h2>Declarative</h2>
        <p>
          Nix makes it <strong>trivial to share development and build
            environments</strong> for your projects, regardless of what programming
          languages and tools you’re using.
        </p>
      </li>
      <li>

        <h2>Reliable</h2>
        <p>
          Nix ensures that installing or upgrading one package <strong>cannot
            break other packages</strong>. It allows you to <strong>roll back to
            previous versions</strong>, and ensures that no package is in an
          inconsistent state during an upgrade.
        </p>
      </li>
    </ul>
```

### Pretty print HTML

(This is a bit of a work in progress)

```console
$ curl --silent https://mgdm.net | htmlq --pretty '#posts'
<section id="posts">
  <h2>I write about...
  </h2>
  <ul class="post-list">
    <li>
      <time datetime="2019-04-29 00:%i:1556496000" pubdate="">
        29/04/2019</time><a href="/weblog/nettop/">
        <h3>Debugging network connections on macOS with nettop
        </h3></a>
      <p>Using nettop to find out what network connections a program is trying to make.
      </p>
    </li>
[...]
```

### Syntax highlighting with [`bat`](https://github.com/sharkdp/bat)

```console
$ curl --silent example.com | htmlq 'body' | bat --language html
```

> <img alt="Syntax highlighted output" width="700" src="https://user-images.githubusercontent.com/2346707/132808980-db8991ff-9177-4cb7-a018-39ad94282374.png" />
